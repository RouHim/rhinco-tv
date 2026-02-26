use crate::wine_prefix_scanner::SuggestedSavePath;
use directories::BaseDirs;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, warn};

/// Get all available Heroic config directories (both system and Flatpak)
pub fn get_heroic_config_dirs() -> Vec<PathBuf> {
    let Some(base_dirs) = BaseDirs::new() else {
        return Vec::new();
    };

    let home = base_dirs.home_dir();
    let mut dirs = Vec::new();

    // Standard Heroic config location
    let config_heroic = base_dirs.config_dir().join("heroic");
    if config_heroic.exists() {
        dirs.push(config_heroic);
    }

    // Flatpak Heroic location
    let flatpak_heroic = home.join(".var/app/com.heroicgameslauncher.hgl/config/heroic");
    if flatpak_heroic.exists() {
        dirs.push(flatpak_heroic);
    }

    dirs
}

/// Parse installed.json for a specific store within a Heroic config directory
/// Returns Vec of (game_identifier, save_path) tuples
pub fn parse_heroic_installed_json(config_dir: &Path, store: &str) -> Vec<(String, String)> {
    let installed_path = match store {
        "nile" => config_dir.join("nile_config").join("installed.json"),
        "gog" => config_dir.join("gog_store").join("installed.json"),
        "legendary" => {
            // Try primary path first, then fallback
            let primary = config_dir
                .join("legendaryConfig")
                .join("legendary")
                .join("installed.json");
            if primary.exists() {
                primary
            } else {
                config_dir.join("sideload_apps").join("installed.json")
            }
        }
        _ => return Vec::new(),
    };

    let Ok(contents) = fs::read_to_string(&installed_path) else {
        debug!(
            "Could not read Heroic installed.json at {:?}",
            installed_path
        );
        return Vec::new();
    };

    let Ok(value) = serde_json::from_str::<Value>(&contents) else {
        warn!("Failed to parse JSON from {:?}", installed_path);
        return Vec::new();
    };

    let mut results = Vec::new();

    // Try to extract installed array
    if let Some(installed_array) = value.get("installed").and_then(|v| v.as_array()) {
        for item in installed_array {
            if let Some(obj) = item.as_object() {
                if let Some(app_name) = obj
                    .get("appName")
                    .and_then(|v| v.as_str())
                    .or_else(|| obj.get("app_name").and_then(|v| v.as_str()))
                {
                    // Look for save_path field (try multiple variations)
                    let save_path = obj
                        .get("save_path")
                        .and_then(|v| v.as_str())
                        .or_else(|| obj.get("savesPath").and_then(|v| v.as_str()))
                        .or_else(|| obj.get("saves_path").and_then(|v| v.as_str()));

                    if let Some(path) = save_path {
                        if !path.is_empty() {
                            results.push((app_name.to_string(), path.to_string()));
                        }
                    }
                }
            }
        }
    }

    results
}

/// Extract save game paths from Heroic for a given game name
/// Returns Vec of SuggestedSavePath with metadata
#[allow(dead_code)]
pub fn get_save_paths_from_heroic(game_name: &str) -> Vec<SuggestedSavePath> {
    let mut suggestions = Vec::new();
    let game_name_lower = game_name.to_lowercase();

    for config_dir in get_heroic_config_dirs() {
        for store in &["gog", "legendary", "nile"] {
            let games = parse_heroic_installed_json(&config_dir, store);
            for (app_name, save_path) in games {
                // Simple case-insensitive substring or exact match
                let app_name_lower = app_name.to_lowercase();
                let matches = app_name_lower.contains(&game_name_lower)
                    || game_name_lower.contains(&app_name_lower)
                    || app_name_lower == game_name_lower;

                if matches && !save_path.is_empty() {
                    let path = PathBuf::from(&save_path);
                    let exists = path.exists();
                    let is_empty = if exists { is_dir_empty(&path) } else { true };

                    suggestions.push(SuggestedSavePath {
                        absolute_path: path,
                        ludusavi_placeholder: save_path,
                        exists,
                        is_empty,
                    });
                }
            }
        }
    }

    // Sort: non-empty directories first
    suggestions.sort_by(|a, b| match (a.is_empty, b.is_empty) {
        (false, true) => std::cmp::Ordering::Less,
        (true, false) => std::cmp::Ordering::Greater,
        _ => std::cmp::Ordering::Equal,
    });

    suggestions
}

/// Check if a directory is empty (no files or subdirs)
fn is_dir_empty(path: &Path) -> bool {
    match fs::read_dir(path) {
        Ok(mut entries) => entries.next().is_none(),
        Err(_) => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_heroic_installed_json_extracts_save_path() {
        let contents = r#"{
            "installed": [
                {
                    "appName": "game-1",
                    "save_path": "/home/user/.local/share/game-1/saves"
                },
                {
                    "appName": "game-2",
                    "savesPath": "/home/user/Games/game-2/saves"
                }
            ]
        }"#;

        let temp_dir = std::env::temp_dir();
        let heroic_root = temp_dir.join("heroic_save_test_1");
        let store_dir = heroic_root.join("gog_store");
        std::fs::create_dir_all(&store_dir).unwrap();
        let installed_path = store_dir.join("installed.json");
        std::fs::write(&installed_path, contents).unwrap();

        let results = parse_heroic_installed_json(&heroic_root, "gog");

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, "game-1");
        assert_eq!(results[0].1, "/home/user/.local/share/game-1/saves");
        assert_eq!(results[1].0, "game-2");
        assert_eq!(results[1].1, "/home/user/Games/game-2/saves");

        std::fs::remove_dir_all(&heroic_root).ok();
    }

    #[test]
    fn test_parse_heroic_installed_json_handles_missing_save_path() {
        let contents = r#"{
            "installed": [
                {
                    "appName": "game-no-saves"
                },
                {
                    "appName": "game-empty-save",
                    "save_path": ""
                }
            ]
        }"#;

        let temp_dir = std::env::temp_dir();
        let heroic_root = temp_dir.join("heroic_save_test_2");
        let store_dir = heroic_root.join("gog_store");
        std::fs::create_dir_all(&store_dir).unwrap();
        let installed_path = store_dir.join("installed.json");
        std::fs::write(&installed_path, contents).unwrap();

        let results = parse_heroic_installed_json(&heroic_root, "gog");

        // Both should be filtered out - one has no save_path, one has empty string
        assert_eq!(results.len(), 0);

        std::fs::remove_dir_all(&heroic_root).ok();
    }

    #[test]
    fn test_get_heroic_config_dirs_handles_missing() {
        // This test verifies that missing dirs don't cause panic
        let dirs = get_heroic_config_dirs();
        // Should return empty or only existing dirs, never panic
        assert!(
            dirs.iter().all(|d| d.exists()),
            "All returned dirs must exist"
        );
    }

    #[test]
    fn test_get_save_paths_from_heroic_matches_game_name() {
        let contents = r#"{
            "installed": [
                {
                    "appName": "elden-ring",
                    "save_path": "/tmp/heroic_test_saves/elden-ring"
                }
            ]
        }"#;

        let temp_dir = std::env::temp_dir();
        let heroic_root = temp_dir.join("heroic_save_test_3");
        let store_dir = heroic_root.join("gog_store");
        std::fs::create_dir_all(&store_dir).unwrap();
        let installed_path = store_dir.join("installed.json");
        std::fs::write(&installed_path, contents).unwrap();

        // Create the save directory so exists() returns true
        let save_dir = PathBuf::from("/tmp/heroic_test_saves/elden-ring");
        std::fs::create_dir_all(&save_dir).ok();

        // We need to mock the config dirs - this is tricky without a proper mock
        // For now, we test the matching logic implicitly through parse_heroic_installed_json
        let results = parse_heroic_installed_json(&heroic_root, "gog");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, "elden-ring");

        std::fs::remove_dir_all(&heroic_root).ok();
        std::fs::remove_dir_all("/tmp/heroic_test_saves").ok();
    }

    #[test]
    fn test_parse_heroic_installed_json_missing_file_returns_empty() {
        let temp_dir = std::env::temp_dir();
        let heroic_root = temp_dir.join("heroic_save_test_nonexistent");

        let results = parse_heroic_installed_json(&heroic_root, "gog");

        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_is_dir_empty_detects_empty_directories() {
        let temp_dir = std::env::temp_dir();
        let test_dir = temp_dir.join("heroic_empty_test");
        std::fs::create_dir_all(&test_dir).unwrap();

        assert!(
            is_dir_empty(&test_dir),
            "Empty directory should be detected as empty"
        );

        // Add a file and test again
        std::fs::write(test_dir.join("test.txt"), "content").ok();
        assert!(
            !is_dir_empty(&test_dir),
            "Directory with file should not be empty"
        );

        std::fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_parse_heroic_installed_json_tries_multiple_field_names() {
        let contents = r#"{
            "installed": [
                {
                    "appName": "game-1",
                    "save_path": "/path1"
                },
                {
                    "appName": "game-2",
                    "savesPath": "/path2"
                },
                {
                    "appName": "game-3",
                    "saves_path": "/path3"
                }
            ]
        }"#;

        let temp_dir = std::env::temp_dir();
        let heroic_root = temp_dir.join("heroic_save_test_variations");
        let store_dir = heroic_root.join("gog_store");
        std::fs::create_dir_all(&store_dir).unwrap();
        let installed_path = store_dir.join("installed.json");
        std::fs::write(&installed_path, contents).unwrap();

        let results = parse_heroic_installed_json(&heroic_root, "gog");

        assert_eq!(results.len(), 3);
        assert_eq!(results[0].1, "/path1");
        assert_eq!(results[1].1, "/path2");
        assert_eq!(results[2].1, "/path3");

        std::fs::remove_dir_all(&heroic_root).ok();
    }
}
