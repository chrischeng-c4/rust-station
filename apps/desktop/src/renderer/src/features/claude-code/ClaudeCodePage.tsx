import { PresetsPanel } from '@/features/workflows/PresetsPanel'

/**
 * Claude Code Tab - Agent Presets Management
 *
 * Manages worktree-scoped agent presets for custom Claude Code behavior.
 * Each worktree stores agents in .claude/agents/*.md files.
 */
export function ClaudeCodePage() {
  return <PresetsPanel />
}
