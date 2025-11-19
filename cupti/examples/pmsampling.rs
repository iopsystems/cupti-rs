//! PM Sampling Example
//!
//! This example demonstrates how to use CUPTI's PM Sampling API to collect
//! GPU performance metrics at regular intervals.

use std::ffi::{CStr, CString};
use std::time::Duration;

use anyhow::{Context, bail};
use clap::Parser;
use cupti::pmsampling::*;
use cupti::{CStringList, get_device_chip_name};

/// PM Sampling example - collect GPU performance metrics at regular intervals
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Device index to sample
    #[arg(short, long, default_value_t = 0)]
    device: usize,

    /// Sampling interval in microseconds
    #[arg(short, long, default_value_t = 1000)]
    interval: u64,

    /// Hardware buffer size in bytes
    #[arg(short, long, default_value_t = 1024 * 1024)]
    buffer_size: usize,

    /// Maximum number of samples to collect
    #[arg(short, long, default_value_t = 100)]
    max_samples: u32,

    /// Metrics to collect (comma-separated)
    #[arg(short, long, default_value = "sm__cycles_elapsed.avg,sm__warps_active.avg.pct_of_peak_sustained_active")]
    metrics: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Initialize the CUPTI profiler interface
    let _guard = cupti::initialize()
        .context("failed to initialize CUPTI profiler")?;

    // Get the chip name for the device
    let chip_name_str = get_device_chip_name(args.device)
        .context("failed to get device chip name")?;
    let chip_name = CString::new(chip_name_str)
        .context("chip name contains null byte")?;

    println!("Device {}: {}", args.device, chip_name_str);

    // Get counter availability image for the device
    let counter_availability = Sampler::get_counter_availability(args.device)
        .context("failed to get counter availability image")?;

    // Create a sampler builder
    let mut builder = Sampler::builder(&chip_name, &counter_availability)
        .context("failed to create sampler builder")?;

    // Parse the metrics from command line argument
    let metric_names: CStringList = args
        .metrics
        .split(',')
        .map(|s| CString::new(s.trim()).unwrap())
        .collect();

    if metric_names.is_empty() {
        bail!("no metrics specified");
    }

    println!("\nMetrics to collect:");
    for name in &metric_names {
        println!("  - {}", name.to_string_lossy());
    }

    // Add metrics to the builder
    builder
        .add_metrics(&metric_names)
        .context("failed to add metrics to sampler")?;

    // Build the sampler (this enables PM sampling on the device)
    let mut sampler = builder
        .build(args.device)
        .context("failed to build sampler")?;

    // Convert sampling interval from microseconds to nanoseconds for GpuTimeInterval trigger
    let sampling_interval_ns = args.interval * 1000;

    // Set the sampling configuration
    // Note: GpuTimeInterval trigger mode is only supported on Ampere GA10x and later
    // For Turing/GA100, use GpuSysclkInterval instead
    sampler
        .set_config(
            args.buffer_size,
            sampling_interval_ns,
            TriggerMode::GpuSysclkInterval, // Use sysclk for broader compatibility
            HardwareBufferAppendMode::KeepOldest,
        )
        .context("failed to set sampler config")?;

    println!("\nSampler configured:");
    println!("  Buffer size: {} bytes", args.buffer_size);
    println!("  Sampling interval: {} cycles", sampling_interval_ns);
    println!("  Trigger mode: GpuSysclkInterval");
    println!("  Buffer mode: KeepOldest");

    // Create metric names as CStr slice for counter data image
    let metric_name_cstrs: Vec<&CStr> = metric_names.iter().collect();

    // Create the counter data image buffer
    let mut counter_data = CounterDataImage::new(&sampler, &metric_name_cstrs, args.max_samples)
        .context("failed to create counter data image")?;

    println!("\nStarting PM sampling...");

    // Start sampling
    sampler.start().context("failed to start sampling")?;

    // Run some GPU workload
    // In a real application, you would run your CUDA kernels here
    // For this example, we'll just sleep to allow time for samples to be collected
    println!("Running workload (sleeping for 100ms)...");
    std::thread::sleep(Duration::from_millis(100));

    // Stop sampling
    sampler.stop().context("failed to stop sampling")?;

    println!("Sampling stopped.");

    // Decode the collected data from hardware buffer to counter data image
    let decode_status = sampler
        .decode_data(&mut counter_data)
        .context("failed to decode sampling data")?;

    println!("\nDecode status:");
    println!("  Stop reason: {:?}", decode_status.stop_reason);
    if decode_status.overflow != 0 {
        println!("  WARNING: Hardware buffer overflow detected!");
        println!("  Consider increasing buffer size or sampling interval.");
    }

    // Get information about the collected samples
    let data_info = counter_data
        .get_data_info()
        .context("failed to get counter data info")?;

    println!("\nSample statistics:");
    println!("  Total samples: {}", data_info.num_total_samples);
    println!("  Populated samples: {}", data_info.num_populated_samples);
    println!("  Completed samples: {}", data_info.num_completed_samples);

    if data_info.num_completed_samples == 0 {
        println!("\nNo samples collected. This could be because:");
        println!("  - No GPU activity occurred during sampling");
        println!("  - The sampling interval was too long");
        println!("  - The hardware buffer was too small");
        return Ok(());
    }

    // Print sample timestamps and evaluate metrics for each sample
    println!("\nSample data:");
    println!("{}", "-".repeat(80));

    for i in 0..data_info.num_completed_samples {
        let sample_info = counter_data
            .get_sample_info(&sampler, i)
            .context("failed to get sample info")?;

        let duration_ns = sample_info.end_timestamp - sample_info.start_timestamp;

        println!(
            "Sample {}: timestamps [{} - {}] (duration: {} ns)",
            i,
            sample_info.start_timestamp,
            sample_info.end_timestamp,
            duration_ns
        );

        // Evaluate the metrics for this sample
        let values = counter_data
            .evaluate(&sampler, i, &metric_names)
            .context("failed to evaluate metrics")?;

        for (name, value) in metric_names.iter().zip(values.iter()) {
            println!("  {}: {:.6}", name.to_string_lossy(), value);
        }
        println!();
    }

    println!("{}", "-".repeat(80));
    println!("PM sampling example completed successfully!");

    Ok(())
}
