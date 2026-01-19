mod llama;
use llama::LlamaClient;

struct App {
    input: String, 
    messages: Vec<String>,
    ai_client: LlamaClient,
    is_loading: bool
}