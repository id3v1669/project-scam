use iced::{Element, Executor, Settings, Task, Theme, application::Application};

use crate::objects::game_data::{
    AchievementStatus, EmailQuestItem, EmailQuestLocation, EmailQuestSubLocation,
    FillerItemLocation, GameData, HintData, LocationData, MessageQuestItem, MessageQuestLocation,
    SubLocationData,
};
use crate::views::{levels_menu, mailbox, main_menu, message_quest};

#[derive(Debug, Clone)]
pub enum Message {
    QuestButtonClicked,
    SendEmail,
    ContentChanged(usize, String),
    Hint,
    MainMenu,
    NewGame,
    SwitchView(CurrentView),
    ClosePopup,
    ShowHint,
    ChangeMailboxFolderLocation(EmailQuestLocation),
    ChangeMailboxFolderSublocation(EmailQuestSubLocation),
    SwapText(usize, usize),
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

pub struct ConQuest {
    current_view: CurrentView,
}

impl Default for ConQuest {
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

impl ConQuest {
    fn new(_flags: ()) -> (Self, Task<Message>) {
        (Self::default(), Task::none())
    }

    fn title(&self) -> String {
        String::from("ConQuest")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::QuestButtonClicked => {
                let mut game_data = crate::objects::game_data::GAMEDATA.lock().unwrap();
                match &mut self.current_view {
                    CurrentView::MessageQuest(state) => {
                        state.dynamic_objects[0] = "Congratualtions, quest is complete.\nSome scamers use messages just to check if phone is active and even replying to them can make you a target for further attacs".to_string();
                    }
                    _ => {}
                }
            }
            Message::SendEmail => match &mut self.current_view {
                CurrentView::Mailbox(state) => {
                    state.dynamic_objects[0] = "Email sent".to_string();
                    state.dynamic_objects[5] = "".to_string();
                    state.dynamic_objects[6] = "".to_string();
                    state.dynamic_objects[7] = "".to_string();
                }
                _ => {}
            },
            Message::ContentChanged(index, content) => match &mut self.current_view {
                CurrentView::Mailbox(state) => {
                    state.dynamic_objects[index] = content;
                }
                CurrentView::MessageQuest(state) => {
                    state.dynamic_objects[index] = content;
                }
                _ => {}
            },
            Message::Hint => {
                let mut game_data = crate::objects::game_data::GAMEDATA.lock().unwrap();
                match &mut self.current_view {
                    CurrentView::Mailbox(state) => {
                        let hint = game_data.email_quest.hint();
                        //state.hinted = hint;
                        // left for now for debugging, won't be used in the final version as level switches on quests completion
                        if hint == EmailQuestItem::None {
                            state.dynamic_objects[0] = "No hint available".to_string();
                        } else {
                            state.hinted = hint;
                        }
                    }
                    CurrentView::MessageQuest(state) => {
                        let hint = game_data.message_quest.hint();
                        //state.hinted = hint;
                        // left for now for debugging, won't be used in the final version as level switches on quests completion
                        if hint == MessageQuestItem::None {
                            state.dynamic_objects[0] = "No hint available".to_string();
                        } else {
                            state.hinted = hint;
                        }
                    }
                    _ => {}
                }
            }
            Message::MainMenu => {
                self.current_view = CurrentView::MainMenu(ViewState {
                    location: FillerItemLocation::None,
                    sublocation: FillerItemLocation::None,
                    hinted: FillerItemLocation::None,
                    dynamic_objects: vec![],
                });
            }
            Message::NewGame => {
                let mut game_data = crate::objects::game_data::GAMEDATA.lock().unwrap();
                *game_data = GameData::new();
                self.current_view =
                    crate::iced_launch::CurrentView::Mailbox(crate::iced_launch::ViewState {
                        location: EmailQuestLocation::Inbox,
                        sublocation: EmailQuestSubLocation::None,
                        hinted: EmailQuestItem::None,
                        dynamic_objects: crate::objects::game_data::DYNAMIC_OBJECTS_MAILBOX
                            .to_vec(),
                    });
            }
            Message::SwitchView(view_type) => {
                self.current_view = view_type;
            }
            Message::ClosePopup => {
                match &mut self.current_view {
                    CurrentView::Mailbox(state) => state.dynamic_objects[0] = "".to_string(),
                    CurrentView::MessageQuest(state) => state.dynamic_objects[0] = "".to_string(),
                    _ => {}
                }
            }
            Message::ShowHint => match &mut self.current_view {
                CurrentView::Mailbox(state) => {}
                _ => {}
            },
            Message::ChangeMailboxFolderLocation(location_l) => {
                let mut completion_task = None;
                match &mut self.current_view {
                    CurrentView::Mailbox(state) => {
                        match location_l {
                            EmailQuestLocation::Spam => {
                                let mut game_data =
                                    crate::objects::game_data::GAMEDATA.lock().unwrap();
                                match game_data.email_quest.spam_folder {
                                    AchievementStatus::NotAchieved => {
                                        if state.hinted != EmailQuestItem::SpamFolder {
                                            game_data.email_quest.spam_folder =
                                                AchievementStatus::Achieved;
                                        } else {
                                            game_data.email_quest.spam_folder =
                                                AchievementStatus::FoundWithHint;
                                        }
                                        state.dynamic_objects[0] = "Your email most likely has some rules by default to protect you from spam and malicious emails. Be aware of those and open the spam folder only if necessary.".to_string();
                                        let _ = game_data.save();
                                        if game_data.email_quest.completed()
                                            == game_data.email_quest.iter().count() as i32
                                        {
                                            completion_task = Some(Message::SwitchView(
                                                CurrentView::MessageQuest(ViewState {
                                                    location: MessageQuestLocation::Messages,
                                                    sublocation: FillerItemLocation::None,
                                                    hinted: MessageQuestItem::None,
                                                    dynamic_objects: state.dynamic_objects.clone(),
                                                }),
                                            ));
                                        }
                                    }
                                    _ => {}
                                };
                            }
                            _ => {}
                        }
                        state.location = location_l;
                        state.sublocation = EmailQuestSubLocation::None;
                    }
                    _ => {}
                }
                return Task::done(completion_task.unwrap_or(Message::Empty));
            }
            Message::ChangeMailboxFolderSublocation(location_l) => {
                let mut completion_task = None;
                match &mut self.current_view {
                    CurrentView::Mailbox(state) => {
                        match location_l {
                            EmailQuestSubLocation::Spam => {
                                let mut game_data =
                                    crate::objects::game_data::GAMEDATA.lock().unwrap();
                                match game_data.email_quest.spam_email {
                                    AchievementStatus::NotAchieved => {
                                        if state.hinted != EmailQuestItem::SpamEmail {
                                            game_data.email_quest.spam_email =
                                                AchievementStatus::Achieved;
                                        } else {
                                            game_data.email_quest.spam_email =
                                                AchievementStatus::FoundWithHint;
                                        }
                                        state.dynamic_objects[0] = "Even opening spam emails is dangerous, as there were cases of email clients being compromised, and opening such emails could use a vulnerability for further attack.".to_string();
                                        let _ = game_data.save();
                                        if game_data.email_quest.completed()
                                            == game_data.email_quest.iter().count() as i32
                                        {
                                            completion_task = Some(Message::SwitchView(
                                                CurrentView::MessageQuest(ViewState {
                                                    location: MessageQuestLocation::Messages,
                                                    sublocation: FillerItemLocation::None,
                                                    hinted: MessageQuestItem::None,
                                                    dynamic_objects: state.dynamic_objects.clone(),
                                                }),
                                            ));
                                        }
                                    }
                                    _ => {}
                                };
                            }
                            _ => {}
                        }
                        state.sublocation = location_l;
                    }
                    _ => return Task::none(),
                }
                return Task::done(completion_task.unwrap_or(Message::Empty));
            }
            Message::SwapText(indexfrom, indexto) => {
                //let task =
                match &mut self.current_view {
                    CurrentView::Mailbox(state) => {
                        let mut completion_task = None;
                        let mut game_data = crate::objects::game_data::GAMEDATA.lock().unwrap();
                        match game_data.email_quest.email_sender {
                            AchievementStatus::NotAchieved => {
                                if state.hinted != EmailQuestItem::EmailSender {
                                    game_data.email_quest.email_sender =
                                        AchievementStatus::Achieved;
                                } else {
                                    game_data.email_quest.email_sender =
                                        AchievementStatus::FoundWithHint;
                                }
                                state.dynamic_objects[0] = "Double checking email sender is a good practice and a nececity for security-related emails. Behind decorator \"Google\" fake email with \"gooble.com\" instead of \"google.com\" was hiding.".to_string();
                                let _ = game_data.save();
                                if game_data.email_quest.completed()
                                    == game_data.email_quest.iter().count() as i32
                                {
                                    completion_task = Some(Message::SwitchView(
                                        CurrentView::MessageQuest(ViewState {
                                            location: MessageQuestLocation::Messages,
                                            sublocation: FillerItemLocation::None,
                                            hinted: MessageQuestItem::None,
                                            dynamic_objects: state.dynamic_objects.clone(),
                                        }),
                                    ));
                                }
                            }
                            _ => {}
                        };
                        let temp = state.dynamic_objects[indexfrom].clone();
                        state.dynamic_objects[indexfrom] = state.dynamic_objects[indexto].clone();
                        state.dynamic_objects[indexto] = temp;
                        if let Some(task) = completion_task {
                            return Task::done(task);
                        } else {
                            return Task::none();
                        }
                    }
                    CurrentView::MessageQuest(state) => {
                        let temp = state.dynamic_objects[indexfrom].clone();
                        state.dynamic_objects[indexfrom] = state.dynamic_objects[indexto].clone();
                        state.dynamic_objects[indexto] = temp;
                        return Task::none();
                    }
                    _ => return Task::none(),
                };
            }
            Message::Empty => return Task::none(),
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        match &self.current_view {
            CurrentView::MainMenu(state) => main_menu::view(),
            CurrentView::LevelsMenu(state) => levels_menu::view(),
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
    iced::application("Somename1", ConQuest::update, ConQuest::view)
        .antialiasing(true)
        .run()?;
    Ok(())
}
