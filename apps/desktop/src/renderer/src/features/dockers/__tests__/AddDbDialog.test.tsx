import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import { AddDbDialog } from '../AddDbDialog'

describe('AddDbDialog', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('renders Add DB button', () => {
    render(<AddDbDialog serviceId="rstn-postgres" serviceName="PostgreSQL" />)
    expect(screen.getByText('Add DB')).toBeInTheDocument()
  })

  it('disables button when disabled prop is true', () => {
    render(<AddDbDialog serviceId="rstn-postgres" serviceName="PostgreSQL" disabled={true} />)
    expect(screen.getByText('Add DB')).toBeDisabled()
  })

  it('opens dialog when button clicked', () => {
    render(<AddDbDialog serviceId="rstn-postgres" serviceName="PostgreSQL" />)
    fireEvent.click(screen.getByText('Add DB'))
    expect(screen.getByRole('heading', { name: 'Create Database' })).toBeInTheDocument()
    expect(screen.getByText('Create a new database in PostgreSQL')).toBeInTheDocument()
  })

  it('shows database name input', () => {
    render(<AddDbDialog serviceId="rstn-postgres" serviceName="PostgreSQL" />)
    fireEvent.click(screen.getByText('Add DB'))
    expect(screen.getByLabelText('Database Name')).toBeInTheDocument()
  })

  it('disables submit button when name is empty', async () => {
    render(<AddDbDialog serviceId="rstn-postgres" serviceName="PostgreSQL" />)
    fireEvent.click(screen.getByText('Add DB'))
    const submitButton = screen.getByRole('button', { name: 'Create Database' })
    expect(submitButton).toBeDisabled()
  })

  it('shows error for invalid database name', async () => {
    render(<AddDbDialog serviceId="rstn-postgres" serviceName="PostgreSQL" />)
    fireEvent.click(screen.getByText('Add DB'))

    const input = screen.getByLabelText('Database Name')
    fireEvent.change(input, { target: { value: '123invalid' } })
    fireEvent.click(screen.getByRole('button', { name: 'Create Database' }))

    await waitFor(() => {
      expect(screen.getByText(/must start with a letter or underscore/)).toBeInTheDocument()
    })
  })

  it('accepts valid database name with alphanumeric and underscores', async () => {
    render(<AddDbDialog serviceId="rstn-postgres" serviceName="PostgreSQL" />)
    fireEvent.click(screen.getByText('Add DB'))

    const input = screen.getByLabelText('Database Name')
    fireEvent.change(input, { target: { value: 'my_database_123' } })
    fireEvent.click(screen.getByRole('button', { name: 'Create Database' }))

    // Should show connection string after successful creation
    await waitFor(() => {
      expect(screen.getByText(/created successfully/)).toBeInTheDocument()
    })
  })

  it('calls onCreateDb when provided', async () => {
    const onCreateDb = vi.fn().mockResolvedValue('postgresql://localhost/testdb')
    render(
      <AddDbDialog
        serviceId="rstn-postgres"
        serviceName="PostgreSQL"
        onCreateDb={onCreateDb}
      />
    )

    fireEvent.click(screen.getByText('Add DB'))
    const input = screen.getByLabelText('Database Name')
    fireEvent.change(input, { target: { value: 'testdb' } })
    fireEvent.click(screen.getByRole('button', { name: 'Create Database' }))

    await waitFor(() => {
      expect(onCreateDb).toHaveBeenCalledWith('rstn-postgres', 'testdb')
    })
  })

  it('shows connection string after successful creation', async () => {
    render(<AddDbDialog serviceId="rstn-postgres" serviceName="PostgreSQL" />)
    fireEvent.click(screen.getByText('Add DB'))

    const input = screen.getByLabelText('Database Name')
    fireEvent.change(input, { target: { value: 'mydb' } })
    fireEvent.click(screen.getByRole('button', { name: 'Create Database' }))

    await waitFor(() => {
      expect(screen.getByDisplayValue(/postgresql:\/\/postgres:postgres@localhost:5432\/mydb/)).toBeInTheDocument()
    })
  })

  it('copies connection string to clipboard', async () => {
    render(<AddDbDialog serviceId="rstn-postgres" serviceName="PostgreSQL" />)
    fireEvent.click(screen.getByText('Add DB'))

    const input = screen.getByLabelText('Database Name')
    fireEvent.change(input, { target: { value: 'mydb' } })
    fireEvent.click(screen.getByRole('button', { name: 'Create Database' }))

    await waitFor(() => {
      expect(screen.getByDisplayValue(/postgresql/)).toBeInTheDocument()
    })

    // Find and click copy button (the one with only an icon, no text)
    const buttons = screen.getAllByRole('button')
    const copyButton = buttons.find(b => b.querySelector('svg') && !b.textContent?.includes('Close'))
    expect(copyButton).toBeDefined()
    if (copyButton) {
      fireEvent.click(copyButton)
      await waitFor(() => {
        expect(navigator.clipboard.writeText).toHaveBeenCalledWith(
          'postgresql://postgres:postgres@localhost:5432/mydb'
        )
      })
    }
  })

  it('resets state when dialog is closed', async () => {
    render(<AddDbDialog serviceId="rstn-postgres" serviceName="PostgreSQL" />)
    fireEvent.click(screen.getByText('Add DB'))

    const input = screen.getByLabelText('Database Name')
    fireEvent.change(input, { target: { value: 'mydb' } })

    // Close by clicking close button or pressing escape
    const closeButton = screen.getByRole('button', { name: /close/i })
    if (closeButton) {
      fireEvent.click(closeButton)
    }

    // Reopen and check state is reset
    fireEvent.click(screen.getByText('Add DB'))
    const newInput = screen.getByLabelText('Database Name')
    expect(newInput).toHaveValue('')
  })

  it('shows error when onCreateDb fails', async () => {
    const onCreateDb = vi.fn().mockRejectedValue(new Error('Connection failed'))
    render(
      <AddDbDialog
        serviceId="rstn-postgres"
        serviceName="PostgreSQL"
        onCreateDb={onCreateDb}
      />
    )

    fireEvent.click(screen.getByText('Add DB'))
    const input = screen.getByLabelText('Database Name')
    fireEvent.change(input, { target: { value: 'testdb' } })
    fireEvent.click(screen.getByRole('button', { name: 'Create Database' }))

    await waitFor(() => {
      expect(screen.getByText('Connection failed')).toBeInTheDocument()
    })
  })
})
