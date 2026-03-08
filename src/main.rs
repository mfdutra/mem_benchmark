mod bandwidth;
mod latency;
mod output;
mod sizes;

use clap::Parser;

#[derive(Parser)]
#[command(name = "mem_benchmark", about = "Memory latency & bandwidth benchmark")]
struct Cli {
    /// Run only the latency benchmark
    #[arg(long)]
    latency_only: bool,

    /// Run only the bandwidth benchmark
    #[arg(long)]
    bandwidth_only: bool,

    /// Minimum buffer size in bytes (must be a power of 2)
    #[arg(long, default_value_t = 1024)]
    min_size: usize,

    /// Maximum buffer size in bytes (must be a power of 2)
    #[arg(long, default_value_t = 1 << 30)]
    max_size: usize,

    /// Number of iterations per measurement (best is kept)
    #[arg(long, default_value_t = 5)]
    iterations: u32,
}

fn pin_to_core_0() {
    let cores = core_affinity::get_core_ids().unwrap_or_default();
    if let Some(&core) = cores.first() {
        core_affinity::set_for_current(core);
        eprintln!("Pinned to core {:?}", core);
    } else {
        eprintln!("Warning: could not pin to a core");
    }
}

fn main() {
    let cli = Cli::parse();
    let run_latency = !cli.bandwidth_only;
    let run_bandwidth = !cli.latency_only;

    pin_to_core_0();

    let test_sizes = sizes::generate_sizes(cli.min_size, cli.max_size);

    if run_latency {
        eprintln!("Running latency benchmark...");
        let results: Vec<_> = test_sizes
            .iter()
            .map(|&size| {
                eprintln!("  latency: {}", sizes::format_size(size));
                latency::measure_latency(size, cli.iterations)
            })
            .collect();
        output::print_latency_table(&results);
    }

    if run_bandwidth {
        eprintln!("Running bandwidth benchmark...");
        let results: Vec<_> = test_sizes
            .iter()
            .map(|&size| {
                eprintln!("  bandwidth: {}", sizes::format_size(size));
                bandwidth::measure_bandwidth(size, cli.iterations)
            })
            .collect();
        output::print_bandwidth_table(&results);
    }
}
