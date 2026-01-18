//! Panel rendering for the Crabculator TUI.
//!
//! Provides rendering functions for the input and result panels, including:
//! - Error highlighting with red underlines
//! - Error message display below error lines
//! - Result panel with aligned evaluation results

use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::editor::Buffer;
use crate::eval::{EvalError, LineResult, evaluate_all_lines};
use crate::ui::highlight::{highlight_line, highlight_line_with_offset};

/// Rusty-red accent color used for borders, errors, and keyboard shortcuts.
const ACCENT_COLOR: Color = Color::Rgb(139, 69, 19);

/// Formats a `LineResult` for display in the result panel.
///
/// # Returns
/// - `Some(String)` with the formatted result for values and assignments
/// - `None` for empty lines or errors (errors shown in input panel)
#[must_use]
pub fn format_result(result: &LineResult) -> Option<String> {
    match result {
        LineResult::Value(value) => Some(format_value(*value)),
        LineResult::Assignment { name, value } => {
            Some(format!("{name} = {}", format_value(*value)))
        }
        LineResult::Empty | LineResult::Error(_) => None,
    }
}

/// Formats a `f64` value for display.
///
/// Whole numbers are displayed without decimal places.
/// Other floats are displayed with their natural decimal representation.
#[must_use]
fn format_value(value: f64) -> String {
    // If the float is a whole number, display without decimals
    if value.fract() == 0.0 && value.abs() < 1e15 {
        format!("{value:.0}")
    } else {
        value.to_string()
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
                Style::default().fg(ACCENT_COLOR),
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
        .fg(ACCENT_COLOR)
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
                |text| Line::from(Span::styled(text, Style::default().fg(Color::White))),
            )
        })
        .collect()
}

/// Builds visible input lines with scrolling and current line highlighting.
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
                Style::default().fg(ACCENT_COLOR),
            ));
            output.push(error_line);
        }
    }

    output
}

/// Builds visible result lines with scrolling and current line highlighting.
#[must_use]
pub fn build_visible_result_lines_with_highlight(
    results: &[LineResult],
    scroll_offset: usize,
    visible_height: usize,
    current_row: usize,
    panel_width: usize,
    memory_pane_left: bool,
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

            let text = format_result(result).unwrap_or_default();
            let content_width = text.len();

            // Build spans with padding for full-width highlight
            // Right-align content when memory pane is on the left
            let mut spans = if memory_pane_left && content_width < panel_width {
                // Add leading padding for right alignment
                let padding = " ".repeat(panel_width - content_width);
                if text.is_empty() {
                    vec![Span::raw(padding)]
                } else {
                    vec![
                        Span::raw(padding),
                        Span::styled(text, Style::default().fg(Color::White)),
                    ]
                }
            } else if text.is_empty() {
                vec![]
            } else {
                vec![Span::styled(text, Style::default().fg(Color::White))]
            };

            // Add trailing padding for left-aligned content (when pane is on right)
            if !memory_pane_left && is_current_line && content_width < panel_width {
                let padding = " ".repeat(panel_width - content_width);
                spans.push(Span::raw(padding));
            }

            let mut line = Line::from(spans);

            if is_current_line {
                line = line.style(highlight_style);
            }

            line
        })
        .collect()
}

/// Returns the style used for highlighting the current line.
///
/// Uses a dark grey background color that matches the command bar.
#[must_use]
pub fn current_line_highlight_style() -> Style {
    Style::default().bg(Color::DarkGray)
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
                Style::default().fg(ACCENT_COLOR),
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
                |text| Line::from(Span::styled(text, Style::default().fg(Color::White))),
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
/// The minimum width is 3 to align with the title "🦀 crabculator":
/// - 2 columns for the line number (aligning with the emoji width)
/// - 1 column for the space separator (aligning with space after emoji)
///
/// # Arguments
/// * `line_count` - The total number of lines in the buffer
///
/// # Returns
/// The width in characters needed for the gutter (minimum 3, or digits + 1 space).
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
    let width = digits + 1;
    // Minimum width of 3 to align with title emoji (2 columns) + space (1 column)
    if width < 3 { 3 } else { width }
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
/// visible but unobtrusive. Uses `Gray` (lighter than `DarkGray`) to ensure
/// visibility even when the current line highlight (`DarkGray` background)
/// is applied.
#[must_use]
pub fn gutter_style() -> Style {
    Style::default().fg(Color::Gray)
}

/// Builds spans for a line with error highlighting and horizontal offset support.
///
/// If the error has a span, only that portion is underlined.
/// Otherwise, the entire line is underlined.
/// The output is sliced to show only the visible portion based on horizontal offset.
///
/// # Arguments
/// * `line_text` - The full line text
/// * `error` - The evaluation error with optional span information
/// * `horizontal_offset` - The first visible column index (0-based)
/// * `visible_width` - The number of visible columns
fn build_error_spans_with_offset<'a>(
    line_text: &'a str,
    error: &EvalError,
    horizontal_offset: usize,
    visible_width: usize,
) -> Vec<Span<'a>> {
    let error_style = Style::default()
        .fg(ACCENT_COLOR)
        .add_modifier(Modifier::UNDERLINED);

    // Calculate the visible slice of the line
    let start_col = horizontal_offset.min(line_text.len());
    let end_col = (horizontal_offset + visible_width).min(line_text.len());
    let visible_text = &line_text[start_col..end_col];

    error.span().map_or_else(
        // No span available, underline entire visible portion
        || vec![Span::styled(visible_text, error_style)],
        |span| {
            // Adjust span coordinates relative to the visible window
            let span_start = span.start.saturating_sub(horizontal_offset);
            let span_end = span.end.saturating_sub(horizontal_offset);

            // Clamp to visible bounds
            let visible_span_start = span_start.min(visible_text.len());
            let visible_span_end = span_end.min(visible_text.len()).max(visible_span_start);

            let mut spans = Vec::new();

            // Text before error
            if visible_span_start > 0 {
                spans.push(Span::raw(&visible_text[..visible_span_start]));
            }

            // Error portion with underline
            if visible_span_start < visible_span_end {
                spans.push(Span::styled(
                    &visible_text[visible_span_start..visible_span_end],
                    error_style,
                ));
            }

            // Text after error
            if visible_span_end < visible_text.len() {
                spans.push(Span::raw(&visible_text[visible_span_end..]));
            }

            spans
        },
    )
}

/// Builds visible input lines with scrolling, highlighting, and line number gutter.
///
/// This combines scrolling, current line highlighting, and line numbers.
/// Lines are padded to fill the panel width for full-width highlighting.
///
/// # Arguments
/// * `lines` - The buffer lines to render
/// * `results` - The evaluation results corresponding to each line
/// * `scroll_offset` - The first visible line index (0-based)
/// * `visible_height` - The number of visible lines in the viewport
/// * `current_row` - The row index where the cursor is positioned (0-indexed)
/// * `horizontal_scroll_offset` - The first visible column index (0-based)
/// * `visible_width` - The number of visible columns in the viewport (including gutter)
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
    horizontal_scroll_offset: usize,
    visible_width: usize,
) -> (Vec<Line<'a>>, usize) {
    let gutter_width = calculate_gutter_width(lines.len());
    let gutter_style_val = gutter_style();
    let highlight_style = current_line_highlight_style();
    let mut output: Vec<Line<'a>> = Vec::new();

    // Calculate the range of lines to render
    let start = scroll_offset.min(lines.len());
    let end = (scroll_offset + visible_height).min(lines.len());

    // Calculate content width (visible width minus gutter)
    let content_width = visible_width.saturating_sub(gutter_width);

    for (i, line_text) in lines.iter().enumerate().take(end).skip(start) {
        let line_number = i + 1; // 1-based line numbers
        let result = results.get(i);
        let is_current_line = i == current_row;

        // Build the line number span
        let line_num_str = format_line_number(line_number, gutter_width);
        let line_num_span = Span::styled(line_num_str, gutter_style_val);

        // Build the main line with potential error or syntax highlighting
        // Note: We need to highlight the visible portion only
        let content_spans = match result {
            Some(LineResult::Error(err)) => build_error_spans_with_offset(
                line_text,
                err,
                horizontal_scroll_offset,
                content_width,
            ),
            _ => highlight_line_with_offset(line_text, horizontal_scroll_offset, content_width),
        };

        // Combine line number and content
        let mut all_spans = vec![line_num_span];
        all_spans.extend(content_spans);

        // Calculate total width of spans and add padding for full-width highlight
        if is_current_line {
            let spans_width: usize = all_spans.iter().map(|s| s.content.len()).sum();
            if spans_width < visible_width {
                let padding = " ".repeat(visible_width - spans_width);
                all_spans.push(Span::raw(padding));
            }
        }

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
                Style::default().fg(ACCENT_COLOR),
            ));
            output.push(error_line);
        }
    }

    (output, gutter_width)
}

/// Determines if the terminal likely supports emoji rendering.
///
/// Uses the `TERM` environment variable to heuristically detect modern terminals
/// that typically support Unicode emoji. Falls back to `false` for unknown terminals.
///
/// # Returns
/// `true` if the terminal likely supports emoji, `false` otherwise
#[must_use]
fn terminal_supports_emoji() -> bool {
    let term = std::env::var("TERM").unwrap_or_default();
    term_value_supports_emoji(&term)
}

/// Checks if a given TERM value indicates emoji support.
///
/// This is the core logic for terminal emoji detection, extracted to enable
/// testing without environment variable manipulation.
///
/// # Arguments
/// * `term` - The TERM environment variable value to check
///
/// # Returns
/// `true` if the terminal type likely supports emoji, `false` otherwise
#[must_use]
fn term_value_supports_emoji(term: &str) -> bool {
    let term = term.to_lowercase();

    // Modern terminals that typically support emoji
    term.contains("xterm")
        || term.contains("256color")
        || term.contains("kitty")
        || term.contains("alacritty")
        || term.contains("iterm")
        || term.contains("tmux")
        || term.contains("screen")
        || term.contains("vte")
        || term.contains("gnome")
        || term.contains("konsole")
}

/// Returns the branded title for the calculator panel.
///
/// Shows the title with crab emoji when the terminal supports it,
/// otherwise falls back to plain text without the emoji.
///
/// # Returns
/// The title string with or without emoji based on terminal capabilities
#[must_use]
fn calculator_panel_title() -> &'static str {
    if terminal_supports_emoji() {
        "🦀 crabculator"
    } else {
        "crabculator"
    }
}

/// Returns the calculator panel title based on explicit emoji support flag.
///
/// This is the core logic for title generation, extracted to enable testing
/// without environment variable manipulation.
///
/// # Arguments
/// * `supports_emoji` - Whether emoji should be included in the title
///
/// # Returns
/// The title string with or without emoji
#[cfg(test)]
#[must_use]
const fn calculator_panel_title_with_emoji_support(supports_emoji: bool) -> &'static str {
    if supports_emoji {
        "🦀 crabculator"
    } else {
        "crabculator"
    }
}

/// Creates a Block widget for the input panel with title underline.
///
/// # Returns
/// A Block configured with:
/// - Branded title with emoji (or without if emoji unsupported)
/// - Full-width rusty-red underline border below title
#[must_use]
pub fn input_panel_block() -> Block<'static> {
    Block::default()
        .title(calculator_panel_title())
        .borders(Borders::TOP)
        .border_style(Style::default().fg(ACCENT_COLOR))
}

/// Creates a Block widget for the memory panel with dark-grey background and conditional red border.
///
/// # Arguments
/// * `memory_pane_left` - When true, red border on right side and title right-aligned; when false, red border on left side and title left-aligned
///
/// # Returns
/// A Block configured with:
/// - Title "Memory" (right-aligned when pane is on left, left-aligned when on right)
/// - Dark-grey background
/// - Full-width rusty-red underline border below title
/// - Red border on the side adjacent to the input panel
#[must_use]
pub fn memory_panel_block(memory_pane_left: bool) -> Block<'static> {
    let (side_border, title_alignment) = if memory_pane_left {
        (Borders::RIGHT, Alignment::Right)
    } else {
        (Borders::LEFT, Alignment::Left)
    };
    Block::default()
        .title("Memory")
        .title_alignment(title_alignment)
        .borders(Borders::TOP | side_border)
        .border_style(Style::default().fg(ACCENT_COLOR))
        .style(Style::default().bg(Color::Rgb(30, 30, 30)))
}

/// Renders the input panel with buffer content, error highlighting, current line highlighting,
/// line number gutter, and scrolling support.
///
/// # Arguments
/// * `frame` - The ratatui Frame to render to
/// * `area` - The area to render the panel in
/// * `buffer` - The text buffer containing input lines
/// * `scroll_offset` - The first visible line index (0-based)
/// * `horizontal_scroll_offset` - The first visible column index (0-based)
pub fn render_input_panel(
    frame: &mut Frame,
    area: Rect,
    buffer: &Buffer,
    scroll_offset: usize,
    horizontal_scroll_offset: usize,
) {
    // Evaluate all lines to get results
    let results = evaluate_all_lines(buffer.lines().iter().map(String::as_str));

    // Get cursor row for highlighting
    let cursor_row = buffer.cursor().row();

    // Calculate visible height (area height minus title row, no borders)
    let visible_height = area.height.saturating_sub(1) as usize;

    // Calculate visible width (no borders)
    let visible_width = area.width as usize;

    // Build styled lines for visible portion with line number gutter
    let (styled_lines, gutter_width) = build_visible_input_lines_with_gutter(
        buffer.lines(),
        &results,
        scroll_offset,
        visible_height,
        cursor_row,
        horizontal_scroll_offset,
        visible_width,
    );

    // Create the paragraph widget with title but no borders
    let paragraph = Paragraph::new(Text::from(styled_lines)).block(input_panel_block());

    frame.render_widget(paragraph, area);

    // Set cursor position (accounting for title row and gutter, no borders)
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

    // Position cursor: area.x + gutter + cursor_col (no border offset)
    // For y: area.y + 1 (title row) + actual_row
    let adjusted_cursor_col = cursor_col.saturating_sub(horizontal_scroll_offset);
    let cursor_x = area.x
        + u16::try_from(gutter_width).unwrap_or(0)
        + u16::try_from(adjusted_cursor_col).unwrap_or(0);
    let cursor_y = area.y + 1 + u16::try_from(actual_row).unwrap_or(0);

    // Only set cursor if it's within the visible area
    if cursor_x < area.x + area.width && cursor_y < area.y + area.height {
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
/// * `memory_pane_left` - Whether the memory pane is on the left side
pub fn render_result_panel(
    frame: &mut Frame,
    area: Rect,
    results: &[LineResult],
    current_row: usize,
    scroll_offset: usize,
    memory_pane_left: bool,
) {
    // Calculate visible height (area height minus borders)
    let visible_height = area.height.saturating_sub(2) as usize;

    // Calculate panel width (area width minus borders)
    let panel_width = area.width.saturating_sub(2) as usize;

    let styled_lines = build_visible_result_lines_with_highlight(
        results,
        scroll_offset,
        visible_height,
        current_row,
        panel_width,
        memory_pane_left,
    );

    // Create the paragraph widget with memory panel styling
    let paragraph =
        Paragraph::new(Text::from(styled_lines)).block(memory_panel_block(memory_pane_left));

    frame.render_widget(paragraph, area);
}

/// Builds the styled text line for the command bar.
///
/// Returns a Line containing all keyboard shortcuts with consistent styling.
/// Keys are highlighted in rusty-red bold, descriptions are plain text.
#[must_use]
pub fn build_command_bar_text<'a>() -> Line<'a> {
    let key_style = Style::default()
        .fg(ACCENT_COLOR)
        .add_modifier(Modifier::BOLD);

    Line::from(vec![
        Span::styled("CTRL+Q", key_style),
        Span::raw(": quit  "),
        Span::styled("CTRL+R", key_style),
        Span::raw(": clear  "),
        Span::styled("CTRL+H", key_style),
        Span::raw(": help  "),
        Span::styled("CTRL+←/→", key_style),
        Span::raw(": move memory  "),
        Span::styled("↑↓", key_style),
        Span::raw(": history"),
    ])
}

/// Renders the command bar at the bottom of the screen.
///
/// Displays available keyboard commands: "CTRL+Q: quit  CTRL+R: clear  CTRL+H: help  ↑↓: history"
///
/// # Arguments
/// * `frame` - The ratatui Frame to render to
/// * `area` - The area to render the command bar in (should be 1 row)
pub fn render_command_bar(frame: &mut Frame, area: Rect) {
    let command_text = build_command_bar_text();
    let command_bar = Paragraph::new(command_text).style(Style::default().bg(Color::DarkGray));

    frame.render_widget(command_bar, area);
}

// ============================================================
// Help Overlay Content and Rendering
// ============================================================

/// Help content lines for the General Usage section.
const HELP_GENERAL_USAGE: &[&str] = &[
    "=== General Usage ===",
    "",
    "Crabculator is a multi-line calculator with variable support.",
    "",
    "Basic Operations:",
    "  + - * /    Arithmetic operators",
    "  %          Modulo",
    "  ^          Exponentiation",
    "  ( )        Grouping",
    "",
    "Variables:",
    "  x = 5      Assign value to variable",
    "  x + 10     Use variable in expression",
    "",
    "Keyboard Shortcuts:",
    "  CTRL+Q     Quit",
    "  CTRL+R     Clear all",
    "  CTRL+H     Toggle help",
    "  ESC        Close help / Quit",
    "  Arrow keys Navigate / Scroll help",
    "",
];

/// Help content lines for the Function Reference section.
const HELP_FUNCTION_REFERENCE: &[&str] = &[
    "=== Function Reference ===",
    "",
    "Basic Math:",
    "  sqrt(x)    Square root",
    "  abs(x)     Absolute value",
    "  min(a,b)   Minimum value",
    "  max(a,b)   Maximum value",
    "",
    "Trigonometric:",
    "  sin(x)     Sine (radians)",
    "  cos(x)     Cosine (radians)",
    "  tan(x)     Tangent (radians)",
    "  asin(x)    Arc sine",
    "  acos(x)    Arc cosine",
    "  atan(x)    Arc tangent",
    "",
    "Hyperbolic:",
    "  sinh(x)    Hyperbolic sine",
    "  cosh(x)    Hyperbolic cosine",
    "  tanh(x)    Hyperbolic tangent",
    "",
    "Logarithmic & Exponential:",
    "  ln(x)      Natural logarithm",
    "  log(x)     Base-10 logarithm",
    "  log2(x)    Base-2 logarithm",
    "  exp(x)     e^x",
    "",
    "Rounding:",
    "  floor(x)   Round down",
    "  ceil(x)    Round up",
    "  round(x)   Round to nearest",
    "  trunc(x)   Truncate to integer",
    "",
    "Constants:",
    "  pi         3.14159...",
    "  e          2.71828...",
    "",
];

/// Returns all help content lines combined.
#[must_use]
pub fn help_content_lines() -> Vec<&'static str> {
    let mut lines = Vec::with_capacity(HELP_GENERAL_USAGE.len() + HELP_FUNCTION_REFERENCE.len());
    lines.extend_from_slice(HELP_GENERAL_USAGE);
    lines.extend_from_slice(HELP_FUNCTION_REFERENCE);
    lines
}

pub const HELP_CONTENT_HEIGHT: usize = 58;

/// Calculates the centered area for an overlay of the given dimensions.
///
/// # Arguments
/// * `area` - The parent area to center within
/// * `width_percent` - The width of the overlay as a percentage of parent (0-100)
/// * `height_percent` - The height of the overlay as a percentage of parent (0-100)
///
/// # Returns
/// A `Rect` centered within the parent area.
#[must_use]
pub const fn centered_rect(area: Rect, width_percent: u16, height_percent: u16) -> Rect {
    let width = area.width * width_percent / 100;
    let height = area.height * height_percent / 100;
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width, height)
}

/// Builds styled lines for the help overlay content.
///
/// # Arguments
/// * `scroll_offset` - The first visible line index (0-based)
/// * `visible_height` - The number of visible lines
///
/// # Returns
/// A vector of styled `Line` objects for rendering.
#[must_use]
pub fn build_help_content_lines(scroll_offset: usize, visible_height: usize) -> Vec<Line<'static>> {
    let all_lines = help_content_lines();
    let header_style = Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD);

    let start = scroll_offset.min(all_lines.len());
    let end = (scroll_offset + visible_height).min(all_lines.len());

    all_lines[start..end]
        .iter()
        .map(|&line| {
            if line.starts_with("===") {
                Line::from(Span::styled(line, header_style))
            } else {
                Line::from(line)
            }
        })
        .collect()
}

/// Renders the help overlay panel.
///
/// Displays a centered, bordered panel with title "Help" containing
/// the help content. Supports vertical scrolling.
///
/// # Arguments
/// * `frame` - The ratatui Frame to render to
/// * `area` - The full screen area
/// * `scroll_offset` - The scroll position for the content
pub fn render_help_overlay(frame: &mut Frame, area: Rect, scroll_offset: usize) {
    use ratatui::widgets::Clear;

    // Create centered overlay (70% width, 80% height)
    let overlay_area = centered_rect(area, 70, 80);

    // Clear the background
    frame.render_widget(Clear, overlay_area);

    // Calculate visible content height (minus borders)
    let visible_height = overlay_area.height.saturating_sub(2) as usize;

    // Build the help content with scrolling
    let content_lines = build_help_content_lines(scroll_offset, visible_height);
    let content = Text::from(content_lines);

    // Build scroll indicator
    let total_lines = HELP_CONTENT_HEIGHT;
    let scroll_info = if total_lines > visible_height {
        let max_scroll = total_lines.saturating_sub(visible_height);
        let percent = if max_scroll > 0 {
            (scroll_offset * 100) / max_scroll
        } else {
            0
        };
        format!(" [{percent}%] ")
    } else {
        String::new()
    };

    // Create paragraph with scroll indicator in title
    let title = if scroll_info.is_empty() {
        " Help ".to_string()
    } else {
        format!(" Help {scroll_info}")
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(ACCENT_COLOR));

    let paragraph = Paragraph::new(content)
        .block(block)
        .style(Style::default().bg(Color::Black));

    frame.render_widget(paragraph, overlay_area);
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
        let result = LineResult::Value(42.0);
        assert_eq!(format_result(&result), Some("42".to_string()));
    }

    #[test]
    fn test_format_result_negative_integer() {
        let result = LineResult::Value(-123.0);
        assert_eq!(format_result(&result), Some("-123".to_string()));
    }

    #[test]
    fn test_format_result_float_value() {
        let result = LineResult::Value(2.75);
        assert_eq!(format_result(&result), Some("2.75".to_string()));
    }

    #[test]
    fn test_format_result_whole_float_displays_without_decimal() {
        let result = LineResult::Value(5.0);
        assert_eq!(format_result(&result), Some("5".to_string()));
    }

    #[test]
    fn test_format_result_assignment() {
        let result = LineResult::Assignment {
            name: "x".to_string(),
            value: 10.0,
        };
        assert_eq!(format_result(&result), Some("x = 10".to_string()));
    }

    #[test]
    fn test_format_result_assignment_with_float() {
        let result = LineResult::Assignment {
            name: "rate".to_string(),
            value: 1.23456,
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
        let value = 1_000_000.0;
        assert_eq!(format_value(value), "1000000");
    }

    #[test]
    fn test_format_value_zero() {
        let value = 0.0;
        assert_eq!(format_value(value), "0");
    }

    #[test]
    fn test_format_value_small_float() {
        let value = 0.001;
        assert_eq!(format_value(value), "0.001");
    }

    // ============================================================
    // build_input_lines tests
    // ============================================================

    #[test]
    fn test_build_input_lines_single_line_no_error() {
        let lines = vec!["5 + 3".to_string()];
        let results = vec![LineResult::Value(8.0)];

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
            LineResult::Value(8.0),
            LineResult::Error(EvalError::new("undefined variable")),
            LineResult::Value(8.0),
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

    #[test]
    fn test_build_error_line_uses_rusty_red_color() {
        let line = "invalid expression";
        let error = EvalError::new("syntax error");

        let styled_line = build_error_line(line, &error);

        // Error style should use rusty-red color (RGB 139, 69, 19)
        assert_eq!(styled_line.spans.len(), 1);
        let span_style = styled_line.spans[0].style;
        assert_eq!(
            span_style.fg,
            Some(ACCENT_COLOR),
            "Error text should use rusty-red accent color"
        );
    }

    // ============================================================
    // build_result_lines tests
    // ============================================================

    #[test]
    fn test_build_result_lines_values() {
        let results = vec![LineResult::Value(8.0), LineResult::Value(2.75)];

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
            LineResult::Value(8.0),
            LineResult::Empty,
            LineResult::Error(EvalError::new("error")),
            LineResult::Assignment {
                name: "x".to_string(),
                value: 5.0,
            },
        ];

        let output = build_result_lines(&results);

        assert_eq!(output.len(), 4);
    }

    #[test]
    fn test_build_result_lines_assignment_format() {
        let results = vec![LineResult::Assignment {
            name: "result".to_string(),
            value: 42.0,
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
        let results = vec![LineResult::Value(8.0), LineResult::Value(20.0)];
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
        let results = vec![LineResult::Value(8.0), LineResult::Value(20.0)];
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
        let results = vec![LineResult::Value(8.0), LineResult::Value(20.0)];
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
        let results = vec![LineResult::Value(8.0), LineResult::Value(20.0)];
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
    fn test_current_line_highlight_style_uses_dark_gray() {
        // Verify the highlight color matches the command bar (DarkGray)
        let style = current_line_highlight_style();
        assert!(style.bg.is_some());
        assert_eq!(style.bg, Some(Color::DarkGray));
    }

    // ============================================================
    // Line Number Gutter tests
    // ============================================================

    #[test]
    fn test_calculate_gutter_width_single_digit_lines() {
        // 1-9 lines need minimum width of 3 for title alignment
        // (2 columns for line number aligning with emoji + 1 space)
        assert_eq!(calculate_gutter_width(1), 3);
        assert_eq!(calculate_gutter_width(9), 3);
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
        // Even 0 lines should have minimum width of 3 for title alignment
        // (2 columns for emoji width + 1 space)
        assert_eq!(calculate_gutter_width(0), 3);
    }

    #[test]
    fn test_gutter_width_aligns_with_title() {
        // The title is "🦀 crabculator" where the emoji takes 2 columns
        // and there's a space before "crabculator".
        // Line numbers should align under the emoji (2 columns),
        // with a space separator, so minimum gutter width is 3.
        // This ensures content starts at the same column as "crabculator".
        assert_eq!(
            calculate_gutter_width(1),
            3,
            "Single line buffer should have gutter width 3 for title alignment"
        );
        assert_eq!(
            calculate_gutter_width(5),
            3,
            "Small buffer should have gutter width 3 for title alignment"
        );
        assert_eq!(
            calculate_gutter_width(9),
            3,
            "9 lines should have gutter width 3 for title alignment"
        );
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
        // Gutter should have a dimmed foreground color (Gray - lighter than DarkGray
        // to remain visible when current line highlight is applied)
        assert_eq!(
            style.fg,
            Some(Color::Gray),
            "Gutter should use Gray foreground for visibility"
        );
    }

    #[test]
    fn test_build_visible_input_lines_with_gutter_returns_correct_width() {
        let lines: Vec<String> = (0..50).map(|i| format!("line {i}")).collect();
        let results: Vec<LineResult> = (0..50).map(|_| LineResult::Empty).collect();

        let (output, gutter_width) =
            build_visible_input_lines_with_gutter(&lines, &results, 0, 10, 0, 0, 80);

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

        let (output, _) = build_visible_input_lines_with_gutter(&lines, &results, 10, 5, 12, 0, 80);

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

        let (output, _) = build_visible_input_lines_with_gutter(&lines, &results, 0, 10, 1, 0, 80);

        // First line should not be highlighted
        assert!(output[0].style.bg.is_none());
        // Second line (current_row = 1) should be highlighted
        assert!(output[1].style.bg.is_some());
    }

    #[test]
    fn test_line_number_visible_on_highlighted_line() {
        // Line numbers SHALL be visible even when the current line is highlighted.
        // This means the line number foreground color must contrast with the
        // highlight background color.
        let lines = vec!["test line".to_string()];
        let results = vec![LineResult::Empty];

        let (output, _) = build_visible_input_lines_with_gutter(&lines, &results, 0, 10, 0, 0, 80);

        // The line is highlighted (current_row = 0)
        let line = &output[0];
        let highlight_bg = line.style.bg;
        assert!(
            highlight_bg.is_some(),
            "Current line should have background highlight"
        );

        // The first span is the line number - verify its foreground differs from background
        let line_num_span = &line.spans[0];
        let gutter_fg = line_num_span.style.fg;

        // The gutter foreground must NOT equal the highlight background
        assert_ne!(
            gutter_fg, highlight_bg,
            "Line number foreground ({gutter_fg:?}) must differ from highlight background ({highlight_bg:?}) for visibility"
        );
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
    fn test_memory_panel_block_returns_valid_block() {
        // Verify memory_panel_block returns a Block.
        // Since Block's internal state is not accessible,
        // we verify it compiles and can be rendered.
        let block_left = memory_panel_block(true);
        let block_right = memory_panel_block(false);
        // Type assertions: these compile only if memory_panel_block returns Block
        let _: Block = block_left;
        let _: Block = block_right;
    }

    #[test]
    fn test_both_panel_blocks_exist() {
        // Both panels should exist and return valid Blocks.
        let input_block = input_panel_block();
        let memory_block = memory_panel_block(true);
        // Both blocks should exist without error
        let _: (Block, Block) = (input_block, memory_block);
    }

    #[test]
    fn test_input_panel_block_has_top_border_for_title_underline() {
        use ratatui::Terminal;
        use ratatui::backend::TestBackend;
        use std::fmt::Write;

        let backend = TestBackend::new(20, 5);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|frame| {
                let area = frame.area();
                let block = input_panel_block();
                frame.render_widget(block, area);
            })
            .unwrap();

        let buffer = terminal.backend().buffer();
        // Debug: print all rows
        let mut all_rows = String::new();
        for y in 0..5 {
            let row: String = (0..20)
                .map(|x| buffer[(x, y)].symbol().chars().next().unwrap_or(' '))
                .collect();
            writeln!(all_rows, "Row {y}: '{row}'").unwrap();
        }

        // The first row (index 0) should contain the title and top border
        // Check for horizontal line characters (box drawing)
        let first_row: String = (0..20)
            .map(|x| buffer[(x, 0)].symbol().chars().next().unwrap_or(' '))
            .collect();
        assert!(
            first_row.contains('─') || first_row.contains('━'),
            "Calculator panel should have top border line. Buffer contents:\n{all_rows}"
        );
    }

    #[test]
    fn test_memory_panel_block_has_top_border_for_title_underline() {
        use ratatui::Terminal;
        use ratatui::backend::TestBackend;

        let backend = TestBackend::new(20, 5);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|frame| {
                let area = frame.area();
                let block = memory_panel_block(true);
                frame.render_widget(block, area);
            })
            .unwrap();

        let buffer = terminal.backend().buffer();
        // The first row (index 0) should contain the title and top border
        let first_row: String = (0..20)
            .map(|x| buffer[(x, 0)].symbol().chars().next().unwrap_or(' '))
            .collect();
        assert!(
            first_row.contains('─') || first_row.contains('━'),
            "Memory panel should have top border (underline below title). Got: '{first_row}'"
        );
    }

    #[test]
    fn test_memory_panel_title_right_aligned_when_pane_left() {
        use ratatui::Terminal;
        use ratatui::backend::TestBackend;

        let backend = TestBackend::new(20, 5);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|frame| {
                let area = frame.area();
                let block = memory_panel_block(true); // pane on left
                frame.render_widget(block, area);
            })
            .unwrap();

        let buffer = terminal.backend().buffer();
        // The title "Memory" should be right-aligned when pane is on left
        let first_row: String = (0..20)
            .map(|x| buffer[(x, 0)].symbol().chars().next().unwrap_or(' '))
            .collect();
        // Check that "Memory" appears near the right side, not at the start
        let memory_pos = first_row.find("Memory");
        assert!(
            memory_pos.is_some() && memory_pos.unwrap() > 5,
            "Memory title should be right-aligned when pane is left. Got: '{first_row}'"
        );
    }

    #[test]
    fn test_memory_panel_title_left_aligned_when_pane_right() {
        use ratatui::Terminal;
        use ratatui::backend::TestBackend;

        let backend = TestBackend::new(20, 5);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|frame| {
                let area = frame.area();
                let block = memory_panel_block(false); // pane on right
                frame.render_widget(block, area);
            })
            .unwrap();

        let buffer = terminal.backend().buffer();
        // The title "Memory" should be left-aligned when pane is on right
        let first_row: String = (0..20)
            .map(|x| buffer[(x, 0)].symbol().chars().next().unwrap_or(' '))
            .collect();
        // When pane is on right, left border exists, so "Memory" should start at position 1
        // Check that "Memory" appears near the left side
        let memory_pos = first_row.find("Memory");
        assert!(
            memory_pos.is_some() && memory_pos.unwrap() <= 3,
            "Memory title should be left-aligned when pane is right. Got: '{first_row}', position: {memory_pos:?}"
        );
    }

    // ============================================================
    // Command bar tests
    // ============================================================

    #[test]
    fn test_command_bar_text_includes_quit() {
        let text = build_command_bar_text();
        let text_str = text.to_string();
        assert!(
            text_str.contains("CTRL+Q") && text_str.contains("quit"),
            "Command bar should contain 'CTRL+Q: quit'"
        );
    }

    #[test]
    fn test_command_bar_text_includes_clear() {
        let text = build_command_bar_text();
        let text_str = text.to_string();
        assert!(
            text_str.contains("CTRL+R") && text_str.contains("clear"),
            "Command bar should contain 'CTRL+R: clear'"
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

    #[test]
    fn test_command_bar_text_includes_move_memory() {
        let text = build_command_bar_text();
        let text_str = text.to_string();
        assert!(
            text_str.contains("CTRL+←/→") && text_str.contains("move memory"),
            "Command bar should contain 'CTRL+←/→: move memory'"
        );
    }

    #[test]
    fn test_command_bar_text_uses_rusty_red_for_shortcuts() {
        let text = build_command_bar_text();
        // Keyboard shortcuts should use rusty-red color (RGB 139, 69, 19)
        let first_span = &text.spans[0];
        assert_eq!(
            first_span.style.fg,
            Some(ACCENT_COLOR),
            "Keyboard shortcuts should use rusty-red accent color"
        );
    }

    // ==========================================================================
    // Panel Title Tests
    // ==========================================================================

    #[test]
    fn test_terminal_supports_emoji_with_xterm() {
        assert!(
            term_value_supports_emoji("xterm-256color"),
            "xterm-256color should support emoji"
        );
    }

    #[test]
    fn test_terminal_supports_emoji_with_kitty() {
        assert!(
            term_value_supports_emoji("xterm-kitty"),
            "kitty should support emoji"
        );
    }

    #[test]
    fn test_terminal_supports_emoji_with_alacritty() {
        assert!(
            term_value_supports_emoji("alacritty"),
            "alacritty should support emoji"
        );
    }

    #[test]
    fn test_terminal_supports_emoji_with_tmux() {
        assert!(
            term_value_supports_emoji("tmux-256color"),
            "tmux should support emoji"
        );
    }

    #[test]
    fn test_terminal_supports_emoji_with_unknown_term() {
        assert!(
            !term_value_supports_emoji("dumb"),
            "dumb terminal should not support emoji"
        );
    }

    #[test]
    fn test_terminal_supports_emoji_with_empty_term() {
        assert!(
            !term_value_supports_emoji(""),
            "empty TERM should not support emoji"
        );
    }

    #[test]
    fn test_terminal_supports_emoji_case_insensitive() {
        // Test uppercase
        assert!(
            term_value_supports_emoji("XTERM-256COLOR"),
            "TERM check should be case-insensitive"
        );
    }

    #[test]
    fn test_calculator_panel_title_with_emoji_support() {
        let title = calculator_panel_title_with_emoji_support(true);
        assert_eq!(
            title, "🦀 crabculator",
            "Title should include crab emoji with space when terminal supports it"
        );
    }

    #[test]
    fn test_calculator_panel_title_without_emoji_support() {
        let title = calculator_panel_title_with_emoji_support(false);
        assert_eq!(
            title, "crabculator",
            "Title should be lowercase without emoji when terminal doesn't support it"
        );
    }

    #[test]
    fn test_calculator_panel_title_has_space_after_emoji() {
        let title = calculator_panel_title_with_emoji_support(true);
        // Verify emoji is followed by a space
        assert!(
            title.starts_with("🦀 c"),
            "Emoji should be followed by a space and lowercase 'c'"
        );
        assert!(
            title.contains("🦀 "),
            "There should be a space after the emoji"
        );
    }

    // ==========================================================================
    // Full-Width Highlighting Tests
    // ==========================================================================

    #[test]
    fn test_build_visible_result_lines_with_highlight_pads_to_panel_width() {
        let results = vec![
            LineResult::Value(42.0), // "42" is 2 chars
            LineResult::Value(100.0),
        ];
        let current_row = 0;
        let panel_width = 20;
        let memory_pane_left = false;

        let output = build_visible_result_lines_with_highlight(
            &results,
            0,
            10,
            current_row,
            panel_width,
            memory_pane_left,
        );

        // The first line (current row) should have spans totaling panel_width
        // "42" (2 chars) + padding (18 chars) = 20 chars
        let first_line = &output[0];
        let total_content_len: usize = first_line.spans.iter().map(|span| span.content.len()).sum();
        assert_eq!(
            total_content_len, panel_width,
            "Highlighted line should be padded to panel width"
        );
    }

    #[test]
    fn test_build_visible_result_lines_with_highlight_non_current_not_padded() {
        let results = vec![LineResult::Value(42.0), LineResult::Value(100.0)];
        let current_row = 0;
        let panel_width = 20;
        let memory_pane_left = false;

        let output = build_visible_result_lines_with_highlight(
            &results,
            0,
            10,
            current_row,
            panel_width,
            memory_pane_left,
        );

        // The second line (not current row) should NOT be padded
        let second_line = &output[1];
        let total_content_len: usize = second_line
            .spans
            .iter()
            .map(|span| span.content.len())
            .sum();
        // "100" is 3 chars, should not be padded to 20
        assert!(
            total_content_len < panel_width,
            "Non-highlighted line should not be padded"
        );
    }

    #[test]
    fn test_build_visible_result_lines_with_highlight_empty_line_padded() {
        let results = vec![LineResult::Empty];
        let current_row = 0;
        let panel_width = 15;
        let memory_pane_left = false;

        let output = build_visible_result_lines_with_highlight(
            &results,
            0,
            10,
            current_row,
            panel_width,
            memory_pane_left,
        );

        // Empty line when current should be padded to full width
        let first_line = &output[0];
        // Empty lines should still have the highlight style applied
        assert!(
            first_line.style.bg.is_some(),
            "Empty current line should have highlight"
        );
    }

    #[test]
    fn test_build_visible_input_lines_with_highlight_applies_style_to_full_line() {
        let lines = vec!["5 + 3".to_string(), "10 * 2".to_string()];
        let results = vec![LineResult::Value(8.0), LineResult::Value(20.0)];
        let current_row = 0;

        let output = build_visible_input_lines_with_highlight(&lines, &results, 0, 10, current_row);

        // Verify the line style has background color applied
        // This means the highlight style covers the entire line, not just content
        assert!(
            output[0].style.bg.is_some(),
            "Current line should have background highlight style"
        );
        // The style should be the current line highlight style (subtle gray)
        let expected_style = current_line_highlight_style();
        assert_eq!(
            output[0].style.bg, expected_style.bg,
            "Line should use the current line highlight background"
        );
    }

    #[test]
    fn test_build_visible_input_lines_with_highlight_style_spans_full_width() {
        // When Line has a style, ratatui applies it to the full line width
        // We verify the style is set on the Line itself, not individual spans
        let lines = vec!["x".to_string()];
        let results = vec![LineResult::Value(1.0)];
        let current_row = 0;

        let output = build_visible_input_lines_with_highlight(&lines, &results, 0, 10, current_row);

        // The Line's style (not span's style) determines full-width highlight
        assert!(
            output[0].style.bg.is_some(),
            "Line style should have background for full-width highlight"
        );
    }

    #[test]
    fn test_build_visible_result_lines_with_highlight_with_large_panel_width() {
        let results = vec![LineResult::Value(1.0)];
        let current_row = 0;
        let panel_width = 100;
        let memory_pane_left = false;

        let output = build_visible_result_lines_with_highlight(
            &results,
            0,
            10,
            current_row,
            panel_width,
            memory_pane_left,
        );

        let first_line = &output[0];
        let total_content_len: usize = first_line.spans.iter().map(|span| span.content.len()).sum();
        assert_eq!(
            total_content_len, panel_width,
            "Line should be padded to full large panel width"
        );
    }

    #[test]
    fn test_build_visible_result_lines_with_highlight_content_equals_panel_width() {
        // Edge case: content is exactly panel width, no padding needed
        let results = vec![LineResult::Value(12345.0)];
        let current_row = 0;
        let panel_width = 5; // "12345" is exactly 5 chars
        let memory_pane_left = false;

        let output = build_visible_result_lines_with_highlight(
            &results,
            0,
            10,
            current_row,
            panel_width,
            memory_pane_left,
        );

        let first_line = &output[0];
        let total_content_len: usize = first_line.spans.iter().map(|span| span.content.len()).sum();
        assert_eq!(
            total_content_len, panel_width,
            "When content equals panel width, total should still equal panel width"
        );
    }

    #[test]
    fn test_build_visible_result_lines_right_aligned_when_pane_left() {
        let results = vec![LineResult::Value(42.0)]; // "42" is 2 chars
        let current_row = 0;
        let panel_width = 10;
        let memory_pane_left = true;

        let output = build_visible_result_lines_with_highlight(
            &results,
            0,
            10,
            current_row,
            panel_width,
            memory_pane_left,
        );

        // Content should be right-aligned: 8 spaces + "42"
        let first_line = &output[0];
        assert!(
            first_line.spans.len() >= 2,
            "Should have padding and content spans"
        );
        // First span should be padding (8 spaces)
        assert_eq!(
            first_line.spans[0].content.as_ref(),
            "        ",
            "First span should be 8 spaces of padding"
        );
    }

    // ============================================================
    // Help Overlay tests
    // ============================================================

    #[test]
    fn test_help_content_lines_returns_all_content() {
        let lines = help_content_lines();
        assert!(
            !lines.is_empty(),
            "Help content should have at least one line"
        );
        // Should have both sections
        let content = lines.join("\n");
        assert!(
            content.contains("General Usage"),
            "Should contain General Usage section"
        );
        assert!(
            content.contains("Function Reference"),
            "Should contain Function Reference section"
        );
    }

    #[test]
    fn test_help_content_height_matches_actual_content() {
        let lines = help_content_lines();
        assert_eq!(
            lines.len(),
            HELP_CONTENT_HEIGHT,
            "HELP_CONTENT_HEIGHT constant should match actual content length"
        );
    }

    #[test]
    fn test_centered_rect_centers_horizontally() {
        let parent = Rect::new(0, 0, 100, 50);
        let centered = centered_rect(parent, 50, 50);

        // 50% of 100 = 50 width, should start at x=25 to center
        assert_eq!(centered.width, 50);
        assert_eq!(centered.x, 25);
    }

    #[test]
    fn test_centered_rect_centers_vertically() {
        let parent = Rect::new(0, 0, 100, 50);
        let centered = centered_rect(parent, 50, 50);

        // 50% of 50 = 25 height, should start at y=12 to center (rounding down)
        assert_eq!(centered.height, 25);
        assert_eq!(centered.y, 12);
    }

    #[test]
    fn test_centered_rect_respects_parent_offset() {
        let parent = Rect::new(10, 20, 100, 50);
        let centered = centered_rect(parent, 50, 50);

        // Should be centered within parent, accounting for parent's offset
        assert_eq!(centered.x, 10 + 25); // parent.x + (100 - 50) / 2
        assert_eq!(centered.y, 20 + 12); // parent.y + (50 - 25) / 2
    }

    #[test]
    fn test_build_help_content_lines_returns_visible_slice() {
        let lines = build_help_content_lines(0, 10);
        assert_eq!(
            lines.len(),
            10,
            "Should return exactly visible_height lines"
        );
    }

    #[test]
    fn test_build_help_content_lines_respects_scroll_offset() {
        let all_lines = help_content_lines();
        let scroll_offset = 5;
        let visible_height = 10;

        let lines = build_help_content_lines(scroll_offset, visible_height);

        // First visible line should be the 6th line (index 5) from content
        let first_line_text = lines[0].to_string();
        assert_eq!(
            first_line_text, all_lines[scroll_offset],
            "First visible line should match expected offset"
        );
    }

    #[test]
    fn test_build_help_content_lines_handles_scroll_near_end() {
        let total = help_content_lines().len();
        let scroll_offset = total.saturating_sub(5);
        let visible_height = 20;

        let lines = build_help_content_lines(scroll_offset, visible_height);

        // Should only return remaining lines, not 20
        let expected = total.saturating_sub(scroll_offset);
        assert_eq!(
            lines.len(),
            expected,
            "Should return only remaining lines at end"
        );
    }

    #[test]
    fn test_build_help_content_lines_styles_headers() {
        let lines = build_help_content_lines(0, 5);

        // First line should be "=== General Usage ===" which is styled as header
        let first_line = &lines[0];
        // Headers should have white bold style
        if !first_line.spans.is_empty() {
            let style = first_line.spans[0].style;
            assert_eq!(
                style.fg,
                Some(Color::White),
                "Header should be styled white"
            );
        }
    }

    #[test]
    fn test_command_bar_includes_help_shortcut() {
        let text = build_command_bar_text();
        let text_str = text.to_string();
        assert!(
            text_str.contains("CTRL+H") && text_str.contains("help"),
            "Command bar should contain 'CTRL+H: help'"
        );
    }
}
