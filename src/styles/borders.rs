use iced::Color;

pub fn hint_border_button() -> iced::Border {
    iced::Border {
        color: Color::parse("#b8bb26").unwrap().into(),
        width: 3.0,
        radius: 6.0.into(),
    }
}

pub fn email_border() -> iced::Border {
    iced::Border {
        color: Color::parse("#505050").unwrap().into(),
        width: 1.0,
        radius: 0.0.into(),
    }
}
