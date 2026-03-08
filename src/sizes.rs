use std::iter::successors;

/// Generate power-of-2 buffer sizes from `min` to `max` (inclusive).
pub fn generate_sizes(min: usize, max: usize) -> Vec<usize> {
    successors(Some(min), |&s| Some(s * 2))
        .take_while(|&s| s <= max)
        .collect()
}

/// Format a byte count as a human-readable string (KB/MB/GB).
pub fn format_size(bytes: usize) -> String {
    const GB: usize = 1 << 30;
    const MB: usize = 1 << 20;
    const KB: usize = 1 << 10;

    if bytes >= GB && bytes % GB == 0 {
        format!("{} GB", bytes / GB)
    } else if bytes >= MB && bytes % MB == 0 {
        format!("{} MB", bytes / MB)
    } else if bytes >= KB && bytes % KB == 0 {
        format!("{} KB", bytes / KB)
    } else {
        format!("{} B", bytes)
    }
}
