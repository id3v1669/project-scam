use crate::iced_launch::Message;
use crate::objects::game_data::{EmailQuestItem, EmailQuestLocation, EmailQuestSubLocation};
use iced::Element;
use iced::widget::{button, column, container, text};

pub fn view() -> Element<'static, Message> {
    container(column![
        text("Welcome to Scam Gamification Game"),
        button("Start").on_press(Message::SwitchView(
            crate::iced_launch::CurrentView::Mailbox(crate::iced_launch::ViewState {
                location: EmailQuestLocation::Inbox,
                sublocation: EmailQuestSubLocation::None,
                hinted: EmailQuestItem::None,
                dynamic_objects: vec![
                    "First task would be to write an email. \nButtons not related to quiz but required to press will have green border. \nTo begin, close this window and press New email button.".to_string(),
                ],
            })
        )),
        // button("Levels").on_press(Message::SwitchView(
        //     crate::iced_launch::ViewType::Level1 (
        //         crate::iced_launch::ViewState {
        //         }
        //     )
        // )),
    ])
    .style(|_| iced::widget::container::Style {
        background: Some(iced::Color::parse("#282828").unwrap().into()),
        ..Default::default()
    })
    .width(iced::Length::Fill)
    .height(iced::Length::Fill)
    .into()
}
