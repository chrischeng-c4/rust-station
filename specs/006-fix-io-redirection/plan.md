# Implementation Plan: Fix I/O Redirection Bug

**Branch**: `006-fix-io-redirection` | **Date**: 2025-11-22 | **Spec**: [spec.md](spec.md)

## Summary

Fix critical bug where I/O redirection operators (`>`, `>>`, `<`) are parsed correctly but not executed. Root cause: `PipelineSegment` struct lacks field to store redirection information, causing parsed redirections to be discarded during pipeline construction. Solution: Add `redirections` field to `PipelineSegment` and ensure redirections flow through entire parsing and execution chain.

## Technical Context

**Language/Version**: Rust 1.75 (edition 2021)
**Primary Dependencies**: tokio (async runtime), nix (Unix syscalls), std::process::Command
**Storage**: N/A (bug fix, no data storage)
**Testing**: cargo test (unit + integration tests)
**Target Platform**: macOS (MVP)
**Project Type**: Single binary (shell implementation)
**Performance Goals**: Redirection overhead <1ms (file open/close time excluded)
**Constraints**: Must not break existing pipeline or job control functionality
**Scale/Scope**: Core shell feature affecting all command execution with redirections

## Constitution Check

### Performance-First (Principle I)
✅ **PASS**: Redirection adds minimal overhead (file operations are unavoidable I/O)
- File descriptor manipulation happens once per command
- No additional allocations in hot path
- Existing tests show no performance regression

### Zero-Config Philosophy (Principle II)
✅ **PASS**: Redirections work automatically, no configuration needed
- Standard POSIX shell syntax (`>`, `>>`, `<`)
- No setup or configuration files required

### Progressive Complexity (Principle III)
✅ **PASS**: Basic redirections are simple; advanced features out of scope
- `>`, `>>`, `<` cover 95% of use cases
- Advanced redirections (2>, &>, etc.) deferred to future

### Modern UX (Principle IV)
✅ **PASS**: Redirections work transparently
- Clear error messages for file permission/not found errors
- Consistent with user expectations from other shells

### Rust-Native (Principle V)
✅ **PASS**: Uses standard Rust ecosystem
- `std::fs::{File, OpenOptions}` for file operations
- `std::process::Stdio` for stream redirection
- No external dependencies required

**Gate Status**: ✅ All gates PASS - proceed with implementation

## Project Structure

### Documentation (this feature)

```text
specs/006-fix-io-redirection/
├── spec.md              # Feature specification
├── plan.md              # This file
├── data-model.md        # Phase 1 output (data structures)
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
crates/rush/
├── src/
│   └── executor/
│       ├── mod.rs           # MODIFY: Add redirections field to PipelineSegment
│       ├── parser.rs        # VERIFY: Redirection parsing (already works)
│       ├── pipeline.rs      # MODIFY: Pass redirections from segments to spawn
│       └── execute.rs       # VERIFY: No changes needed (delegates to pipeline)
└── tests/
    ├── integration/
    │   └── redirection_test.rs  # ADD: Integration tests for all three redirection types
    └── feature_test.rs       # EXISTS: Already has failing tests demonstrating bug
```

**Structure Decision**: Single project structure (rush is a monolithic binary). All changes confined to `crates/rush/src/executor/` module.

## Complexity Tracking

No violations - this is a bug fix restoring expected functionality, not adding new complexity.

## Deployment Strategy

### Pull Request Plan

**Strategy**: Single PR (bug fix, estimated ~150 lines changed)

```
PR #1: Fix I/O redirection bug
  - Add redirections field to PipelineSegment struct
  - Update parser to populate redirections in segments
  - Update pipeline executor to apply redirections
  - Add/update integration tests
  - Verify existing tests still pass
  - Target: ~150 lines (well under 500-line limit)
```

**Rationale**: This is a focused bug fix affecting a single module. All changes are tightly coupled (can't add field without updating parser/executor). Splitting would create broken intermediate states.

### Merge Strategy

1. Create PR from `006-fix-io-redirection` branch
2. All tests must pass (including new redirection tests)
3. Merge to `main` after review
4. Tag as bug fix in release notes

## Phase 0: Research

**Status**: NOT NEEDED (bug fix, no unknowns)

All technical decisions are predetermined:
- Redirection types already defined in codebase (`RedirectionType` enum)
- File operation patterns established (`File::create`, `OpenOptions`)
- Integration with `std::process::Command` is standard Rust

## Phase 1: Design & Data Model

### Data Model Changes

See [data-model.md](data-model.md) for complete entity definitions.

**Key Change**: Add `redirections` field to `PipelineSegment`

```rust
pub struct PipelineSegment {
    pub program: String,
    pub args: Vec<String>,
    pub index: usize,
    pub redirections: Vec<Redirection>,  // NEW FIELD
}
```

**Impact Analysis**:
- ✅ Backward compatible (can initialize with empty vec for no redirections)
- ✅ Existing tests unaffected (zero redirections is valid state)
- ✅ No performance impact (Vec is zero-cost when empty)

### Component Integration

```text
Parser (parser.rs)
  ↓ produces
PipelineSegment { redirections: Vec<Redirection> }
  ↓ consumed by
Pipeline Executor (pipeline.rs)
  ↓ applies redirections via
std::process::Command::stdout/stdin/stderr
```

### Error Handling

Redirection errors handled at execution time:
- **File not found** (input): `rush: file.txt: No such file or directory`
- **Permission denied** (output): `rush: file.txt: Permission denied`
- **Is a directory** (output): `rush: /tmp: Is a directory`

Errors returned as `RushError::Redirection(String)` with clear user message.

## Phase 2: Tasks

See [tasks.md](tasks.md) - generated by `/speckit.tasks` command (not created here).

## Dependencies & Risks

### Dependencies
- ✅ Parser already extracts redirection info - VERIFIED
- ✅ `Redirection` and `RedirectionType` types already defined - VERIFIED
- ✅ Pipeline executor has redirection handling code - VERIFIED (but not called)

### Risks
- ⚠️ **Risk**: Breaking existing pipeline or job control functionality
  - **Mitigation**: Comprehensive test suite, all existing tests must pass
- ⚠️ **Risk**: Interaction with background jobs (`&`)
  - **Mitigation**: Test combinations like `sleep 10 > /tmp/out.txt &`
- ⚠️ **Risk**: Interaction with pipelines (`|`)
  - **Mitigation**: Test combinations like `ls | grep txt > results.txt`

### Success Criteria Verification

From spec.md:
- **SC-001**: Integration tests pass ✅ (will add comprehensive tests)
- **SC-002**: `echo hello > file.txt` works <100ms ✅ (file I/O dominates, shell overhead <1ms)
- **SC-003**: Works with pipelines ✅ (existing pipeline code handles this)
- **SC-004**: Clear error messages ✅ (already implemented in pipeline.rs)

## Implementation Notes

### Key Files to Modify

1. **`crates/rush/src/executor/mod.rs`** (~5 lines)
   - Add `redirections: Vec<Redirection>` field to `PipelineSegment` struct
   - Update `PipelineSegment::new()` constructor

2. **`crates/rush/src/executor/parser.rs`** (~20 lines)
   - Update `split_into_segments()` to extract and store redirections
   - Currently redirections are parsed but not passed to segments

3. **`crates/rush/src/executor/pipeline.rs`** (~30 lines)
   - Update `execute_single_command()` to use `segment.redirections`
   - Remove `extract_redirections_from_args()` call (redundant)
   - Redirections already extracted by parser

4. **`crates/rush/tests/feature_test.rs`** (~50 lines)
   - Fix existing failing tests (they already demonstrate the bug)
   - Add comprehensive test coverage

5. **Documentation** (~10 lines)
   - Update `TEST_COVERAGE.md` with new test counts

**Total Estimate**: ~115 lines changed (well under 500-line PR limit)

### Testing Strategy

1. **Unit Tests**: Test `PipelineSegment` with redirections field
2. **Integration Tests**: Test each redirection type independently
3. **Combination Tests**: Test redirections + pipes, redirections + background
4. **Error Tests**: Test file errors (not found, permission denied, is directory)

### Validation

Before merging:
- [ ] All existing tests pass (247 tests currently)
- [ ] New redirection tests pass (3 user stories × 3 acceptance scenarios = 9 tests minimum)
- [ ] No performance regression (run benchmarks)
- [ ] Documentation updated (TEST_COVERAGE.md)
