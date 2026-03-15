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
| `bun` | 빌드/릴리스 자동화 스크립트 런타임 (미설치 시 자동 설치 또는 `sh` 폴백) |
| `sh` | CI 단계별 셸 스크립트 / bun 없는 환경의 릴리스 폴백 |

> **bun 미설치 대응**: `scripts/release.sh`를 `sh` 전용 폴백으로 병행 제공한다.
> `release.ts` 실행 전 bun 존재 여부를 확인하고, 없으면 `npm i -g bun` 으로 자동 설치하거나
> `release.sh`로 자동 전환한다.

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

릴리스 자동화는 `bun` 유무에 따라 두 경로로 분기한다.

```
scripts/
  release.sh       # sh 전용 릴리스 스크립트 (bun 없는 환경, 기본 경로)
  release.ts       # bun 릴리스 스크립트 (bun 설치 환경, 선택 경로)
  install.sh       # 기존 로컬 설치 스크립트 (유지)
```

**실행 진입점 (`scripts/release.sh` 상단 로직):**

```sh
#!/usr/bin/env sh
if command -v bun >/dev/null 2>&1; then
  exec bun run "$(dirname "$0")/release.ts" "$@"
fi
# bun 없으면 sh 구현으로 계속 진행
```

두 스크립트 모두 동일한 작업을 수행한다:
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

Tap 포뮬라 갱신은 `scripts/release.sh`(또는 `release.ts`)가 릴리스 후 자동으로 PR을 생성한다.

### 5. 설치 명령 (최종 사용자)

```bash
brew tap lovecat/tap
brew install miro
```

또는 원라이너:

```bash
brew install lovecat/tap/miro
```

---

## 필요한 작업 목록 (구현 보류)

- [ ] `.github/workflows/release.yml` 작성
- [ ] `scripts/release.sh` 작성 (sh 기반, bun 폴백 포함)
- [ ] `scripts/release.ts` 작성 (bun 기반, 선택)
- [ ] `lovecat/homebrew-tap` 저장소 생성
- [ ] `Formula/miro.rb` 초기 포뮬라 작성
- [ ] `Cargo.toml` 버전 bump 프로세스 문서화
- [ ] 첫 릴리스 태그 `v0.1.0` 생성 및 검증

---

## 참고

- 현재 로컬 설치: `scripts/install-miro-global.sh`
- 바이너리 경로: `target/release/miro`
- GitHub 저장소: `https://github.com/dambyworld/miro`
