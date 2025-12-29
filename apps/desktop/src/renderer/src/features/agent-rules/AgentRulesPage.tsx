import { useState, useCallback } from 'react'
import { Bot, RefreshCw, AlertTriangle, Info, Plus } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { ScrollArea } from '@/components/ui/scroll-area'
import { useAgentRulesState } from '@/hooks/useAppState'
import { ProfileSelector } from './ProfileSelector'
import { ProfileList } from './ProfileList'
import { ProfileEditorDialog } from './ProfileEditorDialog'
import type { AgentProfile } from '@/types/state'

/**
 * Agent Rules Management Page.
 * Allows managing multiple agent profiles per project.
 */
export function AgentRulesPage() {
  const { agentRulesConfig, project, dispatch, isLoading } = useAgentRulesState()

  // Dialog state
  const [isEditorOpen, setIsEditorOpen] = useState(false)
  const [editingProfile, setEditingProfile] = useState<AgentProfile | undefined>()

  // Handlers
  const handleToggle = useCallback(async () => {
    if (!agentRulesConfig) return
    await dispatch({
      type: 'SetAgentRulesEnabled',
      payload: { enabled: !agentRulesConfig.enabled },
    })
  }, [agentRulesConfig, dispatch])

  const handleSelectProfile = useCallback(
    async (profileId: string | undefined) => {
      await dispatch({
        type: 'SelectAgentProfile',
        payload: { profile_id: profileId },
      })
    },
    [dispatch],
  )

  const handleCreateProfile = useCallback(() => {
    setEditingProfile(undefined)
    setIsEditorOpen(true)
  }, [])

  const handleEditProfile = useCallback((profile: AgentProfile) => {
    setEditingProfile(profile)
    setIsEditorOpen(true)
  }, [])

  const handleDeleteProfile = useCallback(
    async (profileId: string) => {
      if (confirm('Are you sure you want to delete this profile?')) {
        await dispatch({
          type: 'DeleteAgentProfile',
          payload: { id: profileId },
        })
      }
    },
    [dispatch],
  )

  const handleSaveProfile = useCallback(
    async (name: string, prompt: string) => {
      if (editingProfile) {
        // Update existing profile
        await dispatch({
          type: 'UpdateAgentProfile',
          payload: {
            id: editingProfile.id,
            name,
            prompt,
          },
        })
      } else {
        // Create new profile
        await dispatch({
          type: 'CreateAgentProfile',
          payload: { name, prompt },
        })
      }
    },
    [editingProfile, dispatch],
  )

  // Loading state
  if (isLoading) {
    return (
      <div className="flex h-full items-center justify-center">
        <RefreshCw className="h-8 w-8 animate-spin text-muted-foreground" />
      </div>
    )
  }

  // No project open
  if (!project || !agentRulesConfig) {
    return (
      <div className="flex h-full flex-col items-center justify-center">
        <Bot className="h-12 w-12 text-muted-foreground" />
        <h2 className="mt-4 text-xl font-semibold">No Project Open</h2>
        <p className="mt-2 text-muted-foreground">
          Open a project to customize Claude Code behavior.
        </p>
      </div>
    )
  }

  const isEnabled = agentRulesConfig.enabled
  const activeProfile = agentRulesConfig.profiles.find(
    (p) => p.id === agentRulesConfig.active_profile_id,
  )

  return (
    <>
      <ScrollArea className="h-full">
        <div className="space-y-6 p-4">
          {/* Header */}
          <div className="flex items-center justify-between">
            <div>
              <h2 className="text-2xl font-semibold">Agent Rules</h2>
              <p className="mt-1 text-muted-foreground">
                Custom AI behavior for {project.name}
              </p>
            </div>
            <Button variant={isEnabled ? 'default' : 'outline'} onClick={handleToggle}>
              <Bot className="mr-2 h-4 w-4" />
              {isEnabled ? 'Enabled' : 'Disabled'}
            </Button>
          </div>

          {/* Warning Card */}
          {isEnabled && activeProfile && (
            <Card className="border-yellow-500/50 bg-yellow-500/10 p-4">
              <div className="flex items-start gap-3">
                <AlertTriangle className="h-5 w-5 text-yellow-600 dark:text-yellow-500 mt-0.5" />
                <div className="flex-1">
                  <h3 className="font-medium text-yellow-900 dark:text-yellow-100">
                    Custom Rules Active
                  </h3>
                  <p className="mt-1 text-sm text-yellow-800 dark:text-yellow-200">
                    Profile <strong>{activeProfile.name}</strong> will <strong>replace</strong> the
                    default CLAUDE.md instructions.
                  </p>
                </div>
              </div>
            </Card>
          )}

          {/* Profile Selection */}
          <Card className="p-4">
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-lg font-medium">Active Profile</h3>
              <Button size="sm" onClick={handleCreateProfile}>
                <Plus className="mr-2 h-4 w-4" />
                New Profile
              </Button>
            </div>

            <ProfileSelector
              profiles={agentRulesConfig.profiles}
              activeProfileId={agentRulesConfig.active_profile_id}
              onSelect={handleSelectProfile}
              disabled={!isEnabled}
            />

            {!isEnabled && (
              <p className="mt-2 text-xs text-muted-foreground">
                Enable agent rules to select a profile
              </p>
            )}
          </Card>

          {/* Active Profile Preview */}
          {activeProfile && (
            <Card className="p-4">
              <h3 className="text-lg font-medium mb-3">Profile Preview</h3>
              <div className="space-y-2">
                <div className="flex items-center justify-between text-sm">
                  <span className="text-muted-foreground">Name:</span>
                  <span className="font-medium">{activeProfile.name}</span>
                </div>
                {activeProfile.is_builtin && (
                  <div className="flex items-center justify-between text-sm">
                    <span className="text-muted-foreground">Type:</span>
                    <span className="text-yellow-600 dark:text-yellow-500">⭐ Built-in</span>
                  </div>
                )}
                <div className="flex items-center justify-between text-sm">
                  <span className="text-muted-foreground">Updated:</span>
                  <span>{new Date(activeProfile.updated_at).toLocaleString()}</span>
                </div>
                <div className="mt-3">
                  <p className="text-sm text-muted-foreground mb-2">Prompt:</p>
                  <pre className="text-xs font-mono bg-muted p-3 rounded-md overflow-x-auto max-h-[200px] whitespace-pre-wrap">
                    {activeProfile.prompt}
                  </pre>
                </div>
              </div>
            </Card>
          )}

          {/* Profile List */}
          <Card className="p-4">
            <h3 className="text-lg font-medium mb-4">All Profiles</h3>
            <ProfileList
              profiles={agentRulesConfig.profiles}
              activeProfileId={agentRulesConfig.active_profile_id}
              onEdit={handleEditProfile}
              onDelete={handleDeleteProfile}
              onSelect={handleSelectProfile}
            />
          </Card>

          {/* Info Card */}
          <Card className="p-4">
            <div className="flex items-start gap-3">
              <Info className="h-5 w-5 text-blue-600 dark:text-blue-400 mt-0.5" />
              <div className="flex-1 space-y-2">
                <h3 className="font-medium">How Agent Rules Work</h3>
                <ul className="space-y-1 text-sm text-muted-foreground list-disc list-inside">
                  <li>
                    Select a profile to customize Claude Code's system prompt via{' '}
                    <code className="text-xs bg-muted px-1 py-0.5 rounded">
                      --system-prompt-file
                    </code>
                  </li>
                  <li>
                    Built-in profiles (⭐) provide expert templates for Rust, TypeScript, and Code
                    Review
                  </li>
                  <li>Create custom profiles to define your own rules and coding standards</li>
                  <li>Different projects can use different profiles (project-scoped)</li>
                  <li>
                    Built-in profiles cannot be edited or deleted, but you can create custom
                    variants
                  </li>
                </ul>
              </div>
            </div>
          </Card>
        </div>
      </ScrollArea>

      {/* Profile Editor Dialog */}
      <ProfileEditorDialog
        open={isEditorOpen}
        onOpenChange={setIsEditorOpen}
        profile={editingProfile}
        onSave={handleSaveProfile}
      />
    </>
  )
}
