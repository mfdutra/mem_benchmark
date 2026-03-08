use tabled::{Table, Tabled};

use crate::bandwidth::BandwidthResult;
use crate::latency::LatencyResult;
use crate::sizes::format_size;

#[derive(Tabled)]
struct LatencyRow {
    #[tabled(rename = "Buffer Size")]
    size: String,
    #[tabled(rename = "Latency (ns)")]
    latency_ns: String,
}

#[derive(Tabled)]
struct BandwidthRow {
    #[tabled(rename = "Buffer Size")]
    size: String,
    #[tabled(rename = "Read (GB/s)")]
    read_gbs: String,
    #[tabled(rename = "Write (GB/s)")]
    write_gbs: String,
}

pub fn print_latency_table(results: &[LatencyResult]) {
    let rows: Vec<LatencyRow> = results
        .iter()
        .map(|r| LatencyRow {
            size: format_size(r.size_bytes),
            latency_ns: format!("{:.2}", r.latency_ns),
        })
        .collect();

    println!("\n=== Memory Latency ===");
    println!("{}", Table::new(rows));
}

pub fn print_bandwidth_table(results: &[BandwidthResult]) {
    let rows: Vec<BandwidthRow> = results
        .iter()
        .map(|r| BandwidthRow {
            size: format_size(r.size_bytes),
            read_gbs: format!("{:.2}", r.read_gbs),
            write_gbs: format!("{:.2}", r.write_gbs),
        })
        .collect();

    println!("\n=== Memory Bandwidth ===");
    println!("{}", Table::new(rows));
}
