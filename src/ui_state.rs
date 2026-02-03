use uuid::Uuid;

use crate::auth_flow::AuthFlow;
use crate::model::Category;
use crate::system_info::GamingSystemInfo;
use crate::system_update_state::SystemUpdateState;
use crate::ui_app_picker::AppPickerState;
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
    Settings {
        selected_index: usize,
        editing_api_key: bool,
        keyboard: Option<VirtualKeyboard>,
        api_key_buffer: String,
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
}
