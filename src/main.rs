use std::process::exit;

use log::{info, error};
use env_logger;
use serde_yaml;

pub mod notifier;
pub mod provider;
pub mod property;
mod bot_config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let f = std::fs::File::open("config.yaml")?;
    let conf: bot_config::Config = serde_yaml::from_reader(f)?;

    let api_key = &conf.notifier.token;
    let api = frankenstein::Api::new(api_key);
    let notifier = notifier::Notifier::new(conf.notifier, api);

    let new_properties : Vec<property::Property> = conf.providers
        .iter()
        .map(|provider| {
            // info!("Processing provider {}", &provider_data.base_url);
            // provider.process_properties();
            Vec::new()
        })
        .flatten()
        .collect();

    println!("Hello, world!");
    Ok(())
}
