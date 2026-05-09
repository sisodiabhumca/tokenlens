# Homebrew formula for TokenLens.
#
# This file is a placeholder authored by hand. Once the v0.2.0 GitHub Release
# is published with the cargo-dist artifacts, regenerate the SHA256s with
# `cargo dist generate` (or `shasum -a 256 *.tar.gz`) and replace the
# REPLACE_WITH_SHA256_FROM_RELEASE strings below before pushing this file
# to the homebrew-tokenlens tap repo.

class Tokenlens < Formula
  desc "Token-aware observability + filtering layer for AI coding agents"
  homepage "https://github.com/sisodiabhumca/tokenlens-scaffold"
  version "0.2.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/sisodiabhumca/tokenlens-scaffold/releases/download/v#{version}/tokenlens-aarch64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_SHA256_FROM_RELEASE"
    end
    on_intel do
      url "https://github.com/sisodiabhumca/tokenlens-scaffold/releases/download/v#{version}/tokenlens-x86_64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_SHA256_FROM_RELEASE"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/sisodiabhumca/tokenlens-scaffold/releases/download/v#{version}/tokenlens-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "REPLACE_WITH_SHA256_FROM_RELEASE"
    end
    on_intel do
      url "https://github.com/sisodiabhumca/tokenlens-scaffold/releases/download/v#{version}/tokenlens-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "REPLACE_WITH_SHA256_FROM_RELEASE"
    end
  end

  def install
    bin.install "tokenlens"
    (share/"tokenlens/hooks").install Dir["hooks/*"] if Dir.exist?("hooks")
    doc.install "README.md"
  end

  test do
    assert_match "tokenlens", shell_output("#{bin}/tokenlens --version")
  end
end
