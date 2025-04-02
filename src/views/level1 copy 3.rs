use iced::widget::text;
use iced::Element;

pub fn view() -> Element<'static, crate::iced_launch::Message> {
    text("This is View 3").into()
}