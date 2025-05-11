use iced::widget::{Container, button, column, container, horizontal_space, row, text};
use iced::{Color, Length};

use crate::iced_launch::Message;
use crate::objects::game_data::{EmailQuestItem, GameData};

pub fn splitter(
    color: iced::Color,
    width: f32,
    margin_top: f32,
    margin_bottom: f32,
) -> iced::Element<'static, crate::iced_launch::Message> {
    column![
        container(text(""))
            .width(Length::Fill)
            .height(Length::Fixed(margin_top)),
        container(text(""))
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(color)),
                ..Default::default()
            })
            .width(Length::Fill)
            .height(Length::Fixed(width)),
        container(text(""))
            .width(Length::Fill)
            .height(Length::Fixed(margin_bottom))
    ]
    .into()
}

pub fn unified_top_bar(
    game_data: &GameData,
) -> iced::Element<'static, crate::iced_launch::Message> {
    container(row![
        button("Menu")
            .on_press(Message::MainMenu)
            .style(
                |_, status| crate::styles::buttons::regular_rounded_border_as_margin(
                    status, false, 5.0
                )
            )
            .width(Length::Fixed(70.0)),
        button("Hint")
            .on_press(Message::Hint)
            .style(
                |_, status| crate::styles::buttons::regular_rounded_border_as_margin(
                    status, false, 5.0
                )
            )
            .width(Length::Fixed(70.0)),
        horizontal_space(),
        text(format!(
            "Total stars: {}/{}",
            game_data.total_stars(),
            game_data.total_quests(),
        )),
        text("  |  "),
        text(format!(
            "Total quests: {}/{}",
            game_data.total_completed_quests(),
            game_data.total_quests(),
        )),
    ])
    .style(|_| iced::widget::container::Style {
        text_color: iced::Color::parse("#ebdbb2").unwrap().into(),
        ..Default::default()
    })
    .height(Length::Fixed(30.0))
    .into()
}

pub fn email_base(
    hinted: bool,
    title: String,
    emailfrom: String,
    efid1: usize,
    efid2: usize,
    content: impl Into<iced::Element<'static, crate::iced_launch::Message>>,
) -> iced::Element<'static, crate::iced_launch::Message> {
    container(column![
        container(text(title).size(20).font(iced::Font {
            family: iced::font::Family::SansSerif,
            weight: iced::font::Weight::Bold,
            stretch: iced::font::Stretch::Normal,
            style: iced::font::Style::Normal,
        }))
        .width(Length::Fill)
        .height(Length::Fixed(60.0))
        .padding(15) // workadound for text misalignment with container
        .style(|_| container::Style {
            background: Some(Color::parse("#504945").unwrap().into()),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 10.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }),
        container(column![
            row![
                text("From: ").size(16),
                button(text(emailfrom))
                    .on_press(Message::SwapText(efid1, efid2))
                    .style(
                        move |_, status| crate::styles::buttons::regular_rounded_no_border(
                            status, hinted,
                        )
                    )
                    .height(Length::Shrink)
                    .width(Length::Shrink),
            ],
            row![text("To: example@domain.com").size(16),],
            crate::objects::custom_gui::splitter(
                Color::parse("#928374").unwrap().into(),
                2.0,
                10.0,
                10.0
            ),
            content.into(),
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(15) //workadound for content misalignment with container
        .style(|_| container::Style {
            background: Some(Color::parse("#504945").unwrap().into()),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 10.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
    ])
    .style(|_| container::Style {
        text_color: Color::parse("#ebdbb2").unwrap().into(),
        ..Default::default()
    })
    .padding(10)
    .width(Length::Fill)
    .into()
}
