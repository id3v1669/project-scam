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

pub static DYNAMIC_OBJECTS_QUIZ: Lazy<Vec<String>> = Lazy::new(|| {
    vec![
        "Welcome to ConQuest Quiz game. Secure one of four answes and press \"Next\".".to_string(),
        "0".to_string(), //current quiz
        "15".to_string(),                    //quiz ammount
        "1".to_string(), //location - quiz or result

        // quiz1
        "Click the link and reset your password".to_string(),                 //option1
        "Ignore the email".to_string(),                 //option2
        "Carefully check the sender’s email address and confirm it on the official Microsoft website".to_string(),                 //option3
        "Forward the email to friends to warn them".to_string(),                 //option4
        "3".to_string(),                   //correct options
        "".to_string(),                     //option picked
        "You get an email from “support@micr0soft.com” asking you to reset your password right away. What should you do?".to_string(), //description
        "Scammers often use similar-looking email addresses (like micr0soft.com instead of microsoft.com). Always check.".to_string(), //result
        
        // quiz2
        "Restart your computer".to_string(),                //option1
        "Close the browser or tab".to_string(),                //option2
        "Click the link quickly".to_string(),                //option3
        "Download the antivirus from the pop-up".to_string(),                //option4
        "2".to_string(),                   //correct options
        "".to_string(),                     //option picked
        "A message appears on your screen saying “Your device is infected! Click here to fix it now.” What’s the safest choice?".to_string(), //description
        "Real antivirus software doesn’t show up through random browser messages.".to_string(), //result
                                            // Here continue

        // quiz3
        "Give your name and address to get the prize".to_string(),
        "Click the link to claim it".to_string(),
        "Report the email as a scam".to_string(),
        "Reply to ask for more information".to_string(),
        "3".to_string(),
        "".to_string(),
        "An email says you’ve won a gift card and asks for your personal details to claim it. What’s the safest action?".to_string(),
        "Messages saying “You’ve won” are often scams. Never share personal information.".to_string(),

        // quiz4
        "Spelling and grammar mistakes".to_string(),
        "Urgent requests (“Act Now!”)".to_string(),
        "Strange email addresses".to_string(),
        "All of the above".to_string(),
        "4".to_string(),
        "".to_string(),
        "Which of these is a warning sign in a suspicious email?".to_string(),
        "Scam emails often have many warning signs. Be careful!".to_string(),

        // quiz5
        "Click the link".to_string(),
        "Call the bank using the number in the message".to_string(),
        "Log in through the bank’s official website, not the link".to_string(),
        "Forward it to friends".to_string(),
        "3".to_string(),
        "".to_string(),
        "Your bank sends you a message asking to verify your account through a link. What should you do first?".to_string(),
        "Always log in directly through the official website. Links in messages can be fake.".to_string(),

        // quiz6
        "Contact your bank through their official website or app".to_string(),
        "Click the link to check your account".to_string(),
        "Ignore it".to_string(),
        "Call your bank using the number in the message".to_string(),
        "1".to_string(),
        "".to_string(),
        "You get a text message saying: “We’ve noticed strange activity on your bank account. Click here to secure it.” What should you do?".to_string(),
        "Don’t trust links in unexpected texts. Use official ways to contact them.".to_string(),

        // quiz7
        "A lucky break".to_string(),
        "A scam called \"advance fee fraud\"".to_string(),
        "A common online job offer".to_string(),
        "A giveaway".to_string(),
        "2".to_string(),
        "".to_string(),
        "Someone you don’t know on social media offers to send you money if you send them a small payment first. What is this?".to_string(),
        "This is a common scam—never send money to get money back.".to_string(),

        // quiz8
        "Click to claim it".to_string(),
        "Take a screenshot and share it".to_string(),
        "Ignore it and close the tab".to_string(),
        "Enter fake information to see what happens".to_string(),
        "3".to_string(),
        "".to_string(),
        "A pop-up appears saying, “Congratulations! You’re the 100th visitor! Claim your prize!” What should you do?".to_string(),
        "This is a scam trying to steal your information or harm your device.".to_string(),

        // quiz9
        "Send them money".to_string(),
        "Ask for more details in an email".to_string(),
        "Reply and offer help".to_string(),
        "Contact your friend another way (like by phone) to check".to_string(),
        "4".to_string(),
        "".to_string(),
        "You receive an email from your friend saying they are stuck overseas and need money. What’s the best response?".to_string(),
        "Scammers often use hacked accounts to send fake emails. Always verify through another method.".to_string(),

        // quiz10
        "password123".to_string(),
        "John2020".to_string(),
        "Il0vePizza!@#".to_string(),
        "12345678".to_string(),
        "3".to_string(),
        "".to_string(),
        "Which of these passwords is the safest?".to_string(),
        "Strong passwords mix letters, numbers, and symbols".to_string(),

        // quiz11
        "Hover over the link to see where it goes".to_string(),
        "Click quickly so you don’t miss out".to_string(),
        "Copy it and paste it in your browser".to_string(),
        "Trust it if the logo looks real".to_string(),
        "1".to_string(),
        "".to_string(),
        "What should you do before clicking any link in an email or message?".to_string(),
        "Hovering shows the true destination of a link. Always check it.".to_string(),

        // quiz12
        "Urgent requests".to_string(),
        "Generic greetings like “Dear user”".to_string(),
        "Asking for private information".to_string(),
        "Personalized emails from real companies".to_string(),
        "4".to_string(),
        "".to_string(),
        "Which of these is NOT a common sign of a scam?".to_string(),
        "Real companies use personalized and secure messages.".to_string(),

        // quiz13
        "Sure, let’s do it!".to_string(),
        "Ask for proof".to_string(),
        "Report them and protect your account".to_string(),
        "Tell them your cat’s name instead".to_string(),
        "3".to_string(),
        "".to_string(),
        "You’re playing your favorite game and someone messages: “I’ll give you 10,000 coins—just share your login.” What should you do?".to_string(),
        "Never share your login, even for imaginary coins.".to_string(),

        // quiz14
        "They must be forgetful. Let me help.".to_string(),
        "Wait… banks don’t ask for passwords like that.".to_string(),
        "I’ll email my PIN too to be extra helpful.".to_string(),
        "Trust them; they said “please.”".to_string(),
        "2".to_string(),
        "".to_string(),
        "A message says, “This is your bank. We lost your account. Please send your password to fix it.” What do you think?".to_string(),
        "Banks never ask for sensitive information through email. Never.".to_string(),

        // quiz15
        "Sure thing, Boss!".to_string(),
        "Ignore it. That’s not how bosses act.".to_string(),
        "Ask if they want snacks too.".to_string(),
        "Send them Monopoly gift cards instead.".to_string(),
        "24".to_string(),
        "".to_string(),
        "You get an email from “YourBoss123@notarealcompany.biz” saying, “Send me gift cards, fast!” What do you do?".to_string(),
        "Scammers pretend to be people you trust. Always double-check.".to_string(),

        // quiz16
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),

        // quiz17
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),

        // quiz18
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),

        // quiz19
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),

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
                    //println!("Loaded game data: {:?}", data);
                    data
                }
                Err(_) => {
                    //println!("Failed to parse game data, creating new one");
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
