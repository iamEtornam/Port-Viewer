#!/usr/bin/env bash

set -euo pipefail

if [[ $# -ne 2 ]]; then
  echo "Usage: $0 <tag> <sha256>" >&2
  exit 1
fi

tag="$1"
sha256="$2"
version="${tag#v}"

cat <<EOF
class PortViewer < Formula
  desc "Inspect and manage processes listening on local ports"
  homepage "https://github.com/iamEtornam/port-viewer"
  url "https://github.com/iamEtornam/port-viewer/archive/refs/tags/${tag}.tar.gz"
  sha256 "${sha256}"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args(path: ".")
  end

  test do
    assert_match "${version}", shell_output("#{bin}/ports --version")
  end
end
EOF
