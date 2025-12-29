import { Check, ChevronDown } from 'lucide-react'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import type { AgentProfile } from '@/types/state'

interface ProfileSelectorProps {
  /** All available profiles */
  profiles: AgentProfile[]
  /** Currently active profile ID */
  activeProfileId?: string
  /** Callback when a profile is selected */
  onSelect: (profileId: string | undefined) => void
  /** Whether the selector is disabled */
  disabled?: boolean
}

/**
 * Dropdown selector for choosing an agent profile.
 * Shows built-in profiles with a star badge.
 */
export function ProfileSelector({
  profiles,
  activeProfileId,
  onSelect,
  disabled,
}: ProfileSelectorProps) {
  const activeProfile = profiles.find((p) => p.id === activeProfileId)
  const builtinProfiles = profiles.filter((p) => p.is_builtin)
  const customProfiles = profiles.filter((p) => !p.is_builtin)

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="outline" disabled={disabled} className="w-full justify-between">
          <span className="truncate">
            {activeProfile ? (
              <>
                {activeProfile.is_builtin && (
                  <span className="mr-1 text-yellow-600 dark:text-yellow-500">⭐</span>
                )}
                {activeProfile.name}
              </>
            ) : (
              'Select a profile...'
            )}
          </span>
          <ChevronDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="w-[300px]">
        {/* Built-in Profiles */}
        {builtinProfiles.length > 0 && (
          <>
            <DropdownMenuLabel className="text-xs text-muted-foreground">
              Built-in Profiles
            </DropdownMenuLabel>
            {builtinProfiles.map((profile) => (
              <DropdownMenuItem
                key={profile.id}
                onClick={() => onSelect(profile.id)}
                className="cursor-pointer"
              >
                <span className="mr-2 text-yellow-600 dark:text-yellow-500">⭐</span>
                <span className="flex-1">{profile.name}</span>
                {activeProfileId === profile.id && <Check className="h-4 w-4" />}
              </DropdownMenuItem>
            ))}
          </>
        )}

        {/* Custom Profiles */}
        {customProfiles.length > 0 && (
          <>
            {builtinProfiles.length > 0 && <DropdownMenuSeparator />}
            <DropdownMenuLabel className="text-xs text-muted-foreground">
              Custom Profiles
            </DropdownMenuLabel>
            {customProfiles.map((profile) => (
              <DropdownMenuItem
                key={profile.id}
                onClick={() => onSelect(profile.id)}
                className="cursor-pointer"
              >
                <span className="flex-1">{profile.name}</span>
                {activeProfileId === profile.id && <Check className="h-4 w-4" />}
              </DropdownMenuItem>
            ))}
          </>
        )}

        {/* None Option */}
        <DropdownMenuSeparator />
        <DropdownMenuItem
          onClick={() => onSelect(undefined)}
          className="cursor-pointer text-muted-foreground"
        >
          <span className="flex-1">None (use CLAUDE.md)</span>
          {!activeProfileId && <Check className="h-4 w-4" />}
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  )
}
