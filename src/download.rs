use std::process::Command;
use std::path::Path;

pub async fn install_engine() {
    // 1. Check if cmake is even installed
    let check_cmake = Command::new("cmake").arg("--version").output();

    if check_cmake.is_err() {
        eprintln!("Error: 'cmake' is not installed. Please install it to build the engine.");
        return; 
    }

    // 2. Clone the repo
    if !Path::new("llama.cpp").exists() {
        let _ = Command::new("git")
            .args(["clone", "https://github.com/ggml-org/llama.cpp"])
            .status();
    }

    // 3. Configure
    let config = Command::new("cmake")
        .args(["-B", "llama.cpp/build", "-S", "llama.cpp"])
        .status();

    // 4. Build
    if config.is_ok() && config.unwrap().success() {
        let _ = Command::new("cmake")
            .args(["--build", "llama.cpp/build", "--config", "Release", "-j8"])
            .status();
    }
}

pub async fn install_models() {
    // We use "sh" to run the script to avoid permission issues
    let status = Command::new("sh")
        .arg("./scripts/models.sh")
        .status() // This actually RUNS the command
        .expect("Failed to execute models.sh");

    if status.success() {
        println!("Models installed successfully!");
    } else {
        eprintln!("Script failed with exit code: {:?}", status.code());
    }
}