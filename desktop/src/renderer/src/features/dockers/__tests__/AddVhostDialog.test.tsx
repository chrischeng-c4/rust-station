import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import { AddVhostDialog } from '../AddVhostDialog'

describe('AddVhostDialog', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('renders Add vhost button', () => {
    render(<AddVhostDialog serviceId="rstn-rabbitmq" />)
    expect(screen.getByText('Add vhost')).toBeInTheDocument()
  })

  it('disables button when disabled prop is true', () => {
    render(<AddVhostDialog serviceId="rstn-rabbitmq" disabled={true} />)
    expect(screen.getByText('Add vhost')).toBeDisabled()
  })

  it('opens dialog when button clicked', () => {
    render(<AddVhostDialog serviceId="rstn-rabbitmq" />)
    fireEvent.click(screen.getByText('Add vhost'))
    expect(screen.getByText('Create Virtual Host')).toBeInTheDocument()
    expect(screen.getByText('Create a new virtual host in RabbitMQ')).toBeInTheDocument()
  })

  it('shows vhost name input', () => {
    render(<AddVhostDialog serviceId="rstn-rabbitmq" />)
    fireEvent.click(screen.getByText('Add vhost'))
    expect(screen.getByLabelText('Vhost Name')).toBeInTheDocument()
  })

  it('disables submit button when name is empty', async () => {
    render(<AddVhostDialog serviceId="rstn-rabbitmq" />)
    fireEvent.click(screen.getByText('Add vhost'))
    const submitButton = screen.getByRole('button', { name: 'Create Vhost' })
    expect(submitButton).toBeDisabled()
  })

  it('shows error for invalid vhost name starting with number', async () => {
    render(<AddVhostDialog serviceId="rstn-rabbitmq" />)
    fireEvent.click(screen.getByText('Add vhost'))

    const input = screen.getByLabelText('Vhost Name')
    fireEvent.change(input, { target: { value: '123invalid' } })
    fireEvent.click(screen.getByRole('button', { name: 'Create Vhost' }))

    await waitFor(() => {
      expect(screen.getByText(/must start with a letter or underscore/)).toBeInTheDocument()
    })
  })

  it('accepts valid vhost name with alphanumeric, underscores, and hyphens', async () => {
    render(<AddVhostDialog serviceId="rstn-rabbitmq" />)
    fireEvent.click(screen.getByText('Add vhost'))

    const input = screen.getByLabelText('Vhost Name')
    fireEvent.change(input, { target: { value: 'my_vhost-123' } })
    fireEvent.click(screen.getByRole('button', { name: 'Create Vhost' }))

    // Should show connection string after successful creation
    await waitFor(() => {
      expect(screen.getByText(/created successfully/)).toBeInTheDocument()
    })
  })

  it('calls onCreateVhost when provided', async () => {
    const onCreateVhost = vi.fn().mockResolvedValue('amqp://localhost/testvhost')
    render(<AddVhostDialog serviceId="rstn-rabbitmq" onCreateVhost={onCreateVhost} />)

    fireEvent.click(screen.getByText('Add vhost'))
    const input = screen.getByLabelText('Vhost Name')
    fireEvent.change(input, { target: { value: 'testvhost' } })
    fireEvent.click(screen.getByRole('button', { name: 'Create Vhost' }))

    await waitFor(() => {
      expect(onCreateVhost).toHaveBeenCalledWith('rstn-rabbitmq', 'testvhost')
    })
  })

  it('shows connection string after successful creation', async () => {
    render(<AddVhostDialog serviceId="rstn-rabbitmq" />)
    fireEvent.click(screen.getByText('Add vhost'))

    const input = screen.getByLabelText('Vhost Name')
    fireEvent.change(input, { target: { value: 'myvhost' } })
    fireEvent.click(screen.getByRole('button', { name: 'Create Vhost' }))

    await waitFor(() => {
      expect(screen.getByDisplayValue(/amqp:\/\/guest:guest@localhost:5672\/myvhost/)).toBeInTheDocument()
    })
  })

  it('copies connection string to clipboard', async () => {
    render(<AddVhostDialog serviceId="rstn-rabbitmq" />)
    fireEvent.click(screen.getByText('Add vhost'))

    const input = screen.getByLabelText('Vhost Name')
    fireEvent.change(input, { target: { value: 'myvhost' } })
    fireEvent.click(screen.getByRole('button', { name: 'Create Vhost' }))

    await waitFor(() => {
      expect(screen.getByDisplayValue(/amqp/)).toBeInTheDocument()
    })

    // Find and click copy button (the one with only an icon, no text)
    const buttons = screen.getAllByRole('button')
    const copyButton = buttons.find(b => b.querySelector('svg') && !b.textContent?.includes('Close'))
    expect(copyButton).toBeDefined()
    if (copyButton) {
      fireEvent.click(copyButton)
      await waitFor(() => {
        expect(navigator.clipboard.writeText).toHaveBeenCalledWith(
          'amqp://guest:guest@localhost:5672/myvhost'
        )
      })
    }
  })

  it('resets state when dialog is closed', async () => {
    render(<AddVhostDialog serviceId="rstn-rabbitmq" />)
    fireEvent.click(screen.getByText('Add vhost'))

    const input = screen.getByLabelText('Vhost Name')
    fireEvent.change(input, { target: { value: 'myvhost' } })

    // Close by clicking close button
    const closeButton = screen.getByRole('button', { name: /close/i })
    if (closeButton) {
      fireEvent.click(closeButton)
    }

    // Reopen and check state is reset
    fireEvent.click(screen.getByText('Add vhost'))
    const newInput = screen.getByLabelText('Vhost Name')
    expect(newInput).toHaveValue('')
  })

  it('shows error when onCreateVhost fails', async () => {
    const onCreateVhost = vi.fn().mockRejectedValue(new Error('Connection failed'))
    render(<AddVhostDialog serviceId="rstn-rabbitmq" onCreateVhost={onCreateVhost} />)

    fireEvent.click(screen.getByText('Add vhost'))
    const input = screen.getByLabelText('Vhost Name')
    fireEvent.change(input, { target: { value: 'testvhost' } })
    fireEvent.click(screen.getByRole('button', { name: 'Create Vhost' }))

    await waitFor(() => {
      expect(screen.getByText('Connection failed')).toBeInTheDocument()
    })
  })

  it('disables input while creating', async () => {
    const onCreateVhost = vi.fn().mockImplementation(() => new Promise(() => {})) // Never resolves
    render(<AddVhostDialog serviceId="rstn-rabbitmq" onCreateVhost={onCreateVhost} />)

    fireEvent.click(screen.getByText('Add vhost'))
    const input = screen.getByLabelText('Vhost Name')
    fireEvent.change(input, { target: { value: 'testvhost' } })
    fireEvent.click(screen.getByRole('button', { name: 'Create Vhost' }))

    await waitFor(() => {
      expect(screen.getByText('Creating...')).toBeInTheDocument()
      expect(input).toBeDisabled()
    })
  })
})
