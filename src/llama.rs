// src/llama.rs

// Generic Imports
use serde::Deserialize;
use serde_json::json; 
use reqwest::Client;
use std::process::{Command, Stdio};
use ratatui::text::Line;
// For Communication
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

// My Imports
use crate::App;
use crate::download::{install_engine, install_models};

// LlamaClient Struct
pub struct LlamaClient {
    pub client: Client,
    pub url: String,
    pub user_text: String,
    pub ter_text: String,
    pub history: Vec<Line<'static>>,
    pub models: String,
    pub engine_on: bool,
    // For Communication
    pub tx: UnboundedSender<String>,
    pub rx: UnboundedReceiver<String>,
}

// LLamaClient Methods
impl LlamaClient {
    // Creation
    pub fn new() -> Self {
        // For Communication
        let (tx, rx) = unbounded_channel();
        Self {
            client: Client::new(),
            url: format!("http://127.0.0.1:11343"),
            user_text: String::new(),
            ter_text: String::new(),
            history: Vec::new(),
            models: String::new(),
            engine_on: false,
            tx,
            rx,
        }
    }

    // Once installed, Starts Router Mode
    pub async fn start_llama(&mut self) -> std::process::Child {
        Command::new("llama.cpp/build/bin/llama-server")
        .arg("--models-dir")
        .arg("models")
        .arg("--port")
        .arg("11343")
        .arg("--log-disable")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start llama-server")

        //self.engine_on = true;
    }

    // GET Requests
    pub async fn get_health(&self) -> Result<String, Box<dyn std::error::Error>> {
        let res: serde_json::Value = self.client.get(format!("{}/health", &self.url))
            .send()
            .await?
            .json()
            .await?;

        Ok(res.to_string())
    }

    pub async fn get_models(&self) -> Result<String, Box<dyn std::error::Error>> {
        let res: serde_json::Value = self.client.get(format!("{}/models", &self.url))
            .send()    
            .await?
            .json()
            .await?;
        
        Ok(res.to_string())
    }

    // POST Requests
    pub async fn load_model(&self, model: &str) -> Result<String, Box<dyn std::error::Error>> {
        let body = json!({
            "model": "qwen"
        });
        let res: serde_json::Value = self.client.post(format!("{}/models/load", &self.url))
            .json(&body)
            .send()
            .await?
            .json()
            .await?;

        let mut content = res["success"]
            .as_str()
            .ok_or("Failed to get content")?
            .to_string();

        Ok(content)
    }

    pub async fn ask(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Formulation
        let body = json!({
            "model": "qwen",
            "prompt": format!("\nUser: {}\nAssistant:", prompt),
            "n_predict": 200,
            "temperature": 0.2,
            "stop": ["User:", "Assistant:", "\nUser:", "<|im_end|>", "<|endoftext|>"],
            "cache_prompt": true
        });
        // Petition
        let res: serde_json::Value = self.client.post(format!("{}/completion", &self.url))
            .json(&body)
            .send()
            .await?
            .json()
            .await?;

        // Answer Processing
        let mut content = res["content"]
            .as_str()
            .ok_or("Failed to get content")?
            .trim()
            .to_string();
        
        // Cleaning
        if content.starts_with("Assistant:") {
            content = content.replace("Assistant:", "").trim().to_string();
        }
        Ok(content)
    }

    // This method should be called in your main loop every "frame" or "tick"
    pub fn update_terminal_text(&mut self) {
        // "try_recv" is non-blocking. It checks if there's a new line from the script.
        while let Ok(msg) = self.rx.try_recv() {
            self.ter_text = msg; // Update the field Ratatui reads
        }
    }

    // Parsing Commands
    pub async fn parsing(&mut self, app: &mut App) {
        let text: String = self.user_text.drain(..).collect();

        match text.as_str() {
            "go chat" => app.to_chat(),
            "get health" => { let _ = self.get_health().await; },
            "list models" => { let _ = self.get_models().await; },
            "install engine" => {
                let tx = self.tx.clone();
                // Spawn it so it doesn't block the TUI!
                tokio::spawn(async move {
                    install_engine(tx).await;
                });
            },
            "install models" => {
                let tx = self.tx.clone();
                tokio::spawn(async move {
                    install_models(tx).await;
                });
            },
            "start server" => { let _ = self.start_llama().await; },
            "load model" => { let _ = self.load_model("qwen").await;},
            _ => {},
        }
    }
}