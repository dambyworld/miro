# Miro 테마 기능 구성 계획

## 목표

`Miro`에 디자인 테마 기능을 도입한다.  
특히 `Tomorrow Night Blue` 테마는 필수 기본 제공 테마로 포함한다.

이번 단계는 기능 계획을 기준으로 실제 구현까지 진행했고, 아래 결과를 반영한다.

## 현재 상태

- 현재 TUI 색상은 [src/tui.rs](/Users/cozyai/dev/.miro/src/tui.rs)에 직접 하드코딩되어 있다.
- 헤더, 목록, 선택 하이라이트, 푸터, 삭제 확인 모달이 각각 개별 RGB 값으로 정의되어 있다.
- provider별 포인트 컬러도 `codex`, `claude-code`에 대해 직접 분기하고 있다.
- 따라서 새 테마를 추가하려면 현재 구조에서는 여러 위치를 동시에 수정해야 한다.

## 문제 정의

현재 구조는 "단일 고정 색상 스킨"에 가깝다.  
테마 기능을 넣으려면 아래 조건을 만족해야 한다.

1. 화면 전반의 색상과 강조 규칙을 한 번에 교체할 수 있어야 한다.
2. `Tomorrow Night Blue` 테마가 기본 제공되어야 한다.
3. provider 포인트 컬러와 선택 상태가 테마 안에서 일관되게 보여야 한다.
4. 이후 다른 테마를 추가해도 [src/tui.rs](/Users/cozyai/dev/.miro/src/tui.rs) 수정 범위가 작아야 한다.
5. 기본 사용 흐름과 기존 키 바인딩을 깨지 않아야 한다.

## 디자인 방향

### 기본 방향

- 테마 기능은 "색상 집합 교체" 수준이 아니라, 화면 역할별 스타일 세트를 교체하는 방식으로 설계한다.
- 즉, 헤더 색상만 바꾸는 것이 아니라 `header`, `list`, `selected`, `footer`, `dialog`, `status`, `provider badge`를 묶어서 다룬다.
- 초기 버전은 테마 수를 작게 시작하되, 구조는 다중 테마 확장을 전제로 만든다.

### 필수 테마

1. `tomorrow-night-blue`

필수 이유:

- 현재 Miro는 터미널 기반 TUI이므로 어두운 배경과 명확한 대비가 중요하다.
- `Tomorrow Night Blue`는 깊은 남색 배경과 비교적 절제된 강조색 조합이라 목록 밀도와 가독성이 좋다.
- 현재 Miro의 세션 브라우저 성격과도 잘 맞는다.

## 권장 테마 모델

테마는 단일 이름 문자열이 아니라, 역할 기반 스타일 묶음으로 관리한다.

예상 구조:

```text
Theme
- id
- name
- app_background
- header_style
- header_border_style
- list_border_style
- selected_row_style
- title_style
- preview_style
- meta_style
- footer_style
- footer_hint_style
- footer_status_style
- dialog_style
- dialog_border_style
- empty_state_style
- codex_badge_style
- claude_badge_style
```

핵심 원칙:

1. TUI 렌더링 코드는 색상값을 직접 만들지 않고 `Theme`에서 가져온다.
2. provider별 색상도 테마의 일부로 둔다.
3. 선택 하이라이트는 배경색 하나만이 아니라 전경색, 굵기, 필요 시 underline까지 한 세트로 관리한다.

## Tomorrow Night Blue 기준안

`Tomorrow Night Blue`는 아래 성격을 유지해야 한다.

- 전체 배경은 어두운 블루 계열
- 헤더와 푸터는 본문보다 조금 더 깊거나 선명한 블루 계열
- 본문 텍스트는 따뜻한 회백색 또는 차분한 밝은 회색
- 메타 정보는 채도를 낮춘 청회색
- 선택 하이라이트는 배경 대비가 분명하지만 과하게 번쩍이지 않는 블루-바이올렛 계열
- `codex`, `claude-code` 포인트 컬러는 테마 위에서 쉽게 구분되되, 원색 과다 사용은 피함

필수 검토 항목:

1. 좁은 터미널 폭에서도 색만으로 선택 상태가 충분히 보이는지
2. 긴 목록에서 시선이 피로하지 않은지
3. provider 뱃지가 선택 상태와 겹쳐도 읽히는지
4. 삭제 확인 모달이 일반 목록과 충분히 구분되는지

## 기능 범위

이번 테마 기능은 아래 범위를 목표로 잡는다.

### 1. 기본 제공 테마 정의

- 최소 `default`, `tomorrow-night-blue` 제공
- `tomorrow-night-blue`를 기본 테마로 사용
- 기존 색상 세트는 호환용 `default` 또는 `legacy` 성격의 보조 테마로 유지 검토
- `tomorrow-night-blue`는 별도 명시적 ID로도 제공

### 2. 테마 적용 범위

- 헤더
- 세션 목록 보더
- provider 라벨
- 제목 텍스트
- preview 텍스트
- 메타 텍스트
- 선택 하이라이트
- 푸터 도움말
- 상태 메시지
- 삭제 확인 모달
- 빈 목록 상태

### 3. 테마 선택 방식

초기 버전에서는 아래 방식 중 하나를 선택한다.

안 A. CLI 옵션

- 예: `miro --theme tomorrow-night-blue`

장점:

- 구현이 단순하다.
- 테스트와 재현이 쉽다.

단점:

- 매번 옵션을 넣어야 한다.

안 B. 환경 변수

- 예: `MIRO_THEME=tomorrow-night-blue miro`

장점:

- 셸 설정과 궁합이 좋다.
- 전역 기본값 구성에 유리하다.

단점:

- 사용자가 기능 존재를 놓치기 쉽다.

안 C. 설정 파일

- 예: `~/.config/miro/config.toml`

장점:

- 장기적으로 가장 확장성이 좋다.
- 테마 외의 향후 사용자 설정과 묶기 쉽다.

단점:

- 이번 기능 단독으로는 구현 범위가 커진다.

## 권장 적용 순서

1순위는 `Tomorrow Night Blue 기본값 + CLI 옵션 + 내부 테마 레지스트리`다.

권장 이유:

1. 필수 요구사항인 `Tomorrow Night Blue`를 기본 경험으로 바로 제공할 수 있다.
2. 기능 검증이 가장 빠르다.
3. 실패 지점이 적다.
4. 이후 환경 변수나 설정 파일을 얹기 쉬운 구조다.

2단계 확장안:

- `MIRO_THEME` 환경 변수 지원
- 설정 파일 도입

## 구현 방향

### 1단계. 스타일 분리

- [src/tui.rs](/Users/cozyai/dev/.miro/src/tui.rs)에서 하드코딩된 `Style::default().fg(...).bg(...)` 정의를 테마 모듈로 이동
- 예: `src/theme.rs` 또는 `src/ui/theme.rs`
- 렌더링 코드는 `theme.header_style()`, `theme.codex_badge_style()` 같은 방식으로만 접근

### 2단계. 기본 테마 정의

- `tomorrow-night-blue` 테마를 기본 테마 상수 또는 팩토리 함수로 구현
- 기존 색상 세트는 보조 테마로 정리하되, 이름은 `default`보다 `legacy`에 가깝게 검토
- 사용자가 테마를 명시하지 않으면 `tomorrow-night-blue`가 선택되도록 기본값을 고정

### 3단계. 테마 선택 입력 추가

- 우선 `--theme <theme-id>` CLI 인자를 추가
- 잘못된 테마 ID가 들어오면 사용 가능한 테마 목록을 포함한 오류 메시지 제공
- `--theme`를 주지 않으면 `tomorrow-night-blue`를 사용

### 4단계. 앱 상태 연결

- TUI 초기화 시 선택된 테마를 `AppState` 또는 별도 UI 컨텍스트에 주입
- 화면 전체가 같은 테마 인스턴스를 사용하도록 통일

### 5단계. 테마 문서화

- [miro-manual.md](/Users/cozyai/dev/.miro/miro-manual.md)에 사용 가능한 테마와 실행 예시 추가
- `Tomorrow Night Blue`를 필수 제공 테마로 명시

## UX 기준

1. 선택된 세션은 어떤 테마에서도 즉시 구분되어야 한다.
2. provider 구분은 색상에만 의존하지 말고 현재 텍스트 라벨도 유지한다.
3. 푸터 도움말과 상태 메시지는 서로 묻히지 않아야 한다.
4. 삭제 확인 모달은 위험 동작이라는 점이 색상과 레이아웃에서 분명해야 한다.
5. `Tomorrow Night Blue`는 예쁜 테마보다 "길게 봐도 피로하지 않은 테마"를 우선한다.

## 테스트 계획

### 자동 검증

- 테마 ID 파싱 테스트
- 기본 테마가 `tomorrow-night-blue`인지 확인하는 테스트
- `tomorrow-night-blue` 테마 조회 테스트
- 알 수 없는 테마 입력 시 오류 메시지 테스트

### 수동 검증

1. 옵션 없이 `miro` 실행 시 `tomorrow-night-blue`가 적용되는지 확인
2. `miro --theme tomorrow-night-blue` 실행 시 동일한 결과가 나오는지 확인
3. 보조 테마 실행 시 테마 전환이 실제로 반영되는지 확인
4. 긴 목록에서 선택 이동 확인
5. provider 필터 전환 확인
6. 검색 모드 진입 확인
7. 삭제 모달 표시 확인
8. 재진입 실패 시 상태 메시지 가독성 확인

## 리스크

1. 현재 렌더링 코드가 한 파일에 몰려 있어 스타일 분리 시 수정 범위가 생각보다 커질 수 있다.
2. `Tomorrow Night Blue` 색 조합이 실제 터미널 팔레트에서 기대보다 탁하게 보일 수 있다.
3. 선택 하이라이트와 provider 포인트 컬러가 충돌하면 정보 위계가 흐려질 수 있다.
4. 테마 선택 인터페이스를 너무 크게 시작하면 설정 기능까지 범위가 확대될 수 있다.

## 리스크 대응

1. 첫 단계는 색상과 스타일만 분리하고 레이아웃 변경은 최소화한다.
2. `Tomorrow Night Blue`는 스크린샷 기준이 아니라 실제 터미널 대비 기준으로 조정한다.
3. provider 색상은 선택 하이라이트 위에서도 읽히는지 별도 확인한다.
4. 초기 선택 방식은 `--theme` 하나로 제한해 범위를 통제한다.
5. 기본값은 `tomorrow-night-blue`로 고정해 요구사항 해석 차이를 없앤다.

## 완료 기준

아래 조건을 만족하면 테마 기능 MVP로 본다.

1. `miro`가 테마 ID를 받아 실행할 수 있다.
2. `tomorrow-night-blue`가 기본 테마로 적용된다.
3. 최소 2개 이상의 테마를 선택할 수 있다.
4. 헤더, 목록, 선택 상태, 푸터, 삭제 모달이 테마에 따라 일관되게 바뀐다.
5. `tomorrow-night-blue`에서 `codex`, `claude-code` 포인트 컬러가 모두 식별 가능하다.
6. 문서에 테마 사용법과 기본 테마 정책이 반영된다.

## 구현 결과 반영 계획

구현 완료 후 아래 문서를 함께 갱신한다.

1. [miro-theme-feat.md](/Users/cozyai/dev/.miro/miro-theme-feat.md)
2. [miro-manual.md](/Users/cozyai/dev/.miro/miro-manual.md)
3. [miro-theme-plan.md](/Users/cozyai/dev/.miro/miro-theme-plan.md)

검증이 끝나면 기본 테마 정책과 실제 지원 테마 목록, 실행 예시, 테스트 결과를 문서에 반영한다.

## 구현 반영 결과

이번 구현에서는 아래 내용을 반영한다.

1. `tomorrow-night-blue`를 기본 테마로 적용
2. 추가 테마 `Default`, `Cursor Dark`, `Darcula Dark`, `Darcula Light` 제공
3. `themes` 명령으로 사용 가능한 테마 목록 출력
4. `--theme <theme-id>` CLI 옵션 추가
5. TUI 스타일을 [src/theme.rs](/Users/cozyai/dev/.miro/src/theme.rs)로 분리
6. 테마 관련 테스트와 매뉴얼 반영

---

## 추가 테마 확장 계획 (구현 보류)

현재 5종 테마 (`tomorrow-night-blue`, `default`, `cursor-dark`, `darcula-dark`, `darcula-light`) 이후 추가할 테마 후보군을 정리한다.
아래는 계획 단계이며, 실제 구현은 별도 작업에서 진행한다.

### 추가 목표

- 색상 계열(파란계, 보라계, 갈색계, 녹색계, 밝은계)별 대표 테마를 최소 1종씩 확보
- 라이트 계열 테마를 현재 1종(`darcula-light`)에서 2종 이상으로 확장
- 인기 에디터/IDE 테마와의 연관성을 통해 사용자 친숙도 제고

### 후보 테마 목록

각 테마는 `ThemeName` enum에 추가되고, `ThemeName::all()` 목록에 등재된다.
아래 순서는 구현 우선순위 기준이다.

#### 1. `dracula` — Dracula Dark

| 항목 | 내용 |
|------|------|
| 계열 | 보라-검정 다크 |
| 특징 | 보라와 분홍 강조색, 짙은 배경. 전 세계적으로 가장 많이 쓰이는 다크 테마 중 하나 |
| 배경 | `#282a36` (짙은 거의 검정에 가까운 보라) |
| 선택 하이라이트 | `#44475a` (약한 보라 회색) |
| 강조색 | 보라 `#bd93f9`, 분홍 `#ff79c6`, 시안 `#8be9fd` |
| codex 뱃지 | 주황-노랑 `#ffb86c` |
| claude 뱃지 | 시안-민트 `#50fa7b` |
| 주의사항 | 선택 하이라이트 `#44475a`가 배경 대비가 좁을 수 있어 fg 색을 밝게 잡아야 함 |
| CLI ID | `dracula` |

#### 2. `nord` — Nord

| 항목 | 내용 |
|------|------|
| 계열 | 북극 청회색 다크 |
| 특징 | 채도를 낮춘 냉한 파란-회색 계열. 눈 피로가 적고 차분한 느낌 |
| 배경 | `#2e3440` (짙은 청회) |
| 헤더 | `#3b4252` |
| 선택 하이라이트 | `#4c566a` |
| 강조색 | 파랑 `#81a1c1`, 하늘 `#88c0d0`, 민트 `#8fbcbb` |
| codex 뱃지 | 황토-살구 `#ebcb8b` |
| claude 뱃지 | 민트-청록 `#88c0d0` |
| 주의사항 | 채도가 전반적으로 낮아 선택 상태를 fg 강조(bold/underline)로 보완 필수 |
| CLI ID | `nord` |

#### 3. `gruvbox-dark` — Gruvbox Dark

| 항목 | 내용 |
|------|------|
| 계열 | 갈색-황토 레트로 다크 |
| 특징 | 따뜻한 갈색과 황토색 위주. 복고 터미널 느낌. 고대비 버전(`hard`)과 보통 버전 혼용 |
| 배경 | `#282828` (hard: `#1d2021`) |
| 헤더 | `#3c3836` |
| 선택 하이라이트 | `#504945` |
| 강조색 | 노랑 `#d79921`, 주황 `#d65d0e`, 녹색 `#98971a` |
| codex 뱃지 | 황금-노랑 `#fabd2f` |
| claude 뱃지 | 연두-수세미 `#b8bb26` |
| 주의사항 | 녹색 계열이 강조색이므로 claude 뱃지가 일반 텍스트와 혼동될 수 있어 굵기 필수 |
| CLI ID | `gruvbox-dark` |

#### 4. `gruvbox-light` — Gruvbox Light

| 항목 | 내용 |
|------|------|
| 계열 | 황토 레트로 라이트 |
| 특징 | Gruvbox Dark의 라이트 변종. 따뜻한 베이지-크림 배경 |
| 배경 | `#fbf1c7` (크림-베이지) |
| 헤더 | `#d5c4a1` |
| 선택 하이라이트 | `#bdae93` |
| 강조색 | 갈색 `#7c6f64`, 주황 `#af3a03`, 녹색 `#79740e` |
| codex 뱃지 | 진한 주황 `#b57614` |
| claude 뱃지 | 짙은 녹색 `#427b58` |
| 주의사항 | 라이트 테마 중 가장 따뜻한 색 계열. 차가운 계열 터미널 팔레트에서 의도와 다르게 보일 수 있음 |
| CLI ID | `gruvbox-light` |

#### 5. `one-dark` — One Dark

| 항목 | 내용 |
|------|------|
| 계열 | 슬레이트 블루 다크 |
| 특징 | Atom 에디터 기반. VS Code에서도 가장 많이 쓰이는 다크 테마 중 하나. 균형 잡힌 채도 |
| 배경 | `#282c34` |
| 헤더 | `#353b45` |
| 선택 하이라이트 | `#3e4451` |
| 강조색 | 파랑 `#61afef`, 보라 `#c678dd`, 시안 `#56b6c2` |
| codex 뱃지 | 살구-황금 `#e5c07b` |
| claude 뱃지 | 연두 `#98c379` |
| 주의사항 | 선택 하이라이트와 배경 명도 차가 작아 UNDERLINED modifier 필수 |
| CLI ID | `one-dark` |

#### 6. `solarized-dark` — Solarized Dark

| 항목 | 내용 |
|------|------|
| 계열 | 녹색-청록 다크 |
| 특징 | 과학적으로 설계된 대비 색 조합. 8색 베이스 팔레트가 라이트/다크 양쪽에 동작 |
| 배경 | `#002b36` |
| 헤더 | `#073642` |
| 선택 하이라이트 | `#094958` |
| 강조색 | 시안 `#2aa198`, 파랑 `#268bd2`, 녹색 `#859900` |
| codex 뱃지 | 주황 `#cb4b16` |
| claude 뱃지 | 시안 `#2aa198` |
| 주의사항 | 초록/시안 계열이 강하므로 claude 뱃지 색 선택 시 배경과 분리 확인 필요 |
| CLI ID | `solarized-dark` |

#### 7. `solarized-light` — Solarized Light

| 항목 | 내용 |
|------|------|
| 계열 | 아이보리-크림 라이트 |
| 특징 | Solarized의 라이트 변종. 따뜻한 아이보리 배경에 동일 강조 팔레트 |
| 배경 | `#fdf6e3` |
| 헤더 | `#eee8d5` |
| 선택 하이라이트 | `#d0cfc3` |
| 강조색 | 파랑 `#268bd2`, 시안 `#2aa198`, 보라 `#6c71c4` |
| codex 뱃지 | 주황 `#cb4b16` |
| claude 뱃지 | 시안 `#2aa198` |
| 주의사항 | 라이트 테마이므로 하이라이트 배경 명도 차가 다크 계열보다 좁음. bg 대신 bold+underline 강화 검토 |
| CLI ID | `solarized-light` |

#### 8. `catppuccin-mocha` — Catppuccin Mocha

| 항목 | 내용 |
|------|------|
| 계열 | 파스텔 라벤더 다크 |
| 특징 | 최근 가장 빠르게 확산 중인 파스텔 기반 다크 테마. 은은하고 부드러운 채도 |
| 배경 | `#1e1e2e` |
| 헤더 | `#313244` |
| 선택 하이라이트 | `#45475a` |
| 강조색 | 라벤더 `#b4befe`, 파랑 `#89b4fa`, 핑크 `#f38ba8` |
| codex 뱃지 | 피치-살구 `#fab387` |
| claude 뱃지 | 민트-마카롱 `#a6e3a1` |
| 주의사항 | 전체 채도가 낮으므로 뱃지/강조색이 기대보다 차분하게 보일 수 있음. 실제 터미널에서 팔레트 매핑 확인 필수 |
| CLI ID | `catppuccin-mocha` |

#### 9. `tokyo-night` — Tokyo Night

| 항목 | 내용 |
|------|------|
| 계열 | 심야 도시 다크 |
| 특징 | VS Code 플러그인에서 출발한 야경 테마. 짙은 남보라 배경에 청록-보라 강조 |
| 배경 | `#1a1b26` |
| 헤더 | `#24283b` |
| 선택 하이라이트 | `#364a82` |
| 강조색 | 청록 `#7aa2f7`, 보라 `#bb9af7`, 시안 `#7dcfff` |
| codex 뱃지 | 황금 `#e0af68` |
| claude 뱃지 | 연두 `#9ece6a` |
| 주의사항 | `tomorrow-night-blue`와 배경 색 계열이 가장 유사하므로, 사용자 혼동 최소화를 위해 선택 하이라이트를 더 보라-남색으로 차별화 |
| CLI ID | `tokyo-night` |

### 구현 시 공통 고려사항

1. **`ThemeName` enum 순서**: `all()` 배열의 첫 번째가 기본값임. 추가 테마는 기존 5종 뒤에 붙인다.
2. **라이트 테마 주의**: `selected_row` fg/bg 명도 차가 다크보다 좁으므로 `BOLD | UNDERLINED` modifier를 반드시 적용한다.
3. **뱃지 가독성**: `codex_badge`와 `claude_badge`는 `selected_row` 배경 위에서도 읽히는지 항상 확인한다.
4. **테스트 추가**: 각 신규 테마에 대해 `resolves_<theme_id>_theme()` 단위 테스트를 `src/theme.rs`의 `#[cfg(test)]` 블록에 추가한다.
5. **`lists_supported_themes()` 갱신**: `ThemeName::all()` 검증 테스트에 신규 테마를 추가한다.
6. **문서 반영**: 구현 완료 후 `miro-manual.md`의 테마 목록 섹션에 신규 테마를 추가한다.

### 구현 우선순위

| 순위 | 테마 ID | 이유 |
|------|---------|------|
| 1 | `dracula` | 사용자 기반이 가장 넓고, 보라 계열로 기존 블루 계열과 차별화 명확 |
| 2 | `nord` | 채도 낮은 냉한 계열로 기존 테마와 성격 차이 있음. Catppuccin 사용자와 교집합 큼 |
| 3 | `one-dark` | VS Code 기본 인지도 높음. 슬레이트 블루로 `cursor-dark`와 성격이 약간 겹치나 수요 충분 |
| 4 | `gruvbox-dark` | 따뜻한 갈색 계열. 기존 테마 전부 차가운 계열이라 색온도 다양성 확보 |
| 5 | `catppuccin-mocha` | 최근 상승세. 파스텔 계열로 기존 테마와 성격 차별화 |
| 6 | `tokyo-night` | `tomorrow-night-blue`와 유사하나 보라 계열 차별화. 요청 있을 경우 추가 |
| 7 | `solarized-dark` | 클래식하나 오래된 팔레트. 니즈 확인 후 추가 |
| 8 | `solarized-light` | Solarized Dark와 세트. 함께 추가하는 것이 자연스러움 |
| 9 | `gruvbox-light` | `gruvbox-dark`와 세트. 라이트 테마 선호 사용자 대상 |

### 완료 기준 (추가 테마용)

1. 신규 테마 enum variant가 `ThemeName`에 추가된다.
2. 각 테마 팩토리 함수가 `src/theme.rs`에 구현된다.
3. `ThemeName::all()` 및 `Theme::get()` 분기에 등록된다.
4. `display_name()`, `description()`이 각 테마에 정의된다.
5. 단위 테스트가 각 테마에 추가된다.
6. `miro-manual.md` 테마 목록이 갱신된다.

---

## 테마 선택 유지 기능 계획 (구현 보류)

### 문제 정의

현재 TUI 내에서 `t` 키로 테마를 바꿔도, miro를 종료하고 다시 실행하면 항상 `tomorrow-night-blue`로 초기화된다.
`--theme` CLI 옵션은 매번 명시해야 하므로 불편하고, TUI 내 테마 메뉴의 선택이 세션 밖으로 이어지지 않는다.

### 목표

- TUI에서 마지막으로 선택한 테마가 다음 실행에도 자동 적용된다.
- `--theme` CLI 옵션은 한 번만 적용되는 일회성 오버라이드로 유지한다.
- 사용자가 별도로 설정하지 않아도 동작해야 한다 (설정 파일 자동 생성).

### 설계 원칙

1. 설정 파일은 사용자가 의식하지 않아도 자동으로 생성된다.
2. `--theme` CLI 인자는 설정 파일을 덮어쓰지 않는다. 세션 한정 오버라이드다.
3. 테마 적용 우선순위: `--theme` CLI (일회성) > 설정 파일 > 내장 기본값 (`tomorrow-night-blue`).
4. 설정 파일 로드 실패는 조용히 무시하고 기본값으로 폴백한다.
5. 설정 파일 저장 실패도 조용히 무시한다 (TUI 흐름을 끊지 않는다).

### 설정 파일 위치와 형식

#### 위치

```
~/.config/miro/config.toml
```

XDG 표준을 따른다. `dirs` 크레이트가 이미 의존성에 포함되어 있으므로 `dirs::config_dir()`로 경로를 얻는다.
디렉터리가 없으면 첫 저장 시 자동 생성한다.

#### 형식 (TOML)

```toml
theme = "dracula"
```

초기 버전은 `theme` 키 하나만 관리한다.
향후 다른 사용자 설정(필터 기본값, 정렬 기준 등)을 추가하기 쉬운 구조다.

### 필요한 크레이트 추가

| 크레이트 | 용도 | 현재 상태 |
|----------|------|-----------|
| `toml` | config.toml 직렬화/역직렬화 | 미포함 → 추가 필요 |
| `serde` | 구조체 직렬화 | 이미 포함 (`features = ["derive"]`) |
| `dirs` | XDG 경로 조회 | 이미 포함 |

`Cargo.toml`에 `toml = "0.8"` 추가 필요.

### 신규 모듈: `src/config.rs`

```rust
use serde::{Deserialize, Serialize};
use crate::theme::ThemeName;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MiroConfig {
    pub theme: Option<ThemeName>,
}

impl MiroConfig {
    // ~/.config/miro/config.toml 로드. 파일 없으면 Default 반환.
    pub fn load() -> Self { ... }

    // theme 필드 하나만 저장. 실패해도 에러 전파 안 함.
    pub fn save_theme(theme: ThemeName) { ... }
}
```

`ThemeName`을 TOML에 저장하려면 kebab-case 문자열로 직렬화해야 한다.
`serde(rename_all = "kebab-case")`로는 enum variant 이름(`TomorrowNightBlue`)이 변환되지 않으므로,
별도 `Serialize`/`Deserialize` 구현 또는 `#[serde(rename = "...")]` 어노테이션이 필요하다.
가장 단순한 접근은 `theme`를 `Option<String>`으로 저장하고, 로드 시 `ThemeName::from_cli_id(s)` 파서로 변환하는 것이다.

```rust
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MiroConfig {
    pub theme: Option<String>,  // "dracula", "nord" 등 cli_id 문자열 그대로 저장
}
```

로드 시 `ThemeName::from_cli_id(&s)`로 파싱하고, 인식 불가 값은 무시한다.

`ThemeName`에 `from_cli_id(s: &str) -> Option<ThemeName>` 메서드를 추가한다.

### `src/cli.rs` 변경: `--theme` 옵션을 `Option<ThemeName>`으로

현재:
```rust
#[arg(long, value_enum, default_value_t = ThemeName::TomorrowNightBlue)]
pub theme: ThemeName,
```

변경 후:
```rust
#[arg(long, value_enum)]
pub theme: Option<ThemeName>,
```

`default_value_t`를 제거해야 "사용자가 `--theme`를 명시했는지 여부"를 `None`으로 감지할 수 있다.
clap은 `Option<T>` 필드를 인자 미제공 시 자동으로 `None`으로 처리한다.

### `src/lib.rs` 우선순위 로직

```rust
pub fn run() -> Result<()> {
    let cli = Cli::parse();
    let manager = SessionManager::discover()?;

    let config = MiroConfig::load();
    let theme_name = cli.theme
        .or_else(|| config.theme_name())   // 설정 파일 값
        .unwrap_or(ThemeName::TomorrowNightBlue);  // 내장 기본값

    let theme = Theme::get(theme_name);
    ...
}
```

`cli.theme`이 `Some`이면 설정 파일을 무시하고, `None`이면 설정 파일 값을 사용한다.

### `src/tui.rs` 저장 트리거: `apply_selected_theme()`

```rust
fn apply_selected_theme(&mut self) {
    // 기존: self.theme 교체
    // 추가: MiroConfig::save_theme(selected_theme_name);
}
```

TUI에서 테마를 선택·적용할 때만 저장한다.
`--theme` CLI 오버라이드로 실행한 경우에는 저장하지 않는다.

### 변경 파일 목록

| 파일 | 변경 내용 |
|------|-----------|
| `Cargo.toml` | `toml = "0.8"` 추가 |
| `src/config.rs` | 신규: `MiroConfig` 로드/저장 |
| `src/theme.rs` | `ThemeName::from_cli_id()` 추가 |
| `src/cli.rs` | `theme` 필드를 `Option<ThemeName>`으로 변경, 테스트 갱신 |
| `src/lib.rs` | 우선순위 로직 추가, `mod config` 선언 |
| `src/tui.rs` | `apply_selected_theme()`에 `save_theme()` 호출 추가 |
| `miro-manual.md` | 테마 유지 동작 설명 추가 |

### 우선순위 로직 요약

```
실행 시 테마 결정:
  1. --theme 명시됨         → 그 테마 사용, 저장 안 함
  2. --theme 없음, 설정 있음 → 설정 파일 테마 사용
  3. --theme 없음, 설정 없음 → tomorrow-night-blue 사용

저장 시점:
  - TUI t 메뉴에서 Enter로 테마 적용 시 → ~/.config/miro/config.toml 저장
  - --theme CLI 오버라이드는 저장 안 함
```

### 테스트 계획

#### 단위 테스트

| 테스트 | 위치 |
|--------|------|
| `from_cli_id("dracula")` → `Some(ThemeName::Dracula)` | `theme.rs` |
| `from_cli_id("unknown")` → `None` | `theme.rs` |
| `MiroConfig::load()` — 파일 없을 때 `theme = None` 반환 | `config.rs` |
| `MiroConfig::save_theme()` 후 `load()` — 저장된 값 반환 | `config.rs` |
| `cli.theme`이 `None`일 때 clap 파싱 정상 동작 | `cli.rs` |
| `--theme dracula` 파싱 후 `Some(ThemeName::Dracula)` | `cli.rs` |

#### 수동 검증

1. `miro` 실행 → TUI에서 `dracula` 선택 → 종료 후 재실행 → `dracula`가 적용되어 있는지 확인
2. `miro --theme nord` 실행 → `nord` 적용되는지 확인 (저장 안 됨)
3. 재실행 시 여전히 이전에 저장된 테마 유지 확인
4. `~/.config/miro/config.toml` 파일 삭제 후 재실행 → `tomorrow-night-blue` 기본값 확인
5. `~/.config/miro/config.toml`에 잘못된 theme 값 입력 후 재실행 → 에러 없이 기본값 사용 확인

### 리스크와 대응

| 리스크 | 대응 |
|--------|------|
| `config.toml` 저장 실패 (권한, 디스크) | 에러 전파 없이 조용히 무시. TUI 흐름 유지 |
| 잘못된 `theme` 값이 저장되어 파싱 실패 | `from_cli_id()` 반환값이 `None`이면 기본값 사용 |
| `--theme` 사용 시 설정 파일이 덮어써짐 | `Option<ThemeName>` 분기로 명시/미명시를 구분해 덮어쓰지 않음 |
| 기존 `defaults_to_tomorrow_night_blue` 테스트 깨짐 | 테스트에서 `cli.theme == None`으로 조건 변경 |

### 완료 기준

1. `miro` 실행 후 TUI에서 테마 변경 시 `~/.config/miro/config.toml`이 생성/갱신된다.
2. miro 재실행 시 마지막으로 선택한 테마가 자동 적용된다.
3. `miro --theme <id>` 실행 시 해당 테마가 적용되나 설정 파일은 변경되지 않는다.
4. 설정 파일이 없거나 값이 잘못되어도 에러 없이 `tomorrow-night-blue`로 시작한다.
5. `cargo test` 전체 통과.
6. `miro-manual.md`에 테마 유지 동작 설명이 추가된다.
