# miro 배포 계획 (miro-release-plan)

## 개요

`miro`는 Rust로 작성된 터미널 TUI 바이너리다.
사용자가 `brew install` 명령 한 줄로 설치할 수 있도록 배포 파이프라인을 구성한다.

> **구현 상태**: 계획 수립 단계 (구현 보류)

---

## 배포 스택

| 도구 | 역할 |
|------|------|
| `brew` | macOS 패키지 배포 채널 (Homebrew Tap) |
| `sh` | CI 단계별 셸 스크립트 및 릴리스 자동화 |

> **기설치 대응**: miro가 이미 설치된 상태에서 재설치 또는 업그레이드하는 경우를 고려한다.
> `brew install`은 이미 설치된 경우 오류를 반환하므로 `brew upgrade` 경로를 안내해야 한다.
> 포뮬라의 `version` 필드가 기존보다 높을 때만 업그레이드가 적용된다.

---

## 배포 흐름

```
1. 버전 태그 생성 (git tag v0.x.x)
        ↓
2. GitHub Actions: cross-compile (macOS arm64 / x86_64)
        ↓
3. GitHub Release: 바이너리 업로드 + SHA256 체크섬 생성
        ↓
4. Homebrew Tap 포뮬라 자동 갱신 (url + sha256 패치)
        ↓
5. brew install lovecat/tap/miro
```

---

## 세부 계획

### 1. 버전 관리

- `Cargo.toml`의 `version` 필드를 단일 진실 공급원(source of truth)으로 사용
- 릴리스 시 `Cargo.toml` 버전 bump → `git tag v{version}` → `git push origin --tags`
- 버전 형식: `MAJOR.MINOR.PATCH` (SemVer)

### 2. 크로스 컴파일 (GitHub Actions)

파일: `.github/workflows/release.yml`

```yaml
on:
  push:
    tags: ["v*"]

jobs:
  build:
    strategy:
      matrix:
        target:
          - aarch64-apple-darwin   # Apple Silicon
          - x86_64-apple-darwin    # Intel Mac
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4
      - run: rustup target add ${{ matrix.target }}
      - run: cargo build --release --target ${{ matrix.target }}
      - run: |
          BINARY=target/${{ matrix.target }}/release/miro
          SHA=$(shasum -a 256 $BINARY | awk '{print $1}')
          echo "sha256=$SHA" >> $GITHUB_OUTPUT
      - uses: actions/upload-artifact@v4
        with:
          name: miro-${{ matrix.target }}
          path: target/${{ matrix.target }}/release/miro
```

### 3. GitHub Release 생성

릴리스 자동화는 `scripts/release.sh` 단일 스크립트로 처리한다.

```
scripts/
  release.sh    # sh 릴리스 자동화 스크립트
  install.sh    # 기존 로컬 설치 스크립트 (유지)
```

`release.sh`가 수행하는 작업:
- 아티팩트 다운로드
- `tar.gz` 아카이브 생성 (`miro-{version}-{target}.tar.gz`)
- SHA256 계산 및 `checksums.txt` 생성
- `gh release create` 호출 후 파일 업로드
- Homebrew Tap 포뮬라 PR 자동 생성

### 4. Homebrew Tap

저장소: `lovecat/homebrew-tap`

포뮬라 파일: `Formula/miro.rb`

```ruby
class Miro < Formula
  desc "Terminal TUI for managing codex/claude-code sessions"
  homepage "https://github.com/dambyworld/miro"
  version "0.1.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/dambyworld/miro/releases/download/v#{version}/miro-#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "<aarch64-sha256>"
    else
      url "https://github.com/dambyworld/miro/releases/download/v#{version}/miro-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "<x86_64-sha256>"
    end
  end

  def install
    bin.install "miro"
  end

  test do
    system "#{bin}/miro", "--version"
  end
end
```

Tap 포뮬라 갱신은 `scripts/release.sh`가 릴리스 후 자동으로 PR을 생성한다.

### 5. 설치 명령 (최종 사용자)

**신규 설치:**

```bash
brew install lovecat/tap/miro
```

**업그레이드 (기설치 상태):**

```bash
brew upgrade lovecat/tap/miro
```

---

## 6. 배포 검증 (시뮬레이션 테스트)

> **배경**: 개발 PC에는 소스 빌드된 miro가 이미 설치되어 있으므로,
> 동일 PC에서 `brew install`로 재설치하거나 덮어써서는 안 된다.
> 별도 격리 환경에서 배포 패키지의 설치·동작을 검증한다.

### 검증 환경

| 방법 | 설명 |
|------|------|
| **macOS VM** | UTM 또는 Parallels로 클린 macOS 인스턴스 구성 |
| **GitHub Actions** | `release.yml`에 install-test job 추가 — 빌드 직후 brew tap/install 실행 |
| **별도 계정** | 로컬 macOS의 다른 사용자 계정(miro 미설치 상태)에서 테스트 |

권장 방법: **GitHub Actions install-test job** (자동화 가능, 매 릴리스마다 실행)

> **격리 환경 여부**: GitHub Actions의 `runs-on: macos-latest`는 매 job마다
> Homebrew 외 사용자 소프트웨어가 없는 새로운 ephemeral VM에서 실행된다.
> 개발 PC와 완전히 독립된 클린 환경이므로 miro 기설치 충돌 없이 검증 가능하다.

### install-test job 계획

```yaml
install-test:
  needs: release        # release job 완료 후 실행
  runs-on: macos-latest
  steps:
    - name: Install via brew tap
      run: |
        brew tap lovecat/tap
        brew install miro

    - name: Smoke test
      run: |
        miro --version
        # TUI 진입 없이 CLI 플래그만 검증

    - name: Upgrade test
      run: |
        # 동일 버전 재설치 시 brew upgrade가 no-op으로 정상 처리되는지 확인
        brew upgrade miro || true
```

### 검증 항목

- [ ] `brew install lovecat/tap/miro` 오류 없이 완료
- [ ] `miro --version` 이 `Cargo.toml`의 버전과 일치
- [ ] 바이너리가 올바른 아키텍처로 빌드됨 (`file $(which miro)`)
- [ ] 기설치 상태에서 `brew upgrade` 정상 동작
- [ ] `brew uninstall miro` 후 재설치 정상 동작
- [ ] `miro` 실행 시 오류 없이 정상 동작 — 아래 기준 스냅샷과 비교 검증

#### 기준 스냅샷 (개발 PC 기준, v0.1.0)

**`miro --version`**
```
miro 0.1.0
```

**`miro --help`**
```
Terminal TUI for Codex and Claude Code sessions

Usage: miro [OPTIONS] [COMMAND]

Commands:
  themes
  list
  resume
  delete
  help    Print this message or the help of the given subcommand(s)

Options:
      --theme <THEME>  [possible values: default, tomorrow-night-blue,
                        cursor-dark, darcula-dark, darcula-light, dracula,
                        nord, one-dark, gruvbox-dark, gruvbox-light,
                        catppuccin-mocha, tokyo-night, solarized-dark,
                        solarized-light]
  -h, --help           Print help
  -V, --version        Print version
```

**`miro themes`** (테마 목록 첫 항목)
```
Tomorrow Night Blue (default) [tomorrow-night-blue]
  Deep blue low-glare theme used as the default
```

**`miro list`** — 세션 목록 출력 형식 예시
```
[claude-code] <uuid>
  <대화 요약>
  cwd: <작업 디렉터리>
  updated: <타임스탬프 UTC>
```

검증 기준: 서브커맨드(`list`, `themes`, `resume`, `delete`) 모두 오류 없이 응답하고,
출력 형식이 위 스냅샷과 동일해야 한다.
---

## 필요한 작업 목록 (구현 보류)

- [ ] `.github/workflows/release.yml` 작성
- [ ] `scripts/release.sh` 작성 (sh 기반)
- [ ] `lovecat/homebrew-tap` 저장소 생성
- [ ] `Formula/miro.rb` 초기 포뮬라 작성
- [ ] `Cargo.toml` 버전 bump 프로세스 문서화
- [ ] 첫 릴리스 태그 `v0.1.0` 생성 및 검증
- [ ] `release.yml`에 install-test job 추가 (배포 검증 자동화)

---

## 참고

- 현재 로컬 설치: `scripts/install-miro-global.sh`
- 바이너리 경로: `target/release/miro`
- GitHub 저장소: `https://github.com/dambyworld/miro`
