# Miro 구현 메모

## 구현 범위

- `codex`, `claude-code` 세션 목록 조회
- 특정 세션 재진입
- 특정 세션 삭제
- 기본 터미널 TUI
- 비대화형 `list`, `resume`, `delete` 커맨드
- 전역 실행용 `~/bin/miro` 설치 스크립트

## 구현 결정

- 언어와 TUI 프레임워크는 `Rust + Ratatui`로 확정
- `codex`는 `~/.codex/session_index.jsonl`과 `~/.codex/sessions/**`를 함께 사용
- `claude-code`는 `~/.claude/projects/**/sessions-index.json`과 세션 JSONL을 사용
- 목록 표시 제목은 세션 ID가 아니라 사람이 읽을 수 있는 `thread_name`, `summary`, 마지막 대화 문구를 우선 사용
- 선택된 세션은 TUI에서 하이라이트 배경과 굵은 텍스트로 강조
- 릴리스 단일 바이너리는 `target/release/miro`에 생성
- 전역 실행은 `scripts/install-miro-global.sh`가 `~/bin/miro` 래퍼를 생성하는 방식으로 구성

## 구현 파일

- `src/lib.rs`
- `src/main.rs`
- `src/app.rs`
- `src/cli.rs`
- `src/model.rs`
- `src/provider/codex.rs`
- `src/provider/claude.rs`
- `src/provider/mod.rs`
- `src/tui.rs`
- `scripts/install-miro-global.sh`

## 기능 메모

### 목록 조회

- 기본 실행은 TUI 목록 화면
- `miro list`는 표 형태 출력
- `miro list --output json`은 JSON 출력
- provider 필터는 `all`, `codex`, `claude-code`

### 재진입

- `codex`는 `codex resume <session-id>`
- `claude-code`는 `claude --resume <session-id>`
- TUI에서 `Enter`를 누르면 선택된 세션으로 재진입

### 삭제

- `miro delete <session-id> --yes`
- TUI에서 `d` 후 `y`
- `codex` 삭제 시 세션 파일과 `session_index.jsonl` 항목을 함께 제거
- `claude-code` 삭제 시 세션 JSONL과 `sessions-index.json` 항목을 함께 제거

### 전역 실행

- `scripts/install-miro-global.sh install`이 `~/bin/miro` 래퍼를 생성 또는 갱신
- `scripts/install-miro-global.sh uninstall`이 전역 래퍼를 제거
- 래퍼는 `/Users/cozyai/dev/.miro/target/release/miro`를 호출
- 릴리스 바이너리가 없으면 `cargo build --release` 안내 메시지를 출력

## 테스트 범위

- provider 텍스트 추출 로직 단위 테스트
- 인덱스 파싱 단위 테스트
- 삭제 시 인덱스 재작성 검증
- `cargo run -- list --provider codex` 스모크 실행 확인
- `cargo build --release`로 릴리스 바이너리 생성 확인
- `scripts/install-miro-global.sh status`로 설치 상태 확인
- `scripts/install-miro-global.sh install` 후 `which miro`와 `miro list` 확인

## 검증 결과

- `cargo test` 통과
- `cargo build --release` 통과
- `scripts/install-miro-global.sh install`로 `/Users/cozyai/bin/miro` 설치 확인
- `which miro` 결과가 `/Users/cozyai/bin/miro`로 확인
- `/tmp`에서 `miro list --provider codex` 정상 출력 확인
