# mem_benchmark

A Rust CLI tool that benchmarks memory latency and bandwidth across buffer sizes (1 KB – 1 GB), revealing L1/L2/L3 cache vs main memory characteristics.

## What it measures

- **Latency**: Pointer-chasing through a Sattolo-shuffled array (single Hamiltonian cycle), reporting nanoseconds per access
- **Bandwidth**: Sequential read and write throughput in GB/s

Results clearly show step changes at cache boundaries, making it easy to identify cache sizes on your hardware.

### Example output (Apple M3 Pro)

```
=== Memory Latency ===
+-------------+--------------+
| Buffer Size | Latency (ns) |
+-------------+--------------+
| 1 KB        | 1.70         |
| 8 KB        | 1.29         |
| 128 KB      | 1.07         |
| 256 KB      | 4.10         |    <-- L1/L2 boundary
| 4 MB        | 7.02         |
| 16 MB       | 10.97        |    <-- L2/L3 boundary
| 32 MB       | 31.35        |    <-- L3/DRAM boundary
| 64 MB       | 65.23        |
+-------------+--------------+

=== Memory Bandwidth ===
+-------------+-------------+--------------+
| Buffer Size | Read (GB/s) | Write (GB/s) |
+-------------+-------------+--------------+
| 1 KB        | 165.84      | 16.31        |
| 128 KB      | 138.86      | 17.34        |
| 256 KB      | 115.03      | 14.11        |
| 64 MB       | 98.66       | 12.54        |
+-------------+-------------+--------------+
```

## Installation

```bash
git clone https://github.com/mfdutra/mem_benchmark.git
cd mem_benchmark
cargo build --release
```

## Usage

```bash
# Full benchmark (1 KB – 1 GB)
./target/release/mem_benchmark

# Latency only, up to 64 MB
./target/release/mem_benchmark --latency-only --max-size 67108864

# Bandwidth only
./target/release/mem_benchmark --bandwidth-only

# Custom range and iterations
./target/release/mem_benchmark --min-size 4096 --max-size 16777216 --iterations 10
```

### Options

| Flag               | Default    | Description                                    |
|--------------------|------------|------------------------------------------------|
| `--latency-only`   | off        | Run only the latency benchmark                 |
| `--bandwidth-only` | off        | Run only the bandwidth benchmark               |
| `--min-size`       | 1024       | Minimum buffer size in bytes (power of 2)      |
| `--max-size`       | 1073741824 | Maximum buffer size in bytes (power of 2)      |
| `--iterations`     | 5          | Iterations per measurement (best run is kept)  |

## How it works

### Latency

A `Vec<usize>` is shuffled with [Sattolo's algorithm](https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle#Sattolo's_algorithm) to create a single cycle that visits every element exactly once. The benchmark chases pointers through this cycle, measuring the average time per access. The **minimum** latency across iterations is reported (hardware is deterministic; variance is OS noise).

### Bandwidth

Sequential read (`iter().sum()`) and write (`iter_mut().for_each()`) loops with `black_box` to prevent dead-code elimination and unwanted compiler optimizations. Repetitions are dynamically calibrated to target ~200 ms of work per measurement. The **maximum** bandwidth across iterations is reported.

### Core pinning

The process is pinned to core 0 via `core_affinity` to reduce scheduling noise.

### Why not C++?

Performance is identical: Both compile to the same quality machine code. The hot loops (pointer chasing, sequential sum) are
trivial enough that LLVM produces equivalent output for both languages. The unsafe get_unchecked in Rust gives us the same codegen
as raw pointer/array access in C++.

Where C++ could arguably help:
- Inline assembly: Easier to use asm volatile with memory fences or specific prefetch instructions if you wanted cycle-level
precision. Rust has core::arch::asm! but it's less ergonomic.
- Intrinsics: More straightforward access to SIMD/cache control intrinsics (_mm_clflush, _mm_prefetch) if you wanted to add
cache-line flushing or non-temporal stores.
- Existing ecosystem: Tools like lmbench and tinymembench are C/C++, so there's more prior art to reference.

Where Rust is better for this project:
- cargo makes dependency management trivial (clap, tabled, core_affinity) — no CMake/vcpkg headaches
- Safe defaults with surgical unsafe only where needed (the one get_unchecked call)
- black_box is stable and purpose-built for benchmarking; C++ equivalent (benchmark::DoNotOptimize) requires Google Benchmark as a
dependency

Bottom line: The hot loops are 3-5 lines of code where both languages produce identical machine code. Everything else (CLI
parsing, table formatting, shuffling) is non-critical code where Rust's tooling is more convenient. Switching to C++ would add
build complexity for zero performance gain.

## License

This project is licensed under the GNU General Public License v3.0. See [LICENSE](LICENSE) for details.
