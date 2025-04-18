use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

pub static GAMEDATA: Lazy<Mutex<GameData>> = Lazy::new(|| Mutex::new(GameData::load()));

#[derive(Serialize, Deserialize)]
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
    fn total_stars(&self) -> f32 {
        let mut total_stars = 0.0;
        total_stars += self.email_quest.stars();
        total_stars += self.message_quest.stars();
        total_stars
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
            Ok(file) => serde_json::from_str(&file).unwrap_or_else(|_| Self::new()),
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

#[derive(Default, Debug, Clone)]
pub enum FillerItemLocation {
    #[default]
    None,
}

#[derive(Default, Debug, Clone, Copy)]
pub enum EmailQuestItem {
    #[default]
    None,
    NewEmail,
    EmailSender,
    BlockedContent,
    EmailAttachment,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailQuest {
    pub new_email: AchievementStatus,
    pub email_sender: AchievementStatus,
    pub blocked_content: AchievementStatus,
    pub email_attachment: AchievementStatus,
    pub spam_folder: AchievementStatus,
    pub spam_email: AchievementStatus,
}

impl Default for EmailQuest {
    fn default() -> Self {
        Self {
            new_email: AchievementStatus::NotAchieved,
            email_sender: AchievementStatus::NotAchieved,
            blocked_content: AchievementStatus::NotAchieved,
            email_attachment: AchievementStatus::NotAchieved,
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
            &self.new_email,
            &self.email_sender,
            &self.blocked_content,
            &self.email_attachment,
            &self.spam_folder,
            &self.spam_email,
        ]
        .into_iter()
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum MessageQuestItem {
    #[default]
    None,
    MoneyAsk,
}

#[derive(Default, Debug, Clone, Copy)]
pub enum MessageQuestLocation {
    #[default]
    Messages,
    MoneyAskMessage,
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
