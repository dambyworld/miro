# Miro Manual

## 개요

`Miro`는 `codex`, `claude-code` 세션을 한 화면에서 조회하고, 다시 열고, 삭제할 수 있는 터미널 TUI 프로그램이다.

이 구현은 아래 문서를 기준으로 완성되었다.

- 계획 문서: `miro-plan.md`
- 구현 메모: `miro-feat.md`

## 빌드와 실행

### 개발 실행

```bash
cargo run
```

기본 실행은 TUI 모드다.

### 릴리스 빌드

```bash
cargo build --release
```

실행 파일 경로:

```bash
target/release/miro
```

현재 생성된 단일 바이너리:

```bash
/Users/cozyai/dev/.miro/target/release/miro
```

바이너리 직접 실행:

```bash
/Users/cozyai/dev/.miro/target/release/miro
```

## 지원 기능

1. `codex`, `claude-code` 세션 목록 조회
2. 특정 세션 재진입
3. 특정 세션 삭제

## 데이터 소스

### codex

- 인덱스: `~/.codex/session_index.jsonl`
- 세션 본문: `~/.codex/sessions/**`
- 재진입: `codex resume <session-id>`

### claude-code

- 인덱스: `~/.claude/projects/**/sessions-index.json`
- 세션 본문: `~/.claude/projects/**/*.jsonl`
- 재진입: `claude --resume <session-id>`

## 목록 표시 규칙

- 세션 ID 대신 사람이 이해할 수 있는 제목을 우선 표시한다.
- `codex`는 `thread_name`을 기본 제목으로 사용한다.
- `claude-code`는 `summary` 또는 `firstPrompt`를 기본 제목으로 사용한다.
- 보조 정보로 마지막 대화 문구, 작업 디렉터리, 최근 갱신 시각을 함께 보여준다.

## TUI 사용법

기본 실행:

```bash
cargo run
```

릴리스 바이너리 직접 실행:

```bash
/Users/cozyai/dev/.miro/target/release/miro
```

키 바인딩:

- `Up` / `Down`: 세션 이동
- `Enter`: 선택 세션 재진입
- `d`: 삭제 확인 모달 열기
- `y`: 삭제 확정
- `n` 또는 `Esc`: 삭제 취소
- `f`: provider 필터 순환
- `/`: 검색 입력 시작
- `Backspace`: 검색어 삭제
- `r`: 목록 새로고침
- `q`: 종료

선택된 세션은 하이라이트 배경과 강조 텍스트로 표시된다.

## CLI 사용법

### 세션 목록 조회

```bash
cargo run -- list
/Users/cozyai/dev/.miro/target/release/miro list
```

provider 필터:

```bash
cargo run -- list --provider codex
cargo run -- list --provider claude-code
```

JSON 출력:

```bash
cargo run -- list --output json
```

### 특정 세션 재진입

```bash
cargo run -- resume <session-id>
/Users/cozyai/dev/.miro/target/release/miro resume <session-id>
```

provider를 명시해야 할 때:

```bash
cargo run -- resume <session-id> --provider codex
cargo run -- resume <session-id> --provider claude-code
```

### 특정 세션 삭제

```bash
cargo run -- delete <session-id> --yes
/Users/cozyai/dev/.miro/target/release/miro delete <session-id> --yes
```

provider를 명시해야 할 때:

```bash
cargo run -- delete <session-id> --provider codex --yes
cargo run -- delete <session-id> --provider claude-code --yes
```

## 삭제 동작

### codex

- 세션 JSONL 파일을 삭제한다.
- `session_index.jsonl`에서 해당 세션 항목을 제거한다.

### claude-code

- 세션 JSONL 파일을 삭제한다.
- 같은 프로젝트 디렉터리의 `sessions-index.json`에서 해당 세션 항목을 제거한다.

## 테스트

전체 테스트:

```bash
cargo test
```

검증된 항목:

- `codex` 메시지 텍스트 추출
- `claude-code` 메시지 텍스트 추출
- `codex` 인덱스 날짜 파싱
- `codex` 삭제 시 인덱스 재작성

## 현재 한계

- `codex`와 `claude-code` 세션 구조가 향후 바뀌면 파싱 규칙도 같이 수정해야 한다.
- 삭제는 로컬 파일과 인덱스 기준으로만 처리한다.
- 재진입은 각 CLI가 제공하는 공식 `resume` 커맨드에 의존한다.
