use iced::{Element, Executor, Settings, Task, Theme, application::Application};

use crate::objects::game_data::{
    EmailQuestItem, EmailQuestLocation, EmailQuestSubLocation, FillerItemLocation, HintData,
    LocationData, MessageQuestItem, MessageQuestLocation, SubLocationData,
};
use crate::views::{levels_menu, mailbox, main_menu, message_quest};

#[derive(Debug, Clone)]
pub enum Message {
    SwitchView(CurrentView),
    ClosePopup,
    ShowHint,
    ChangeMailboxFolderLocation(EmailQuestLocation),
    ChangeMailboxFolderSublocation(EmailQuestSubLocation),
    Empty,
}

#[derive(Debug, Default, Clone)]
pub struct ViewState<T1: HintData, T2: LocationData, T3: SubLocationData> {
    pub location: T2,
    pub sublocation: T3,
    pub hinted: T1,
    // workaround to avoid adding extra data types and variables
    // first element reserved for to replace popup message text
    pub dynamic_objects: Vec<String>,
}

type MainMenuViewState = ViewState<FillerItemLocation, FillerItemLocation, FillerItemLocation>;
type LevelsMenuViewState = ViewState<FillerItemLocation, FillerItemLocation, FillerItemLocation>;
type EmailQuestViewState = ViewState<EmailQuestItem, EmailQuestLocation, EmailQuestSubLocation>;
type MessageQuestViewState = ViewState<MessageQuestItem, MessageQuestLocation, FillerItemLocation>;

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
                location: FillerItemLocation::None,
                sublocation: FillerItemLocation::None,
                hinted: FillerItemLocation::None,
                dynamic_objects: vec![],
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
                    CurrentView::MainMenu(state) => state.dynamic_objects[0] = "".to_string(),
                    CurrentView::LevelsMenu(state) => state.dynamic_objects[0] = "".to_string(),
                    CurrentView::Mailbox(state) => state.dynamic_objects[0] = "".to_string(),
                    CurrentView::MessageQuest(state) => state.dynamic_objects[0] = "".to_string(),
                }
            }
            Message::ShowHint => match &mut self.current_view {
                CurrentView::Mailbox(state) => {}
                _ => {}
            },
            Message::ChangeMailboxFolderLocation(location_l) => match &mut self.current_view {
                CurrentView::Mailbox(state) => {
                    state.location = location_l;
                    state.sublocation = EmailQuestSubLocation::None;
                }
                _ => {}
            },
            Message::ChangeMailboxFolderSublocation(location_l) => match &mut self.current_view {
                CurrentView::Mailbox(state) => {
                    state.sublocation = location_l;
                }
                _ => {}
            },
            Message::Empty => {}
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        match &self.current_view {
            CurrentView::MainMenu(state) => main_menu::view(),
            CurrentView::LevelsMenu(state) => {
                levels_menu::view()
            }
            CurrentView::Mailbox(state) => mailbox::view(
                state.location,
                state.sublocation,
                state.hinted,
                state.dynamic_objects.clone(),
            ),
            CurrentView::MessageQuest(state) => message_quest::view(
                state.location,
                state.sublocation,
                state.hinted,
                state.dynamic_objects.clone(),
            ),
        }
    }
}

pub fn custom_start() -> Result<(), Box<dyn std::error::Error>> {
    iced::application("Somename1", MyApp::update, MyApp::view)
        .antialiasing(true)
        .run()?;
    Ok(())
}
