use iced::widget::{button, column, container, row, text, stack, horizontal_space};
use iced::{Color, Element, Length};
use crate::iced_launch::Message;
use iced::advanced::Widget;
use iced::theme;

pub fn view() -> Element<'static, Message> {
    // Top row with background color
    let top_bar = container(
        row![
            button("New email").on_press(Message::Empty)
                .style(|_, _| iced::widget::button::Style {
                    background: Some(Color::parse("#A1A1A1").unwrap().into()),
                    text_color: Color::parse("#7A7A7A").unwrap().into(),
                    border: iced::Border {
                        color: Color::parse("#b8bb26").unwrap().into(),
                        width: 3.0,
                        radius: 6.0.into(),
                    },
                    ..Default::default()
                }),
        ]
    )
    .style(|_| container::Style {
        background: Some(iced::Background::Color(Color::from_rgb8(0x29, 0x29, 0x29))),
        ..Default::default()
    })
    .padding(10)
    .width(Length::Fill);

    // Left column
    let left_column = column![
        button("Inbox").on_press(Message::Empty),
        button("Spam").on_press(Message::Empty),
    ]
    .width(Length::FillPortion(1));

    // Right column
    let right_column = column![
        text("Field 1"),
        text("Field 2"),
        text("Field 3"),
    ]
    .width(Length::FillPortion(3));

    // Main content with dark background
    let main_content = container(
        column![
            top_bar,
            row![left_column, right_column].height(Length::Fill)
        ]
    )
    .style(|_| container::Style {
        background: Some(Color::parse("#1E1E1E").unwrap().into()),
        ..Default::default()
    });

    // Overlay with dimmed effect
    let overlay = container(
        container(
            column![
                row![
                    horizontal_space(),
                    button("x")
                        .on_press(Message::Empty)
                        .style(|_, _| iced::widget::button::Style {
                            background: Some(Color::parse("#282828").unwrap().into()),
                            text_color: Color::parse("#fb4934").unwrap().into(),
                            border: iced::Border {
                                color: Color::parse("#fb4934").unwrap().into(),
                                width: 1.0,
                                radius: 12.0.into(),
                            },
                            ..Default::default()
                        })
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
                container(text("First task would be to write an email. \nButtons not related to quiz but required to press will have green border. \nTo begin, close this window and press New email button."))
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
            .spacing(20)
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
        .width(Length::Fixed(300.0)) // text container size required as horizontal_space doesn't respect Shrink an takes all screen
    )
    .center(Length::Fill) // fill the screen and center the content of overlay
    .style(|_| container::Style {
        background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.7).into()),
        ..Default::default()
    });

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

// fn close_button_style() -> iced::Theme::Custom {
//     iced::Theme::Custom(Box::new(|theme| iced::widget::button::Style {
//         background: Some(iced::Background::Color(Color::TRANSPARENT)),
//         border: iced::Border {
//             color: theme.palette().text,
//             width: 1.0,
//             radius: 12.0.into(),
//         },
//         text_color: theme.palette().text,
//         ..Default::default()
//     }))
// }


                // Close button row
                //container(
                //     row![
                //     button(
                //         text("x")
                //             .size(20)
                //             //.horizontal_alignment()
                //     )
                //     .on_press(Message::Empty)
                //     .padding(iced::Padding {
                //         top: 1.0, 
                //         right: 8.0, 
                //         bottom:3.0, 
                //         left: 8.0
                //     })
                //     .style(|_, _| iced::widget::button::Style {
                //         background: Some(iced::Background::Color(Color::TRANSPARENT)),
                //         border: iced::Border {
                //             color: Color::parse("#ff00ff").unwrap().into(),
                //             width: 1.0,
                //             radius: 12.0.into(),
                //         },
                //         text_color: Color::parse("#ffffff").unwrap().into(),
                //         ..Default::default()
                //     }),
                // ]
                // .width(Length::Fill)
                // .align_y(iced::Alignment::End),
                // ).style(|_| container::Style {
                //     border: iced::Border {
                //         color: Color::parse("#ffff00").unwrap().into(),
                //         width: 1.0,
                //         radius: 8.0.into(),
                //     },
                //     ..Default::default()
                // })
                // .width(Length::Fill),
                // Content