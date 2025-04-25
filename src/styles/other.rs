use iced::Color;

pub fn empty_email() -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: iced::Color::parse("#ebdbb2").unwrap().into(),
        border: crate::styles::borders::email_border(),
        ..Default::default()
    }
}
