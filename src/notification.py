import os
import json
from datetime import datetime
from typing import Optional
from log import log
from pathlib import Path

STATE_PATH = os.getenv('STATE_PATH') or 'state.json'
STATE_PATH = Path(STATE_PATH)

class State:
    def __init__(self, last_updated_at: Optional[str] = None):
        self.last_updated_at = last_updated_at

    @classmethod
    def try_create(cls):
        if not STATE_PATH.exists():
            log.info(f"Create empty state at {STATE_PATH}")
            with open(STATE_PATH, 'w') as f:
                f.write(json.dumps({}))
            return cls(last_updated_at=None)
        with open(STATE_PATH, 'r') as f:
            content = f.read()
        state: dict = json.loads(content)
        return cls(last_updated_at=state.get('last_updated_at'))

    def save(self):
        log.info(f"Saving state to {STATE_PATH}")
        with open(STATE_PATH, 'w') as f:
            json.dump(self.__dict__, f, indent=4)

class Notification:
    def __init__(self, id: int, kind: str, title: str, url: Optional[str], repo_url: Optional[str], repo_name: Optional[str], updated_at: Optional[str]):
        self.id = id
        self.kind = kind
        self.title = title
        self.url = url
        self.repo_url = repo_url
        self.repo_name = repo_name
        self.updated_at = updated_at

    def is_new(self, state: State) -> bool:
        # If the new notification has a different timestamp from the previous one, it's new
        if state.last_updated_at and self.updated_at:
            try:
                state_updated_at = datetime.fromisoformat(state.last_updated_at)
                updated_at = datetime.fromisoformat(self.updated_at)
                return updated_at > state_updated_at
            except ValueError:
                return True
        return True
