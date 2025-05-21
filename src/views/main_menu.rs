use crate::iced_launch::Message;
use crate::objects::game_data::AchievementStatus;
use crate::objects::game_data::{
    EmailQuestItem, EmailQuestLocation, EmailQuestSubLocation, FillerItemLocation, GameData,
};
use crate::styles::buttons::regular_rounded_no_border;
use iced::Element;
use iced::widget::{button, column, container, row, text};

pub fn view() -> Element<'static, Message> {
    let game_data = crate::objects::game_data::GAMEDATA.lock().unwrap();
    container(
        column![
            text("Welcome to Scam Gamification Project"),
            crate::objects::custom_gui::splitter(iced::Color::TRANSPARENT, 15.0, 0.0, 0.0),
            row![
                text(format!(
                    "Quest Stars: {}/{}",
                    game_data.total_stars(),
                    game_data.total_quests(),
                )),
                text("  |  "),
                text(format!(
                    "Quests: {}/{}",
                    game_data.total_completed_quests(),
                    game_data.total_quests(),
                )),
            ],
            crate::objects::custom_gui::splitter(iced::Color::TRANSPARENT, 15.0, 0.0, 0.0),
            button("Start Quiz Game")
                .on_press(Message::SwitchView(crate::iced_launch::CurrentView::Quiz(
                    crate::iced_launch::ViewState {
                        location: FillerItemLocation::None,
                        sublocation: FillerItemLocation::None,
                        hinted: FillerItemLocation::None,
                        dynamic_objects: {
                            let mut dynamic_objects =
                                crate::objects::game_data::DYNAMIC_OBJECTS_QUIZ.to_vec();
                            dynamic_objects[0] = "".to_string();
                            dynamic_objects
                        },
                    }
                )))
                .style(|_, status| regular_rounded_no_border(status, false))
                .width(iced::Length::Fixed(150.0)),
            crate::objects::custom_gui::splitter(iced::Color::TRANSPARENT, 15.0, 0.0, 0.0),
            button("New Hint Game")
                .on_press(Message::NewGame)
                .style(|_, status| regular_rounded_no_border(status, false))
                .width(iced::Length::Fixed(150.0)),
            crate::objects::custom_gui::splitter(iced::Color::TRANSPARENT, 15.0, 0.0, 0.0),
            button("Resume Hint Game")
                .on_press(Message::SwitchView(
                    crate::iced_launch::CurrentView::Mailbox(crate::iced_launch::ViewState {
                        location: EmailQuestLocation::Inbox,
                        sublocation: EmailQuestSubLocation::None,
                        hinted: EmailQuestItem::None,
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
            crate::objects::custom_gui::splitter(iced::Color::TRANSPARENT, 15.0, 0.0, 0.0),
            button("Hint Levels Menu")
                .on_press(Message::SwitchView(
                    crate::iced_launch::CurrentView::LevelsMenu(crate::iced_launch::ViewState {
                        location: FillerItemLocation::None,
                        sublocation: FillerItemLocation::None,
                        hinted: FillerItemLocation::None,
                        dynamic_objects: vec![],
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
