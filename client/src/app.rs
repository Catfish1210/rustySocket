use std::error;
use std::time::{Duration, Instant};
use ratatui::style::Color;
use crossterm::event::KeyCode;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, PartialEq, Eq)]
pub enum AppSection {
    Username,
    ColorPicker,
    ConnectButton,
}
#[derive(Debug, PartialEq, Eq)]
pub enum AppScreen {
    Join,
    Chat,
    // More screens here
}
#[derive(Debug)]
pub struct Message {
    pub from_user: String,
    pub content: String,
}

impl Message {
    pub fn new(from_user: String, content: String) -> Self {
        Self { from_user, content }
    }
}
/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub counter: u8,
    pub username: String,
    pub show_cursor: bool,
    pub selected_color: usize,
    pub is_connect_selected: bool,
    pub last_blink: Instant,
    pub colors: Vec<(&'static str, Color)>,
    pub current_section: AppSection,
    pub current_screen: AppScreen,
    pub messages: Vec<Message>,
    pub input: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            username: String::new(),
            show_cursor: true,
            selected_color: 0,
            is_connect_selected: false,
            last_blink: Instant::now(),
            colors: vec![
                ("Red", Color::Red),
                ("Green", Color::Green),
                ("Blue", Color::Blue),
                ("Cyan", Color::Cyan),
                ("Magenta", Color::Magenta),
                ("Yellow", Color::Yellow),
            ],
            current_section: AppSection::Username,
            current_screen: AppScreen::Join,
            messages: vec![
                Message::new("Alpha".to_string(), "Hello1".to_string()),
                Message::new("Bravo".to_string(), "Hello2".to_string()),
                Message::new("Charlie".to_string(), "Hello3".to_string()),
            ],
            input: String::new(),
        }
    }
}

impl App {
    
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }
    
    // Blink update
    pub fn update_blink(&mut self) {
        if self.current_section == AppSection::Username {
            if self.last_blink.elapsed() >= Duration::from_millis(500) {
                self.show_cursor = !self.show_cursor;
                self.last_blink = Instant::now();
            }
        } else {
            self.show_cursor = false;
        }

    }
    
    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}
