use crate::iced_launch::Message;
use iced::advanced::Widget;
use iced::theme;
use iced::widget::{button, column, container, horizontal_space, row, stack, text};
use iced::{Color, Element, Length};

pub fn view(
    show_popup: bool,
    popup_message: &String,
    active_location: i32,
) -> Element<'static, Message> {
    let top_bar =
        container(row![button("New email").on_press(Message::Empty).style(
            |_, status| crate::styles::buttons::new_email_button(status, false)
        ),])
        .style(|_| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb8(0x29, 0x29, 0x29))),
            ..Default::default()
        })
        .padding(10)
        .width(Length::Fill);

    // Left column
    let left_column = column![
        button("Inbox")
            .on_press(Message::Empty)
            .style(|_, status| crate::styles::buttons::email_folder_button(status, false, true))
            .width(Length::Fill),
        button("Spam")
            .on_press(Message::Empty)
            .style(|_, status| crate::styles::buttons::email_folder_button(status, true, false))
            .width(Length::Fill),
    ]
    .width(Length::Fixed(100.0));

    let middle_column = iced::widget::Column::with_children(
        (1..=25) // Creates a range from 1 to 25
            .map(|i| {
                container(
                    iced::widget::text("ToBeEmail").align_x(iced::alignment::Horizontal::Left),
                )
                .style(|_| crate::styles::other::empty_email())
                .padding(iced::Padding {
                    top: 15.0,
                    right: 10.0,
                    bottom: 15.0,
                    left: 10.0,
                })
                .width(Length::Fill)
                .into()
            })
            .collect::<Vec<Element<Message>>>(), // Collect into Vec<Element>
    )
    .width(Length::Fixed(200.0))
    .align_x(iced::alignment::Horizontal::Left)
    .spacing(0);

    let right_column = column![text("Select an item to read"),];

    // Main content with dark background
    let main_content = container(column![
        top_bar,
        row![left_column, middle_column, right_column].height(Length::Fill)
    ])
    .style(|_| container::Style {
        background: Some(Color::parse("#1E1E1E").unwrap().into()),
        ..Default::default()
    });

    // // Overlay with dimmed effect
    //: iced::widget::Container<Message>
    let overlay: iced::widget::Container<Message> = if show_popup {
        container(
            container(
                column![
                    row![
                        horizontal_space(),
                        button("x")
                            .on_press(Message::ClosePopup)
                            .style(|_: &iced::Theme, _| iced::widget::button::Style {
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
