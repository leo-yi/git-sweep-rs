use clap::{Parser, Subcommand};

/// Clean up your Git remote branches.
#[derive(Parser, Debug)]
#[command(
    name = "git-sweep-rs",
    about = "Clean up your Git remote branches.",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CommandKind,
}

#[derive(Subcommand, Debug)]
pub enum CommandKind {
    /// Preview the branches that will be deleted
    Preview {
        /// The name of the remote you wish to clean up
        #[arg(long, default_value = "origin")]
        origin: String,

        /// The name of what you consider the master branch
        #[arg(long, default_value = "master")]
        master: String,

        /// Do not fetch from the remote
        #[arg(long = "nofetch")]
        nofetch: bool,

        /// Comma-separated list of branches to skip
        #[arg(long = "skip", default_value = "")]
        skips: String,
    },

    /// Delete merged branches from the remote
    Cleanup {
        /// Do not ask, cleanup immediately
        #[arg(long)]
        force: bool,

        /// The name of the remote you wish to clean up
        #[arg(long, default_value = "origin")]
        origin: String,

        /// The name of what you consider the master branch
        #[arg(long, default_value = "master")]
        master: String,

        /// Do not fetch from the remote
        #[arg(long = "nofetch")]
        nofetch: bool,

        /// Comma-separated list of branches to skip
        #[arg(long = "skip", default_value = "")]
        skips: String,
    },

    /// Preview local branches that will be deleted
    PreviewLocal {
        /// The name of the branch to compare against
        #[arg(long, default_value = "master")]
        master: String,

        /// Comma-separated list of branches to skip
        #[arg(long = "skip", default_value = "")]
        skips: String,
    },

    /// Delete merged local branches
    CleanupLocal {
        /// Do not ask, cleanup immediately
        #[arg(long)]
        force: bool,

        /// The name of the branch to compare against
        #[arg(long, default_value = "master")]
        master: String,

        /// Comma-separated list of branches to skip
        #[arg(long = "skip", default_value = "")]
        skips: String,
    },
}
