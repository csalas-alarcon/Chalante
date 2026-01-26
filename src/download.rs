use std::process::Command;
use std::path::Path;
use cmake::Config;

pub async fn install_engine() {
    // 1. Clone the repo if it doesn't exist
    if !Path::new("llama.cpp").exists() {
        Command::new("git")
            .args(["clone", "https://github.com/ggml-org/llama.cpp"])
            .status()
            .expect("Failed to clone llama.cpp");
    }

    // 2. Build using the cmake crate
    Config::new("llama.cpp")
        .profile("Release")
        .build_arg("-j8")
        .build();

    println!("cargo:rerun-if-changed=build.rs");
}

pub async fn install_models() {
    Command::new("./scripts/models.sh");
}