use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;
use tracing::warn;

const LUDUSAVI_TIMEOUT: Duration = Duration::from_secs(60);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LudusaviOperation {
    Backup,
    BackupWithCloudSync,
    #[allow(dead_code)]
    Restore,
    QueryBackups,
}

impl LudusaviOperation {
    fn command_args(&self) -> Vec<&'static str> {
        match self {
            LudusaviOperation::Backup => vec!["backup", "--api", "--force"],
            LudusaviOperation::BackupWithCloudSync => {
                vec!["backup", "--api", "--force", "--cloud-sync"]
            }
            LudusaviOperation::Restore => vec!["restore", "--api", "--force"],
            LudusaviOperation::QueryBackups => vec!["backups", "--api"],
        }
    }

    pub fn operation_name(&self) -> &'static str {
        match self {
            LudusaviOperation::Backup => "backup",
            LudusaviOperation::BackupWithCloudSync => "backup-with-cloud-sync",
            LudusaviOperation::Restore => "restore",
            LudusaviOperation::QueryBackups => "query-backups",
        }
    }
}

#[derive(Debug, Clone)]
pub struct LudusaviResult {
    #[allow(dead_code)]
    pub success: bool,
    #[allow(dead_code)]
    pub games_processed: usize,
    pub has_backups: bool,
    #[allow(dead_code)]
    pub error_message: Option<String>,
    #[allow(dead_code)]
    pub unknown_games: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum LudusaviError {
    #[allow(dead_code)]
    NotInstalled,
    Timeout,
    CommandFailed(String),
    ParseError(String),
}

impl std::fmt::Display for LudusaviError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LudusaviError::NotInstalled => write!(f, "Ludusavi is not installed"),
            LudusaviError::Timeout => write!(f, "Ludusavi operation timed out"),
            LudusaviError::CommandFailed(msg) => write!(f, "Ludusavi command failed: {}", msg),
            LudusaviError::ParseError(msg) => write!(f, "Failed to parse Ludusavi output: {}", msg),
        }
    }
}

impl std::error::Error for LudusaviError {}

/// Check if ludusavi is installed (blocking, use at startup)
pub fn ludusavi_available() -> bool {
    std::process::Command::new("ludusavi")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Execute ludusavi operation asynchronously
pub async fn execute_operation(
    game_name: &str,
    operation: LudusaviOperation,
) -> Result<LudusaviResult, LudusaviError> {
    let args = operation.command_args();

    let mut cmd = Command::new("ludusavi");
    cmd.args(&args);
    cmd.arg(game_name);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.stdin(Stdio::null());

    let output = timeout(LUDUSAVI_TIMEOUT, cmd.output())
        .await
        .map_err(|_| LudusaviError::Timeout)?
        .map_err(|e| LudusaviError::CommandFailed(e.to_string()))?;

    process_command_output(
        output.status.success(),
        &output.stdout,
        &output.stderr,
        operation,
    )
}

fn process_command_output(
    exit_success: bool,
    stdout: &[u8],
    stderr: &[u8],
    operation: LudusaviOperation,
) -> Result<LudusaviResult, LudusaviError> {
    let stdout_str = String::from_utf8_lossy(stdout);
    let stderr_str = String::from_utf8_lossy(stderr);

    // Log stderr when present on non-zero exit
    if !exit_success && !stderr_str.trim().is_empty() {
        warn!(
            "Ludusavi exited with non-zero status. stderr: {}",
            stderr_str.trim()
        );
    }

    // If stdout is empty and exit was non-zero, it's a genuine failure
    if stdout_str.trim().is_empty() && !exit_success {
        return Err(LudusaviError::CommandFailed(stderr_str.to_string()));
    }

    // Try parsing stdout — works for both success and "known failure" cases (unknown games)
    match parse_json_response(&stdout_str, operation) {
        Ok(result) => Ok(result),
        Err(e) => {
            if !exit_success {
                // Stdout wasn't parseable AND exit was non-zero → genuine command failure
                // Use stderr (human-readable) rather than parse error
                Err(LudusaviError::CommandFailed(stderr_str.to_string()))
            } else {
                // Zero exit but bad JSON → propagate parse error
                Err(e)
            }
        }
    }
}

fn parse_json_response(
    json: &str,
    operation: LudusaviOperation,
) -> Result<LudusaviResult, LudusaviError> {
    if json.trim().is_empty() {
        return Ok(LudusaviResult {
            success: false,
            games_processed: 0,
            has_backups: false,
            error_message: Some("No response from Ludusavi".into()),
            unknown_games: vec![],
        });
    }

    let v: serde_json::Value =
        serde_json::from_str(json).map_err(|e| LudusaviError::ParseError(e.to_string()))?;

    fn get_bool_at(v: &serde_json::Value, path: &[&str]) -> bool {
        path.iter()
            .try_fold(v, |acc, &key| acc.get(key))
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    fn get_u64_at(v: &serde_json::Value, path: &[&str]) -> u64 {
        path.iter()
            .try_fold(v, |acc, &key| acc.get(key))
            .and_then(|v| v.as_u64())
            .unwrap_or(0)
    }

    let some_games_failed = get_bool_at(&v, &["errors", "someGamesFailed"]);

    let unknown_games_vec: Vec<String> = v
        .get("errors")
        .and_then(|e| e.get("unknownGames"))
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|item| item.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    let unknown_games_flag = !unknown_games_vec.is_empty();

    let games_processed = get_u64_at(&v, &["overall", "processedGames"]) as usize;

    let has_backups = if operation == LudusaviOperation::QueryBackups {
        v.get("games")
            .and_then(|g| g.as_object())
            .map(|games| {
                games.values().any(|game| {
                    game.get("backups")
                        .and_then(|b| b.as_array())
                        .map(|arr| !arr.is_empty())
                        .unwrap_or(false)
                })
            })
            .unwrap_or(false)
    } else {
        false
    };

    Ok(LudusaviResult {
        success: !some_games_failed && !unknown_games_flag,
        games_processed,
        has_backups,
        error_message: if unknown_games_flag {
            Some("Game not found in Ludusavi database".into())
        } else {
            None
        },
        unknown_games: unknown_games_vec,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ludusavi_operation_backup_args() {
        let args = LudusaviOperation::Backup.command_args();
        assert_eq!(args, vec!["backup", "--api", "--force"]);
    }

    #[test]
    fn test_ludusavi_operation_backup_with_cloud_sync_args() {
        let args = LudusaviOperation::BackupWithCloudSync.command_args();
        assert_eq!(args, vec!["backup", "--api", "--force", "--cloud-sync"]);
    }

    #[test]
    fn test_ludusavi_operation_restore_args() {
        let args = LudusaviOperation::Restore.command_args();
        assert_eq!(args, vec!["restore", "--api", "--force"]);
    }

    #[test]
    fn test_ludusavi_operation_query_backups_args() {
        let args = LudusaviOperation::QueryBackups.command_args();
        assert_eq!(args, vec!["backups", "--api"]);
    }

    #[test]
    fn test_parse_successful_backup_response() {
        let json = r#"{
            "errors": { "someGamesFailed": false },
            "overall": { "totalGames": 1, "totalBytes": 1024, "processedGames": 1, "processedBytes": 1024 },
            "games": {
                "Test Game": {
                    "decision": "Processed",
                    "files": { "/path/to/save": { "bytes": 1024 } }
                }
            }
        }"#;

        let result = parse_json_response(json, LudusaviOperation::Backup).unwrap();
        assert!(result.success);
        assert_eq!(result.games_processed, 1);
        assert!(result.error_message.is_none());
        assert!(result.unknown_games.is_empty());
    }

    #[test]
    fn test_parse_game_not_found_response() {
        let json = r#"{
            "errors": { "unknownGames": ["Unknown Game"] },
            "overall": { "totalGames": 0, "processedGames": 0 },
            "games": {}
        }"#;

        let result = parse_json_response(json, LudusaviOperation::Backup).unwrap();
        assert!(!result.success);
        assert_eq!(result.games_processed, 0);
        assert!(result.error_message.is_some());
        assert!(result.error_message.as_ref().unwrap().contains("not found"));
        assert!(result.unknown_games.contains(&"Unknown Game".to_string()));
    }

    #[test]
    fn test_parse_backups_query_with_backups() {
        let json = r#"{
            "games": {
                "Test Game": {
                    "backups": [
                        { "name": "backup-2024-01-15T10-30-00", "when": "2024-01-15T10:30:00Z" }
                    ]
                }
            }
        }"#;

        let result = parse_json_response(json, LudusaviOperation::QueryBackups).unwrap();
        assert!(result.success);
        assert!(result.has_backups);
        assert!(result.unknown_games.is_empty());
    }

    #[test]
    fn test_parse_backups_query_without_backups() {
        let json = r#"{
            "games": {
                "Test Game": {
                    "backups": []
                }
            }
        }"#;

        let result = parse_json_response(json, LudusaviOperation::QueryBackups).unwrap();
        assert!(result.success);
        assert!(!result.has_backups);
        assert!(result.unknown_games.is_empty());
    }

    #[test]
    fn test_parse_backups_query_no_game_entry() {
        let json = r#"{
            "games": {}
        }"#;

        let result = parse_json_response(json, LudusaviOperation::QueryBackups).unwrap();
        assert!(result.success);
        assert!(!result.has_backups);
        assert!(result.unknown_games.is_empty());
    }

    #[test]
    fn test_parse_empty_response() {
        let result = parse_json_response("", LudusaviOperation::Backup).unwrap();
        assert!(!result.success);
        assert!(result.error_message.is_some());
        assert!(result
            .error_message
            .as_ref()
            .unwrap()
            .contains("No response"));
        assert!(result.unknown_games.is_empty());
    }

    #[test]
    fn test_parse_whitespace_response() {
        let result = parse_json_response("   \n\t  ", LudusaviOperation::Backup).unwrap();
        assert!(!result.success);
        assert!(result.error_message.is_some());
        assert!(result.unknown_games.is_empty());
    }

    #[test]
    fn test_parse_invalid_json() {
        let result = parse_json_response("not valid json", LudusaviOperation::Backup);
        assert!(matches!(result, Err(LudusaviError::ParseError(_))));
    }

    #[test]
    fn test_parse_some_games_failed() {
        let json = r#"{
            "errors": { "someGamesFailed": true },
            "overall": { "processedGames": 0 },
            "games": {}
        }"#;

        let result = parse_json_response(json, LudusaviOperation::Backup).unwrap();
        assert!(!result.success);
        assert!(result.unknown_games.is_empty());
    }

    #[test]
    fn test_unknown_games_extracted_from_array() {
        let json = r#"{
            "errors": { "unknownGames": ["Game One", "Game Two", "Game Three"] },
            "overall": { "processedGames": 0 },
            "games": {}
        }"#;

        let result = parse_json_response(json, LudusaviOperation::Backup).unwrap();
        assert!(!result.success);
        assert_eq!(result.unknown_games.len(), 3);
        assert!(result.unknown_games.contains(&"Game One".to_string()));
        assert!(result.unknown_games.contains(&"Game Two".to_string()));
        assert!(result.unknown_games.contains(&"Game Three".to_string()));
        assert!(result.error_message.is_some());
        assert!(result.error_message.as_ref().unwrap().contains("not found"));
    }

    #[test]
    fn test_ludusavi_error_display() {
        assert_eq!(
            format!("{}", LudusaviError::NotInstalled),
            "Ludusavi is not installed"
        );
        assert_eq!(
            format!("{}", LudusaviError::Timeout),
            "Ludusavi operation timed out"
        );
        assert!(format!("{}", LudusaviError::CommandFailed("test".into())).contains("test"));
        assert!(format!("{}", LudusaviError::ParseError("parse".into())).contains("parse"));
    }

    #[test]
    fn test_has_backups_only_for_query_operation() {
        let json = r#"{
            "games": {
                "Test Game": {
                    "backups": [{ "name": "backup-1" }]
                }
            }
        }"#;

        // QueryBackups should check has_backups
        let result = parse_json_response(json, LudusaviOperation::QueryBackups).unwrap();
        assert!(result.has_backups);
        assert!(result.unknown_games.is_empty());

        // Backup operation should NOT set has_backups (it's irrelevant)
        let result = parse_json_response(json, LudusaviOperation::Backup).unwrap();
        assert!(!result.has_backups);
        assert!(result.unknown_games.is_empty());
    }

    #[test]
    fn test_process_output_nonzero_exit_with_valid_unknown_games_json() {
        let json = r#"{"errors":{"unknownGames":["Need For Speed Underground 2"]},"overall":{"processedGames":0},"games":{}}"#;
        let stderr = "No info for these games:\n  - Need For Speed Underground 2";

        let result = process_command_output(
            false,
            json.as_bytes(),
            stderr.as_bytes(),
            LudusaviOperation::Backup,
        )
        .unwrap();

        assert!(!result.success);
        assert_eq!(result.unknown_games.len(), 1);
        assert!(result
            .unknown_games
            .contains(&"Need For Speed Underground 2".to_string()));
    }

    #[test]
    fn test_process_output_nonzero_exit_with_empty_stdout() {
        let result = process_command_output(
            false,
            b"",
            b"Some error occurred",
            LudusaviOperation::Backup,
        );

        assert!(matches!(result, Err(LudusaviError::CommandFailed(_))));
        if let Err(LudusaviError::CommandFailed(msg)) = result {
            assert!(msg.contains("Some error occurred"));
        }
    }

    #[test]
    fn test_process_output_nonzero_exit_with_garbage_stdout() {
        let result = process_command_output(
            false,
            b"not valid json at all",
            b"Process crashed",
            LudusaviOperation::Backup,
        );

        // Should return CommandFailed (stderr), NOT ParseError
        assert!(matches!(result, Err(LudusaviError::CommandFailed(_))));
        if let Err(LudusaviError::CommandFailed(msg)) = result {
            assert!(msg.contains("Process crashed"));
        }
    }

    #[test]
    fn test_process_output_nonzero_exit_with_some_games_failed() {
        let json =
            r#"{"errors":{"someGamesFailed":true},"overall":{"processedGames":0},"games":{}}"#;

        let result = process_command_output(
            false,
            json.as_bytes(),
            b"Some games failed",
            LudusaviOperation::Backup,
        )
        .unwrap();

        assert!(!result.success);
        assert!(result.unknown_games.is_empty());
    }

    #[test]
    fn test_process_output_zero_exit_still_works() {
        let json = r#"{"errors":{"someGamesFailed":false},"overall":{"processedGames":1},"games":{"Test":{}}}"#;

        let result =
            process_command_output(true, json.as_bytes(), b"", LudusaviOperation::Backup).unwrap();

        assert!(result.success);
    }
}
