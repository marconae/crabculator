//! Layout management for the Crabculator TUI.
//!
//! Provides the main layout constraints for the split-panel interface,
//! including the command bar at the bottom of the screen.

use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Layout areas for the main UI.
///
/// Contains the computed areas for content panels and the command bar.
#[derive(Debug, Clone, Copy)]
pub struct LayoutAreas {
    /// The area for the main content (input and results panels).
    pub content_area: Rect,
    /// The area for the command bar at the bottom.
    pub command_bar: Rect,
}

/// Creates the main layout with content area and command bar.
///
/// The layout divides the terminal into:
/// - Content area (all but last 2 rows): For input and results panels
/// - Command bar (2 rows): Horizontal separator line + command text
///
/// # Arguments
/// * `area` - The total available area to divide
///
/// # Returns
/// A `LayoutAreas` struct containing the computed areas.
#[must_use]
pub fn create_main_layout(area: Rect) -> LayoutAreas {
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Content area takes remaining space
            Constraint::Length(2), // Command bar: 1 row separator + 1 row text
        ]);

    let chunks = vertical_layout.split(area);

    LayoutAreas {
        content_area: chunks[0],
        command_bar: chunks[1],
    }
}

/// Creates the horizontal panel layout with 80/20 split.
///
/// The layout divides the content area into two panels:
/// - Input/expression area (80%)
/// - Memory/results area (20%)
///
/// # Arguments
/// * `memory_pane_left` - When true, memory pane is on left (20%/80%); when false, on right (80%/20%)
#[must_use]
pub fn create_panel_layout(memory_pane_left: bool) -> Layout {
    let constraints = if memory_pane_left {
        [Constraint::Percentage(20), Constraint::Percentage(80)]
    } else {
        [Constraint::Percentage(80), Constraint::Percentage(20)]
    };
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Main Layout Tests (Vertical: content area + command bar) ===

    #[test]
    fn main_layout_creates_two_areas() {
        let area = Rect::new(0, 0, 100, 50);
        let areas = create_main_layout(area);

        // Should have content area and command bar
        assert!(
            areas.content_area.height > 0,
            "Content area should have height"
        );
        assert!(
            areas.command_bar.height > 0,
            "Command bar should have height"
        );
    }

    #[test]
    fn main_layout_command_bar_is_two_rows() {
        let area = Rect::new(0, 0, 100, 50);
        let areas = create_main_layout(area);

        assert_eq!(
            areas.command_bar.height, 2,
            "Command bar should be exactly 2 rows (separator + text)"
        );
    }

    #[test]
    fn main_layout_command_bar_at_bottom() {
        let area = Rect::new(0, 0, 100, 50);
        let areas = create_main_layout(area);

        assert_eq!(
            areas.command_bar.y, 48,
            "Command bar should be at the bottom (y=48 for height 50 with 2-row bar)"
        );
    }

    #[test]
    fn main_layout_content_area_fills_remaining() {
        let area = Rect::new(0, 0, 100, 50);
        let areas = create_main_layout(area);

        assert_eq!(
            areas.content_area.height, 48,
            "Content area should be 48 rows (50 - 2 for command bar)"
        );
        assert_eq!(areas.content_area.y, 0, "Content area should start at y=0");
    }

    #[test]
    fn main_layout_command_bar_full_width() {
        let area = Rect::new(0, 0, 100, 50);
        let areas = create_main_layout(area);

        assert_eq!(
            areas.command_bar.width, 100,
            "Command bar should span full width"
        );
        assert_eq!(areas.command_bar.x, 0, "Command bar should start at x=0");
    }

    // === Panel Layout Tests (Horizontal: input + results) ===

    #[test]
    fn panel_layout_creates_two_chunks() {
        let layout = create_panel_layout(false);
        let area = Rect::new(0, 0, 100, 49);
        let chunks = layout.split(area);

        assert_eq!(chunks.len(), 2, "Layout should create exactly 2 chunks");
    }

    #[test]
    fn panel_layout_splits_80_20_memory_right() {
        let layout = create_panel_layout(false);
        let area = Rect::new(0, 0, 100, 49);
        let chunks = layout.split(area);

        // Input panel (left) should be 80% of width
        assert_eq!(chunks[0].width, 80, "Input panel should be 80% width");
        // Memory panel (right) should be 20% of width
        assert_eq!(chunks[1].width, 20, "Memory panel should be 20% width");
    }

    #[test]
    fn panel_layout_splits_20_80_memory_left() {
        let layout = create_panel_layout(true);
        let area = Rect::new(0, 0, 100, 49);
        let chunks = layout.split(area);

        // Memory panel (left) should be 20% of width
        assert_eq!(chunks[0].width, 20, "Memory panel should be 20% width");
        // Input panel (right) should be 80% of width
        assert_eq!(chunks[1].width, 80, "Input panel should be 80% width");
    }

    #[test]
    fn panel_layout_preserves_height() {
        let layout = create_panel_layout(false);
        let area = Rect::new(0, 0, 100, 49);
        let chunks = layout.split(area);

        assert_eq!(chunks[0].height, 49, "Left panel should preserve height");
        assert_eq!(chunks[1].height, 49, "Right panel should preserve height");
    }

    #[test]
    fn panel_layout_is_horizontal() {
        let layout = create_panel_layout(false);
        let area = Rect::new(0, 0, 100, 49);
        let chunks = layout.split(area);

        // Both chunks should be at y=0 (horizontal layout)
        assert_eq!(chunks[0].y, 0, "Left panel should start at y=0");
        assert_eq!(chunks[1].y, 0, "Right panel should start at y=0");

        // Right panel should start after left panel
        assert_eq!(chunks[1].x, 80, "Right panel should start at x=80");
    }

    // === Layout adapts to terminal resize ===

    #[test]
    fn layout_adapts_to_terminal_resize() {
        // Test with different terminal sizes
        let small = Rect::new(0, 0, 80, 24);
        let large = Rect::new(0, 0, 200, 100);

        let small_areas = create_main_layout(small);
        let large_areas = create_main_layout(large);

        // Command bar should always be 2 rows (separator + text)
        assert_eq!(small_areas.command_bar.height, 2);
        assert_eq!(large_areas.command_bar.height, 2);

        // Content area should adapt
        assert_eq!(small_areas.content_area.height, 22);
        assert_eq!(large_areas.content_area.height, 98);
    }
}
