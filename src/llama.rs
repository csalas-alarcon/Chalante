use serde::Deserialize;
use serde_json::json; 
use reqwest::Client;
use std::process::{Command, Stdio};
use ratatui::text::Line;

// LlamaClient Struct
#[derive(Clone)]
pub struct LlamaClient {
    pub client: Client,
    pub url: String,
    pub user_text: String,
    pub history: Vec<Line<'static>>,
    pub models: String,
    pub engine_on: bool,
}

// LLamaClient Methods
impl LlamaClient {
    // Creation
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            url: format!("http://127.0.0.1:11343"),
            user_text: String::new(),
            history: Vec::new(),
            models: String::new(),
            engine_on: false,
        }
    }

    // Once installed, Starts Router Mode
    pub async fn start_llama(&mut self) -> std::process::Child {
        Command::new("llama.cpp/build/bin/llama-server")
        .arg("--models-dir")
        .arg("models")
        .arg("--port")
        .arg("11343")
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
            "model": model
        });
        let res: serde_json::Value = self.client.post(format!("{}/models/load", &self.url))
            .json(&body)
            .send()
            .await?
            .json()
            .await?;

        let mut content = res["succes"]
            .as_str()
            .ok_or("Failed to get content")?
            .to_string();

        Ok(content)
    }

    // Query
    pub async fn ask(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Formulation
        let body = json!({
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
}