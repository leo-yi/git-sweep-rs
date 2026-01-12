mod cli;
mod git_ops;

use clap::Parser;
use crate::cli::{Cli, CommandKind};

fn main() {
    let cli = Cli::parse();

    if let Err(code) = run(cli) {
        std::process::exit(code);
    }
}

fn run(cli: Cli) -> Result<i32, i32> {
    // cleanup-local 和 preview-local 命令不需要检查 Git 仓库，因为它们操作的是本地分支
    if !matches!(cli.command, CommandKind::CleanupLocal { .. } | CommandKind::PreviewLocal { .. }) {
        // 先确认当前目录是一个 Git 仓库
        if let Err(e) = git_ops::ensure_git_repo() {
            eprintln!("{}", e);
            return Err(1);
        }
    }

    let res = match cli.command {
        CommandKind::Preview {
            origin,
            master,
            nofetch,
            skips,
        } => {
            git_ops::run_sweep(true, origin, master, !nofetch, git_ops::parse_skips(&skips))
                .map(|_| ())
        }
        CommandKind::Cleanup {
            force,
            origin,
            master,
            nofetch,
            skips,
        } => git_ops::run_cleanup(force, origin, master, !nofetch, git_ops::parse_skips(&skips)),
        CommandKind::PreviewLocal {
            master,
            skips,
        } => git_ops::run_preview_local(master, git_ops::parse_skips(&skips)),
        CommandKind::CleanupLocal {
            force,
            master,
            skips,
        } => git_ops::run_cleanup_local(force, master, git_ops::parse_skips(&skips)),
    };

    match res {
        Ok(_) => Ok(0),
        Err(e) => {
            eprintln!("{}", e);
            Err(1)
        }
    }
}
