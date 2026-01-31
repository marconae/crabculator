# Project Mission: Crabculator

## Purpose

A terminal-based calculator with a text-editor experience. Users write multiple lines of calculations, define variables, and see results update live as they type.

**Core capabilities:**
- Split-panel TUI with repositionable memory pane for results
- Real-time evaluation of math expressions (e.g., `(5+3)*7*sqrt(9)`)
- Variable assignment and reuse across lines (e.g., `a = 5 + 3` then `5*a`)
- Inline error display with red underlined tokens, debounced while typing
- Text-editor navigation (cursor movement, editing anywhere in the document)
- Session persistence (buffer content and variables saved across restarts)

## UI Layout

Memory pane on the left (default), calculator pane on the right. Repositionable via CTRL+arrow keys.

```
 ┌─────────────────────Memory-─┬──── 🦀 crabculator───────────────────────────┐
 │                       a = 5 │ 1 a = 5                                      │
 │                      c = 15 │ 2 c = 5*3                                    │
 │                         400 │ 3 (a+c)^2                                    │
 │                          27 │ 4 9*sqrt(9)█                                 │
 │                             │                                              │
 │   results right-aligned ──► │ ◄── gutter + expressions                     │
 │                             │                                              │
 │                             │                                              │
 │                             │                                              │
 │                             │                                              │
 ├─────────────────────────────┴──────────────────────────────────────────────┤
 │ CTRL+Q: quit  CTRL+R: clear  CTRL+H: help  CTRL+←/→: move memory  ↑↓: hi   │
 └────────────────────────────────────────────────────────────────────────────┘
```

- Titles sit on the top border; "Memory" right-aligned, "crabculator" left-aligned with crab icon
- A thin vertical line (`│`) separates the two panes
- Line numbers in a subtle gutter; expressions to the right
- Variables highlighted in cyan, operators dimmed
- Errors shown inline: red underline on the token, dim italic message below (debounced 500ms)
- Command bar at the bottom with keyboard shortcuts
- Cursor shown as a block (`█`) at the current position

## Target Users

- Developers and engineers who live in the terminal and want quick calculations without leaving it
- Students and learners who want an interactive math scratchpad
- Power users who prefer terminal tools over GUI calculators

## Out of Scope

- Graphing or plotting
- Symbolic math (algebra simplification, derivatives, integrals)
- Scripting, loops, or user-defined functions

## Tech Stack

- **Language:** Rust 2024 edition
- **TUI Framework:** Ratatui 0.30 (with Crossterm 0.29 backend)
- **Math Engine:** Custom hand-rolled recursive descent parser and evaluator
- **Theme Detection:** terminal-colorsaurus for light/dark terminal theme detection
- **Persistence:** dirs crate for home directory resolution; plain text format at `~/.crabculator/state.txt`

### Tooling

| Command | Purpose |
|---------|---------|
| `cargo build` | Build |
| `cargo test` | Test |
| `cargo clippy` | Lint |
| `cargo fmt` | Format |

### Linting Configuration

- `unsafe_code = "forbid"`
- Clippy: `all`, `pedantic`, and `nursery` warning groups enabled
- `rustfmt.toml`: `max_width = 100`

## Architecture

Layered architecture: event loop -> application state -> domain modules.

```
src/
├── main.rs              # Entry point, TUI event loop, keystroke dispatch
├── app.rs               # Application state (buffer, cursor, scroll, theme, timers)
├── lib.rs               # Crate root, module declarations
├── terminal.rs          # Terminal setup/teardown (raw mode, alternate screen)
├── editor/
│   ├── mod.rs           # Editor module exports
│   ├── buffer.rs        # Text buffer with line management
│   └── cursor.rs        # Cursor navigation and position tracking
├── eval/
│   ├── mod.rs           # Evaluation module exports
│   ├── ast.rs           # Abstract syntax tree node types
│   ├── token.rs         # Tokenizer / lexer
│   ├── parser.rs        # Recursive descent expression parser
│   ├── evaluator.rs     # AST evaluator with function/constant support
│   ├── context.rs       # Variable context management
│   └── error.rs         # Error formatting with source spans
├── storage/
│   ├── mod.rs           # Storage module exports
│   ├── paths.rs         # Home directory and file path resolution
│   └── state.rs         # Plain text state serialization/deserialization
└── ui/
    ├── mod.rs           # UI module exports, top-level render orchestration
    ├── layout.rs        # Panel layout (split ratios, positioning)
    ├── render.rs        # Line building, styling, panel rendering
    ├── highlight.rs     # Syntax highlighting (variables, operators, errors)
    └── theme.rs         # Terminal theme detection and color adaptation
```

### Code Style

- Follow Rust idioms and clippy recommendations
- Use `Result` for fallible operations
- Prefer composition over deep inheritance
- Keep functions small and focused

## Domain Context

### Expression Syntax

- Basic arithmetic: `+`, `-`, `*`, `/`, `%`, `^`
- Parentheses for grouping: `(5 + 3) * 2`
- Built-in functions: `sqrt`, `cbrt`, `abs`, `pow`, `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`, `sinh`, `cosh`, `tanh`, `asinh`, `acosh`, `atanh`, `ln`, `log`, `log10`, `log2`, `exp`, `exp2`, `floor`, `ceil`, `round`, `min`, `max`, `hypot`
- Constants: `pi`, `e`
- Variable assignment: `name = expression`
- Variable reference: use variable name in expressions

### Line Evaluation

Each line is evaluated independently but shares a variable context:
- Line with `=`: assignment, stores result in variable, displays result
- Line without `=`: expression only, displays result
- Empty line: no evaluation, no result
- Invalid expression: displays error with underlined token (debounced 500ms while typing)

### Glossary

| Term | Meaning |
|------|---------|
| Memory pane | The results panel showing evaluation output, positioned left or right of the editor |
| Gutter | The line number column in the editor pane |
| Context | The variable store shared across all lines during evaluation |

## Constraints

- Must work on Linux, macOS, and Windows
- Terminal must support ANSI colors
- Minimum terminal size: 80x24
- Variable names: alphanumeric + underscore, must start with letter

## Performance

- Re-evaluate all lines on any change
- Evaluation should complete within a single frame (~16ms) to maintain responsive typing

## External Dependencies

- **None** — standalone terminal application with no network or external service dependencies
- State stored locally at `~/.crabculator/state.txt`
