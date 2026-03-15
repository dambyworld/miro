mod app;
mod cli;
pub(crate) mod config;
mod model;
mod provider;
mod theme;
mod tui;

use std::process::{Command, ExitStatus};

use anyhow::{Context, Result, bail};
use clap::Parser;

use crate::app::SessionManager;
use crate::cli::{Cli, Commands, ListOutput};
use crate::config::MiroConfig;
use crate::model::{ProviderKind, SessionRecord};
use crate::theme::{Theme, ThemeName};

pub use crate::model::{ProviderKind as PublicProviderKind, SessionRecord as PublicSessionRecord};

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    let manager = SessionManager::discover()?;

    let saved = MiroConfig::load();
    let theme_name = cli
        .theme
        .or_else(|| saved.theme_name())
        .unwrap_or(ThemeName::TomorrowNightBlue);
    let theme = Theme::get(theme_name);

    match cli.command {
        Some(Commands::Themes) => list_themes(),
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
        None => tui::run_tui(manager, theme),
    }
}

fn list_themes() -> Result<()> {
    for theme in ThemeName::all() {
        let default_marker = if *theme == ThemeName::TomorrowNightBlue {
            " (default)"
        } else {
            ""
        };
        println!(
            "{}{} [{}]\n  {}\n",
            theme.display_name(),
            default_marker,
            theme.cli_id(),
            theme.description(),
        );
    }
    Ok(())
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
