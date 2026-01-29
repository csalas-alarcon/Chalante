// src/main.rs

// Generic Imports
use ratatui::{
    text::{Line, Span},          
    style::{Color, Style, Stylize}, 
    DefaultTerminal,};
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
// My Imports
mod app;
mod download;
mod llama;
mod ui;

use app::{App, CurrentScreen};
use llama::LlamaClient;
use ui::{show_chat, show_config, show_welcome};

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
        // For Install Scripts
        client.update_terminal_text();
        // For Refreshing the Screen
        terminal.draw(|f| match app.current_screen {
                // The Welcome Screen
                CurrentScreen::Welcome => show_welcome(f),
                // The Config Screen
                CurrentScreen::Config => show_config(f, app, client),
                // The Chat Terminal
                CurrentScreen::Chat => show_chat(f, app, client),
        })?;

        // Run the Loop every 30ms
        if event::poll(Duration::from_millis(30))? {
            // We Check for Keyboard Actions
            if let Event::Key(key) = event::read()? {
                match app.current_screen {
                    // Welcome Actions
                    // This Screen is just for show, it's pretty. 
                    // We can either exit or go to Config
                    CurrentScreen::Welcome => {
                        match key.code {
                            KeyCode::Enter => app.to_config(),
                            KeyCode::Esc => break Ok(()),
                            _ => {}
                        }
                    }
                    // Config Actions
                    // From here everything is installed or initialized
                    // Actions are exiting, go to chat AND (big AND) run commands.
                    CurrentScreen::Config => {
                        match key.code {
                            KeyCode::Enter => {
                                client.parsing(app).await;
                            },
                            KeyCode::Esc => break Ok(()),
                            // Writing
                            KeyCode::Char(c) => client.user_text.push(c),
                            // Deleting
                            KeyCode::Backspace => { client.user_text.pop(); },
                            _ => {}
                        }
                    }
                    // Chat Actions
                    // The Actual TUI Client, actions are literrally
                    // ALL THE FEATURES
                    CurrentScreen::Chat => {
                        match key.code {
                            // Asking
                            KeyCode::Enter => {
                                client.parsing(app).await;
                            }
                            // Exiting
                            KeyCode::Esc => break Ok(()),
                            // Writing
                            KeyCode::Char(c) => client.user_text.push(c),
                            // Deleting
                            KeyCode::Backspace => { client.user_text.pop(); },
                            
                            // Selection Commands
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
}