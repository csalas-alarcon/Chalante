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

use tokio::sync::mpsc;

// My Imports
mod llama;
mod app;
use app::{App, CurrentScreen};
mod ui;
use ui::{show_welcome, show_config, show_chat};
mod download;

// ENTRANCE
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    
    // Initialize our state
    let mut app = App::new();
    let mut client = LlamaClient::new();

    let result = run(&mut terminal, &mut app).await;
    ratatui::restore();
    result
}


async fn run(terminal: &mut DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    loop {
        terminal.draw(|f| {
            match app.current_screen {
                // The Welcome Screen
                CurrentScreen::Welcome => {
                    show_welcome(f);
                }
                // The Config Screen
                CurrentScreen::Config => {
                    if download_progress <= 0 {
                        if let Ok(progress) = rx.try_recv() {
                            app.download_progress = progress;
                        }
                    }
                    show_config(f);
                }
                // The Chat Terminal
                CurrentScreen::Chat => {
                    show_chat(f, app);
                }
            }
        })?;
        // We Check for Keyboard Actions
        if let Event::Key(key) = event::read()? {
            match app.current_screen {
                // Welcome Screen Ones
                CurrentScreen::Welcome => {
                    if let KeyCode::Enter = key.code {
                        app.current_screen = CurrentScreen::Config;
                    }
                    if let KeyCode::Esc = key.code {
                        break Ok(());
                    }
                }

                CurrentScreen::Config => {
                    if let KeyCode::Enter = key.code {
                        app.current_screen = CurrentScreen::Chat;
                    }
                    if let KeyCode::Esc = key.code {
                        break Ok(());
                    }
                    if let KeyCode::q = key.code {
                        let (tx, mut rx) = mpsc::channel(1);

                        // Spawn the downloader
                        tokio::spawn(async move {
                            download::install_engine(tx).await;
                        });
                    }
                }
                // The Chat Ones
                CurrentScreen::Chat => {
                    match key.code {
                        KeyCode::Char(c) => client.user_text.push(c),
                        KeyCode::Backspace => client.input.pop(),
                        KeyCode::Esc => {
                            break Ok(());
                        }
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
                        KeyCode::Right => {
                            let model_to_load = app.models[app.selected_model_index].clone();
                        
                            // 1. Create a specific copy just for the background thread
                            let model_for_thread = model_to_load.clone(); 
                            let ai_clone = app.ai.clone();
                            
                            tokio::spawn(async move {
                                // This copy gets moved/consumed here
                                let _ = ai_clone.switch_model(&model_for_thread).await;
                            });
                        
                            // 2. Now the original 'model_to_load' is still available for the UI!
                            app.messages.push(Line::from(vec![
                                Span::styled(
                                    format!("System: Loading {}...", model_to_load), 
                                    Style::default().fg(Color::Red).italic()
                                )
                            ]));
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}