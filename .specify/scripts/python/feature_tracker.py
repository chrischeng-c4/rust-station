"""Tracks features and maps them to GitHub issues."""

import re
from pathlib import Path

from .github_client import GitHubClient
from .models import Feature, FeatureStatus, Task, UserStory
from .session_manager import SessionManager


class FeatureTracker:
    """Tracks features and maps them to GitHub issues."""

    def __init__(
        self,
        repo_root: Path,
        github_client: GitHubClient,
        session_manager: SessionManager,
    ):
        self.repo_root = repo_root
        self.specs_dir = repo_root / "specs"
        self.github = github_client
        self.sessions = session_manager

    def discover_features_from_github(self) -> list[Feature]:
        """
        Discover features from GitHub issues.
        GitHub issues are the source of truth.
        """
        feature_issues = self.github.find_feature_issues()
        features = []

        for issue in feature_issues:
            parsed = self.github.parse_feature_from_issue(issue)
            if not parsed:
                continue

            feature_num, feature_name = parsed
            branch = f"{feature_num:03d}-{self._slugify(feature_name)}"
            spec_dir = self.specs_dir / branch

            # Determine status from local files
            status = self._determine_status(spec_dir, issue.state)

            feature = Feature(
                number=feature_num,
                name=feature_name,
                description=feature_name,
                issue_number=issue.number,
                branch=branch,
                spec_dir=spec_dir,
                status=status,
                session_id=self.sessions.get_or_create_session(branch),
            )

            # Load user stories from sub-issues
            story_issues = self.github.find_user_story_issues(issue.number)
            for si in story_issues:
                parsed_story = self.github.parse_user_story_from_issue(si)
                if parsed_story:
                    story_id, _ = parsed_story
                    feature.user_stories.append(
                        UserStory(
                            id=story_id,
                            description=si.title,
                            priority="P1",  # TODO: Parse from body
                            issue_number=si.number,
                        )
                    )

            features.append(feature)

        return sorted(features, key=lambda f: f.number)

    def _determine_status(self, spec_dir: Path, issue_state: str) -> FeatureStatus:
        """Determine feature status from local files and issue state."""
        if issue_state == "CLOSED":
            return FeatureStatus.COMPLETE

        if not spec_dir.exists():
            return FeatureStatus.ISSUE_CREATED

        spec_file = spec_dir / "spec.md"
        plan_file = spec_dir / "plan.md"
        tasks_file = spec_dir / "tasks.md"

        if tasks_file.exists():
            # Check if all tasks complete
            if self._all_tasks_complete(tasks_file):
                return FeatureStatus.REVIEWING
            return FeatureStatus.IMPLEMENTING

        if plan_file.exists():
            return FeatureStatus.PLANNED

        if spec_file.exists():
            # Check for clarifications section
            content = spec_file.read_text()
            if "## Clarifications" in content:
                return FeatureStatus.CLARIFIED
            return FeatureStatus.SPECIFIED

        return FeatureStatus.ISSUE_CREATED

    def _all_tasks_complete(self, tasks_file: Path) -> bool:
        """Check if all tasks in tasks.md are marked complete."""
        content = tasks_file.read_text()
        incomplete = re.findall(r"- \[ \]", content)
        complete = re.findall(r"- \[x\]", content, re.IGNORECASE)
        return len(incomplete) == 0 and len(complete) > 0

    def _slugify(self, text: str) -> str:
        """Convert text to slug format."""
        text = text.lower()
        text = re.sub(r"[^a-z0-9]+", "-", text)
        text = re.sub(r"-+", "-", text)
        return text.strip("-")

    def get_feature_by_number(self, number: int) -> Feature | None:
        """Get a specific feature by number."""
        features = self.discover_features_from_github()
        for f in features:
            if f.number == number:
                return f
        return None

    def get_open_features(self) -> list[Feature]:
        """Get all features that are not complete."""
        features = self.discover_features_from_github()
        return [f for f in features if f.status != FeatureStatus.COMPLETE]

    def parse_tasks_from_file(self, tasks_file: Path) -> list[Task]:
        """Parse tasks from a tasks.md file."""
        if not tasks_file.exists():
            return []

        content = tasks_file.read_text()
        tasks = []

        # Pattern: - [ ] T001 [P] [US1] Description with file path
        pattern = re.compile(r"- \[([ xX])\] (T\d+)\s*(\[P\])?\s*(\[US\d+\])?\s*(.+)")

        for match in pattern.finditer(content):
            completed = match.group(1).lower() == "x"
            task_id = match.group(2)
            is_parallel = match.group(3) is not None
            user_story = match.group(4)[1:-1] if match.group(4) else None
            description = match.group(5).strip()

            # Extract file path from description
            file_match = re.search(r"in\s+`?([^`\s]+)`?$", description)
            file_path = file_match.group(1) if file_match else None

            tasks.append(
                Task(
                    id=task_id,
                    description=description,
                    file_path=file_path,
                    is_parallel=is_parallel,
                    user_story=user_story,
                    completed=completed,
                )
            )

        return tasks

    def get_user_stories_from_spec(self, spec_file: Path) -> list[str]:
        """Extract user story IDs from spec.md."""
        if not spec_file.exists():
            return []

        content = spec_file.read_text()
        # Match patterns like "## US1:" or "### US1:" or "**US1:**"
        pattern = re.compile(r"(?:##?\s*|^\*\*)(US\d+)[:\*]", re.MULTILINE)
        return list(set(pattern.findall(content)))
