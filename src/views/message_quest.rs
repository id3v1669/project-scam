use iced::widget::text;
use iced::Element;

pub fn view(
    show_popup: bool,
    popup_message: &String,
    location: crate::objects::game_data::MessageQuestLocation,
    hinted: crate::objects::game_data::MessageQuestItem,
) -> Element<'static, crate::iced_launch::Message> {
    text("This is message_quest").into()
}