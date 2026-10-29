# GitFlow on this fork

## Branches

| Branch | Role |
| --- | --- |
| `main` | What we ship. Only merges from `develop` (or a hotfix). A push here **promotes** an existing pre-release; it does not rebuild. |
| `develop` | Integration. Feature branches merge here. A push here **builds** installers and publishes a GitHub **pre-release**. |
| `feature/…` | Day-to-day work. Open PRs against `develop`, not `main`. |
| `hotfix/…` | Urgent fix off `main`. Merge to `main`, then back-merge into `develop`. |

`privacy-lockdown` and other topic branches are feature branches: merge them into `develop`.

## Fork versioning

Upstream tags bare versions (`v2.0.86`). This fork **never** does that.

App version (in `package.json`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`):

```text
{upstream-base}-sj.{N}
```

Examples: `2.0.86-sj.1`, `2.0.86-sj.2`, then after syncing a newer upstream `2.0.87-sj.1`.

Git tag and GitHub Release name: `v` + that string (`v2.0.86-sj.1`).

Bump `-sj.N` when you want a new build identity. Syncing a newer upstream base resets `N` to `1`.

**Windows MSI note:** WiX only allows a numeric fourth version field. The Windows jobs stamp `bundle.windows.wix.version` to `{upstream-base}.{N}` (for example `2.0.86.1`) via `tools/stamp-wix-version.sh` before `tauri build`. The GitHub tag stays `v*-sj.N`.

## Pre-release (push to `develop`)

1. Version files must already be `{base}-sj.{N}`.
2. A push that changes app/build paths (see `release-prerelease.yml` `paths:`) builds the private Windows MSI and the Linux `.deb` / `.rpm`. Docs-only pushes do not.
3. It moves (or creates) the tag `v{version}` onto that commit and publishes a **pre-release** with those assets.

The same `-sj.N` pushed again replaces that pre-release (new commit, same version string). That is for iterating on `develop` before promoting. Run the workflow by hand (`workflow_dispatch`) if you need a rebuild without a path change.

Arch / install-matrix packaging stays in `arch-package.yml` and is **manual** (`workflow_dispatch`), so every `develop` push does not pay for that matrix.

CI (`ci.yml`) runs on **pull requests** (and the merge queue), not again on every push to `develop`/`main`. Each CI job is skipped when its inputs did not change (frontend vs Rust vs platform-boundary).

## Release (merge to `main`) — no rebuild

Promote reuses the **exact** installers already attached to the pre-release:

1. Merge `develop` into `main` (prefer a merge commit or fast-forward so the tagged commit stays on the history; a squash is fine if the tree matches the tagged commit).
2. The promote workflow reads the version from the repo, checks that `v{version}` is a pre-release whose tagged tree matches `main`, then runs:

   ```bash
   gh release edit "v{version}" --prerelease=false --latest
   ```

Same tag, same assets, same bytes you tested. It does **not** run `tauri build` again.

If there is no pre-release for that version, promote fails on purpose: build on `develop` first.

## Manual rebuilds

- Windows MSI only: `package.yml` → *Run workflow*
- Full Linux package matrix (Arch, install checks): `arch-package.yml` → *Run workflow*
