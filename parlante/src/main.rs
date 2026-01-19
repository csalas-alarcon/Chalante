use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal,
};
use crossterm::event::{self, Event, KeyCode};

mod llama;
use llama::LlamaClient;

// PUT THE STRUCT HERE
struct App {
    input: String,
    messages: Vec<String>,
    ai: LlamaClient,
}

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
                    Constraint::Min(1),    // Chat History
                    Constraint::Length(3), // Input Box
                ])
                .split(f.area());
            
            // Draw Chat History
            let history_text = app.messages.join("\n");
            f.render_widget(
                Paragraph::new(history_text).block(Block::default().borders(Borders::ALL).title(" Chaty ")),
                chunks[0],
            );

            // Draw Input Box
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
                    app.messages.push(format!("You: {}", user_text));

                    // This is where the "freeze" happens. 
                    // We'll fix this with Channels next.
                    if let Ok(response) = app.ai.ask(&user_text).await {
                        app.messages.push(format!("AI: {}", response));
                    }
                }
                _ => {}
            }
        }
    }
}