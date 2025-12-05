# Rush Shell Specifications (001-100)

A modern, fast, Fish-like shell written in Rust with POSIX compatibility.

## Feature Directory Structure

Each feature is organized in its own directory with:
- `spec.md` - Requirement specification with user stories and acceptance criteria
- `plan.md` - Implementation plan with architecture and technical approach
- `tasks.md` - (When applicable) Breakdown of implementation tasks
- `checklists/` - (Optional) Quality assurance and testing checklists

---

## Complete Feature Roadmap (001-100)

### Category A: Core Shell (001-016) - COMPLETE

| # | Feature | Status | Tests |
|---|---------|--------|-------|
| 001 | Rush MVP (REPL, execution, history) | ✅ Done | 107+ |
| 002 | Tab completion | ✅ Done | 20+ |
| 003 | Autosuggestions (fish-like) | ✅ Done | 15+ |
| 004 | Pipes | ✅ Done | 10+ |
| 005 | Output redirection (>, >>, <) | ✅ Done | 10+ |
| 006 | Job control (&, fg, bg, Ctrl+Z) | ✅ Done | 26+ |
| 007 | Stderr redirection (2>, 2>>) | ✅ Done | - |
| 008 | Aliases | ✅ Done | - |
| 009 | Globbing (*, ?, []) | ✅ Done | 16+ |
| 010 | Command substitution $() | ✅ Done | - |
| 011 | Array variables | ✅ Done | - |
| 012 | Additional builtins | ✅ Done | - |
| 013 | CD builtin | ✅ Done | 8+ |
| 014 | Environment variables | ✅ Done | 20+ |
| 015 | Source builtin | ✅ Done | - |
| 016 | Exit builtin | ✅ Done | - |

### Category B: Control Flow (017-026)

| # | Feature | Priority | Dependencies |
|---|---------|----------|--------------|
| 017 | if/then/else/elif/fi | P0 | None |
| 018 | for/in/do/done loops | P0 | None |
| 019 | while/until loops | P0 | None |
| 020 | case/esac pattern matching | P1 | None |
| 021 | Shell functions | P0 | None |
| 022 | break statement | P1 | 018, 019 |
| 023 | continue statement | P1 | 018, 019 |
| 024 | return statement | P1 | 021 |
| 025 | Subshells () | P1 | None |
| 026 | Command groups { } | P1 | None |

### Category C: Expansions (027-036)

| # | Feature | Priority | Dependencies |
|---|---------|----------|--------------|
| 027 | Arithmetic expansion $((N+1)) | P1 | None |
| 028 | Parameter expansion ${var:-default} | P1 | None |
| 029 | Brace expansion {a,b,c} | P2 | None |
| 030 | Sequence expansion {1..10} | P2 | 029 |
| 031 | Extended globbing (**) | P2 | 009 |
| 032 | Tilde expansion (all contexts) | P2 | None |
| 033 | Here-strings <<< | P2 | 045 |
| 034 | Process substitution <() >() | P2 | 004 |
| 035 | History expansion (!!, !$, !n) | P2 | None |
| 036 | Word splitting control (IFS) | P2 | None |

### Category D: Configuration (037-044)

| # | Feature | Priority | Dependencies |
|---|---------|----------|--------------|
| 037 | ~/.rushrc startup file | P0 | 015 |
| 038 | Configuration file (TOML) | P1 | None |
| 039 | Prompt customization (PS1 etc) | P1 | 038 |
| 040 | Theme system | P2 | 038 |
| 041 | Profile files (~/.rush_profile) | P2 | 037 |
| 042 | XDG config support | P2 | 038 |
| 043 | Environment file loading (.env) | P2 | 014 |
| 044 | Per-directory config | P3 | 038 |

### Category E: Signal & Process (045-052)

| # | Feature | Priority | Dependencies |
|---|---------|----------|--------------|
| 045 | Heredocs (<< and <<-) | ✅ Done | - |
| 046 | trap (signal handling) | P1 | None |
| 047 | wait builtin | P1 | 006 |
| 048 | kill builtin | P1 | 006 |
| 049 | disown builtin | P2 | 006 |
| 050 | nohup handling | P2 | 006 |
| 051 | Coprocesses (coproc) | P3 | 004, 006 |
| 052 | Named pipes (FIFO) | P3 | None |

### Category F: Variables (053-060)

| # | Feature | Priority | Dependencies |
|---|---------|----------|--------------|
| 053 | Readonly variables | P2 | 014 |
| 054 | Local variables | P1 | 021 |
| 055 | Integer variables (typeset -i) | P2 | 014 |
| 056 | Associative arrays | P2 | 011 |
| 057 | Nameref variables | P3 | 014 |
| 058 | declare/typeset builtin | P2 | 014 |
| 059 | Dynamic variable names | P3 | 014 |
| 060 | Special variables ($RANDOM, $LINENO) | P2 | None |

### Category G: Builtins (061-074)

| # | Feature | Priority | Dependencies |
|---|---------|----------|--------------|
| 061 | read builtin | P1 | None |
| 062 | test enhancements ([[ ]]) | P1 | None |
| 063 | shift builtin | P1 | 021 |
| 064 | getopts builtin | P2 | 021 |
| 065 | eval builtin | P2 | None |
| 066 | exec builtin | P2 | None |
| 067 | let builtin | P2 | 027 |
| 068 | printf enhancements | P2 | None |
| 069 | pushd/popd/dirs | P2 | 013 |
| 070 | hash builtin | P3 | None |
| 071 | umask builtin | P2 | None |
| 072 | ulimit builtin | P3 | None |
| 073 | times builtin | P3 | None |
| 074 | command/builtin keywords | P2 | None |

### Category H: Script Execution (075-080)

| # | Feature | Priority | Dependencies |
|---|---------|----------|--------------|
| 075 | Script file execution (shebang) | P0 | 017-019 |
| 076 | set -e (errexit) | P1 | None |
| 077 | set -x (xtrace) | P1 | None |
| 078 | set -o pipefail | P1 | 004 |
| 079 | Command-line script mode (-c) | P1 | None |
| 080 | Strict mode options | P2 | 076-078 |

### Category I: History (081-086)

| # | Feature | Priority | Dependencies |
|---|---------|----------|--------------|
| 081 | fc builtin (edit/run history) | P2 | None |
| 082 | history command enhancements | P2 | None |
| 083 | Ctrl+R interactive search | P1 | None |
| 084 | History substring search | P2 | 083 |
| 085 | History timestamps | P2 | None |
| 086 | Shared history (across sessions) | P2 | None |

### Category J: Completion (087-092)

| # | Feature | Priority | Dependencies |
|---|---------|----------|--------------|
| 087 | Programmable completion | P1 | 002 |
| 088 | compgen/complete builtins | P2 | 087 |
| 089 | Completion scripts loading | P2 | 087 |
| 090 | Git completions | P2 | 087 |
| 091 | Docker completions | P2 | 087 |
| 092 | Fuzzy completion | P2 | 002 |

### Category K: Interactive Features (093-098)

| # | Feature | Priority | Dependencies |
|---|---------|----------|--------------|
| 093 | select (interactive menus) | P2 | 018 |
| 094 | Auto-cd (directory as command) | P2 | 013 |
| 095 | Spelling correction | P3 | None |
| 096 | Command-not-found hooks | P2 | None |
| 097 | Preexec/precmd hooks | P2 | None |
| 098 | Abbreviations (fish-like) | P2 | 008 |

### Category L: Modern Features (099-100)

| # | Feature | Priority | Dependencies |
|---|---------|----------|--------------|
| 099 | Plugin system | P3 | 038 |
| 100 | JSON output mode | P3 | None |

---

## Future Vision (101+)

| # | Feature | Description |
|---|---------|-------------|
| 101 | Structured data pipelines | Nushell-like data handling |
| 102 | HTTP client builtin | Built-in curl-like functionality |
| 103 | Git integration | Git-aware prompts and commands |
| 104 | AI-assisted completions | Smart suggestions |
| 105 | LSP support | Editor integration |
| 106 | Windows support | Cross-platform |
| 107 | Sandbox/restricted mode | Security |
| 108 | Remote execution | SSH integration |
| 109 | Cross-shell script compat | Bash/Zsh compatibility layer |
| 110 | Performance profiler | Script profiling |

---

## Implementation Phases

### Phase 1: Scripting Foundation (017-026, 075)
Enable basic scripting - if/for/while/case, functions, script execution

### Phase 2: Configuration (037-044)
~/.rushrc, TOML config, themes, prompts

### Phase 3: Expansions & Variables (027-036, 053-060)
Arithmetic, parameter expansion, brace expansion, local vars

### Phase 4: Builtins (061-074)
read, test, shift, getopts, pushd/popd, etc.

### Phase 5: Process Control (046-052, 076-080)
trap, wait, kill, set options, pipefail

### Phase 6: Interactive Polish (081-098)
History, completion, auto-cd, hooks, abbreviations

### Phase 7: Modern Features (099-110)
Plugins, JSON mode, structured data, AI assistance

---

## Design Philosophy (Hybrid Approach)

1. **Fish-like UX** - Out-of-box usability, syntax highlighting, suggestions
2. **POSIX syntax** - if/for/while/case use POSIX keywords
3. **No POSIX baggage** - Modern defaults, sane error handling
4. **Config via TOML** - Not complex shell scripts like .bashrc
5. **Small PRs** - <1,500 lines each, one feature = one issue = one PR

---

## Development Workflow

```bash
/speckit.specify  # Create spec.md
/speckit.clarify  # Refine requirements
/speckit.plan     # Create plan.md
/speckit.tasks    # Create tasks.md
gh issue create   # Sync to GitHub
/speckit.implement # Build it
```

---

## Summary

| Category | Range | Count | Status |
|----------|-------|-------|--------|
| Core Shell | 001-016 | 16 | ✅ Complete |
| Control Flow | 017-026 | 10 | Pending |
| Expansions | 027-036 | 10 | Pending |
| Configuration | 037-044 | 8 | Pending |
| Signal/Process | 045-052 | 8 | 1 Done |
| Variables | 053-060 | 8 | Pending |
| Builtins | 061-074 | 14 | Pending |
| Script Execution | 075-080 | 6 | Pending |
| History | 081-086 | 6 | Pending |
| Completion | 087-092 | 6 | Pending |
| Interactive | 093-098 | 6 | Pending |
| Modern | 099-100 | 2 | Pending |
| **Total** | **001-100** | **100** | **17 Done, 83 Pending** |

---

**Last Updated**: 2025-12-06
**Total Features**: 100 (+ 10 future vision)
**Complete**: 17 (001-016, 045)
