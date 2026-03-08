use std::hint::black_box;
use std::time::{Duration, Instant};

pub struct BandwidthResult {
    pub size_bytes: usize,
    pub read_gbs: f64,
    pub write_gbs: f64,
}

const TARGET_DURATION: Duration = Duration::from_millis(200);

/// Calculate the number of repetitions needed to reach ~200ms of work.
fn calibrate_reps(size_bytes: usize) -> usize {
    let count = size_bytes / std::mem::size_of::<u64>();
    let buf = vec![1u64; count];

    let start = Instant::now();
    let _sum: u64 = black_box(buf.iter().sum());
    let single = start.elapsed();

    if single.is_zero() {
        1000
    } else {
        (TARGET_DURATION.as_nanos() / single.as_nanos()).max(1) as usize
    }
}

fn measure_read(buf: &[u64], reps: usize) -> f64 {
    let start = Instant::now();
    for _ in 0..reps {
        let sum: u64 = buf.iter().sum();
        black_box(sum);
    }
    let elapsed = start.elapsed().as_secs_f64();
    let total_bytes = buf.len() * std::mem::size_of::<u64>() * reps;
    total_bytes as f64 / elapsed / 1e9
}

fn measure_write(buf: &mut [u64], reps: usize) -> f64 {
    let start = Instant::now();
    for _ in 0..reps {
        buf.iter_mut().for_each(|x| *x = black_box(42u64));
    }
    let elapsed = start.elapsed().as_secs_f64();
    let total_bytes = buf.len() * std::mem::size_of::<u64>() * reps;
    total_bytes as f64 / elapsed / 1e9
}

/// Measure sequential read/write bandwidth. Returns max across `iterations`.
pub fn measure_bandwidth(size_bytes: usize, iterations: u32) -> BandwidthResult {
    let count = size_bytes / std::mem::size_of::<u64>();
    let mut buf = vec![1u64; count];
    let reps = calibrate_reps(size_bytes);

    let (max_read, max_write) = (0..iterations).fold((0.0f64, 0.0f64), |(mr, mw), _| {
        (mr.max(measure_read(&buf, reps)), mw.max(measure_write(&mut buf, reps)))
    });

    BandwidthResult {
        size_bytes,
        read_gbs: max_read,
        write_gbs: max_write,
    }
}
