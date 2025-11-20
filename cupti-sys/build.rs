use std::env;

fn main() {
    // Add CUDA library search paths
    add_cuda_lib_paths();

    // Link to CUPTI library
    println!("cargo:rustc-link-lib=cupti");

    // Re-run if these environment variables change
    println!("cargo:rerun-if-env-changed=CUDA_HOME");
    println!("cargo:rerun-if-env-changed=CUDA_PATH");
    println!("cargo:rerun-if-env-changed=CUDA_ROOT");
}

fn add_cuda_lib_paths() {
    // Check environment variables for custom CUDA installation
    for var in &["CUDA_HOME", "CUDA_PATH", "CUDA_ROOT"] {
        if let Ok(cuda_path) = env::var(var) {
            println!("cargo:rustc-link-search=native={}/lib64", cuda_path);
            println!("cargo:rustc-link-search=native={}/lib", cuda_path);
            println!("cargo:rustc-link-search=native={}/lib/x86_64-linux-gnu", cuda_path);
        }
    }

    // Add standard CUDA installation paths
    println!("cargo:rustc-link-search=native=/usr/local/cuda/lib64");
    println!("cargo:rustc-link-search=native=/usr/local/cuda/lib");
    // Debian/Ubuntu package installations place libraries here
    println!("cargo:rustc-link-search=native=/usr/local/cuda/lib/x86_64-linux-gnu");
}
