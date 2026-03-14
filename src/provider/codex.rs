use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use walkdir::WalkDir;

use crate::model::{ProviderKind, SessionRecord};
use crate::provider::{SessionProvider, home_dir, shorten_line};

pub struct CodexProvider {
    root: PathBuf,
}

impl CodexProvider {
    pub fn new() -> Result<Self> {
        Ok(Self {
            root: home_dir()?.join(".codex"),
        })
    }

    fn session_index_path(&self) -> PathBuf {
        self.root.join("session_index.jsonl")
    }

    fn sessions_root(&self) -> PathBuf {
        self.root.join("sessions")
    }

    fn session_file_map(&self) -> HashMap<String, PathBuf> {
        let pattern =
            Regex::new(r"([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})\.jsonl$")
                .expect("valid regex");
        WalkDir::new(self.sessions_root())
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_file())
            .filter_map(|entry| {
                let path = entry.into_path();
                let name = path.file_name()?.to_str()?;
                let session_id = pattern
                    .captures(name)?
                    .get(1)
                    .map(|match_| match_.as_str().to_string())?;
                Some((session_id, path))
            })
            .collect()
    }

    fn read_preview_and_cwd(path: &Path) -> Result<(Option<String>, PathBuf)> {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("failed to read codex session file {}", path.display()))?;
        let mut preview = None;
        let mut cwd = None;

        for line in contents.lines() {
            let value: Value = match serde_json::from_str(line) {
                Ok(value) => value,
                Err(_) => continue,
            };

            if value.get("type").and_then(Value::as_str) == Some("session_meta") {
                cwd = value
                    .get("payload")
                    .and_then(|payload| payload.get("cwd"))
                    .and_then(Value::as_str)
                    .map(PathBuf::from)
                    .or(cwd);
            }

            if let Some(text) = extract_codex_text(&value) {
                let short = shorten_line(&text);
                if !short.is_empty() && !short.starts_with("You are Codex") {
                    preview = Some(short);
                }
            }
        }

        Ok((preview, cwd.unwrap_or_else(|| PathBuf::from("."))))
    }
}

impl SessionProvider for CodexProvider {
    fn kind(&self) -> ProviderKind {
        ProviderKind::Codex
    }

    fn list_sessions(&self) -> Result<Vec<SessionRecord>> {
        let contents = match fs::read_to_string(self.session_index_path()) {
            Ok(contents) => contents,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(error) => return Err(error).context("failed to read codex session index"),
        };

        let file_map = self.session_file_map();
        let mut sessions = Vec::new();

        for line in contents.lines() {
            let entry: CodexIndexEntry = match serde_json::from_str(line) {
                Ok(entry) => entry,
                Err(_) => continue,
            };
            let Some(file_path) = file_map.get(&entry.id).cloned() else {
                continue;
            };
            let (preview, cwd) = Self::read_preview_and_cwd(&file_path)?;
            sessions.push(SessionRecord {
                provider: ProviderKind::Codex,
                session_id: entry.id,
                title: entry
                    .thread_name
                    .unwrap_or_else(|| "Untitled Codex session".to_string()),
                preview,
                cwd,
                updated_at: entry.updated_at,
                file_path,
            });
        }

        Ok(sessions)
    }

    fn delete_session(&self, session: &SessionRecord) -> Result<()> {
        if session.file_path.exists() {
            fs::remove_file(&session.file_path).with_context(|| {
                format!(
                    "failed to remove codex session file {}",
                    session.file_path.display()
                )
            })?;
        }

        let index_path = self.session_index_path();
        let contents = match fs::read_to_string(&index_path) {
            Ok(contents) => contents,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(error) => return Err(error).context("failed to load codex session index"),
        };

        let kept: Vec<&str> = contents
            .lines()
            .filter(|line| !line.contains(&format!("\"id\":\"{}\"", session.session_id)))
            .collect();
        fs::write(
            &index_path,
            kept.join("\n") + if kept.is_empty() { "" } else { "\n" },
        )
        .context("failed to rewrite codex session index")?;
        Ok(())
    }
}

fn extract_codex_text(value: &Value) -> Option<String> {
    match value.get("type").and_then(Value::as_str) {
        Some("response_item") => {
            let payload = value.get("payload")?;
            if payload.get("type").and_then(Value::as_str) != Some("message") {
                return None;
            }
            let mut texts = Vec::new();
            let content = payload.get("content")?.as_array()?;
            for part in content {
                if let Some(text) = part.get("text").and_then(Value::as_str) {
                    texts.push(text.to_string());
                }
            }
            Some(texts.join(" "))
        }
        Some("event_msg") => value
            .get("payload")
            .and_then(|payload| payload.get("message"))
            .and_then(Value::as_str)
            .map(ToOwned::to_owned),
        _ => None,
    }
}

#[derive(Debug, Deserialize)]
struct CodexIndexEntry {
    id: String,
    thread_name: Option<String>,
    updated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use chrono::Datelike;
    use serde_json::json;

    use super::extract_codex_text;

    #[test]
    fn extracts_text_from_response_item() {
        let value = json!({
            "type": "response_item",
            "payload": {
                "type": "message",
                "content": [
                    {"type": "input_text", "text": "hello"},
                    {"type": "output_text", "text": "world"}
                ]
            }
        });
        assert_eq!(extract_codex_text(&value).as_deref(), Some("hello world"));
    }

    #[test]
    fn parses_updated_at() {
        let entry: super::CodexIndexEntry = serde_json::from_str(
            r#"{"id":"1","thread_name":"sample","updated_at":"2026-03-14T11:19:00.702545Z"}"#,
        )
        .unwrap();
        assert_eq!(entry.updated_at.year(), 2026);
    }
}
