use std::{env, time::Duration};
use dotenv::dotenv;
use eyre::{Context, Ok, Result};
mod notification;
use reqwest::blocking::ClientBuilder;
use notification::State;
mod ntfy;
mod github;

fn main() -> Result<()> {
    env_logger::init();
    log::info!("App start");
    dotenv()?;
    let github_token = env::var("GITHUB_TOKEN").context("Please set GITHUB_TOKEN in .env")?;
    let ntfy_url = env::var("NTFY_URL").context("Please set NTFY_URL in .env")?;
    let mut state = State::try_create().unwrap();

    log::debug!("ntfy_url: {}", ntfy_url);
    let client = ClientBuilder::new().timeout(Duration::from_secs(5)).build()?;
    let notifications = github::get_notifications(&client, &github_token)?;
    
    for notification in notifications {
        let is_new = notification.is_new(&mut state);
        log::debug!("id: {} last: {:?} is_new: {}", notification.id, state.last_id, is_new);
        if is_new {
            ntfy::send_notification(&client, &ntfy_url, &notification)?;
        }
    }
    Ok(())
}
