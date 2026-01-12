use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use anyhow::{anyhow, Context, Result};

/// 确认当前工作目录在一个 Git 仓库中
pub fn ensure_git_repo() -> Result<()> {
    let status = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("failed to execute git")?;

    if !status.success() {
        return Err(anyhow!("This is not a Git repository"));
    }

    Ok(())
}

/// 把 "--skip" 的逗号分隔字符串解析成 Vec<String>
pub fn parse_skips(s: &str) -> Vec<String> {
    s.split(',')
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(String::from)
        .collect()
}

/// 对 preview / cleanup 都通用的逻辑：找到可以删除的分支并打印
pub fn run_sweep(
    dry_run: bool,
    origin: String,
    master: String,
    fetch: bool,
    skips: Vec<String>,
) -> Result<Vec<String>> {
    let workdir = std::env::current_dir().context("failed to get current dir")?;

    if fetch {
        fetch_remote(&workdir, &origin)?;
    }

    let merged = merged_remote_branches(&workdir, &origin, &master, &skips)?;

    if !merged.is_empty() {
        println!(
            "These branches have been merged into {}:\n",
            master
        );
        for b in &merged {
            println!("  {}", b);
        }
    } else {
        println!("No remote branches are available for cleaning up");
    }

    if dry_run && !merged.is_empty() {
        let mut cmdline = String::from("git-sweep cleanup");
        cmdline.push_str(&format!(" --origin {} --master {}", origin, master));
        if !fetch {
            cmdline.push_str(" --nofetch");
        }
        if !skips.is_empty() {
            cmdline.push_str(" --skip ");
            cmdline.push_str(&skips.join(","));
        }

        println!("\nTo delete them, run again with `{}`", cmdline);
    }

    Ok(merged)
}

/// cleanup 模式：在 run_sweep 基础上再执行删除
pub fn run_cleanup(
    force: bool,
    origin: String,
    master: String,
    fetch: bool,
    skips: Vec<String>,
) -> Result<()> {
    let branches = run_sweep(false, origin.clone(), master, fetch, skips)?;

    if branches.is_empty() {
        return Ok(());
    }

    let mut do_delete = force;
    if !force {
        print!("\nDelete these branches? (y/n) ");
        io::stdout().flush().ok();

        let mut answer = String::new();
        io::stdin().read_line(&mut answer)?;
        if answer.trim().to_lowercase().starts_with('y') {
            do_delete = true;
        } else {
            println!("\nOK, aborting.");
        }
    }

    if do_delete {
        let workdir = std::env::current_dir().context("failed to get current dir")?;
        println!();
        for b in &branches {
            print!("  deleting {}", b);
            io::stdout().flush().ok();
            delete_remote_branch(&workdir, &origin, b)?;
            println!(" (done)");
        }

        println!("\nAll done!");
        println!("\nTell everyone to run `git fetch --prune` to sync with this remote.");
        println!("(you don't have to, yours is synced)");
    }

    Ok(())
}

/// cleanup_local 模式：清理已合并到指定分支的本地分支
pub fn run_cleanup_local(
    force: bool,
    master: String,
    skips: Vec<String>,
) -> Result<()> {
    let workdir = std::env::current_dir().context("failed to get current dir")?;
    
    // 获取所有已合并的本地分支
    let merged = merged_local_branches(&workdir, &master, &skips)?;

    if merged.is_empty() {
        println!("No local branches are available for cleaning up");
        return Ok(());
    }

    println!("These local branches have been merged into {}:\n", master);
    for b in &merged {
        println!("  {}", b);
    }

    let mut do_delete = force;
    if !force {
        print!("\nDelete these local branches? (y/n) ");
        io::stdout().flush().ok();

        let mut answer = String::new();
        io::stdin().read_line(&mut answer)?;
        if answer.trim().to_lowercase().starts_with('y') {
            do_delete = true;
        } else {
            println!("\nOK, aborting.");
        }
    }

    if do_delete {
        println!();
        for b in &merged {
            print!("  deleting {}", b);
            io::stdout().flush().ok();
            delete_local_branch(&workdir, b)?;
            println!(" (done)");
        }

        println!("\nAll done!");
    }

    Ok(())
}

/// preview_local 模式：预览已合并到指定分支的本地分支
pub fn run_preview_local(
    master: String,
    skips: Vec<String>,
) -> Result<()> {
    let workdir = std::env::current_dir().context("failed to get current dir")?;
    
    // 获取所有已合并的本地分支
    let merged = merged_local_branches(&workdir, &master, &skips)?;

    if merged.is_empty() {
        println!("No local branches are available for cleaning up");
        return Ok(());
    }

    println!("These local branches have been merged into {}:\n", master);
    for b in &merged {
        println!("  {}", b);
    }

    println!("\nTo delete them, run:");
    let mut cmdline = String::from("git-sweep-rs cleanup-local");
    cmdline.push_str(&format!(" --master {}", master));
    if !skips.is_empty() {
        cmdline.push_str(" --skip ");
        cmdline.push_str(&skips.join(","));
    }
    println!("  {}", cmdline);

    Ok(())
}

/// 对指定 remote 做一次 fetch
fn fetch_remote(workdir: &PathBuf, origin: &str) -> Result<()> {
    println!("Fetching from the remote");
    let status = Command::new("git")
        .arg("fetch")
        .arg(origin)
        .current_dir(workdir)
        .status()
        .context("failed to execute git fetch")?;

    if !status.success() {
        return Err(anyhow!("git fetch {} failed", origin));
    }

    Ok(())
}

/// 列出已经完全合并到 master 的远程分支（不含 HEAD / master / skips）
fn merged_remote_branches(
    workdir: &PathBuf,
    origin: &str,
    master: &str,
    skips: &[String],
) -> Result<Vec<String>> {
    let output = Command::new("git")
        .arg("for-each-ref")
        .arg("--format=%(refname:short)")
        .arg(format!("refs/remotes/{}", origin))
        .current_dir(workdir)
        .output()
        .context("failed to execute git for-each-ref")?;

    if !output.status.success() {
        return Err(anyhow!(
            "git for-each-ref failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let mut candidates = Vec::new();

    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let prefix = format!("{}/", origin);
        if !line.starts_with(&prefix) {
            continue;
        }

        let branch = &line[prefix.len()..];

        if branch == "HEAD" || branch == master {
            continue;
        }
        if skips.iter().any(|s| s == branch) {
            continue;
        }

        candidates.push(branch.to_string());
    }

    let mut merged = Vec::new();

    for branch in candidates {
        let upstream = format!("{}/{}", origin, master);
        let head = format!("{}/{}", origin, &branch);

        let output = Command::new("git")
            .arg("cherry")
            .arg(&upstream)
            .arg(&head)
            .current_dir(workdir)
            .output()
            .with_context(|| format!("failed to run git cherry for branch {}", branch))?;

        if !output.status.success() {
            continue;
        }

        if output.stdout.is_empty() {
            merged.push(branch);
        }
    }

    Ok(merged)
}

/// 删除远程分支：git push origin :branch
fn delete_remote_branch(workdir: &PathBuf, origin: &str, branch: &str) -> Result<()> {
    let status = Command::new("git")
        .arg("push")
        .arg(origin)
        .arg(format!(":{}", branch))
        .current_dir(workdir)
        .status()
        .with_context(|| format!("failed to run git push to delete {}", branch))?;

    if !status.success() {
        return Err(anyhow!("git push {} :{} failed", origin, branch));
    }

    Ok(())
}

/// 列出已经完全合并到 master 的本地分支（不含 HEAD / master / skips）
fn merged_local_branches(
    workdir: &PathBuf,
    master: &str,
    skips: &[String],
) -> Result<Vec<String>> {
    // 使用 git branch --merged 获取已合并的分支
    let output = Command::new("git")
        .arg("branch")
        .arg("--merged")
        .arg(master)
        .current_dir(workdir)
        .output()
        .context("failed to execute git branch --merged")?;

    if !output.status.success() {
        return Err(anyhow!(
            "git branch --merged failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let mut merged = Vec::new();

    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // 去掉前面的 * 标记（当前分支）
        let branch = line.trim_start_matches('*').trim();

        // 跳过 master 分支本身
        if branch == master {
            continue;
        }

        // 跳过 skip 列表中的分支
        if skips.iter().any(|s| s == branch) {
            continue;
        }

        merged.push(branch.to_string());
    }

    Ok(merged)
}

/// 删除本地分支：git branch -d branch
fn delete_local_branch(workdir: &PathBuf, branch: &str) -> Result<()> {
    let status = Command::new("git")
        .arg("branch")
        .arg("-d")
        .arg(branch)
        .current_dir(workdir)
        .status()
        .with_context(|| format!("failed to run git branch -d {}", branch))?;

    if !status.success() {
        return Err(anyhow!("git branch -d {} failed", branch));
    }

    Ok(())
}
