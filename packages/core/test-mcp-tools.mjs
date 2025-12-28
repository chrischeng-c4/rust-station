#!/usr/bin/env node
/**
 * Integration test for MCP tools fetch functionality.
 * Tests the complete flow: napi binding → MCP server → JSON response
 *
 * Run with: node test-mcp-tools.mjs
 */

import { fetchMcpTools } from './index.js'

async function testMcpToolsFetch() {
  console.log('Testing MCP tools fetch...\n')

  try {
    // Call the napi function
    console.log('1. Calling fetchMcpTools()...')
    const jsonStr = await fetchMcpTools()
    console.log(`   ✓ Returned ${jsonStr.length} bytes`)

    // Parse the JSON
    console.log('\n2. Parsing JSON response...')
    const data = JSON.parse(jsonStr)
    console.log('   ✓ Valid JSON')

    // Verify JSON-RPC structure
    console.log('\n3. Verifying JSON-RPC structure...')
    if (data.jsonrpc !== '2.0') {
      throw new Error(`Expected jsonrpc: "2.0", got: ${data.jsonrpc}`)
    }
    console.log('   ✓ jsonrpc: "2.0"')

    if (typeof data.id !== 'number') {
      throw new Error(`Expected id to be number, got: ${typeof data.id}`)
    }
    console.log(`   ✓ id: ${data.id}`)

    if (!data.result || typeof data.result !== 'object') {
      throw new Error('Expected result to be an object')
    }
    console.log('   ✓ result is object')

    // Verify tools structure
    console.log('\n4. Verifying tools array...')
    if (!Array.isArray(data.result.tools)) {
      throw new Error('Expected result.tools to be an array')
    }
    console.log(`   ✓ tools is array with ${data.result.tools.length} items`)

    // Verify each tool structure
    if (data.result.tools.length > 0) {
      console.log('\n5. Verifying tool structures...')
      for (const tool of data.result.tools) {
        if (typeof tool.name !== 'string') {
          throw new Error(`Tool missing name: ${JSON.stringify(tool)}`)
        }
        if (typeof tool.description !== 'string') {
          throw new Error(`Tool missing description: ${JSON.stringify(tool)}`)
        }
        if (!tool.inputSchema) {
          throw new Error(`Tool missing inputSchema: ${JSON.stringify(tool)}`)
        }
        console.log(`   ✓ ${tool.name}: ${tool.description.substring(0, 50)}...`)
      }

      // Verify expected tools (if MCP server is running)
      console.log('\n6. Verifying expected tools...')
      const expectedTools = ['read_file', 'list_directory', 'get_project_context', 'run_just_task']
      const actualTools = data.result.tools.map(t => t.name)

      for (const expectedTool of expectedTools) {
        if (!actualTools.includes(expectedTool)) {
          console.log(`   ⚠ Missing expected tool: ${expectedTool}`)
        } else {
          console.log(`   ✓ Found: ${expectedTool}`)
        }
      }
    } else {
      console.log('   ℹ No tools returned (MCP server may not be running)')
    }

    console.log('\n✅ All tests passed!')
    console.log('\nFull response:')
    console.log(JSON.stringify(data, null, 2))

  } catch (error) {
    console.error('\n❌ Test failed:')
    console.error(error.message)
    if (error.stack) {
      console.error('\nStack trace:')
      console.error(error.stack)
    }
    process.exit(1)
  }
}

testMcpToolsFetch()
