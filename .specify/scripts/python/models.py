"""Data models for spec-kit automation."""

from dataclasses import dataclass, field
from enum import Enum
from pathlib import Path
from typing import Optional
import uuid


class FeatureStatus(Enum):
    """Workflow states for a feature."""

    DISCOVERED = "discovered"  # Found in roadmap, no issue
    ISSUE_CREATED = "issue_created"  # GitHub issue exists
    SPECIFIED = "specified"  # spec.md created
    CLARIFIED = "clarified"  # Clarifications resolved
    PLANNED = "planned"  # plan.md created
    TASKED = "tasked"  # tasks.md created
    IMPLEMENTING = "implementing"  # Implementation in progress
    REVIEWING = "reviewing"  # PR under review
    COMPLETE = "complete"  # Merged and closed


@dataclass
class Feature:
    """Represents a feature being tracked."""

    number: int  # Feature number (e.g., 17)
    name: str  # Short name (e.g., "if-then-else")
    description: str  # Full description
    issue_number: Optional[int] = None  # GitHub main issue number
    branch: str = ""  # Branch name (e.g., "017-if-then-else")
    spec_dir: Optional[Path] = None  # Path to specs/{branch}/
    status: FeatureStatus = FeatureStatus.DISCOVERED
    session_id: str = field(default_factory=lambda: str(uuid.uuid4()))
    user_stories: list["UserStory"] = field(default_factory=list)
    pr_number: Optional[int] = None

    def __post_init__(self):
        if not self.branch:
            self.branch = f"{self.number:03d}-{self.name}"


@dataclass
class UserStory:
    """Represents a user story within a feature."""

    id: str  # e.g., "US1"
    description: str
    priority: str  # P1, P2, etc.
    issue_number: Optional[int] = None  # Sub-issue number
    tasks: list["Task"] = field(default_factory=list)
    status: str = "pending"  # pending, in_progress, complete


@dataclass
class Task:
    """Represents a task from tasks.md."""

    id: str  # e.g., "T001"
    description: str
    file_path: Optional[str] = None
    is_parallel: bool = False
    user_story: Optional[str] = None  # US1, US2, etc.
    completed: bool = False


@dataclass
class ClaudeResult:
    """Result from a Claude CLI invocation."""

    success: bool
    output: str  # Raw output text
    json_output: Optional[dict] = None  # Parsed JSON (if --output-format json)
    session_id: Optional[str] = None
    artifacts: list[str] = field(default_factory=list)  # Files created/modified
    cost_usd: Optional[float] = None
    duration_seconds: float = 0.0
    error_message: Optional[str] = None


@dataclass
class GitHubIssue:
    """Represents a GitHub issue."""

    number: int
    title: str
    body: str
    state: str  # OPEN, CLOSED
    labels: list[str] = field(default_factory=list)
    comments: list[str] = field(default_factory=list)


@dataclass
class GitHubPR:
    """Represents a GitHub pull request."""

    number: int
    title: str
    body: str
    head_branch: str
    base_branch: str
    state: str  # OPEN, MERGED, CLOSED
    linked_issues: list[int] = field(default_factory=list)
