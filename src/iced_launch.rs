use iced::{Element, Executor, Settings, Task, Theme, application::Application};

use crate::objects::game_data::{
    EmailQuestItem, EmailQuestLocation, FillerItemLocation, MessageQuestItem, MessageQuestLocation, LocationData, HintData,
};
use crate::views::{levels_menu, mailbox, main_menu};

#[derive(Debug, Clone)]
pub enum Message {
    SwitchView(CurrentView),
    ClosePopup,
    ShowHint,
    ChangeMailboxFolderLocation(EmailQuestLocation),
    Empty,
}

#[derive(Debug, Default, Clone)]
pub struct ViewState<
    T1: HintData,
    T2: LocationData,
> {
    pub show_popup: bool,
    pub popup_message: String,
    pub location: T2,
    pub hinted: T1,
}

type MainMenuViewState = ViewState<FillerItemLocation, FillerItemLocation>;
type LevelsMenuViewState = ViewState<FillerItemLocation, FillerItemLocation>;
type EmailQuestViewState = ViewState<EmailQuestItem, EmailQuestLocation>;
type MessageQuestViewState = ViewState<MessageQuestItem, MessageQuestLocation>;

#[derive(Debug, Clone)]
pub enum CurrentView {
    MainMenu(MainMenuViewState),
    LevelsMenu(LevelsMenuViewState),
    Mailbox(EmailQuestViewState),
    MessageQuest(MessageQuestViewState),
}

pub struct MyApp {
    current_view: CurrentView,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            current_view: CurrentView::MainMenu(ViewState {
                show_popup: true,
                popup_message: "Welcome!".to_string(),
                location: FillerItemLocation::None,
                hinted: FillerItemLocation::None,
            }),
        }
    }
}

impl MyApp {
    fn new(_flags: ()) -> (Self, Task<Message>) {
        (Self::default(), Task::none())
    }

    fn title(&self) -> String {
        String::from("My App")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SwitchView(view_type) => {
                self.current_view = view_type;
            }
            Message::ClosePopup => {
                // Handle popup closing for any view
                match &mut self.current_view {
                    CurrentView::MainMenu(state) => state.show_popup = false,
                    CurrentView::LevelsMenu(state) => state.show_popup = false,
                    CurrentView::Mailbox(state) => state.show_popup = false,
                    CurrentView::MessageQuest(state) => state.show_popup = false,
                }
            }
            Message::ShowHint => match &mut self.current_view {
                CurrentView::Mailbox(state) => {}
                _ => {}
            },
            Message::ChangeMailboxFolderLocation(location_l) => {
                println!("Changing mailbox folder location to {:?}", location_l);
                match &mut self.current_view {
                    CurrentView::Mailbox(state) => {
                        println!("inside match mailbox");
                        state.location = location_l;
                    },
                    _ => {}
                }
            },
            Message::Empty => {}
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        match &self.current_view {
            CurrentView::MainMenu(state) => main_menu::view(state.show_popup, &state.popup_message),
            CurrentView::LevelsMenu(state) => {
                levels_menu::view(state.show_popup, &state.popup_message)
            }
            CurrentView::Mailbox(state) => mailbox::view(
                state.show_popup,
                &state.popup_message,
                crate::objects::game_data::EmailQuestLocation::Inbox,
                crate::objects::game_data::EmailQuestItem::None,
            ),
            CurrentView::MessageQuest(state) => mailbox::view(
                state.show_popup,
                &state.popup_message,
                crate::objects::game_data::EmailQuestLocation::Inbox,
                crate::objects::game_data::EmailQuestItem::None,
            )
        }
    }
}

pub fn custom_start() -> Result<(), Box<dyn std::error::Error>> {
    iced::application("Somename1", MyApp::update, MyApp::view)
        .antialiasing(true)
        .run()?;
    Ok(())
}
