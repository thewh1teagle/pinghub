import requests
from typing import List, Dict, Optional
from notification import Notification

GITHUB_URL = "https://api.github.com/notifications"


def get_notifications(client: requests.Session, token: str) -> List[Notification]:
    headers = {
        "Authorization": f"Bearer {token}",
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
        "User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36"
    }
    
    response = client.get(GITHUB_URL, headers=headers)
    response.raise_for_status()  # This will raise an error for HTTP responses that are 4xx/5xx
    data = response.json()

    notifications = []
    
    for raw_notification in data:
        id = int(raw_notification["id"])
        subject = raw_notification["subject"]
        title = subject.get("title", "No Title")
        url = subject.get("url", "").replace("api.github.com/repos/", "github.com/") if subject.get("url") else None
        kind = subject.get("type", "Unknown Type")
        repository = raw_notification["repository"]
        repo_url = repository.get("html_url")
        repo_name = repository.get("name")
        updated_at = raw_notification.get("updated_at")
        
        notification = Notification(id, title, url, repo_name, kind, repo_url, updated_at)
        notifications.append(notification)

    notifications.sort(key=lambda x: x.id)
    return notifications
