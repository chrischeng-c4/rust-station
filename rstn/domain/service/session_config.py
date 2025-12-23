"""Session configuration manager.

Generates session-specific configuration files (like MCP config)
in temporary directories.
"""

from __future__ import annotations

import json
from pathlib import Path
import tempfile
import shutil

from rstn.logging import get_logger

log = get_logger("rstn.domain.session_config")


class SessionConfigManager:
    """Manages session-specific configuration files."""

    def __init__(self, base_dir: Path | None = None) -> None:
        """Initialize with base directory.

        Args:
            base_dir: Optional base directory for temp files.
                     Defaults to system temp / rstn.
        """
        if base_dir is None:
            self.base_dir = Path(tempfile.gettempdir()) / "rstn"
        else:
            self.base_dir = base_dir

    def get_session_dir(self, session_id: str) -> Path:
        """Get the directory for a specific session.

        Args:
            session_id: Unique session identifier

        Returns:
            Path to the session directory
        """
        return self.base_dir / session_id

    def create_mcp_config(self, session_id: str, mcp_port: int) -> Path:
        """Create an MCP configuration file for a session.

        Args:
            session_id: Unique session identifier
            mcp_port: The port the internal MCP server is listening on

        Returns:
            Path to the created mcp-config.json
        """
        session_dir = self.get_session_dir(session_id)
        session_dir.mkdir(parents=True, exist_ok=True)

        config_path = session_dir / "mcp-config.json"
        
        # Build the MCP config in Claude Code format
        config = {
            "mcpServers": {
                "rstn": {
                    "type": "http",
                    "url": f"http://127.0.0.1:{mcp_port}/mcp"
                }
            }
        }

        with config_path.open("w", encoding="utf-8") as f:
            json.dump(config, f, indent=2)

        log.debug("Created MCP config", path=str(config_path), session_id=session_id)
        return config_path

    def cleanup_session(self, session_id: str) -> None:
        """Remove session directory and all its files.

        Args:
            session_id: Unique session identifier
        """
        session_dir = self.get_session_dir(session_id)
        if session_dir.exists():
            shutil.rmtree(session_dir)
            log.debug("Cleaned up session directory", session_id=session_id)

    def cleanup_all(self) -> None:
        """Remove the entire rstn base temp directory."""
        if self.base_dir.exists():
            shutil.rmtree(self.base_dir)
            log.debug("Cleaned up all session directories")
