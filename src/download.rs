// src/download.rs

// Generic Imports
use std::process::Command;
use std::path::Path;

// Llama.cpp
pub async fn install_engine() {
    // 1. CMake installed?
    let check_cmake = Command::new("cmake").arg("--version").output();

    if check_cmake.is_err() {
        eprintln!("Error: 'cmake' is not installed. Please install it to build the engine.");
        return; 
    }

    // 2. Clone the repo
    if !Path::new("llama.cpp").exists() {
        let _ = Command::new("git")
            .args(["clone", "https://github.com/ggml-org/llama.cpp"])
            .output();
    }

    // 3. Configure
    let config = Command::new("cmake")
        .args(["-B", "llama.cpp/build", "-S", "llama.cpp"])
        .output();

    // 4. Build
    if config.is_ok() {
        let _ = Command::new("cmake")
            .args(["--build", "llama.cpp/build", "--config", "Release", "-j8"])
            .output();
    }
}

// Models: Phi2, Qwen, Danube
pub async fn install_models() {
    // Just executing the Script
    let status = Command::new("sh")
        .arg("./scripts/models.sh")
        .output()
        .expect("Failed to execute models.sh");
}