# git-sweep-rs

[English](README.en.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [हिन्दी](README.hi.md) · [Français](README.fr.md) · [Русский](README.ru.md)

> ⚠️ **警告：使用前請務必備份倉庫**
>
> 請在使用本工具前先備份你的倉庫（例如將所有重要本地分支推送到遠端或建立倉庫備份）。此工具會刪除分支，刪除操作不可逆。首次使用時先執行 `git-sweep-rs preview` 檢查將被刪除的分支，確認無誤後再執行 `cleanup` 或 `cleanup-local`。如需跳過確認可使用 `--force`（慎用）。

一個使用 Rust 編寫的 Git 分支清理工具，幫助你快速清理已合併到主分支的遠端和本地分支。

## ✨ 特性

- 🚀 **高效能**：使用 Rust 編寫，執行快速
- 🔍 **安全預覽**：刪除前可以先預覽要清理的分支
- 🌐 **遠端分支清理**：清理已合併到主分支的遠端分支
- 💻 **本地分支清理**：清理已合併到主分支的本地分支
- 🎯 **彈性配置**：支援自訂主分支名稱、跳過特定分支
- ✅ **互動確認**：刪除前會要求確認（可使用 `--force` 跳過）

## 📦 安裝

### 從原始碼編譯

確保你已安裝 [Rust](https://www.rust-lang.org/tools/install)，然後執行：

```bash
git clone https://github.com/leo-yi/git-sweep-rs
cd git-sweep-rs
cargo build --release
```

### 安裝到系統

**方法 1：使用 cargo install**

```bash
cargo install --path .
```

**方法 2：手動複製**

```bash
sudo cp target/release/git-sweep-rs /usr/local/bin/
```

## 🚀 使用方法

### 遠端分支操作

#### 預覽要刪除的遠端分支

```bash
git-sweep-rs preview --origin origin --master master
```

#### 刪除遠端分支

```bash
# 刪除前會要求確認
git-sweep-rs cleanup --origin origin --master master

# 強制刪除，不需要確認
git-sweep-rs cleanup --force --origin origin --master master
```

### 本地分支操作

#### 預覽要刪除的本地分支

```bash
git-sweep-rs preview-local --master master
```

#### 刪除本地分支

```bash
# 刪除前會要求確認
git-sweep-rs cleanup-local --master master

# 強制刪除，不需要確認
git-sweep-rs cleanup-local --force --master master
```

## 📖 命令說明

### 命令列表

| 命令 | 說明 | 操作對象 |
|------|------|---------|
| `preview` | 預覽要刪除的遠端分支 | 遠端分支 |
| `cleanup` | 刪除已合併的遠端分支 | 遠端分支 |
| `preview-local` | 預覽要刪除的本地分支 | 本地分支 |
| `cleanup-local` | 刪除已合併的本地分支 | 本地分支 |

### 通用參數

#### preview / cleanup 命令

- `--origin <ORIGIN>`: 指定遠端倉庫名稱（預設：`origin`）
- `--master <MASTER>`: 指定主分支名稱（預設：`master`）
- `--nofetch`: 不從遠端取得最新資料
- `--skip <BRANCHES>`: 跳過指定分支，使用逗號分隔多個分支
- `--force`: 強制刪除，不需要確認（僅限 cleanup 命令）

#### preview-local / cleanup-local 命令

- `--master <MASTER>`: 指定主分支名稱（預設：`master`）
- `--skip <BRANCHES>`: 跳過指定分支，使用逗號分隔多個分支
- `--force`: 強制刪除，不需要確認（僅限 cleanup-local 命令）

## 💡 使用範例

### 範例 1：清理遠端分支

```bash
# 1. 先預覽哪些分支會被刪除
git-sweep-rs preview --origin origin --master main

# 2. 確認無誤後執行刪除
git-sweep-rs cleanup --origin origin --master main
```

### 範例 2：清理本地分支

```bash
# 1. 先預覽哪些本地分支會被刪除
git-sweep-rs preview-local --master main

# 2. 確認無誤後執行刪除
git-sweep-rs cleanup-local --master main
```

### 範例 3：跳過特定分支

```bash
# 跳過 develop 和 staging 分支
git-sweep-rs preview --master main --skip "develop,staging"
git-sweep-rs cleanup --master main --skip "develop,staging"
```

### 範例 4：不取得遠端更新

```bash
# 使用本地快取的遠端分支資訊
git-sweep-rs preview --nofetch
git-sweep-rs cleanup --nofetch
```

## ⚠️ 注意事項

1. **遠端分支刪除為危險操作**
   - 建議先使用 `preview` 命令檢查要刪除的分支
   - 刪除的遠端分支難以恢復
   - 其他團隊成員請執行 `git fetch --prune` 以同步

2. **本地分支刪除**
   - 使用 `git branch -d` 可安全刪除；未合併分支會拒絕刪除
   - 如需強制刪除未合併分支，請手動使用 `git branch -D`

3. **主分支名稱**
   - 許多專案使用 `main` 而非 `master` 作為主分支
   - 請使用 `--master main` 以符合你的專案

## 🔧 工作原理

### 遠端分支清理

1. 從遠端倉庫取得最新資訊（除非使用 `--nofetch`）
2. 使用 `git for-each-ref` 列出所有遠端分支
3. 使用 `git cherry` 檢查每個分支是否已合併到主分支
4. 列出已合併分支
5. 使用 `git push origin :branch` 刪除遠端分支

### 本地分支清理

1. 使用 `git branch --merged` 列出已合併的本地分支
2. 排除主分支與跳過列表中的分支
3. 列出已合併的分支
4. 使用 `git branch -d` 安全刪除本地分支

## 🤝 貢獻

歡迎提交 Issue 與 Pull Request！

## 📝 授權

MIT License

## 🙏 致謝

本專案受到 Python 版本 [git-sweep](https://github.com/arc90/git-sweep) 的啟發。

## 📮 回饋

如果你在使用過程中遇到任何問題或有改進建議，歡迎：

- 提交 [Issue](https://github.com/leo-yi/git-sweep-rs/issues)
- 發起 [Pull Request](https://github.com/leo-yi/git-sweep-rs/pulls)

---

**⭐ 如果這個專案對你有幫助，歡迎 star！**

