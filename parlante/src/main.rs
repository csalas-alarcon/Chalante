use serde_json::json; 
use reqwest::Client;

// 1. Data Structure
struct LlamaClient {
    http_client: Client,
    url: String,
}

// 2. The Struct's methods
impl LlamaClient {
    fn new(port: u16) -> Self {
        Self {
            http_client: Client::new(),
            url: format!("http://127.0.0.1:{}/completion", port),
        }
    }

    //3. Action
    async fn ask(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Professional Use B )
    let ai = LlamaClient::new(11343);

    println!("Making some quantum connections");

    let response = ai.ask("Tell me about the best Football Match in History, which is 2010 World Cup Final").await?;

    println!("\nAI response: {}", response);

    Ok(())
}