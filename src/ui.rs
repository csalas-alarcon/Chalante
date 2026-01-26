
// src/ui.rs
use crate::app::App; // Import App
use crate::llama::LlamaClient; // Import Client
use ratatui::Frame;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Wrap, List, ListItem, ListState, Gauge},
    text::{Line, Span},          
    style::{Color, Style, Stylize}, 
    DefaultTerminal,
};
// WELCOME SCREEN
pub fn show_welcome(f: &mut Frame) {

    let logo = r#"
   ______ __  __ ___     __     ___     _   __ ______ ______
  / ____// / / //   |   / /    /   |   / | / //_  __// ____/
 / /    / /_/ // /| |  / /    / /| |  /  |/ /  / /  / __/   
/ /___ / __  // ___ | / /___ / ___ | / /|  /  / /  / /___   
\____//_/ /_//_/  |_|/_____//_/  |_|/_/ |_/  /_/  /_____/   

    -- The Efficient Approach to AI --
     Press [ENTER] to initialize
 "#;

    let welcome_block = Paragraph::new(logo)
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(Color::Magenta).bold())
        .block(Block::default()
        .borders(Borders::ALL));

    f.render_widget(welcome_block, f.area());
}

// CONFIGURATION SCREEN
pub fn show_config(f: &mut Frame, app: &App) {
    let logo = r#"
    ______ __  __ ___     __     ___     _   __ ______ ______
   / ____// / / //   |   / /    /   |   / | / //_  __// ____/
  / /    / /_/ // /| |  / /    / /| |  /  |/ /  / /  / __/   
 / /___ / __  // ___ | / /___ / ___ | / /|  /  / /  / /___   
 \____//_/ /_//_/  |_|/_____//_/  |_|/_/ |_/  /_/  /_____/   
 
     -- The Efficient Approach to AI --
      This is the Config Page
  "#;

    let screen = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(10), Constraint::Percentage(40)])
        .split(f.area());

    let title = Paragraph::new(logo)
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(Color::Magenta).bold())
        .block(Block::default()
        .borders(Borders::ALL));

    let progress_bar = Gauge::default()
        .block(Block::default().title("Downloading Llama.cpp").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Magenta))
        .percent(app.download_progress);

    let text = Paragraph::new("Lorem Ipsum")
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(Color::Green).bold())
        .block(Block::default()
        .borders(Borders::ALL));

    f.render_widget(title, screen[0]);
    f.render_widget(progress_bar, screen[1]);
    f.render_widget(text, screen[2]);

}

// CHAT SCREEN
pub fn show_chat(f: &mut Frame, app: &App, client: &LlamaClient) {
    // Split the Screen
    let screen = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(f.area());

    // Split the Info Area
    let info_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(screen[0]);

    // THE LIST [0][0]
    let items: Vec<ListItem> = app.models
        .iter()
        .map(|m| ListItem::new(m.as_str()))
        .collect();
    
    let list = List::new(items)
        .block(Block::default()
        .borders(Borders::ALL)
        .title(" Available Models "))
        .highlight_style(Style::default().bg(Color::Green).fg(Color::Black))
        .highlight_symbol(">> ");
    
    let mut state = ListState::default();
    state.select(Some(app.selected_model_index));

    // THE STATS [0][1]
    let stats = Paragraph::new("Nothing happens")
        .block(Block::default()
        .borders(Borders::ALL)
        .title(" Stats "))
        .wrap(Wrap { trim: false });

    // Split the Chat Area
    let chat_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(screen[1]);
    
    // THE CHAT [1][0]
    let chat = Paragraph::new(client.history.clone()) 
        .block(Block::default().borders(Borders::ALL).title(" The Chat "))
        .wrap(Wrap { trim: true });

    // THE INPUT [1][1]
    let input_box = Paragraph::new(client.user_text.as_str())
        .block(Block::default()
        .borders(Borders::ALL)
        .title(" Your Input "));

    // For the List
    f.render_stateful_widget(list, info_area[0], &mut state);
    // For the Stats Window
    f.render_widget(stats, info_area[1]);
    // For the Chat Itself
    f.render_widget(chat, chat_area[0]);
    // For the Input
    f.render_widget(input_box, chat_area[1]);
}