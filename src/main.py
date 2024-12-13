import os
import requests
from dotenv import load_dotenv
from notification import State
from github import get_notifications
from telegram import send_notification
from log import log
from pytimeparse.timeparse import timeparse
import time

def main():
    log.info("App start")
    load_dotenv()
    
    # Get environment variables
    github_token = os.getenv('GITHUB_TOKEN')
    bot_token = os.getenv('BOT_TOKEN')
    user_id = os.getenv('USER_ID')
    interval_env = os.getenv('INTERVAL') or '60s'
    interval = timeparse(interval_env)
    
    if not github_token or not bot_token or not user_id:
        log.error("Please set GITHUB_TOKEN, BOT_TOKEN, and USER_ID in .env")
        return
    
    state = State.try_create()
    log.debug(f"user_id: {user_id}")
    client = requests.Session()
    client.timeout = 5
    
    while True:
        try:
            notifications = get_notifications(client, github_token)
        except Exception as e:
            log.error(f"Error fetching notifications: {e}")
            time.sleep(5)
            return
        
        for notification in notifications:
            log.debug(f"id: {notification.id} last: {state.last_updated_at} updated_at: {notification.updated_at}")
            
            # Check if notification is new
            if notification.is_new(state):
                state.last_updated_at = notification.updated_at
                state.save()
                send_notification(bot_token, user_id, notification)
        log.info(f'Sleep for {interval} seconds')
        time.sleep(interval)


if __name__ == "__main__":
    main()
