"""Custom exceptions for spec-kit automation."""


class SpecKitError(Exception):
    """Base exception for spec-kit automation."""

    pass


class GitHubError(SpecKitError):
    """Error from GitHub operations."""

    pass


class ClaudeError(SpecKitError):
    """Error from Claude CLI operations."""

    pass


class FeatureError(SpecKitError):
    """Error from feature tracking operations."""

    pass


class WorkflowError(SpecKitError):
    """Error from workflow orchestration."""

    pass
