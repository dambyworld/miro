use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;
use walkdir::WalkDir;

use crate::model::{ProviderKind, SessionRecord};
use crate::provider::{SessionProvider, home_dir, shorten_line};

pub struct ClaudeProvider {
    root: PathBuf,
}

impl ClaudeProvider {
    pub fn new() -> Result<Self> {
        Ok(Self {
            root: home_dir()?.join(".claude"),
        })
    }

    fn projects_root(&self) -> PathBuf {
        self.root.join("projects")
    }

    fn project_index_paths(&self) -> Vec<PathBuf> {
        WalkDir::new(self.projects_root())
            .min_depth(2)
            .max_depth(2)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.into_path())
            .filter(|path| {
                path.file_name().and_then(|name| name.to_str()) == Some("sessions-index.json")
            })
            .collect()
    }

    fn read_preview(path: &Path) -> Result<Option<String>> {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("failed to read claude session file {}", path.display()))?;
        let mut preview = None;
        for line in contents.lines() {
            let value: Value = match serde_json::from_str(line) {
                Ok(value) => value,
                Err(_) => continue,
            };
            if let Some(text) = extract_claude_text(&value) {
                let short = shorten_line(&text);
                if !short.is_empty()
                    && !short.contains("Caveat:")
                    && short != "No response requested."
                {
                    preview = Some(short);
                }
            }
        }
        Ok(preview)
    }
}

impl SessionProvider for ClaudeProvider {
    fn kind(&self) -> ProviderKind {
        ProviderKind::ClaudeCode
    }

    fn list_sessions(&self) -> Result<Vec<SessionRecord>> {
        let mut sessions = Vec::new();
        for path in self.project_index_paths() {
            let contents = fs::read_to_string(&path)
                .with_context(|| format!("failed to read {}", path.display()))?;
            let index: ClaudeIndex = serde_json::from_str(&contents)
                .with_context(|| format!("failed to parse {}", path.display()))?;
            for entry in index.entries {
                let session_path = PathBuf::from(&entry.full_path);
                if !session_path.exists() {
                    continue;
                }
                let preview = Self::read_preview(&session_path)?;
                let title = entry
                    .summary
                    .clone()
                    .filter(|value| !value.trim().is_empty())
                    .or_else(|| {
                        entry
                            .first_prompt
                            .clone()
                            .filter(|value| !value.trim().is_empty())
                    })
                    .unwrap_or_else(|| "Untitled Claude session".to_string());
                sessions.push(SessionRecord {
                    provider: ProviderKind::ClaudeCode,
                    session_id: entry.session_id,
                    title,
                    preview,
                    cwd: PathBuf::from(entry.project_path),
                    updated_at: entry.modified,
                    file_path: session_path,
                });
            }
        }
        Ok(sessions)
    }

    fn delete_session(&self, session: &SessionRecord) -> Result<()> {
        if session.file_path.exists() {
            fs::remove_file(&session.file_path).with_context(|| {
                format!(
                    "failed to remove claude session file {}",
                    session.file_path.display()
                )
            })?;
        }

        let Some(project_dir) = session.file_path.parent() else {
            return Ok(());
        };
        let index_path = project_dir.join("sessions-index.json");
        if !index_path.exists() {
            return Ok(());
        }

        let contents = fs::read_to_string(&index_path)
            .with_context(|| format!("failed to read {}", index_path.display()))?;
        let mut index: ClaudeIndex = serde_json::from_str(&contents)
            .with_context(|| format!("failed to parse {}", index_path.display()))?;
        index
            .entries
            .retain(|entry| entry.session_id != session.session_id);
        fs::write(&index_path, serde_json::to_string_pretty(&index)? + "\n")
            .with_context(|| format!("failed to rewrite {}", index_path.display()))?;
        Ok(())
    }
}

fn extract_claude_text(value: &Value) -> Option<String> {
    let kind = value.get("type").and_then(Value::as_str)?;
    match kind {
        "user" | "assistant" => {
            let message = value.get("message")?;
            if let Some(content) = message.get("content").and_then(Value::as_str) {
                return Some(content.to_string());
            }
            if let Some(parts) = message.get("content").and_then(Value::as_array) {
                let texts: Vec<String> = parts
                    .iter()
                    .filter_map(|part| part.get("text").and_then(Value::as_str))
                    .map(ToOwned::to_owned)
                    .collect();
                if !texts.is_empty() {
                    return Some(texts.join(" "));
                }
            }
            None
        }
        _ => value
            .get("content")
            .and_then(Value::as_str)
            .map(ToOwned::to_owned),
    }
}

#[derive(Debug, Deserialize, serde::Serialize)]
struct ClaudeIndex {
    version: u32,
    entries: Vec<ClaudeEntry>,
    original_path: Option<String>,
}

#[derive(Clone, Debug, Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ClaudeEntry {
    session_id: String,
    full_path: String,
    first_prompt: Option<String>,
    summary: Option<String>,
    project_path: String,
    modified: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::extract_claude_text;

    #[test]
    fn extracts_text_from_message_array() {
        let value = json!({
            "type": "assistant",
            "message": {
                "content": [
                    {"type":"text","text":"hello"}
                ]
            }
        });
        assert_eq!(extract_claude_text(&value).as_deref(), Some("hello"));
    }

    #[test]
    fn extracts_text_from_content_string() {
        let value = json!({
            "type":"system",
            "content":"<local-command-stdout>Bye!</local-command-stdout>"
        });
        assert_eq!(
            extract_claude_text(&value).as_deref(),
            Some("<local-command-stdout>Bye!</local-command-stdout>")
        );
    }
}
