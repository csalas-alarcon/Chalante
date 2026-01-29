// src/ui.rs


// Generic Imports
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

// My Imports
use crate::app::App; 
use crate::llama::LlamaClient; 
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
pub fn show_config(f: &mut Frame, app: &App, client: &LlamaClient) {

    let instructions = r#"From here you control the whole platform.
Follow this steps if it's your first time:

Install llama.cpp           ->  "install engine"
Install Initial models      ->  "install models"
Start llama-server          ->  "start server"
Load the Default Model      ->  "load model"

To load a specific model (quen, phi2 or danube):
-> "load model <model>"

For Diagnostics you (here and in the Chat Area):
Get list of Cached Models   ->  "list models"
Get Server Status           ->  "get health"

To go from place to place:
Go to chat Area (Config)    ->  "go chat"
Go to Config Page (Chat)    -> "go config""#;

    let screen = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(f.area());

    // Split the Interactive
    let interactive_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(screen[1]);

    // 2. Build the Paragraph
    let config_panel = Paragraph::new(instructions)
        .alignment(ratatui::layout::Alignment::Left)
        .style(Style::default().fg(Color::Magenta))
        .block(
            // Yep, this is just for the title
            Block::default()
                .borders(Borders::ALL)
                .title(" CONFIG PAGE ")
                .title_alignment(ratatui::layout::Alignment::Center) 
                .title_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Magenta))
        );

    let text = Paragraph::new(client.user_text.as_str())
        .alignment(ratatui::layout::Alignment::Left)
        .style(Style::default().fg(Color::Green).bold())
        .block(Block::default()
        .borders(Borders::ALL)
        .title(" Command-Line "))
        .wrap(Wrap { trim: false });

    let output = Paragraph::new(client.ter_text.join("\n"))
        .alignment(ratatui::layout::Alignment::Left)
        .block(Block::default()
        .borders(Borders::ALL)
        .title(" Output "))
        .wrap(Wrap { trim: false });

    
    /* Future Feature for Installing LLama.cpp
    let progress_bar = Gauge::default()
        .block(Block::default().title("Downloading Llama.cpp").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Magenta))
        .percent(app.download_progress);
    f.render_widget(progress_bar, screen[3]);
    */
    f.render_widget(config_panel, screen[0]);
    f.render_widget(text, interactive_area[0]);
    f.render_widget(output, interactive_area[1]);
    

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
    let stats = Paragraph::new(client.ter_text.join("\n"))
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