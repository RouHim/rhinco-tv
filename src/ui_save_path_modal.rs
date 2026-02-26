use iced::alignment::Horizontal;
use iced::widget::{Column, Container, Row, Text};
use iced::{Color, Element, Length};
use std::collections::HashSet;

use crate::messages::Message;
use crate::ui_theme::*;
use crate::wine_prefix_scanner::SuggestedSavePath;

#[derive(Debug, Clone)]
pub struct SuggestedSavePathDisplay {
    // Kept for future debugging/tooltip functionality - shows full resolved path
    #[allow(dead_code)]
    pub absolute_path: String,
    pub ludusavi_placeholder: String,
    pub exists: bool,
    pub is_empty: bool,
}

impl From<&SuggestedSavePath> for SuggestedSavePathDisplay {
    fn from(path: &SuggestedSavePath) -> Self {
        Self {
            absolute_path: path.absolute_path.display().to_string(),
            ludusavi_placeholder: path.ludusavi_placeholder.clone(),
            exists: path.exists,
            is_empty: path.is_empty,
        }
    }
}

pub fn render_save_path_modal(
    game_name: &str,
    suggested_paths: &[SuggestedSavePathDisplay],
    selected_indices: &HashSet<usize>,
    selected_button: usize,
    scale: f32,
) -> Element<'static, Message> {
    let title = Text::new(format!("Configure Save Paths: {}", game_name))
        .font(SANSATION)
        .size(scaled(BASE_FONT_HEADER, scale))
        .color(Color::WHITE);

    let title_container = Container::new(title)
        .padding(scaled(BASE_PADDING_MEDIUM, scale))
        .width(Length::Fill)
        .center_x(Length::Fill);

    let mut content_column = Column::new().spacing(scaled(BASE_PADDING_SMALL, scale));

    if suggested_paths.is_empty() {
        let hint_text = Text::new("We were not able to auto-detect save game paths for this game. Please configure save games manually using the Ludusavi GUI.")
            .font(SANSATION)
            .size(scaled(BASE_FONT_LARGE, scale))
            .color(COLOR_TEXT_MUTED)
            .align_x(Horizontal::Center);
        content_column = content_column.push(hint_text);
    } else {
        for (index, path) in suggested_paths.iter().enumerate() {
            let is_selected = selected_indices.contains(&index);
            let is_focused = selected_button == index;

            let status_icon = if path.exists {
                if path.is_empty {
                    "⚠"
                } else {
                    "✓"
                }
            } else {
                "✗"
            };

            let status_color = if path.exists {
                if path.is_empty {
                    Color::from_rgb(0.9, 0.9, 0.0)
                } else {
                    Color::from_rgb(0.0, 0.8, 0.0)
                }
            } else {
                Color::from_rgb(0.8, 0.0, 0.0)
            };

            let checkbox = if is_selected { "x" } else { " " };
            let label = format!(
                "[{}] {} {}",
                checkbox, status_icon, path.ludusavi_placeholder
            );

            content_column = content_column.push(path_item(label, is_focused, status_color, scale));
        }
    }

    let content_container = Container::new(content_column)
        .padding(scaled(BASE_PADDING_MEDIUM, scale))
        .width(Length::Fill)
        .center_x(Length::Fill);

    let buttons_row = if suggested_paths.is_empty() {
        let cancel_button_index = 0;
        Row::with_children(vec![modal_button(
            "Cancel",
            selected_button == cancel_button_index,
            scale,
        )])
    } else {
        let save_button_index = suggested_paths.len();
        let cancel_button_index = suggested_paths.len() + 1;
        Row::with_children(vec![
            modal_button("Save", selected_button == save_button_index, scale),
            modal_button("Cancel", selected_button == cancel_button_index, scale),
        ])
    }
    .spacing(scaled(BASE_PADDING_MEDIUM, scale));

    let buttons_container = Container::new(buttons_row)
        .padding(scaled(BASE_PADDING_SMALL, scale))
        .width(Length::Fill)
        .center_x(Length::Fill);

    let hint = Text::new("Press B or − to close")
        .font(SANSATION)
        .size(scaled(BASE_FONT_SMALL, scale))
        .color(COLOR_TEXT_HINT);

    let hint_container = Container::new(hint)
        .padding(scaled(BASE_PADDING_SMALL, scale))
        .width(Length::Fill)
        .center_x(Length::Fill);

    let modal_column = Column::new()
        .push(title_container)
        .push(content_container)
        .push(buttons_container)
        .push(hint_container)
        .spacing(scaled(BASE_PADDING_SMALL, scale));

    let border_radius = scaled(10.0, scale);
    let modal_box = Container::new(modal_column)
        .width(scaled_fixed(MODAL_WIDTH_MEDIUM, scale))
        .padding(scaled(BASE_PADDING_MEDIUM, scale))
        .style(move |_| iced::widget::container::Style {
            background: Some(COLOR_PANEL.into()),
            border: iced::Border {
                color: Color::WHITE,
                width: 1.0,
                radius: border_radius.into(),
            },
            ..Default::default()
        });

    Container::new(modal_box)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .padding(scaled(MODAL_OVERLAY_PADDING, scale))
        .style(|_| iced::widget::container::Style {
            background: Some(Color::TRANSPARENT.into()),
            ..Default::default()
        })
        .into()
}

fn path_item(
    label: String,
    is_focused: bool,
    status_color: Color,
    scale: f32,
) -> Element<'static, Message> {
    let text = Text::new(label)
        .font(SANSATION)
        .size(scaled(BASE_FONT_LARGE, scale))
        .color(if is_focused {
            Color::WHITE
        } else {
            status_color
        });

    let border_radius = scaled(8.0, scale);
    Container::new(text)
        .padding(scaled(12.0, scale))
        .width(Length::Fill)
        .style(move |_| {
            if is_focused {
                iced::widget::container::Style {
                    background: Some(COLOR_ACCENT.into()),
                    text_color: Some(Color::WHITE),
                    border: iced::Border {
                        color: Color::WHITE,
                        width: 1.0,
                        radius: border_radius.into(),
                    },
                    ..Default::default()
                }
            } else {
                iced::widget::container::Style {
                    background: Some(COLOR_PANEL.into()),
                    text_color: Some(status_color),
                    border: iced::Border {
                        color: COLOR_TEXT_MUTED,
                        width: 1.0,
                        radius: border_radius.into(),
                    },
                    ..Default::default()
                }
            }
        })
        .into()
}

fn modal_button(label: &'static str, is_selected: bool, scale: f32) -> Element<'static, Message> {
    let text = Text::new(label)
        .font(SANSATION)
        .size(scaled(BASE_FONT_LARGE, scale))
        .color(if is_selected {
            Color::WHITE
        } else {
            COLOR_TEXT_MUTED
        })
        .align_x(Horizontal::Center);

    let border_radius = scaled(8.0, scale);
    Container::new(text)
        .padding(scaled(12.0, scale))
        .width(Length::Fill)
        .center_x(Length::Fill)
        .style(move |_| {
            if is_selected {
                iced::widget::container::Style {
                    background: Some(COLOR_ACCENT.into()),
                    text_color: Some(Color::WHITE),
                    border: iced::Border {
                        color: Color::WHITE,
                        width: 1.0,
                        radius: border_radius.into(),
                    },
                    ..Default::default()
                }
            } else {
                iced::widget::container::Style {
                    background: Some(COLOR_PANEL.into()),
                    text_color: Some(COLOR_TEXT_MUTED),
                    border: iced::Border {
                        color: COLOR_TEXT_MUTED,
                        width: 1.0,
                        radius: border_radius.into(),
                    },
                    ..Default::default()
                }
            }
        })
        .into()
}

const SPINNER_FRAMES: [&str; 4] = ["●  ", " ● ", "  ●", " ● "];

pub fn render_save_path_scanning_modal(
    game_name: &str,
    spinner_tick: usize,
    scale: f32,
) -> Element<'static, Message> {
    let title = Text::new("Scanning Save Paths...")
        .font(SANSATION)
        .size(scaled(BASE_FONT_HEADER, scale))
        .color(Color::WHITE)
        .align_x(Horizontal::Center);

    let subtitle = Text::new(game_name.to_string())
        .font(SANSATION)
        .size(scaled(BASE_FONT_LARGE, scale))
        .color(COLOR_TEXT_BRIGHT)
        .align_x(Horizontal::Center);

    let spinner = Text::new(SPINNER_FRAMES[spinner_tick % SPINNER_FRAMES.len()])
        .font(SANSATION)
        .size(scaled(BASE_FONT_DISPLAY, scale))
        .color(COLOR_ACCENT)
        .align_x(Horizontal::Center);

    let hint = Text::new("Searching wine prefixes")
        .font(SANSATION)
        .size(scaled(BASE_FONT_MEDIUM, scale))
        .color(COLOR_TEXT_HINT)
        .align_x(Horizontal::Center);

    let modal_content = Column::new()
        .push(title)
        .push(subtitle)
        .push(spinner)
        .push(hint)
        .spacing(scaled(BASE_PADDING_MEDIUM, scale))
        .align_x(Horizontal::Center);

    let border_radius = scaled(10.0, scale);
    let modal_box = Container::new(modal_content)
        .padding(scaled(BASE_PADDING_LARGE, scale))
        .width(scaled_fixed(MODAL_WIDTH_MEDIUM, scale))
        .style(move |_| iced::widget::container::Style {
            background: Some(COLOR_PANEL.into()),
            border: iced::Border {
                color: Color::WHITE,
                width: 1.0,
                radius: border_radius.into(),
            },
            ..Default::default()
        });

    Container::new(modal_box)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .padding(scaled(MODAL_OVERLAY_PADDING, scale))
        .style(|_| iced::widget::container::Style {
            background: Some(Color::TRANSPARENT.into()),
            ..Default::default()
        })
        .into()
}
