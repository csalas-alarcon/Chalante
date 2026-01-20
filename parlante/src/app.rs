//src/app.rs

use ratatui::text::Line;
use crate::llama::LlamaClient; // Use 'crate::' to find your other module

pub enum CurrentScreen {
    Welcome,
    Chat,
}

pub struct App {
    pub input: String,
    pub messages: Vec<Line<'static>>,
    pub ai: LlamaClient,
    pub current_screen: CurrentScreen,
    pub models: Vec<String>,
    pub selected_model_index: usize,
}

impl App {
    pub fn new(port: u16) -> Self {
        Self {
            input: String::new(),
            messages: Vec::new(),
            ai: LlamaClient::new(port),
            current_screen: CurrentScreen::Welcome,
            models: vec!["Llama 3.2 1B".into(), "Gemma 2B".into(), "Phi-3 Mini".into()],
            selected_model_index: 0,
        }
    }
}