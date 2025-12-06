"""Handles all GitHub operations via gh CLI."""

import json
import re
import subprocess
from pathlib import Path

from .exceptions import GitHubError
from .models import GitHubIssue, GitHubPR


class GitHubClient:
    """Handles all GitHub operations via gh CLI."""

    # Patterns for issue title parsing
    FEATURE_PATTERN = re.compile(r"Feature\s+(\d+):\s*(.+)", re.IGNORECASE)
    USER_STORY_PATTERN = re.compile(r"US(\d+):\s*(.+)", re.IGNORECASE)
    PARENT_PATTERN = re.compile(r"Parent:\s*#(\d+)", re.IGNORECASE)

    def __init__(self, repo_root: Path):
        self.repo_root = repo_root
        self._verify_gh_cli()

    def _verify_gh_cli(self) -> None:
        """Verify gh CLI is installed and authenticated."""
        try:
            result = subprocess.run(
                ["gh", "auth", "status"],
                capture_output=True,
                text=True,
                check=True,
            )
        except subprocess.CalledProcessError as e:
            raise GitHubError(f"GitHub CLI not authenticated: {e.stderr}")
        except FileNotFoundError:
            raise GitHubError("GitHub CLI (gh) not installed")

    def _run_gh(
        self, args: list[str], check: bool = True
    ) -> subprocess.CompletedProcess:
        """Execute gh command."""
        result = subprocess.run(
            ["gh"] + args,
            capture_output=True,
            text=True,
            cwd=self.repo_root,
            check=False,
        )
        if check and result.returncode != 0:
            raise GitHubError(f"gh command failed: {result.stderr}")
        return result

    # === Issue Operations ===

    def list_issues(
        self,
        state: str = "all",
        labels: list[str] | None = None,
        limit: int = 100,
    ) -> list[GitHubIssue]:
        """List GitHub issues."""
        args = [
            "issue",
            "list",
            "--state",
            state,
            "--limit",
            str(limit),
            "--json",
            "number,title,body,state,labels",
        ]
        if labels:
            args.extend(["--label", ",".join(labels)])

        result = self._run_gh(args)
        issues_data = json.loads(result.stdout)

        return [
            GitHubIssue(
                number=i["number"],
                title=i["title"],
                body=i.get("body", ""),
                state=i["state"],
                labels=[label["name"] for label in i.get("labels", [])],
            )
            for i in issues_data
        ]

    def get_issue(self, number: int) -> GitHubIssue:
        """Get a specific issue with comments."""
        args = [
            "issue",
            "view",
            str(number),
            "--json",
            "number,title,body,state,labels,comments",
        ]
        result = self._run_gh(args)
        data = json.loads(result.stdout)

        return GitHubIssue(
            number=data["number"],
            title=data["title"],
            body=data.get("body", ""),
            state=data["state"],
            labels=[label["name"] for label in data.get("labels", [])],
            comments=[c["body"] for c in data.get("comments", [])],
        )

    def create_issue(
        self,
        title: str,
        body: str,
        labels: list[str] | None = None,
    ) -> int:
        """Create a new issue. Returns issue number."""
        args = ["issue", "create", "--title", title, "--body", body]
        if labels:
            for label in labels:
                args.extend(["--label", label])

        result = self._run_gh(args)
        # Parse issue URL to get number
        # Output format: https://github.com/owner/repo/issues/123
        match = re.search(r"/issues/(\d+)", result.stdout)
        if not match:
            raise GitHubError(f"Could not parse issue number from: {result.stdout}")
        return int(match.group(1))

    def update_issue_body(self, number: int, body: str) -> None:
        """Update an issue's body."""
        self._run_gh(["issue", "edit", str(number), "--body", body])

    def add_issue_comment(self, number: int, body: str) -> None:
        """Add a comment to an issue."""
        self._run_gh(["issue", "comment", str(number), "--body", body])

    def close_issue(self, number: int) -> None:
        """Close an issue."""
        self._run_gh(["issue", "close", str(number)])

    def add_label(self, number: int, label: str) -> None:
        """Add a label to an issue."""
        self._run_gh(["issue", "edit", str(number), "--add-label", label])

    # === PR Operations ===

    def create_pr(
        self,
        title: str,
        body: str,
        head: str,
        base: str = "main",
    ) -> int:
        """Create a pull request. Returns PR number."""
        args = [
            "pr",
            "create",
            "--title",
            title,
            "--body",
            body,
            "--head",
            head,
            "--base",
            base,
        ]
        result = self._run_gh(args)
        match = re.search(r"/pull/(\d+)", result.stdout)
        if not match:
            raise GitHubError(f"Could not parse PR number from: {result.stdout}")
        return int(match.group(1))

    def get_pr(self, number: int) -> GitHubPR:
        """Get PR details."""
        args = [
            "pr",
            "view",
            str(number),
            "--json",
            "number,title,body,headRefName,baseRefName,state",
        ]
        result = self._run_gh(args)
        data = json.loads(result.stdout)

        # Extract linked issues from body
        linked = re.findall(
            r"(?:closes|fixes|resolves)\s*#(\d+)", data.get("body", ""), re.I
        )

        return GitHubPR(
            number=data["number"],
            title=data["title"],
            body=data.get("body", ""),
            head_branch=data["headRefName"],
            base_branch=data["baseRefName"],
            state=data["state"],
            linked_issues=[int(i) for i in linked],
        )

    def merge_pr(self, number: int, merge_method: str = "merge") -> None:
        """Merge a pull request."""
        self._run_gh(["pr", "merge", str(number), f"--{merge_method}"])

    def get_pr_for_branch(self, branch: str) -> GitHubPR | None:
        """Get PR for a specific branch."""
        args = [
            "pr",
            "list",
            "--head",
            branch,
            "--json",
            "number,title,body,headRefName,baseRefName,state",
            "--limit",
            "1",
        ]
        result = self._run_gh(args, check=False)
        if result.returncode != 0:
            return None

        prs = json.loads(result.stdout)
        if not prs:
            return None

        data = prs[0]
        return GitHubPR(
            number=data["number"],
            title=data["title"],
            body=data.get("body", ""),
            head_branch=data["headRefName"],
            base_branch=data["baseRefName"],
            state=data["state"],
        )

    # === Feature Parsing ===

    def parse_feature_from_issue(self, issue: GitHubIssue) -> tuple[int, str] | None:
        """
        Parse feature number and name from issue title.
        Returns (feature_number, feature_name) or None.
        """
        match = self.FEATURE_PATTERN.match(issue.title)
        if match:
            return int(match.group(1)), match.group(2).strip()
        return None

    def parse_user_story_from_issue(
        self, issue: GitHubIssue
    ) -> tuple[str, int] | None:
        """
        Parse user story ID and parent issue from issue.
        Returns (story_id, parent_issue_number) or None.
        """
        match = self.USER_STORY_PATTERN.match(issue.title)
        if not match:
            return None

        story_id = f"US{match.group(1)}"

        parent_match = self.PARENT_PATTERN.search(issue.body)
        if not parent_match:
            return None

        return story_id, int(parent_match.group(1))

    def find_feature_issues(self) -> list[GitHubIssue]:
        """Find all feature issues (main issues, not sub-issues)."""
        issues = self.list_issues(state="all")
        return [i for i in issues if self.FEATURE_PATTERN.match(i.title)]

    def find_user_story_issues(self, parent_number: int) -> list[GitHubIssue]:
        """Find all sub-issues (user stories) for a parent feature."""
        issues = self.list_issues(state="all")
        stories = []
        for issue in issues:
            parsed = self.parse_user_story_from_issue(issue)
            if parsed and parsed[1] == parent_number:
                stories.append(issue)
        return stories
