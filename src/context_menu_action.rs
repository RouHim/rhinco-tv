use crate::model::Category;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextMenuAction {
    Launch,
    BackupSaves,
    RestoreSaves,
    ConfigureSavePaths,
    OpenSaveSettings,
    RemoveApp,
    QuitLauncher,
    CloseMenu,
}

pub fn context_menu_items(
    category: Category,
    ludusavi_available: bool,
) -> Vec<(String, ContextMenuAction)> {
    match (category, ludusavi_available) {
        (Category::Apps, _) => vec![
            ("Launch".to_string(), ContextMenuAction::Launch),
            ("Remove Entry".to_string(), ContextMenuAction::RemoveApp),
            ("Quit Launcher".to_string(), ContextMenuAction::QuitLauncher),
            ("Close".to_string(), ContextMenuAction::CloseMenu),
        ],
        (Category::Games, true) => vec![
            ("Launch".to_string(), ContextMenuAction::Launch),
            ("Backup Saves".to_string(), ContextMenuAction::BackupSaves),
            ("Restore Saves".to_string(), ContextMenuAction::RestoreSaves),
            (
                "Configure Save Paths".to_string(),
                ContextMenuAction::ConfigureSavePaths,
            ),
            (
                "Save Sync Settings".to_string(),
                ContextMenuAction::OpenSaveSettings,
            ),
            ("Quit Launcher".to_string(), ContextMenuAction::QuitLauncher),
            ("Close".to_string(), ContextMenuAction::CloseMenu),
        ],
        (Category::Games, false) => vec![
            ("Launch".to_string(), ContextMenuAction::Launch),
            ("Quit Launcher".to_string(), ContextMenuAction::QuitLauncher),
            ("Close".to_string(), ContextMenuAction::CloseMenu),
        ],
        (Category::System, true) => vec![
            ("Launch".to_string(), ContextMenuAction::Launch),
            (
                "Save Sync Settings".to_string(),
                ContextMenuAction::OpenSaveSettings,
            ),
            ("Quit Launcher".to_string(), ContextMenuAction::QuitLauncher),
            ("Close".to_string(), ContextMenuAction::CloseMenu),
        ],
        (Category::System, false) => vec![
            ("Launch".to_string(), ContextMenuAction::Launch),
            ("Quit Launcher".to_string(), ContextMenuAction::QuitLauncher),
            ("Close".to_string(), ContextMenuAction::CloseMenu),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::{context_menu_items, ContextMenuAction};
    use crate::model::Category;

    #[test]
    fn test_context_menu_items_for_apps() {
        let items = context_menu_items(Category::Apps, true);

        assert_eq!(
            items,
            vec![
                ("Launch".to_string(), ContextMenuAction::Launch),
                ("Remove Entry".to_string(), ContextMenuAction::RemoveApp),
                ("Quit Launcher".to_string(), ContextMenuAction::QuitLauncher),
                ("Close".to_string(), ContextMenuAction::CloseMenu),
            ]
        );
    }

    #[test]
    fn test_context_menu_items_for_games_with_ludusavi() {
        let items = context_menu_items(Category::Games, true);

        assert_eq!(
            items,
            vec![
                ("Launch".to_string(), ContextMenuAction::Launch),
                ("Backup Saves".to_string(), ContextMenuAction::BackupSaves),
                ("Restore Saves".to_string(), ContextMenuAction::RestoreSaves),
                (
                    "Configure Save Paths".to_string(),
                    ContextMenuAction::ConfigureSavePaths,
                ),
                (
                    "Save Sync Settings".to_string(),
                    ContextMenuAction::OpenSaveSettings,
                ),
                ("Quit Launcher".to_string(), ContextMenuAction::QuitLauncher),
                ("Close".to_string(), ContextMenuAction::CloseMenu),
            ]
        );
    }

    #[test]
    fn test_configure_save_paths_always_present_for_games_with_ludusavi() {
        let items = context_menu_items(Category::Games, true);
        assert!(
            items
                .iter()
                .any(|(_, action)| *action == ContextMenuAction::ConfigureSavePaths),
            "ConfigureSavePaths must always be present in Games menu when ludusavi is available"
        );
    }

    #[test]
    fn test_context_menu_items_for_games_without_ludusavi() {
        let items = context_menu_items(Category::Games, false);

        assert_eq!(
            items,
            vec![
                ("Launch".to_string(), ContextMenuAction::Launch),
                ("Quit Launcher".to_string(), ContextMenuAction::QuitLauncher),
                ("Close".to_string(), ContextMenuAction::CloseMenu),
            ]
        );
    }

    #[test]
    fn test_context_menu_items_for_system_with_ludusavi() {
        let items = context_menu_items(Category::System, true);

        assert_eq!(
            items,
            vec![
                ("Launch".to_string(), ContextMenuAction::Launch),
                (
                    "Save Sync Settings".to_string(),
                    ContextMenuAction::OpenSaveSettings,
                ),
                ("Quit Launcher".to_string(), ContextMenuAction::QuitLauncher),
                ("Close".to_string(), ContextMenuAction::CloseMenu),
            ]
        );
    }

    #[test]
    fn test_context_menu_items_for_system_without_ludusavi() {
        let items = context_menu_items(Category::System, false);

        assert_eq!(
            items,
            vec![
                ("Launch".to_string(), ContextMenuAction::Launch),
                ("Quit Launcher".to_string(), ContextMenuAction::QuitLauncher),
                ("Close".to_string(), ContextMenuAction::CloseMenu),
            ]
        );
    }
}
