use std::process::Command;
use std::path::Path;
use cmake::Config;

use tokio::sync::mpsc;

pub async fn install_engine(tx: mpsc::Sender<u16>) {
    // 1. Clone the repo if it doesn't exist
    let _ = tx.send(10).await;
    if !Path::new("llama.cpp").exists() {
        Command::new("git")
            .args(["clone", "https://github.com/ggml-org/llama.cpp"])
            .status()
            .expect("Failed to clone llama.cpp");
    }

    let _ = tx.send(50).await;
    // 2. Build using the cmake crate
    // This replaces: cmake -B build && cmake --build build --config Release -j 8
    Config::new("llama.cpp")
        .profile("Release")
        .build_arg("-j8")
        .build();

    println!("cargo:rerun-if-changed=build.rs");
    let _ = tx.send(100).await;
}