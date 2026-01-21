use serde_json::json; 
use reqwest::Client;

// 1. Data Structure
#[derive(Clone)]
pub struct LlamaClient {
    pub http_client: Client,
    pub url: String,
}

// 2. The Struct's methods
impl LlamaClient {
    pub fn new(port: u16) -> Self {
        Self {
            http_client: Client::new(),
            url: format!("http://127.0.0.1:{}/completion", port),
        }
    }

    //3. Action
    pub async fn ask(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let body = json!({
            // No fancy tags, just clear labels.
            "prompt": format!("\nUser: {}\nAssistant:", prompt),
            "n_predict": 200,
            "temperature": 0.2, // Lower temperature makes it less likely to hallucinate
            "stop": ["User:", "Assistant:", "\nUser:", "<|im_end|>", "<|endoftext|>"],
            "cache_prompt": true
        });
    
        let res: serde_json::Value = self.http_client
            .post(&self.url)
            .json(&body)
            .send()
            .await?
            .json()
            .await?;
    
        let mut content = res["content"]
            .as_str()
            .ok_or("Failed to get content")?
            .trim()
            .to_string();
        
        // Qwen and Phi often leave the word "Assistant:" in the string, let's kill it.
        if content.starts_with("Assistant:") {
            content = content.replace("Assistant:", "").trim().to_string();
        }
    
        Ok(content)
    }

    // 4. Switching Models
    pub async fn switch_model(&self, model_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("http://127.0.0.1:11343/models/load");
        let body = serde_json::json!({ "model": format!("models/{}", model_name) });

        self.http_client.post(&url).json(&body).send().await?;
        Ok(())
    }
}