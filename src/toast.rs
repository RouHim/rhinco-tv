use chrono::{DateTime, Local};
use iced::widget::{container, text, Container};
use iced::{Color, Element, Length};

use crate::messages::Message;
use crate::ui_theme::{
    scaled, BASE_FONT_MEDIUM, BASE_PADDING_MEDIUM, COLOR_BACKGROUND, COLOR_ERROR, COLOR_SUCCESS,
    COLOR_TEXT_BRIGHT,
};

const TOAST_DISPLAY_DURATION_SECS: i64 = 4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToastSeverity {
    Success,
    Error,
    #[allow(dead_code)]
    Info,
}

impl ToastSeverity {
    pub fn color(&self) -> Color {
        match self {
            ToastSeverity::Success => COLOR_SUCCESS,
            ToastSeverity::Error => COLOR_ERROR,
            ToastSeverity::Info => crate::ui_theme::COLOR_ACCENT,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ToastState {
    Hidden,
    Showing {
        message: String,
        severity: ToastSeverity,
        started_at: DateTime<Local>,
    },
}

#[derive(Debug, Clone)]
pub struct Toast {
    state: ToastState,
}

impl Toast {
    pub fn new() -> Self {
        Self {
            state: ToastState::Hidden,
        }
    }

    pub fn show(&mut self, message: &str, severity: ToastSeverity) {
        self.state = ToastState::Showing {
            message: message.to_string(),
            severity,
            started_at: Local::now(),
        };
    }

    pub fn should_dismiss(&self) -> bool {
        match &self.state {
            ToastState::Hidden => false,
            ToastState::Showing { started_at, .. } => {
                let elapsed = Local::now().signed_duration_since(*started_at);
                elapsed.num_seconds() >= TOAST_DISPLAY_DURATION_SECS
            }
        }
    }

    pub fn dismiss(&mut self) {
        self.state = ToastState::Hidden;
    }

    pub fn is_showing(&self) -> bool {
        matches!(self.state, ToastState::Showing { .. })
    }

    pub fn view(&self, scale: f32) -> Element<'_, Message> {
        match &self.state {
            ToastState::Hidden => iced::widget::Space::new()
                .width(Length::Shrink)
                .height(Length::Shrink)
                .into(),
            ToastState::Showing {
                message, severity, ..
            } => {
                let bg_color = severity.color();

                Container::new(
                    text(message)
                        .size(scaled(BASE_FONT_MEDIUM, scale))
                        .color(COLOR_TEXT_BRIGHT),
                )
                .padding(scaled(BASE_PADDING_MEDIUM, scale))
                .style(move |_theme| container::Style {
                    background: Some(bg_color.into()),
                    text_color: Some(COLOR_TEXT_BRIGHT),
                    border: iced::Border {
                        color: COLOR_BACKGROUND,
                        width: scaled(2.0, scale),
                        radius: scaled(8.0, scale).into(),
                    },
                    ..Default::default()
                })
                .width(Length::Shrink)
                .height(Length::Shrink)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
            }
        }
    }
}

impl Default for Toast {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_hidden_toast() {
        let toast = Toast::new();
        assert!(!toast.is_showing());
    }

    #[test]
    fn test_default_creates_hidden_toast() {
        let toast = Toast::default();
        assert!(!toast.is_showing());
    }

    #[test]
    fn test_show_transitions_to_showing_state() {
        let mut toast = Toast::new();
        toast.show("Test message", ToastSeverity::Info);
        assert!(toast.is_showing());
    }

    #[test]
    fn test_show_captures_message_and_severity() {
        let mut toast = Toast::new();
        toast.show("Error occurred", ToastSeverity::Error);

        if let ToastState::Showing {
            message, severity, ..
        } = &toast.state
        {
            assert_eq!(message, "Error occurred");
            assert_eq!(*severity, ToastSeverity::Error);
        } else {
            panic!("Toast should be in Showing state");
        }
    }

    #[test]
    fn test_dismiss_transitions_to_hidden_state() {
        let mut toast = Toast::new();
        toast.show("Test", ToastSeverity::Success);
        assert!(toast.is_showing());

        toast.dismiss();
        assert!(!toast.is_showing());
    }

    #[test]
    fn test_should_dismiss_false_when_hidden() {
        let toast = Toast::new();
        assert!(!toast.should_dismiss());
    }

    #[test]
    fn test_should_dismiss_false_immediately_after_show() {
        let mut toast = Toast::new();
        toast.show("Test", ToastSeverity::Info);
        assert!(!toast.should_dismiss());
    }

    #[test]
    fn test_should_dismiss_true_after_duration() {
        let mut toast = Toast::new();
        toast.show("Test", ToastSeverity::Info);

        if let ToastState::Showing {
            message,
            severity,
            started_at: _,
        } = &toast.state
        {
            toast.state = ToastState::Showing {
                message: message.clone(),
                severity: *severity,
                started_at: Local::now() - chrono::Duration::seconds(5),
            };
        }

        assert!(toast.should_dismiss());
    }

    #[test]
    fn test_show_replaces_existing_toast() {
        let mut toast = Toast::new();
        toast.show("First message", ToastSeverity::Info);
        toast.show("Second message", ToastSeverity::Error);

        if let ToastState::Showing {
            message, severity, ..
        } = &toast.state
        {
            assert_eq!(message, "Second message");
            assert_eq!(*severity, ToastSeverity::Error);
        } else {
            panic!("Toast should be in Showing state");
        }
    }

    #[test]
    fn test_dismiss_on_hidden_is_safe() {
        let mut toast = Toast::new();
        toast.dismiss();
        assert!(!toast.is_showing());
    }

    #[test]
    fn test_multiple_dismiss_calls_are_safe() {
        let mut toast = Toast::new();
        toast.show("Test", ToastSeverity::Success);
        toast.dismiss();
        toast.dismiss();
        toast.dismiss();
        assert!(!toast.is_showing());
    }

    #[test]
    fn test_severity_colors() {
        assert_ne!(ToastSeverity::Success.color(), ToastSeverity::Error.color());
        assert_ne!(ToastSeverity::Success.color(), ToastSeverity::Info.color());
        assert_ne!(ToastSeverity::Error.color(), ToastSeverity::Info.color());
    }

    #[test]
    fn test_view_returns_element_when_hidden() {
        let toast = Toast::new();
        let _element = toast.view(1.0);
    }

    #[test]
    fn test_view_returns_element_when_showing() {
        let mut toast = Toast::new();
        toast.show("Test message", ToastSeverity::Success);
        let _element = toast.view(1.0);
    }

    #[test]
    fn test_view_respects_scale_factor() {
        let mut toast = Toast::new();
        toast.show("Test", ToastSeverity::Info);

        let _element1 = toast.view(0.5);
        let _element2 = toast.view(1.0);
        let _element3 = toast.view(2.0);
    }
}
