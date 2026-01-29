use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

const LUDUSAVI_TIMEOUT: Duration = Duration::from_secs(60);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LudusaviOperation {
    Backup,
    BackupWithCloudSync,
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
}

#[derive(Debug, Clone)]
pub struct LudusaviResult {
    pub success: bool,
    pub games_processed: usize,
    pub has_backups: bool,
    pub error_message: Option<String>,
}

#[derive(Debug)]
pub enum LudusaviError {
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

    let output = match timeout(LUDUSAVI_TIMEOUT, cmd.output()).await {
        Ok(Ok(output)) => output,
        Ok(Err(e)) => {
            return Err(LudusaviError::CommandFailed(e.to_string()));
        }
        Err(_) => {
            return Err(LudusaviError::Timeout);
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(LudusaviError::CommandFailed(stderr.to_string()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_json_response(&stdout, operation)
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
        });
    }

    let v: serde_json::Value =
        serde_json::from_str(json).map_err(|e| LudusaviError::ParseError(e.to_string()))?;

    let some_games_failed = v
        .get("errors")
        .and_then(|e| e.get("someGamesFailed"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let unknown_games = v
        .get("errors")
        .and_then(|e| e.get("unknownGames"))
        .and_then(|v| v.as_array())
        .map(|a| !a.is_empty())
        .unwrap_or(false);

    let games_processed = v
        .get("overall")
        .and_then(|o| o.get("processedGames"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;

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
        success: !some_games_failed && !unknown_games,
        games_processed,
        has_backups,
        error_message: if unknown_games {
            Some("Game not found in Ludusavi database".into())
        } else {
            None
        },
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
        assert!(result
            .error_message
            .as_ref()
            .unwrap()
            .contains("not found"));
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
    }

    #[test]
    fn test_parse_backups_query_no_game_entry() {
        let json = r#"{
            "games": {}
        }"#;

        let result = parse_json_response(json, LudusaviOperation::QueryBackups).unwrap();
        assert!(result.success);
        assert!(!result.has_backups);
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
    }

    #[test]
    fn test_parse_whitespace_response() {
        let result = parse_json_response("   \n\t  ", LudusaviOperation::Backup).unwrap();
        assert!(!result.success);
        assert!(result.error_message.is_some());
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

        // Backup operation should NOT set has_backups (it's irrelevant)
        let result = parse_json_response(json, LudusaviOperation::Backup).unwrap();
        assert!(!result.has_backups);
    }
}
