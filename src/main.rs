use std::io;
use std::time::Instant;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};

use crabculator::app;
use crabculator::terminal;
use crabculator::ui;

#[allow(clippy::too_many_lines)]
fn main() -> io::Result<()> {
    terminal::install_panic_hook();

    let mut terminal = terminal::setup_terminal()?;
    let mut app = app::App::new();

    while app.running {
        terminal.draw(|frame| ui::render(frame, &mut app))?;

        if event::poll(std::time::Duration::from_millis(250))?
            && let Event::Key(key) = event::read()?
        {
            let mut should_save = false;

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
                        for _ in 0..10 {
                            app.scroll_help_up();
                        }
                    }
                    KeyCode::PageDown => {
                        for _ in 0..10 {
                            app.scroll_help_down(ui::HELP_CONTENT_HEIGHT);
                        }
                    }
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Char('c' | 'q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.quit();
                    }
                    KeyCode::Char('h') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.toggle_help();
                    }
                    KeyCode::Char('r') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.clear_all();
                        app.last_edit_time = Some(Instant::now());
                        should_save = true;
                    }
                    KeyCode::Esc => {
                        app.quit();
                    }
                    KeyCode::Char(c) => {
                        app.buffer.insert_char(c);
                        app.last_edit_time = Some(Instant::now());
                        should_save = true;
                    }
                    KeyCode::Enter => {
                        app.buffer.insert_newline();
                        app.last_edit_time = Some(Instant::now());
                        should_save = true;
                    }
                    KeyCode::Backspace => {
                        app.buffer.delete_char_before();
                        app.last_edit_time = Some(Instant::now());
                        should_save = true;
                    }
                    KeyCode::Delete => {
                        app.buffer.delete_char_at();
                        app.last_edit_time = Some(Instant::now());
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

            if should_save {
                app.save_state();
            }
        }
    }

    terminal::restore_terminal()?;
    Ok(())
}
