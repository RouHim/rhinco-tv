use chrono::{DateTime, Local};
use iced::window;
use std::path::PathBuf;
use uuid::Uuid;

use crate::desktop_apps::DesktopApp;
use crate::gamepad::GamepadInfo;
use crate::input::Action;
use crate::ludusavi::{LudusaviError, LudusaviResult};
use crate::model::AppEntry;
use crate::storage::AppConfig;
use crate::sudo_askpass::AskpassEvent;
use crate::system_info::GamingSystemInfo;
use crate::system_update_state::SystemUpdateProgress;
use crate::toast::ToastSeverity;
use crate::updater::ReleaseInfo;
use crate::virtual_keyboard::KeyboardMessage;
use crate::wine_prefix_scanner::SuggestedSavePath;

#[derive(Debug, Clone)]
pub enum Message {
    AppsLoaded(Result<AppConfig, String>),
    GamesLoaded(Vec<AppEntry>),
    ImageFetched(Uuid, PathBuf),
    Input(Action),
    ScaleFactorChanged(f64),
    WindowResized(f32, f32),
    // App picker messages
    OpenAppPicker,
    AvailableAppsLoaded(Vec<DesktopApp>),
    AddSelectedApp,
    CloseAppPicker,
    AppPickerScrolled(iced::widget::scrollable::Viewport),
    // System Update messages
    StartSystemUpdate,
    SystemUpdateProgress(SystemUpdateProgress),
    CloseSystemUpdateModal,
    CancelSystemUpdate,
    RequestReboot,
    // App Update messages
    AppUpdateCheckCompleted(Result<Option<ReleaseInfo>, String>),
    StartAppUpdate,
    AppUpdateApplied(Result<(), String>),
    CloseAppUpdateModal,
    // System Info messages
    OpenSystemInfo,
    SystemInfoLoaded(Box<GamingSystemInfo>),
    CloseSystemInfoModal,
    // Game/App lifecycle
    GameExited,
    WindowOpened(window::Id),
    WindowFocused(window::Id),
    RestartApp,
    GamepadBatteryUpdate(Vec<GamepadInfo>),
    SystemBatteryUpdated(Option<gilrs::PowerInfo>),
    Tick(DateTime<Local>),
    AppUpdateSpinnerTick,
    SpinnerTick,
    AskpassEvent(AskpassEvent),
    AuthKeyboard(KeyboardMessage),
    AuthSubmit,
    AuthCancel,
    OverlayAlphaUpdate(iced_anim::Event<f32>),
    // Toast messages
    #[allow(dead_code)]
    ShowToast {
        message: String,
        severity: ToastSeverity,
    },
    #[allow(dead_code)]
    DismissToast,
    ToastTick,
    // Ludusavi settings messages
    #[allow(dead_code)]
    OpenLudusaviSettings,
    CloseLudusaviSettings,
    ToggleAutoBackup,
    ToggleAutoCloudSync,
    ToggleAutostart,
    LudusaviOperationCompleted {
        operation: String,
        game_name: Option<String>,
        result: Result<LudusaviResult, LudusaviError>,
    },
    BackupStatusReceived {
        game_name: String,
        status: Option<bool>,
    },
    // Settings modal messages
    OpenSettings,
    CloseSettings,
    UpdateSteamGridDbApiKey(String),
    SettingsKeyboard(KeyboardMessage),
    // Save path config messages
    #[allow(dead_code)]
    OpenSavePathConfig {
        game_name: String,
        unknown_games: Vec<String>,
    },
    SavePathsDiscovered {
        game_name: String,
        paths: Vec<SuggestedSavePath>,
    },
    ConfirmSavePaths {
        game_name: String,
        selected_paths: Vec<String>,
    },
    SavePathConfigWritten {
        game_name: String,
        result: Result<(), String>,
    },
    CloseSavePathConfig,
    None,
}
