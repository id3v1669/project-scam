use crate::iced_launch::Message;
use crate::objects::game_data::EmailQuestItem;
use iced::advanced::Widget;
use iced::theme;
use iced::widget::{button, column, container, horizontal_space, row, stack, text, text_input};
use iced::{Color, Element, Length};

pub fn view(
    location: crate::objects::game_data::EmailQuestLocation,
    sublocation: crate::objects::game_data::EmailQuestSubLocation,
    hinted: crate::objects::game_data::EmailQuestItem,
    dynamic_objects: Vec<String>,
) -> Element<'static, Message> {
    let game_data = crate::objects::game_data::GAMEDATA.lock().unwrap();
    let game_bar = crate::objects::custom_gui::unified_top_bar(&game_data);
    let top_bar = container(row![
        button("New email")
            .on_press(Message::ChangeMailboxFolderSublocation(
                crate::objects::game_data::EmailQuestSubLocation::NewEmail,
            ))
            .style(
                move |_, status| crate::styles::buttons::regular_rounded_no_border(status, false)
            ), //hinted == EmailQuestItem::NewEmail))
    ])
    .style(|_| container::Style {
        background: Some(iced::Background::Color(Color::from_rgb8(0x29, 0x29, 0x29))),
        ..Default::default()
    })
    .padding(10)
    .width(Length::Fill);

    // Left column
    let left_column = column![
        button("Inbox")
            .on_press(Message::ChangeMailboxFolderLocation(
                crate::objects::game_data::EmailQuestLocation::Inbox
            ))
            .style(move |_, status| {
                let inbox_button_active = matches!(
                    location,
                    crate::objects::game_data::EmailQuestLocation::Inbox
                );
                let inbox_button_hinted = (matches!(
                    hinted,
                    EmailQuestItem::EmailSender //| EmailQuestItem::BlockedContent
                                                //| EmailQuestItem::EmailAttachment
                ) && !inbox_button_active);
                crate::styles::buttons::email_folder_button(
                    status,
                    inbox_button_hinted,
                    inbox_button_active,
                )
            })
            .width(Length::Fill),
        button("Spam")
            .on_press(Message::ChangeMailboxFolderLocation(
                crate::objects::game_data::EmailQuestLocation::Spam
            ))
            .style(move |_, status| {
                let spam_button_active = matches!(
                    location,
                    crate::objects::game_data::EmailQuestLocation::Spam
                );
                let spam_button_hinted = (matches!(
                    hinted,
                    EmailQuestItem::SpamFolder | EmailQuestItem::SpamEmail
                ) && !spam_button_active);
                crate::styles::buttons::email_folder_button(
                    status,
                    spam_button_hinted,
                    spam_button_active,
                )
            })
            .width(Length::Fill),
    ]
    .width(Length::Fixed(100.0));

    let middle_column = match location {
        crate::objects::game_data::EmailQuestLocation::Inbox => {
            iced::widget::Column::with_children(
                std::iter::once(
                    button(
                        column![
                            text("Reset Password").size(16).font(iced::Font {
                                family: iced::font::Family::SansSerif,
                                weight: iced::font::Weight::Bold,
                                stretch: iced::font::Stretch::Normal,
                                style: iced::font::Style::Normal,
                            }),
                            text("To recover password ...").size(14)
                        ]
                        .spacing(4),
                    )
                    .on_press(Message::ChangeMailboxFolderSublocation(
                        crate::objects::game_data::EmailQuestSubLocation::Inbox1,
                    ))
                    .padding(iced::Padding {
                        top: 0.0,
                        right: 0.0,
                        bottom: 0.0,
                        left: 0.0,
                    })
                    .style(move |_, status| {
                        let email = matches!(
                            sublocation,
                            crate::objects::game_data::EmailQuestSubLocation::Inbox1
                        );
                        let email_button_hinted = (matches!(
                            hinted,
                            EmailQuestItem::EmailSender //| EmailQuestItem::BlockedContent
                                                        //| EmailQuestItem::EmailAttachment
                        ) && !email);
                        crate::styles::buttons::email_button(status, email_button_hinted, email)
                    })
                    .width(Length::Fill)
                    .height(Length::Fixed(50.0))
                    .into(),
                )
                .chain((1..=50).map(|i| {
                    container(iced::widget::text(""))
                        .style(|_| crate::styles::other::empty_email())
                        .padding(iced::Padding {
                            top: 15.0,
                            right: 10.0,
                            bottom: 15.0,
                            left: 10.0,
                        })
                        .width(Length::Fill)
                        .height(Length::Fixed(50.0))
                        .into()
                }))
                .collect::<Vec<Element<Message>>>(),
            )
            .width(Length::Fixed(200.0))
            .align_x(iced::alignment::Horizontal::Left)
            .spacing(0)
        }
        crate::objects::game_data::EmailQuestLocation::Spam => {
            iced::widget::Column::with_children(
                std::iter::once(
                    //container(
                    button(
                        column![
                            text("Trixie").size(16).font(iced::Font {
                                family: iced::font::Family::SansSerif,
                                weight: iced::font::Weight::Bold,
                                stretch: iced::font::Stretch::Normal,
                                style: iced::font::Style::Normal,
                            }),
                            text("Me met a few ...").size(14)
                        ]
                        .spacing(4),
                    )
                    .on_press(Message::ChangeMailboxFolderSublocation(
                        crate::objects::game_data::EmailQuestSubLocation::Spam,
                    ))
                    .padding(iced::Padding {
                        top: 0.0,
                        right: 0.0,
                        bottom: 0.0,
                        left: 0.0,
                    })
                    .style(move |_, status| {
                        let email = matches!(
                            sublocation,
                            crate::objects::game_data::EmailQuestSubLocation::Spam
                        );
                        let email_button_hinted = (matches!(
                            hinted,
                            EmailQuestItem::SpamFolder | EmailQuestItem::SpamEmail
                        ) && !email);
                        crate::styles::buttons::email_button(status, email_button_hinted, email)
                    })
                    .width(Length::Fill)
                    .height(Length::Fixed(50.0))
                    .into(),
                )
                .chain((1..=50).map(|i| {
                    container(iced::widget::text(""))
                        .style(|_| crate::styles::other::empty_email())
                        .padding(iced::Padding {
                            top: 15.0,
                            right: 10.0,
                            bottom: 15.0,
                            left: 10.0,
                        })
                        .width(Length::Fill)
                        .height(Length::Fixed(50.0))
                        .into()
                }))
                .collect::<Vec<Element<Message>>>(),
            )
            .width(Length::Fixed(200.0))
            .align_x(iced::alignment::Horizontal::Left)
            .spacing(0)
        }
        _ => column![],
    };

    let right_column = match sublocation {
        crate::objects::game_data::EmailQuestSubLocation::None => {
            container(text("Select email to read"))
            .width(Length::Fill)
                .height(Length::Fill)
                .center(Length::Fill)
                .style(|_| container::Style {
                    text_color: Color::parse("#ebdbb2").unwrap().into(),
                    ..Default::default()
                })
        }
        crate::objects::game_data::EmailQuestSubLocation::Inbox1 => {
            container(crate::objects::custom_gui::email_base(
                hinted == EmailQuestItem::EmailSender,
                "Reset Password".to_string(),
                dynamic_objects[1].clone(),
                1,
                2,
                column![text("Convining email with recovery link.").size(25),],
            ))
        }
        crate::objects::game_data::EmailQuestSubLocation::Spam => {
            container(crate::objects::custom_gui::email_base(
                hinted == EmailQuestItem::EmailSender,
                "Trixie".to_string(),
                dynamic_objects[3].clone(),
                3,
                4,
                column![text("Convining email with recovery link.").size(25),],
            ))
        }
        crate::objects::game_data::EmailQuestSubLocation::NewEmail => {
            container(column![
                container(
                    text_input("Title", &dynamic_objects[7])
                        .on_input(|s| Message::ContentChanged(7, s))
                        .style(|_, _| text_input::Style {
                            background: Color::parse("#7c6f64").unwrap().into(),
                            value: Color::parse("#ebdbb2").unwrap().into(),
                            border: iced::Border::default(),
                            icon: Color::TRANSPARENT.into(),
                            placeholder: Color::parse("#a89984").unwrap().into(),
                            selection: Color::TRANSPARENT.into(),
                        })
                )
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
                        text_input("Email...", &dynamic_objects[6])
                            .on_input(|s| Message::ContentChanged(6, s))
                            .style(|_, _| text_input::Style {
                                background: Color::parse("#7c6f64").unwrap().into(),
                                value: Color::parse("#ebdbb2").unwrap().into(),
                                border: iced::Border::default(),
                                icon: Color::TRANSPARENT.into(),
                                placeholder: Color::TRANSPARENT.into(),
                                selection: Color::TRANSPARENT.into(),
                            })
                            .width(Length::Fixed(200.0)),
                        horizontal_space(),
                        button("Send")
                            .on_press(Message::SendEmail)
                            .style(|_, status| {
                                crate::styles::buttons::regular_rounded_border_as_margin(
                                    status, false, 5.0,
                                )
                            })
                    ],
                    crate::objects::custom_gui::splitter(
                        Color::parse("#928374").unwrap().into(),
                        2.0,
                        10.0,
                        10.0
                    ),
                    container(
                        text_input("Email...", &dynamic_objects[5])
                            .on_input(|s| Message::ContentChanged(5, s))
                            .style(|_, _| text_input::Style {
                                background: Color::TRANSPARENT.into(),
                                value: Color::parse("#ebdbb2").unwrap().into(),
                                border: iced::Border::default(),
                                icon: Color::TRANSPARENT.into(),
                                placeholder: Color::TRANSPARENT.into(),
                                selection: Color::TRANSPARENT.into(),
                            })
                    )
                    .style(|_| iced::widget::container::Style {
                        background: Some(Color::parse("#7c6f64").unwrap().into()),
                        text_color: iced::Color::parse("#ebdbb2").unwrap().into(),
                        border: crate::styles::borders::email_border(),
                        ..Default::default()
                    })
                    .height(Length::Fill)
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
            //container(
            //    text_input("Type something here...", &dynamic_objects[5])
            //    .on_input(|s|Message::ContentChanged(5,s))
            //)
        }
    };

    // Main content with dark background
    let main_content = container(column![
        game_bar,
        top_bar,
        row![left_column, middle_column, right_column].height(Length::Fill)
    ])
    .style(|_| container::Style {
        background: Some(Color::parse("#1E1E1E").unwrap().into()),
        ..Default::default()
    });

    // Overlay with dimmed effect
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
        // Main content at the bottom layer
        container(main_content)
            .width(Length::Fill)
            .height(Length::Fill),
        // Overlay on top
        overlay
    ]
    .into()
}
