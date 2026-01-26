use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    // Rerun if llama.cpp source changes
    println!("cargo:rerun-if-changed=inference/llama.cpp");

    // CMake Config
    let mut dst = Config::new("inference/llama.cpp");

    // Detecting Hardware Acceleration
     if cfg!(feature = "cuda") {
        dst.define("GGML_CUDA", "ON");
     }

    dst.define("CMAKE_BUILD_TYPE", "RELEASE");

    // The Build Process
    let _built_path = dst.build();
}