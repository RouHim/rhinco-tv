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

#[allow(dead_code)]
fn default_integration() -> String {
    "override".to_string()
}

impl CustomGameEntry {
    #[allow(dead_code)]
    pub fn new(name: String, files: Vec<String>) -> Self {
        Self {
            name,
            files,
            integration: default_integration(),
        }
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
/// Removes a custom game entry by name
/// Returns true if the game was found and removed, false otherwise
pub fn remove_custom_game(path: &Path, game_name: &str) -> Result<bool> {
    let mut config = read_ludusavi_config(path)?;

    let mapping = config
        .as_mapping_mut()
        .context("Config root is not a mapping")?;

    let custom_games = match mapping.get_mut(serde_yml::Value::String("customGames".to_string())) {
        Some(games) => games,
        None => return Ok(false),
    };

    let games_seq = custom_games
        .as_sequence_mut()
        .context("customGames is not a sequence")?;

    let original_len = games_seq.len();

    games_seq.retain(|game| {
        if let Some(game_map) = game.as_mapping() {
            if let Some(name_value) = game_map.get(serde_yml::Value::String("name".to_string())) {
                if let Some(name_str) = name_value.as_str() {
                    return name_str != game_name;
                }
            }
        }
        true
    });

    let removed = games_seq.len() < original_len;

    if removed {
        let yaml_str =
            serde_yml::to_string(&config).context("Failed to serialize config to YAML")?;
        std::fs::write(path, yaml_str).context("Failed to write Ludusavi config file")?;
    }

    Ok(removed)
}

#[allow(dead_code)]
/// Retrieves a custom game entry by name
/// Returns None if the game is not found
pub fn get_custom_game(path: &Path, game_name: &str) -> Result<Option<CustomGameEntry>> {
    let config = read_ludusavi_config(path)?;

    let mapping = config
        .as_mapping()
        .context("Config root is not a mapping")?;

    let custom_games = match mapping.get(serde_yml::Value::String("customGames".to_string())) {
        Some(games) => games,
        None => return Ok(None),
    };

    let games_seq = custom_games
        .as_sequence()
        .context("customGames is not a sequence")?;

    for game in games_seq {
        if let Some(game_map) = game.as_mapping() {
            if let Some(name_value) = game_map.get(serde_yml::Value::String("name".to_string())) {
                if let Some(name_str) = name_value.as_str() {
                    if name_str == game_name {
                        let entry: CustomGameEntry = serde_yml::from_value(game.clone())
                            .context("Failed to deserialize custom game entry")?;
                        return Ok(Some(entry));
                    }
                }
            }
        }
    }

    Ok(None)
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

        let entry = CustomGameEntry::new("MyGame".to_string(), vec!["/path/to/saves".to_string()]);

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

        let entry = CustomGameEntry::new("NewGame".to_string(), vec!["/new/saves".to_string()]);

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

        let entry = CustomGameEntry::new(
            "DuplicateGame".to_string(),
            vec!["/new/path".to_string(), "/another/path".to_string()],
        );

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
    fn test_remove_custom_game() {
        let temp_file = NamedTempFile::new().unwrap();
        let yaml_content = r#"
customGames:
  - name: Game1
    files:
      - /path1
    integration: override
  - name: Game2
    files:
      - /path2
    integration: override
"#;
        std::fs::write(temp_file.path(), yaml_content).unwrap();

        let result = remove_custom_game(temp_file.path(), "Game1");
        assert!(result.is_ok());
        assert!(result.unwrap());

        let config = read_ludusavi_config(temp_file.path()).unwrap();
        let mapping = config.as_mapping().unwrap();
        let custom_games = mapping
            .get(serde_yml::Value::String("customGames".to_string()))
            .unwrap();
        let games_seq = custom_games.as_sequence().unwrap();
        assert_eq!(games_seq.len(), 1);

        let remaining: CustomGameEntry = serde_yml::from_value(games_seq[0].clone()).unwrap();
        assert_eq!(remaining.name, "Game2");
    }

    #[test]
    fn test_remove_nonexistent_game() {
        let temp_file = NamedTempFile::new().unwrap();
        let yaml_content = r#"
customGames:
  - name: OnlyGame
    files:
      - /path
    integration: override
"#;
        std::fs::write(temp_file.path(), yaml_content).unwrap();

        let result = remove_custom_game(temp_file.path(), "NonExistent");
        assert!(result.is_ok());
        assert!(!result.unwrap());

        let config = read_ludusavi_config(temp_file.path()).unwrap();
        let mapping = config.as_mapping().unwrap();
        let custom_games = mapping
            .get(serde_yml::Value::String("customGames".to_string()))
            .unwrap();
        let games_seq = custom_games.as_sequence().unwrap();
        assert_eq!(games_seq.len(), 1);
    }

    #[test]
    fn test_get_custom_game_found() {
        let temp_file = NamedTempFile::new().unwrap();
        let yaml_content = r#"
customGames:
  - name: FoundGame
    files:
      - /save/path
    integration: extend
"#;
        std::fs::write(temp_file.path(), yaml_content).unwrap();

        let result = get_custom_game(temp_file.path(), "FoundGame");
        assert!(result.is_ok());

        let entry = result.unwrap();
        assert!(entry.is_some());

        let entry = entry.unwrap();
        assert_eq!(entry.name, "FoundGame");
        assert_eq!(entry.files, vec!["/save/path".to_string()]);
        assert_eq!(entry.integration, "extend");
    }

    #[test]
    fn test_get_custom_game_not_found() {
        let temp_file = NamedTempFile::new().unwrap();
        let yaml_content = r#"
customGames:
  - name: SomeGame
    files:
      - /path
    integration: override
"#;
        std::fs::write(temp_file.path(), yaml_content).unwrap();

        let result = get_custom_game(temp_file.path(), "NotHere");
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
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
        let entry = CustomGameEntry::new(
            "RoundtripGame".to_string(),
            vec!["/roundtrip/save".to_string()],
        );
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
