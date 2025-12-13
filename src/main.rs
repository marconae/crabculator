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

            match key.code {
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.quit();
                }
                KeyCode::Char('q') if key.modifiers.is_empty() => {
                    app.quit();
                }
                KeyCode::Char('c') if key.modifiers.is_empty() => {
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

            // Auto-save state after buffer modifications
            if should_save {
                app.save_state();
            }
        }
    }

    terminal::restore_terminal()?;
    Ok(())
}
