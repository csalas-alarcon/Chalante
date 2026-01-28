// src/main.rs

// Generic Imports
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Wrap, List, ListItem, ListState},
    text::{Line, Span},          
    style::{Color, Style, Stylize}, 
    DefaultTerminal,
};

use crossterm::event::{self, Event, KeyCode};

// My Imports
mod llama;
use llama::{LlamaClient};
mod app;
use app::{App, CurrentScreen};
mod ui;
use ui::{show_welcome, show_config, show_chat};

// ENTRANCE
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize 
    let mut terminal = ratatui::init();
    let mut app = App::new();
    let mut client = LlamaClient::new();

    // Run Main Loop
    let result = run(&mut terminal, &mut app, &mut client).await;
    ratatui::restore();
    result
}

async fn run(terminal: &mut DefaultTerminal, app: &mut App, client: &mut LlamaClient) -> std::io::Result<()> {
    loop {
        terminal.draw(|f| {
            match app.current_screen {
                // The Welcome Screen
                CurrentScreen::Welcome => {
                    show_welcome(f);
                }
                // The Config Screen
                CurrentScreen::Config => {
                    show_config(f, app, client);
                }
                // The Chat Terminal
                CurrentScreen::Chat => {
                    show_chat(f, app, client);
                }
            }
        })?;
        // We Check for Keyboard Actions
        if let Event::Key(key) = event::read()? {
            match app.current_screen {
                // Welcome Ones
                CurrentScreen::Welcome => {
                    match key.code {
                        KeyCode::Enter => {
                            app.current_screen = CurrentScreen::Config;
                        }
                        KeyCode::Esc => {
                            break Ok(());
                        }
                        _ => {}
                    }
                }
                // Config Screen Ones
                CurrentScreen::Config => {
                    match key.code {
                        KeyCode::Enter => {
                            client.parsing(app).await;
                        }
                        KeyCode::Esc => {
                            break Ok(());
                        }
                        // Writing
                        KeyCode::Char(c) => client.user_text.push(c),
                        // Deleting
                        KeyCode::Backspace => { client.user_text.pop(); }
                        _ => {}
                    }
                }
                // Chat Ones
                CurrentScreen::Chat => {
                    match key.code {
                        // Writing
                        KeyCode::Char(c) => client.user_text.push(c),
                        // Deleting
                        KeyCode::Backspace => { client.user_text.pop(); },
                        // Exiting
                        KeyCode::Esc => {
                            break Ok(());
                        }
                        // Asking
                        KeyCode::Enter => {
                            let input: String = client.user_text.drain(..).collect();
                            
                            // User message added
                            client.history.push(Line::from(vec![
                                Span::raw("You: "),
                                Span::styled(input.clone(), Style::default().fg(Color::Cyan)),
                            ]));
                        
                            // AI response added
                            if let Ok(response) = client.ask(&input).await {
                                // AI message: Label is white, but the response text is Yellow
                                client.history.push(Line::from(vec![
                                    Span::raw("AI: "),
                                    Span::styled(response, Style::default().fg(Color::Yellow)),
                                ]));
                            }
                        }
                        // List Commands
                        KeyCode::Up => {
                            if app.selected_model_index > 0 {
                                app.selected_model_index -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if app.selected_model_index < app.models.len() - 1 {
                                app.selected_model_index += 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}