//! Simple CUPTI Initialization Example
//!
//! This example demonstrates initializing the CUPTI profiler interface.
//! It simply calls `cupti::initialize()` and ignores the result.

fn main() {
    // Initialize the CUPTI profiler interface
    let _result = cupti::initialize();

    // Ignore the result - we're just testing that linking works
    println!("CUPTI initialization attempted");
}
