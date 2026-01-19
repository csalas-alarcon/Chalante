use ratatui::text::Line;
use crate::llama::LlamaClient; // Use 'crate::' to find your other module

pub struct App {
    pub input: String,
    pub messages: Vec<Line<'static>>,
    pub ai: LlamaClient,
}

impl App {
    pub fn new(port: u16) -> Self {
        Self {
            input: String::new(),
            messages: Vec::new(),
            ai: LlamaClient::new(port),
        }
    }
}