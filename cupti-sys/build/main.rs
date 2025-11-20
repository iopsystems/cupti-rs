use crate::cuda_sdk::CudaSdk;

mod cuda_sdk;

fn main() {
    let sdk = CudaSdk::new().expect("cannot create CUDA SDK instance");

    println!(
        "Discovered CUDA SDK Version {}.{}",
        sdk.driver_version_major(),
        sdk.driver_version_minor()
    );

    println!("cargo::rerun-if-changed=build");
    for e in sdk.related_cuda_envs() {
        println!("cargo::rerun-if-env-changed={e}");
    }

    for libdir in sdk.cuda_library_paths() {
        println!("cargo::rustc-link-search=native={}", libdir.display());
    }

    println!("cargo::rustc-link-lib=dylib=cupti");
}
