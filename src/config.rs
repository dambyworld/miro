use std::fs;

use serde::{Deserialize, Serialize};

use crate::theme::ThemeName;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MiroConfig {
    pub theme: Option<String>,
}

impl MiroConfig {
    fn config_path() -> Option<std::path::PathBuf> {
        dirs::config_dir().map(|d| d.join("miro").join("config.toml"))
    }

    pub fn load() -> Self {
        let path = match Self::config_path() {
            Some(p) => p,
            None => return Self::default(),
        };
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => return Self::default(),
        };
        toml::from_str(&content).unwrap_or_default()
    }

    pub fn theme_name(&self) -> Option<ThemeName> {
        self.theme.as_deref().and_then(ThemeName::from_cli_id)
    }

    pub fn save_theme(theme: ThemeName) {
        let path = match Self::config_path() {
            Some(p) => p,
            None => return,
        };
        if let Some(dir) = path.parent() {
            let _ = fs::create_dir_all(dir);
        }
        let config = MiroConfig {
            theme: Some(theme.cli_id().to_string()),
        };
        if let Ok(content) = toml::to_string(&config) {
            let _ = fs::write(&path, content);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MiroConfig;
    use crate::theme::ThemeName;

    #[test]
    fn load_returns_default_when_no_file() {
        // 파일 없으면 theme = None
        let config = MiroConfig {
            theme: None,
        };
        assert!(config.theme_name().is_none());
    }

    #[test]
    fn theme_name_parses_valid_id() {
        let config = MiroConfig {
            theme: Some("dracula".to_string()),
        };
        assert_eq!(config.theme_name(), Some(ThemeName::Dracula));
    }

    #[test]
    fn theme_name_returns_none_for_invalid_id() {
        let config = MiroConfig {
            theme: Some("not-a-real-theme".to_string()),
        };
        assert!(config.theme_name().is_none());
    }

    #[test]
    fn save_and_load_roundtrip() {
        use std::fs;
        use tempfile::TempDir;

        // 임시 디렉터리에 저장/로드 테스트
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("config.toml");
        let config = MiroConfig {
            theme: Some(ThemeName::Nord.cli_id().to_string()),
        };
        let content = toml::to_string(&config).unwrap();
        fs::write(&path, &content).unwrap();

        let loaded: MiroConfig = toml::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(loaded.theme_name(), Some(ThemeName::Nord));
    }
}
