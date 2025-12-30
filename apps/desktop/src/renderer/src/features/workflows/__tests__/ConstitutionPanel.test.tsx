import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import { ConstitutionPanel } from '../ConstitutionPanel'

// Mock useAppState hook
const mockDispatch = vi.fn()
const mockUseAppState = vi.fn()

vi.mock('@/hooks/useAppState', () => ({
  useAppState: () => mockUseAppState(),
}))

// Helper to create mock state with Constitution workflow
const createMockState = (options: {
  constitutionExists?: boolean | null
  workflow?: {
    status: 'collecting' | 'generating' | 'complete'
    currentQuestion?: number
    answers?: Record<string, string>
    output?: string
  } | null
}) => ({
  active_project_index: 0,
  projects: [
    {
      id: 'test-project',
      path: '/test/project',
      active_worktree_index: 0,
      worktrees: [
        {
          path: '/test/project',
          branch: 'main',
          is_main: true,
          tasks: {
            commands: [],
            task_statuses: {},
            active_command: 'constitution-init',
            output: [],
            is_loading: false,
            error: null,
            constitution_exists: options.constitutionExists ?? null,
            constitution_workflow: options.workflow
              ? {
                  current_question: options.workflow.currentQuestion ?? 0,
                  answers: options.workflow.answers ?? {},
                  output: options.workflow.output ?? '',
                  status: options.workflow.status,
                }
              : null,
          },
          chat: { messages: [], is_typing: false, error: null },
          mcp: {
            status: 'stopped',
            port: null,
            config_path: null,
            error: null,
            logs: [],
            available_tools: [],
          },
          terminal: null,
        },
      ],
      agent_rules_config: {
        enabled: false,
        custom_prompt: '',
        temp_file_path: null,
        profiles: [],
        active_profile_id: null,
      },
      env_config: {
        tracked_patterns: [],
        auto_copy_on_create: false,
        source_worktree_path: null,
        last_copy_result: null,
      },
    },
  ],
})

describe('ConstitutionPanel', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    mockDispatch.mockResolvedValue(undefined)
  })

  describe('Mount Behavior', () => {
    it('dispatches ClearConstitutionWorkflow and CheckConstitutionExists on mount', async () => {
      mockUseAppState.mockReturnValue({
        state: createMockState({ constitutionExists: null }),
        dispatch: mockDispatch,
        isLoading: false,
      })

      render(<ConstitutionPanel />)

      await waitFor(() => {
        expect(mockDispatch).toHaveBeenCalledWith({ type: 'ClearConstitutionWorkflow' })
        expect(mockDispatch).toHaveBeenCalledWith({ type: 'CheckConstitutionExists' })
      })
    })

    it('shows loading spinner while constitution_exists is null', () => {
      mockUseAppState.mockReturnValue({
        state: createMockState({ constitutionExists: null }),
        dispatch: mockDispatch,
        isLoading: false,
      })

      render(<ConstitutionPanel />)
      expect(screen.getByText(/Checking constitution.../)).toBeInTheDocument()
    })
  })

  describe('Constitution Exists State', () => {
    it('shows success message when constitution exists', () => {
      mockUseAppState.mockReturnValue({
        state: createMockState({ constitutionExists: true }),
        dispatch: mockDispatch,
        isLoading: false,
      })

      render(<ConstitutionPanel />)
      expect(screen.getByText(/Constitution Exists/)).toBeInTheDocument()
      expect(screen.getByText(/\.rstn\/constitutions\//)).toBeInTheDocument()
    })

    it('shows Regenerate button when constitution exists', () => {
      mockUseAppState.mockReturnValue({
        state: createMockState({ constitutionExists: true }),
        dispatch: mockDispatch,
        isLoading: false,
      })

      render(<ConstitutionPanel />)
      expect(screen.getByRole('button', { name: /Regenerate with Q&A/ })).toBeInTheDocument()
    })
  })

  describe('Constitution Missing State', () => {
    it('shows initialization options when constitution does not exist', () => {
      mockUseAppState.mockReturnValue({
        state: createMockState({ constitutionExists: false }),
        dispatch: mockDispatch,
        isLoading: false,
      })

      render(<ConstitutionPanel />)
      expect(screen.getByText(/Initialize Constitution/)).toBeInTheDocument()
      expect(screen.getByRole('button', { name: /Apply Default Template/ })).toBeInTheDocument()
      expect(screen.getByRole('button', { name: /Create with Q&A/ })).toBeInTheDocument()
    })

    it('calls dispatch when Apply Default Template is clicked', async () => {
      mockUseAppState.mockReturnValue({
        state: createMockState({ constitutionExists: false }),
        dispatch: mockDispatch,
        isLoading: false,
      })

      render(<ConstitutionPanel />)
      const applyButton = screen.getByRole('button', { name: /Apply Default Template/ })
      fireEvent.click(applyButton)

      await waitFor(() => {
        expect(mockDispatch).toHaveBeenCalledWith({ type: 'ApplyDefaultConstitution' })
      })
    })

    it('calls dispatch when Create with Q&A is clicked', async () => {
      mockUseAppState.mockReturnValue({
        state: createMockState({ constitutionExists: false }),
        dispatch: mockDispatch,
        isLoading: false,
      })

      render(<ConstitutionPanel />)
      const qaButton = screen.getByRole('button', { name: /Create with Q&A/ })
      fireEvent.click(qaButton)

      await waitFor(() => {
        expect(mockDispatch).toHaveBeenCalledWith({ type: 'StartConstitutionWorkflow' })
      })
    })
  })

  describe('Collecting Phase', () => {
    it('renders first question in collecting phase', () => {
      mockUseAppState.mockReturnValue({
        state: createMockState({
          constitutionExists: false,
          workflow: { status: 'collecting', currentQuestion: 0 },
        }),
        dispatch: mockDispatch,
        isLoading: false,
      })

      render(<ConstitutionPanel />)
      // Use heading role to find the question title specifically (not the progress list)
      expect(screen.getByRole('heading', { name: /What technology stack does this project use/ })).toBeInTheDocument()
    })

    it('shows progress indicator with question count', () => {
      mockUseAppState.mockReturnValue({
        state: createMockState({
          constitutionExists: false,
          workflow: { status: 'collecting', currentQuestion: 1, answers: { tech_stack: 'React' } },
        }),
        dispatch: mockDispatch,
        isLoading: false,
      })

      render(<ConstitutionPanel />)
      expect(screen.getByText('1 / 4')).toBeInTheDocument()
    })

    it('shows checkmarks for answered questions', () => {
      mockUseAppState.mockReturnValue({
        state: createMockState({
          constitutionExists: false,
          workflow: {
            status: 'collecting',
            currentQuestion: 2,
            answers: { tech_stack: 'React', security: 'JWT' },
          },
        }),
        dispatch: mockDispatch,
        isLoading: false,
      })

      const { container } = render(<ConstitutionPanel />)
      const checkmarks = container.querySelectorAll('.text-green-500')
      expect(checkmarks.length).toBeGreaterThanOrEqual(1)
    })

    it('allows typing answer in textarea', () => {
      mockUseAppState.mockReturnValue({
        state: createMockState({
          constitutionExists: false,
          workflow: { status: 'collecting', currentQuestion: 0 },
        }),
        dispatch: mockDispatch,
        isLoading: false,
      })

      render(<ConstitutionPanel />)
      const textarea = screen.getByPlaceholderText('Type your answer...')
      fireEvent.change(textarea, { target: { value: 'React + Rust' } })
      expect(textarea).toHaveValue('React + Rust')
    })

    it('calls dispatch when Next button clicked', async () => {
      mockUseAppState.mockReturnValue({
        state: createMockState({
          constitutionExists: false,
          workflow: { status: 'collecting', currentQuestion: 0 },
        }),
        dispatch: mockDispatch,
        isLoading: false,
      })

      render(<ConstitutionPanel />)
      const textarea = screen.getByPlaceholderText('Type your answer...')
      const nextButton = screen.getByRole('button', { name: /Next/ })

      fireEvent.change(textarea, { target: { value: 'React + Rust' } })
      fireEvent.click(nextButton)

      await waitFor(() => {
        expect(mockDispatch).toHaveBeenCalledWith({
          type: 'AnswerConstitutionQuestion',
          payload: { answer: 'React + Rust' },
        })
      })
    })

    it('disables Next button when answer is empty', () => {
      mockUseAppState.mockReturnValue({
        state: createMockState({
          constitutionExists: false,
          workflow: { status: 'collecting', currentQuestion: 0 },
        }),
        dispatch: mockDispatch,
        isLoading: false,
      })

      render(<ConstitutionPanel />)
      const nextButton = screen.getByRole('button', { name: /Next/ })
      expect(nextButton).toBeDisabled()
    })

    it('shows Generate button after all questions answered', () => {
      mockUseAppState.mockReturnValue({
        state: createMockState({
          constitutionExists: false,
          workflow: {
            status: 'collecting',
            currentQuestion: 4,
            answers: {
              tech_stack: 'React',
              security: 'JWT',
              code_quality: '80%',
              architecture: 'State-first',
            },
          },
        }),
        dispatch: mockDispatch,
        isLoading: false,
      })

      render(<ConstitutionPanel />)
      expect(screen.getByText(/All questions answered!/)).toBeInTheDocument()
      expect(screen.getByRole('button', { name: /Generate Constitution/ })).toBeInTheDocument()
    })

    it('calls dispatch when Generate Constitution clicked', async () => {
      mockUseAppState.mockReturnValue({
        state: createMockState({
          constitutionExists: false,
          workflow: {
            status: 'collecting',
            currentQuestion: 4,
            answers: {
              tech_stack: 'React',
              security: 'JWT',
              code_quality: '80%',
              architecture: 'State-first',
            },
          },
        }),
        dispatch: mockDispatch,
        isLoading: false,
      })

      render(<ConstitutionPanel />)
      const generateButton = screen.getByRole('button', { name: /Generate Constitution/ })
      fireEvent.click(generateButton)

      await waitFor(() => {
        expect(mockDispatch).toHaveBeenCalledWith({ type: 'GenerateConstitution' })
      })
    })
  })

  describe('Generating Phase', () => {
    it('renders generating state with spinner', () => {
      mockUseAppState.mockReturnValue({
        state: createMockState({
          constitutionExists: false,
          workflow: { status: 'generating', currentQuestion: 4, output: '# Project Constitution\n' },
        }),
        dispatch: mockDispatch,
        isLoading: false,
      })

      render(<ConstitutionPanel />)
      expect(screen.getByText(/Generating Constitution.../)).toBeInTheDocument()
      expect(screen.getByText(/Streaming from Claude Code.../)).toBeInTheDocument()
    })

    it('displays streaming output as markdown', () => {
      mockUseAppState.mockReturnValue({
        state: createMockState({
          constitutionExists: false,
          workflow: {
            status: 'generating',
            currentQuestion: 4,
            output: '# Project Constitution\n\n## Technology Stack\n\nReact + Rust',
          },
        }),
        dispatch: mockDispatch,
        isLoading: false,
      })

      render(<ConstitutionPanel />)
      expect(screen.getByText('Project Constitution')).toBeInTheDocument()
      expect(screen.getByText('Technology Stack')).toBeInTheDocument()
    })

    it('shows waiting message when output is empty', () => {
      mockUseAppState.mockReturnValue({
        state: createMockState({
          constitutionExists: false,
          workflow: { status: 'generating', currentQuestion: 4, output: '' },
        }),
        dispatch: mockDispatch,
        isLoading: false,
      })

      render(<ConstitutionPanel />)
      expect(screen.getByText(/Waiting for Claude.../)).toBeInTheDocument()
    })
  })

  describe('Complete Phase', () => {
    it('renders completion state with success message', () => {
      mockUseAppState.mockReturnValue({
        state: createMockState({
          constitutionExists: false,
          workflow: {
            status: 'complete',
            currentQuestion: 4,
            output: '# Project Constitution\n\nComplete!',
          },
        }),
        dispatch: mockDispatch,
        isLoading: false,
      })

      render(<ConstitutionPanel />)
      expect(screen.getByText(/Constitution Generated/)).toBeInTheDocument()
      expect(screen.getByText(/Constitution saved to/)).toBeInTheDocument()
    })

    it('displays final constitution content', () => {
      const finalContent = '# Project Constitution\n\n## Rules\n\n- Rule 1\n- Rule 2'
      mockUseAppState.mockReturnValue({
        state: createMockState({
          constitutionExists: false,
          workflow: { status: 'complete', currentQuestion: 4, output: finalContent },
        }),
        dispatch: mockDispatch,
        isLoading: false,
      })

      render(<ConstitutionPanel />)
      expect(screen.getByText('Project Constitution')).toBeInTheDocument()
      expect(screen.getByText('Rules')).toBeInTheDocument()
    })

    it('shows success styling in complete state', () => {
      mockUseAppState.mockReturnValue({
        state: createMockState({
          constitutionExists: false,
          workflow: { status: 'complete', currentQuestion: 4, output: '# Done' },
        }),
        dispatch: mockDispatch,
        isLoading: false,
      })

      const { container } = render(<ConstitutionPanel />)
      const successIcons = container.querySelectorAll('.text-green-500')
      expect(successIcons.length).toBeGreaterThanOrEqual(1)
    })
  })

  describe('Loading State', () => {
    it('shows spinner when isLoading is true', () => {
      mockUseAppState.mockReturnValue({
        state: createMockState({ constitutionExists: false }),
        dispatch: mockDispatch,
        isLoading: true,
      })

      render(<ConstitutionPanel />)
      expect(screen.getByText(/Checking constitution.../)).toBeInTheDocument()
    })
  })
})
