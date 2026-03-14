mod claude;
mod codex;

use std::path::PathBuf;

use anyhow::Result;

use crate::model::{ProviderKind, SessionRecord};

pub fn build_providers() -> Result<Vec<Box<dyn SessionProvider>>> {
    Ok(vec![
        Box::new(codex::CodexProvider::new()?),
        Box::new(claude::ClaudeProvider::new()?),
    ])
}

pub trait SessionProvider {
    fn kind(&self) -> ProviderKind;
    fn list_sessions(&self) -> Result<Vec<SessionRecord>>;
    fn delete_session(&self, session: &SessionRecord) -> Result<()>;
}

fn home_dir() -> Result<PathBuf> {
    dirs::home_dir().ok_or_else(|| anyhow::anyhow!("home directory not found"))
}

fn shorten_line(input: &str) -> String {
    let collapsed = input.split_whitespace().collect::<Vec<_>>().join(" ");
    let cleaned = collapsed
        .replace("<command-name>", "")
        .replace("</command-name>", "")
        .replace("<command-message>", "")
        .replace("</command-message>", "")
        .replace("<command-args>", "")
        .replace("</command-args>", "")
        .replace("<local-command-stdout>", "")
        .replace("</local-command-stdout>", "")
        .replace("<local-command-caveat>", "")
        .replace("</local-command-caveat>", "");
    let trimmed = cleaned.trim();
    let shortened: String = trimmed.chars().take(100).collect();
    if trimmed.chars().count() > 100 {
        format!("{}...", shortened.chars().take(97).collect::<String>())
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::shorten_line;

    #[test]
    fn shorten_line_collapses_command_tags() {
        let value = shorten_line(
            "<command-name>/resume</command-name>\n<command-message>resume</command-message>",
        );
        assert_eq!(value, "/resume resume");
    }
}
