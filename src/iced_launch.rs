use iced::{Element, Executor, Settings, Task, Theme, application::Application};

use crate::views::{levels_menu, mailbox, main_menu};
use crate::objects::game_data::{MainMenuItem, LevelsMenuItem, EmailQuestItem, MessageQuestItem};

#[derive(Debug, Clone)]
pub enum Message {
    SwitchView(CurrentView),
    ClosePopup,
    ShowHint,
    Empty,
}

#[derive(Debug, Default, Clone)]
pub struct ViewState<T: crate::objects::game_data::HintData> {
    pub show_popup: bool,
    pub popup_message: String,
    pub show_hint: T,
}

type MainMenuViewState = ViewState<MainMenuItem>;
type LevelsMenuViewState = ViewState<LevelsMenuItem>;
type EmailQuestViewState = ViewState<EmailQuestItem>;
type MessageQuestViewState = ViewState<MessageQuestItem>;


#[derive(Debug, Clone)]
pub enum CurrentView {
    MainMenu(MainMenuViewState),
    LevelsMenu(LevelsMenuViewState),
    Mailbox(EmailQuestViewState),
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
                show_hint: MainMenuItem::None,
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
                }
            }
            Message::ShowHint => {
                match &mut self.current_view {
                    CurrentView::Mailbox(state) => {

                    }
                    _ => {}
                }
            }
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
            CurrentView::Mailbox(state) => mailbox::view(state.show_popup, &state.popup_message, crate::objects::game_data::EmailQuestLocation::Inbox, crate::objects::game_data::EmailQuestItem::None),
        }
    }
}

pub fn custom_start() -> Result<(), Box<dyn std::error::Error>> {
    iced::application("Somename1", MyApp::update, MyApp::view)
        .antialiasing(true)
        .run()?;
    Ok(())
}
