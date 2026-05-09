# Homebrew formula for TokenLens.
# Auto-published by the v0.2.0 release. To bump: regenerate from
# https://github.com/sisodiabhumca/tokenlens/releases.

class Tokenlens < Formula
  desc "Token-aware observability + filtering layer for AI coding agents"
  homepage "https://github.com/sisodiabhumca/tokenlens"
  version "0.2.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/sisodiabhumca/tokenlens/releases/download/v#{version}/tokenlens-aarch64-apple-darwin.tar.gz"
      sha256 "ce7db82a7e0fdfeb6721a05fcb444e10270e03488e0e0bfae5b66d9c51c37ebd"
    end
    on_intel do
      url "https://github.com/sisodiabhumca/tokenlens/releases/download/v#{version}/tokenlens-x86_64-apple-darwin.tar.gz"
      sha256 "5a8cf84c5b5cbba75c9adab38548d0305b359d5e62852bbde76f058d5af8078e"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/sisodiabhumca/tokenlens/releases/download/v#{version}/tokenlens-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "8613cb2e3b5e4a7ff2cff143aaa0561d16f2bde19fd9098c91e1ec3462aac6e5"
    end
    on_intel do
      url "https://github.com/sisodiabhumca/tokenlens/releases/download/v#{version}/tokenlens-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "9138e5ce780417d82e40cdf614e3cd76a719bb59aeef02027bbdfd4cf3d8caab"
    end
  end

  def install
    bin.install "tokenlens"
    (share/"tokenlens/hooks").install Dir["hooks/*"] if Dir.exist?("hooks")
    doc.install "README.md" if File.exist?("README.md")
  end

  test do
    assert_match "tokenlens", shell_output("#{bin}/tokenlens --version")
  end
end
