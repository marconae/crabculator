//! Panel rendering for the Crabculator TUI.
//!
//! Provides rendering functions for the input and result panels, including:
//! - Error highlighting with red underlines
//! - Error message display below error lines
//! - Result panel with aligned evaluation results

use evalexpr::Value;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::editor::Buffer;
use crate::eval::{EvalError, LineResult, evaluate_all_lines};
use crate::ui::highlight::highlight_line;

/// Formats a `LineResult` for display in the result panel.
///
/// # Returns
/// - `Some(String)` with the formatted result for values and assignments
/// - `None` for empty lines or errors (errors shown in input panel)
#[must_use]
pub fn format_result(result: &LineResult) -> Option<String> {
    match result {
        LineResult::Value(value) => Some(format_value(value)),
        LineResult::Assignment { name, value } => Some(format!("{name} = {}", format_value(value))),
        LineResult::Empty | LineResult::Error(_) => None,
    }
}

/// Formats a `Value` for display.
///
/// Integers are displayed without decimal places.
/// Floats are displayed with decimal places unless they are whole numbers.
#[must_use]
fn format_value(value: &Value) -> String {
    match value {
        Value::Int(i) => i.to_string(),
        Value::Float(f) => {
            // If the float is a whole number, display without decimals
            if f.fract() == 0.0 {
                format!("{f:.0}")
            } else {
                f.to_string()
            }
        }
        // For other value types, use default display
        other => format!("{other}"),
    }
}

/// Builds styled text lines for the input panel.
///
/// Handles:
/// - Normal text styling
/// - Error spans with red underline
/// - Error messages below error lines
///
/// # Arguments
/// * `lines` - The buffer lines to render
/// * `results` - The evaluation results corresponding to each line
///
/// # Returns
/// A vector of styled `Line` objects ready for rendering.
#[must_use]
pub fn build_input_lines<'a>(lines: &'a [String], results: &'a [LineResult]) -> Vec<Line<'a>> {
    let mut output: Vec<Line<'a>> = Vec::new();

    for (i, line_text) in lines.iter().enumerate() {
        let result = results.get(i);

        // Build the main line with potential error or syntax highlighting
        let styled_line = match result {
            Some(LineResult::Error(err)) => build_error_line(line_text, err),
            _ => Line::from(highlight_line(line_text)),
        };
        output.push(styled_line);

        // Add error message below error lines
        if let Some(LineResult::Error(err)) = result {
            let error_line = Line::from(Span::styled(
                format!("  ^ {}", err.message()),
                Style::default().fg(Color::Red),
            ));
            output.push(error_line);
        }
    }

    output
}

/// Builds a styled line with error highlighting.
///
/// If the error has a span, only that portion is underlined.
/// Otherwise, the entire line is underlined.
fn build_error_line<'a>(line_text: &'a str, error: &EvalError) -> Line<'a> {
    let error_style = Style::default()
        .fg(Color::Red)
        .add_modifier(Modifier::UNDERLINED);

    error.span().map_or_else(
        // No span available, underline entire line
        || Line::from(Span::styled(line_text, error_style)),
        |span| {
            // Clamp span to line bounds
            let start = span.start.min(line_text.len());
            let end = span.end.min(line_text.len()).max(start);

            let mut spans = Vec::new();

            // Text before error
            if start > 0 {
                spans.push(Span::raw(&line_text[..start]));
            }

            // Error portion with underline
            if start < end {
                spans.push(Span::styled(&line_text[start..end], error_style));
            }

            // Text after error
            if end < line_text.len() {
                spans.push(Span::raw(&line_text[end..]));
            }

            Line::from(spans)
        },
    )
}

/// Builds styled text lines for the result panel.
///
/// Results are aligned with their corresponding input lines.
/// Empty lines and error lines show nothing.
///
/// # Arguments
/// * `results` - The evaluation results to display
///
/// # Returns
/// A vector of styled `Line` objects ready for rendering.
#[must_use]
pub fn build_result_lines(results: &[LineResult]) -> Vec<Line<'_>> {
    results
        .iter()
        .map(|result| {
            format_result(result).map_or_else(
                || Line::from(""),
                |text| Line::from(Span::styled(text, Style::default().fg(Color::Green))),
            )
        })
        .collect()
}

/// Builds visible input lines with scrolling support.
///
/// Only builds lines that are visible in the viewport, based on scroll offset
/// and visible height. This avoids building lines that won't be displayed.
///
/// # Arguments
/// * `lines` - The buffer lines to render
/// * `results` - The evaluation results corresponding to each line
/// * `scroll_offset` - The first visible line index (0-based)
/// * `visible_height` - The number of visible lines in the viewport
///
/// # Returns
/// A vector of styled `Line` objects for the visible portion only.
#[must_use]
#[allow(dead_code)] // Reserved for future scrolling without highlighting
pub fn build_visible_input_lines<'a>(
    lines: &'a [String],
    results: &'a [LineResult],
    scroll_offset: usize,
    visible_height: usize,
) -> Vec<Line<'a>> {
    let mut output: Vec<Line<'a>> = Vec::new();

    // Calculate the range of lines to render
    let start = scroll_offset.min(lines.len());
    let end = (scroll_offset + visible_height).min(lines.len());

    for (i, line_text) in lines.iter().enumerate().take(end).skip(start) {
        let result = results.get(i);

        // Build the main line with potential error or syntax highlighting
        let styled_line = match result {
            Some(LineResult::Error(err)) => build_error_line(line_text, err),
            _ => Line::from(highlight_line(line_text)),
        };
        output.push(styled_line);

        // Add error message below error lines
        if let Some(LineResult::Error(err)) = result {
            let error_line = Line::from(Span::styled(
                format!("  ^ {}", err.message()),
                Style::default().fg(Color::Red),
            ));
            output.push(error_line);
        }
    }

    output
}

/// Builds visible result lines with scrolling support.
///
/// Only builds result lines that are visible in the viewport, based on scroll
/// offset and visible height. Results are synchronized with input lines.
///
/// # Arguments
/// * `results` - The evaluation results to display
/// * `scroll_offset` - The first visible line index (0-based)
/// * `visible_height` - The number of visible lines in the viewport
///
/// # Returns
/// A vector of styled `Line` objects for the visible portion only.
#[must_use]
#[allow(dead_code)] // Reserved for future scrolling without highlighting
pub fn build_visible_result_lines(
    results: &[LineResult],
    scroll_offset: usize,
    visible_height: usize,
) -> Vec<Line<'_>> {
    // Calculate the range of results to render
    let start = scroll_offset.min(results.len());
    let end = (scroll_offset + visible_height).min(results.len());

    results[start..end]
        .iter()
        .map(|result| {
            format_result(result).map_or_else(
                || Line::from(""),
                |text| Line::from(Span::styled(text, Style::default().fg(Color::Green))),
            )
        })
        .collect()
}

/// Builds visible input lines with scrolling and current line highlighting.
///
/// Combines scrolling support with current line highlighting for the input panel.
///
/// # Arguments
/// * `lines` - The buffer lines to render
/// * `results` - The evaluation results corresponding to each line
/// * `scroll_offset` - The first visible line index (0-based)
/// * `visible_height` - The number of visible lines in the viewport
/// * `current_row` - The row index where the cursor is positioned (0-indexed)
///
/// # Returns
/// A vector of styled `Line` objects for the visible portion only.
#[must_use]
pub fn build_visible_input_lines_with_highlight<'a>(
    lines: &'a [String],
    results: &'a [LineResult],
    scroll_offset: usize,
    visible_height: usize,
    current_row: usize,
) -> Vec<Line<'a>> {
    let mut output: Vec<Line<'a>> = Vec::new();
    let highlight_style = current_line_highlight_style();

    // Calculate the range of lines to render
    let start = scroll_offset.min(lines.len());
    let end = (scroll_offset + visible_height).min(lines.len());

    for (i, line_text) in lines.iter().enumerate().take(end).skip(start) {
        let result = results.get(i);
        let is_current_line = i == current_row;

        // Build the main line with potential error or syntax highlighting
        let mut styled_line = match result {
            Some(LineResult::Error(err)) => build_error_line(line_text, err),
            _ => Line::from(highlight_line(line_text)),
        };

        // Apply current line highlight
        if is_current_line {
            styled_line = styled_line.style(highlight_style);
        }

        output.push(styled_line);

        // Add error message below error lines (without highlight)
        if let Some(LineResult::Error(err)) = result {
            let error_line = Line::from(Span::styled(
                format!("  ^ {}", err.message()),
                Style::default().fg(Color::Red),
            ));
            output.push(error_line);
        }
    }

    output
}

/// Builds visible result lines with scrolling and current line highlighting.
///
/// Combines scrolling support with current line highlighting for the results panel.
///
/// # Arguments
/// * `results` - The evaluation results to display
/// * `scroll_offset` - The first visible line index (0-based)
/// * `visible_height` - The number of visible lines in the viewport
/// * `current_row` - The row index where the cursor is positioned (0-indexed)
///
/// # Returns
/// A vector of styled `Line` objects for the visible portion only.
#[must_use]
pub fn build_visible_result_lines_with_highlight(
    results: &[LineResult],
    scroll_offset: usize,
    visible_height: usize,
    current_row: usize,
) -> Vec<Line<'_>> {
    let highlight_style = current_line_highlight_style();

    // Calculate the range of results to render
    let start = scroll_offset.min(results.len());
    let end = (scroll_offset + visible_height).min(results.len());

    results[start..end]
        .iter()
        .enumerate()
        .map(|(visible_idx, result)| {
            let actual_idx = start + visible_idx;
            let is_current_line = actual_idx == current_row;

            let mut line = format_result(result).map_or_else(
                || Line::from(""),
                |text| Line::from(Span::styled(text, Style::default().fg(Color::Green))),
            );

            if is_current_line {
                line = line.style(highlight_style);
            }

            line
        })
        .collect()
}

/// Returns the style used for highlighting the current line.
///
/// Uses a subtle dark gray background that works well in terminal themes.
#[must_use]
pub fn current_line_highlight_style() -> Style {
    Style::default().bg(Color::Rgb(50, 50, 50))
}

/// Builds styled text lines for the input panel with current line highlighting.
///
/// Handles:
/// - Normal text styling
/// - Error spans with red underline
/// - Error messages below error lines
/// - Background highlighting for the current cursor row
///
/// # Arguments
/// * `lines` - The buffer lines to render
/// * `results` - The evaluation results corresponding to each line
/// * `current_row` - The row index where the cursor is positioned (0-indexed)
///
/// # Returns
/// A vector of styled `Line` objects ready for rendering.
#[must_use]
pub fn build_input_lines_with_highlight<'a>(
    lines: &'a [String],
    results: &'a [LineResult],
    current_row: usize,
) -> Vec<Line<'a>> {
    let mut output: Vec<Line<'a>> = Vec::new();
    let highlight_style = current_line_highlight_style();

    for (i, line_text) in lines.iter().enumerate() {
        let result = results.get(i);
        let is_current_line = i == current_row;

        // Build the main line with potential error or syntax highlighting
        let mut styled_line = match result {
            Some(LineResult::Error(err)) => build_error_line(line_text, err),
            _ => Line::from(highlight_line(line_text)),
        };

        // Apply current line highlight
        if is_current_line {
            styled_line = styled_line.style(highlight_style);
        }

        output.push(styled_line);

        // Add error message below error lines (without highlight)
        if let Some(LineResult::Error(err)) = result {
            let error_line = Line::from(Span::styled(
                format!("  ^ {}", err.message()),
                Style::default().fg(Color::Red),
            ));
            output.push(error_line);
        }
    }

    output
}

/// Builds styled text lines for the result panel with current line highlighting.
///
/// Results are aligned with their corresponding input lines.
/// Empty lines and error lines show nothing but still receive highlighting.
///
/// # Arguments
/// * `results` - The evaluation results to display
/// * `current_row` - The row index where the cursor is positioned (0-indexed)
///
/// # Returns
/// A vector of styled `Line` objects ready for rendering.
#[must_use]
pub fn build_result_lines_with_highlight(
    results: &[LineResult],
    current_row: usize,
) -> Vec<Line<'_>> {
    let highlight_style = current_line_highlight_style();

    results
        .iter()
        .enumerate()
        .map(|(i, result)| {
            let is_current_line = i == current_row;

            let mut line = format_result(result).map_or_else(
                || Line::from(""),
                |text| Line::from(Span::styled(text, Style::default().fg(Color::Green))),
            );

            if is_current_line {
                line = line.style(highlight_style);
            }

            line
        })
        .collect()
}

// ============================================================
// Line Number Gutter Functions
// ============================================================

/// Calculates the width needed for the line number gutter.
///
/// The gutter width includes space for the line numbers (right-aligned)
/// plus one space for separation from the content.
///
/// # Arguments
/// * `line_count` - The total number of lines in the buffer
///
/// # Returns
/// The width in characters needed for the gutter (digits + 1 space).
#[must_use]
pub const fn calculate_gutter_width(line_count: usize) -> usize {
    // Calculate digits needed for the largest line number using integer math
    let digits = if line_count == 0 {
        1 // At least 1 digit even for empty buffer
    } else {
        // Count digits by repeatedly dividing by 10
        let mut n = line_count;
        let mut count = 0;
        while n > 0 {
            count += 1;
            n /= 10;
        }
        count
    };
    // Add 1 for the trailing space separator
    digits + 1
}

/// Formats a line number for display in the gutter.
///
/// Line numbers are right-aligned within the gutter width,
/// with a trailing space to separate from content.
///
/// # Arguments
/// * `line_number` - The 1-based line number to format
/// * `gutter_width` - The total gutter width (including trailing space)
///
/// # Returns
/// A string with the line number right-aligned and a trailing space.
#[must_use]
pub fn format_line_number(line_number: usize, gutter_width: usize) -> String {
    // gutter_width includes trailing space, so actual number width is gutter_width - 1
    let number_width = gutter_width - 1;
    format!("{line_number:>number_width$} ")
}

/// Returns the style for the line number gutter.
///
/// The gutter uses a subtle dimmed foreground color to keep line numbers
/// visible but unobtrusive. No distinct background is used, allowing the
/// gutter to blend with the content area.
#[must_use]
pub fn gutter_style() -> Style {
    Style::default().fg(Color::DarkGray)
}

/// Builds spans for a line with error highlighting (without wrapping in Line).
///
/// If the error has a span, only that portion is underlined.
/// Otherwise, the entire line is underlined.
fn build_error_spans<'a>(line_text: &'a str, error: &EvalError) -> Vec<Span<'a>> {
    let error_style = Style::default()
        .fg(Color::Red)
        .add_modifier(Modifier::UNDERLINED);

    error.span().map_or_else(
        // No span available, underline entire line
        || vec![Span::styled(line_text, error_style)],
        |span| {
            // Clamp span to line bounds
            let start = span.start.min(line_text.len());
            let end = span.end.min(line_text.len()).max(start);

            let mut spans = Vec::new();

            // Text before error
            if start > 0 {
                spans.push(Span::raw(&line_text[..start]));
            }

            // Error portion with underline
            if start < end {
                spans.push(Span::styled(&line_text[start..end], error_style));
            }

            // Text after error
            if end < line_text.len() {
                spans.push(Span::raw(&line_text[end..]));
            }

            spans
        },
    )
}

/// Builds styled text lines for the input panel with line number gutter.
///
/// Handles:
/// - Line numbers in a gutter with distinct styling
/// - Normal text styling
/// - Error spans with red underline
/// - Error messages below error lines (without line numbers)
///
/// # Arguments
/// * `lines` - The buffer lines to render
/// * `results` - The evaluation results corresponding to each line
///
/// # Returns
/// A tuple of (styled lines, gutter width) for rendering.
#[must_use]
#[allow(dead_code)] // Used for non-scrolled rendering and testing
pub fn build_input_lines_with_gutter<'a>(
    lines: &'a [String],
    results: &'a [LineResult],
) -> (Vec<Line<'a>>, usize) {
    let gutter_width = calculate_gutter_width(lines.len());
    let gutter_style_val = gutter_style();
    let mut output: Vec<Line<'a>> = Vec::new();

    for (i, line_text) in lines.iter().enumerate() {
        let line_number = i + 1; // 1-based line numbers
        let result = results.get(i);

        // Build the line number span
        let line_num_str = format_line_number(line_number, gutter_width);
        let line_num_span = Span::styled(line_num_str, gutter_style_val);

        // Build the main line with potential error or syntax highlighting
        let content_spans = match result {
            Some(LineResult::Error(err)) => build_error_spans(line_text, err),
            _ => highlight_line(line_text),
        };

        // Combine line number and content
        let mut all_spans = vec![line_num_span];
        all_spans.extend(content_spans);
        output.push(Line::from(all_spans));

        // Add error message below error lines (indented, no line number)
        if let Some(LineResult::Error(err)) = result {
            // Create indentation matching gutter width
            let indent = " ".repeat(gutter_width);
            let error_line = Line::from(Span::styled(
                format!("{}  ^ {}", indent, err.message()),
                Style::default().fg(Color::Red),
            ));
            output.push(error_line);
        }
    }

    (output, gutter_width)
}

/// Builds visible input lines with scrolling, highlighting, and line number gutter.
///
/// This combines scrolling, current line highlighting, and line numbers.
///
/// # Arguments
/// * `lines` - The buffer lines to render
/// * `results` - The evaluation results corresponding to each line
/// * `scroll_offset` - The first visible line index (0-based)
/// * `visible_height` - The number of visible lines in the viewport
/// * `current_row` - The row index where the cursor is positioned (0-indexed)
///
/// # Returns
/// A tuple of (styled lines, gutter width) for rendering.
#[must_use]
pub fn build_visible_input_lines_with_gutter<'a>(
    lines: &'a [String],
    results: &'a [LineResult],
    scroll_offset: usize,
    visible_height: usize,
    current_row: usize,
) -> (Vec<Line<'a>>, usize) {
    let gutter_width = calculate_gutter_width(lines.len());
    let gutter_style_val = gutter_style();
    let highlight_style = current_line_highlight_style();
    let mut output: Vec<Line<'a>> = Vec::new();

    // Calculate the range of lines to render
    let start = scroll_offset.min(lines.len());
    let end = (scroll_offset + visible_height).min(lines.len());

    for (i, line_text) in lines.iter().enumerate().take(end).skip(start) {
        let line_number = i + 1; // 1-based line numbers
        let result = results.get(i);
        let is_current_line = i == current_row;

        // Build the line number span
        let line_num_str = format_line_number(line_number, gutter_width);
        let line_num_span = Span::styled(line_num_str, gutter_style_val);

        // Build the main line with potential error or syntax highlighting
        let content_spans = match result {
            Some(LineResult::Error(err)) => build_error_spans(line_text, err),
            _ => highlight_line(line_text),
        };

        // Combine line number and content
        let mut all_spans = vec![line_num_span];
        all_spans.extend(content_spans);
        let mut styled_line = Line::from(all_spans);

        // Apply current line highlight
        if is_current_line {
            styled_line = styled_line.style(highlight_style);
        }

        output.push(styled_line);

        // Add error message below error lines (indented, no line number, no highlight)
        if let Some(LineResult::Error(err)) = result {
            // Create indentation matching gutter width
            let indent = " ".repeat(gutter_width);
            let error_line = Line::from(Span::styled(
                format!("{}  ^ {}", indent, err.message()),
                Style::default().fg(Color::Red),
            ));
            output.push(error_line);
        }
    }

    (output, gutter_width)
}

/// Creates a Block widget for the input panel with rounded borders and dark grey styling.
///
/// # Returns
/// A Block configured with:
/// - Title "Input"
/// - All borders enabled
/// - Rounded border type
/// - Dark grey border color
#[must_use]
pub fn input_panel_block() -> Block<'static> {
    Block::default()
        .title("Input")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray))
}

/// Creates a Block widget for the result panel with rounded borders and dark grey styling.
///
/// # Returns
/// A Block configured with:
/// - Title "Results"
/// - All borders enabled
/// - Rounded border type
/// - Dark grey border color
#[must_use]
pub fn result_panel_block() -> Block<'static> {
    Block::default()
        .title("Results")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray))
}

/// Renders the input panel with buffer content, error highlighting, current line highlighting,
/// line number gutter, and scrolling support.
///
/// # Arguments
/// * `frame` - The ratatui Frame to render to
/// * `area` - The area to render the panel in
/// * `buffer` - The text buffer containing input lines
/// * `scroll_offset` - The first visible line index (0-based)
pub fn render_input_panel(frame: &mut Frame, area: Rect, buffer: &Buffer, scroll_offset: usize) {
    // Evaluate all lines to get results
    let results = evaluate_all_lines(buffer.lines().iter().map(String::as_str));

    // Get cursor row for highlighting
    let cursor_row = buffer.cursor().row();

    // Calculate visible height (area height minus borders)
    let visible_height = area.height.saturating_sub(2) as usize;

    // Build styled lines for visible portion with line number gutter
    let (styled_lines, gutter_width) = build_visible_input_lines_with_gutter(
        buffer.lines(),
        &results,
        scroll_offset,
        visible_height,
        cursor_row,
    );

    // Create the paragraph widget with rounded borders and dark grey styling
    let paragraph = Paragraph::new(Text::from(styled_lines)).block(input_panel_block());

    frame.render_widget(paragraph, area);

    // Set cursor position (inside the border, accounting for gutter)
    let cursor_col = buffer.cursor().col();

    // Account for error messages that push lines down (within visible range only)
    let mut actual_row = 0;
    for i in scroll_offset..cursor_row.min(scroll_offset + visible_height) {
        if i == cursor_row {
            break;
        }
        actual_row += 1;
        if matches!(results.get(i), Some(LineResult::Error(_))) {
            actual_row += 1; // Error message line
        }
    }

    // Position cursor accounting for border (1 pixel) and gutter width
    let cursor_x = area.x
        + 1
        + u16::try_from(gutter_width).unwrap_or(0)
        + u16::try_from(cursor_col).unwrap_or(0);
    let cursor_y = area.y + 1 + u16::try_from(actual_row).unwrap_or(0);

    // Only set cursor if it's within the visible area
    if cursor_x < area.x + area.width - 1 && cursor_y < area.y + area.height - 1 {
        frame.set_cursor_position((cursor_x, cursor_y));
    }
}

/// Renders the result panel with evaluation results, current line highlighting, and scrolling.
///
/// Results are aligned with their corresponding input lines.
/// The line at `current_row` is highlighted to match the editor cursor position.
/// Only visible lines (based on `scroll_offset`) are rendered.
///
/// # Arguments
/// * `frame` - The ratatui Frame to render to
/// * `area` - The area to render the panel in
/// * `results` - The evaluation results to display
/// * `current_row` - The row index where the cursor is positioned (0-indexed)
/// * `scroll_offset` - The first visible line index (0-based)
pub fn render_result_panel(
    frame: &mut Frame,
    area: Rect,
    results: &[LineResult],
    current_row: usize,
    scroll_offset: usize,
) {
    // Calculate visible height (area height minus borders)
    let visible_height = area.height.saturating_sub(2) as usize;

    let styled_lines = build_visible_result_lines_with_highlight(
        results,
        scroll_offset,
        visible_height,
        current_row,
    );

    // Create the paragraph widget with rounded borders and dark grey styling
    let paragraph = Paragraph::new(Text::from(styled_lines)).block(result_panel_block());

    frame.render_widget(paragraph, area);
}

/// Builds the styled text line for the command bar.
///
/// Returns a Line containing all keyboard shortcuts with consistent styling.
/// Keys are highlighted in yellow bold, descriptions are plain text.
#[must_use]
pub fn build_command_bar_text<'a>() -> Line<'a> {
    let key_style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);

    Line::from(vec![
        Span::styled("q", key_style),
        Span::raw(": quit  "),
        Span::styled("c", key_style),
        Span::raw(": clear  "),
        Span::styled("↑↓", key_style),
        Span::raw(": history"),
    ])
}

/// Renders the command bar at the bottom of the screen.
///
/// Displays available keyboard commands: "q: quit  c: clear  ↑↓: history"
///
/// # Arguments
/// * `frame` - The ratatui Frame to render to
/// * `area` - The area to render the command bar in (should be 1 row)
pub fn render_command_bar(frame: &mut Frame, area: Rect) {
    let command_text = build_command_bar_text();
    let command_bar = Paragraph::new(command_text).style(Style::default().bg(Color::DarkGray));

    frame.render_widget(command_bar, area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval::ErrorSpan;

    // ============================================================
    // format_result tests
    // ============================================================

    #[test]
    fn test_format_result_integer_value() {
        let result = LineResult::Value(Value::Int(42));
        assert_eq!(format_result(&result), Some("42".to_string()));
    }

    #[test]
    fn test_format_result_negative_integer() {
        let result = LineResult::Value(Value::Int(-123));
        assert_eq!(format_result(&result), Some("-123".to_string()));
    }

    #[test]
    fn test_format_result_float_value() {
        let result = LineResult::Value(Value::Float(2.75));
        assert_eq!(format_result(&result), Some("2.75".to_string()));
    }

    #[test]
    fn test_format_result_whole_float_displays_without_decimal() {
        let result = LineResult::Value(Value::Float(5.0));
        assert_eq!(format_result(&result), Some("5".to_string()));
    }

    #[test]
    fn test_format_result_assignment() {
        let result = LineResult::Assignment {
            name: "x".to_string(),
            value: Value::Int(10),
        };
        assert_eq!(format_result(&result), Some("x = 10".to_string()));
    }

    #[test]
    fn test_format_result_assignment_with_float() {
        let result = LineResult::Assignment {
            name: "rate".to_string(),
            value: Value::Float(1.23456),
        };
        assert_eq!(format_result(&result), Some("rate = 1.23456".to_string()));
    }

    #[test]
    fn test_format_result_empty_returns_none() {
        let result = LineResult::Empty;
        assert_eq!(format_result(&result), None);
    }

    #[test]
    fn test_format_result_error_returns_none() {
        let result = LineResult::Error(EvalError::new("test error"));
        assert_eq!(format_result(&result), None);
    }

    // ============================================================
    // format_value tests
    // ============================================================

    #[test]
    fn test_format_value_large_integer() {
        let value = Value::Int(1_000_000);
        assert_eq!(format_value(&value), "1000000");
    }

    #[test]
    fn test_format_value_zero() {
        let value = Value::Int(0);
        assert_eq!(format_value(&value), "0");
    }

    #[test]
    fn test_format_value_small_float() {
        let value = Value::Float(0.001);
        assert_eq!(format_value(&value), "0.001");
    }

    // ============================================================
    // build_input_lines tests
    // ============================================================

    #[test]
    fn test_build_input_lines_single_line_no_error() {
        let lines = vec!["5 + 3".to_string()];
        let results = vec![LineResult::Value(Value::Int(8))];

        let output = build_input_lines(&lines, &results);

        assert_eq!(output.len(), 1);
    }

    #[test]
    fn test_build_input_lines_adds_error_message() {
        let lines = vec!["invalid".to_string()];
        let results = vec![LineResult::Error(EvalError::new("undefined variable"))];

        let output = build_input_lines(&lines, &results);

        // Should have 2 lines: the error line and the error message
        assert_eq!(output.len(), 2);
    }

    #[test]
    fn test_build_input_lines_multiple_lines_with_error() {
        let lines = vec![
            "5 + 3".to_string(),
            "invalid".to_string(),
            "10 - 2".to_string(),
        ];
        let results = vec![
            LineResult::Value(Value::Int(8)),
            LineResult::Error(EvalError::new("undefined variable")),
            LineResult::Value(Value::Int(8)),
        ];

        let output = build_input_lines(&lines, &results);

        // Line 1 + Error line + Error message + Line 3 = 4 total
        assert_eq!(output.len(), 4);
    }

    #[test]
    fn test_build_input_lines_empty_line() {
        let lines = vec![String::new()];
        let results = vec![LineResult::Empty];

        let output = build_input_lines(&lines, &results);

        assert_eq!(output.len(), 1);
    }

    // ============================================================
    // build_error_line tests
    // ============================================================

    #[test]
    fn test_build_error_line_no_span_underlines_entire_line() {
        let line = "invalid expression";
        let error = EvalError::new("syntax error");

        let styled_line = build_error_line(line, &error);

        // The line should have one span (the entire line styled)
        assert_eq!(styled_line.spans.len(), 1);
    }

    #[test]
    fn test_build_error_line_with_span_highlights_portion() {
        let line = "5 + abc + 3";
        let error = EvalError::with_span("undefined variable", ErrorSpan::new(4, 7));

        let styled_line = build_error_line(line, &error);

        // Should have 3 spans: before (5 + ), error (abc), after ( + 3)
        assert_eq!(styled_line.spans.len(), 3);
    }

    #[test]
    fn test_build_error_line_span_at_start() {
        let line = "abc + 5";
        let error = EvalError::with_span("undefined variable", ErrorSpan::new(0, 3));

        let styled_line = build_error_line(line, &error);

        // Should have 2 spans: error (abc), after ( + 5)
        assert_eq!(styled_line.spans.len(), 2);
    }

    #[test]
    fn test_build_error_line_span_at_end() {
        let line = "5 + abc";
        let error = EvalError::with_span("undefined variable", ErrorSpan::new(4, 7));

        let styled_line = build_error_line(line, &error);

        // Should have 2 spans: before (5 + ), error (abc)
        assert_eq!(styled_line.spans.len(), 2);
    }

    #[test]
    fn test_build_error_line_span_beyond_line_length_is_clamped() {
        let line = "abc";
        let error = EvalError::with_span("error", ErrorSpan::new(0, 100));

        let styled_line = build_error_line(line, &error);

        // Should clamp to line length and have 1 span
        assert_eq!(styled_line.spans.len(), 1);
    }

    // ============================================================
    // build_result_lines tests
    // ============================================================

    #[test]
    fn test_build_result_lines_values() {
        let results = vec![
            LineResult::Value(Value::Int(8)),
            LineResult::Value(Value::Float(2.75)),
        ];

        let output = build_result_lines(&results);

        assert_eq!(output.len(), 2);
    }

    #[test]
    fn test_build_result_lines_empty_line_produces_empty_string() {
        let results = vec![LineResult::Empty];

        let output = build_result_lines(&results);

        assert_eq!(output.len(), 1);
        // The line should be empty
        assert!(output[0].spans.is_empty() || output[0].to_string().is_empty());
    }

    #[test]
    fn test_build_result_lines_error_produces_empty_string() {
        let results = vec![LineResult::Error(EvalError::new("error"))];

        let output = build_result_lines(&results);

        assert_eq!(output.len(), 1);
        // The line should be empty (errors shown in input panel)
        assert!(output[0].spans.is_empty() || output[0].to_string().is_empty());
    }

    #[test]
    fn test_build_result_lines_mixed() {
        let results = vec![
            LineResult::Value(Value::Int(8)),
            LineResult::Empty,
            LineResult::Error(EvalError::new("error")),
            LineResult::Assignment {
                name: "x".to_string(),
                value: Value::Int(5),
            },
        ];

        let output = build_result_lines(&results);

        assert_eq!(output.len(), 4);
    }

    #[test]
    fn test_build_result_lines_assignment_format() {
        let results = vec![LineResult::Assignment {
            name: "result".to_string(),
            value: Value::Int(42),
        }];

        let output = build_result_lines(&results);

        assert_eq!(output.len(), 1);
        // Check that the formatted output contains the assignment
        let line_str = output[0].to_string();
        assert!(line_str.contains("result"));
        assert!(line_str.contains("42"));
    }

    // ============================================================
    // Current line highlighting tests
    // ============================================================

    #[test]
    fn test_build_input_lines_with_highlight_highlights_current_row() {
        let lines = vec!["5 + 3".to_string(), "10 * 2".to_string()];
        let results = vec![
            LineResult::Value(Value::Int(8)),
            LineResult::Value(Value::Int(20)),
        ];
        let current_row = 0;

        let output = build_input_lines_with_highlight(&lines, &results, current_row);

        // Line 0 should be highlighted
        assert_eq!(output.len(), 2);
        // First line should have the highlight style
        assert!(output[0].style.bg.is_some());
        // Second line should not have highlight
        assert!(output[1].style.bg.is_none());
    }

    #[test]
    fn test_build_input_lines_with_highlight_second_row() {
        let lines = vec!["5 + 3".to_string(), "10 * 2".to_string()];
        let results = vec![
            LineResult::Value(Value::Int(8)),
            LineResult::Value(Value::Int(20)),
        ];
        let current_row = 1;

        let output = build_input_lines_with_highlight(&lines, &results, current_row);

        // First line should not have highlight
        assert!(output[0].style.bg.is_none());
        // Second line should have the highlight style
        assert!(output[1].style.bg.is_some());
    }

    #[test]
    fn test_build_input_lines_with_highlight_error_line_still_highlighted() {
        let lines = vec!["invalid".to_string()];
        let results = vec![LineResult::Error(EvalError::new("undefined variable"))];
        let current_row = 0;

        let output = build_input_lines_with_highlight(&lines, &results, current_row);

        // Should have 2 lines: the error line and the error message
        assert_eq!(output.len(), 2);
        // First line (error line) should be highlighted
        assert!(output[0].style.bg.is_some());
        // Error message line should not be highlighted (it's not an actual input line)
        assert!(output[1].style.bg.is_none());
    }

    #[test]
    fn test_build_result_lines_with_highlight_highlights_current_row() {
        let results = vec![
            LineResult::Value(Value::Int(8)),
            LineResult::Value(Value::Int(20)),
        ];
        let current_row = 0;

        let output = build_result_lines_with_highlight(&results, current_row);

        assert_eq!(output.len(), 2);
        // First line should have highlight
        assert!(output[0].style.bg.is_some());
        // Second line should not have highlight
        assert!(output[1].style.bg.is_none());
    }

    #[test]
    fn test_build_result_lines_with_highlight_second_row() {
        let results = vec![
            LineResult::Value(Value::Int(8)),
            LineResult::Value(Value::Int(20)),
        ];
        let current_row = 1;

        let output = build_result_lines_with_highlight(&results, current_row);

        // First line should not have highlight
        assert!(output[0].style.bg.is_none());
        // Second line should have highlight
        assert!(output[1].style.bg.is_some());
    }

    #[test]
    fn test_build_result_lines_with_highlight_empty_line_highlighted() {
        let results = vec![LineResult::Empty];
        let current_row = 0;

        let output = build_result_lines_with_highlight(&results, current_row);

        assert_eq!(output.len(), 1);
        // Empty line should still be highlighted when it's the current row
        assert!(output[0].style.bg.is_some());
    }

    #[test]
    fn test_current_line_highlight_style_is_subtle() {
        // Verify the highlight color is a subtle dark gray
        let style = current_line_highlight_style();
        assert!(style.bg.is_some());
        // The background should be set to a gray color
        if let Some(Color::Rgb(r, g, b)) = style.bg {
            // Should be a subtle gray (values around 40-60 for dark theme)
            assert!(r == g && g == b, "Highlight should be gray (r=g=b)");
            assert!(r < 100, "Highlight should be subtle/dark");
        }
    }

    // ============================================================
    // Scrollable input lines tests
    // ============================================================

    #[test]
    fn test_build_visible_input_lines_returns_slice() {
        let lines: Vec<String> = (0..10).map(|i| format!("line {i}")).collect();
        let results: Vec<LineResult> = (0..10).map(|_| LineResult::Empty).collect();
        let scroll_offset = 2;
        let visible_height = 3;

        let output = build_visible_input_lines(&lines, &results, scroll_offset, visible_height);

        // Should return only lines 2, 3, 4 (3 lines starting at offset 2)
        assert_eq!(output.len(), 3);
    }

    #[test]
    fn test_build_visible_input_lines_with_zero_offset() {
        let lines: Vec<String> = (0..5).map(|i| format!("line {i}")).collect();
        let results: Vec<LineResult> = (0..5).map(|_| LineResult::Empty).collect();
        let scroll_offset = 0;
        let visible_height = 3;

        let output = build_visible_input_lines(&lines, &results, scroll_offset, visible_height);

        // Should return lines 0, 1, 2
        assert_eq!(output.len(), 3);
    }

    #[test]
    fn test_build_visible_input_lines_at_end_of_buffer() {
        let lines: Vec<String> = (0..5).map(|i| format!("line {i}")).collect();
        let results: Vec<LineResult> = (0..5).map(|_| LineResult::Empty).collect();
        let scroll_offset = 3;
        let visible_height = 10; // More than available

        let output = build_visible_input_lines(&lines, &results, scroll_offset, visible_height);

        // Should return only lines 3, 4 (remaining lines)
        assert_eq!(output.len(), 2);
    }

    #[test]
    fn test_build_visible_input_lines_includes_error_messages() {
        let lines = vec![
            "5 + 3".to_string(),
            "invalid".to_string(),
            "10 - 2".to_string(),
        ];
        let results = vec![
            LineResult::Value(Value::Int(8)),
            LineResult::Error(EvalError::new("undefined variable")),
            LineResult::Value(Value::Int(8)),
        ];
        let scroll_offset = 0;
        let visible_height = 10;

        let output = build_visible_input_lines(&lines, &results, scroll_offset, visible_height);

        // Line 0 + Line 1 (error) + error message + Line 2 = 4 lines total
        assert_eq!(output.len(), 4);
    }

    // ============================================================
    // Scrollable result lines tests
    // ============================================================

    #[test]
    fn test_build_visible_result_lines_returns_slice() {
        let results: Vec<LineResult> = (0..10).map(|i| LineResult::Value(Value::Int(i))).collect();
        let scroll_offset = 2;
        let visible_height = 3;

        let output = build_visible_result_lines(&results, scroll_offset, visible_height);

        // Should return only results 2, 3, 4 (3 items starting at offset 2)
        assert_eq!(output.len(), 3);
    }

    #[test]
    fn test_build_visible_result_lines_at_start() {
        let results: Vec<LineResult> = (0..5).map(|i| LineResult::Value(Value::Int(i))).collect();
        let scroll_offset = 0;
        let visible_height = 3;

        let output = build_visible_result_lines(&results, scroll_offset, visible_height);

        // Should return results 0, 1, 2
        assert_eq!(output.len(), 3);
    }

    #[test]
    fn test_build_visible_result_lines_at_end() {
        let results: Vec<LineResult> = (0..5).map(|i| LineResult::Value(Value::Int(i))).collect();
        let scroll_offset = 3;
        let visible_height = 10; // More than available

        let output = build_visible_result_lines(&results, scroll_offset, visible_height);

        // Should return only results 3, 4 (remaining results)
        assert_eq!(output.len(), 2);
    }

    // ============================================================
    // Line Number Gutter tests
    // ============================================================

    #[test]
    fn test_calculate_gutter_width_single_digit_lines() {
        // 1-9 lines need 1 char plus 1 space = 2
        assert_eq!(calculate_gutter_width(1), 2);
        assert_eq!(calculate_gutter_width(9), 2);
    }

    #[test]
    fn test_calculate_gutter_width_double_digit_lines() {
        // 10-99 lines need 2 chars plus 1 space = 3
        assert_eq!(calculate_gutter_width(10), 3);
        assert_eq!(calculate_gutter_width(99), 3);
    }

    #[test]
    fn test_calculate_gutter_width_triple_digit_lines() {
        // 100-999 lines need 3 chars plus 1 space = 4
        assert_eq!(calculate_gutter_width(100), 4);
        assert_eq!(calculate_gutter_width(999), 4);
    }

    #[test]
    fn test_calculate_gutter_width_four_digit_lines() {
        // 1000-9999 lines need 4 chars plus 1 space = 5
        assert_eq!(calculate_gutter_width(1000), 5);
        assert_eq!(calculate_gutter_width(9999), 5);
    }

    #[test]
    fn test_calculate_gutter_width_minimum() {
        // Even 0 lines should have minimum width for display
        assert_eq!(calculate_gutter_width(0), 2);
    }

    #[test]
    fn test_format_line_number_right_aligned() {
        // Line numbers should be right-aligned within gutter width
        // Gutter width of 3 (for up to 99 lines)
        assert_eq!(format_line_number(1, 3), " 1 ");
        assert_eq!(format_line_number(9, 3), " 9 ");
        assert_eq!(format_line_number(10, 3), "10 ");
        assert_eq!(format_line_number(99, 3), "99 ");
    }

    #[test]
    fn test_format_line_number_single_digit_width() {
        // Gutter width of 2 (for up to 9 lines)
        assert_eq!(format_line_number(1, 2), "1 ");
        assert_eq!(format_line_number(9, 2), "9 ");
    }

    #[test]
    fn test_format_line_number_four_digit() {
        // Gutter width of 5 (for up to 9999 lines)
        assert_eq!(format_line_number(1, 5), "   1 ");
        assert_eq!(format_line_number(999, 5), " 999 ");
        assert_eq!(format_line_number(1234, 5), "1234 ");
    }

    #[test]
    fn test_gutter_style_uses_subtle_styling() {
        let style = gutter_style();
        // Gutter should NOT have a background color (blends with content area)
        assert!(
            style.bg.is_none(),
            "Gutter should not have a distinct background color"
        );
        // Gutter should have a dimmed foreground color (DarkGray)
        assert_eq!(
            style.fg,
            Some(Color::DarkGray),
            "Gutter should use DarkGray foreground"
        );
    }

    #[test]
    fn test_build_input_lines_with_gutter_includes_line_numbers() {
        let lines = vec!["5 + 3".to_string(), "10 - 2".to_string()];
        let results = vec![
            LineResult::Value(Value::Int(8)),
            LineResult::Value(Value::Int(8)),
        ];

        let (output, gutter_width) = build_input_lines_with_gutter(&lines, &results);

        // Should have 2 output lines (no errors)
        assert_eq!(output.len(), 2);
        // Gutter width should be 2 (for 2 lines = single digit + space)
        assert_eq!(gutter_width, 2);
        // Each line should start with a line number
        let line1_str = output[0].to_string();
        let line2_str = output[1].to_string();
        assert!(
            line1_str.starts_with("1 "),
            "First line should start with '1 '"
        );
        assert!(
            line2_str.starts_with("2 "),
            "Second line should start with '2 '"
        );
    }

    #[test]
    fn test_build_input_lines_with_gutter_error_line_has_number() {
        let lines = vec!["invalid".to_string()];
        let results = vec![LineResult::Error(EvalError::new("undefined variable"))];

        let (output, gutter_width) = build_input_lines_with_gutter(&lines, &results);

        // Should have 2 lines: error line + error message
        assert_eq!(output.len(), 2);
        assert_eq!(gutter_width, 2);
        // Error line should have line number
        let line1_str = output[0].to_string();
        assert!(
            line1_str.starts_with("1 "),
            "Error line should start with '1 '"
        );
        // Error message line should NOT have a line number (indented continuation)
        let line2_str = output[1].to_string();
        assert!(
            line2_str.starts_with("  "),
            "Error message should be indented, not numbered"
        );
    }

    #[test]
    fn test_build_input_lines_with_gutter_many_lines() {
        // Test with 100 lines to verify gutter width calculation
        let lines: Vec<String> = (1..=100).map(|i| format!("line {i}")).collect();
        let results: Vec<LineResult> = (0..100).map(|_| LineResult::Empty).collect();

        let (output, gutter_width) = build_input_lines_with_gutter(&lines, &results);

        assert_eq!(output.len(), 100);
        // 100 lines = 3 digits + 1 space = 4
        assert_eq!(gutter_width, 4);
        // First line should be right-aligned
        let line1_str = output[0].to_string();
        assert!(
            line1_str.starts_with("  1 "),
            "Line 1 should be right-aligned as '  1 '"
        );
        // Line 100 should use full width
        let line100_str = output[99].to_string();
        assert!(
            line100_str.starts_with("100 "),
            "Line 100 should start with '100 '"
        );
    }

    #[test]
    fn test_build_visible_input_lines_with_gutter_returns_correct_width() {
        let lines: Vec<String> = (0..50).map(|i| format!("line {i}")).collect();
        let results: Vec<LineResult> = (0..50).map(|_| LineResult::Empty).collect();

        let (output, gutter_width) =
            build_visible_input_lines_with_gutter(&lines, &results, 0, 10, 0);

        // Should return only 10 visible lines
        assert_eq!(output.len(), 10);
        // Gutter width is based on total lines (50), not visible lines
        // 50 lines = 2 digits + 1 space = 3
        assert_eq!(gutter_width, 3);
    }

    #[test]
    fn test_build_visible_input_lines_with_gutter_scrolled_shows_correct_numbers() {
        let lines: Vec<String> = (0..20).map(|i| format!("line {i}")).collect();
        let results: Vec<LineResult> = (0..20).map(|_| LineResult::Empty).collect();

        let (output, _) = build_visible_input_lines_with_gutter(&lines, &results, 10, 5, 12);

        // Should return 5 lines starting at offset 10
        assert_eq!(output.len(), 5);
        // First visible line should show line 11 (1-indexed)
        let line1_str = output[0].to_string();
        assert!(
            line1_str.starts_with("11 "),
            "First visible line should show line number 11"
        );
    }

    #[test]
    fn test_build_visible_input_lines_with_gutter_highlights_current() {
        let lines = vec!["line 1".to_string(), "line 2".to_string()];
        let results = vec![LineResult::Empty, LineResult::Empty];

        let (output, _) = build_visible_input_lines_with_gutter(&lines, &results, 0, 10, 1);

        // First line should not be highlighted
        assert!(output[0].style.bg.is_none());
        // Second line (current_row = 1) should be highlighted
        assert!(output[1].style.bg.is_some());
    }

    // ============================================================
    // Panel Border Styling tests
    // ============================================================

    #[test]
    fn test_input_panel_block_returns_valid_block() {
        // Verify input_panel_block returns a Block configured with rounded borders
        // and dark grey styling. Since Block's internal state is not accessible,
        // we verify it compiles and can be rendered (indirectly tested by render functions).
        let block = input_panel_block();
        // The block should have borders configured (verified by existence)
        // Type assertion: this compiles only if input_panel_block returns Block
        let _: Block = block;
    }

    #[test]
    fn test_result_panel_block_returns_valid_block() {
        // Verify result_panel_block returns a Block configured with rounded borders
        // and dark grey styling. Since Block's internal state is not accessible,
        // we verify it compiles and can be rendered (indirectly tested by render functions).
        let block = result_panel_block();
        // Type assertion: this compiles only if result_panel_block returns Block
        let _: Block = block;
    }

    #[test]
    fn test_both_panel_blocks_have_same_border_configuration() {
        // Both panels should have consistent border styling.
        // This test ensures both functions exist and return valid Blocks.
        // The actual border configuration (rounded, dark grey) is specified in code
        // and verified visually or through integration tests.
        let input_block = input_panel_block();
        let result_block = result_panel_block();
        // Both blocks should exist without error
        let _: (Block, Block) = (input_block, result_block);
    }

    // ============================================================
    // Command bar tests
    // ============================================================

    #[test]
    fn test_command_bar_text_includes_quit() {
        let text = build_command_bar_text();
        let text_str = text.to_string();
        assert!(
            text_str.contains('q') && text_str.contains("quit"),
            "Command bar should contain 'q: quit'"
        );
    }

    #[test]
    fn test_command_bar_text_includes_clear() {
        let text = build_command_bar_text();
        let text_str = text.to_string();
        assert!(
            text_str.contains('c') && text_str.contains("clear"),
            "Command bar should contain 'c: clear'"
        );
    }

    #[test]
    fn test_command_bar_text_includes_history_hint() {
        let text = build_command_bar_text();
        let text_str = text.to_string();
        assert!(
            text_str.contains("↑↓") && text_str.contains("history"),
            "Command bar should contain '↑↓: history'"
        );
    }
}
