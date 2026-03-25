# miro ✨

> codex랑 claude-code 세션을 한눈에 볼 수 있는 터미널 TUI예요 🌸

---

## 이런 분들께 딱이에요 💕

- `codex`랑 `claude-code` 세션이 너무 많아서 어디 있는지 모르겠을 때
- 이전 대화를 빠르게 다시 열고 싶을 때
- 필요 없는 세션을 깔끔하게 정리하고 싶을 때

---

## 설치

```bash
brew install dambyworld/tap/miro
```

업그레이드도 간단해요!

```bash
brew upgrade dambyworld/tap/miro
```

---

## 사용법

```bash
miro
```

그냥 `miro`만 치면 돼요 🎀

### 키 바인딩

| 키 | 동작 |
|----|------|
| `↑` `↓` | 세션 이동 |
| `Enter` | 세션 재진입 |
| `d` → `y` | 세션 삭제 |
| `f` | provider 필터 전환 |
| `/` | 세션 검색 (`title`, `preview`, `cwd`, `session_id`) |
| `Esc` | 검색 입력 종료 또는 현재 검색값 초기화 |
| `t` | 테마 메뉴 |
| `r` | 목록 새로고침, 헤더/푸터 갱신 표시 |
| `q` | 종료 |

`/` 검색은 `session_id` 일부 문자열로도 동작해요.
`/e120...`, `(e120...)`, `"e120..."`처럼 앞뒤에 기호가 붙어도 `session_id` 검색은 동작해요.
검색 입력 중 `Esc`는 입력만 닫고, 일반 모드 `Esc`는 현재 검색값을 지워요.

---

## 테마

총 14가지 테마를 지원해요 🎨

```bash
miro themes          # 테마 목록 보기
miro --theme dracula # 테마 지정해서 실행
```

TUI 안에서 `t`를 누르면 실시간으로 바꿀 수 있고, 선택한 테마는 자동으로 저장돼요!

새로고침(`r`)을 누르면 헤더의 `refreshed` 시각과 푸터 상태 메시지가 바로 갱신돼서 실제 동작 여부를 확인할 수 있어요.

---

## 빌드

```bash
cargo build --release
```

---

## 저장소

- 소스: https://github.com/dambyworld/miro
- Homebrew Tap: https://github.com/dambyworld/homebrew-tap
