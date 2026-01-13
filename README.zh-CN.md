# git-sweep-rs

[English](README.en.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [हिन्दी](README.hi.md) · [Français](README.fr.md) · [Русский](README.ru.md)

> ⚠️ **警告：使用前请务必备份仓库**
>
> 请在使用本工具前先备份你的仓库（例如将所有重要本地分支推送到远程或创建仓库备份）。此工具会删除分支，删除操作不可逆。首次使用时先运行 `git-sweep-rs preview` 检查将被删除的分支，确认无误后再执行 `cleanup` 或 `cleanup-local`。如需跳过确认可使用 `--force`（慎用）。

一个用 Rust 编写的 Git 分支清理工具，帮助你快速清理已合并的远程和本地分支。

## ✨ 特性

- 🚀 **高性能**：使用 Rust 编写，运行快速
- 🔍 **安全预览**：删除前可以先预览要清理的分支
- 🌐 **远程分支清理**：清理已合并到主分支的远程分支
- 💻 **本地分支清理**：清理已合并到主分支的本地分支
- 🎯 **灵活配置**：支持自定义主分支名称、跳过特定分支
- ✅ **交互确认**：删除前会要求确认（可使用 `--force` 跳过）

## 📦 安装

### 从源码编译

确保你已经安装了 [Rust](https://www.rust-lang.org/tools/install)，然后运行：

```bash
git clone https://github.com/leo-yi/git-sweep-rs
cd git-sweep-rs
cargo build --release
```

### 安装到系统

**方法 1：使用 cargo install**

```bash
cargo install --path .
```

**方法 2：手动复制**

```bash
sudo cp target/release/git-sweep-rs /usr/local/bin/
```

## 🚀 使用方法

### 远程分支操作

#### 预览要删除的远程分支

```bash
git-sweep-rs preview --origin origin --master master
```

#### 删除远程分支

```bash
# 删除前会要求确认
git-sweep-rs cleanup --origin origin --master master

# 强制删除，不需要确认
git-sweep-rs cleanup --force --origin origin --master master
```

### 本地分支操作

#### 预览要删除的本地分支

```bash
git-sweep-rs preview-local --master master
```

#### 删除本地分支

```bash
# 删除前会要求确认
git-sweep-rs cleanup-local --master master

# 强制删除，不需要确认
git-sweep-rs cleanup-local --force --master master
```

## 📖 命令说明

### 命令列表

| 命令 | 说明 | 操作对象 |
|------|------|---------|
| `preview` | 预览要删除的远程分支 | 远程分支 |
| `cleanup` | 删除已合并的远程分支 | 远程分支 |
| `preview-local` | 预览要删除的本地分支 | 本地分支 |
| `cleanup-local` | 删除已合并的本地分支 | 本地分支 |

### 通用参数

#### preview / cleanup 命令

- `--origin <ORIGIN>`: 指定远程仓库名称（默认：`origin`）
- `--master <MASTER>`: 指定主分支名称（默认：`master`）
- `--nofetch`: 不从远程获取最新数据
- `--skip <BRANCHES>`: 跳过指定分支，多个分支用逗号分隔
- `--force`: 强制删除，不需要确认（仅 cleanup 命令）

#### preview-local / cleanup-local 命令

- `--master <MASTER>`: 指定主分支名称（默认：`master`）
- `--skip <BRANCHES>`: 跳过指定分支，多个分支用逗号分隔
- `--force`: 强制删除，不需要确认（仅 cleanup-local 命令）

## 💡 使用示例

### 场景 1：清理远程分支

```bash
# 1. 先预览哪些分支会被删除
git-sweep-rs preview --origin origin --master main

# 2. 确认无误后执行删除
git-sweep-rs cleanup --origin origin --master main
```

### 场景 2：清理本地分支

```bash
# 1. 先预览哪些本地分支会被删除
git-sweep-rs preview-local --master main

# 2. 确认无误后执行删除
git-sweep-rs cleanup-local --master main
```

### 场景 3：跳过特定分支

```bash
# 跳过 develop 和 staging 分支
git-sweep-rs preview --master main --skip "develop,staging"
git-sweep-rs cleanup --master main --skip "develop,staging"
```

### 场景 4：不获取远程更新

```bash
# 使用本地缓存的远程分支信息
git-sweep-rs preview --nofetch
git-sweep-rs cleanup --nofetch
```

## ⚠️ 注意事项

1. **远程分支删除是危险操作**
   - 建议先使用 `preview` 命令查看将要删除的分支
   - 删除的远程分支无法简单恢复
   - 其他团队成员需要运行 `git fetch --prune` 来同步

2. **本地分支删除**
   - 使用 `git branch -d` 安全删除，未完全合并的分支会拒绝删除
   - 如果确实需要删除未合并的分支，请使用 `git branch -D` 手动删除

3. **主分支名称**
   - 很多项目使用 `main` 而不是 `master` 作为主分支
   - 请根据你的项目实际情况使用 `--master main` 参数

## 🔧 工作原理

### 远程分支清理

1. 从远程仓库获取最新信息（除非使用 `--nofetch`）
2. 使用 `git for-each-ref` 列出所有远程分支
3. 使用 `git cherry` 检查每个分支是否已完全合并到主分支
4. 列出所有已合并的分支
5. 使用 `git push origin :branch` 删除远程分支

### 本地分支清理

1. 使用 `git branch --merged` 列出所有已合并的本地分支
2. 排除主分支本身和跳过列表中的分支
3. 列出所有已合并的分支
4. 使用 `git branch -d` 安全删除本地分支

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📝 许可证

MIT License

## 🙏 致谢

本项目受到 Python 版本 [git-sweep](https://github.com/arc90/git-sweep) 的启发。

## 📮 反馈

如果你在使用过程中遇到任何问题或有改进建议，欢迎：

- 提交 [Issue](https://github.com/leo-yi/git-sweep-rs/issues)
- 发起 [Pull Request](https://github.com/leo-yi/git-sweep-rs/pulls)

---

**⭐ 如果这个项目对你有帮助，欢迎 star！**

