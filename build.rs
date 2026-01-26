use cmake::Config;

fn main() {
    println!("cargo:rerun-if-changed=vendor/llama.cpp");

    let mut dst = Config::new("vendor/llama.cpp");

    // Enable Vulkan for broad Linux compatibility
    dst.define("GGML_VULKAN", "ON");
    
    // Optional: Still allow CUDA if the feature is enabled
    if cfg!(feature = "cuda") {
        dst.define("GGML_CUDA", "ON");
    }

    dst.define("CMAKE_BUILD_TYPE", "Release");
    
    // Build
    let built_path = dst.build();

    // Link the search path
    println!("cargo:rustc-link-search=native={}/lib", built_path.display());
    println!("cargo:rustc-link-lib=static=llama");
}