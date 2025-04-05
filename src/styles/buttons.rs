use iced::Color;

pub fn email_folder_button(
    status: iced::widget::button::Status,
    hinted: bool,
    custom_status: bool,
) -> iced::widget::button::Style {
    iced::widget::button::Style {
        background: {
            match status {
                iced::widget::button::Status::Pressed => {
                    Some(Color::parse("#ebdbb2").unwrap().into())
                }
                iced::widget::button::Status::Hovered => {
                    Some(Color::parse("#928374").unwrap().into())
                }
                iced::widget::button::Status::Active => {
                    if custom_status {
                        Some(Color::parse("#a89984").unwrap().into())
                    } else {
                        Some(iced::Background::Color(Color::TRANSPARENT))
                    }
                }
                _ => Some(Color::parse("#A1A1A1").unwrap().into()),
            }
        },
        text_color: {
            if custom_status {
                Color::parse("#1d2021").unwrap().into()
            } else {
                Color::parse("#ebdbb2").unwrap().into()
            }
        },
        border: {
            if hinted {
                crate::styles::borders::hint_border_button()
            } else {
                iced::Border::default()
            }
        },
        ..Default::default()
    }
}

pub fn new_email_button(
    status: iced::widget::button::Status,
    hinted: bool,
) -> iced::widget::button::Style {
    iced::widget::button::Style {
        background: {
            match status {
                iced::widget::button::Status::Pressed => {
                    Some(Color::parse("#ebdbb2").unwrap().into())
                }
                iced::widget::button::Status::Hovered => {
                    Some(Color::parse("#928374").unwrap().into())
                }
                iced::widget::button::Status::Active => {
                    Some(Color::parse("#a89984").unwrap().into())
                }
                _ => Some(Color::parse("#A1A1A1").unwrap().into()),
            }
        },
        text_color: Color::parse("#1d2021").unwrap().into(),
        border: {
            if hinted {
                crate::styles::borders::hint_border_button()
            } else {
                iced::Border::default()
            }
        },
        ..Default::default()
    }
}
