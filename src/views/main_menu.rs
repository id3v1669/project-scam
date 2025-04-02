use iced::widget::{button, column, text};
use iced::Element;
use crate::iced_launch::Message;

pub fn view() -> Element<'static, Message> {
    column![
        text("Welcome to Scam Gamification Game"),
        button("Start").on_press(Message::Level1),
        button("Levels").on_press(Message::LevelsMenu),
    ].into()
}