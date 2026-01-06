import React, { useState, useCallback } from 'react'
import { FileText, Plus, Info, Pencil, Trash2, Star } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { ScrollArea } from '@/components/ui/scroll-area'
import { PageHeader } from '@/components/shared/PageHeader'
import { LoadingState } from '@/components/shared/LoadingState'
import { EmptyState } from '@/components/shared/EmptyState'
import { useAppState } from '@/hooks/useAppState'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import type { ConstitutionPreset } from '@/types/state'

interface PresetEditorDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  preset?: ConstitutionPreset
  onSave: (name: string, prompt: string) => void
}

function PresetEditorDialog({ open, onOpenChange, preset, onSave }: PresetEditorDialogProps) {
  const [name, setName] = useState('')
  const [prompt, setPrompt] = useState('')
  const [errors, setErrors] = useState<{ name?: string; prompt?: string }>({})

  const isEditing = !!preset

  // Reset form when dialog opens/closes or preset changes
  React.useEffect(() => {
    if (open) {
      setName(preset?.name || '')
      setPrompt(preset?.prompt || '')
      setErrors({})
    } else {
      setName('')
      setPrompt('')
      setErrors({})
    }
  }, [open, preset])

  const validate = () => {
    const newErrors: { name?: string; prompt?: string } = {}

    if (!name.trim()) {
      newErrors.name = 'Name is required'
    }

    if (!prompt.trim()) {
      newErrors.prompt = 'Prompt is required'
    }

    setErrors(newErrors)
    return Object.keys(newErrors).length === 0
  }

  const handleSave = () => {
    if (validate()) {
      onSave(name.trim(), prompt.trim())
      onOpenChange(false)
    }
  }

  const charCount = prompt.length

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-2xl max-h-[80vh] flex flex-col">
        <DialogHeader>
          <DialogTitle>{isEditing ? 'Edit Preset' : 'Create New Preset'}</DialogTitle>
          <DialogDescription>
            {isEditing
              ? 'Update the preset name and system prompt.'
              : 'Create a custom constitution preset with specific instructions.'}
          </DialogDescription>
        </DialogHeader>

        <div className="flex-1 space-y-4 overflow-y-auto py-4">
          {/* Name Input */}
          <div className="space-y-2">
            <Label htmlFor="preset-name">
              Preset Name <span className="text-red-500">*</span>
            </Label>
            <Input
              id="preset-name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="e.g. Rust Expert, Code Reviewer"
              className={errors.name ? 'border-red-500' : ''}
            />
            {errors.name && <p className="text-xs text-red-500">{errors.name}</p>}
          </div>

          {/* Prompt Textarea */}
          <div className="space-y-2 flex-1 flex flex-col">
            <Label htmlFor="preset-prompt">
              System Prompt <span className="text-red-500">*</span>
            </Label>
            <textarea
              id="preset-prompt"
              value={prompt}
              onChange={(e) => setPrompt(e.target.value)}
              placeholder={`Example:

You are a Rust programming expert.

Core Principles:
- Always use snake_case for function names
- Prefer Result<T, E> over Option when errors are possible
- Write comprehensive tests for all functions

Code Style:
- Use rustfmt and clippy
- Add doc comments for public APIs`}
              className={`flex-1 min-h-[300px] rounded-md border px-3 py-2 text-sm font-mono ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 resize-none ${
                errors.prompt ? 'border-red-500' : 'border-input bg-background'
              }`}
            />
            <div className="flex items-center justify-between text-xs text-muted-foreground">
              <span>{charCount.toLocaleString()} characters</span>
              {errors.prompt && <span className="text-red-500">{errors.prompt}</span>}
            </div>
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Cancel
          </Button>
          <Button onClick={handleSave}>{isEditing ? 'Save Changes' : 'Create Preset'}</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}

/**
 * Constitution Presets Management Panel.
 * Allows selecting and managing constitution presets (full system prompt replacement).
 */
export function PresetsPanel() {
  const { state, dispatch, isLoading } = useAppState()

  // Dialog state
  const [isEditorOpen, setIsEditorOpen] = useState(false)
  const [editingPreset, setEditingPreset] = useState<ConstitutionPreset | undefined>()

  // Get presets config from active worktree
  const presetsConfig = state.projects[state.active_project_index]
    ?.worktrees[
      state.projects[state.active_project_index]?.active_worktree_index ?? 0
    ]?.tasks?.constitution_presets

  // Handlers
  const handleSelectPreset = useCallback(
    async (presetId: string | null) => {
      await dispatch({
        type: 'SelectConstitutionPreset',
        payload: { preset_id: presetId },
      })
    },
    [dispatch],
  )

  const handleCreatePreset = useCallback(() => {
    setEditingPreset(undefined)
    setIsEditorOpen(true)
  }, [])

  const handleEditPreset = useCallback((preset: ConstitutionPreset) => {
    setEditingPreset(preset)
    setIsEditorOpen(true)
  }, [])

  const handleDeletePreset = useCallback(
    async (presetId: string) => {
      if (confirm('Are you sure you want to delete this preset?')) {
        await dispatch({
          type: 'DeleteConstitutionPreset',
          payload: { id: presetId },
        })
      }
    },
    [dispatch],
  )

  const handleSavePreset = useCallback(
    async (name: string, prompt: string) => {
      if (editingPreset) {
        // Update existing preset
        await dispatch({
          type: 'UpdateConstitutionPreset',
          payload: {
            id: editingPreset.id,
            name,
            prompt,
          },
        })
      } else {
        // Create new preset
        await dispatch({
          type: 'CreateConstitutionPreset',
          payload: { name, prompt },
        })
      }
    },
    [editingPreset, dispatch],
  )

  // Loading state
  if (isLoading) {
    return <LoadingState />
  }

  // No presets config
  if (!presetsConfig) {
    return (
      <EmptyState
        icon={FileText}
        title="No Worktree Active"
        description="Open a worktree to manage constitution presets"
      />
    )
  }

  const activePreset = presetsConfig.presets.find((p) => p.id === presetsConfig.active_preset_id)
  const builtinPresets = presetsConfig.presets.filter((p) => p.is_builtin)
  const customPresets = presetsConfig.presets.filter((p) => !p.is_builtin)

  return (
    <>
      <div className="flex h-full flex-col">
        <PageHeader
          title="Constitution Presets"
          subtitle="Full system prompt replacement mode"
          icon={<FileText className="h-5 w-5" />}
        />

        <ScrollArea className="flex-1 p-4 pt-0">
          <div className="space-y-4">
            {/* Active Preset */}
            {activePreset && (
              <Card className="p-4 border-blue-500/50 bg-blue-50 dark:bg-blue-950/20">
                <div className="flex items-start justify-between mb-3">
                  <div>
                    <div className="flex items-center gap-2">
                      {activePreset.is_builtin && (
                        <Star className="h-4 w-4 text-yellow-600 dark:text-yellow-500" />
                      )}
                      <h3 className="font-medium">{activePreset.name}</h3>
                    </div>
                    <p className="text-xs text-muted-foreground mt-1">Active Preset</p>
                  </div>
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => handleSelectPreset(null)}
                  >
                    Deactivate
                  </Button>
                </div>
                <div className="mt-3">
                  <p className="text-xs text-muted-foreground mb-2">System Prompt:</p>
                  <pre className="text-xs font-mono bg-muted p-3 rounded-md overflow-x-auto max-h-[150px] whitespace-pre-wrap">
                    {activePreset.prompt}
                  </pre>
                </div>
              </Card>
            )}

            {/* Create Button */}
            <Button onClick={handleCreatePreset} className="w-full" size="sm">
              <Plus className="mr-2 h-4 w-4" />
              Create New Preset
            </Button>

            {/* Built-in Presets */}
            {builtinPresets.length > 0 && (
              <div className="space-y-2">
                <h3 className="text-sm font-medium text-muted-foreground">Built-in Presets</h3>
                {builtinPresets.map((preset) => (
                  <Card
                    key={preset.id}
                    className={`p-3 cursor-pointer transition-colors ${
                      presetsConfig.active_preset_id === preset.id
                        ? 'border-primary bg-primary/5'
                        : 'hover:border-primary/50'
                    }`}
                    onClick={() => handleSelectPreset(preset.id)}
                  >
                    <div className="flex items-start justify-between">
                      <div className="flex-1 min-w-0">
                        <div className="flex items-center gap-2">
                          <Star className="h-4 w-4 text-yellow-600 dark:text-yellow-500 shrink-0" />
                          <h4 className="font-medium truncate">{preset.name}</h4>
                        </div>
                        <p className="mt-1 text-xs text-muted-foreground line-clamp-2">
                          {preset.prompt.split('\n')[0]}
                        </p>
                      </div>
                      <div className="ml-2 shrink-0 text-xs text-muted-foreground">Built-in</div>
                    </div>
                  </Card>
                ))}
              </div>
            )}

            {/* Custom Presets */}
            {customPresets.length > 0 && (
              <div className="space-y-2">
                <h3 className="text-sm font-medium text-muted-foreground">Custom Presets</h3>
                {customPresets.map((preset) => (
                  <Card
                    key={preset.id}
                    className={`p-3 cursor-pointer transition-colors ${
                      presetsConfig.active_preset_id === preset.id
                        ? 'border-primary bg-primary/5'
                        : 'hover:border-primary/50'
                    }`}
                    onClick={() => handleSelectPreset(preset.id)}
                  >
                    <div className="flex items-start justify-between">
                      <div className="flex-1 min-w-0">
                        <h4 className="font-medium truncate">{preset.name}</h4>
                        <p className="mt-1 text-xs text-muted-foreground line-clamp-2">
                          {preset.prompt.split('\n')[0] || 'No description'}
                        </p>
                        <p className="mt-1 text-xs text-muted-foreground">
                          Updated {new Date(preset.updated_at).toLocaleDateString()}
                        </p>
                      </div>
                      <div className="ml-2 flex items-center gap-1 shrink-0">
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={(e) => {
                            e.stopPropagation()
                            handleEditPreset(preset)
                          }}
                        >
                          <Pencil className="h-4 w-4" />
                        </Button>
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={(e) => {
                            e.stopPropagation()
                            handleDeletePreset(preset.id)
                          }}
                        >
                          <Trash2 className="h-4 w-4" />
                        </Button>
                      </div>
                    </div>
                  </Card>
                ))}
              </div>
            )}

            {customPresets.length === 0 && (
              <Card className="p-4">
                <p className="text-sm text-muted-foreground text-center">
                  No custom presets yet. Create your first preset!
                </p>
              </Card>
            )}

            {/* Info Card */}
            <Card className="p-4">
              <div className="flex items-start gap-3">
                <Info className="h-5 w-5 text-blue-600 dark:text-blue-400 mt-0.5" />
                <div className="flex-1 space-y-2">
                  <h3 className="font-medium">How Presets Work</h3>
                  <ul className="space-y-1 text-sm text-muted-foreground list-disc list-inside">
                    <li>
                      Presets <strong>replace</strong> the entire system prompt (not append)
                    </li>
                    <li>Built-in presets (⭐) provide expert templates for common roles</li>
                    <li>Create custom presets to define your own coding standards</li>
                    <li>Only one preset can be active at a time</li>
                    <li>Built-in presets cannot be edited or deleted</li>
                  </ul>
                </div>
              </div>
            </Card>
          </div>
        </ScrollArea>
      </div>

      {/* Preset Editor Dialog */}
      <PresetEditorDialog
        open={isEditorOpen}
        onOpenChange={setIsEditorOpen}
        preset={editingPreset}
        onSave={handleSavePreset}
      />
    </>
  )
}
