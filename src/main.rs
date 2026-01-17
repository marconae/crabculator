use std::io;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};

use crabculator::app;
use crabculator::terminal;
use crabculator::ui;

fn main() -> io::Result<()> {
    terminal::install_panic_hook();

    let mut terminal = terminal::setup_terminal()?;
    let mut app = app::App::new();

    while app.running {
        terminal.draw(|frame| ui::render(frame, &mut app))?;

        if event::poll(std::time::Duration::from_millis(250))?
            && let Event::Key(key) = event::read()?
        {
            // Track whether we need to save state after this key event
            let mut should_save = false;

            // Modal behavior: if help overlay is visible, capture input for overlay
            if app.help_visible {
                match key.code {
                    KeyCode::Char('h') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.toggle_help();
                    }
                    KeyCode::Esc => {
                        app.close_help();
                    }
                    KeyCode::Up => {
                        app.scroll_help_up();
                    }
                    KeyCode::Down => {
                        app.scroll_help_down(ui::HELP_CONTENT_HEIGHT);
                    }
                    KeyCode::PageUp => {
                        // Scroll up by 10 lines
                        for _ in 0..10 {
                            app.scroll_help_up();
                        }
                    }
                    KeyCode::PageDown => {
                        // Scroll down by 10 lines
                        for _ in 0..10 {
                            app.scroll_help_down(ui::HELP_CONTENT_HEIGHT);
                        }
                    }
                    _ => {} // Ignore other keys when help is visible
                }
            } else {
                // Normal editor mode
                match key.code {
                    KeyCode::Char('c' | 'q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.quit();
                    }
                    KeyCode::Char('h') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.toggle_help();
                    }
                    KeyCode::Char('r') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.clear_all();
                        should_save = true;
                    }
                    KeyCode::Esc => {
                        app.quit();
                    }
                    KeyCode::Char(c) => {
                        app.buffer.insert_char(c);
                        should_save = true;
                    }
                    KeyCode::Enter => {
                        app.buffer.insert_newline();
                        should_save = true;
                    }
                    KeyCode::Backspace => {
                        app.buffer.delete_char_before();
                        should_save = true;
                    }
                    KeyCode::Delete => {
                        app.buffer.delete_char_at();
                        should_save = true;
                    }
                    KeyCode::Left | KeyCode::Right
                        if key.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        app.toggle_memory_pane_position();
                    }
                    KeyCode::Left => {
                        app.buffer.move_cursor_left();
                    }
                    KeyCode::Right => {
                        app.buffer.move_cursor_right();
                    }
                    KeyCode::Up => {
                        app.buffer.move_cursor_up();
                    }
                    KeyCode::Down => {
                        app.buffer.move_cursor_down();
                    }
                    KeyCode::Home => {
                        app.buffer.move_cursor_to_line_start();
                    }
                    KeyCode::End => {
                        app.buffer.move_cursor_to_line_end();
                    }
                    _ => {}
                }
            }

            // Auto-save state after buffer modifications
            if should_save {
                app.save_state();
            }
        }
    }

    terminal::restore_terminal()?;
    Ok(())
}
