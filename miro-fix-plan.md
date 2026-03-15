# Miro TUI 개선 계획

## 목표

현재 `Miro`에서 `codex` 세션만 조회되고, `claude-code` 세션은 조회되지 않는 문제를 분석하고 수정 계획을 수립한다.

이 문서는 수정 계획으로 작성되었고, 현재 구현 반영까지 완료된 상태다.

## 구현 결과

- `claude-code` 세션 조회가 정상 동작하도록 수정 완료
- `codex`, `claude-code` 세션이 함께 목록에 표시되도록 수정 완료
- TUI 목록 밀도를 높여 기본 화면에서 더 많은 세션을 볼 수 있도록 조정 완료
- TUI 색상 계층과 하이라이트를 더 선명하게 조정 완료

## 현재 문제

- `cargo run -- list --provider codex`는 정상적으로 세션을 출력한다.
- `cargo run -- list --provider claude-code`는 아무 세션도 출력하지 않는다.
- 따라서 기본 TUI에서도 실제로는 `codex` 세션만 보이는 상태다.

## 재현 결과

### 1. codex provider

- `~/.codex/session_index.jsonl`와 `~/.codex/sessions/**` 조합으로 정상 조회된다.

### 2. claude-code provider

- 현재 구현은 `~/.claude/projects/**/sessions-index.json`만 탐색한다.
- 실제 로컬 데이터에서는 `sessions-index.json`이 있는 프로젝트가 매우 적다.
- 반면 `~/.claude/projects/*/*.jsonl` 세션 파일은 여러 프로젝트에 존재한다.

## 확인한 로컬 증거

### 1. 인덱스 없는 프로젝트가 많음

다음 프로젝트 디렉터리에는 `sessions-index.json` 없이 JSONL 세션 파일만 존재한다.

- `~/.claude/projects/-Users-cozyai-dev`
- `~/.claude/projects/-Users-cozyai-WebstormProjects-temp0`
- `~/.claude/projects/-Users-cozyai-IdeaProjects-dday`
- `~/.claude/projects/-Users-cozyai-WebstormProjects-dday`

### 2. 인덱스가 있어도 stale 상태일 수 있음

`~/.claude/projects/-Users-cozyai-Test/sessions-index.json`는 존재하지만:

- `entries`는 2개만 포함
- 각 `fullPath`는 현재 디스크에 존재하지 않음
- 현재 구현은 `if !session_path.exists() { continue; }`로 인해 전부 skip

즉, 인덱스가 있어도 신뢰할 수 없는 상태다.

### 3. JSONL만으로도 필요한 필드를 읽을 수 있음

실제 Claude 세션 JSONL 첫 레코드에서 아래 값이 확인된다.

- `sessionId`
- `cwd`
- `timestamp`
- `message` 또는 `content`

따라서 최소한의 세션 목록은 인덱스 없이도 JSONL 스캔만으로 구성 가능하다.

## 현재 코드 기준 원인

문제 지점은 [src/provider/claude.rs](/Users/cozyai/dev/.miro/src/provider/claude.rs)다.

핵심 원인:

1. `project_index_paths()`가 `sessions-index.json`만 수집한다.
2. `list_sessions()`가 인덱스 엔트리만 기준으로 세션 목록을 만든다.
3. 인덱스가 없거나 `fullPath`가 stale이면 세션이 전부 누락된다.
4. JSONL 파일 직접 스캔 fallback이 없다.

## 원인 요약

`claude-code` 세션 조회 로직이 "인덱스 파일이 항상 존재하고, 그 안의 `fullPath`도 항상 유효하다"는 가정에 묶여 있다.  
하지만 실제 로컬 저장 구조는 그 가정을 만족하지 않기 때문에, 정상 세션이 있어도 목록에서 누락된다.

## 수정 방향

`claude-code` provider를 아래 우선순위로 바꾼다.

### 1. JSONL 직접 스캔을 기본 수집 경로로 추가

- `~/.claude/projects/*/*.jsonl` 파일을 직접 순회한다.
- `sessions-index.json` 유무와 무관하게 실제 세션 파일을 기준으로 세션을 수집한다.

### 2. 인덱스는 보조 메타데이터 소스로만 사용

- `sessions-index.json`이 있으면 `summary`, `firstPrompt`, `modified` 같은 풍부한 필드를 우선 활용한다.
- 단, `fullPath`가 stale여도 엔트리 자체를 세션 존재 판단의 절대 기준으로 삼지 않는다.

### 3. JSONL 자체에서 세션 정보를 복원

JSONL에서 아래 항목을 복원한다.

- `sessionId`
- `cwd`
- 마지막 유효 `timestamp`
- 표시용 제목 후보
- 마지막 대화 문구 preview

### 4. 인덱스와 JSONL 결과를 병합

- 키는 `sessionId`
- 인덱스 정보가 있으면 제목/요약을 보강
- JSONL 정보가 있으면 실제 파일 경로, `cwd`, 최신 시각을 보강
- 같은 세션이 중복 수집되면 실제 파일이 존재하는 쪽을 우선

## 세부 수정 계획

### 1단계. Claude provider 수집 전략 변경

- `project_index_paths()` 외에 `project_session_paths()` 추가
- JSONL 파일 직접 순회 로직 구현
- `sessions-index.json` 의존 단일 경로 구조를 제거

### 2단계. JSONL 파서 추가

- Claude JSONL 첫/마지막 유효 레코드에서 `sessionId`, `cwd`, `timestamp` 추출
- 표시용 제목은 다음 우선순위로 선정
  1. 인덱스 `summary`
  2. 인덱스 `firstPrompt`
  3. 첫 유효 사용자 메시지
  4. 마지막 유효 메시지 preview
  5. `Untitled Claude session`

### 3단계. 병합 로직 추가

- 인덱스 기반 엔트리와 JSONL 기반 엔트리를 `sessionId` 기준으로 병합
- stale `fullPath`는 무시하고 실제 존재하는 JSONL 경로를 우선 사용
- `updated_at`은 실제 JSONL의 최신 timestamp를 우선, 없으면 인덱스 `modified`를 fallback

### 4단계. 삭제 로직 보강 계획

현재 삭제도 `sessions-index.json` 존재를 전제로 일부 동작한다.  
조회 수정 후에는 삭제도 아래 방향으로 점검한다.

- JSONL 파일 삭제는 그대로 유지
- `sessions-index.json`이 있으면 항목 제거
- 인덱스가 없으면 파일 삭제만으로도 정상 동작하도록 유지

### 5단계. 목록 밀도 개선 계획

현재 TUI는 한 화면에 보이는 세션 수가 적어서, `claude-code` 세션이 정상 조회되더라도 전체 목록 확인 효율이 낮을 수 있다.  
따라서 조회 수정과 함께 목록 밀도도 같이 조정한다.

- 각 세션 카드의 세로 높이를 줄인다.
- 제목, preview, 보조 정보 줄 수를 재조정한다.
- 기본 터미널 크기에서 약 20건 전후가 보이도록 레이아웃을 조정한다.
- 필요하면 선택된 항목만 상세 정보를 더 보여주고, 비선택 항목은 한 줄 또는 두 줄 표시로 축약한다.

### 6단계. TUI 컬러 테마 개선 계획

현재 TUI는 기능 검증 위주라 시각적으로 다소 밋밋하다.  
조회 문제 수정과 함께 최소한의 시각 개선도 같이 반영한다.

- provider별 색상 포인트를 둔다.
- 선택된 세션 하이라이트를 더 선명하게 조정한다.
- 제목, preview, 보조 정보의 색상 대비를 분리한다.
- 전체 화면이 단조롭지 않도록 헤더, 목록, 도움말 영역의 스타일을 차등 적용한다.

## 테스트 계획

### 1. 단위 테스트

- 인덱스 없이 JSONL만 있는 Claude 프로젝트를 읽을 수 있는지 검증
- `fullPath`가 stale인 `sessions-index.json`이 있어도 JSONL fallback으로 세션이 조회되는지 검증
- 인덱스와 JSONL이 동시에 있을 때 `summary`/`firstPrompt`가 우선 적용되는지 검증

### 2. 통합 테스트

- `cargo run -- list --provider claude-code` 결과에 실제 Claude 세션이 1개 이상 포함되는지 검증
- 기본 목록에서 `codex`, `claude-code`가 함께 보이는지 검증
- 기본 TUI 레이아웃에서 한 화면에 약 20건 전후의 세션이 보이는지 수동 확인

## UX 조정 기준

- 목록은 현재보다 더 촘촘하게 표시한다.
- 폰트 크기를 직접 제어할 수 없는 터미널 환경 특성상, 실제 대응은 행 높이 축소와 표시 정보 압축으로 수행한다.
- 기본 화면에서 세션 20건 안팎을 확인할 수 있도록 설계한다.
- 선택된 세션은 밀도를 높이더라도 하이라이트가 충분히 눈에 띄어야 한다.
- UI는 현재보다 더 컬러풀하게 조정하되, 가독성을 해치지 않는 범위에서 색상 대비를 분명하게 준다.
- 제목, preview, provider, 메타정보는 서로 다른 색상 계층으로 구분한다.
- 선택 상태는 단순 반전색보다 더 명확한 포인트 컬러 조합을 사용한다.

## 수정 완료 기준

아래 조건을 만족하면 수정 완료로 본다.

1. `claude-code` 세션이 실제 로컬 JSONL 기준으로 목록에 나타난다.
2. `codex`와 `claude-code` 세션이 동시에 하나의 목록에서 보인다.
3. `sessions-index.json`이 없어도 Claude 세션 조회가 가능하다.
4. `sessions-index.json`의 `fullPath`가 stale여도 Claude 세션 조회가 가능하다.
5. 기존 `codex` 조회 동작은 깨지지 않는다.
6. 기본 TUI 화면에서 세션 약 20건 전후를 확인할 수 있도록 목록 밀도가 개선된다.
7. 기본 TUI 화면이 현재보다 더 컬러풀하고, 정보 계층이 색상으로도 분명히 구분된다.

## 구현 반영 상태

이번 계획의 핵심 항목은 실제 코드에 반영되었다.  
구현 상세와 검증 결과는 [miro-fix-feat.md](/Users/cozyai/dev/.miro/miro-fix-feat.md)에 정리한다.
