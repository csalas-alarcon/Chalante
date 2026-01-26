use cmake::Config;

use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let vendor_path = Path::new(&out_dir).join("llama.cpp");

    // Cloning llama.cpp Repo
    if !vendor_path.exists() {
        let status = Command::new("git")
            .args(["clone", "https://github.com/ggml-org/llama.cpp.git"])
            .arg(&vendor_path)
            .status()
            .expect("Failed to clone llama.cpp");

        if !status.success() {
            panic!("Git clone failed with status {}", status);
        }
    }

    // Compiling with CMAKE
    let dst = Config::new(&vendor_path)
        .define("GGML_CUDA", "OFF")
        .define("GGML_VULKAN", "OFF")
        .define("LLAMA_BUILD_EXAMPLES", "ON")
        .define("CMAKE_BUILD_TYPE", "RELEASE")
        .build();

    // 3. Set the environment variable so your Rust code knows where the server is
    // Note: cmake-rs usually puts binaries in /bin/ within the output directory
    println!("cargo:rustc-env=LLAMA_SERVER_PATH={}/bin/llama-server", dst.display());

    // 4. Link the library
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=llama");
    println!("cargo:rerun-if-changed=build.rs");
}