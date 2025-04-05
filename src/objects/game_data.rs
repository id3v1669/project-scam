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
        Self::default()
    }
    fn total_stars(&self) -> f32 {
        let mut total_stars = 0.0;
        total_stars += self.email_quest.stars();
        total_stars += self.message_quest.stars();
        total_stars
    }
}

#[derive(Debug, Clone)]
pub struct EmailQuest {
    pub new_email: AchievementStatus,
    pub email_sender: AchievementStatus,
    pub blocked_content: AchievementStatus,
    pub email_attachment: AchievementStatus,
    pub spam_email: AchievementStatus,
}

impl Default for EmailQuest {
    fn default() -> Self {
        Self {
            new_email: AchievementStatus::NotAchieved,
            email_sender: AchievementStatus::NotAchieved,
            blocked_content: AchievementStatus::NotAchieved,
            email_attachment: AchievementStatus::NotAchieved,
            spam_email: AchievementStatus::NotAchieved,
        }
    }
}
impl EmailQuest {
    pub fn new() -> Self {
        Self::default()
    }
    fn stars(&self) -> f32 {
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
    pub fn iter(&self) -> impl Iterator<Item = &AchievementStatus> {
        [
            &self.new_email,
            &self.email_sender,
            &self.blocked_content,
            &self.email_attachment,
            &self.spam_email,
        ]
        .into_iter()
    }
}

#[derive(Debug, Clone)]
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
    pub fn new() -> Self {
        Self::default()
    }
    fn stars(&self) -> f32 {
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
    pub fn iter(&self) -> impl Iterator<Item = &AchievementStatus> {
        [&self.money_ask].into_iter()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AchievementStatus {
    Achieved,
    FoundWithHint,
    NotAchieved,
}
