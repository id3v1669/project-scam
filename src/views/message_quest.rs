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

// pub fn smartphone_messages_ui() -> iced::widget::Container<'static> {
//     // Mock messages data
//     let messages = vec![
//         ("Hey there!", true),   // (content, is_received)
//         ("Hi! How are you?", false),
//         ("I'm good, thanks!", true),
//         ("Want to meet up?", false),
//     ];

//     container(
//         column![
//             // Status bar
//             container(
//                 row![
//                     text("9:41").size(14),
//                     horizontal_space(),
//                     row![
//                         text("📶").size(14),
//                         text("100%").size(14),
//                     ]
//                     .spacing(5)
//                 ]
//                 .padding(10)
//             )
//             .style(status_bar_style())
//             .width(Length::Fill),

//             // Messages area
//             scrollable(
//                 column![
//                     messages.iter().map(|(msg, received)| {
//                         container(
//                             text(*msg)
//                                 .size(16)
//                                 .horizontal_alignment(if *received {
//                                     iced::alignment::Horizontal::Left
//                                 } else {
//                                     iced::alignment::Horizontal::Right
//                                 })
//                         )
//                         .style(if *received {
//                             received_message_style()
//                         } else {
//                             sent_message_style()
//                         })
//                         .padding(10)
//                         .width(Length::Fill)
//                     })
//                     .collect::<Vec<_>>()
//                 ]
//                 .spacing(10)
//                 .padding(20)
//             )
//             .height(Length::Fill)
//             .width(Length::Fill),

//             // Input area
//             container(
//                 row![
//                     text_input("Type a message...", "")
//                         .padding(10)
//                         .width(Length::Fill),
//                     button("📤")
//                         .padding(10)
//                         .style(send_button_style())
//                 ]
//                 .spacing(10)
//                 .align_items(Alignment::Center)
//             )
//             .style(input_area_style())
//             .width(Length::Fill)
//             .padding(10)
//         ]
//         //.spacing(0)
//     )
//     .style(phone_container_style())
//     .width(Length::Fixed(375.0))  // Typical smartphone width
//     .height(Length::Fixed(667.0)) // Typical smartphone height
//     .center_x()
//     .center_y()
// }

// Style functions
// fn phone_container_style() -> iced::theme::Container {
//     iced::theme::Container::Custom(Box::new(|theme| iced::widget::container::Appearance {
//         background: Some(iced::Background::Color(Color::from_rgb(0.1, 0.1, 0.1))),
//         border: iced::Border {
//             color: Color::BLACK,
//             width: 2.0,
//             radius: 20.0.into(),
//         },
//         ..theme.appearance(&iced::theme::Container::default())
//     }))
// }

// fn status_bar_style() -> iced::theme::Container {
//     iced::theme::Container::Custom(Box::new(|theme| iced::widget::container::Appearance {
//         background: Some(iced::Background::Color(Color::from_rgb(0.15, 0.15, 0.15))),
//         text_color: Some(Color::WHITE),
//         ..theme.appearance(&iced::theme::Container::default())
//     }))
// }

// fn sent_message_style() -> iced::widget::container::Style {
//     iced::theme::Container::Custom(Box::new(|_| iced::widget::container::Appearance {
//         background: Some(iced::Background::Color(Color::from_rgb(0.0, 0.4, 1.0))),
//         border: iced::Border {
//             color: Color::TRANSPARENT,
//             width: 0.0,
//             radius: 15.0.into(),
//         },
//         ..Default::default()
//     }))
// }

// fn received_message_style() -> iced::widget::container::Style {
//     iced::widget::container::Style {
//         background: Some(iced::Background::Color(Color::from_rgb(0.2, 0.2, 0.2))),
//         border: iced::Border {
//             color: Color::TRANSPARENT,
//             width: 0.0,
//             radius: 15.0.into(),
//         },
//         ..Default::default()
//     }
// }

// fn input_area_style() -> iced::widget::button::Style {
//     iced::widget::button::Style {
//         background: Some(iced::Background::Color(Color::from_rgb(0.15, 0.15, 0.15))),
//         border: iced::Border {
//             color: Color::TRANSPARENT,
//             width: 0.0,
//             radius: 10.0.into(),
//         },
//         ..Default::default()
//     }
// }

// fn send_button_style() -> iced::widget::button::Style {
//     iced::widget::button::Style {
//         background: Some(iced::Background::Color(Color::from_rgb(0.0, 0.4, 1.0))),
//         //border_radius: 8.0.into(),
//         text_color: Color::WHITE,
//         ..Default::default()
//     }
// }
