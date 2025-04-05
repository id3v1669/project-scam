use iced::{Element, Executor, Settings, Task, Theme, application::Application};

use crate::views::{level1, levels_menu, main_menu};

#[derive(Debug, Clone)]
pub enum Message {
    SwitchView(CurrentView),
    ClosePopup,
    Empty,
}

#[derive(Debug, Default, Clone)]
pub struct ViewState {
    pub show_popup: bool,
    pub popup_message: String,
}

#[derive(Debug, Clone)]
pub enum CurrentView {
    MainMenu(ViewState),
    LevelsMenu(ViewState),
    Level1(ViewState),
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
                    CurrentView::Level1(state) => state.show_popup = false,
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
            CurrentView::Level1(state) => level1::view(state.show_popup, &state.popup_message, 1),
        }
    }
}

pub fn custom_start() -> Result<(), Box<dyn std::error::Error>> {
    iced::application("Somename1", MyApp::update, MyApp::view)
        .antialiasing(true)
        .run()?;
    Ok(())
}
