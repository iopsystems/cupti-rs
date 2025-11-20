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
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH")
        .expect("CARGO_CFG_TARGET_ARCH should be set by Cargo");
    let target_os = env::var("CARGO_CFG_TARGET_OS")
        .expect("CARGO_CFG_TARGET_OS should be set by Cargo");
    let target_env = env::var("CARGO_CFG_TARGET_ENV")
        .expect("CARGO_CFG_TARGET_ENV should be set by Cargo");

    let targets_lib = format!("targets/{}-{}/lib", target_arch, target_os);

    // Debian/Ubuntu multiarch tuple (e.g., x86_64-linux-gnu, aarch64-linux-gnu)
    let multiarch = if target_os == "linux" && target_env == "gnu" {
        Some(format!("{}-{}-{}", target_arch, target_os, target_env))
    } else {
        None
    };

    // Check environment variables for custom CUDA installation
    for var in &["CUDA_HOME", "CUDA_PATH", "CUDA_ROOT"] {
        if let Ok(cuda_path) = env::var(var) {
            println!("cargo:rustc-link-search=native={}/{}", cuda_path, targets_lib);
            println!("cargo:rustc-link-search=native={}/lib64", cuda_path);
            println!("cargo:rustc-link-search=native={}/lib", cuda_path);
            if let Some(ref ma) = multiarch {
                println!("cargo:rustc-link-search=native={}/lib/{}", cuda_path, ma);
            }
        }
    }

    // Add standard CUDA installation paths
    println!("cargo:rustc-link-search=native=/usr/local/cuda/{}", targets_lib);
    println!("cargo:rustc-link-search=native=/usr/local/cuda/lib64");
    println!("cargo:rustc-link-search=native=/usr/local/cuda/lib");
    if let Some(ref ma) = multiarch {
        // Debian/Ubuntu package installations place libraries here
        println!("cargo:rustc-link-search=native=/usr/local/cuda/lib/{}", ma);
    }
}
