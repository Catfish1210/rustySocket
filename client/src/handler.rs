use crate::app::{App, AppResult, AppScreen, AppSection, Message};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.current_section {
        // Input section
        AppSection::Username => match key_event.code {
            KeyCode::Enter => {
                // Move to color picker section
                app.current_section = AppSection::ColorPicker;
            }
            KeyCode::Backspace => {
                app.username.pop();
            }
            KeyCode::Char(c) => {
                app.username.push(c);
            }
            KeyCode::Right => {
                app.current_section = AppSection::ColorPicker;
            }
            
            _ => {}
        },
        // Color section
        AppSection::ColorPicker => match key_event.code {
            KeyCode::Enter => {
                app.current_section = AppSection::ConnectButton;
            }
            KeyCode::Up => {
                if app.selected_color > 0 {
                    app.selected_color -= 1;
                }
            }
            KeyCode::Down => {
                if app.selected_color < app.colors.len() - 1 {
                    app.selected_color += 1;
                }
            }
            KeyCode::Left => {
                app.current_section = AppSection::Username;
            }
            KeyCode::Right => {
                app.current_section = AppSection::ConnectButton;
            }
            
            _ => {}
        },
        // Connect btn
        AppSection::ConnectButton => match key_event.code {
            KeyCode::Enter => {
                app.is_connect_selected = true;
                // Finalize and start connecting proccess ***TODO
                app.current_screen = AppScreen::Chat; // Switch scenes
            }
            KeyCode::Left => {
                app.current_section = AppSection::ColorPicker;
            }
            _ => {}
        },
    }

    match app.current_screen {
        AppScreen::Chat => match key_event.code {
            KeyCode::Enter => {
                // Send message
                if !app.input.is_empty() {
                    let message = Message::new(app.username.clone(), app.input.clone());
                    app.messages.push(message);
                    app.input.clear();
                }
            }
            KeyCode::Backspace => {
                app.input.pop();
            }
            KeyCode::Char(c) => {
                app.input.push(c);
            }

            _ => {}
        },
        AppScreen::Join => { }
    }

    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }

        _ => {}
    }

    Ok(())
}
