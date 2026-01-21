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
            "prompt": format!("<|begin_of_text|><|start_header_id|>user<|end_header_id|>\n\n{}<|eot_id|><|start_header_id|>assistant<|end_header_id|>\n\n", prompt),
            "n_predict": 200,
            "stop": ["<|eot_id|>", "<|start_header_id|>", "You:", "User:"] // Stop tokens!
        });

        let res: serde_json::Value = self.http_client
            .post(&self.url)
            .json(&body)
            .send()
            .await?
            .json()
            .await?;

        // Extract the Text or Else....
        let content = res["content"]
            .as_str()
            .ok_or("Failed to get content from Llama")?
            .to_string();
        
            Ok(content)
    }

    // 4. Switching Models
    pub async fn switch_model(&self, model_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("http://127.0.0.1:11343/models/load");
        let body = serde_json::json!({ "model": format!("parlante/models/{}", model_name) });

        self.http_client.post(&url).json(&body).send().await?;
        Ok(())
    }
}