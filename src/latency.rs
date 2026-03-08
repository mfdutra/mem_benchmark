use rand::Rng;
use std::hint::black_box;
use std::time::Instant;

/// Build a pointer-chase chain using Sattolo's algorithm.
/// Guarantees a single Hamiltonian cycle visiting every element exactly once.
fn build_chain(len: usize) -> Vec<usize> {
    let mut chain: Vec<usize> = (0..len).collect();
    let mut rng = rand::rng();

    // Sattolo shuffle: swap each element with a random earlier position (exclusive)
    // This produces a single cycle of length `len`.
    for i in (1..len).rev() {
        let j = rng.random_range(0..i); // exclusive of i — Sattolo, not Fisher-Yates
        chain.swap(i, j);
    }

    chain
}

/// Result of a single latency measurement.
pub struct LatencyResult {
    pub size_bytes: usize,
    pub latency_ns: f64,
}

/// Measure pointer-chasing latency for a buffer of `size_bytes`.
/// Returns the minimum latency across `iterations` runs.
pub fn measure_latency(size_bytes: usize, iterations: u32) -> LatencyResult {
    let count = size_bytes / std::mem::size_of::<usize>();
    let chain = build_chain(count);
    let chases = 1_000_000usize.max(count * 4);

    let min_ns = (0..iterations)
        .map(|_| {
            let mut idx = 0usize;
            let start = Instant::now();
            for _ in 0..chases {
                // SAFETY: chain is a permutation of 0..count, so all indices are valid.
                idx = unsafe { *chain.get_unchecked(idx) };
            }
            let elapsed = start.elapsed();
            black_box(idx);
            elapsed.as_nanos() as f64 / chases as f64
        })
        .fold(f64::INFINITY, f64::min);

    LatencyResult {
        size_bytes,
        latency_ns: min_ns,
    }
}
