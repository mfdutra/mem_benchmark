# mem_benchmark

Memory latency & bandwidth benchmark tool written in Rust.

## Build & Run
```bash
cargo build --release
./target/release/mem_benchmark              # full run (1KB–1GB)
./target/release/mem_benchmark --latency-only --max-size 67108864  # quick latency test
```

## Architecture
- `sizes.rs` — power-of-2 buffer size generation & human-readable formatting
- `latency.rs` — pointer-chasing latency using Sattolo shuffle (single Hamiltonian cycle)
- `bandwidth.rs` — sequential read/write bandwidth with dynamic rep calibration
- `output.rs` — tabular formatting via `tabled`
- `main.rs` — CLI (clap), core pinning, orchestration

## Key Design Notes
- Sattolo shuffle (not Fisher-Yates) ensures one full-length cycle for accurate latency measurement
- `unsafe get_unchecked` in latency loop to avoid bounds-check overhead (~0.3ns)
- Min latency / max bandwidth across iterations (best run = most representative)
- Dynamic reps in bandwidth to target ~200ms of work per measurement
