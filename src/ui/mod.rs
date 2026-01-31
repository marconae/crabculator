//! UI module for the Crabculator TUI.
//!
//! Contains layout management and panel rendering functionality.

mod highlight;
mod layout;
mod render;
mod theme;

pub use theme::AppTheme;

pub use highlight::{Token, TokenType, highlight_line, token_style, tokenize};

pub use layout::{LayoutAreas, create_main_layout, create_panel_layout};
pub use render::{
    HELP_CONTENT_HEIGHT, build_help_content_lines, build_input_lines, build_result_lines,
    build_visible_input_lines, build_visible_result_lines, centered_rect, format_result,
    help_content_lines, render_command_bar, render_help_overlay, render_input_panel,
    render_result_panel,
};

use crate::app::App;
use crate::eval::evaluate_all_lines_with_context;
use ratatui::Frame;

/// Renders the main UI layout with input, results panels, and command bar.
///
/// This function evaluates all lines using the app's context, which stores
/// variables for persistence. Variables defined during evaluation are
/// stored in `app.context`.
///
/// # Arguments
///
/// * `frame` - The ratatui Frame to render to
/// * `app` - Mutable reference to the application state
pub fn render(frame: &mut Frame, app: &mut App) {
    let areas = create_main_layout(frame.area());

    let panels = create_panel_layout(app.memory_pane_left).split(areas.content_area);

    let (input_panel_idx, memory_panel_idx) = if app.memory_pane_left {
        (1, 0) // Input on right, memory on left
    } else {
        (0, 1) // Input on left, memory on right
    };

    let visible_height = panels[input_panel_idx].height.saturating_sub(2) as usize;
    let visible_width = panels[input_panel_idx].width.saturating_sub(2) as usize;

    app.adjust_scroll(visible_height);
    app.adjust_horizontal_scroll(visible_width);

    let results = evaluate_all_lines_with_context(
        app.buffer.lines().iter().map(String::as_str),
        &mut app.context,
    );

    render_input_panel(
        frame,
        panels[input_panel_idx],
        &app.buffer,
        app.scroll_offset,
        app.horizontal_scroll_offset,
        app.last_edit_time,
    );

    render_result_panel(
        frame,
        panels[memory_panel_idx],
        &results,
        app.scroll_offset,
        app.memory_pane_left,
    );

    render_command_bar(frame, areas.command_bar);

    if app.help_visible {
        render_help_overlay(frame, frame.area(), app.help_scroll_offset);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::layout::Rect;

    #[test]
    fn render_layout_creates_correct_structure() {
        let area = Rect::new(0, 0, 100, 50);
        let areas = create_main_layout(area);

        assert_eq!(areas.content_area.height, 48);
        assert_eq!(areas.command_bar.height, 2);
    }

    #[test]
    fn panel_layout_creates_two_chunks() {
        let layout = create_panel_layout(false);
        let area = Rect::new(0, 0, 100, 49);
        let chunks = layout.split(area);

        assert_eq!(chunks.len(), 2, "Layout should create exactly 2 chunks");
    }
}
