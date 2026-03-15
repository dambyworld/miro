#!/usr/bin/env python3
"""포뮬라 파일 생성 스크립트 (GitHub Actions update-tap job에서 호출)"""
import os

version = os.environ["MIRO_VERSION"]
arm_sha = os.environ["ARM_SHA256"]
x86_sha = os.environ["X86_SHA256"]
output  = os.environ.get("OUTPUT_FILE", "homebrew-tap/Formula/miro.rb")

formula = (
    "class Miro < Formula\n"
    '  desc "Terminal TUI for managing codex/claude-code sessions"\n'
    '  homepage "https://github.com/dambyworld/miro"\n'
    f'  version "{version}"\n'
    "\n"
    "  on_macos do\n"
    "    if Hardware::CPU.arm?\n"
    f'      url "https://github.com/dambyworld/miro/releases/download/v{version}/miro-{version}-aarch64-apple-darwin.tar.gz"\n'
    f'      sha256 "{arm_sha}"\n'
    "    else\n"
    f'      url "https://github.com/dambyworld/miro/releases/download/v{version}/miro-{version}-x86_64-apple-darwin.tar.gz"\n'
    f'      sha256 "{x86_sha}"\n'
    "    end\n"
    "  end\n"
    "\n"
    "  def install\n"
    '    bin.install "miro"\n'
    "  end\n"
    "\n"
    "  test do\n"
    '    system "#{bin}/miro", "--version"\n'
    "  end\n"
    "end\n"
)

os.makedirs(os.path.dirname(output) or ".", exist_ok=True)
with open(output, "w") as f:
    f.write(formula)

print(f"Formula written → {output}")
print(f"  version : {version}")
print(f"  arm sha : {arm_sha}")
print(f"  x86 sha : {x86_sha}")
