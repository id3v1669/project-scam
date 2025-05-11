use crate::iced_launch::Message;
use crate::objects::game_data::AchievementStatus;
use crate::objects::game_data::{
    EmailQuestItem, EmailQuestLocation, EmailQuestSubLocation, FillerItemLocation, GameData,
    MessageQuestItem, MessageQuestLocation,
};
use crate::styles::buttons::regular_rounded_no_border;
use iced::Element;
use iced::widget::{button, column, container, row, text};

pub fn view() -> Element<'static, Message> {
    container(
        column![
            text("Pick a level to play"),
            crate::objects::custom_gui::splitter(iced::Color::TRANSPARENT, 15.0, 0.0, 0.0),
            button("Mailbox Quest")
                .on_press(
                    Message::SwitchView(
                        crate::iced_launch::CurrentView::Mailbox(
                            crate::iced_launch::ViewState {
                                location: EmailQuestLocation::Inbox,
                                sublocation: EmailQuestSubLocation::None,
                                hinted: EmailQuestItem::None,
                                dynamic_objects: {
                                    let mut dynamic_objects =
                                        crate::objects::game_data::DYNAMIC_OBJECTS_MAILBOX.to_vec();
                                    dynamic_objects[0] = "".to_string();
                                    dynamic_objects
                                },
                            }
                        ))
                )
                .style(|_, status| regular_rounded_no_border(status, false))
                .width(iced::Length::Fixed(150.0)),
            crate::objects::custom_gui::splitter(iced::Color::TRANSPARENT, 15.0, 0.0, 0.0),
            button("Message Quest")
                .on_press(Message::SwitchView(
                    crate::iced_launch::CurrentView::MessageQuest(crate::iced_launch::ViewState {
                        location: MessageQuestLocation::Messages,
                        sublocation: FillerItemLocation::None,
                        hinted: MessageQuestItem::None,
                        dynamic_objects: {
                            let mut dynamic_objects =
                                crate::objects::game_data::DYNAMIC_OBJECTS_MAILBOX.to_vec();
                            dynamic_objects[0] = "".to_string();
                            dynamic_objects
                        },
                    })
                ))
                .style(|_, status| regular_rounded_no_border(status, false))
                .width(iced::Length::Fixed(150.0)),
        ]
        .align_x(iced::Alignment::Center),
    )
    .style(|_| iced::widget::container::Style {
        text_color: iced::Color::parse("#ebdbb2").unwrap().into(),
        background: Some(iced::Color::parse("#282828").unwrap().into()),
        ..Default::default()
    })
    .width(iced::Length::Fill)
    .height(iced::Length::Fill)
    .align_y(iced::Alignment::Center)
    .into()
}
