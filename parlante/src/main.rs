use serde_json::json; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // JSON BODY
    let body = json!({
        "prompt": "<|im_start|>user\nWrite a 1-sentence poem for my beloved.<|im_end|>\n<|im_start|>assistant\n",
        "n_predict": 50
    });

    println!("Connecting to Llama.cpp... ");

    let res = client
        .post("http://127.0.0.1:11343/completion")
        .json(&body)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    if let Some(content) = res.get("content") {
        println!("\nLlama says: {}", content);
    }

    Ok(())
}