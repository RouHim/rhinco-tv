use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Represents a custom game entry in Ludusavi's config.yaml
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomGameEntry {
    pub name: String,
    pub files: Vec<String>,
    #[serde(default = "default_integration")]
    pub integration: String,
}

fn default_integration() -> String {
    "override".to_string()
}

/// Returns the path to Ludusavi's config.yaml file
pub fn ludusavi_config_path() -> Option<PathBuf> {
    let home = std::env::var("HOME").ok()?;
    Some(
        PathBuf::from(home)
            .join(".config")
            .join("ludusavi")
            .join("config.yaml"),
    )
}

/// Reads the Ludusavi config file and returns it as a dynamic YAML value
pub fn read_ludusavi_config(path: &Path) -> Result<serde_yml::Value> {
    if !path.exists() {
        return Ok(serde_yml::Value::Mapping(serde_yml::Mapping::new()));
    }

    let content = std::fs::read_to_string(path).context("Failed to read Ludusavi config file")?;
    let value: serde_yml::Value =
        serde_yml::from_str(&content).context("Failed to parse Ludusavi config YAML")?;
    Ok(value)
}

/// Writes a custom game entry to the Ludusavi config file
/// Preserves all existing config fields and merges into the customGames array
/// If a game with the same name exists, updates its files list
pub fn write_custom_game(path: &Path, entry: &CustomGameEntry) -> Result<()> {
    let mut config = read_ludusavi_config(path)?;

    let mapping = config
        .as_mapping_mut()
        .context("Config root is not a mapping")?;

    let custom_games = mapping
        .entry(serde_yml::Value::String("customGames".to_string()))
        .or_insert_with(|| serde_yml::Value::Sequence(Vec::new()));

    let games_seq = custom_games
        .as_sequence_mut()
        .context("customGames is not a sequence")?;

    let entry_value =
        serde_yml::to_value(entry).context("Failed to serialize custom game entry")?;

    let mut found = false;
    for game in games_seq.iter_mut() {
        if let Some(game_map) = game.as_mapping() {
            if let Some(name_value) = game_map.get(serde_yml::Value::String("name".to_string())) {
                if let Some(name_str) = name_value.as_str() {
                    if name_str == entry.name {
                        *game = entry_value.clone();
                        found = true;
                        break;
                    }
                }
            }
        }
    }

    if !found {
        games_seq.push(entry_value);
    }

    let yaml_str = serde_yml::to_string(&config).context("Failed to serialize config to YAML")?;

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .context("Failed to create Ludusavi config directory")?;
        }
    }

    std::fs::write(path, yaml_str).context("Failed to write Ludusavi config file")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_ludusavi_config_path_returns_some() {
        let path = ludusavi_config_path();
        assert!(path.is_some());
        let path = path.unwrap();
        assert!(path.to_string_lossy().contains(".config"));
        assert!(path.to_string_lossy().contains("ludusavi"));
        assert!(path.to_string_lossy().ends_with("config.yaml"));
    }

    #[test]
    fn test_read_valid_config() {
        let temp_file = NamedTempFile::new().unwrap();
        let yaml_content = r#"
runtime:
  threads: 1
customGames:
  - name: TestGame
    files:
      - /path/to/save
    integration: override
"#;
        std::fs::write(temp_file.path(), yaml_content).unwrap();

        let result = read_ludusavi_config(temp_file.path());
        assert!(result.is_ok());

        let config = result.unwrap();
        assert!(config.as_mapping().is_some());
        let mapping = config.as_mapping().unwrap();
        assert!(mapping.contains_key(serde_yml::Value::String("runtime".to_string())));
        assert!(mapping.contains_key(serde_yml::Value::String("customGames".to_string())));
    }

    #[test]
    fn test_read_nonexistent_config() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().with_extension("nonexistent");

        let result = read_ludusavi_config(&path);
        assert!(result.is_ok());

        let config = result.unwrap();
        assert!(config.as_mapping().is_some());
        assert!(config.as_mapping().unwrap().is_empty());
    }

    #[test]
    fn test_write_custom_game_to_empty_config() {
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), "{}").unwrap();

        let entry = CustomGameEntry {
            name: "MyGame".to_string(),
            files: vec!["/path/to/saves".to_string()],
            integration: "override".to_string(),
        };
        let result = write_custom_game(temp_file.path(), &entry);
        assert!(result.is_ok());

        let config = read_ludusavi_config(temp_file.path()).unwrap();
        let mapping = config.as_mapping().unwrap();
        let custom_games = mapping
            .get(serde_yml::Value::String("customGames".to_string()))
            .unwrap();
        let games_seq = custom_games.as_sequence().unwrap();
        assert_eq!(games_seq.len(), 1);

        let written_entry: CustomGameEntry = serde_yml::from_value(games_seq[0].clone()).unwrap();
        assert_eq!(written_entry.name, "MyGame");
        assert_eq!(written_entry.files, vec!["/path/to/saves".to_string()]);
        assert_eq!(written_entry.integration, "override");
    }

    #[test]
    fn test_write_custom_game_to_existing_config() {
        let temp_file = NamedTempFile::new().unwrap();
        let yaml_content = r#"
runtime:
  threads: 2
backup:
  path: /backup
"#;
        std::fs::write(temp_file.path(), yaml_content).unwrap();

        let entry = CustomGameEntry {
            name: "NewGame".to_string(),
            files: vec!["/new/saves".to_string()],
            integration: "override".to_string(),
        };
        let result = write_custom_game(temp_file.path(), &entry);
        assert!(result.is_ok());

        let config = read_ludusavi_config(temp_file.path()).unwrap();
        let mapping = config.as_mapping().unwrap();
        assert!(mapping.contains_key(serde_yml::Value::String("runtime".to_string())));
        assert!(mapping.contains_key(serde_yml::Value::String("backup".to_string())));

        let custom_games = mapping
            .get(serde_yml::Value::String("customGames".to_string()))
            .unwrap();
        let games_seq = custom_games.as_sequence().unwrap();
        assert_eq!(games_seq.len(), 1);
    }

    #[test]
    fn test_write_custom_game_updates_duplicate() {
        let temp_file = NamedTempFile::new().unwrap();
        let yaml_content = r#"
customGames:
  - name: DuplicateGame
    files:
      - /old/path
    integration: override
"#;
        std::fs::write(temp_file.path(), yaml_content).unwrap();

        let entry = CustomGameEntry {
            name: "DuplicateGame".to_string(),
            files: vec!["/new/path".to_string(), "/another/path".to_string()],
            integration: "override".to_string(),
        };

        let result = write_custom_game(temp_file.path(), &entry);
        assert!(result.is_ok());

        let config = read_ludusavi_config(temp_file.path()).unwrap();
        let mapping = config.as_mapping().unwrap();
        let custom_games = mapping
            .get(serde_yml::Value::String("customGames".to_string()))
            .unwrap();
        let games_seq = custom_games.as_sequence().unwrap();
        assert_eq!(games_seq.len(), 1);

        let updated_entry: CustomGameEntry = serde_yml::from_value(games_seq[0].clone()).unwrap();
        assert_eq!(updated_entry.name, "DuplicateGame");
        assert_eq!(updated_entry.files.len(), 2);
        assert!(updated_entry.files.contains(&"/new/path".to_string()));
        assert!(updated_entry.files.contains(&"/another/path".to_string()));
    }

    #[test]
    fn test_roundtrip_preserves_other_fields() {
        let temp_file = NamedTempFile::new().unwrap();
        let yaml_content = r#"
runtime:
  threads: 4
backup:
  path: /backups
  compression: zstd
manifest:
  url: https://example.com
customGames:
  - name: ExistingGame
    files:
      - /existing
    integration: override
"#;
        std::fs::write(temp_file.path(), yaml_content).unwrap();

        // Write a new custom game
        let entry = CustomGameEntry {
            name: "RoundtripGame".to_string(),
            files: vec!["/roundtrip/save".to_string()],
            integration: "override".to_string(),
        };
        write_custom_game(temp_file.path(), &entry).unwrap();

        let config = read_ludusavi_config(temp_file.path()).unwrap();
        let mapping = config.as_mapping().unwrap();

        assert!(mapping.contains_key(serde_yml::Value::String("runtime".to_string())));
        assert!(mapping.contains_key(serde_yml::Value::String("backup".to_string())));
        assert!(mapping.contains_key(serde_yml::Value::String("manifest".to_string())));

        let runtime = mapping
            .get(serde_yml::Value::String("runtime".to_string()))
            .unwrap();
        let runtime_map = runtime.as_mapping().unwrap();
        let threads = runtime_map
            .get(serde_yml::Value::String("threads".to_string()))
            .unwrap();
        assert_eq!(threads.as_u64(), Some(4));

        let custom_games = mapping
            .get(serde_yml::Value::String("customGames".to_string()))
            .unwrap();
        let games_seq = custom_games.as_sequence().unwrap();
        assert_eq!(games_seq.len(), 2);

        let game_names: Vec<String> = games_seq
            .iter()
            .filter_map(|g| {
                g.as_mapping().and_then(|m| {
                    m.get(serde_yml::Value::String("name".to_string()))
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                })
            })
            .collect();
        assert!(game_names.contains(&"ExistingGame".to_string()));
        assert!(game_names.contains(&"RoundtripGame".to_string()));
    }
}
