use iced::alignment::Horizontal;
use iced::widget::{Column, Container, Text};
use iced::{Color, Element, Length};

use crate::messages::Message;
use crate::ui_theme::*;
use crate::virtual_keyboard::VirtualKeyboard;

pub fn render_settings_modal<'a>(
    selected_index: usize,
    autostart_enabled: bool,
    auto_backup: bool,
    auto_cloud_sync: bool,
    api_key_set: bool,
    editing_api_key: bool,
    keyboard: Option<&'a VirtualKeyboard>,
    scale: f32,
) -> Element<'a, Message> {
    if editing_api_key {
        if let Some(kb) = keyboard {
            return render_keyboard_overlay(kb, scale);
        }
    }

    let title = Text::new("Settings")
        .font(SANSATION)
        .size(scaled(BASE_FONT_HEADER, scale))
        .color(Color::WHITE);

    let title_container = Container::new(title)
        .padding(scaled(BASE_PADDING_MEDIUM, scale))
        .width(Length::Fill)
        .center_x(Length::Fill);

    let mut content_column = Column::new().spacing(scaled(BASE_PADDING_SMALL, scale));

    content_column = content_column.push(modal_item(
        format!(
            "[{}] Autostart at login",
            if autostart_enabled { "x" } else { " " }
        ),
        selected_index == 0,
        scale,
    ));

    content_column = content_column.push(modal_item(
        format!(
            "[{}] Auto-backup on game exit",
            if auto_backup { "x" } else { " " }
        ),
        selected_index == 1,
        scale,
    ));

    content_column = content_column.push(modal_item(
        format!(
            "[{}] Auto-sync to cloud",
            if auto_cloud_sync { "x" } else { " " }
        ),
        selected_index == 2,
        scale,
    ));

    content_column = content_column.push(modal_item(
        format!(
            "SteamGridDB API Key: [{}]",
            if api_key_set { "Set" } else { "Not Set" }
        ),
        selected_index == 3,
        scale,
    ));

    content_column =
        content_column.push(modal_item("Close".to_string(), selected_index == 4, scale));

    let content_container = Container::new(content_column)
        .padding(scaled(BASE_PADDING_MEDIUM, scale))
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

fn modal_item(label: String, is_selected: bool, scale: f32) -> Element<'static, Message> {
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

fn render_keyboard_overlay<'a>(keyboard: &'a VirtualKeyboard, scale: f32) -> Element<'a, Message> {
    let title = Text::new("Enter SteamGridDB API Key")
        .font(SANSATION)
        .size(scaled(BASE_FONT_HEADER, scale))
        .color(Color::WHITE);

    let title_container = Container::new(title)
        .padding(scaled(BASE_PADDING_MEDIUM, scale))
        .width(Length::Fill)
        .center_x(Length::Fill);

    let input_display = Text::new(keyboard.display_value())
        .font(SANSATION)
        .size(scaled(BASE_FONT_LARGE, scale))
        .color(COLOR_TEXT_BRIGHT)
        .align_x(Horizontal::Center);

    let input_container = Container::new(input_display)
        .padding(scaled(BASE_PADDING_MEDIUM, scale))
        .width(Length::Fill)
        .center_x(Length::Fill)
        .style(move |_| iced::widget::container::Style {
            background: Some(COLOR_PANEL.into()),
            border: iced::Border {
                color: Color::WHITE,
                width: 1.0,
                radius: scaled(6.0, scale).into(),
            },
            ..Default::default()
        });

    let keyboard_view = keyboard.view(scale).map(Message::SettingsKeyboard);

    let content_column = Column::new()
        .spacing(scaled(BASE_PADDING_SMALL, scale))
        .push(title_container)
        .push(input_container)
        .push(keyboard_view)
        .align_x(Horizontal::Center);

    let border_radius = scaled(10.0, scale);
    let modal_box = Container::new(content_column)
        .width(scaled_fixed(MODAL_WIDTH_LARGE, scale))
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
