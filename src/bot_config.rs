use super::notifier::NotifyConfig;
use super::provider::Provider;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub notifier: NotifyConfig,
    pub providers: Vec<Provider>
}

