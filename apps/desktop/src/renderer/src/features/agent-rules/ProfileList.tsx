import { Pencil, Trash2, Star } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import type { AgentProfile } from '@/types/state'

interface ProfileListProps {
  /** All available profiles */
  profiles: AgentProfile[]
  /** Currently active profile ID */
  activeProfileId?: string
  /** Callback when edit is clicked */
  onEdit: (profile: AgentProfile) => void
  /** Callback when delete is clicked */
  onDelete: (profileId: string) => void
  /** Callback when a profile is selected */
  onSelect: (profileId: string) => void
}

/**
 * List view of all agent profiles with edit/delete actions.
 * Built-in profiles show a star badge and cannot be edited/deleted.
 */
export function ProfileList({
  profiles,
  activeProfileId,
  onEdit,
  onDelete,
  onSelect,
}: ProfileListProps) {
  if (profiles.length === 0) {
    return (
      <div className="text-center py-8 text-muted-foreground">
        No profiles available. Create your first custom profile!
      </div>
    )
  }

  const builtinProfiles = profiles.filter((p) => p.is_builtin)
  const customProfiles = profiles.filter((p) => !p.is_builtin)

  return (
    <div className="space-y-4">
      {/* Built-in Profiles */}
      {builtinProfiles.length > 0 && (
        <div className="space-y-2">
          <h3 className="text-sm font-medium text-muted-foreground">Built-in Profiles</h3>
          {builtinProfiles.map((profile) => (
            <Card
              key={profile.id}
              className={`p-3 cursor-pointer transition-colors ${
                activeProfileId === profile.id
                  ? 'border-primary bg-primary/5'
                  : 'hover:border-primary/50'
              }`}
              onClick={() => onSelect(profile.id)}
            >
              <div className="flex items-start justify-between">
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2">
                    <Star className="h-4 w-4 text-yellow-600 dark:text-yellow-500 shrink-0" />
                    <h4 className="font-medium truncate">{profile.name}</h4>
                  </div>
                  <p className="mt-1 text-xs text-muted-foreground line-clamp-2">
                    {profile.prompt.split('\n')[0]}
                  </p>
                </div>
                <div className="ml-2 shrink-0 text-xs text-muted-foreground">Built-in</div>
              </div>
            </Card>
          ))}
        </div>
      )}

      {/* Custom Profiles */}
      {customProfiles.length > 0 && (
        <div className="space-y-2">
          <h3 className="text-sm font-medium text-muted-foreground">Custom Profiles</h3>
          {customProfiles.map((profile) => (
            <Card
              key={profile.id}
              className={`p-3 cursor-pointer transition-colors ${
                activeProfileId === profile.id
                  ? 'border-primary bg-primary/5'
                  : 'hover:border-primary/50'
              }`}
              onClick={() => onSelect(profile.id)}
            >
              <div className="flex items-start justify-between">
                <div className="flex-1 min-w-0">
                  <h4 className="font-medium truncate">{profile.name}</h4>
                  <p className="mt-1 text-xs text-muted-foreground line-clamp-2">
                    {profile.prompt.split('\n')[0] || 'No description'}
                  </p>
                  <p className="mt-1 text-xs text-muted-foreground">
                    Updated {new Date(profile.updated_at).toLocaleDateString()}
                  </p>
                </div>
                <div className="ml-2 flex items-center gap-1 shrink-0">
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={(e) => {
                      e.stopPropagation()
                      onEdit(profile)
                    }}
                  >
                    <Pencil className="h-4 w-4" />
                  </Button>
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={(e) => {
                      e.stopPropagation()
                      onDelete(profile.id)
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
    </div>
  )
}
