
use eyre::{Context, Ok, Result};

use reqwest::{blocking::Client, header::{HeaderMap, HeaderValue}};
use serde_json::Value;
use crate::notification::Notification;

const GITHUB_URL: &str = "https://api.github.com/notifications"; 

pub fn get_notifications(client: &Client, token: &str) -> Result<Vec<Notification>> {
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", token)).unwrap());
    headers.insert("Accept", HeaderValue::from_str("application/vnd.github+json").unwrap());
    headers.insert("X-GitHub-Api-Version", HeaderValue::from_str("2022-11-28").unwrap());
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36"));
    let resp = client.get(GITHUB_URL).headers(headers).send().unwrap();
    let text = resp.text().unwrap();
    let data: Value = serde_json::from_str(&text).context("from_str")?;
    // let data: Value = resp.json().unwrap();
    let raw_notifications = data.as_array().unwrap();
    let mut notifications: Vec<Notification> = Vec::new();
    

    // Parse notifications
    for raw_notification in raw_notifications {
        let id = raw_notification["id"].as_str().unwrap().parse::<i128>().unwrap();
        let subject = &raw_notification["subject"];
        let title = subject["title"].as_str().unwrap_or("No Title");
        let url = subject["url"].as_str().map(|u| u.replace("api.github.com/repos/", "github.com/"));
        let kind = subject["type"].as_str().unwrap_or("Uknown Type").to_string(); 
        let respository = &raw_notification["repository"];
        let html_url = respository["html_url"].as_str().map(|s| s.to_string());
        let repo_name = respository["name"].as_str().map(|s| s.to_string());
        let updated_at = raw_notification["updated_at"].as_str().map(|s| s.to_string());
        let notification = Notification {id, title: title.into(), url, repo_name, kind, repo_url: html_url, updated_at};
        notifications.push(notification);
    }
    notifications.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(notifications)
}