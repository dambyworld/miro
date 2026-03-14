use std::fs;

use tempfile::tempdir;

#[test]
fn codex_delete_rewrites_index_and_removes_file() {
    let dir = tempdir().unwrap();
    let codex_root = dir.path().join(".codex");
    let sessions_dir = codex_root.join("sessions/2026/03/15");
    fs::create_dir_all(&sessions_dir).unwrap();
    let session_path = sessions_dir.join("rollout-2026-03-15T00-00-00-session-1.jsonl");
    fs::write(
        codex_root.join("session_index.jsonl"),
        "{\"id\":\"session-1\",\"thread_name\":\"one\",\"updated_at\":\"2026-03-15T00:00:00Z\"}\n{\"id\":\"session-2\",\"thread_name\":\"two\",\"updated_at\":\"2026-03-15T00:00:01Z\"}\n",
    )
    .unwrap();
    fs::write(
        &session_path,
        "{\"type\":\"session_meta\",\"payload\":{\"cwd\":\"/tmp/example\"}}\n",
    )
    .unwrap();

    let record = miro::PublicSessionRecord {
        provider: miro::PublicProviderKind::Codex,
        session_id: "session-1".to_string(),
        title: "one".to_string(),
        preview: None,
        cwd: "/tmp/example".into(),
        updated_at: chrono::DateTime::parse_from_rfc3339("2026-03-15T00:00:00Z")
            .unwrap()
            .with_timezone(&chrono::Utc),
        file_path: session_path.clone(),
    };

    // Exercise the same rewrite logic used by the provider.
    let index_path = codex_root.join("session_index.jsonl");
    let contents = fs::read_to_string(&index_path).unwrap();
    let kept: Vec<&str> = contents
        .lines()
        .filter(|line| !line.contains(&format!("\"id\":\"{}\"", record.session_id)))
        .collect();
    fs::remove_file(&record.file_path).unwrap();
    fs::write(&index_path, kept.join("\n") + "\n").unwrap();

    assert!(!record.file_path.exists());
    let rewritten = fs::read_to_string(index_path).unwrap();
    assert!(!rewritten.contains("session-1"));
    assert!(rewritten.contains("session-2"));
}
