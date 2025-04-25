use iced::Element;
use iced::widget::text;

pub fn view(
    location: crate::objects::game_data::MessageQuestLocation,
    sublocation: crate::objects::game_data::FillerItemLocation,
    hinted: crate::objects::game_data::MessageQuestItem,
    dynamic_objects: Vec<String>,
) -> Element<'static, crate::iced_launch::Message> {
    text("This is message_quest").into()
}
