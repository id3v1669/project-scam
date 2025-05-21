use crate::iced_launch::Message;
use crate::objects::custom_gui::splitter;
use iced::Color;
use iced::Element;
use iced::Length;
use iced::widget::{button, column, container, horizontal_space, row, stack, text};

pub fn view(
    location: crate::objects::game_data::MessageQuestLocation,
    sublocation: crate::objects::game_data::FillerItemLocation,
    hinted: crate::objects::game_data::MessageQuestItem,
    dynamic_objects: Vec<String>,
) -> Element<'static, crate::iced_launch::Message> {
    let game_data = crate::objects::game_data::GAMEDATA.lock().unwrap();
    let game_bar = crate::objects::custom_gui::unified_top_bar(&game_data);
    let main = container(
        column![
            container(column![
                container(column![
                    row![text("|||").size(14), text("100%").size(14),],
                    text("Messaging").size(20),
                    splitter(iced::Color::TRANSPARENT, 15.0, 0.0, 0.0),
                ])
                .style(|_| iced::widget::container::Style {
                    background: Some(iced::Color::parse("98971a").unwrap().into()),
                    border: iced::Border {
                        color: iced::Color::TRANSPARENT.into(),
                        width: 0.0,
                        radius: 0.0.into(),
                    },
                    ..Default::default()
                })
                .width(Length::Fill),
                //container(
                button(row![
                    column![
                        text("Unknown number").size(18),
                        text("Mom, I'm in trouble ...").size(13),
                    ],
                    horizontal_space(),
                    button("Quick Reply")
                        .on_press(Message::QuestButtonClicked)
                        .style(move |_, status| {
                            crate::styles::buttons::regular_rounded_no_border(
                                status,
                                (hinted
                                    == crate::objects::game_data::MessageQuestItem::Manipulation),
                            )
                        })
                        .width(Length::Fixed(70.0)),
                ])
                .style(|_, _| {
                    iced::widget::button::Style {
                        background: Some(iced::Background::Color(Color::TRANSPARENT)),
                        text_color: Color::parse("#ebdbb2").unwrap().into(),
                        border: crate::styles::borders::email_border(),
                        ..Default::default()
                    }
                })
                .width(Length::Fill)
            ])
            .padding(15)
            .style(|_| iced::widget::container::Style {
                background: Some(iced::Color::parse("282828").unwrap().into()),
                border: iced::Border {
                    color: iced::Color::TRANSPARENT.into(),
                    width: 15.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
            .width(Length::Fill)
            .height(Length::Fill),
            container(text(""))
                .style(|_| iced::widget::container::Style {
                    background: Some(iced::Color::parse("282828").unwrap().into()),
                    border: iced::Border {
                        color: iced::Color::parse("#ebdbb2").unwrap().into(),
                        width: 1.0,
                        radius: 20.0.into(),
                    },
                    ..Default::default()
                })
                .width(Length::Fixed(50.0))
                .height(Length::Fixed(50.0)),
            splitter(iced::Color::TRANSPARENT, 15.0, 0.0, 0.0),
        ]
        .align_x(iced::Alignment::Center),
    )
    .style(|_| iced::widget::container::Style {
        background: Some(iced::Color::parse("#00000f").unwrap().into()),
        border: iced::Border {
            color: iced::Color::parse("#ebdbb2").unwrap().into(),
            width: 2.0,
            radius: 20.0.into(),
        },
        ..Default::default()
    })
    .width(Length::Fixed(450.0)) // Typical smartphone width
    .height(Length::Fixed(900.0)); // Typical smartphone height

    let overlay: iced::widget::Container<Message> = if !dynamic_objects[0].is_empty() {
        container(
            container(
                column![
                    row![
                        horizontal_space(),
                        button("x")
                            .on_press(Message::ClosePopup)
                            .style(|_, status| crate::styles::buttons::danger_button_style(status))
                            .padding(iced::Padding {
                                top: 1.0,
                                right: 8.0,
                                bottom: 3.0,
                                left: 8.0
                            }),
                    ]
                    .padding(iced::Padding {
                        top: 3.0,
                        right: 3.0,
                        bottom: 1.0,
                        left: 1.0
                    }),
                    container(text(dynamic_objects[0].clone()))
                        .style(|_| container::Style {
                            text_color: Color::parse("#ebdbb2").unwrap().into(),
                            ..Default::default()
                        })
                        .padding(iced::Padding {
                            top: 0.0,
                            right: 10.0,
                            bottom: 10.0,
                            left: 10.0
                        })
                ]
                .spacing(20),
            )
            .style(|_| container::Style {
                background: Some(Color::parse("#1E1E1E").unwrap().into()),
                border: iced::Border {
                    color: Color::from_rgb8(100, 100, 100),
                    width: 1.0,
                    radius: 8.0.into(),
                },
                ..Default::default()
            })
            .width(Length::Fixed(300.0)), // text container size required as horizontal_space doesn't respect Shrink an takes all screen
        )
        .center(Length::Fill) // fill the screen and center the content of overlay
        .style(|_| container::Style {
            background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.7).into()),
            ..Default::default()
        })
    } else {
        container(horizontal_space())
    };
    stack![
        container(column![game_bar, main].align_x(iced::Alignment::Center),)
            .style(|_| iced::widget::container::Style {
                text_color: iced::Color::parse("#ebdbb2").unwrap().into(),
                background: Some(iced::Color::parse("#282828").unwrap().into()),
                ..Default::default()
            })
            .center(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fill),
        overlay,
    ]
    .into()
}
