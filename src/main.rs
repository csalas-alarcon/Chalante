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
mod app;
use app::{App, CurrentScreen};
mod ui;
use ui::{show_welcome, show_chat};

// ENTRANCE
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    
    // Initialize our state
    let mut app = App::new();

    let result = run_app(&mut terminal, &mut app).await;
    ratatui::restore();
    result
}


async fn run_app(terminal: &mut DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    loop {
        terminal.draw(|f| {
            match app.current_screen {
                // The Welcome Screen
                CurrentScreen::Welcome => {
                    show_welcome(f);
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
                        app.current_screen = CurrentScreen::Chat;
                    }
                    if let KeyCode::Esc = key.code {
                        break Ok(());
                    }
                }
                // The Chat Ones
                CurrentScreen::Chat => {
                    match key.code {
                        KeyCode::Char(c) => app.input.push(c),
                        KeyCode::Backspace => { app.input.pop(); },
                        KeyCode::Esc => {
                            if let Some(mut child) = app.llama_process.take() {
                                let _ = child.kill();
                            }
                            break Ok(());
                        }
                        KeyCode::Enter => {
                            let user_text: String = app.input.drain(..).collect();
                            
                            // User message: Label is white, but the text is Cyan
                            app.messages.push(Line::from(vec![
                                Span::raw("You: "),
                                Span::styled(user_text.clone(), Style::default().fg(Color::Cyan)),
                            ]));
                        
                            // Ask the AI
                            if let Ok(response) = app.ai.ask(&user_text).await {
                                // AI message: Label is white, but the response text is Yellow
                                app.messages.push(Line::from(vec![
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
                        KeyCode::Left => {
                            if let Ok(raw_json) = app.ai.get_models_info_raw().await {
                                // This will put the literal {"models": [...]} string in the box
                                app.current_model = raw_json; 
                            }
                        }
                        KeyCode::F(1) => {
                            if app.llama_process.is_none() {
                                app.messages.push(Line::from(vec![
                                    Span::styled("System: Starting Llama Server...", Style::default().fg(Color::Green))
                                ]));
                                let child = app.ai.start_llama().await;
                                app.llama_process = Some(child);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}