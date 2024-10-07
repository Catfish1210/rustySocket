use crate::app::{App, AppResult, AppSection};
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

        AppSection::ConnectButton => match key_event.code {
            KeyCode::Enter => {
                app.is_connect_selected = true;
                // Finalize and start connecting proccess ***TODO
            }
            KeyCode::Left => {
                app.current_section = AppSection::ColorPicker;
            }
            _ => {}
        },
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
//        // Connect
//        KeyCode::Enter => {
//            if app.is_connect_selected {
//                //app.AttemptConnection ***TODO fetch name aw
//            }
//        }
//        
//        KeyCode::Up => {
//            if app.is_connect_selected{
//                app.is_connect_selected = false;
//            } else if app.selected_color > 0 {
//                app.selected_color -= 1;
//            }
//        }
//
//        KeyCode::Down => {
//           if !app.is_connect_selected && app.selected_color < 5 {
//               app.selected_color += 1;
//           } else {
//               app.is_connect_selected = true;
//           }
//        }
//
//        KeyCode::Char(c) if !app.is_connect_selected => {
//            app.username.push(c);
//        }
//        KeyCode::Backspace if !app.is_connect_selected => {
//            app.username.pop();
//        }
        ////





        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
