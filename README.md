# git-sweep-rs

[简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [हिन्दी](README.hi.md) · [Français](README.fr.md) · [Русский](README.ru.md)

> ⚠️ **Warning: Back up your repository before use**
>
> Please back up your repository before using this tool (for example, push important local branches to a remote or create a repository backup). This tool deletes branches and deletions are irreversible. For first-time use, run `git-sweep-rs preview` to inspect branches that would be removed, and only run `cleanup` or `cleanup-local` after confirming. Use `--force` to skip confirmations (use with caution).

A Git branch cleanup tool written in Rust to help you quickly remove branches that have been merged into your main branch, both remote and local.

## ✨ Features

- 🚀 High performance: written in Rust for speed
- 🔍 Safe preview: preview branches to be deleted before removing
- 🌐 Remote cleanup: remove remote branches merged into main
- 💻 Local cleanup: remove local branches merged into main
- 🎯 Flexible: configurable main branch name and skip list
- ✅ Interactive confirmation: asks for confirmation before deleting (use `--force` to skip)

## 📦 Installation

### Build from source

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed, then:

```bash
git clone https://github.com/leo-yi/git-sweep-rs
cd git-sweep-rs
cargo build --release
```

### Install to system

**Method 1: Use cargo install**

```bash
cargo install --path .
```

**Method 2: Manual copy**

```bash
sudo cp target/release/git-sweep-rs /usr/local/bin/
```

## 🚀 Usage

### Remote branches

#### Preview remote branches to be deleted

```bash
git-sweep-rs preview --origin origin --master master
```

#### Delete remote branches

```bash
# will ask for confirmation before deleting
git-sweep-rs cleanup --origin origin --master master

# force delete without confirmation
git-sweep-rs cleanup --force --origin origin --master master
```

### Local branches

#### Preview local branches to be deleted

```bash
git-sweep-rs preview-local --master master
```

#### Delete local branches

```bash
# will ask for confirmation before deleting
git-sweep-rs cleanup-local --master master

# force delete without confirmation
git-sweep-rs cleanup-local --force --master master
```

## 📖 Commands

| Command | Description | Target |
|--------:|:-----------|:------|
| `preview` | Preview remote branches to be deleted | remote branches |
| `cleanup` | Delete merged remote branches | remote branches |
| `preview-local` | Preview local branches to be deleted | local branches |
| `cleanup-local` | Delete merged local branches | local branches |

### Common options

#### preview / cleanup

- `--origin <ORIGIN>`: remote name (default: `origin`)
- `--master <MASTER>`: main branch name (default: `master`)
- `--nofetch`: do not fetch updates from remote
- `--skip <BRANCHES>`: comma-separated list of branches to skip
- `--force`: force delete without confirmation (cleanup only)

#### preview-local / cleanup-local

- `--master <MASTER>`: main branch name (default: `master`)
- `--skip <BRANCHES>`: comma-separated list of branches to skip
- `--force`: force delete without confirmation (cleanup-local only)

## 💡 Examples

### Scenario 1: Clean remote branches

```bash
# 1. Preview branches to be deleted
git-sweep-rs preview --origin origin --master main

# 2. Confirm and delete
git-sweep-rs cleanup --origin origin --master main
```

### Scenario 2: Clean local branches

```bash
# 1. Preview local branches to be deleted
git-sweep-rs preview-local --master main

# 2. Confirm and delete
git-sweep-rs cleanup-local --master main
```

### Scenario 3: Skip branches

```bash
# skip develop and staging
git-sweep-rs preview --master main --skip "develop,staging"
git-sweep-rs cleanup --master main --skip "develop,staging"
```

### Scenario 4: Use cached remote info

```bash
# use local cached remote branch info
git-sweep-rs preview --nofetch
git-sweep-rs cleanup --nofetch
```

## ⚠️ Notes

1. Deleting remote branches is a destructive action
   - Use `preview` to inspect branches first
   - Deleted remote branches are not easily recoverable
   - Other collaborators should run `git fetch --prune` to sync

2. Deleting local branches
   - Use `git branch -d` for safe deletion; it refuses for unmerged branches
   - Use `git branch -D` to force-delete unmerged branches manually

3. Main branch name
   - Many repos use `main` instead of `master`
   - Use `--master main` to match your repo

## 🔧 How it works

### Remote cleanup

1. Fetch remote updates (unless `--nofetch`)
2. List remote branches with `git for-each-ref`
3. Use `git cherry` to check if a branch is fully merged into main
4. List branches that are merged
5. Delete remote branch with `git push origin :branch`

### Local cleanup

1. List merged local branches with `git branch --merged`
2. Exclude the main branch and any skip list
3. List branches that are merged
4. Delete local branch with `git branch -d`

## 🤝 Contributing

Issues and PRs are welcome!

## 📝 License

MIT License

## 🙏 Acknowledgements

Inspired by the Python project [git-sweep](https://github.com/arc90/git-sweep).

## 📮 Feedback

If you run into issues or have suggestions:

- Open an [Issue](https://github.com/leo-yi/git-sweep-rs/issues)
- Create a [Pull Request](https://github.com/leo-yi/git-sweep-rs/pulls)

---

⭐ If this project helps you, please star!

