# Constitution Workflow - Known Issues

**Date**: 2025-12-29
**Context**: E2E testing and debugging of Constitution Initialization workflow

This document tracks all issues discovered during Constitution workflow implementation and testing.

---

## 🔴 CRITICAL - E2E Test Infrastructure Issues

### Issue #1: Projects Close Immediately After Loading in E2E Tests

**Status**: BLOCKING all E2E tests
**Severity**: Critical
**Component**: State Management / E2E Test Environment

#### Description

When running E2E tests with Playwright, projects load successfully but then close/disappear within 2 seconds. This makes all Constitution workflow E2E tests fail.

#### Reproduction Steps

1. Run E2E test: `pnpm exec playwright test constitution-workflow.spec.ts`
2. Test opens a project using `openProject()` helper
3. State check shows `worktrees.length > 0` (success)
4. UI renders "Commands" panel (success)
5. After 2-second wait, state check shows `active_project = null` (failure)

#### Evidence

```
✓ State check passes: worktrees exist
✓ UI renders: Commands panel visible
✗ After 2s: Project closed - state validation failed
```

Debug output:
```
BEFORE CLICK - Has project: false
BEFORE CLICK - Has worktrees: undefined
```

#### What We Tried

- ✅ Using real rustation project instead of fake test project → Still fails
- ✅ Adding longer wait times (2-5 seconds) → Still fails
- ✅ Waiting for UI elements before proceeding → Still fails
- ✅ Polling state until worktrees exist → Passes, but project closes afterward

#### Root Cause Hypothesis

1. **Async validation failure**: Backend validates git repo/justfile asynchronously and rejects the project
2. **State persistence issue**: State updates don't persist between dispatches in test environment
3. **Worktree loading failure**: Initial load succeeds but worktree loading fails silently
4. **Test isolation problem**: Each test gets a fresh state that doesn't persist

#### Files Involved

- `e2e/test-helpers.ts` (openProject function)
- `packages/core/src/reducer.rs` (OpenProject action handler)
- `packages/core/src/lib.rs` (State management)

#### Next Steps

1. **Add detailed logging** to OpenProject action handler to see why projects close
2. **Check git validation logic** - might be rejecting projects asynchronously
3. **Verify worktree loading** - add logs to worktree enumeration
4. **Test with minimal project** - single file, no git, to isolate issue
5. **Compare E2E environment vs dev** - why does it work in dev but not E2E?

---

## 🟡 MEDIUM - Test Helper Issues

### Issue #2: createTestProject() Creates Invalid Projects

**Status**: Workaround applied (using real project)
**Severity**: Medium
**Component**: E2E Test Helpers

#### Description

The `createTestProject()` helper creates minimal test projects with just `.git/` and `justfile`, but these may be invalid for rustation's requirements.

#### Current Implementation

```typescript
export async function createTestProject(): Promise<string> {
  const tmpDir = await fs.mkdtemp(path.join(os.tmpdir(), 'rstn-test-'))

  // Initialize as git repo (rstn requires git)
  await fs.mkdir(path.join(tmpDir, '.git'))

  // Create a minimal justfile
  await fs.writeFile(path.join(tmpDir, 'justfile'), 'test:\n\techo "test"')

  return tmpDir
}
```

#### Problems

1. Empty `.git/` directory may not be recognized as valid git repo
2. Missing git metadata (HEAD, config, refs)
3. No actual git commits or branches
4. May trigger validation failures

#### Workaround

Tests now use the real rustation project:
```typescript
testProjectPath = '/Users/chrischeng/projects/rustation'
```

#### Proper Fix Needed

Create valid git repos using `git init` command:
```bash
git init
git config user.name "Test User"
git config user.email "test@example.com"
git add .
git commit -m "Initial commit"
```

---

## 🟢 FIXED - UI and State Issues

### Issue #3: Null Check Bug in ConstitutionPanel ✅ FIXED

**Status**: Fixed
**Severity**: High
**Component**: ConstitutionPanel.tsx

#### Description

ConstitutionPanel crashed with `TypeError: Cannot read properties of null (reading 'active_project')` when user clicked "Initialize Constitution".

#### Root Cause

Missing optional chaining before accessing `state.active_project`:

```typescript
// BEFORE (line 18) - BROKEN
const workflow = state.active_project?.worktrees?.[...]

// AFTER (line 18) - FIXED
const workflow = state?.active_project?.worktrees?.[...]
```

#### Fix Applied

**File**: `apps/desktop/src/renderer/src/features/tasks/ConstitutionPanel.tsx`
**Line**: 18
**Change**: Added `?.` before `active_project`

---

### Issue #4: E2E Tests Searching for Wrong Command Name ✅ FIXED

**Status**: Fixed
**Severity**: Medium
**Component**: E2E Tests

#### Description

Tests searched for "Initialize Constitution" but the actual displayed text is "constitution-init" (the command name).

#### Root Cause

TaskCard displays `command.name` for non-Claude Code commands:
```typescript
{isClaudeCode ? 'Claude Code' : command.name}
```

Constitution command definition:
```typescript
const CONSTITUTION_INIT_COMMAND: JustCommandInfo = {
  name: 'constitution-init',  // This is what's displayed
  description: 'Initialize project constitution (CESDD)',
  recipe: '',
}
```

#### Fix Applied

Updated all test selectors:
```typescript
// BEFORE
page.getByText('Initialize Constitution')

// AFTER
page.getByText('constitution-init')
```

---

### Issue #5: E2E Tests Clicking Text Instead of Button ✅ FIXED

**Status**: Fixed
**Severity**: Medium
**Component**: E2E Tests

#### Description

Tests were clicking command name text, which has no click handler. Only the button (play icon) triggers actions.

#### Root Cause

TaskCard structure:
```typescript
<div>  {/* Card container - not clickable */}
  <span>{command.name}</span>  {/* Text - not clickable */}
  <Button onClick={onRun}>     {/* Button - clickable */}
    <Play />
  </Button>
</div>
```

#### Fix Applied

Updated test to click button instead of text:
```typescript
// BEFORE
await page.getByText('constitution-init').click()

// AFTER
const constitutionCard = page.locator('div:has-text("constitution-init")')
const playButton = constitutionCard.locator('button').first()
await playButton.click()
```

---

### Issue #6: Missing Build Before E2E Tests ✅ FIXED

**Status**: Fixed
**Severity**: Medium
**Component**: Build Process

#### Description

E2E tests run against built artifacts (`out/` directory), not live source code. Changes to source weren't reflected in tests until rebuild.

#### Solution

Always rebuild before running E2E tests:
```bash
cd apps/desktop && pnpm build
cd ../e2e && pnpm exec playwright test
```

#### Lesson Learned

E2E workflow:
1. Make source changes
2. **Build desktop app** (`pnpm build`)
3. Run E2E tests
4. Repeat

---

## 🔵 KNOWN LIMITATIONS

### Limitation #1: napi-rs State Initialization Outside Electron

**Status**: Documented
**Component**: Core Package / napi-rs

#### Description

Calling `stateInit()` from standalone Node.js crashes with:
```
Assertion failed: (func) != nullptr
napi_release_threadsafe_function
```

#### Impact

Cannot test state management in isolation - must use full Electron environment.

#### Debug Script Created

`/tmp/debug-constitution.mjs` - crashes when calling `stateInit()`

#### Root Cause

Threadsafe function setup in `stateInit()` requires Electron's renderer process context. Not compatible with standalone Node.js.

#### Workaround

Use Playwright E2E tests which run full Electron app.

---

## 📊 Test Results Summary

### Current Status

| Test | Status | Notes |
|------|--------|-------|
| should display Initialize Constitution command | ✅ PASSING | |
| should show ConstitutionPanel when command is clicked | ❌ FAILING | Project closes after load |
| should enable Next button when answer is typed | ❌ FAILING | Project closes after load |
| should advance through all 4 questions | ❌ FAILING | Project closes after load |
| should show checkmarks for answered questions | ❌ FAILING | Project closes after load |
| should preserve state when navigating away and back | ❌ FAILING | Project closes after load |
| should handle Generate Constitution click | ❌ FAILING | Project closes after load |
| should create constitution.md file after generation | ⏭️ SKIPPED | Requires Claude CLI |

**Pass Rate**: 1/8 (12.5%)
**Blocker**: Issue #1 (Projects close after loading)

---

## 🛠️ Fixes Applied This Session

1. ✅ Fixed null check bug in ConstitutionPanel.tsx
2. ✅ Removed debug logging from ConstitutionPanel.tsx and TasksPage.tsx
3. ✅ Created `e2e/test-helpers.ts` with helper functions
4. ✅ Fixed `openProject()` to poll state instead of waiting for UI
5. ✅ Updated test selectors to search for correct command name
6. ✅ Updated tests to click button instead of text
7. ✅ Rebuilt desktop app before running E2E tests
8. ✅ Added state validation to detect project closure
9. ✅ Added debug logging to trace state changes

---

## 🎯 Immediate Next Steps

### Priority 1: Fix Project Closure Issue (Issue #1)

1. **Add verbose logging** to OpenProject action handler:
   ```rust
   Action::OpenProject { payload } => {
       println!("[DEBUG] OpenProject started: {:?}", payload.path);
       // ... existing code ...
       println!("[DEBUG] OpenProject complete: {} worktrees loaded", worktrees.len());
   }
   ```

2. **Check git validation** in worktree loading:
   - Does it fail silently if git repo is invalid?
   - Add error logging for git operations

3. **Test with different projects**:
   - Minimal project (single file + git init)
   - Project without justfile
   - Project with invalid git repo
   - Identify which validation is failing

4. **Compare dev vs E2E environments**:
   - Why does project stay loaded in dev?
   - What's different about E2E test environment?

### Priority 2: Improve Test Infrastructure

1. **Fix `createTestProject()`** to create valid git repos
2. **Add project validation test** to verify test projects are valid
3. **Add state persistence test** to verify state doesn't reset between actions

### Priority 3: Manual Testing

Since E2E is blocked, verify feature works manually:

1. Start dev server: `cd apps/desktop && pnpm dev`
2. Open rustation project
3. Click "constitution-init" command
4. Verify ConstitutionPanel appears with 4 questions
5. Complete workflow and verify constitution.md is created

---

## 📝 Documentation

### Files Modified

- `apps/desktop/src/renderer/src/features/tasks/ConstitutionPanel.tsx`
- `apps/desktop/src/renderer/src/features/tasks/TasksPage.tsx`
- `e2e/test-helpers.ts` (NEW)
- `e2e/constitution-workflow.spec.ts` (REWRITTEN)

### Files Created

- `/tmp/debug-constitution.mjs` - Debug script for state testing
- `/tmp/test-dispatch.mjs` - Test napi-rs exports

### Test Coverage

- Unit tests: None (frontend components)
- Integration tests: None
- E2E tests: 8 tests (1 passing, 7 blocked by Issue #1)

---

## 🤔 Questions for Investigation

1. **Why do projects close after loading in E2E tests but not in dev?**
2. **What validation runs asynchronously after OpenProject?**
3. **Does worktree enumeration fail silently?**
4. **Is there a state reset happening between dispatches?**
5. **Do E2E tests get a fresh state for each test?**

---

## 💡 Lessons Learned

### What Worked

- State-first debugging: Checking state directly revealed issues faster than UI testing
- Incremental fixes: Fixing one issue at a time made progress measurable
- Real projects: Using actual rustation project eliminated "invalid test project" variable

### What Didn't Work

- Fake test projects: Empty .git directories aren't valid
- Longer wait times: Project closure happens regardless of wait duration
- UI-based waiting: Waiting for UI elements doesn't guarantee state is stable

### Best Practices Going Forward

1. **Always check state**, not just UI
2. **Validate test fixtures** - ensure test projects are valid
3. **Add logging early** - don't wait until things fail
4. **Test infrastructure first** - verify helpers work before writing tests
5. **Manual test critical paths** - E2E can't catch everything

---

## 📞 Contact / Next Session

When resuming work on this issue:

1. Read this document first
2. Focus on Issue #1 (project closure)
3. Add logging to OpenProject handler
4. Run single test with verbose output
5. Compare behavior in dev vs E2E

**Key Files to Review**:
- `packages/core/src/reducer.rs` (OpenProject handler, line ~200-250)
- `packages/core/src/lib.rs` (state_init, state_dispatch functions)
- `e2e/test-helpers.ts` (openProject function, line 9-49)

**Last Known State**: 1/8 tests passing, blocked by project closure issue
