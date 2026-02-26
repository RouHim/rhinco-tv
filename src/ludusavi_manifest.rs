#![allow(dead_code)]

use crate::wine_prefix_scanner::SuggestedSavePath;
use anyhow::{anyhow, Context};
use directories::BaseDirs;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

const LUDUSAVI_MANIFEST_URL: &str =
    "https://raw.githubusercontent.com/mtkennerly/ludusavi-manifest/master/data/manifest.yaml";

#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct LudusaviManifest(pub HashMap<String, ManifestGame>);

#[derive(Debug, Clone, Deserialize)]
pub struct ManifestGame {
    pub files: Option<HashMap<String, ManifestFileEntry>>,
    pub steam: Option<ManifestSteam>,
    #[serde(rename = "installDir")]
    pub install_dir: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ManifestFileEntry {
    pub tags: Option<Vec<String>>,
    pub when: Option<Vec<ManifestWhen>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ManifestSteam {
    pub id: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ManifestWhen {
    pub os: Option<String>,
    pub store: Option<String>,
}

pub fn load_manifest() -> Result<LudusaviManifest, anyhow::Error> {
    let local_manifest_path = BaseDirs::new().map(|dirs| {
        dirs.home_dir()
            .join(".config")
            .join("ludusavi")
            .join("manifest.yaml")
    });

    let mut local_error: Option<anyhow::Error> = None;
    if let Some(path) = local_manifest_path {
        match fs::read_to_string(&path) {
            Ok(content) => {
                return serde_yml::from_str::<LudusaviManifest>(&content).with_context(|| {
                    format!("failed to parse local manifest at {}", path.display())
                });
            }
            Err(err) => {
                tracing::warn!(
                    path = %path.display(),
                    error = %err,
                    "failed to read local ludusavi manifest, trying remote"
                );
                local_error = Some(anyhow!(err).context(format!(
                    "failed to read local manifest at {}",
                    path.display()
                )));
            }
        }
    } else {
        tracing::warn!("could not resolve home directory for local ludusavi manifest path");
    }

    tracing::info!(url = LUDUSAVI_MANIFEST_URL, "downloading ludusavi manifest");
    let mut response = match ureq::get(LUDUSAVI_MANIFEST_URL).call() {
        Ok(response) => response,
        Err(err) => {
            tracing::warn!(error = %err, "failed to download ludusavi manifest");
            let mut combined = anyhow!(err).context("failed to download remote ludusavi manifest");
            if let Some(local_error) = local_error {
                combined = combined.context(format!(
                    "local manifest fallback also failed: {local_error}"
                ));
            }
            return Err(combined);
        }
    };

    let content = response
        .body_mut()
        .read_to_string()
        .context("failed to read remote ludusavi manifest response body")?;

    serde_yml::from_str::<LudusaviManifest>(&content)
        .context("failed to parse remote ludusavi manifest yaml")
}

pub fn find_game_by_steam_appid(
    manifest: &LudusaviManifest,
    appid: u32,
) -> Option<(&str, &ManifestGame)> {
    manifest
        .0
        .iter()
        .find(|(_, game)| game.steam.as_ref().and_then(|steam| steam.id) == Some(appid))
        .map(|(name, game)| (name.as_str(), game))
}

pub fn find_game_by_name<'a>(
    manifest: &'a LudusaviManifest,
    name: &str,
) -> Option<(&'a str, &'a ManifestGame)> {
    let normalized_input = normalize_game_name(name);
    let input_tokens = normalized_input.split_whitespace().collect::<Vec<_>>();

    if let Some(matched) = manifest
        .0
        .iter()
        .find(|(game_name, _)| normalize_game_name(game_name) == normalized_input)
    {
        return Some((matched.0.as_str(), matched.1));
    }

    if input_tokens.len() >= 2 {
        if let Some(matched) = manifest.0.iter().find(|(game_name, _)| {
            let normalized_candidate = normalize_game_name(game_name);
            let mut candidate_tokens = normalized_candidate.split_whitespace().collect::<Vec<_>>();

            if candidate_tokens.first() == Some(&"the") {
                candidate_tokens.remove(0);
            }

            candidate_tokens.starts_with(&input_tokens)
        }) {
            return Some((matched.0.as_str(), matched.1));
        }
    }

    let threshold = if normalized_input.len() < 10 {
        0.90
    } else {
        0.85
    };

    manifest
        .0
        .iter()
        .filter_map(|(game_name, game)| {
            let normalized_candidate = normalize_game_name(game_name);
            let score = strsim::jaro_winkler(&normalized_input, &normalized_candidate);
            if score >= threshold {
                Some((game_name.as_str(), game, score))
            } else {
                None
            }
        })
        .max_by(|left, right| left.2.total_cmp(&right.2))
        .map(|(name, game, _)| (name, game))
}

pub fn resolve_placeholder(placeholder_path: &str, prefix_path: &Path) -> Option<PathBuf> {
    if placeholder_path.contains("<base>") {
        return None;
    }

    let separator = placeholder_path.find('/').unwrap_or(placeholder_path.len());
    let placeholder = &placeholder_path[..separator];
    let remainder = if separator < placeholder_path.len() {
        &placeholder_path[separator + 1..]
    } else {
        ""
    };

    let home = std::env::var("HOME")
        .ok()
        .map(PathBuf::from)
        .or_else(|| BaseDirs::new().map(|dirs| dirs.home_dir().to_path_buf()));

    let base = match placeholder {
        "<winDocuments>" => prefix_path
            .join("drive_c")
            .join("users")
            .join("steamuser")
            .join("Documents"),
        "<winAppData>" => prefix_path
            .join("drive_c")
            .join("users")
            .join("steamuser")
            .join("AppData")
            .join("Roaming"),
        "<winLocalAppData>" => prefix_path
            .join("drive_c")
            .join("users")
            .join("steamuser")
            .join("AppData")
            .join("Local"),
        "<winSavedGames>" => prefix_path
            .join("drive_c")
            .join("users")
            .join("steamuser")
            .join("Saved Games"),
        "<winUserProfile>" => prefix_path.join("drive_c").join("users").join("steamuser"),
        "<winProgramData>" => prefix_path.join("drive_c").join("ProgramData"),
        "<home>" => home?,
        "<xdgConfig>" => home?.join(".config"),
        "<xdgData>" => home?.join(".local").join("share"),
        _ => return None,
    };

    if remainder.is_empty() {
        Some(base)
    } else {
        Some(base.join(remainder))
    }
}

pub fn get_save_paths_from_manifest(
    manifest: &LudusaviManifest,
    game_name: &str,
    steam_appid: Option<&str>,
    prefix_path: &Path,
) -> Vec<SuggestedSavePath> {
    tracing::debug!(game_name = game_name, steam_appid = ?steam_appid, "looking up manifest save paths");

    let game_match = steam_appid
        .and_then(|appid| appid.parse::<u32>().ok())
        .and_then(|appid| {
            tracing::debug!(appid = appid, "trying ludusavi lookup by steam appid");
            find_game_by_steam_appid(manifest, appid)
        })
        .or_else(|| {
            tracing::debug!(game_name = game_name, "trying ludusavi lookup by game name");
            find_game_by_name(manifest, game_name)
        });

    let Some((matched_name, game)) = game_match else {
        tracing::debug!(game_name = game_name, "no ludusavi manifest entry matched");
        return Vec::new();
    };

    tracing::debug!(matched_name = matched_name, "matched ludusavi game entry");

    let mut suggestions = Vec::new();
    let Some(files) = &game.files else {
        return suggestions;
    };

    for (placeholder, metadata) in files {
        if !is_supported_os_condition(metadata.when.as_deref()) {
            continue;
        }

        let Some(absolute_path) = resolve_placeholder(placeholder, prefix_path) else {
            continue;
        };

        let exists = absolute_path.exists();
        let is_empty = if exists && absolute_path.is_dir() {
            fs::read_dir(&absolute_path)
                .map(|mut entries| entries.next().is_none())
                .unwrap_or(true)
        } else {
            false
        };

        suggestions.push(SuggestedSavePath {
            absolute_path,
            ludusavi_placeholder: placeholder.clone(),
            exists,
            is_empty,
        });
    }

    suggestions
}

fn normalize_game_name(name: &str) -> String {
    let mut normalized = name.to_lowercase();
    for character in ['™', '®', '©', ':', '-'] {
        normalized = normalized.replace(character, " ");
    }

    normalized = collapse_whitespace(&normalized);

    loop {
        let trimmed = normalized.trim();
        if !trimmed.ends_with(')') {
            break;
        }

        let Some(opening_idx) = trimmed.rfind(" (") else {
            break;
        };

        let inner = &trimmed[opening_idx + 2..trimmed.len() - 1];
        if inner.len() == 4 && inner.chars().all(|c| c.is_ascii_digit()) {
            normalized = trimmed[..opening_idx].trim().to_string();
        } else {
            break;
        }
    }

    let suffixes = [
        " game of the year",
        " definitive edition",
        " complete edition",
        " ultimate edition",
        " deluxe edition",
        " remastered",
    ];

    for suffix in suffixes {
        if normalized.ends_with(suffix) {
            normalized = normalized.trim_end_matches(suffix).trim().to_string();
        }
    }

    collapse_whitespace(&normalized)
}

fn collapse_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn is_supported_os_condition(conditions: Option<&[ManifestWhen]>) -> bool {
    let Some(conditions) = conditions else {
        return true;
    };

    conditions.iter().any(|condition| {
        condition
            .os
            .as_deref()
            .map(|os| {
                let lower = os.to_lowercase();
                lower == "windows" || lower == "linux"
            })
            .unwrap_or(false)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    const TEST_MANIFEST_YAML: &str = r#"
"The Witcher 3: Wild Hunt":
  files:
    "<winDocuments>/The Witcher 3/gamesaves":
      when:
        - os: windows
  installDir:
    "The Witcher 3 Wild Hunt": {}
  steam:
    id: 292030
"DOOM Eternal":
  files:
    "<winDocuments>/id Software/DOOMEternal/saved":
      when:
        - os: windows
  steam:
    id: 782330
"#;

    fn parse_manifest(yaml: &str) -> LudusaviManifest {
        match serde_yml::from_str::<LudusaviManifest>(yaml) {
            Ok(manifest) => manifest,
            Err(error) => panic!("test manifest should parse: {error}"),
        }
    }

    #[test]
    fn test_find_game_by_steam_appid_returns_correct_game() {
        let manifest = parse_manifest(TEST_MANIFEST_YAML);

        let matched = find_game_by_steam_appid(&manifest, 292030);

        assert!(matched.is_some());
        let (name, game) = match matched {
            Some(matched) => matched,
            None => panic!("game should be matched by appid"),
        };
        assert_eq!(name, "The Witcher 3: Wild Hunt");
        assert_eq!(game.steam.as_ref().and_then(|s| s.id), Some(292030));
    }

    #[test]
    fn test_fuzzy_name_match_applies_threshold_for_short_names() {
        let manifest = parse_manifest(TEST_MANIFEST_YAML);

        let witcher = find_game_by_name(&manifest, "Witcher 3");
        assert!(witcher.is_some());
        assert_eq!(
            witcher.map(|(name, _)| name),
            Some("The Witcher 3: Wild Hunt")
        );

        let doom = find_game_by_name(&manifest, "DOOM");
        assert!(doom.is_none());
    }

    #[test]
    fn test_resolve_placeholder_win_documents_returns_expected_path() {
        let temp = match TempDir::new() {
            Ok(temp) => temp,
            Err(error) => panic!("temp dir should be created: {error}"),
        };
        let prefix = temp.path().join("pfx");

        let resolved = resolve_placeholder("<winDocuments>/Game/saves", &prefix);

        assert_eq!(
            resolved,
            Some(
                prefix
                    .join("drive_c")
                    .join("users")
                    .join("steamuser")
                    .join("Documents")
                    .join("Game")
                    .join("saves")
            )
        );
    }

    #[test]
    fn test_resolve_placeholder_skips_base_placeholder() {
        let temp = match TempDir::new() {
            Ok(temp) => temp,
            Err(error) => panic!("temp dir should be created: {error}"),
        };
        let prefix = temp.path().join("pfx");

        let resolved = resolve_placeholder("<base>/saves", &prefix);

        assert!(resolved.is_none());
    }

    #[test]
    fn test_get_save_paths_from_manifest_with_appid_returns_suggested_path() {
        let manifest = parse_manifest(TEST_MANIFEST_YAML);
        let temp = match TempDir::new() {
            Ok(temp) => temp,
            Err(error) => panic!("temp dir should be created: {error}"),
        };
        let prefix = temp.path().join("pfx");

        let target = prefix
            .join("drive_c")
            .join("users")
            .join("steamuser")
            .join("Documents")
            .join("The Witcher 3")
            .join("gamesaves");
        assert!(
            fs::create_dir_all(&target).is_ok(),
            "target save directory should be created"
        );
        assert!(
            fs::write(target.join("save_1.sav"), b"save").is_ok(),
            "save file should be created"
        );

        let saves = get_save_paths_from_manifest(
            &manifest,
            "Completely Different Name",
            Some("292030"),
            &prefix,
        );

        assert_eq!(saves.len(), 1);
        let first = &saves[0];
        assert_eq!(
            first.ludusavi_placeholder,
            "<winDocuments>/The Witcher 3/gamesaves"
        );
        assert_eq!(first.absolute_path, target);
        assert!(first.exists);
        assert!(!first.is_empty);
    }
}
