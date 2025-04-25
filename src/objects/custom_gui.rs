use iced::Length;
use iced::widget::{column, container, text};

pub fn splitter(
    color: iced::Color,
    width: f32,
    margin_top: f32,
    margin_bottom: f32,
) -> iced::Element<'static, crate::iced_launch::Message> {
    column![
        container(text(""))
            .width(Length::Fill)
            .height(Length::Fixed(margin_top)),
        container(text(""))
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(color)),
                ..Default::default()
            })
            .width(Length::Fill)
            .height(Length::Fixed(width)),
        container(text(""))
            .width(Length::Fill)
            .height(Length::Fixed(margin_bottom))
    ]
    .into()
}
