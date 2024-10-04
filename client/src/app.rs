use std::error;
use std::time::{Duration, Instant};
use ratatui::style::Color;
use crossterm::event::KeyCode;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    pub username: String,
    pub show_cursor: bool,
    pub selected_color: usize,
    pub is_connect_selected: bool,
    pub last_blink: Instant,
    pub colors: Vec<(&'static str, Color)>,
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
        if self.last_blink.elapsed() >= Duration::from_millis(500) {
            self.show_cursor = !self.show_cursor; // Visibility
            self.last_blink = Instant::now();
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
