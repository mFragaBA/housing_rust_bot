use log::info;
use rand::seq::SliceRandom;
use frankenstein::{Api, SendMessageParams, TelegramApi};
use super::property::Property;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct NotifyConfig {
    pub messages: Vec<String>,
    pub chat_id: String,
    pub token: String
}

pub struct Notifier {
    pub config: NotifyConfig,
    pub api: Api,
}

impl Notifier {
    pub fn new(config: NotifyConfig, api: Api) -> Notifier {
        Notifier { config, api }
    }

    // pub fn notify(properties: ?) {
    pub fn notify(self, properties: Vec<Property>) {
        info!("Notifying about {} properties", 1);
        let text = self.config.messages.choose(&mut rand::thread_rng()).unwrap();

        let send_message_params = SendMessageParams::builder()
            .chat_id(self.config.chat_id.clone())
            .text(text)
            .build();

        self.api.send_message(&send_message_params).unwrap();

        for prop in properties {
            info!("Notifying about {}", &prop.url);
            let send_message_params = SendMessageParams::builder()
                .chat_id(self.config.chat_id.clone())
                .parse_mode(frankenstein::ParseMode::MarkdownV2)
                .text(format!("[{}]({})", &prop.title, &prop.url))
                .build();
            self.api.send_message(&send_message_params).unwrap();
        }
    }
}
