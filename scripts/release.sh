#!/usr/bin/env sh
# release.sh — miro 로컬 릴리스 자동화 스크립트
#
# 용도: 로컬에서 GitHub Release를 수동으로 생성할 때 사용한다.
#       (GitHub Actions가 정상 동작하면 이 스크립트는 불필요)
#
# 사전 조건:
#   - gh CLI 설치 및 인증 완료 (gh auth login)
#   - 릴리스 태그가 이미 push된 상태 (git push origin --tags)
#
# 사용법:
#   ./scripts/release.sh <version>
#   ./scripts/release.sh 0.1.0

set -e

VERSION="${1:-}"
if [ -z "$VERSION" ]; then
  # Cargo.toml에서 버전 자동 추출
  VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/.*= *"\(.*\)"/\1/')
fi

echo "[release] miro v${VERSION} 릴리스 시작"

TAG="v${VERSION}"
ARM_TARGET="aarch64-apple-darwin"
X86_TARGET="x86_64-apple-darwin"
ARM_ARCHIVE="miro-${VERSION}-${ARM_TARGET}.tar.gz"
X86_ARCHIVE="miro-${VERSION}-${X86_TARGET}.tar.gz"

# ── 1. 빌드 ──────────────────────────────────────────────
echo "[release] 크로스 컴파일 중..."
rustup target add "${ARM_TARGET}" "${X86_TARGET}"
cargo build --release --target "${ARM_TARGET}"
cargo build --release --target "${X86_TARGET}"

# ── 2. 아카이브 생성 ──────────────────────────────────────
echo "[release] tar.gz 아카이브 생성 중..."
tar -czf "${ARM_ARCHIVE}" -C "target/${ARM_TARGET}/release" miro
tar -czf "${X86_ARCHIVE}" -C "target/${X86_TARGET}/release" miro

# ── 3. SHA256 체크섬 ──────────────────────────────────────
echo "[release] SHA256 체크섬 계산 중..."
shasum -a 256 "${ARM_ARCHIVE}" "${X86_ARCHIVE}" > checksums.txt
cat checksums.txt

ARM_SHA=$(grep "${ARM_TARGET}" checksums.txt | awk '{print $1}')
X86_SHA=$(grep "${X86_TARGET}" checksums.txt | awk '{print $1}')

# ── 4. GitHub Release 생성 ────────────────────────────────
echo "[release] GitHub Release 생성 중..."
gh release create "${TAG}" \
  --title "miro ${TAG}" \
  --notes "## miro ${TAG}

### 설치

\`\`\`bash
brew install Ruska-Zone/tap/miro
\`\`\`

### 업그레이드

\`\`\`bash
brew upgrade Ruska-Zone/tap/miro
\`\`\`" \
  "${ARM_ARCHIVE}" "${X86_ARCHIVE}" checksums.txt

echo "[release] GitHub Release 완료: https://github.com/dambyworld/miro/releases/tag/${TAG}"

# ── 5. Homebrew Tap 포뮬라 업데이트 안내 ──────────────────
echo ""
echo "[release] ──────────────────────────────────────────────────"
echo "[release] Homebrew Tap 포뮬라를 수동으로 업데이트하세요."
echo "[release] 저장소: Ruska-Zone/homebrew-tap"
echo "[release] 파일:   Formula/miro.rb"
echo ""
echo "[release] aarch64 SHA256: ${ARM_SHA}"
echo "[release] x86_64  SHA256: ${X86_SHA}"
echo "[release] version:        ${VERSION}"
echo "[release] ──────────────────────────────────────────────────"

# ── 6. 임시 파일 정리 ─────────────────────────────────────
rm -f "${ARM_ARCHIVE}" "${X86_ARCHIVE}" checksums.txt
echo "[release] 완료"
