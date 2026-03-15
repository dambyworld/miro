# Miro 테마 기능 구현 메모

## 구현 범위

- TUI 테마 모듈 분리
- `Tomorrow Night Blue` 기본 테마 적용
- 초기 제공 테마 `Default`, `Cursor Dark`, `Darcula Dark`, `Darcula Light`
- 추가 제공 테마 `Dracula`, `Nord`, `One Dark`, `Gruvbox Dark`, `Gruvbox Light`, `Catppuccin Mocha`, `Tokyo Night`, `Solarized Dark`, `Solarized Light`
- `--theme <theme-id>` CLI 옵션 추가
- `themes` 메뉴 명령 추가
- `ThemeName::cli_id()` 메서드 추가 (list_themes 하드코딩 제거)
- **테마 선택 유지**: TUI에서 선택한 테마를 `~/.config/miro/config.toml`에 자동 저장/복원
- 테마 관련 테스트 추가
- 매뉴얼 업데이트

## 구현 결정

- 테마는 [src/theme.rs](/Users/cozyai/dev/.miro/src/theme.rs)에서 역할 기반 스타일 묶음으로 관리한다.
- 기본 테마는 `tomorrow-night-blue`다.
- 기존 하드코딩 계열은 `Default` 테마로 정리한다.
- 사용 가능한 테마 목록은 `miro themes`로 출력한다.
- TUI는 더 이상 RGB 값을 직접 만들지 않고 `Theme`에서 스타일을 받아 렌더링한다.
- 테마 선택은 `--theme` CLI 옵션 또는 TUI `t` 메뉴로 제공한다.
- TUI에서 적용한 테마는 `~/.config/miro/config.toml`에 자동 저장된다.
- 테마 적용 우선순위: `--theme` CLI (세션 한정) > 설정 파일 > `tomorrow-night-blue`.
- `--theme` CLI 오버라이드는 설정 파일을 변경하지 않는다.

## 구현 파일

- [src/theme.rs](/Users/cozyai/dev/.miro/src/theme.rs)
- [src/config.rs](/Users/cozyai/dev/.miro/src/config.rs) — 신규: 설정 파일 로드/저장
- [src/cli.rs](/Users/cozyai/dev/.miro/src/cli.rs)
- [src/lib.rs](/Users/cozyai/dev/.miro/src/lib.rs)
- [src/tui.rs](/Users/cozyai/dev/.miro/src/tui.rs)
- [miro-theme-plan.md](/Users/cozyai/dev/.miro/miro-theme-plan.md)
- [miro-manual.md](/Users/cozyai/dev/.miro/miro-manual.md)

## 기능 메모

### 기본 테마

- `miro`를 옵션 없이 실행하면 `Tomorrow Night Blue` 테마가 적용된다.
- 헤더에는 현재 사용 중인 테마 이름이 표시된다.

### 선택 가능한 테마

- `tomorrow-night-blue` (기본)
- `default`
- `cursor-dark`
- `darcula-dark`
- `darcula-light`
- `dracula`
- `nord`
- `one-dark`
- `gruvbox-dark`
- `gruvbox-light`
- `catppuccin-mocha`
- `tokyo-night`
- `solarized-dark`
- `solarized-light`

### 사용 예시

```bash
miro themes
miro
miro --theme tomorrow-night-blue
miro --theme default
miro --theme cursor-dark
miro --theme darcula-dark
miro --theme darcula-light
miro --theme dracula
miro --theme nord
miro --theme one-dark
miro --theme gruvbox-dark
miro --theme gruvbox-light
miro --theme catppuccin-mocha
miro --theme tokyo-night
miro --theme solarized-dark
miro --theme solarized-light
```

### 적용 범위

- 헤더
- 세션 목록 보더
- provider 라벨
- 제목, preview, 메타 텍스트
- 선택 하이라이트
- 푸터 도움말과 상태 메시지
- 삭제 확인 모달
- 빈 상태 메시지

## 테스트 결과

- `cargo test` 통과 (31개 테스트)
- `cargo build --release` 통과
- `./target/release/miro themes` 14개 테마 정상 출력 확인
- `./target/release/miro list --provider codex` 정상 출력 확인
- `./target/release/miro --theme darcula-light list --provider claude-code` 정상 출력 확인
- 신규 9종 테마 각각 단위 테스트 통과
- `cli_id_matches_kebab_case` 테스트 통과
- `from_cli_id` 파싱 테스트 통과
- `config::save_and_load_roundtrip` 저장/복원 테스트 통과
- 설정 파일 없을 때 기본값 사용 테스트 통과
- 잘못된 theme 값 무시 테스트 통과

## 남은 범위

- 환경 변수 기반 테마 선택은 아직 미구현
- 설정 파일 기반 영구 테마 저장은 아직 미구현
