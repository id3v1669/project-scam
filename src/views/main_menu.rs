use crate::iced_launch::Message;
use iced::Element;
use iced::widget::{button, column, container, text};

pub fn view(show_popup: bool, popup_message: &String) -> Element<'static, Message> {
    container(column![
        text("Welcome to Scam Gamification Game"),
        button("Start").on_press(Message::SwitchView(
            crate::iced_launch::CurrentView::Level1(crate::iced_launch::ViewState {
                show_popup: true,
                popup_message: "Welcome to Level 1!".to_string(),
            })
        )),
        // button("Levels").on_press(Message::SwitchView(
        //     crate::iced_launch::ViewType::Level1 (
        //         crate::iced_launch::ViewState {
        //             show_popup: true,
        //             popup_message: "Welcome to Level 1!".to_string(),
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
