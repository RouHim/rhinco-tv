use iced::alignment::Horizontal;
use iced::widget::{Column, Container, Text};
use iced::{Color, Element, Length};

use crate::messages::Message;
use crate::ui_theme::*;

const SPINNER_FRAMES: [&str; 4] = ["●  ", " ● ", "  ●", " ● "];

pub fn render_ludusavi_progress_modal<'a>(
    operation_name: &'a str,
    game_name: &'a str,
    spinner_tick: usize,
    scale: f32,
) -> Element<'a, Message> {
    let title = Text::new(format!("{}...", operation_name))
        .font(SANSATION)
        .size(scaled(BASE_FONT_HEADER, scale))
        .color(Color::WHITE)
        .align_x(Horizontal::Center);

    let subtitle = Text::new(game_name)
        .font(SANSATION)
        .size(scaled(BASE_FONT_LARGE, scale))
        .color(COLOR_TEXT_BRIGHT)
        .align_x(Horizontal::Center);

    let spinner = Text::new(SPINNER_FRAMES[spinner_tick % SPINNER_FRAMES.len()])
        .font(SANSATION)
        .size(scaled(BASE_FONT_DISPLAY, scale))
        .color(COLOR_ACCENT)
        .align_x(Horizontal::Center);

    let hint = Text::new("Please wait")
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
