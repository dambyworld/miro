use clap::{Parser, Subcommand, ValueEnum};

use crate::model::ProviderKind;
use crate::theme::ThemeName;

#[derive(Debug, Parser)]
#[command(
    name = "miro",
    version,
    about = "Terminal TUI for Codex and Claude Code sessions"
)]
pub struct Cli {
    #[arg(long, value_enum)]
    pub theme: Option<ThemeName>,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Themes,
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

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::Cli;
    use crate::theme::ThemeName;

    #[test]
    fn theme_is_none_when_not_specified() {
        let cli = Cli::parse_from(["miro"]);
        assert_eq!(cli.theme, None);
    }

    #[test]
    fn accepts_default_theme_argument() {
        let cli = Cli::parse_from(["miro", "--theme", "default"]);
        assert_eq!(cli.theme, Some(ThemeName::Default));
    }

    #[test]
    fn accepts_tomorrow_night_blue_argument() {
        let cli = Cli::parse_from(["miro", "--theme", "tomorrow-night-blue"]);
        assert_eq!(cli.theme, Some(ThemeName::TomorrowNightBlue));
    }

    #[test]
    fn invalid_theme_lists_available_values() {
        let error = Cli::try_parse_from(["miro", "--theme", "unknown"]).unwrap_err();
        let message = error.to_string();
        assert!(message.contains("tomorrow-night-blue"));
        assert!(message.contains("default"));
        assert!(message.contains("cursor-dark"));
        assert!(message.contains("darcula-dark"));
        assert!(message.contains("darcula-light"));
        assert!(message.contains("dracula"));
        assert!(message.contains("nord"));
        assert!(message.contains("one-dark"));
        assert!(message.contains("gruvbox-dark"));
        assert!(message.contains("gruvbox-light"));
        assert!(message.contains("catppuccin-mocha"));
        assert!(message.contains("tokyo-night"));
        assert!(message.contains("solarized-dark"));
        assert!(message.contains("solarized-light"));
    }

    #[test]
    fn parses_themes_command() {
        let cli = Cli::parse_from(["miro", "themes"]);
        assert!(matches!(cli.command, Some(super::Commands::Themes)));
    }
}
