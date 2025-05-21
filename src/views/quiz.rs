use crate::iced_launch::Message;
use crate::objects::custom_gui::splitter;
use iced::Alignment;
use iced::Color;
use iced::Element;
use iced::Length;
use iced::widget::canvas::Image;
use iced::widget::{
    button, column, container, horizontal_space, image, row, scrollable, stack, text,
};

use crate::styles::buttons::regular_rounded_no_border;

pub fn view(
    _location: crate::objects::game_data::FillerItemLocation,
    _sublocation: crate::objects::game_data::FillerItemLocation,
    _hinted: crate::objects::game_data::FillerItemLocation,
    dynamic_objects: Vec<String>,
) -> Element<'static, crate::iced_launch::Message> {
    let game_data = crate::objects::game_data::GAMEDATA.lock().unwrap();

    // Clone needed values FIRST
    let id_val = dynamic_objects[1].clone();
    let id = 4 + 8 * id_val.parse::<usize>().unwrap();

    // Clone frequently used values
    let do5 = dynamic_objects[id + 5].clone(); // Used in multiple closures
    let do1 = dynamic_objects[1].clone(); // Used in Next button
    let do2 = dynamic_objects[2].clone(); // Used in Next button

    // Helper macro to reduce repetition
    macro_rules! create_button {
        ($offset:expr) => {{
            let content = dynamic_objects[id + $offset].clone();
            let check_value = dynamic_objects[id + 5].clone(); // Clone specific value

            button(text(content))
                .on_press(Message::ContentChanged(id + 5, ($offset + 1).to_string()))
                .style(move |_, status| {
                    regular_rounded_no_border(
                        status,
                        ($offset + 1) == check_value.parse::<usize>().unwrap_or(usize::MAX),
                    )
                })
                .width(Length::Fixed(300.0))
        }};
    }

    macro_rules! create_container_option {
        ($offset:expr, $index:expr) => {{
            let local_id = 4 + 8 * $index;
            let content = dynamic_objects[local_id + $offset].clone();
            let check_value = dynamic_objects[local_id + 5].clone();
            let fioofas = $offset.clone() + 1;
            let tmp = fioofas.to_string();
            let offset_char = tmp.chars().nth(0).unwrap();
            let ofbol: bool = dynamic_objects[local_id + 4].contains(offset_char);
            container(text(content))
                .style(move |_| iced::widget::container::Style {
                    text_color: iced::Color::parse("#1d2021").unwrap().into(),
                    background: {
                        if check_value == fioofas.to_string() {
                            if ofbol.clone() {
                                Some(Color::parse("#b8bb26").unwrap().into())
                            } else {
                                Some(Color::parse("#fb4934").unwrap().into())
                            }
                        } else {
                            Some(Color::parse("#a89984").unwrap().into())
                        }
                    },
                    border: if ofbol.clone() {
                        iced::Border {
                            color: iced::Color::parse("#b8bb26").unwrap().into(),
                            width: 2.0,
                            radius: 10.0.into(),
                        }
                    } else {
                        iced::Border {
                            color: iced::Color::parse("#fb4934").unwrap().into(),
                            width: 2.0,
                            radius: 10.0.into(),
                        }
                    },
                    ..Default::default()
                })
                .padding(5)
                .width(Length::Fixed(300.0))
                .align_x(Alignment::Center)
        }};
    }

    let results = container(scrollable(
        iced::widget::Column::with_children(
            std::iter::once(splitter(Color::TRANSPARENT, 15.0, 0.0, 0.0).into())
                .chain(
                    (0..dynamic_objects[2].parse::<usize>().unwrap_or(0)).map(|i| {
                        container(
                            column![
                                text(dynamic_objects[(4 + 8 * i) + 6].clone()), //question
                                splitter(Color::TRANSPARENT, 15.0, 0.0, 0.0),
                                // image(format!("src/assets/q{}.png", (i + 1)))
                                //     .width(Length::Fixed(300.0)),
                                splitter(Color::TRANSPARENT, 15.0, 0.0, 0.0),
                                create_container_option!(0, i),
                                splitter(Color::TRANSPARENT, 15.0, 0.0, 0.0),
                                create_container_option!(1, i),
                                splitter(Color::TRANSPARENT, 15.0, 0.0, 0.0),
                                create_container_option!(2, i),
                                splitter(Color::TRANSPARENT, 15.0, 0.0, 0.0),
                                create_container_option!(3, i),
                                splitter(Color::TRANSPARENT, 15.0, 0.0, 0.0),
                                text(format!(
                                    "Description: {}",
                                    dynamic_objects[(4 + 8 * i) + 7].clone()
                                )),
                                splitter(Color::parse("#665c54").unwrap(), 2.0, 7.0, 7.0),
                            ]
                            .align_x(Alignment::Center),
                        )
                        .padding(5)
                        .into()
                    }),
                )
                .collect::<Vec<_>>(),
        )
        .align_x(Alignment::Center),
    ));

    let main = container(
        column![
            text(dynamic_objects[id + 6].clone()),
            splitter(Color::TRANSPARENT, 15.0, 0.0, 0.0),
            // image(format!(
            //     "src/assets/q{}.png",
            //     (id_val.parse::<usize>().unwrap() + 1)
            // ))
            // .width(Length::Fixed(300.0)),
            splitter(Color::TRANSPARENT, 15.0, 0.0, 0.0),
            create_button!(0),
            splitter(Color::TRANSPARENT, 15.0, 0.0, 0.0),
            create_button!(1),
            splitter(Color::TRANSPARENT, 15.0, 0.0, 0.0),
            create_button!(2),
            splitter(Color::TRANSPARENT, 15.0, 0.0, 0.0),
            create_button!(3),
            splitter(Color::TRANSPARENT, 15.0, 0.0, 0.0),
            button("Next")
                .on_press({
                    let check_empty = do5.clone();
                    let d1 = do1.clone();
                    let d2 = do2.clone();
                    let target_id = id + 5;
                    let next_id = id + 4;

                    // Return concrete Message instead of closure
                    if check_empty.is_empty() {
                        Message::ContentChanged(0, "Quest option has to be picked".to_string())
                    } else if d1.parse::<usize>().unwrap() + 1 == d2.parse::<usize>().unwrap() {
                        Message::FinishQuiz
                    } else {
                        Message::ContentChanged(1, (d1.parse::<usize>().unwrap() + 1).to_string())
                    }
                })
                .style(|_, status| iced::widget::button::Style {
                    background: {
                        match status {
                            iced::widget::button::Status::Pressed => {
                                Some(Color::parse("#fe8019").unwrap().into())
                            }
                            iced::widget::button::Status::Hovered => {
                                Some(Color::parse("#af3a03").unwrap().into())
                            }
                            iced::widget::button::Status::Active => {
                                Some(Color::parse("#d65d0e").unwrap().into())
                            }
                            _ => Some(Color::parse("#A1A1A1").unwrap().into()),
                        }
                    },
                    text_color: Color::parse("#1d2021").unwrap().into(),
                    border: {
                        iced::Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: 10.0.into(),
                        }
                    },
                    ..Default::default()
                })
                .width(Length::Fixed(150.0))
        ]
        .align_x(Alignment::Center),
    );

    // For overlay - clone needed value early
    let overlay_text = dynamic_objects[0].clone();
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
        container(if dynamic_objects[3] == "1" {
            main
        } else {
            results
        })
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
