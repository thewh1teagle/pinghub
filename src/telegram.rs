use eyre::Result;
use reqwest::blocking::Client;

use crate::notification::Notification;

pub fn send_notification(client: &Client, bot_token: &str, user_id: &str, notification: &Notification) -> Result<()> {
    let telegram_url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    
    // Determine the link based on the availability of URL
    let link = if let Some(url) = &notification.url {
        format!("<a href=\"{}\">{}</a>", url, &notification.title)
    } else if let Some(repo_url) = &notification.repo_url {
        format!("<a href=\"{}\">{}</a>", repo_url, &notification.title)
    } else {
        notification.title.clone() // Use title without link if both URL and repo URL are None
    };

    // Format the notification text including the repo name and kind
    let text = format!("📢 {} - {} 🚀 ({})", 
        notification.repo_name.as_deref().unwrap_or("Unknown Repo"), 
        link, 
        notification.kind
    );

    // Prepare request parameters
    let params = [
        ("chat_id", user_id),
        ("text", &text),
        ("parse_mode", "HTML"), // Specify HTML parse mode for formatting
        ("disable_web_page_preview", "true")
    ];

    // Send the notification via POST request
    let res = client.post(&telegram_url)
        .form(&params)
        .send()?;
    
    // Check if the request was successful
    // res.error_for_status()?;

    // Log the response
    log::debug!("Notification sent: {}", res.text().unwrap());
    
    Ok(())
}
