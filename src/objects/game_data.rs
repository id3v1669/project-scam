use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

pub static GAMEDATA: Lazy<Mutex<GameData>> = Lazy::new(|| Mutex::new(GameData::load()));

pub static DYNAMIC_OBJECTS_MAILBOX: Lazy<Vec<String>> = Lazy::new(|| {
    vec![
    "Welcome to ConQuest. To achieve maximum ammount of stars, find all hidden popup messages without use of hints.".to_string(),
    "Google".to_string(),
    "support@gooble.com".to_string(),
    "Trixie".to_string(),
    "trixie@domail.com".to_string(),
    "".to_string(),
    "".to_string(),
    "".to_string(),
    "John Doe".to_string(),
    "john_doe@domain.com".to_string(),
    "Jane Doe".to_string(),
    "jane_doe@domain.com".to_string(),
]
});

#[derive(Debug, Serialize, Deserialize)]
pub struct GameData {
    pub email_quest: EmailQuest,
    pub message_quest: MessageQuest,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            email_quest: EmailQuest::default(),
            message_quest: MessageQuest::default(),
        }
    }
}
impl GameData {
    pub fn new() -> Self {
        let data = Self::default();
        data.save().expect("Failed to save game data");
        data
    }
    pub fn total_stars(&self) -> f32 {
        let mut total_stars = 0.0;
        total_stars += self.email_quest.stars();
        total_stars += self.message_quest.stars();
        total_stars
    }
    pub fn total_quests(&self) -> i32 {
        let mut total_quests = 0;
        total_quests += self.email_quest.iter().count() as i32;
        total_quests += self.message_quest.iter().count() as i32;
        total_quests
    }
    pub fn total_completed_quests(&self) -> i32 {
        let mut completed_quests = 0;
        completed_quests += self.email_quest.completed();
        completed_quests += self.message_quest.completed();
        completed_quests
    }
    pub fn rm(&self) -> std::io::Result<()> {
        // rm save file
        let directories = directories::ProjectDirs::from("com", "id3v1669", "ProjectScam")
            .expect("Failed to get directories");
        let file_path = directories.config_dir().join("game_data.json");
        match std::fs::remove_file(file_path) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Error removing file: {}", e);
                std::process::exit(1);
            }
        }
    }
    pub fn save(&self) -> std::io::Result<()> {
        let directories = directories::ProjectDirs::from("com", "id3v1669", "ProjectScam")
            .expect("Failed to get directories");

        match std::fs::create_dir_all(directories.config_dir()) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error creating directory: {}", e);
                std::process::exit(1);
            }
        }
        let file_path = directories.config_dir().join("game_data.json");
        let file = serde_json::to_string_pretty(self)?;
        match std::fs::write(file_path, file) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error writing to file: {}", e);
                std::process::exit(1);
            }
        }
        Ok(())
    }
    pub fn load() -> Self {
        let directories = directories::ProjectDirs::from("com", "id3v1669", "ProjectScam")
            .expect("Failed to get directories");

        let file_path = directories.config_dir().join("game_data.json");

        match std::fs::read_to_string(file_path) {
            Ok(file) => match serde_json::from_str(&file) {
                Ok(data) => {
                    println!("Loaded game data: {:?}", data);
                    data
                }
                Err(_) => {
                    println!("Failed to parse game data, creating new one");
                    Self::new()
                }
            },
            Err(_) => Self::new(),
        }
    }
}

pub trait HintData: std::fmt::Debug + Default + Clone {}

impl HintData for EmailQuestItem {}
impl HintData for MessageQuestItem {}
impl HintData for FillerItemLocation {}

pub trait LocationData: std::fmt::Debug + Default + Clone {}

impl LocationData for EmailQuestLocation {}
impl LocationData for MessageQuestLocation {}
impl LocationData for FillerItemLocation {}

pub trait SubLocationData: std::fmt::Debug + Default + Clone {}

impl SubLocationData for EmailQuestSubLocation {}
impl SubLocationData for FillerItemLocation {}

#[derive(Default, Debug, Clone, Copy)]
pub enum FillerItemLocation {
    #[default]
    None,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum EmailQuestItem {
    #[default]
    None,
    //NewEmail,
    EmailSender,
    //BlockedContent,
    //EmailAttachment,
    SpamFolder,
    SpamEmail,
}

#[derive(Default, Debug, Clone, PartialEq, Copy)]
pub enum EmailQuestLocation {
    #[default]
    Inbox,
    Spam,
    NewEmail,
    ReadEmail,
}

#[derive(Default, Debug, Clone, PartialEq, Copy)]
pub enum EmailQuestSubLocation {
    #[default]
    None,
    Inbox1,
    Spam,
    NewEmail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailQuest {
    //pub new_email: AchievementStatus,
    pub email_sender: AchievementStatus,
    //pub blocked_content: AchievementStatus,
    //pub email_attachment: AchievementStatus,
    pub spam_folder: AchievementStatus,
    pub spam_email: AchievementStatus,
}

impl Default for EmailQuest {
    fn default() -> Self {
        Self {
            //new_email: AchievementStatus::NotAchieved,
            email_sender: AchievementStatus::NotAchieved,
            //blocked_content: AchievementStatus::NotAchieved,
            //email_attachment: AchievementStatus::NotAchieved,
            spam_folder: AchievementStatus::NotAchieved,
            spam_email: AchievementStatus::NotAchieved,
        }
    }
}

impl EmailQuest {
    pub fn stars(&self) -> f32 {
        let mut total_stars = 0.0;
        for each in self.iter() {
            total_stars += match each {
                AchievementStatus::Achieved => 1.0,
                AchievementStatus::FoundWithHint => 0.5,
                AchievementStatus::NotAchieved => 0.0,
            };
        }
        total_stars
    }
    pub fn completed(&self) -> i32 {
        let mut completed = 0;
        for each in self.iter() {
            if each == &AchievementStatus::Achieved || each == &AchievementStatus::FoundWithHint {
                completed += 1;
            }
        }
        completed
    }
    pub fn iter(&self) -> impl Iterator<Item = &AchievementStatus> {
        [
            //&self.new_email,
            &self.email_sender,
            //&self.blocked_content,
            //&self.email_attachment,
            &self.spam_folder,
            &self.spam_email,
        ]
        .into_iter()
    }
    pub fn hint(&self) -> EmailQuestItem {
        // if self.new_email == AchievementStatus::NotAchieved {
        //     EmailQuestItem::NewEmail
        // } else
        if self.email_sender == AchievementStatus::NotAchieved {
            EmailQuestItem::EmailSender
        // } else if self.blocked_content == AchievementStatus::NotAchieved {
        //     EmailQuestItem::BlockedContent
        // } else if self.email_attachment == AchievementStatus::NotAchieved {
        //     EmailQuestItem::EmailAttachment
        } else if self.spam_folder == AchievementStatus::NotAchieved {
            EmailQuestItem::SpamFolder
        } else if self.spam_email == AchievementStatus::NotAchieved {
            EmailQuestItem::SpamEmail
        } else {
            EmailQuestItem::None
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum MessageQuestItem {
    #[default]
    None,
    Manipulation,
}

#[derive(Default, Debug, Clone, Copy)]
pub enum MessageQuestLocation {
    #[default]
    Messages,
    ManipulationMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageQuest {
    pub money_ask: AchievementStatus,
}

impl Default for MessageQuest {
    fn default() -> Self {
        Self {
            money_ask: AchievementStatus::NotAchieved,
        }
    }
}
impl MessageQuest {
    pub fn hint(&self) -> MessageQuestItem {
        if self.money_ask == AchievementStatus::NotAchieved {
            MessageQuestItem::Manipulation
        } else {
            MessageQuestItem::None
        }
    }
    pub fn stars(&self) -> f32 {
        let mut total_stars = 0.0;
        for each in self.iter() {
            total_stars += match each {
                AchievementStatus::Achieved => 1.0,
                AchievementStatus::FoundWithHint => 0.5,
                AchievementStatus::NotAchieved => 0.0,
            };
        }
        total_stars
    }
    pub fn completed(&self) -> i32 {
        let mut completed = 0;
        for each in self.iter() {
            if each == &AchievementStatus::Achieved || each == &AchievementStatus::FoundWithHint {
                completed += 1;
            }
        }
        completed
    }
    pub fn iter(&self) -> impl Iterator<Item = &AchievementStatus> {
        [&self.money_ask].into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AchievementStatus {
    Achieved,
    FoundWithHint,
    NotAchieved,
}
