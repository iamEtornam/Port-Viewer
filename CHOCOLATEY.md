# Chocolatey Release Setup

`port-viewer` is distributed on Windows through Chocolatey:

- Package name: `port-viewer`
- Installed command: `ports`
- User install command: `choco install port-viewer -y`

See the [official Chocolatey CLI executable guide](https://docs.chocolatey.org/en-us/guides/create/create-cli-package/)
for background on how portable/CLI executable packages work.

## How It Works

The package uses the **CLI executable (portable)** approach from the Chocolatey
docs — no install script is needed. `ports.exe` is embedded directly in the
package. Chocolatey's built-in Shimgen tool automatically creates a `ports`
shim on the system PATH so the command is available everywhere.

When a GitHub release is published, `.github/workflows/chocolatey.yml`:

1. Downloads `ports-windows-x86_64.zip` from the release.
2. Extracts `ports.exe` from the zip.
3. Renders the package (nuspec + embedded exe + LICENSE) via
   `scripts/render-chocolatey-package.sh`.
4. Runs `choco pack` to produce the `.nupkg`.
5. Runs `choco push` to publish to [chocolatey.org](https://chocolatey.org/).

## One-Time Setup

Before the workflow can publish:

1. Create an account at [chocolatey.org](https://community.chocolatey.org/).
2. Go to **Account → API Key** and copy your key.
3. Add it to this repository as a GitHub Actions secret named
   `CHOCOLATEY_API_KEY`.
4. Push the first package and wait for Chocolatey moderation to approve it
   (usually within a few hours to a day for new packages).

After approval, every subsequent release publishes automatically and
`choco install port-viewer -y` will work for all Windows users.

## Test Locally

```powershell
# 1. Download the Windows zip from a release
$tag = "v0.2.1"
Invoke-WebRequest `
  "https://github.com/iamEtornam/port-viewer/releases/download/$tag/ports-windows-x86_64.zip" `
  -OutFile ports-windows-x86_64.zip

# 2. Extract ports.exe
Expand-Archive ports-windows-x86_64.zip extracted

# 3. Render the package (in Git Bash / WSL)
bash ./scripts/render-chocolatey-package.sh 0.2.1 extracted/ports.exe

# 4. Pack
choco pack dist\chocolatey\port-viewer\port-viewer.nuspec

# 5. Test install from local source
choco install port-viewer --source . -y
ports --version

# 6. Test uninstall
choco uninstall port-viewer -y
```

## Notes

- Package name is `port-viewer`; installed command is `ports`. Both names are
  surfaced in the nuspec `<summary>` and `<description>` so users can find it.
- The package targets the `x86_64` Windows release asset. Chocolatey's Shimgen
  handles the PATH shim — no PowerShell install script is needed.
- First-time submissions to the Chocolatey Community Repository go through
  moderation before they become publicly installable.
