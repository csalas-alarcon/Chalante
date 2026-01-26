use serde::Deserialize;
use serde_json::json; 
use reqwest::Client;
use std::process::{Command, Stdio};

// LlamaClient Struct
#[derive(Clone)]
pub struct LlamaClient {
    pub http_client: Client,
    pub url: String,
}

// LLamaClient Methods
impl LlamaClient {
    // Creation
    pub fn new() -> Self {
        Self {
            http_client: Client::new(),
            url: format!("http://127.0.0.1:11343/completion"),
        }
    }

    pub async fn start_llama(&self) -> std::process::Child {
        Command::new("../llama_bin/build/bin/llama-server")
        .arg("--models-dir")
        .arg("../models")
        .arg("--port")
        .arg("11343")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start llama-server")
    }

    pub async fn switch_model(&self, model: &str) -> Result<(), Box<dyn std::error::Error>> {
        let body = json!({
            "model": model
        });

        let load_url = self.url.replace("/completion", "/models/load");

        let response = self.http_client
            .post(&load_url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(())
        } else {
            let err_text = response.text().await?;
            Err(format!("Server failed to load model: {}", err_text).into())
        }
    }

    pub async fn get_models_info_raw(&self) -> Result<String, Box<dyn std::error::Error>> {
        let models_url = self.url.replace("/completion", "/models");

        let response_text = self.http_client
            .get(&models_url)
            .send()
            .await?
            .text()
            .await?;
        
        Ok(response_text)
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
        let res: serde_json::Value = self.http_client
            .post(&self.url)
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