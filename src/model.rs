use std::fmt;
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use clap::ValueEnum;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderKind {
    Codex,
    ClaudeCode,
    OpenCode,
}

impl ProviderKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Codex => "codex",
            Self::ClaudeCode => "claude-code",
            Self::OpenCode => "opencode",
        }
    }

    pub fn resume_command(self, session_id: &str) -> CommandSpec {
        match self {
            Self::Codex => CommandSpec::new("codex", ["resume", session_id]),
            Self::ClaudeCode => CommandSpec::new("claude", ["--resume", session_id]),
            Self::OpenCode => CommandSpec::new("opencode", ["--session", session_id]),
        }
    }
}

impl fmt::Display for ProviderKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct SessionRecord {
    pub provider: ProviderKind,
    pub session_id: String,
    pub title: String,
    pub preview: Option<String>,
    pub cwd: PathBuf,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub file_path: PathBuf,
}

#[derive(Clone, Debug)]
pub struct CommandSpec {
    pub program: String,
    pub args: Vec<String>,
}

impl CommandSpec {
    pub fn new<I, S>(program: &str, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        Self {
            program: program.to_string(),
            args: args
                .into_iter()
                .map(|arg| arg.as_ref().to_string())
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ProviderKind;

    #[test]
    fn codex_resume_command_uses_codex_resume() {
        let command = ProviderKind::Codex.resume_command("session-123");

        assert_eq!(command.program, "codex");
        assert_eq!(command.args, vec!["resume", "session-123"]);
    }

    #[test]
    fn claude_code_resume_command_uses_resume_flag() {
        let command = ProviderKind::ClaudeCode.resume_command("session-123");

        assert_eq!(command.program, "claude");
        assert_eq!(command.args, vec!["--resume", "session-123"]);
    }

    #[test]
    fn opencode_resume_command_uses_session_flag() {
        let command = ProviderKind::OpenCode.resume_command("ses_abc123");

        assert_eq!(command.program, "opencode");
        assert_eq!(command.args, vec!["--session", "ses_abc123"]);
    }
}
