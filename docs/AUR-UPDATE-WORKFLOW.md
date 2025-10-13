# AUR Update Workflow

## Quick Update Process

When you're ready to release a new version:

### 1. Update version in your project

```bash
cd ~/code/hyprmarker

# Edit Cargo.toml - update version
vim Cargo.toml

# Commit changes
git add Cargo.toml
git commit -m "Bump version to 0.2.0"
git push origin master
```

### 2. Run the automated AUR update script

```bash
cd ~/code/hyprmarker
./tools/update-aur.sh
```

**What it does:**
1. ✅ Reads version from `Cargo.toml`
2. ✅ Creates git tag (e.g., `v0.2.0`) and pushes to GitHub
3. ✅ Copies `packaging/PKGBUILD.aur` to `~/aur-packages/PKGBUILD`
4. ✅ Updates `pkgver` and resets `pkgrel` to 1
5. ✅ Generates `.SRCINFO`
6. ✅ Optionally tests build locally
7. ✅ Commits and pushes to AUR

**Done!** Users can now update with `yay -Syu hyprmarker`

---

## Manual Update Process

If you prefer to do it manually:

```bash
# 1. Update version in Cargo.toml
cd ~/code/hyprmarker
vim Cargo.toml  # Change version to 0.2.0

# 2. Commit and tag
git add Cargo.toml
git commit -m "Bump version to 0.2.0"
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin master --tags

# 3. Update AUR PKGBUILD
cd ~/aur-packages
cp ~/code/hyprmarker/packaging/PKGBUILD.aur ./PKGBUILD
vim PKGBUILD  # Update pkgver=0.2.0, pkgrel=1

# 4. Regenerate .SRCINFO
makepkg --printsrcinfo > .SRCINFO

# 5. Test build (optional but recommended)
makepkg -si

# 6. Commit and push to AUR
git add PKGBUILD .SRCINFO
git commit -m "Update to v0.2.0"
git push origin master
```

---

## Version Numbering

### Major releases (breaking changes)
```
0.1.0 → 1.0.0
pkgver=1.0.0
pkgrel=1
```

### Minor releases (new features)
```
0.1.0 → 0.2.0
pkgver=0.2.0
pkgrel=1
```

### Patch releases (bug fixes)
```
0.1.0 → 0.1.1
pkgver=0.1.1
pkgrel=1
```

### PKGBUILD-only updates (no code changes)
```
Same version: 0.1.0
pkgver=0.1.0
pkgrel=2  ← Increment this
```

---

## Troubleshooting

### "Tag already exists"
```bash
# Delete local tag
git tag -d v0.1.0

# Delete remote tag
git push --delete origin v0.1.0

# Recreate
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

### "Build fails on AUR"
```bash
# Test locally first
cd ~/aur-packages
makepkg -si

# Common issues:
# - Missing dependencies in PKGBUILD
# - GitHub tag doesn't exist
# - Wrong source URL
```

### "Permission denied" when pushing to AUR
```bash
# Check SSH key is configured
cat ~/.ssh/config | grep -A 3 "aur.archlinux.org"

# Should show:
# Host aur.archlinux.org
#   IdentityFile ~/.ssh/aur_ed25519
#   User aur
```

---

## File Locations

| File | Purpose |
|------|---------|
| `~/code/hyprmarker/Cargo.toml` | Source of truth for version |
| `~/code/hyprmarker/packaging/PKGBUILD.aur` | Template for AUR PKGBUILD |
| `~/aur-packages/PKGBUILD` | Actual AUR package file |
| `~/aur-packages/.SRCINFO` | Generated metadata (auto-updated) |

---

## Quick Commands Reference

```bash
# Update AUR (automated)
./tools/update-aur.sh

# Check current version
grep '^version = ' Cargo.toml

# List git tags
git tag

# Check AUR package status
cd ~/aur-packages && git log -1

# Rebuild local package
cd ~/aur-packages && makepkg -si
```
