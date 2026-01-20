// src/main.rs
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Wrap, List, ListItem, ListState},
    text::{Line, Span},          
    style::{Color, Style, Stylize}, 
    DefaultTerminal,
};
use crossterm::event::{self, Event, KeyCode};

mod llama;

mod app;
use app::{App, CurrentScreen};


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    
    // Initialize our state
    let mut app = App::new(11343);

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
                    let logo = r#"
   ______ __  __ ___     __     ___     _   __ ______ ______
  / ____// / / //   |   / /    /   |   / | / //_  __// ____/
 / /    / /_/ // /| |  / /    / /| |  /  |/ /  / /  / __/   
/ /___ / __  // ___ | / /___ / ___ | / /|  /  / /  / /___   
\____//_/ /_//_/  |_|/_____//_/  |_|/_/ |_/  /_/  /_____/   

       -- The Quantum Chat Terminal --
        Press [ENTER] to initialize
    "#;

                    let welcome_block = Paragraph::new(logo)
                        .alignment(ratatui::layout::Alignment::Center)
                        .style(Style::default().fg(Color::Magenta).bold())
                        .block(Block::default().borders(Borders::ALL));

                    f.render_widget(welcome_block, f.area());
                }
                // The Chat Terminal
                CurrentScreen::Chat => {
                    // We Split the Screen
                    let main_layout = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                        .split(f.area());
                    // We Split the Left Side
                    let sidebar= Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                        .split(main_layout[0]);

                    // Split the Right Side into Chat and Input
                    let chat_area = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Min(1), Constraint::Length(3)])
                        .split(main_layout[1]);
                    
                    // The List (Top Left) - Ratatui::widgets::{List, ListItem};
                    let items: Vec<ListItem> = 
                        app.models
                        .iter()
                        .map(|m| ListItem::new(m.as_str()))
                        .collect();
                    
                    let list = List::new(items)
                        .block(Block::default().borders(Borders::ALL).title(" Available Models "))
                        .highlight_style(Style::default().bg(Color::Green).fg(Color::Black))
                        .highlight_symbol(">> ");
                    
                    let mut state = ListState::default();
                    state.select(Some(app.selected_model_index));

                    f.render_stateful_widget(list, sidebar[0], &mut state);

                    // The Blank Box (Bottom Left)
                    f.render_widget(Block::default().borders(Borders::ALL).title(" Stats "), sidebar[1]);
                    
                    // The Chat History (Top Right)
                    let history = Paragraph::new(app.messages.clone()) 
                    .block(Block::default().borders(Borders::ALL).title(" The Chat "))
                    .wrap(Wrap { trim: true });

                    f.render_widget(history, chat_area[0]);

                    // The Input Box (Bottom Right)
                    let input_box = Paragraph::new(app.input.as_str()).block(Block::default().borders(Borders::ALL).title(" Your Input "));
                    f.render_widget(input_box, chat_area[1]);

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
                        KeyCode::Esc => break Ok(()),
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
                        _ => {}
                    }
                }
            }
        }
    }
}