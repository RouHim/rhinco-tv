#![allow(dead_code)]

use crate::wine_prefix_scanner::SuggestedSavePath;
use std::collections::HashMap;
use std::path::Path;

pub fn detect_save_paths(
    game_name: &str,
    steam_appid: Option<&str>,
    prefix_path: &Path,
) -> Vec<SuggestedSavePath> {
    let mut suggestions = Vec::new();

    match crate::ludusavi_manifest::load_manifest() {
        Ok(manifest) => {
            suggestions.extend(crate::ludusavi_manifest::get_save_paths_from_manifest(
                &manifest,
                game_name,
                steam_appid,
                prefix_path,
            ));
        }
        Err(error) => {
            tracing::warn!(
                game_name = game_name,
                steam_appid = ?steam_appid,
                error = %error,
                "failed to load ludusavi manifest, continuing with other save sources"
            );
        }
    }

    if let Some(appid) = steam_appid {
        tracing::debug!(steam_appid = appid, "querying steam cloud save paths");
        suggestions.extend(crate::steam_cloud_paths::get_save_paths_from_steam_cloud(
            appid,
        ));
    }

    tracing::debug!(game_name = game_name, "querying heroic save paths");
    suggestions.extend(crate::heroic_save_paths::get_save_paths_from_heroic(
        game_name,
    ));

    tracing::debug!(prefix_path = %prefix_path.display(), "scanning wine prefix for saves");
    suggestions.extend(crate::wine_prefix_scanner::scan_prefix_for_saves(
        prefix_path,
    ));

    deduplicate_paths(suggestions)
}

pub fn deduplicate_paths(paths: Vec<SuggestedSavePath>) -> Vec<SuggestedSavePath> {
    let mut deduped: Vec<SuggestedSavePath> = Vec::new();
    let mut key_to_index: HashMap<String, usize> = HashMap::new();

    for candidate in paths {
        let key = dedup_key(&candidate);

        if let Some(existing_index) = key_to_index.get(&key).copied() {
            if !deduped[existing_index].exists && candidate.exists {
                deduped[existing_index] = candidate;
            }
            continue;
        }

        key_to_index.insert(key, deduped.len());
        deduped.push(candidate);
    }

    deduped
}

fn dedup_key(path: &SuggestedSavePath) -> String {
    if path.ludusavi_placeholder.trim().is_empty() {
        format!("path:{}", path.absolute_path.to_string_lossy())
    } else {
        format!(
            "placeholder:{}",
            normalize_placeholder(&path.ludusavi_placeholder)
        )
    }
}

fn normalize_placeholder(value: &str) -> String {
    value.trim().replace('\\', "/").to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_deduplicate_paths_prefers_existing_entry_for_same_placeholder() {
        let missing = SuggestedSavePath {
            absolute_path: PathBuf::from("/tmp/game/saves-a"),
            ludusavi_placeholder: "<winDocuments>/Game/Saves".to_string(),
            exists: false,
            is_empty: true,
        };
        let existing = SuggestedSavePath {
            absolute_path: PathBuf::from("/tmp/game/saves-b"),
            ludusavi_placeholder: "<windocuments>/game/saves".to_string(),
            exists: true,
            is_empty: false,
        };
        let by_absolute_path_duplicate = SuggestedSavePath {
            absolute_path: PathBuf::from("/tmp/cloud/save-slot"),
            ludusavi_placeholder: String::new(),
            exists: false,
            is_empty: true,
        };
        let by_absolute_path_existing = SuggestedSavePath {
            absolute_path: PathBuf::from("/tmp/cloud/save-slot"),
            ludusavi_placeholder: String::new(),
            exists: true,
            is_empty: false,
        };

        let deduped = deduplicate_paths(vec![
            missing,
            existing.clone(),
            by_absolute_path_duplicate,
            by_absolute_path_existing.clone(),
        ]);

        assert_eq!(deduped.len(), 2);
        assert_eq!(deduped[0], existing);
        assert_eq!(deduped[1], by_absolute_path_existing);
    }

    #[test]
    fn test_detect_save_paths_returns_vec_without_panicking() {
        let temp = match TempDir::new() {
            Ok(temp) => temp,
            Err(error) => panic!("temp dir should be created: {error}"),
        };

        let detected = detect_save_paths("nonexistent game", None, temp.path());

        assert!(
            detected
                .iter()
                .all(|entry| !entry.absolute_path.as_os_str().is_empty()),
            "detected entries should always contain a path"
        );
    }
}
