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

pub fn email_button(
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
                crate::styles::borders::email_border()
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

pub fn regular_rounded_no_border(
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
                iced::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 10.0.into(),
                }
            }
        },
        ..Default::default()
    }
}

pub fn regular_rounded_border_as_margin(
    status: iced::widget::button::Status,
    hinted: bool,
    margin: f32,
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
                iced::Border {
                    color: Color::TRANSPARENT,
                    width: margin,
                    radius: 10.0.into(),
                }
            }
        },
        ..Default::default()
    }
}

pub fn danger_button_style(status: iced::widget::button::Status) -> iced::widget::button::Style {
    iced::widget::button::Style {
        background: {
            match status {
                iced::widget::button::Status::Pressed => {
                    Some(Color::parse("#cc241d90").unwrap().into())
                }
                iced::widget::button::Status::Hovered => {
                    Some(Color::parse("#cc241d30").unwrap().into())
                }
                iced::widget::button::Status::Active => {
                    Some(Color::parse("#282828").unwrap().into())
                }
                _ => Some(Color::parse("#A1A1A1").unwrap().into()),
            }
        },
        text_color: Color::parse("#fb4934").unwrap().into(),
        border: iced::Border {
            color: Color::parse("#fb4934").unwrap().into(),
            width: 1.0,
            radius: 12.0.into(),
        },
        ..Default::default()
    }
}
