// src/llama.rs

// Generic Imports
use serde::Deserialize;
use serde_json::json; 
use reqwest::Client;
use std::process::{Command, Stdio};
use ratatui::{
    text::{Line, Span},          
    style::{Color, Style, Stylize}, };
// For Communication
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

// My Imports
use crate::app::{ App, CurrentScreen };
use crate::download::{install_engine, install_models};

// Helper Structs (Just to read Models' JSON)
#[derive(Deserialize)]
struct ModelList {
    data: Vec<ModelEntry>,
}

#[derive(Deserialize)]
struct ModelEntry {
    id: String,
    status: ModelStatus,
}

#[derive(Deserialize)]
struct ModelStatus {
    value: String,
}


// LlamaClient Struct
pub struct LlamaClient {
    pub client: Client,
    pub url: String,
    pub user_text: String,
    pub ter_text: Vec<String>,
    pub history: Vec<Line<'static>>,
    pub models: String,
    pub engine_on: bool,
    // For Communication
    pub tx: UnboundedSender<String>,
    pub rx: UnboundedReceiver<String>,
    // To get a Hold of the Server
    pub actual_model: String,
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
            ter_text: Vec::new(),
            history: Vec::new(),
            models: String::new(),
            engine_on: false,
            tx,
            rx,
            actual_model: String::from("qwen"),
        }
    }

    // Once installed, Starts Router Mode
    pub async fn start_llama(&mut self) {
        let child = Command::new("llama.cpp/build/bin/llama-server")
        .args(["--models-dir", "models", "--port", "11343", "--log-disable"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start llama-server");

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

    pub fn readable(&self, raw_json: &str) -> String {
        // 1. Parse the raw string into our structs
        let parsed: Result<ModelList, _> = serde_json::from_str(raw_json);

        match parsed {
            Ok(list) => {
                // 2. Map each entry into a clean string line
                let lines: Vec<String> = list.data
                    .iter()
                    .map(|m| format!("- {}: [{}]", m.id, m.status.value))
                    .collect();

                // 3. Join them with newlines
                if lines.is_empty() {
                    "No models found.".to_string()
                } else {
                    lines.join("\n")
                }
            }
            Err(_) => "Error: Could not parse model list.".to_string(),
        }
    }

    // POST Requests
    pub async fn load_model(&mut self, model: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.actual_model = model.to_string();
        let body = json!({
            "model": model
        });
        let res: serde_json::Value = self.client.post(format!("{}/models/load", &self.url))
            .json(&body)
            .send()
            .await?
            .json()
            .await?;

        let content = res["success"]
            .as_str()
            .ok_or("Failed to get content")?
            .to_string();

        
        Ok(content)
    }

    pub async fn ask(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Formulation
        let body = json!({
            "model": self.actual_model,
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
        while let Ok(msg) = self.rx.try_recv() {
            self.ter_text.push(msg);
            
            // Keep only the last 5 lines
            if self.ter_text.len() > 10 {
                self.ter_text.remove(0);
            }
        }
    }

    // Parsing Commands
    pub async fn parsing(&mut self, app: &mut App) {
        let text: String = self.user_text.drain(..).collect();
        match app.current_screen {
            // Parsing for the Config Page
            CurrentScreen::Config => {
                match text.as_str() {
                    "go chat" => app.to_chat(),
                    "get health" => { 
                        if let Ok(health) = self.get_health().await {
                            self.ter_text.clear(); // Clear old logs
                            self.ter_text.push(health); // Add the new one
                        } else {
                            self.ter_text.clear();
                            self.ter_text.push("Error: Server unreachable".to_string());
                        }
                    },
                    "list models" => { 
                        if let Ok(models) = self.get_models().await {
                            self.ter_text.clear();
                            // readable returns a String, so we push it
                            self.ter_text.push(self.readable(&models)); 
                        } else {
                            self.ter_text.clear();
                            self.ter_text.push("Error: Could not retrieve models".to_string());
                        }
                    },
                    "start server" => { 
                        let _ = self.start_llama().await; 
                        self.ter_text.clear();
                        self.ter_text.push("Llama Server Started".to_string());
                    },
                    "load model" => { 
                        if let Ok(res) = self.load_model("qwen").await {
                            self.ter_text.clear();
                            self.ter_text.push(format!("Model Loaded: {}", res));
                        }
                    },
                    "load model qwen" => { 
                        if let Ok(res) = self.load_model("qwen").await {
                            self.ter_text.clear();
                            self.ter_text.push(format!("Model Loaded: {}", res));
                        }
                    },
                    "load model phi2" => { 
                        if let Ok(res) = self.load_model("phi2").await {
                            self.ter_text.clear();
                            self.ter_text.push(format!("Model Loaded: {}", res));
                        }
                    },
                    "load model danube" => { 
                        if let Ok(res) = self.load_model("danube").await {
                            self.ter_text.clear();
                            self.ter_text.push(format!("Model Loaded: {}", res));
                        }
                    },
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
                    _ => {},
                }
            }
            // Parsing for the Chat Page
            CurrentScreen::Chat => {
                match text.as_str() {
                    "go config" => app.to_config(),
                    "get health" => { 
                        if let Ok(health) = self.get_health().await {
                            self.ter_text.clear(); // Clear old logs
                            self.ter_text.push(health); // Add the new one
                        } else {
                            self.ter_text.clear();
                            self.ter_text.push("Error: Server unreachable".to_string());
                        }
                    },
                    "list models" => { 
                        if let Ok(models) = self.get_models().await {
                            self.ter_text.clear();
                            // readable returns a String, so we push it
                            self.ter_text.push(self.readable(&models)); 
                        } else {
                            self.ter_text.clear();
                            self.ter_text.push("Error: Could not retrieve models".to_string());
                        }
                    },
                    _ => {
                        // User message added
                        self.history.push(Line::from(vec![
                            Span::raw("You: "),
                            Span::styled(text.clone(), Style::default().fg(Color::Cyan)),
                        ]));

                        // AI response added
                        if let Ok(response) = self.ask(&text).await {
                            self.history.push(Line::from(vec![
                                Span::raw("AI: "),
                                Span::styled(response, Style::default().fg(Color::Yellow)),
                            ]));
                        }
                    }
                }
            }
            _ => {},
        }
        
    }
}