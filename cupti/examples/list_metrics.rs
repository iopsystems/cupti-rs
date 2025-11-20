//! List Available Metrics Example
//!
//! This example shows how to query and list available GPU performance metrics
//! for PM sampling.

use std::ffi::CString;

use anyhow::Context;
use clap::Parser;
use cupti::get_device_chip_name;
use cupti::pmsampling::Sampler;
use cupti::profiler::MetricType;

/// List available GPU performance metrics for PM sampling
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Device index to query
    #[arg(short, long, default_value_t = 0)]
    device: usize,

    /// Show only counter metrics
    #[arg(long)]
    counters: bool,

    /// Show only ratio metrics
    #[arg(long)]
    ratios: bool,

    /// Show only throughput metrics
    #[arg(long)]
    throughputs: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Initialize the CUPTI profiler interface
    let _guard = cupti::initialize().context("failed to initialize CUPTI profiler")?;

    // Get the chip name for the device
    let chip_name_str =
        get_device_chip_name(args.device).context("failed to get device chip name")?;
    let chip_name = CString::new(chip_name_str).context("chip name contains null byte")?;

    println!("Device {}: {}", args.device, chip_name_str);

    // Get counter availability image for the device
    let counter_availability = Sampler::get_counter_availability(args.device)
        .context("failed to get counter availability image")?;

    // Create a sampler builder to query metrics
    let builder = Sampler::builder(&chip_name, &counter_availability)
        .context("failed to create sampler builder")?;

    // If no specific type is requested, show all
    let show_all = !args.counters && !args.ratios && !args.throughputs;

    if show_all || args.counters {
        println!("\nAvailable Counter Metrics:");
        println!("{}", "=".repeat(60));

        let counters = builder
            .get_base_metrics(MetricType::Counter)
            .context("failed to get counter metrics")?;

        for (i, name) in counters.iter().enumerate() {
            println!("  {}. {}", i + 1, name.to_string_lossy());
        }

        println!("\nTotal counter metrics: {}", counters.len());
    }

    if show_all || args.ratios {
        println!("\nAvailable Ratio Metrics:");
        println!("{}", "=".repeat(60));

        let ratios = builder
            .get_base_metrics(MetricType::Ratio)
            .context("failed to get ratio metrics")?;

        for (i, name) in ratios.iter().enumerate() {
            println!("  {}. {}", i + 1, name.to_string_lossy());
        }

        println!("\nTotal ratio metrics: {}", ratios.len());
    }

    if show_all || args.throughputs {
        println!("\nAvailable Throughput Metrics:");
        println!("{}", "=".repeat(60));

        let throughputs = builder
            .get_base_metrics(MetricType::Throughput)
            .context("failed to get throughput metrics")?;

        for (i, name) in throughputs.iter().enumerate() {
            println!("  {}. {}", i + 1, name.to_string_lossy());
        }

        println!("\nTotal throughput metrics: {}", throughputs.len());
    }

    Ok(())
}
