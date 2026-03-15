# Miro Manual

## 개요

`Miro`는 `codex`, `claude-code` 세션을 한 화면에서 조회하고, 다시 열고, 삭제할 수 있는 터미널 TUI 프로그램이다.

이 구현은 아래 문서를 기준으로 완성되었다.

- 계획 문서: `miro-plan.md`
- 구현 메모: `miro-feat.md`
- 테마 계획 문서: `miro-theme-plan.md`
- 테마 구현 메모: `miro-theme-feat.md`

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

## 전역 실행 구성

설치 스크립트:

```bash
scripts/install-miro-global.sh install
```

이 스크립트는 `~/bin/miro` 래퍼를 생성하거나 갱신한다.  
래퍼는 현재 저장소의 릴리스 바이너리 `/Users/cozyai/dev/.miro/target/release/miro`를 호출한다.

상태 확인:

```bash
scripts/install-miro-global.sh status
which miro
```

제거:

```bash
scripts/install-miro-global.sh uninstall
```

업데이트 절차:

```bash
cargo build --release
scripts/install-miro-global.sh install
```

프로젝트 경로가 바뀌면 기존 래퍼의 절대 경로가 깨질 수 있다.  
이 경우 새 경로에서 설치 스크립트를 다시 실행해야 한다.

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

### 테마

- 기본 테마는 `Tomorrow Night Blue`다.
- 테마는 `--theme` 옵션으로 선택할 수 있다.
- 사용 가능한 테마 목록은 `miro themes`로 확인할 수 있다.
- TUI 화면 안에서는 `t`로 테마 메뉴를 열 수 있다.
- 테마 메뉴가 열리면 `Up` / `Down`으로 이동하고 `Enter`로 적용하며 `Esc` 또는 `t`로 닫는다.

#### 테마 유지

- TUI `t` 메뉴에서 `Enter`로 테마를 적용하면 `~/.config/miro/config.toml`에 자동 저장된다.
- 다음 실행 시 저장된 테마가 자동으로 적용된다.
- `--theme` CLI 옵션은 한 세션에만 적용되는 일회성 오버라이드다. 설정 파일을 변경하지 않는다.
- 설정 파일이 없거나 값이 잘못되면 `tomorrow-night-blue`로 시작한다.

테마 적용 우선순위:

```
--theme CLI 옵션 (세션 한정) > ~/.config/miro/config.toml > tomorrow-night-blue
```

설정 파일 예시 (`~/.config/miro/config.toml`):

```toml
theme = "dracula"
```

지원 테마 (총 14종):

| CLI ID | 이름 | 계열 |
|--------|------|------|
| `tomorrow-night-blue` | Tomorrow Night Blue | 딥 블루 다크 (기본) |
| `default` | Default | 블루-그레이 다크 |
| `cursor-dark` | Cursor Dark | 슬레이트 다크 |
| `darcula-dark` | Darcula Dark | JetBrains 다크 |
| `darcula-light` | Darcula Light | 소프트 라이트 |
| `dracula` | Dracula | 보라-검정 다크 |
| `nord` | Nord | 북극 청회 다크 |
| `one-dark` | One Dark | Atom 슬레이트 다크 |
| `gruvbox-dark` | Gruvbox Dark | 황토 레트로 다크 |
| `gruvbox-light` | Gruvbox Light | 황토 레트로 라이트 |
| `catppuccin-mocha` | Catppuccin Mocha | 파스텔 라벤더 다크 |
| `tokyo-night` | Tokyo Night | 심야 도시 다크 |
| `solarized-dark` | Solarized Dark | 청록 다크 |
| `solarized-light` | Solarized Light | 아이보리 라이트 |

예시:

```bash
miro themes
miro --theme tomorrow-night-blue
miro --theme dracula
miro --theme nord
miro --theme one-dark
miro --theme gruvbox-dark
miro --theme catppuccin-mocha
miro --theme tokyo-night
miro --theme solarized-dark
miro --theme solarized-light
miro --theme gruvbox-light
```

기본 실행:

```bash
cargo run
```

릴리스 바이너리 직접 실행:

```bash
/Users/cozyai/dev/.miro/target/release/miro
```

전역 명령 실행:

```bash
miro
```

기본 실행 시 `Tomorrow Night Blue` 테마가 적용된다.

키 바인딩:

- `Up` / `Down`: 세션 이동
- `Enter`: 선택 세션 재진입
- `t`: 테마 메뉴 열기 또는 닫기
- `d`: 삭제 확인 모달 열기
- `y`: 삭제 확정
- `n` 또는 `Esc`: 삭제 취소
- `f`: provider 필터 순환
- `/`: 검색 입력 시작
- `Backspace`: 검색어 삭제
- `r`: 목록 새로고침
- `q`: 종료

선택된 세션은 하이라이트 배경과 강조 텍스트로 표시된다.
하단 푸터에는 항상 주요 메뉴 설명이 표시된다.
테마 메뉴가 열리면 현재 지원 테마 목록과 기본 테마 표시를 바로 확인할 수 있다.

## CLI 사용법

### 세션 목록 조회

```bash
cargo run -- list
/Users/cozyai/dev/.miro/target/release/miro list
miro list
miro --theme default list
```

### 테마 목록 조회

```bash
cargo run -- themes
/Users/cozyai/dev/.miro/target/release/miro themes
miro themes
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
miro resume <session-id>
```

provider를 명시해야 할 때:

```bash
cargo run -- resume <session-id> --provider codex
cargo run -- resume <session-id> --provider claude-code
```

재진입 동작 메모:

- `codex`, `claude-code` 모두 세션이 생성된 원래 작업 디렉터리를 기준으로 재진입을 시도한다.
- TUI에서 재진입 실패가 발생해도 앱이 즉시 종료되지 않고 목록 화면으로 복귀한다.
- 외부 세션을 종료하고 돌아왔을 때도 TUI 레이아웃이 다시 복구되도록 처리한다.

### 특정 세션 삭제

```bash
cargo run -- delete <session-id> --yes
/Users/cozyai/dev/.miro/target/release/miro delete <session-id> --yes
miro delete <session-id> --yes
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
- `--theme` 미지정 시 `None` 반환 확인
- `--theme` 인자 파싱 (`default`, `tomorrow-night-blue` 등)
- `themes` 명령 파싱
- 잘못된 테마 이름 입력 시 오류 메시지 검증 (14종 전부 포함)
- 신규 9종 테마 각각 단위 테스트 (`dracula`, `nord`, `one-dark`, `gruvbox-dark`, `gruvbox-light`, `catppuccin-mocha`, `tokyo-night`, `solarized-dark`, `solarized-light`)
- `cli_id()` 메서드 케밥케이스 검증
- `from_cli_id()` 알려진 ID 파싱 및 알 수 없는 값 `None` 반환
- `MiroConfig::theme_name()` 유효/무효 값 처리
- `MiroConfig` 저장/복원 roundtrip (tempfile 기반)

## 현재 한계

- `codex`와 `claude-code` 세션 구조가 향후 바뀌면 파싱 규칙도 같이 수정해야 한다.
- 삭제는 로컬 파일과 인덱스 기준으로만 처리한다.
- 재진입은 각 CLI가 제공하는 공식 `resume` 커맨드에 의존한다.
- Claude 제목 품질은 세션 메타데이터에 따라 `/login`, `/exit` 같은 커맨드성 문구가 일부 포함될 수 있다.
