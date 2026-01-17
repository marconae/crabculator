//! Integration tests for Crabculator.
//!
//! These tests verify the complete expression evaluation flow as specified
//! in the expression-eval capability spec.

use crabculator::app::App;
use crabculator::editor::Buffer;
use crabculator::eval::{EvalContext, LineResult, evaluate_all_lines, evaluate_line};

// ============================================================
// App Lifecycle Tests
// ============================================================

#[test]
fn test_app_lifecycle() {
    let mut app = App::new();
    assert!(app.running);
    app.quit();
    assert!(!app.running);
}

#[test]
fn test_app_initializes_with_buffer() {
    let app = App::new();
    // Buffer should have at least one line (either from persisted state or empty default)
    assert!(app.buffer.line_count() >= 1);
    // Context should be initialized (may have variables from persisted state)
    let _ = app.context.extract_variables();
}

// ============================================================
// Expression Evaluation Integration Tests
// ============================================================

/// Entering `5 + 3` shows `8` in result panel
#[test]
fn test_single_expression_evaluation() {
    let lines = ["5 + 3"];
    let results = evaluate_all_lines(lines);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0], LineResult::Value(8.0));
}

/// Entering `a = 10` shows `10` and stores variable
#[test]
fn test_variable_assignment() {
    let mut context = EvalContext::new();
    let result = evaluate_line("a = 10", &mut context);

    assert_eq!(
        result,
        LineResult::Assignment {
            name: "a".to_string(),
            value: 10.0,
        }
    );
    assert_eq!(context.get_variable("a"), Some(10.0));
}

/// Entering `a * 2` shows `20` (uses stored variable)
#[test]
fn test_variable_reference() {
    let lines = ["a = 10", "a * 2"];
    let results = evaluate_all_lines(lines);

    assert_eq!(results.len(), 2);
    assert_eq!(results[1], LineResult::Value(20.0));
}

/// Entering invalid expression returns error
#[test]
fn test_error_detection() {
    let lines = ["undefined_var + 5"];
    let results = evaluate_all_lines(lines);

    assert_eq!(results.len(), 1);
    assert!(matches!(results[0], LineResult::Error(_)));
}

/// Empty lines produce no result
#[test]
fn test_empty_line_evaluation() {
    let lines = [""];
    let results = evaluate_all_lines(lines);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0], LineResult::Empty);
}

// ============================================================
// Buffer Integration Tests
// ============================================================

/// Characters inserted at cursor position
#[test]
fn test_buffer_character_insertion() {
    let mut buffer = Buffer::new();
    buffer.insert_char('5');
    buffer.insert_char('+');
    buffer.insert_char('3');

    assert_eq!(buffer.lines()[0], "5+3");
}

/// Enter creates new line, moves cursor to start of new line
#[test]
fn test_buffer_newline_creation() {
    let mut buffer = Buffer::new();
    buffer.insert_char('a');
    buffer.insert_newline();
    buffer.insert_char('b');

    assert_eq!(buffer.line_count(), 2);
    assert_eq!(buffer.lines()[0], "a");
    assert_eq!(buffer.lines()[1], "b");
    assert_eq!(buffer.cursor().row(), 1);
    assert_eq!(buffer.cursor().col(), 1);
}

/// Backspace removes character before cursor
#[test]
fn test_buffer_backspace() {
    let mut buffer = Buffer::new();
    buffer.insert_char('a');
    buffer.insert_char('b');
    buffer.insert_char('c');
    buffer.delete_char_before();

    assert_eq!(buffer.lines()[0], "ab");
}

/// Backspace at line start merges with previous line
#[test]
fn test_buffer_line_merge() {
    let mut buffer = Buffer::new();
    buffer.insert_char('a');
    buffer.insert_newline();
    buffer.insert_char('b');
    buffer.move_cursor_to_line_start();
    buffer.delete_char_before();

    assert_eq!(buffer.line_count(), 1);
    assert_eq!(buffer.lines()[0], "ab");
}

// ============================================================
// Cursor Navigation Tests
// ============================================================

/// Cursor navigation moves between lines correctly
#[test]
fn test_cursor_navigation() {
    let mut buffer = Buffer::new();
    buffer.insert_char('a');
    buffer.insert_newline();
    buffer.insert_char('b');

    // Start at end of line 2
    assert_eq!(buffer.cursor().row(), 1);

    // Move up
    buffer.move_cursor_up();
    assert_eq!(buffer.cursor().row(), 0);

    // Move down
    buffer.move_cursor_down();
    assert_eq!(buffer.cursor().row(), 1);
}

#[test]
fn test_cursor_home_end() {
    let mut buffer = Buffer::new();
    buffer.insert_char('a');
    buffer.insert_char('b');
    buffer.insert_char('c');

    // Cursor should be at end
    assert_eq!(buffer.cursor().col(), 3);

    // Move to start
    buffer.move_cursor_to_line_start();
    assert_eq!(buffer.cursor().col(), 0);

    // Move to end
    buffer.move_cursor_to_line_end();
    assert_eq!(buffer.cursor().col(), 3);
}

// ============================================================
// End-to-End Expression Flow Tests
// ============================================================

/// Complex multi-line calculation with variables
#[test]
fn test_multiline_calculation() {
    let lines = ["base = 100", "rate = 0.15", "base * rate"];
    let results = evaluate_all_lines(lines);

    assert_eq!(results.len(), 3);

    // base = 100
    assert!(matches!(&results[0], LineResult::Assignment { name, value }
        if name == "base" && (*value - 100.0).abs() < f64::EPSILON));

    // rate = 0.15
    assert!(matches!(&results[1], LineResult::Assignment { name, value }
        if name == "rate" && (*value - 0.15).abs() < f64::EPSILON));

    // base * rate = 15.0
    assert_eq!(results[2], LineResult::Value(15.0));
}

/// Expressions with parentheses
#[test]
fn test_expression_with_parentheses() {
    let lines = ["(5 + 3) * 2"];
    let results = evaluate_all_lines(lines);

    assert_eq!(results[0], LineResult::Value(16.0));
}

/// Expressions with built-in functions
#[test]
fn test_expression_with_functions() {
    let lines = ["sqrt(16)"];
    let results = evaluate_all_lines(lines);

    assert_eq!(results[0], LineResult::Value(4.0));
}

/// Error in one line doesn't affect other lines
#[test]
fn test_error_isolation() {
    let lines = ["5 + 3", "undefined_var", "10 - 2"];
    let results = evaluate_all_lines(lines);

    assert_eq!(results[0], LineResult::Value(8.0));
    assert!(matches!(results[1], LineResult::Error(_)));
    assert_eq!(results[2], LineResult::Value(8.0));
}

// ============================================================
// State Persistence Integration Tests
// ============================================================

use crabculator::storage::{PersistedState, load_from_path, save_to_path};
use tempfile::tempdir;

/// State persistence roundtrip: save buffer lines, reload, verify restored
///
/// This test simulates the app lifecycle:
/// - Start app -> edit buffer -> quit -> restart -> buffer restored
/// Note: Variables are no longer persisted; they're recomputed from buffer evaluation.
#[test]
fn test_state_persistence_roundtrip() {
    // Create a temporary directory for the state file
    let dir = tempdir().expect("should create temp dir");
    let state_file = dir.path().join("state.txt");

    // === Simulate first session: user edits buffer ===

    // Buffer content representing expressions the user typed
    let buffer_lines = vec![
        "price = 100".to_string(),
        "tax_rate = 0.15".to_string(),
        "price * tax_rate".to_string(),
        "total = price + (price * tax_rate)".to_string(),
    ];

    // Create state and save (simulates quit)
    let original_state = PersistedState::new(buffer_lines);
    save_to_path(&original_state, &state_file).expect("save should succeed");

    // === Simulate restart: load state ===

    let loaded_state = load_from_path(&state_file)
        .expect("load should succeed")
        .expect("state file should exist");

    // === Verify buffer content is restored ===

    assert_eq!(
        loaded_state.buffer_lines.len(),
        4,
        "should have 4 lines in buffer"
    );
    assert_eq!(loaded_state.buffer_lines[0], "price = 100");
    assert_eq!(loaded_state.buffer_lines[1], "tax_rate = 0.15");
    assert_eq!(loaded_state.buffer_lines[2], "price * tax_rate");
    assert_eq!(
        loaded_state.buffer_lines[3],
        "total = price + (price * tax_rate)"
    );

    // === Verify complete state equality ===

    assert_eq!(
        original_state, loaded_state,
        "loaded state should match original"
    );
}

/// State persistence with empty state: verify empty state round-trips correctly
#[test]
fn test_state_persistence_empty_state() {
    let dir = tempdir().expect("should create temp dir");
    let state_file = dir.path().join("state.txt");

    // Save empty state (fresh start scenario)
    let original_state = PersistedState::empty();
    save_to_path(&original_state, &state_file).expect("save should succeed");

    // Load and verify
    let loaded_state = load_from_path(&state_file)
        .expect("load should succeed")
        .expect("state file should exist");

    assert!(loaded_state.buffer_lines.is_empty());
    assert_eq!(original_state, loaded_state);
}

/// State persistence preserves buffer content with special characters
#[test]
fn test_state_persistence_special_characters() {
    let dir = tempdir().expect("should create temp dir");
    let state_file = dir.path().join("state.txt");

    // Use buffer lines with special characters
    let original_state = PersistedState::new(vec![
        "pi = 3.14159".to_string(),
        "e = 2.71828".to_string(),
        "result = pi * e".to_string(),
    ]);
    save_to_path(&original_state, &state_file).expect("save should succeed");

    let loaded_state = load_from_path(&state_file)
        .expect("load should succeed")
        .expect("state file should exist");

    assert_eq!(original_state, loaded_state);
}

/// Bug test: Variables MUST be stored in app.context after buffer evaluation
///
/// This test verifies that when expressions with variable assignments are evaluated,
/// the variables are stored in the App's `EvalContext` so they can be persisted.
///
/// Prior bug: `evaluate_all_lines` created its own local `EvalContext` instead of
/// using app.context, so variables were lost after rendering.
#[test]
fn test_app_context_stores_variables_after_evaluation() {
    use crabculator::editor::Buffer;
    use crabculator::eval::{EvalContext, evaluate_all_lines_with_context};

    // Create fresh buffer and context (avoiding persisted state)
    let mut buffer = Buffer::new();
    let mut context = EvalContext::new();

    // Simulate user typing: "a = 5"
    for c in "a = 5".chars() {
        buffer.insert_char(c);
    }
    buffer.insert_newline();

    // Simulate user typing: "b = a + 10"
    for c in "b = a + 10".chars() {
        buffer.insert_char(c);
    }

    // Evaluate the buffer with context (this is what render should do)
    let _ =
        evaluate_all_lines_with_context(buffer.lines().iter().map(String::as_str), &mut context);

    // CRITICAL: Variables must be stored in context after evaluation
    let variables = context.extract_variables();

    assert!(
        variables.contains_key("a"),
        "Variable 'a' should be stored in context after evaluation"
    );
    assert_eq!(
        variables.get("a"),
        Some(&5.0),
        "Variable 'a' should equal 5.0"
    );
    assert!(
        variables.contains_key("b"),
        "Variable 'b' should be stored in context after evaluation"
    );
    assert_eq!(
        variables.get("b"),
        Some(&15.0),
        "Variable 'b' should equal 15.0"
    );
}

/// Test: Full persistence flow saves buffer lines only
///
/// This is an end-to-end test that verifies the simplified persistence flow:
/// 1. User types expressions with variable assignments
/// 2. Buffer lines are saved to state file
/// 3. Variables are recomputed from evaluation on next load
#[test]
fn test_save_state_saves_buffer_lines() {
    use crabculator::editor::Buffer;

    let dir = tempdir().expect("should create temp dir");
    let state_file = dir.path().join("state.txt");

    // Create fresh buffer
    let mut buffer = Buffer::new();

    // Type "x = 42"
    for c in "x = 42".chars() {
        buffer.insert_char(c);
    }

    // Save state (simulating app.save_state())
    let state = PersistedState::new(buffer.lines().to_vec());
    save_to_path(&state, &state_file).expect("save should succeed");

    // Load and verify buffer lines were persisted
    let loaded = load_from_path(&state_file)
        .expect("load should succeed")
        .expect("state should exist");

    assert_eq!(
        loaded.buffer_lines,
        vec!["x = 42"],
        "Buffer lines should be persisted"
    );
}

// ============================================================
// Quit Hotkey Tests
// ============================================================

/// Test that 'q' character can be inserted into buffer (not exit)
///
/// Previous behavior: pressing 'q' would exit the application.
/// New behavior: 'q' is just another character that gets inserted.
#[test]
fn test_typing_q_inserts_character_not_exit() {
    let mut buffer = Buffer::new();

    // Type 'q' character
    buffer.insert_char('q');

    // Buffer should contain 'q', not be empty
    assert_eq!(buffer.lines()[0], "q");
}

/// Test that 'q' can be used in variable names
///
/// This verifies the complete flow: user types a variable name containing 'q',
/// the buffer stores it correctly, and it can be evaluated.
#[test]
fn test_variable_name_with_q_works() {
    let lines = ["qty = 5", "qty * 2"];
    let results = evaluate_all_lines(lines);

    assert_eq!(results.len(), 2);
    // qty = 5 should be an assignment
    assert!(matches!(&results[0], LineResult::Assignment { name, value }
        if name == "qty" && (*value - 5.0).abs() < f64::EPSILON));
    // qty * 2 should equal 10
    assert_eq!(results[1], LineResult::Value(10.0));
}

/// Test expressions containing 'q' character in various positions
#[test]
fn test_expressions_with_q_character() {
    let mut buffer = Buffer::new();

    // Type "sqr = 4" (variable with 'q')
    for c in "sqr = 4".chars() {
        buffer.insert_char(c);
    }

    assert_eq!(buffer.lines()[0], "sqr = 4");

    // Clear and test another
    buffer = Buffer::new();
    for c in "eq = 1".chars() {
        buffer.insert_char(c);
    }

    assert_eq!(buffer.lines()[0], "eq = 1");
}

// ============================================================
// Horizontal Scroll Tests
// ============================================================

/// Test that cursor remains at expected position after typing long line
///
/// When typing a line longer than the visible width, the cursor should
/// be at the end of the line (not lost or reset).
#[test]
fn test_cursor_position_on_long_line_input() {
    let mut buffer = Buffer::new();
    let long_text = "0123456789abcdefghijklmnopqrstuvwxyz";

    // Type a long line
    for c in long_text.chars() {
        buffer.insert_char(c);
    }

    // Cursor should be at the end of the line
    assert_eq!(buffer.cursor().col(), long_text.len());
    assert_eq!(buffer.lines()[0], long_text);
}

/// Test horizontal scroll adjustment keeps cursor visible
///
/// This tests the App's horizontal scroll adjustment logic
/// to ensure cursor stays within visible area.
#[test]
fn test_horizontal_scroll_adjustment_on_long_input() {
    use crabculator::app::App;

    let mut app = App::new();
    // Clear any persisted state to start fresh
    app.clear_all();

    let long_text = "0123456789abcdefghijklmnopqrstuvwxyz0123456789";

    // Type a long line
    for c in long_text.chars() {
        app.buffer.insert_char(c);
    }

    // Simulate a narrow visible width (typical terminal might be 80, but we test with 20)
    let visible_width = 20;

    // Adjust horizontal scroll
    app.adjust_horizontal_scroll(visible_width);

    // Cursor column minus scroll offset should be within visible range
    let cursor_col = app.buffer.cursor().col();
    let offset = app.horizontal_scroll_offset;

    assert!(
        cursor_col >= offset,
        "Cursor should be at or after scroll offset"
    );
    assert!(
        cursor_col < offset + visible_width,
        "Cursor should be within visible area"
    );
}

/// Test that moving cursor left on long line adjusts scroll
#[test]
fn test_horizontal_scroll_adjusts_on_cursor_left() {
    use crabculator::app::App;

    let mut app = App::new();
    // Clear any persisted state to start fresh
    app.clear_all();

    let long_text = "0123456789abcdefghijklmnopqrstuvwxyz";

    // Type a long line
    for c in long_text.chars() {
        app.buffer.insert_char(c);
    }

    // Simulate scroll being set (as if we had scrolled right)
    let visible_width = 10;
    app.adjust_horizontal_scroll(visible_width);
    let initial_offset = app.horizontal_scroll_offset;

    // Move cursor to start
    app.buffer.move_cursor_to_line_start();
    app.adjust_horizontal_scroll(visible_width);

    // Scroll should have adjusted to show cursor at start
    assert!(
        app.horizontal_scroll_offset < initial_offset,
        "Scroll offset should decrease when moving cursor left"
    );
}

/// Test that Home key navigation with horizontal scroll works correctly
#[test]
fn test_home_key_with_horizontal_scroll() {
    use crabculator::app::App;

    let mut app = App::new();
    // Clear any persisted state to start fresh
    app.clear_all();

    // Type a long line
    for c in "0123456789abcdefghijklmnopqrstuvwxyz".chars() {
        app.buffer.insert_char(c);
    }

    let visible_width = 15;
    app.adjust_horizontal_scroll(visible_width);

    // Press Home (move to line start)
    app.buffer.move_cursor_to_line_start();
    app.adjust_horizontal_scroll(visible_width);

    // Cursor should be at column 0
    assert_eq!(app.buffer.cursor().col(), 0);
    // Scroll offset should be 0 or very small (within margin)
    assert!(
        app.horizontal_scroll_offset <= 4,
        "Scroll offset should be minimal after Home key"
    );
}

/// Test that End key navigation with horizontal scroll works correctly
#[test]
fn test_end_key_with_horizontal_scroll() {
    use crabculator::app::App;

    let mut app = App::new();
    // Clear any persisted state to start fresh
    app.clear_all();

    let long_text = "0123456789abcdefghijklmnopqrstuvwxyz";

    // Type a long line
    for c in long_text.chars() {
        app.buffer.insert_char(c);
    }

    // Move to start, then press End
    app.buffer.move_cursor_to_line_start();
    let visible_width = 15;
    app.adjust_horizontal_scroll(visible_width);

    app.buffer.move_cursor_to_line_end();
    app.adjust_horizontal_scroll(visible_width);

    // Cursor should be at end of line
    assert_eq!(app.buffer.cursor().col(), long_text.len());

    // Cursor should be visible (within scroll window)
    let cursor_col = app.buffer.cursor().col();
    let offset = app.horizontal_scroll_offset;
    assert!(
        cursor_col >= offset && cursor_col < offset + visible_width,
        "Cursor should be visible after End key"
    );
}
