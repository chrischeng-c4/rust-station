"""Manages Claude session IDs for features."""

import json
import uuid
from datetime import datetime
from pathlib import Path


class SessionManager:
    """Manages Claude session IDs for features."""

    def __init__(self, storage_path: Path):
        self.storage_path = storage_path
        self._sessions: dict[str, dict] = {}
        self._load()

    def _load(self) -> None:
        """Load sessions from disk."""
        if self.storage_path.exists():
            with open(self.storage_path) as f:
                self._sessions = json.load(f)

    def _save(self) -> None:
        """Save sessions to disk."""
        self.storage_path.parent.mkdir(parents=True, exist_ok=True)
        with open(self.storage_path, "w") as f:
            json.dump(self._sessions, f, indent=2)

    def get_or_create_session(self, feature_branch: str) -> str:
        """Get existing session ID or create new one for a feature."""
        if feature_branch in self._sessions:
            return self._sessions[feature_branch]["session_id"]

        session_id = str(uuid.uuid4())
        self._sessions[feature_branch] = {
            "session_id": session_id,
            "created_at": datetime.now().isoformat(),
            "last_used": datetime.now().isoformat(),
        }
        self._save()
        return session_id

    def update_last_used(self, feature_branch: str) -> None:
        """Update last used timestamp for a session."""
        if feature_branch in self._sessions:
            self._sessions[feature_branch]["last_used"] = datetime.now().isoformat()
            self._save()

    def get_session(self, feature_branch: str) -> str | None:
        """Get session ID if it exists."""
        if feature_branch in self._sessions:
            return self._sessions[feature_branch]["session_id"]
        return None

    def clear_session(self, feature_branch: str) -> None:
        """Remove a session."""
        if feature_branch in self._sessions:
            del self._sessions[feature_branch]
            self._save()

    def list_sessions(self) -> dict[str, dict]:
        """List all sessions."""
        return self._sessions.copy()
