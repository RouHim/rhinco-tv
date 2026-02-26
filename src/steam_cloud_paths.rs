#![allow(dead_code)]
use crate::wine_prefix_scanner::SuggestedSavePath;
use directories::BaseDirs;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use tracing::{debug, warn};

pub fn get_steam_roots() -> Vec<PathBuf> {
    let Some(base_dirs) = BaseDirs::new() else {
        return Vec::new();
    };

    let home = base_dirs.home_dir();
    [
        home.join(".steam/steam"),
        home.join(".local/share/Steam"),
        home.join(".steam/root"),
    ]
    .into_iter()
    .filter(|path| path.exists())
    .collect()
}

pub fn parse_remotestorage_log(steam_root: &Path, steam_appid: &str) -> Vec<PathBuf> {
    let log_path = steam_root.join("logs").join("remotestorage_log.txt");
    let file = match File::open(&log_path) {
        Ok(file) => file,
        Err(error) => {
            debug!(
                "Could not open Steam remotestorage log {:?}: {}",
                log_path, error
            );
            return Vec::new();
        }
    };

    let mut seen = HashSet::new();
    let mut paths = Vec::new();

    for line_result in BufReader::new(file).lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(error) => {
                warn!("Failed reading line in {:?}: {}", log_path, error);
                continue;
            }
        };

        if !line.contains(steam_appid) {
            continue;
        }

        for path in extract_absolute_paths_from_line(&line) {
            if seen.insert(path.clone()) {
                paths.push(path);
            }
        }
    }

    paths
}

pub fn get_save_paths_from_steam_cloud(steam_appid: &str) -> Vec<SuggestedSavePath> {
    let mut seen = HashSet::new();
    let mut suggestions = Vec::new();

    for steam_root in get_steam_roots() {
        for path in parse_remotestorage_log(&steam_root, steam_appid) {
            if seen.insert(path.clone()) {
                let exists = path.exists();
                let is_empty = if exists { is_dir_empty(&path) } else { true };
                suggestions.push(SuggestedSavePath {
                    absolute_path: path,
                    ludusavi_placeholder: String::new(),
                    exists,
                    is_empty,
                });
            }
        }
    }

    suggestions
}

fn extract_absolute_paths_from_line(line: &str) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let bytes = line.as_bytes();
    let mut cursor = 0;

    while cursor < bytes.len() {
        if bytes[cursor] != b'/' {
            cursor += 1;
            continue;
        }

        let start = cursor;
        let mut end = cursor;

        while end < bytes.len() {
            let current = bytes[end];
            let is_terminator = current.is_ascii_whitespace()
                || matches!(
                    current,
                    b'"' | b'\'' | b',' | b';' | b')' | b'(' | b'[' | b']'
                );
            if is_terminator {
                break;
            }
            end += 1;
        }

        if let Some(segment) = line.get(start..end) {
            let trimmed = segment.trim_end_matches(['.', ':']);
            let candidate = PathBuf::from(trimmed);
            if candidate.is_absolute() {
                paths.push(candidate);
            }
        }

        cursor = end.saturating_add(1);
    }

    paths
}

fn is_dir_empty(path: &Path) -> bool {
    match std::fs::read_dir(path) {
        Ok(mut entries) => entries.next().is_none(),
        Err(_) => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_get_steam_roots_returns_existing_paths() {
        let roots = get_steam_roots();
        assert!(roots.iter().all(|path| path.is_absolute()));
        assert!(roots.iter().all(|path| path.exists()));
    }

    #[test]
    fn test_parse_remotestorage_log_extracts_paths_for_matching_appid() {
        let temp_dir = TempDir::new().expect("temp dir should be created");
        let logs_dir = temp_dir.path().join("logs");
        fs::create_dir_all(&logs_dir).expect("logs dir should be created");

        let save_dir_a = temp_dir.path().join("game_a");
        let save_dir_b = temp_dir.path().join("game_b");

        let log_content = format!(
            "[2026-02-26] appid=620 path=\"{}\" synced\n[2026-02-26] appid=777 path=\"{}\" ignored\n[2026-02-26] appid=620 file={} done\n[2026-02-26] appid=620 path=\"{}\" duplicate\n",
            save_dir_a.display(),
            temp_dir.path().join("other_app").display(),
            save_dir_b.display(),
            save_dir_a.display()
        );

        fs::write(logs_dir.join("remotestorage_log.txt"), log_content)
            .expect("log file should be written");

        let parsed = parse_remotestorage_log(temp_dir.path(), "620");

        assert_eq!(parsed.len(), 2);
        assert!(parsed.contains(&save_dir_a));
        assert!(parsed.contains(&save_dir_b));
    }
}
