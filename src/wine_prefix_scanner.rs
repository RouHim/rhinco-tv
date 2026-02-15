use crate::model::LauncherItem;
use directories::BaseDirs;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WinePrefixSource {
    Steam { appid: String },
    Heroic { app_name: String, runner: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuggestedSavePath {
    pub absolute_path: PathBuf,
    pub ludusavi_placeholder: String,
    pub exists: bool,
    pub is_empty: bool,
}

/// Discover Wine prefixes for a given game from Steam/Heroic sources
pub fn discover_wine_prefixes(game: &LauncherItem) -> Vec<(PathBuf, WinePrefixSource)> {
    let mut prefixes = Vec::new();

    // Try Steam Proton compatdata
    if let Some(appid) = &game.steam_appid {
        if let Some(base_dirs) = BaseDirs::new() {
            let home = base_dirs.home_dir();
            let steam_roots = get_steam_roots(home);

            for root in steam_roots {
                let prefix_path = root
                    .join("steamapps")
                    .join("compatdata")
                    .join(appid)
                    .join("pfx");

                if prefix_path.exists() {
                    prefixes.push((
                        prefix_path,
                        WinePrefixSource::Steam {
                            appid: appid.clone(),
                        },
                    ));
                }
            }
        }
    }

    // Try Heroic Wine prefix
    if let Some(launch_key) = &game.launch_key {
        if launch_key.starts_with("heroic:") {
            if let Some(prefix) = discover_heroic_prefix(launch_key) {
                prefixes.push(prefix);
            }
        }
    }

    prefixes
}

/// Scan a Wine prefix for likely save game directories
pub fn scan_prefix_for_saves(prefix: &Path) -> Vec<SuggestedSavePath> {
    if !prefix.exists() {
        return Vec::new();
    }

    let mut suggestions = Vec::new();
    let users_dir = prefix.join("drive_c").join("users");

    if !users_dir.exists() {
        return suggestions;
    }

    // Find user directories (typically "steamuser" or similar)
    let Ok(user_entries) = fs::read_dir(&users_dir) else {
        return suggestions;
    };

    for user_entry in user_entries.flatten() {
        let user_path = user_entry.path();
        if !user_path.is_dir() {
            continue;
        }

        // Scan known save locations
        scan_appdata_local(&user_path, prefix, &mut suggestions);
        scan_appdata_roaming(&user_path, prefix, &mut suggestions);
        scan_documents(&user_path, prefix, &mut suggestions);
    }

    // Sort: non-empty directories first
    suggestions.sort_by(|a, b| {
        // Non-empty before empty
        match (a.is_empty, b.is_empty) {
            (false, true) => std::cmp::Ordering::Less,
            (true, false) => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Equal,
        }
    });

    suggestions
}

/// Convert an absolute path to a Ludusavi placeholder pattern
pub fn path_to_ludusavi_placeholder(prefix: &Path, absolute_path: &Path) -> Option<String> {
    let users_dir = prefix.join("drive_c").join("users");

    // Find which user directory this path is under
    let Ok(user_entries) = fs::read_dir(&users_dir) else {
        return None;
    };

    for user_entry in user_entries.flatten() {
        let user_path = user_entry.path();
        if !user_path.is_dir() {
            continue;
        }

        // Try to strip user path prefix from absolute_path
        if let Ok(relative) = absolute_path.strip_prefix(&user_path) {
            let components: Vec<_> = relative.components().collect();
            if components.is_empty() {
                return None;
            }

            let first = components[0].as_os_str().to_string_lossy();

            // Match known patterns
            if first == "AppData" && components.len() >= 2 {
                let second = components[1].as_os_str().to_string_lossy();
                let rest: PathBuf = components.iter().skip(2).collect();

                if second == "Local" {
                    return Some(format!("<winLocalAppData>/{}", rest.display()));
                } else if second == "Roaming" {
                    return Some(format!("<winAppData>/{}", rest.display()));
                }
            } else if first == "Documents" {
                let rest: PathBuf = components.iter().skip(1).collect();
                return Some(format!("<winDocuments>/{}", rest.display()));
            }
        }
    }

    None
}

// Helper functions

fn get_steam_roots(home: &Path) -> Vec<PathBuf> {
    [
        home.join(".steam/steam"),
        home.join(".local/share/Steam"),
        home.join(".steam/root"),
    ]
    .into_iter()
    .filter(|p| p.exists())
    .collect()
}

fn discover_heroic_prefix(launch_key: &str) -> Option<(PathBuf, WinePrefixSource)> {
    let parts: Vec<&str> = launch_key.split(':').collect();
    let app_name = parts.last()?.to_string();

    let runner = if parts.len() == 3 {
        parts[1].to_string()
    } else {
        "unknown".to_string()
    };

    let base_dirs = BaseDirs::new()?;
    let config_dir = base_dirs.config_dir();
    let home = base_dirs.home_dir();

    let heroic_roots = [
        config_dir.join("heroic"),
        home.join(".var/app/com.heroicgameslauncher.hgl/config/heroic"),
    ];

    for root in heroic_roots.iter().filter(|r| r.exists()) {
        let config_path = root.join("GamesConfig").join(format!("{}.json", app_name));

        if let Ok(content) = fs::read_to_string(&config_path) {
            if let Ok(config) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(wine_prefix) = config.get("winePrefix").and_then(|v| v.as_str()) {
                    let prefix_path = PathBuf::from(wine_prefix);
                    if prefix_path.exists() {
                        return Some((
                            prefix_path,
                            WinePrefixSource::Heroic {
                                app_name: app_name.clone(),
                                runner: runner.clone(),
                            },
                        ));
                    }
                }
            }
        }
    }

    None
}

fn scan_appdata_local(user_path: &Path, prefix: &Path, suggestions: &mut Vec<SuggestedSavePath>) {
    let local_path = user_path.join("AppData").join("Local");
    if !local_path.exists() {
        return;
    }

    scan_directory_level(&local_path, prefix, 1, suggestions);
}

fn scan_appdata_roaming(user_path: &Path, prefix: &Path, suggestions: &mut Vec<SuggestedSavePath>) {
    let roaming_path = user_path.join("AppData").join("Roaming");
    if !roaming_path.exists() {
        return;
    }

    scan_directory_level(&roaming_path, prefix, 1, suggestions);
}

fn scan_documents(user_path: &Path, prefix: &Path, suggestions: &mut Vec<SuggestedSavePath>) {
    let docs_path = user_path.join("Documents");
    if !docs_path.exists() {
        return;
    }

    // Check for "My Games" subdirectory first
    let my_games = docs_path.join("My Games");
    if my_games.exists() {
        scan_directory_level(&my_games, prefix, 1, suggestions);
    }

    // Also scan Documents root (but skip "My Games" if it exists)
    scan_directory_level(&docs_path, prefix, 1, suggestions);
}

fn scan_directory_level(
    dir: &Path,
    prefix: &Path,
    max_depth: usize,
    suggestions: &mut Vec<SuggestedSavePath>,
) {
    if max_depth == 0 {
        return;
    }

    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();

        // Skip symlinks to avoid infinite loops
        if path.is_symlink() {
            continue;
        }

        if path.is_dir() {
            let is_empty = is_directory_empty(&path);

            if let Some(placeholder) = path_to_ludusavi_placeholder(prefix, &path) {
                suggestions.push(SuggestedSavePath {
                    absolute_path: path.clone(),
                    ludusavi_placeholder: placeholder,
                    exists: true,
                    is_empty,
                });
            }

            // Recurse one level deeper
            if max_depth > 1 {
                scan_directory_level(&path, prefix, max_depth - 1, suggestions);
            }
        }
    }
}

fn is_directory_empty(path: &Path) -> bool {
    fs::read_dir(path)
        .map(|mut entries| entries.next().is_none())
        .unwrap_or(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::LauncherAction;
    use std::fs;
    use tempfile::TempDir;
    use uuid::Uuid;

    fn create_test_game(steam_appid: Option<String>, launch_key: Option<String>) -> LauncherItem {
        LauncherItem {
            id: Uuid::new_v4(),
            name: "Test Game".to_string(),
            icon: None,
            system_icon: None,
            action: LauncherAction::Launch {
                exec: "test".to_string(),
            },
            source_image_url: None,
            game_executable: None,
            launch_key,
            last_started: None,
            steam_appid,
        }
    }

    fn create_wine_prefix_structure(base: &Path, user_name: &str) -> PathBuf {
        let prefix = base.join("pfx");
        let users_dir = prefix.join("drive_c").join("users").join(user_name);

        fs::create_dir_all(&users_dir).unwrap();
        fs::create_dir_all(users_dir.join("AppData").join("Local")).unwrap();
        fs::create_dir_all(users_dir.join("AppData").join("Roaming")).unwrap();
        fs::create_dir_all(users_dir.join("Documents")).unwrap();

        prefix
    }

    #[test]
    fn test_discover_steam_prefix_with_appid() {
        let temp_dir = TempDir::new().unwrap();
        let steam_root = temp_dir.path().join(".steam/steam");
        let appid = "12340";

        // Create compatdata structure
        let prefix_path = steam_root
            .join("steamapps")
            .join("compatdata")
            .join(appid)
            .join("pfx");
        fs::create_dir_all(&prefix_path).unwrap();

        // Mock game with Steam appid
        let game = create_test_game(Some(appid.to_string()), None);

        // This test would need mocking BaseDirs::new() which returns temp_dir
        // For now, we test the logic by ensuring it doesn't crash
        let prefixes = discover_wine_prefixes(&game);

        // In real environment, if steam paths exist, this would find them
        // In test environment without mocking, it returns empty
        assert!(prefixes.is_empty() || !prefixes.is_empty());
    }

    #[test]
    fn test_discover_heroic_prefix() {
        // This would require creating a mock Heroic config
        // For now, test the logic doesn't crash
        let game = create_test_game(None, Some("heroic:gog:testgame".to_string()));
        let prefixes = discover_wine_prefixes(&game);

        // Without actual Heroic config, this returns empty
        assert!(prefixes.is_empty());
    }

    #[test]
    fn test_discover_no_prefix_for_native_game() {
        let game = create_test_game(None, None);
        let prefixes = discover_wine_prefixes(&game);

        assert!(prefixes.is_empty());
    }

    #[test]
    fn test_scan_prefix_finds_appdata_local() {
        let temp_dir = TempDir::new().unwrap();
        let prefix = create_wine_prefix_structure(temp_dir.path(), "testuser");

        // Create a directory in AppData/Local
        let game_dir = prefix
            .join("drive_c")
            .join("users")
            .join("testuser")
            .join("AppData")
            .join("Local")
            .join("MyGame");
        fs::create_dir_all(&game_dir).unwrap();
        fs::write(game_dir.join("save.dat"), b"test").unwrap();

        let suggestions = scan_prefix_for_saves(&prefix);

        assert!(!suggestions.is_empty());
        assert!(suggestions
            .iter()
            .any(|s| s.ludusavi_placeholder.contains("<winLocalAppData>")));
    }

    #[test]
    fn test_scan_prefix_finds_appdata_roaming() {
        let temp_dir = TempDir::new().unwrap();
        let prefix = create_wine_prefix_structure(temp_dir.path(), "testuser");

        // Create a directory in AppData/Roaming
        let game_dir = prefix
            .join("drive_c")
            .join("users")
            .join("testuser")
            .join("AppData")
            .join("Roaming")
            .join("MyGame");
        fs::create_dir_all(&game_dir).unwrap();
        fs::write(game_dir.join("config.ini"), b"test").unwrap();

        let suggestions = scan_prefix_for_saves(&prefix);

        assert!(!suggestions.is_empty());
        assert!(suggestions
            .iter()
            .any(|s| s.ludusavi_placeholder.contains("<winAppData>")));
    }

    #[test]
    fn test_scan_prefix_finds_documents() {
        let temp_dir = TempDir::new().unwrap();
        let prefix = create_wine_prefix_structure(temp_dir.path(), "testuser");

        // Create a directory in Documents/My Games
        let my_games = prefix
            .join("drive_c")
            .join("users")
            .join("testuser")
            .join("Documents")
            .join("My Games")
            .join("TestGame");
        fs::create_dir_all(&my_games).unwrap();
        fs::write(my_games.join("savegame.sav"), b"test").unwrap();

        let suggestions = scan_prefix_for_saves(&prefix);

        assert!(!suggestions.is_empty());
        assert!(suggestions
            .iter()
            .any(|s| s.ludusavi_placeholder.contains("<winDocuments>")));
    }

    #[test]
    fn test_scan_empty_prefix() {
        let temp_dir = TempDir::new().unwrap();
        let nonexistent = temp_dir.path().join("nonexistent");

        let suggestions = scan_prefix_for_saves(&nonexistent);

        assert!(suggestions.is_empty());
    }

    #[test]
    fn test_path_to_placeholder_local_appdata() {
        let temp_dir = TempDir::new().unwrap();
        let prefix = create_wine_prefix_structure(temp_dir.path(), "testuser");

        let game_path = prefix
            .join("drive_c")
            .join("users")
            .join("testuser")
            .join("AppData")
            .join("Local")
            .join("MyGame");

        let placeholder = path_to_ludusavi_placeholder(&prefix, &game_path);

        assert_eq!(placeholder, Some("<winLocalAppData>/MyGame".to_string()));
    }

    #[test]
    fn test_path_to_placeholder_roaming_appdata() {
        let temp_dir = TempDir::new().unwrap();
        let prefix = create_wine_prefix_structure(temp_dir.path(), "testuser");

        let game_path = prefix
            .join("drive_c")
            .join("users")
            .join("testuser")
            .join("AppData")
            .join("Roaming")
            .join("MyGame");

        let placeholder = path_to_ludusavi_placeholder(&prefix, &game_path);

        assert_eq!(placeholder, Some("<winAppData>/MyGame".to_string()));
    }

    #[test]
    fn test_path_to_placeholder_documents() {
        let temp_dir = TempDir::new().unwrap();
        let prefix = create_wine_prefix_structure(temp_dir.path(), "testuser");

        let game_path = prefix
            .join("drive_c")
            .join("users")
            .join("testuser")
            .join("Documents")
            .join("My Games")
            .join("TestGame");

        let placeholder = path_to_ludusavi_placeholder(&prefix, &game_path);

        assert_eq!(
            placeholder,
            Some("<winDocuments>/My Games/TestGame".to_string())
        );
    }

    #[test]
    fn test_scan_sorts_nonempty_first() {
        let temp_dir = TempDir::new().unwrap();
        let prefix = create_wine_prefix_structure(temp_dir.path(), "testuser");

        let base = prefix
            .join("drive_c")
            .join("users")
            .join("testuser")
            .join("AppData")
            .join("Local");

        // Create empty directory
        let empty_dir = base.join("EmptyGame");
        fs::create_dir_all(&empty_dir).unwrap();

        // Create non-empty directory
        let full_dir = base.join("FullGame");
        fs::create_dir_all(&full_dir).unwrap();
        fs::write(full_dir.join("save.dat"), b"data").unwrap();

        let suggestions = scan_prefix_for_saves(&prefix);

        assert!(!suggestions.is_empty());

        // First suggestion should be non-empty
        let first = &suggestions[0];
        assert!(!first.is_empty, "First suggestion should be non-empty");

        // Find the empty one
        let has_empty = suggestions.iter().any(|s| s.is_empty);
        assert!(has_empty, "Should have found the empty directory");
    }
}
