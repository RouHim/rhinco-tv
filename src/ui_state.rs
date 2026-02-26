use std::collections::HashSet;
use uuid::Uuid;

use crate::auth_flow::AuthFlow;
use crate::model::Category;
use crate::system_info::GamingSystemInfo;
use crate::system_update_state::SystemUpdateState;
use crate::ui_app_picker::AppPickerState;
use crate::ui_save_path_modal::SuggestedSavePathDisplay;
use crate::updater::ReleaseInfo;
use crate::virtual_keyboard::VirtualKeyboard;

pub enum ModalState {
    None,
    ContextMenu {
        index: usize,
    },
    AppPicker(AppPickerState),
    SystemUpdate(SystemUpdateState),
    SystemUpdateAuth {
        update: SystemUpdateState,
        auth: AuthState,
    },
    AppUpdate(AppUpdateState),
    SystemInfo {
        info: Box<Option<GamingSystemInfo>>,
        selected_index: usize,
    },
    Auth(AuthState),
    AppNotFound {
        item_id: Uuid,
        item_name: String,
        category: Category,
        selected_index: usize,
    },
    Help,
    LudusaviSettings {
        selected_index: usize,
    },
    RestoreConfirm {
        game_name: String,
        selected_index: usize,
    },
    LudusaviProgress {
        operation_name: String,
        game_name: String,
        spinner_tick: usize,
    },
    SavePathScanning {
        game_name: String,
        spinner_tick: usize,
    },
    Settings {
        selected_index: usize,
        editing_api_key: bool,
        keyboard: Option<VirtualKeyboard>,
        api_key_buffer: String,
    },
    SavePathConfig {
        game_name: String,
        suggested_paths: Vec<SuggestedSavePathDisplay>,
        selected_indices: HashSet<usize>,
        selected_button: usize,
        manual_path: String,
        editing_manual: bool,
    },
}

pub struct AppUpdateState {
    pub release: ReleaseInfo,
    pub phase: AppUpdatePhase,
    pub status_message: Option<String>,
    pub spinner_tick: usize,
}

pub struct AuthState {
    pub flow: AuthFlow,
    pub keyboard: VirtualKeyboard,
}

impl AppUpdateState {
    pub fn new(release: ReleaseInfo) -> Self {
        Self {
            release,
            phase: AppUpdatePhase::Prompt,
            status_message: None,
            spinner_tick: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppUpdatePhase {
    Prompt,
    Updating,
    Completed,
    Failed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_state_initialization() {
        let settings = ModalState::Settings {
            selected_index: 0,
            editing_api_key: false,
            keyboard: None,
            api_key_buffer: String::new(),
        };

        match settings {
            ModalState::Settings {
                selected_index,
                editing_api_key,
                keyboard,
                api_key_buffer,
            } => {
                assert_eq!(selected_index, 0);
                assert!(!editing_api_key);
                assert!(keyboard.is_none());
                assert_eq!(api_key_buffer, "");
            }
            _ => panic!("Expected ModalState::Settings variant"),
        }
    }

    #[test]
    fn test_restore_confirm_initialization() {
        let modal = ModalState::RestoreConfirm {
            game_name: "Test Game".to_string(),
            selected_index: 1,
        };

        match modal {
            ModalState::RestoreConfirm {
                game_name,
                selected_index,
            } => {
                assert_eq!(game_name, "Test Game");
                assert_eq!(selected_index, 1); // Cancel = safe default
            }
            _ => panic!("Expected ModalState::RestoreConfirm variant"),
        }
    }

    #[test]
    fn test_ludusavi_progress_initialization() {
        let modal = ModalState::LudusaviProgress {
            operation_name: "backup".to_string(),
            game_name: "Test Game".to_string(),
            spinner_tick: 0,
        };

        match modal {
            ModalState::LudusaviProgress {
                operation_name,
                game_name,
                spinner_tick,
            } => {
                assert_eq!(operation_name, "backup");
                assert_eq!(game_name, "Test Game");
                assert_eq!(spinner_tick, 0);
            }
            _ => panic!("Expected ModalState::LudusaviProgress variant"),
        }
    }

    #[test]
    fn test_save_path_scanning_initialization() {
        let modal = ModalState::SavePathScanning {
            game_name: "Test Game".to_string(),
            spinner_tick: 0,
        };

        match modal {
            ModalState::SavePathScanning {
                game_name,
                spinner_tick,
            } => {
                assert_eq!(game_name, "Test Game");
                assert_eq!(spinner_tick, 0);
            }
            _ => panic!("Expected ModalState::SavePathScanning variant"),
        }
    }
}
