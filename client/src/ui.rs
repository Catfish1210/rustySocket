use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Text, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{App, AppScreen, AppSection};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    match app.current_screen {
        AppScreen::Join => render_join_menu(frame, app),
        AppScreen::Chat => render_chat_menu(frame, app),
    }

}


fn render_chat_menu(frame: &mut Frame, app: &App) {
    let size = frame.area();
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(80),
            Constraint::Percentage(20),
        ].as_ref());

    let chunks = layout.split(size);

    let messages: String = app.messages.iter()
        .map(|msg| format!("{}: {}", msg.from_user, msg.content))
        .collect::<Vec<String>>()
        .join("\n");

    let messages_paragraph = Paragraph::new(messages)
        .block(Block::default().title("Chat").borders(Borders::ALL));
    
    frame.render_widget(messages_paragraph, chunks[0]);

    let input_paragraph = Paragraph::new(format!("{}: {}", app.username, app.input))
        .block(Block::default().title("Type a message").borders(Borders::ALL));
    
    frame.render_widget(input_paragraph, chunks[1]);
}

fn render_join_menu(frame: &mut Frame, app: &App) {
    let total_width = frame.area().width;
    let predefined_width = 40;
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - (predefined_width * 100 / total_width)) / 2),
            Constraint::Length(predefined_width),
            Constraint::Percentage((100 - (predefined_width * 100 / total_width)) / 2),
        ])
        .split(frame.area());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(10),
                Constraint::Length(3),  
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(horizontal_chunks[1]);

    // Borderstyles for sections
    let username_border_style = if app.current_section == AppSection::Username {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };

    let color_picker_border_style = if app.current_section == AppSection::ColorPicker {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };

    let connect_button_border_style = if app.current_section == AppSection::ConnectButton {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };

    // Username input
    let input_field = if app.show_cursor {
        format!(" {}_", app.username)
    } else {
        format!(" {}", app.username)
    };

    let username_color = app.colors[app.selected_color].1;
    frame.render_widget(
        Paragraph::new(input_field)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Username")
                    .border_type(BorderType::Rounded)
                    .border_style(username_border_style),
            )
            .style(Style::default().fg(username_color))
            .alignment(Alignment::Left),
        chunks[0],
    );

    let items: Vec<ListItem> = app.colors
        .iter()
        .enumerate()
        .map(|(i, (label, color))| {
            let sym = if i == app.selected_color {
                Span::styled(" x ", Style::default().fg(username_color))
            } else {
                Span::raw("   ")
            };

            ListItem::new(Line::from(vec![
                Span::raw(" - "),
                Span::styled("[", Style::default().fg(Color::White)),
                sym,
                Span::styled("]", Style::default().fg(Color::White)),
                Span::raw(" "),
                Span::styled(label.to_string(), Style::default().fg(*color)),
            ]))
        })
        .collect();
    
    let color_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Select Color")
                .border_type(BorderType::Rounded)
                .border_style(color_picker_border_style),
         )
        .style(Style::default().fg(Color::White));
    frame.render_widget(color_list, chunks[1]);

    // Connect 
    let connect_style = if app.is_connect_selected {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    
    let connect_button = Paragraph::new(Text::from("Connect"))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(connect_button_border_style),
        )
        .style(connect_style)
        .alignment(Alignment::Center);
    frame.render_widget(connect_button, chunks[2]);
}