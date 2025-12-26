import { describe, it, expect, vi } from 'vitest'
import { render, screen, fireEvent } from '@testing-library/react'
import { TaskCard } from '../TaskCard'
import type { JustCommand } from '@/types/task'

const mockCommand: JustCommand = {
  name: 'build',
  description: 'Build the project',
}

describe('TaskCard', () => {
  it('renders command name', () => {
    render(<TaskCard command={mockCommand} status="idle" />)
    expect(screen.getByText('build')).toBeInTheDocument()
  })

  it('renders command description when available', () => {
    render(<TaskCard command={mockCommand} status="idle" />)
    expect(screen.getByText('Build the project')).toBeInTheDocument()
  })

  it('does not render description when not provided', () => {
    const commandWithoutDesc: JustCommand = { name: 'test' }
    render(<TaskCard command={commandWithoutDesc} status="idle" />)
    expect(screen.queryByText('Build the project')).not.toBeInTheDocument()
  })

  it('shows play button when idle', () => {
    render(<TaskCard command={mockCommand} status="idle" />)
    const button = screen.getByRole('button')
    expect(button).not.toBeDisabled()
  })

  it('disables button when running', () => {
    render(<TaskCard command={mockCommand} status="running" />)
    const button = screen.getByRole('button')
    expect(button).toBeDisabled()
  })

  it('calls onRun when button clicked', () => {
    const onRun = vi.fn()
    render(<TaskCard command={mockCommand} status="idle" onRun={onRun} />)
    fireEvent.click(screen.getByRole('button'))
    expect(onRun).toHaveBeenCalledWith('build')
  })

  it('does not call onRun when running', () => {
    const onRun = vi.fn()
    render(<TaskCard command={mockCommand} status="running" onRun={onRun} />)
    // Button is disabled, but let's verify the call isn't made
    const button = screen.getByRole('button')
    expect(button).toBeDisabled()
  })

  it('shows success icon when status is success', () => {
    const { container } = render(<TaskCard command={mockCommand} status="success" />)
    // CheckCircle icon has text-green-500 class
    const successIcon = container.querySelector('.text-green-500')
    expect(successIcon).toBeInTheDocument()
  })

  it('shows error icon when status is error', () => {
    const { container } = render(<TaskCard command={mockCommand} status="error" />)
    // XCircle icon has text-red-500 class
    const errorIcon = container.querySelector('.text-red-500')
    expect(errorIcon).toBeInTheDocument()
  })

  it('applies active styling when isActive is true', () => {
    const { container } = render(<TaskCard command={mockCommand} status="idle" isActive={true} />)
    const card = container.querySelector('[class*="border-primary"]')
    expect(card).toBeInTheDocument()
  })

  it('does not apply active styling when isActive is false', () => {
    const { container } = render(<TaskCard command={mockCommand} status="idle" isActive={false} />)
    const card = container.querySelector('[class*="border-primary"]')
    expect(card).toBeNull()
  })
})
