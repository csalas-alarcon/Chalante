use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Wrap},
    text::{Line, Span},           // ADD THIS
    style::{Color, Style, Stylize}, // ADD THIS
    DefaultTerminal,
};
use crossterm::event::{self, Event, KeyCode};

mod llama;
use llama::LlamaClient;

mod app;
use app::App;



#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    
    // Initialize our state
    let mut app = App {
        input: String::new(),
        messages: Vec::new(),
        ai: LlamaClient::new(11343),
    };

    let result = run_app(&mut terminal, &mut app).await;
    ratatui::restore();
    result
}

async fn run_app(terminal: &mut DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(1),
                    Constraint::Length(3),
                ])
                .split(f.area());
            
            // WRINKLE FIX: Paragraph can take a Vec<Line> directly. 
            // We clone it because Paragraph needs ownership to draw.
            let history = Paragraph::new(app.messages.clone()) 
                .block(Block::default().borders(Borders::ALL).title(" Chaty "))
                .wrap(Wrap { trim: true });
            
            f.render_widget(history, chunks[0]);

            f.render_widget(
                Paragraph::new(app.input.as_str()).block(Block::default().borders(Borders::ALL).title(" User's Input ")),
                chunks[1],
            );
        })?;

        if let Event::Key(key) = event::read()? {
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
                _ => {}
            }
        }
    }
}