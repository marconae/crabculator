# Project Mission: Crabculator

## Purpose

Crabculator is a terminal-based calculator application that provides a text-editor-like experience for writing and evaluating math expressions. Users can write multiple lines of calculations, define variables, and see results update live as they type.

**Key features:**
- Split-panel TUI: left panel (80%) for input, right panel (20%) for results
- Real-time evaluation of math expressions (e.g., `(5+3)*7*sqrt(9)`)
- Variable assignment and reuse across lines (e.g., `a = 5 + 3` then `5*a`)
- Inline error display with red underlined tokens and explanations
- Text-editor navigation (cursor movement, editing anywhere in the document)
- Variable persistence across sessions

## Tech Stack

- **Language:** Rust 2024 edition
- **TUI Framework:** Ratatui (with Crossterm backend)
- **Math Parser:** evalexpr (most starred, actively maintained)
- **Persistence:** dirs crate for home directory resolution (~/.crabculator/)
- **Serialization:** serde + serde_json for variable storage

### Tooling

- **Build:** `cargo build`
- **Test:** `cargo test`
- **Lint:** `cargo clippy`
- **Format:** `cargo fmt`

## Project Conventions

### Architecture Patterns

```
src/
├── main.rs           # Entry point, TUI event loop
├── app.rs            # Application state management
├── ui/
│   ├── mod.rs        # UI module exports
│   ├── layout.rs     # Panel layout (80/20 split)
│   └── render.rs     # Rendering logic
├── editor/
│   ├── mod.rs        # Editor module exports
│   ├── buffer.rs     # Text buffer with cursor
│   └── cursor.rs     # Cursor navigation
├── eval/
│   ├── mod.rs        # Evaluation module exports
│   ├── parser.rs     # Expression parsing wrapper
│   ├── context.rs    # Variable context management
│   └── error.rs      # Error formatting with spans
└── storage/
    ├── mod.rs        # Storage module exports
    └── persist.rs    # Variable persistence
```

### Code Style

- Follow Rust idioms and clippy recommendations
- Use `Result` for fallible operations
- Prefer composition over deep inheritance
- Keep functions small and focused

## Domain Context

### Expression Syntax

- Basic arithmetic: `+`, `-`, `*`, `/`, `%`
- Parentheses for grouping: `(5 + 3) * 2`
- Built-in functions: `sqrt`, `sin`, `cos`, `tan`, `log`, `ln`, `abs`, `floor`, `ceil`
- Constants: `pi`, `e`
- Variable assignment: `name = expression`
- Variable reference: use variable name in expressions

### Line Evaluation

Each line is evaluated independently but shares a variable context:
- Line with `=`: assignment, stores result in variable, displays result
- Line without `=`: expression only, displays result
- Empty line: no evaluation, no result
- Invalid expression: displays error with underlined token

## Important Constraints

- Must work on Linux, macOS, and Windows
- Terminal must support ANSI colors
- Minimum terminal size: 80x24
- Variable names: alphanumeric + underscore, must start with letter

## Performance Considerations

- Re-evaluate all lines on any change (simple approach for MVP)
- Future optimization: dependency tracking to only re-evaluate affected lines
- Keep UI responsive: evaluation should complete within frame time (~16ms)

## External Dependencies

- **None** - this is a standalone terminal application
- State (buffer content and variables) stored locally at `~/.crabculator/state.json`
