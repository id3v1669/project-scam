use iced::{
    application::Application,
    Task, Element, Executor, Settings, Theme
};

use crate::views::{main_menu, levels_menu, level1};

#[derive(Debug, Clone)]
pub enum Message {
    MainMenu,
    LevelsMenu,
    Level1,
    //ClosePopup,
    Empty, // for testing purposes
}

pub struct MyApp {
    current_view: CurrentView,
}

#[derive(Debug, Default)]
struct ViewState {
    show_popup: bool,
    popup_message: String,
}

#[derive(Debug)]
enum CurrentView {
    MainMenu(ViewState),
    LevelsMenu(ViewState),
    Level1(ViewState),
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            current_view: CurrentView::MainMenu(
                ViewState {
                    show_popup: false,
                    popup_message: "This is an overlay".to_string(),
                },
            ),
        }
    }
}

impl MyApp {
    fn new() -> Self {
        MyApp {
            current_view: CurrentView::MainMenu(
                ViewState {
                    show_popup: false,
                    popup_message: "This is an overlay".to_string(),
                },
            ),
        }
    }

    fn title(&self) -> String {
        String::from("Somename2")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::MainMenu => self.current_view = CurrentView::MainMenu(
                ViewState {
                    show_popup: false,
                    popup_message: "This is an overlay".to_string(),
                },
            ),
            Message::LevelsMenu => self.current_view = CurrentView::LevelsMenu(
                ViewState {
                    show_popup: false,
                    popup_message: "This is an overlay".to_string(),
                },
            ),
            Message::Level1 => {
                self.current_view = CurrentView::Level1(ViewState {
                    show_popup: true,
                    popup_message: "This is a test".to_string(),
                });
            },
            // Message::ClosePopup => {
            //     self.current_view = &mut self.current_view {
            //         state.show_popup = false;
            //     }
            // },
            Message::Empty => {} // for testing purposes
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        match self.current_view {
            CurrentView::MainMenu(_) => main_menu::view(),
            CurrentView::LevelsMenu(_) => levels_menu::view(),
            CurrentView::Level1(_) => level1::view(),
        }
    }
}

pub fn custom_start() -> Result<(), Box<dyn std::error::Error>> {
    iced::application(
        "Somename1",
        MyApp::update,
        MyApp::view,
    )
    .antialiasing(true)
    .run()?;
    Ok(())
}