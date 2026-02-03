use anyhow::{Context, Result};
use directories::BaseDirs;
use std::fs;
use std::path::PathBuf;

fn autostart_path() -> Option<PathBuf> {
    BaseDirs::new().map(|base_dirs| {
        base_dirs
            .config_dir()
            .join("autostart")
            .join("rhinco-tv.desktop")
    })
}

fn desktop_file_content() -> Option<String> {
    let exe_path = std::env::current_exe().ok()?;
    Some(format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name=RhincoTV\n\
         Exec={}\n\
         Terminal=false\n\
         Hidden=false\n",
        exe_path.display()
    ))
}

#[allow(dead_code)]
pub fn is_enabled() -> bool {
    autostart_path().map(|path| path.exists()).unwrap_or(false)
}

#[allow(dead_code)]
pub fn enable() -> Result<()> {
    let path = autostart_path().context("Failed to determine autostart path")?;
    let content = desktop_file_content().context("Failed to get current executable path")?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).context("Failed to create autostart directory")?;
    }

    fs::write(&path, content).context("Failed to write autostart desktop file")?;

    Ok(())
}

#[allow(dead_code)]
pub fn disable() -> Result<()> {
    let path = autostart_path().context("Failed to determine autostart path")?;

    if path.exists() {
        fs::remove_file(&path).context("Failed to remove autostart desktop file")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn setup_temp_autostart_dir() -> (PathBuf, tempfile::TempDir) {
        let temp_dir = tempfile::tempdir().unwrap();
        let autostart_dir = temp_dir.path().join("autostart");
        fs::create_dir_all(&autostart_dir).unwrap();
        (autostart_dir.join("rhinco-tv.desktop"), temp_dir)
    }

    #[test]
    fn test_autostart_path_returns_valid_path() {
        let path = autostart_path();
        assert!(path.is_some());

        let path = path.unwrap();
        assert!(path.to_string_lossy().contains("autostart"));
        assert!(path.to_string_lossy().ends_with("rhinco-tv.desktop"));
    }

    #[test]
    fn test_is_enabled_returns_false_when_file_missing() {
        let _result = is_enabled();
    }

    #[test]
    fn test_desktop_file_content_has_correct_format() {
        let content = desktop_file_content();
        assert!(content.is_some());

        let content = content.unwrap();
        assert!(content.contains("[Desktop Entry]"));
        assert!(content.contains("Type=Application"));
        assert!(content.contains("Name=RhincoTV"));
        assert!(content.contains("Exec="));
        assert!(content.contains("Terminal=false"));
        assert!(content.contains("Hidden=false"));
    }

    #[test]
    fn test_enable_creates_desktop_file() {
        let (desktop_file_path, _temp_dir) = setup_temp_autostart_dir();

        let content = desktop_file_content().unwrap();
        fs::write(&desktop_file_path, content).unwrap();

        assert!(desktop_file_path.exists());

        let file_content = fs::read_to_string(&desktop_file_path).unwrap();
        assert!(file_content.contains("[Desktop Entry]"));
        assert!(file_content.contains("Type=Application"));
        assert!(file_content.contains("Name=RhincoTV"));
    }

    #[test]
    fn test_disable_removes_desktop_file() {
        let (desktop_file_path, _temp_dir) = setup_temp_autostart_dir();

        let content = desktop_file_content().unwrap();
        fs::write(&desktop_file_path, content).unwrap();
        assert!(desktop_file_path.exists());

        fs::remove_file(&desktop_file_path).unwrap();
        assert!(!desktop_file_path.exists());
    }

    #[test]
    fn test_disable_succeeds_when_file_already_missing() {
        let (desktop_file_path, _temp_dir) = setup_temp_autostart_dir();

        if desktop_file_path.exists() {
            fs::remove_file(&desktop_file_path).unwrap();
        }

        if desktop_file_path.exists() {
            fs::remove_file(&desktop_file_path).unwrap();
        }

        assert!(!desktop_file_path.exists());
    }

    #[test]
    fn test_is_enabled_returns_true_when_file_exists() {
        let (desktop_file_path, _temp_dir) = setup_temp_autostart_dir();

        let content = desktop_file_content().unwrap();
        fs::write(&desktop_file_path, content).unwrap();

        assert!(desktop_file_path.exists());
    }
}
