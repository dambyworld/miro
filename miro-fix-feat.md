# Miro Fix 구현 메모

## 작업 목표

`claude-code` 세션이 목록에 나타나지 않는 문제를 수정하고, 동시에 TUI 목록 밀도와 컬러 테마를 개선한다.

## 구현 요약

- `claude-code` provider를 `sessions-index.json` 단일 의존 구조에서 벗어나게 수정
- `~/.claude/projects/*/*.jsonl` 세션 파일 직접 스캔 로직 추가
- `sessions-index.json`은 보조 메타데이터 소스로만 사용하도록 변경
- stale `fullPath`가 있어도 실제 JSONL 파일이 있으면 세션을 표시하도록 수정
- 기본 TUI 목록을 한 줄 밀도 중심으로 재구성
- provider별 색상 포인트와 더 강한 선택 하이라이트 적용
- `claude-code` 재진입 시 세션의 원래 `cwd`에서 resume 하도록 수정
- 재진입 실패 시 TUI가 즉시 종료되지 않고 목록으로 복귀하도록 수정
- 외부 세션 종료 후 TUI 레이아웃이 유지되도록 터미널 복구 경로 보강

## 코드 변경

### Claude provider

수정 파일:

- `src/provider/claude.rs`

핵심 변경:

- `project_session_paths()` 추가
- Claude JSONL에서 `sessionId`, `cwd`, `timestamp`, 첫 사용자 메시지, preview 추출
- 인덱스 엔트리와 JSONL 레코드를 `sessionId` 기준으로 병합
- 인덱스가 없거나 stale여도 실제 JSONL 기준으로 세션 조회 가능

제목 우선순위:

1. 인덱스 `summary`
2. 인덱스 `firstPrompt`
3. JSONL 첫 유효 사용자 메시지
4. JSONL preview
5. `Untitled Claude session`

### TUI

수정 파일:

- `src/lib.rs`
- `src/tui.rs`

핵심 변경:

- 세션 항목을 3줄 카드형에서 1줄 밀도형으로 축소
- 헤더/푸터 높이를 줄여 목록 영역 확대
- `codex`, `claude-code` provider별 색상 차등 적용
- 제목, preview, 메타정보를 서로 다른 색상 계층으로 분리
- 선택 항목 하이라이트를 더 강한 포인트 컬러로 조정
- 하단 도움말이 항상 보이도록 푸터 렌더링 유지
- 재진입 후 alt-screen, clear, cursor, autoresize를 복구하도록 수정

## 테스트 및 검증

실행한 검증:

- `cargo test`
- `cargo run -- list --provider claude-code`
- `cargo run -- list --provider codex`
- `cargo run -- resume <claude-session-id> --provider claude-code` 최소 동작 확인

검증 결과:

- Claude provider 단위 테스트 2개 추가
- 전체 테스트 통과
- 실제 로컬 환경에서 `claude-code` 세션 목록 출력 확인

## 추가된 테스트

- 인덱스 없이 JSONL만 있는 Claude 프로젝트 조회
- `sessions-index.json`의 `fullPath`가 stale여도 JSONL fallback으로 조회

## 관찰 결과

- 수정 전에는 `cargo run -- list --provider claude-code`가 빈 결과였다.
- 수정 후에는 실제 `claude-code` 세션들이 정상적으로 출력된다.
- 수정 전에는 `claude-code` 재진입 시 `No conversation found with session ID`로 즉시 종료될 수 있었다.
- 수정 후에는 세션의 원래 작업 디렉터리 기준으로 재진입을 시도한다.
- 수정 전에는 재진입 실패 시 TUI가 종료되거나, 복귀 후 레이아웃이 깨질 수 있었다.
- 수정 후에는 실패 시 목록 화면으로 복귀하고, 재진입 후에도 터미널 레이아웃 복구 경로를 가진다.
- 현재 제목 품질은 세션 메타데이터 품질에 따라 다르며, `/login`, `/exit` 같은 로컬 커맨드 기반 제목이 일부 포함될 수 있다.

## 후속 개선 후보

- Claude 제목 생성 시 slash command 노이즈를 더 적극적으로 제거
- 선택된 세션만 상세 preview를 추가로 보여주는 2단 구성
- CLI 출력 포맷에서도 provider별 정보 계층을 더 명확하게 정리
