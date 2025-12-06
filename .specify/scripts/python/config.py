"""Configuration for spec-kit automation."""

import json
from dataclasses import dataclass
from pathlib import Path


@dataclass
class Config:
    """Configuration for spec-kit automation."""

    repo_root: Path
    permission_mode: str = "acceptEdits"
    claude_timeout: int = 600  # 10 minutes
    stop_on_error: bool = True

    def __post_init__(self):
        # Load from config file if exists
        config_file = self.repo_root / ".specify/scripts/python/config.json"
        if config_file.exists():
            with open(config_file) as f:
                data = json.load(f)
                self.permission_mode = data.get("permission_mode", self.permission_mode)
                self.claude_timeout = data.get("claude_timeout", self.claude_timeout)
                self.stop_on_error = data.get("stop_on_error", self.stop_on_error)

    def save(self) -> None:
        """Save configuration to file."""
        config_file = self.repo_root / ".specify/scripts/python/config.json"
        config_file.parent.mkdir(parents=True, exist_ok=True)
        with open(config_file, "w") as f:
            json.dump(
                {
                    "permission_mode": self.permission_mode,
                    "claude_timeout": self.claude_timeout,
                    "stop_on_error": self.stop_on_error,
                },
                f,
                indent=2,
            )
