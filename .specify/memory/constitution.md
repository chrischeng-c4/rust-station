<!--
Sync Impact Report:
- Version: 1.0.0 → 1.1.0
- Change Type: MINOR amendment (configuration strategy updated)
- Modified Sections: Configuration Strategy (Python → Rhai)
- Rationale: Rhai better aligns with Performance-First and Rust-Native principles
  - Python: 20-50ms startup, 10-50MB overhead, FFI complexity
  - Rhai: ~1ms startup, <1MB overhead, pure Rust, proven in production
- Templates Status:
  ✅ spec-template.md - No changes needed
  ✅ plan-template.md - No changes needed (constitution check will reference updated principles)
  ✅ tasks-template.md - No changes needed
- Follow-up TODOs: None
-->

# rush Shell Constitution

## Core Principles

### I. Performance-First (NON-NEGOTIABLE)

rush MUST prioritize performance at every level:

- **Fast startup**: Shell initialization MUST complete in <100ms on modern hardware
- **Instant responsiveness**: Command prompt MUST respond to input within 16ms (60 FPS)
- **Minimal overhead**: Command execution overhead MUST be <5ms compared to direct process spawn
- **Memory efficiency**: Baseline memory footprint MUST be <10MB for interactive session
- **No blocking operations**: All I/O MUST be async; blocking calls require explicit justification

**Rationale**: Users switch to rush for speed. Any perceived sluggishness defeats the primary value proposition. This is what differentiates rush from feature-heavy but slower shells.

**Enforcement**: All features MUST include performance benchmarks. PRs that regress performance require justification or redesign.

### II. Zero-Config Philosophy

rush MUST work beautifully without configuration:

- **Sensible defaults**: Every feature MUST have intelligent defaults requiring no user setup
- **Fish-like UX**: Syntax highlighting, autosuggestions, and tab completions work immediately
- **Progressive disclosure**: Advanced features available but not required for basic usage
- **No mandatory setup**: User can start being productive in <30 seconds after install
- **Configuration optional**: RC files extend functionality but are never required

**Rationale**: Configuration friction is the enemy of adoption. fish demonstrated that shells can be powerful AND zero-config. rush follows this principle.

**Enforcement**: New features MUST work with zero configuration. Any feature requiring setup MUST justify why defaults are impossible.

### III. Progressive Complexity

rush MUST be simple by default, powerful when needed:

- **Layered functionality**: Core features simple; advanced features opt-in
- **Discoverability**: Help system and autosuggestions reveal advanced features
- **No forced complexity**: Users never pay (performance/complexity cost) for features they don't use
- **Escape hatches**: Advanced users can access full power without fighting defaults
- **Learn as you go**: Features discoverable through natural usage patterns

**Rationale**: Beginners need simplicity. Experts need power. The same shell must serve both without compromise.

**Enforcement**: Feature proposals MUST explain both basic and advanced use cases. Complexity MUST be opt-in, never mandatory.

### IV. Modern UX

rush MUST provide a contemporary, delightful user experience:

- **Syntax highlighting**: Commands, paths, strings, and operators colored in real-time
- **Autosuggestions**: Ghost text from history and completions as users type
- **Smart completions**: Context-aware tab completions for commands, paths, flags, and arguments
- **Visual feedback**: Clear indicators for job status, error states, and command outcomes
- **Accessible design**: Color schemes respect accessibility; fallbacks for limited terminals

**Rationale**: Modern developers expect IDE-quality experiences everywhere. The shell should feel as polished as their editor.

**Enforcement**: UX features MUST be implemented before 1.0 release. User testing required for interaction patterns.

### V. Rust-Native

rush MUST leverage Rust's ecosystem and philosophy:

- **Pure Rust**: Core implementation in Rust; FFI only when absolutely necessary
- **Ecosystem integration**: Prefer mature Rust crates over reinventing (tokio, clap, rustyline ecosystem)
- **Zero-cost abstractions**: Use Rust's type system to eliminate runtime overhead
- **Memory safety**: No unsafe code without rigorous justification and documentation
- **Idiomatic code**: Follow Rust API guidelines and community best practices

**Rationale**: Rust provides safety, performance, and modern tooling. Building in Rust gives us compile-time guarantees and a rich ecosystem.

**Enforcement**: All dependencies MUST be justified. Unsafe code requires review and documentation. Clippy warnings MUST be addressed.

## Platform & MVP Scope

### Target Platform

**MVP (v0.1)**: macOS only

- Focus on single platform for faster iteration
- Leverage macOS-specific optimizations (kqueue, CoreFoundation)
- Cross-platform support deferred to post-MVP

**Post-MVP**: Linux, then Windows via WSL

**Rationale**: Narrow scope accelerates delivery. macOS provides unified target for proving core concepts.

### MVP Feature Set

The following features MUST work in v0.1:

1. **Basic REPL**: Read-eval-print loop with line editing
2. **Syntax Highlighting**: Real-time command colorization
3. **Autosuggestions**: Ghost text from command history
4. **Tab Completions**: Commands, paths, and basic flags
5. **Command History**: Persistent history with search
6. **Job Control**: Background jobs, fg, bg, jobs commands
7. **Script Execution**: Run shell scripts from files

Features explicitly OUT of scope for MVP:

- Advanced scripting (functions, conditionals, loops)
- Plugin system
- Custom themes
- Network features
- Advanced job control (job groups, terminal multiplexing)

## Configuration Strategy

### Hybrid Approach

rush configuration follows a staged approach:

**Phase 1 (MVP)**: TOML-based configuration

- Config file: `~/.config/rush/rush.toml`
- Simple key-value pairs for settings
- Fast parsing, zero dependencies
- Example:
  ```toml
  [appearance]
  theme = "default"

  [behavior]
  history_size = 10000
  ```

**Phase 2 (Post-MVP)**: Optional Rhai scripting

- Config file: `~/.config/rush/rushrc.rhai`
- Rhai embedded scripting language (pure Rust)
- Advanced customization for power users
- Example:
  ```rust
  fn prompt() {
      let branch = git_branch();
      return `${branch} $ `;
  }

  fn on_command(cmd) {
      if cmd.starts_with("rm -rf") {
          return confirm("Really delete?");
      }
  }
  ```

**Rationale**:

- TOML provides fast, simple config for MVP (aligns with Principle I: Performance-First)
- Rhai scripting deferred until core features proven (aligns with Principle III: Progressive Complexity)
- Rhai is Rust-native, ~1ms startup, <1MB overhead (maintains Principle I and V)
- JavaScript-like syntax familiar to most developers (better UX than custom DSL)
- Hybrid approach serves both simple and power users (aligns with Principle II: Zero-Config)

**Default Behavior**: If no config file exists, rush works perfectly with hardcoded defaults.

## Development Workflow

### Specification-Driven Development (MANDATORY)

All rush development MUST follow spec-driven workflow:

1. **Constitution** → defines principles (this document)
2. **Specification** → defines WHAT to build (requirements, user stories)
3. **Plan** → defines HOW to build (architecture, technical decisions)
4. **Tasks** → defines implementation steps
5. **Implementation** → builds according to spec

**Enforcement**:

- No code without specifications
- All code MUST trace to requirements
- Architecture decisions MUST reference constitution principles
- PRs MUST link to specification artifacts

### Testing Philosophy

- **Unit tests**: Required for core logic
- **Integration tests**: Required for feature completions (REPL, job control, etc.)
- **Performance benchmarks**: Required for performance-critical paths
- **Manual testing**: Required for UX features (syntax highlighting, completions)

### Code Quality

- **Rust edition**: 2021 (or latest stable)
- **Clippy**: All warnings addressed or explicitly allowed with justification
- **Formatting**: cargo fmt enforced via CI
- **Documentation**: Public APIs MUST have doc comments
- **Examples**: Complex features MUST have usage examples

## Governance

### Constitution Authority

This constitution supersedes all other development practices and guidelines:

- All features MUST align with core principles
- Principle violations require explicit justification and approval
- Convenience never trumps principles
- When in doubt, refer to principles

### Amendment Process

Constitution amendments require:

1. Documented proposal explaining change rationale
2. Impact analysis on existing features and specifications
3. Version bump following semantic versioning:
   - **MAJOR**: Backward-incompatible principle changes
   - **MINOR**: New principles or material expansions
   - **PATCH**: Clarifications, wording improvements
4. Update to all dependent templates and specifications

### Compliance Review

All pull requests MUST verify compliance:

- Feature aligns with core principles
- Performance requirements met
- Zero-config philosophy maintained
- Complexity justified
- Specification artifacts exist and linked

### Complexity Justification

Any complexity MUST be justified against principles:

- Heavy dependencies → justify against Principle V (Rust-Native)
- Required configuration → justify against Principle II (Zero-Config)
- Performance overhead → justify against Principle I (Performance-First)
- Mandatory advanced features → justify against Principle III (Progressive Complexity)

**Version**: 1.1.0 | **Ratified**: 2025-11-14 | **Last Amended**: 2025-11-14
