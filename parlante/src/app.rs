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
    pub current_model: String,
    pub llama_process: Option<std::process::Child>,
}

impl App {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            messages: Vec::new(),
            ai: LlamaClient::new(),
            current_screen: CurrentScreen::Welcome,
            models: vec!["phi2.gguf".into(), "qwen.gguf".into(), "danube.gguf".into()],
            selected_model_index: 0,
            current_model : String::from("Danube"),
            llama_process: None,
        }
    }
}