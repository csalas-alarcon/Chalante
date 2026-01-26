use std::process::Command;
use std::path::Path;
use cmake::Config;

fn main() {
    // 1. Clone the repo if it doesn't exist
    if !Path::new("llama.cpp").exists() {
        Command::new("git")
            .args(["clone", "https://github.com/ggml-org/llama.cpp"])
            .status()
            .expect("Failed to clone llama.cpp");
    }

    // 2. Build using the cmake crate
    // This replaces: cmake -B build && cmake --build build --config Release -j 8
    Config::new("llama.cpp")
        .profile("Release")
        .build_arg("-j8")
        .build();

    println!("cargo:rerun-if-changed=build.rs");
}