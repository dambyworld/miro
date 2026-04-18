use std::path::PathBuf;

use anyhow::{Context, Result};
use chrono::{DateTime, TimeZone, Utc};
use rusqlite::Connection;

use crate::model::{ProviderKind, SessionRecord};
use crate::provider::SessionProvider;

pub struct OpenCodeProvider {
    db_path: PathBuf,
}

impl OpenCodeProvider {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("home directory not found"))?;
        let db_path = home.join(".local/share/opencode/opencode.db");
        Ok(Self { db_path })
    }

    fn db_path(&self) -> &PathBuf {
        &self.db_path
    }

    fn extract_preview(&self, conn: &Connection, session_id: &str) -> Result<Option<String>> {
        let mut stmt = conn.prepare(
            "SELECT data FROM message WHERE session_id = ? ORDER BY time_created LIMIT 1"
        )?;
        
        let rows = stmt.query_map([session_id], |row| {
            let data: String = row.get(0)?;
            Ok(data)
        })?;

        for row in rows {
            let data = row?;
            // JSON에서 텍스트 추출 시도
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&data) {
                if let Some(text) = json.get("content").and_then(|c| c.as_str()) {
                    return Ok(Some(crate::provider::shorten_line(text)));
                }
            }
        }
        
        Ok(None)
    }
}

impl SessionProvider for OpenCodeProvider {
    fn kind(&self) -> ProviderKind {
        ProviderKind::OpenCode
    }

    fn list_sessions(&self) -> Result<Vec<SessionRecord>> {
        if !self.db_path.exists() {
            return Ok(Vec::new());
        }

        let conn = Connection::open(self.db_path())
            .with_context(|| format!("failed to open opencode database: {}", self.db_path.display()))?;

        let mut stmt = conn.prepare(
            "SELECT id, title, directory, time_created, time_updated FROM session ORDER BY time_updated DESC"
        )?;

        let rows = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let title: String = row.get(1)?;
            let directory: String = row.get(2)?;
            let time_created: i64 = row.get(3)?;
            let time_updated: i64 = row.get(4)?;
            
            Ok((id, title, directory, time_created, time_updated))
        })?;

        let mut sessions = Vec::new();
        for row in rows {
            let (id, title, directory, _time_created, time_updated) = row?;
            
            // Convert Unix timestamp (ms) to DateTime<Utc>
            let updated_at = Utc.timestamp_millis_opt(time_updated)
                .single()
                .unwrap_or_else(Utc::now);

            // Extract preview from first message
            let preview = self.extract_preview(&conn, &id).ok().flatten();

            sessions.push(SessionRecord {
                provider: ProviderKind::OpenCode,
                session_id: id.clone(),
                title,
                preview,
                cwd: PathBuf::from(directory),
                updated_at,
                file_path: self.db_path.clone(),
            });
        }

        Ok(sessions)
    }

    fn delete_session(&self, session: &SessionRecord) -> Result<()> {
        if !self.db_path.exists() {
            return Ok(());
        }

        let conn = Connection::open(self.db_path())
            .with_context(|| format!("failed to open opencode database: {}", self.db_path.display()))?;

        // Delete from session table (messages will be deleted via FK CASCADE)
        conn.execute(
            "DELETE FROM session WHERE id = ?",
            [&session.session_id],
        ).with_context(|| format!("failed to delete opencode session: {}", session.session_id))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn create_test_db(path: &PathBuf) -> Connection {
        let conn = Connection::open(path).unwrap();
        
        conn.execute(
            "CREATE TABLE session (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                directory TEXT NOT NULL,
                time_created INTEGER NOT NULL,
                time_updated INTEGER NOT NULL
            )",
            [],
        ).unwrap();

        conn.execute(
            "CREATE TABLE message (
                id TEXT PRIMARY KEY,
                session_id TEXT NOT NULL,
                time_created INTEGER NOT NULL,
                time_updated INTEGER NOT NULL,
                data TEXT NOT NULL
            )",
            [],
        ).unwrap();

        conn
    }

    #[test]
    fn lists_sessions_from_database() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("opencode.db");
        let conn = create_test_db(&db_path);

        // Insert test session
        conn.execute(
            "INSERT INTO session (id, title, directory, time_created, time_updated) VALUES (?1, ?2, ?3, ?4, ?5)",
            ["ses_test123", "Test Session", "/tmp/test", "1704067200000", "1704067200000"],
        ).unwrap();

        // Insert test message
        conn.execute(
            "INSERT INTO message (id, session_id, time_created, time_updated, data) VALUES (?1, ?2, ?3, ?4, ?5)",
            ["msg_1", "ses_test123", "1704067200000", "1704067200000", r#"{"role":"user","content":"Hello opencode"}"#],
        ).unwrap();

        drop(conn);

        let provider = OpenCodeProvider { db_path };
        let sessions = provider.list_sessions().unwrap();

        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].session_id, "ses_test123");
        assert_eq!(sessions[0].title, "Test Session");
        assert_eq!(sessions[0].cwd, PathBuf::from("/tmp/test"));
        assert_eq!(sessions[0].provider, ProviderKind::OpenCode);
    }

    #[test]
    fn returns_empty_when_no_database() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("nonexistent.db");
        
        let provider = OpenCodeProvider { db_path };
        let sessions = provider.list_sessions().unwrap();

        assert!(sessions.is_empty());
    }

    #[test]
    fn deletes_session_from_database() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("opencode.db");
        let conn = create_test_db(&db_path);

        conn.execute(
            "INSERT INTO session (id, title, directory, time_created, time_updated) VALUES (?1, ?2, ?3, ?4, ?5)",
            ["ses_delete", "Delete Me", "/tmp/delete", "1704067200000", "1704067200000"],
        ).unwrap();

        drop(conn);

        let provider = OpenCodeProvider { db_path: db_path.clone() };
        let session = SessionRecord {
            provider: ProviderKind::OpenCode,
            session_id: "ses_delete".to_string(),
            title: "Delete Me".to_string(),
            preview: None,
            cwd: PathBuf::from("/tmp/delete"),
            updated_at: Utc::now(),
            file_path: db_path,
        };

        provider.delete_session(&session).unwrap();

        // Verify deletion
        let conn = Connection::open(&session.file_path).unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM session WHERE id = ?",
            ["ses_delete"],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(count, 0);
    }
}
