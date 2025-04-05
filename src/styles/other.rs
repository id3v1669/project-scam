use iced::Color;

pub fn empty_email() -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: iced::Color::parse("#ebdbb2").unwrap().into(),
        border: iced::Border {
            color: Color::parse("#505050").unwrap().into(),
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}
