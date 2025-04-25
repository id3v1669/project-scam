use iced::Element;
use iced::widget::text;

pub fn view(
) -> Element<'static, crate::iced_launch::Message> {
    text("This is View 2").into()
}
