use eyre::Result;
use reqwest::blocking::Client;

use crate::notification::Notification;

pub fn send_notification(client: &Client, bot_token: &str, user_id: &str, notification: &Notification) -> Result<()> {
    let telegram_url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    let link = format!("<a href=\"{}\">{}</a>", notification.url, notification.title);
    let text = format!("📢 {} - {} 🚀", notification.repo_name, link);
    let params = [
        ("chat_id", user_id),
        ("text", &text),
        ("parse_mode", "HTML"), // Specify HTML parse mode for formatting
    ];
    let res = client.post(&telegram_url)
        .form(&params)
        .send()?;
    // res.error_for_status()?;
    log::debug!("Notification sent: {}", res.text().unwrap());
    Ok(())
}
