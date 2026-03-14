use clap::{Parser, Subcommand, ValueEnum};

use crate::model::ProviderKind;

#[derive(Debug, Parser)]
#[command(
    name = "miro",
    version,
    about = "Terminal TUI for Codex and Claude Code sessions"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    List {
        #[arg(long)]
        provider: Option<ProviderKind>,
        #[arg(long, value_enum, default_value_t = ListOutput::Table)]
        output: ListOutput,
    },
    Resume {
        session_id: String,
        #[arg(long)]
        provider: Option<ProviderKind>,
    },
    Delete {
        session_id: String,
        #[arg(long)]
        provider: Option<ProviderKind>,
        #[arg(long)]
        yes: bool,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum ListOutput {
    Table,
    Json,
}
