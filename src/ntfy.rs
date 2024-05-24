use eyre::Result;
use reqwest::{blocking::Client, header::{HeaderMap, HeaderValue}};

use crate::notification::Notification;

pub fn send_notification(client: &Client, url: &str, notification: &Notification) -> Result<()> {
    let body = format!("{}: {}", notification.repo_name, notification.title);
    let mut headers = HeaderMap::new();
    headers.insert("Click", HeaderValue::from_str(&notification.url).unwrap());
    let res = client.post(url).headers(headers).body(body).send().unwrap();
    res.error_for_status()?;
    Ok(())
}
