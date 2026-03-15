mod app;
mod cli;
mod model;
mod provider;
mod tui;

use std::process::{Command, ExitStatus};

use anyhow::{Context, Result, bail};
use clap::Parser;

use crate::app::SessionManager;
use crate::cli::{Cli, Commands, ListOutput};
use crate::model::{ProviderKind, SessionRecord};

pub use crate::model::{ProviderKind as PublicProviderKind, SessionRecord as PublicSessionRecord};

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    let manager = SessionManager::discover()?;

    match cli.command {
        Some(Commands::List { provider, output }) => list_sessions(&manager, provider, output),
        Some(Commands::Resume {
            session_id,
            provider,
        }) => {
            let session = manager.find_session(&session_id, provider)?;
            run_resume_command(&session)
        }
        Some(Commands::Delete {
            session_id,
            provider,
            yes,
        }) => {
            let session = manager.find_session(&session_id, provider)?;
            if !yes {
                bail!("delete requires --yes");
            }
            manager.delete_session(&session)?;
            println!(
                "deleted {} {}",
                session.provider.as_str(),
                session.session_id
            );
            Ok(())
        }
        None => tui::run_tui(manager),
    }
}

fn list_sessions(
    manager: &SessionManager,
    provider: Option<ProviderKind>,
    output: ListOutput,
) -> Result<()> {
    let sessions = manager.list_sessions(provider)?;
    match output {
        ListOutput::Table => {
            for session in sessions {
                println!(
                    "[{provider}] {id}\n  {title}\n  {preview}\n  cwd: {cwd}\n  updated: {updated}\n",
                    provider = session.provider.as_str(),
                    id = session.session_id,
                    title = session.title,
                    preview = session.preview.as_deref().unwrap_or("-"),
                    cwd = session.cwd.display(),
                    updated = session.updated_at,
                );
            }
        }
        ListOutput::Json => {
            println!("{}", serde_json::to_string_pretty(&sessions)?);
        }
    }
    Ok(())
}

pub(crate) fn run_resume_command(session: &SessionRecord) -> Result<()> {
    let command = session.provider.resume_command(&session.session_id);
    let mut process = Command::new(&command.program);
    process.args(&command.args);
    if session.cwd.exists() {
        process.current_dir(&session.cwd);
    }
    let status = process
        .status()
        .with_context(|| format!("failed to launch {}", command.program))?;
    ensure_success(status, &command.program)
}

fn ensure_success(status: ExitStatus, program: &str) -> Result<()> {
    if status.success() {
        Ok(())
    } else {
        bail!("{program} exited with status {status}")
    }
}
