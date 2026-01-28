//src/app.rs

// Posible Screen
pub enum CurrentScreen {
    Welcome,
    Config,
    Chat,
}

// App Struct
pub struct App {
    pub current_screen: CurrentScreen,
    pub download_progress: u16,
    pub engine_installed: bool,
    pub models: Vec<String>,
    pub selected_model_index: usize,
}

// App Methods
impl App {
    pub fn new() -> Self {
        Self {
            current_screen: CurrentScreen::Welcome,
            download_progress: 0,
            engine_installed: false,
            models: vec!["phi2.gguf".into(), "qwen.gguf".into(), "danube.gguf".into()],
            selected_model_index: 0,
        }
    }

    pub fn to_chat(&mut self) {
        self.current_screen = CurrentScreen::Chat;
    }

    pub fn to_config(&mut self) {
        self.current_screen = CurrentScreen::Config;
    }
}