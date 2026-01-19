use serde_json::json; 
use reqwest::Client;

// 1. Data Structure
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
            "prompt": format!("<|im_start|>user\n{}<|im_end|>\n<|im_start|>assitant\n", prompt),
            "n_predict": 100
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
}