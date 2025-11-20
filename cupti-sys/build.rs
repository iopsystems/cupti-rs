use std::env;
use std::path::PathBuf;

fn main() {
    // Try to find CUPTI library path
    let cupti_lib_path = find_cupti_lib_path();

    if let Some(path) = cupti_lib_path {
        println!("cargo:rustc-link-search=native={}", path.display());
    }

    // Link to CUPTI library
    println!("cargo:rustc-link-lib=cupti");

    // Re-run if these environment variables change
    println!("cargo:rerun-if-env-changed=CUDA_PATH");
    println!("cargo:rerun-if-env-changed=CUDA_ROOT");
    println!("cargo:rerun-if-env-changed=CUPTI_PATH");
}

fn find_cupti_lib_path() -> Option<PathBuf> {
    // Check environment variables first
    if let Ok(cupti_path) = env::var("CUPTI_PATH") {
        let path = PathBuf::from(cupti_path);
        if path.exists() {
            return Some(path);
        }
    }

    // Check CUDA_PATH or CUDA_ROOT
    for var in &["CUDA_PATH", "CUDA_ROOT"] {
        if let Ok(cuda_path) = env::var(var) {
            // Try lib64 first (common on Linux)
            let lib64_path = PathBuf::from(&cuda_path).join("lib64");
            if lib64_path.exists() {
                return Some(lib64_path);
            }
            // Try lib
            let lib_path = PathBuf::from(&cuda_path).join("lib");
            if lib_path.exists() {
                return Some(lib_path);
            }
        }
    }

    // Check common installation locations
    let common_paths = vec![
        "/usr/local/cuda/lib64",
        "/usr/local/cuda/lib",
        "/usr/lib/x86_64-linux-gnu",  // Debian/Ubuntu package location
        "/usr/lib64",
        "/usr/lib",
        "/opt/cuda/lib64",
        "/opt/cuda/lib",
    ];

    for path_str in common_paths {
        let path = PathBuf::from(path_str);
        // Check if libcupti exists in this directory
        if path.join("libcupti.so").exists() || path.join("libcupti.a").exists() {
            return Some(path);
        }
    }

    // If we can't find it, return None and let the linker search default paths
    None
}
