# Homebrew Release Setup

`port-viewer` is distributed through a custom Homebrew tap:

- Tap repo: [`iamEtornam/homebrew-tap`](https://github.com/iamEtornam/homebrew-tap) ✅ live
- Formula name: `port-viewer`
- Installed command: `ports`

Users can install right now:

```bash
brew install iamEtornam/tap/port-viewer
```

## How It Works

When a GitHub release is published, `.github/workflows/homebrew-tap.yml`:

1. Downloads the tagged source tarball from GitHub.
2. Computes the SHA-256 checksum.
3. Renders `Formula/port-viewer.rb` via `scripts/render-homebrew-formula.sh`.
4. Commits and pushes the updated formula to `iamEtornam/homebrew-tap`.

The formula builds `port-viewer` from source using Rust (Cargo). No bottles
are required for the initial release — Homebrew compiles on the user's machine.

## One-Time Setup Required

One secret must be added before the automation works:

| Secret | Scope | Purpose |
|--------|-------|---------|
| `HOMEBREW_TAP_TOKEN` | This repo → Actions secrets | Allows the workflow to push formula updates to `iamEtornam/homebrew-tap` |

**To create the token:**

1. Go to GitHub → Settings → Developer settings → Fine-grained personal access tokens.
2. Create a token scoped to `iamEtornam/homebrew-tap` with **Contents: Read and Write**.
3. Add it to this repository: Settings → Secrets and variables → Actions →
   **New repository secret** → name `HOMEBREW_TAP_TOKEN`.

After this, every tagged release automatically updates the formula.

## Manual Formula Render

To preview a formula locally before a release:

```bash
./scripts/render-homebrew-formula.sh v0.2.1 <sha256>
```

To compute the SHA-256 of a source tarball:

```bash
curl -fsSL \
  https://github.com/iamEtornam/port-viewer/archive/refs/tags/v0.2.1.tar.gz \
  -o source.tar.gz \
  && sha256sum source.tar.gz
```

## Notes

- The formula builds from the GitHub source tarball, not the pre-built binary
  release assets. This is intentional — it follows Homebrew conventions and
  lets users audit what they install.
- If you want faster installs (pre-built bottles), that can be added later
  without changing the formula name or tap structure.
