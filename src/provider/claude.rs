use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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

    fn project_session_paths(&self) -> Vec<PathBuf> {
        WalkDir::new(self.projects_root())
            .min_depth(2)
            .max_depth(2)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.into_path())
            .filter(|path| {
                path.extension().and_then(|extension| extension.to_str()) == Some("jsonl")
            })
            .collect()
    }

    fn read_index_entries(&self) -> Result<HashMap<String, ClaudeIndexEntry>> {
        let mut entries = HashMap::new();
        for path in self.project_index_paths() {
            let contents = fs::read_to_string(&path)
                .with_context(|| format!("failed to read {}", path.display()))?;
            let index: ClaudeIndex = serde_json::from_str(&contents)
                .with_context(|| format!("failed to parse {}", path.display()))?;
            for entry in index.entries {
                entries.insert(entry.session_id.clone(), entry);
            }
        }
        Ok(entries)
    }

    fn read_session_file(path: &Path) -> Result<Option<ClaudeSessionFileRecord>> {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("failed to read claude session file {}", path.display()))?;

        let mut session_id = None;
        let mut cwd = None;
        let mut first_user_message = None;
        let mut preview = None;
        let mut updated_at: Option<DateTime<Utc>> = None;

        for line in contents.lines() {
            let value: Value = match serde_json::from_str(line) {
                Ok(value) => value,
                Err(_) => continue,
            };

            if session_id.is_none() {
                session_id = value
                    .get("sessionId")
                    .and_then(Value::as_str)
                    .map(ToOwned::to_owned);
            }

            if cwd.is_none() {
                cwd = value
                    .get("cwd")
                    .and_then(Value::as_str)
                    .map(PathBuf::from)
                    .or(cwd);
            }

            if let Some(timestamp) = extract_timestamp(&value) {
                updated_at = Some(match updated_at {
                    Some(current) => current.max(timestamp),
                    None => timestamp,
                });
            }

            if let Some(text) = extract_claude_text(&value) {
                let short = shorten_line(&text);
                if is_meaningful_text(&short) {
                    preview = Some(short.clone());
                    if first_user_message.is_none()
                        && value.get("type").and_then(Value::as_str) == Some("user")
                    {
                        first_user_message = Some(short);
                    }
                }
            }
        }

        let Some(session_id) = session_id else {
            return Ok(None);
        };

        Ok(Some(ClaudeSessionFileRecord {
            session_id,
            cwd: cwd.unwrap_or_else(|| PathBuf::from(".")),
            updated_at: updated_at.unwrap_or_else(Utc::now),
            first_user_message,
            preview,
            file_path: path.to_path_buf(),
        }))
    }
}

impl SessionProvider for ClaudeProvider {
    fn kind(&self) -> ProviderKind {
        ProviderKind::ClaudeCode
    }

    fn list_sessions(&self) -> Result<Vec<SessionRecord>> {
        let index_entries = self.read_index_entries()?;
        let mut file_records = HashMap::new();

        for path in self.project_session_paths() {
            if let Some(record) = Self::read_session_file(&path)? {
                file_records.insert(record.session_id.clone(), record);
            }
        }

        let mut merged_ids: Vec<String> = file_records
            .keys()
            .chain(index_entries.keys())
            .cloned()
            .collect();
        merged_ids.sort();
        merged_ids.dedup();

        let mut sessions = Vec::new();
        for session_id in merged_ids {
            let file_record = file_records.get(&session_id).cloned();
            let index_entry = index_entries.get(&session_id);

            let fallback_file_record = index_entry.and_then(|entry| {
                let path = PathBuf::from(&entry.full_path);
                if path.exists() {
                    Self::read_session_file(&path).ok().flatten()
                } else {
                    None
                }
            });

            let Some(file_record) = file_record.or(fallback_file_record) else {
                continue;
            };

            let title = index_entry
                .and_then(|entry| entry.summary.as_ref())
                .filter(|value| !value.trim().is_empty())
                .cloned()
                .or_else(|| {
                    index_entry
                        .and_then(|entry| entry.first_prompt.as_ref())
                        .filter(|value| !value.trim().is_empty())
                        .cloned()
                })
                .or_else(|| file_record.first_user_message.clone())
                .or_else(|| file_record.preview.clone())
                .unwrap_or_else(|| "Untitled Claude session".to_string());

            let cwd = index_entry
                .map(|entry| PathBuf::from(&entry.project_path))
                .filter(|path| path != Path::new("."))
                .unwrap_or_else(|| file_record.cwd.clone());

            let updated_at = index_entry
                .map(|entry| entry.modified)
                .map(|timestamp| timestamp.max(file_record.updated_at))
                .unwrap_or(file_record.updated_at);

            sessions.push(SessionRecord {
                provider: ProviderKind::ClaudeCode,
                session_id,
                title,
                preview: file_record.preview.clone(),
                cwd,
                updated_at,
                file_path: file_record.file_path.clone(),
            });
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

fn extract_timestamp(value: &Value) -> Option<DateTime<Utc>> {
    value
        .get("timestamp")
        .and_then(Value::as_str)
        .and_then(parse_timestamp)
        .or_else(|| {
            value
                .get("snapshot")
                .and_then(|snapshot| snapshot.get("timestamp"))
                .and_then(Value::as_str)
                .and_then(parse_timestamp)
        })
}

fn parse_timestamp(value: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|timestamp| timestamp.with_timezone(&Utc))
}

fn is_meaningful_text(value: &str) -> bool {
    !value.is_empty()
        && !value.contains("Caveat:")
        && value != "No response requested."
        && value != "Status dialog dismissed"
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

#[derive(Debug, Deserialize, Serialize)]
struct ClaudeIndex {
    version: u32,
    entries: Vec<ClaudeIndexEntry>,
    original_path: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClaudeIndexEntry {
    session_id: String,
    full_path: String,
    first_prompt: Option<String>,
    summary: Option<String>,
    project_path: String,
    modified: DateTime<Utc>,
}

#[derive(Clone, Debug)]
struct ClaudeSessionFileRecord {
    session_id: String,
    cwd: PathBuf,
    updated_at: DateTime<Utc>,
    first_user_message: Option<String>,
    preview: Option<String>,
    file_path: PathBuf,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use chrono::{TimeZone, Utc};
    use serde_json::json;
    use tempfile::tempdir;

    use crate::provider::SessionProvider;

    use super::{ClaudeProvider, extract_claude_text};

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

    #[test]
    fn lists_sessions_from_jsonl_without_index() {
        let dir = tempdir().unwrap();
        let provider = ClaudeProvider {
            root: dir.path().join(".claude"),
        };
        let project_dir = provider.projects_root().join("-Users-test-dev");
        fs::create_dir_all(&project_dir).unwrap();
        let session_path = project_dir.join("session-a.jsonl");
        fs::write(
            &session_path,
            concat!(
                "{\"type\":\"user\",\"sessionId\":\"session-a\",\"cwd\":\"/tmp/work\",\"timestamp\":\"2026-03-15T08:00:00Z\",\"message\":{\"role\":\"user\",\"content\":\"Fix this bug\"}}\n",
                "{\"type\":\"assistant\",\"sessionId\":\"session-a\",\"cwd\":\"/tmp/work\",\"timestamp\":\"2026-03-15T08:01:00Z\",\"message\":{\"role\":\"assistant\",\"content\":[{\"type\":\"text\",\"text\":\"Working on it now\"}]}}\n"
            ),
        )
        .unwrap();

        let sessions = provider.list_sessions().unwrap();
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].session_id, "session-a");
        assert_eq!(sessions[0].title, "Fix this bug");
        assert_eq!(sessions[0].cwd, Path::new("/tmp/work"));
        assert_eq!(sessions[0].preview.as_deref(), Some("Working on it now"));
    }

    #[test]
    fn falls_back_to_jsonl_when_index_path_is_stale() {
        let dir = tempdir().unwrap();
        let provider = ClaudeProvider {
            root: dir.path().join(".claude"),
        };
        let project_dir = provider.projects_root().join("-Users-test-dev");
        fs::create_dir_all(&project_dir).unwrap();
        let session_path = project_dir.join("session-b.jsonl");
        fs::write(
            &session_path,
            concat!(
                "{\"type\":\"user\",\"sessionId\":\"session-b\",\"cwd\":\"/tmp/work-b\",\"timestamp\":\"2026-03-15T09:00:00Z\",\"message\":{\"role\":\"user\",\"content\":\"Actual prompt\"}}\n",
                "{\"type\":\"assistant\",\"sessionId\":\"session-b\",\"cwd\":\"/tmp/work-b\",\"timestamp\":\"2026-03-15T09:02:00Z\",\"message\":{\"role\":\"assistant\",\"content\":[{\"type\":\"text\",\"text\":\"Actual preview\"}]}}\n"
            ),
        )
        .unwrap();
        fs::write(
            project_dir.join("sessions-index.json"),
            serde_json::to_string_pretty(&json!({
                "version": 1,
                "entries": [{
                    "sessionId": "session-b",
                    "fullPath": format!("{}/missing.jsonl", project_dir.display()),
                    "firstPrompt": "Indexed prompt",
                    "summary": "Indexed summary",
                    "projectPath": "/tmp/indexed",
                    "modified": "2026-03-15T09:01:00Z"
                }],
                "originalPath": "/tmp/indexed"
            }))
            .unwrap(),
        )
        .unwrap();

        let sessions = provider.list_sessions().unwrap();
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].title, "Indexed summary");
        assert_eq!(sessions[0].preview.as_deref(), Some("Actual preview"));
        assert_eq!(sessions[0].cwd, Path::new("/tmp/indexed"));
        assert_eq!(
            sessions[0].updated_at,
            Utc.with_ymd_and_hms(2026, 3, 15, 9, 2, 0).unwrap()
        );
    }
}
