use std::{env, time::Duration};
use eyre::{Context, Ok, Result};
mod notification;
use reqwest::blocking::ClientBuilder;
use notification::State;
mod telegram;
mod github;

fn main() -> Result<()> {
    env_logger::init();
    log::info!("App start");
    if cfg!(debug_assertions) {
       dotenv::dotenv().unwrap(); 
    } else {
        let env_path = env::current_exe().unwrap().parent().unwrap().join(".env");
        dotenv::from_path(env_path)?;
    }
    
    let github_token = env::var("GITHUB_TOKEN").context("Please set GITHUB_TOKEN in .env")?;
    let bot_token = env::var("BOT_TOKEN").context("Please set BOT_TOKEN in .env")?;
    let user_id = env::var("USER_ID").context("Please set USER_ID in .env")?;
    let mut state = State::try_create().unwrap();

    log::debug!("user_id: {}", user_id);
    let client = ClientBuilder::new().timeout(Duration::from_secs(5)).build()?;
    let notifications = github::get_notifications(&client, &github_token)?;
    
    for notification in notifications {
        log::debug!("id: {} last: {:?} updated_at: {:?}", notification.id, state.last_updated_at, notification.updated_at);
        if notification.is_new(&mut state) {
            state.last_updated_at = notification.updated_at.clone();
            state.save();
            telegram::send_notification(&client, &bot_token, &user_id, &notification)?;
        }
    }
    Ok(())
}
