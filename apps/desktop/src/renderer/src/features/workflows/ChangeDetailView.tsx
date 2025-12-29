import { FileText, Play, Check, X, Clock, Archive, RefreshCw, Rocket } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { useAppState } from '@/hooks/useAppState'
import type { Change } from '@/types/state'

interface ChangeDetailViewProps {
  change: Change
}

/**
 * ChangeDetailView - Shows change details, proposal, and plan
 */
export function ChangeDetailView({ change }: ChangeDetailViewProps) {
  const { dispatch } = useAppState()

  const handleGenerateProposal = () => {
    dispatch({ type: 'GenerateProposal', payload: { change_id: change.id } })
  }

  const handleGeneratePlan = () => {
    dispatch({ type: 'GeneratePlan', payload: { change_id: change.id } })
  }

  const handleApprovePlan = () => {
    dispatch({ type: 'ApprovePlan', payload: { change_id: change.id } })
  }

  const handleCancelChange = () => {
    dispatch({ type: 'CancelChange', payload: { change_id: change.id } })
  }

  const handleSyncContext = () => {
    dispatch({ type: 'SyncContext', payload: { change_id: change.id } })
  }

  const handleArchive = () => {
    dispatch({ type: 'ArchiveChange', payload: { change_id: change.id } })
  }

  const handleExecutePlan = () => {
    dispatch({ type: 'ExecutePlan', payload: { change_id: change.id } })
  }

  const isPlanning = change.status === 'planning'
  const isImplementing = change.status === 'implementing'
  const hasProposal = !!change.proposal
  const hasPlan = !!change.plan
  const canGenerateProposal = change.status === 'proposed' && !hasProposal
  const canGeneratePlan = hasProposal && !hasPlan && change.status !== 'planning'
  const canApprove = change.status === 'planned'
  const canExecute = change.status === 'planned'
  const canCancel = !['done', 'archived', 'cancelled', 'implementing'].includes(change.status)
  const canSyncAndArchive = change.status === 'done'
  const isArchived = change.status === 'archived'

  return (
    <Card className="h-full">
      <CardHeader className="pb-2">
        <div className="flex items-start justify-between">
          <div>
            <CardTitle className="text-lg">{change.name}</CardTitle>
            <CardDescription className="mt-1">{change.intent}</CardDescription>
          </div>
          <Badge variant="outline" className="capitalize">
            {change.status}
          </Badge>
        </div>
      </CardHeader>

      <CardContent className="h-[calc(100%-100px)]">
        <Tabs defaultValue="proposal" className="h-full">
          <TabsList className="mb-4">
            <TabsTrigger value="proposal" className="gap-1">
              <FileText className="h-4 w-4" />
              Proposal
            </TabsTrigger>
            <TabsTrigger value="plan" className="gap-1">
              <FileText className="h-4 w-4" />
              Plan
            </TabsTrigger>
            <TabsTrigger value="implementation" className="gap-1">
              <Rocket className="h-4 w-4" />
              Implementation
            </TabsTrigger>
          </TabsList>

          <TabsContent value="proposal" className="h-[calc(100%-50px)]">
            {isPlanning && change.streaming_output ? (
              <ScrollArea className="h-full rounded-md border p-4">
                <div className="flex items-center gap-2 mb-2 text-yellow-600">
                  <Clock className="h-4 w-4 animate-spin" />
                  <span className="text-sm font-medium">Generating...</span>
                </div>
                <pre className="whitespace-pre-wrap text-sm">{change.streaming_output}</pre>
              </ScrollArea>
            ) : hasProposal ? (
              <ScrollArea className="h-full rounded-md border p-4">
                <pre className="whitespace-pre-wrap text-sm">{change.proposal}</pre>
              </ScrollArea>
            ) : (
              <div className="flex h-full flex-col items-center justify-center gap-4">
                <FileText className="h-12 w-12 text-muted-foreground" />
                <p className="text-muted-foreground">No proposal generated yet</p>
                {canGenerateProposal && (
                  <Button onClick={handleGenerateProposal}>
                    <Play className="mr-2 h-4 w-4" />
                    Generate Proposal
                  </Button>
                )}
              </div>
            )}
          </TabsContent>

          <TabsContent value="plan" className="h-[calc(100%-50px)]">
            {isPlanning && !hasProposal && change.streaming_output ? (
              <ScrollArea className="h-full rounded-md border p-4">
                <div className="flex items-center gap-2 mb-2 text-yellow-600">
                  <Clock className="h-4 w-4 animate-spin" />
                  <span className="text-sm font-medium">Generating plan...</span>
                </div>
                <pre className="whitespace-pre-wrap text-sm">{change.streaming_output}</pre>
              </ScrollArea>
            ) : hasPlan ? (
              <ScrollArea className="h-full rounded-md border p-4">
                <pre className="whitespace-pre-wrap text-sm">{change.plan}</pre>
              </ScrollArea>
            ) : (
              <div className="flex h-full flex-col items-center justify-center gap-4">
                <FileText className="h-12 w-12 text-muted-foreground" />
                <p className="text-muted-foreground">
                  {hasProposal ? 'No plan generated yet' : 'Generate a proposal first'}
                </p>
                {canGeneratePlan && (
                  <Button onClick={handleGeneratePlan}>
                    <Play className="mr-2 h-4 w-4" />
                    Generate Plan
                  </Button>
                )}
              </div>
            )}
          </TabsContent>

          <TabsContent value="implementation" className="h-[calc(100%-50px)]">
            {isImplementing && change.streaming_output ? (
              <ScrollArea className="h-full rounded-md border p-4">
                <div className="flex items-center gap-2 mb-2 text-blue-600">
                  <Rocket className="h-4 w-4 animate-pulse" />
                  <span className="text-sm font-medium">Implementing...</span>
                </div>
                <pre className="whitespace-pre-wrap text-sm">{change.streaming_output}</pre>
              </ScrollArea>
            ) : change.status === 'done' ? (
              <ScrollArea className="h-full rounded-md border p-4">
                <div className="flex items-center gap-2 mb-2 text-green-600">
                  <Check className="h-4 w-4" />
                  <span className="text-sm font-medium">Implementation Complete</span>
                </div>
                <pre className="whitespace-pre-wrap text-sm">{change.streaming_output}</pre>
              </ScrollArea>
            ) : change.status === 'failed' ? (
              <ScrollArea className="h-full rounded-md border p-4">
                <div className="flex items-center gap-2 mb-2 text-red-600">
                  <X className="h-4 w-4" />
                  <span className="text-sm font-medium">Implementation Failed</span>
                </div>
                <pre className="whitespace-pre-wrap text-sm">{change.streaming_output}</pre>
              </ScrollArea>
            ) : (
              <div className="flex h-full flex-col items-center justify-center gap-4">
                <Rocket className="h-12 w-12 text-muted-foreground" />
                <p className="text-muted-foreground">
                  {hasPlan ? 'Ready to implement' : 'Generate a plan first'}
                </p>
                {canExecute && (
                  <Button onClick={handleExecutePlan} className="bg-blue-600 hover:bg-blue-700">
                    <Rocket className="mr-2 h-4 w-4" />
                    Execute Plan
                  </Button>
                )}
              </div>
            )}
          </TabsContent>
        </Tabs>

        {/* Action Buttons */}
        <div className="mt-4 flex gap-2 border-t pt-4">
          {canApprove && (
            <Button onClick={handleApprovePlan} className="bg-green-600 hover:bg-green-700">
              <Check className="mr-2 h-4 w-4" />
              Approve Plan
            </Button>
          )}
          {canExecute && (
            <Button onClick={handleExecutePlan} className="bg-blue-600 hover:bg-blue-700">
              <Rocket className="mr-2 h-4 w-4" />
              Execute Plan
            </Button>
          )}
          {isImplementing && (
            <Badge variant="secondary" className="px-3 py-1">
              <Rocket className="mr-2 h-4 w-4 animate-pulse" />
              Implementing...
            </Badge>
          )}
          {canSyncAndArchive && (
            <>
              <Button onClick={handleSyncContext} variant="outline">
                <RefreshCw className="mr-2 h-4 w-4" />
                Sync to Context
              </Button>
              <Button onClick={handleArchive} className="bg-blue-600 hover:bg-blue-700">
                <Archive className="mr-2 h-4 w-4" />
                Archive
              </Button>
            </>
          )}
          {isArchived && (
            <Badge variant="secondary" className="px-3 py-1">
              <Archive className="mr-2 h-4 w-4" />
              Archived
            </Badge>
          )}
          {canCancel && (
            <Button variant="destructive" onClick={handleCancelChange}>
              <X className="mr-2 h-4 w-4" />
              Cancel Change
            </Button>
          )}
        </div>
      </CardContent>
    </Card>
  )
}
