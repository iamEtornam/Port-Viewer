#!/usr/bin/env bash
#
# Render a Chocolatey CLI executable package for port-viewer.
#
# Following the official "CLI Executable Package" guide:
# https://docs.chocolatey.org/en-us/guides/create/create-cli-package/
#
# The package embeds ports.exe directly. Chocolatey's built-in Shimgen
# tool automatically creates a PATH shim — no install script required.
#
# Usage: render-chocolatey-package.sh <version> <path-to-ports.exe> [output_dir]

set -euo pipefail

if [[ $# -lt 2 || $# -gt 3 ]]; then
  echo "Usage: $0 <version> <path-to-ports.exe> [output_dir]" >&2
  exit 1
fi

version="$1"
ports_exe="$2"
output_dir="${3:-dist/chocolatey/port-viewer}"
tools_dir="${output_dir}/tools"

if [[ ! -f "$ports_exe" ]]; then
  echo "Error: ports.exe not found at: $ports_exe" >&2
  exit 1
fi

mkdir -p "$tools_dir"

cp "$ports_exe" "${tools_dir}/ports.exe"
cp LICENSE "${tools_dir}/LICENSE.txt"

cat > "${output_dir}/port-viewer.nuspec" <<EOF
<?xml version="1.0" encoding="utf-8"?>
<package xmlns="http://schemas.microsoft.com/packaging/2015/06/nuspec.xsd">
  <metadata>
    <id>port-viewer</id>
    <version>${version}</version>
    <title>port-viewer</title>
    <authors>Etornam</authors>
    <owners>Etornam</owners>
    <projectUrl>https://github.com/iamEtornam/port-viewer</projectUrl>
    <packageSourceUrl>https://github.com/iamEtornam/port-viewer</packageSourceUrl>
    <licenseUrl>https://github.com/iamEtornam/port-viewer/blob/main/LICENSE</licenseUrl>
    <requireLicenseAcceptance>false</requireLicenseAcceptance>
    <projectSourceUrl>https://github.com/iamEtornam/port-viewer</projectSourceUrl>
    <docsUrl>https://github.com/iamEtornam/port-viewer/blob/main/README.md</docsUrl>
    <bugTrackerUrl>https://github.com/iamEtornam/port-viewer/issues</bugTrackerUrl>
    <tags>ports networking process developer-tools cli rust portable</tags>
    <summary>Inspect and manage processes listening on local ports.</summary>
    <description>A fast, beautiful CLI for inspecting and managing processes listening on
your machine's ports. Detects frameworks, maps Docker ports, shows CPU and memory
usage, and lets you interactively kill processes. Command: ports</description>
    <releaseNotes>https://github.com/iamEtornam/port-viewer/releases/tag/v${version}</releaseNotes>
  </metadata>
  <files>
    <file src="tools\ports.exe" target="tools\ports.exe" />
    <file src="tools\LICENSE.txt" target="tools\LICENSE.txt" />
  </files>
</package>
EOF

echo "Package rendered at: ${output_dir}"
echo "  ${output_dir}/port-viewer.nuspec"
echo "  ${tools_dir}/ports.exe"
echo "  ${tools_dir}/LICENSE.txt"
echo ""
echo "Shimgen will auto-create a 'ports' shim — no install script needed."
