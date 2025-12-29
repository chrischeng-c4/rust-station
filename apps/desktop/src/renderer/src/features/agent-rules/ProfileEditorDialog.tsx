import { useState, useEffect } from 'react'
import { Button } from '@/components/ui/button'
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
import type { AgentProfile } from '@/types/state'

interface ProfileEditorDialogProps {
  /** Whether the dialog is open */
  open: boolean
  /** Callback when dialog should close */
  onOpenChange: (open: boolean) => void
  /** Profile being edited (undefined for new profile) */
  profile?: AgentProfile
  /** Callback when save is clicked */
  onSave: (name: string, prompt: string) => void
}

/**
 * Dialog for creating or editing an agent profile.
 * Validates that name and prompt are not empty.
 */
export function ProfileEditorDialog({
  open,
  onOpenChange,
  profile,
  onSave,
}: ProfileEditorDialogProps) {
  const [name, setName] = useState('')
  const [prompt, setPrompt] = useState('')
  const [errors, setErrors] = useState<{ name?: string; prompt?: string }>({})

  const isEditing = !!profile

  // Reset form when dialog opens/closes or profile changes
  useEffect(() => {
    if (open) {
      setName(profile?.name || '')
      setPrompt(profile?.prompt || '')
      setErrors({})
    } else {
      setName('')
      setPrompt('')
      setErrors({})
    }
  }, [open, profile])

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
          <DialogTitle>{isEditing ? 'Edit Profile' : 'Create New Profile'}</DialogTitle>
          <DialogDescription>
            {isEditing
              ? 'Update the profile name and system prompt.'
              : 'Create a custom agent profile with specific instructions.'}
          </DialogDescription>
        </DialogHeader>

        <div className="flex-1 space-y-4 overflow-y-auto py-4">
          {/* Name Input */}
          <div className="space-y-2">
            <Label htmlFor="profile-name">
              Profile Name <span className="text-red-500">*</span>
            </Label>
            <Input
              id="profile-name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="e.g. Rust Expert, Code Reviewer"
              className={errors.name ? 'border-red-500' : ''}
            />
            {errors.name && <p className="text-xs text-red-500">{errors.name}</p>}
          </div>

          {/* Prompt Textarea */}
          <div className="space-y-2 flex-1 flex flex-col">
            <Label htmlFor="profile-prompt">
              System Prompt <span className="text-red-500">*</span>
            </Label>
            <textarea
              id="profile-prompt"
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
          <Button onClick={handleSave}>{isEditing ? 'Save Changes' : 'Create Profile'}</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
