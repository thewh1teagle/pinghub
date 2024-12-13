import requests
from log import log

class Notification:
    def __init__(self, id, kind, title, url, repo_url, repo_name, updated_at):
        self.id = id
        self.kind = kind
        self.title = title
        self.url = url
        self.repo_url = repo_url
        self.repo_name = repo_name
        self.updated_at = updated_at

def send_notification(bot_token, user_id, notification):
    telegram_url = f"https://api.telegram.org/bot{bot_token}/sendMessage"
    
    # Determine the link based on the availability of URL
    if notification.url:
        link = f'<a href="{notification.url}">{notification.title}</a>'
    elif notification.repo_url:
        link = f'<a href="{notification.repo_url}">{notification.title}</a>'
    else:
        link = notification.title  # Use title without link if both URL and repo URL are None
    
    # Format the notification text including the repo name and kind
    text = f"📢 {notification.repo_name if notification.repo_name else 'Unknown Repo'} - {link} 🚀 ({notification.kind})"
    
    # Prepare request parameters
    params = {
        "chat_id": user_id,
        "text": text,
        "parse_mode": "HTML",  # Specify HTML parse mode for formatting
        "disable_web_page_preview": "true"
    }

    # Send the notification via POST request
    try:
        res = requests.post(telegram_url, data=params)
        res.raise_for_status()  # Check if the request was successful
        # Log the response
        log.info(f"Notification sent: {res.text}")
    except requests.exceptions.RequestException as e:
        log.info(f"Error sending notification: {e}")