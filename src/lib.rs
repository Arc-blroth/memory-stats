//! # Memory Stats
//!
//! A cross-platform memory profiler for Rust, supporting Windows, Linux, and MacOS. This crate provides two metrics:
//!
//! - **"Physical" Memory**, which corresponds to the _Resident Set Size_ on Linux and MacOS and the _Working Set_ on Windows.
//! - **"Virtual" Memory**, which corresponds to the _Virtual Size_ on Linux and MacOS and the _Pagefile Usage_ on Windows.
//!
//! ## Usage
//!
//! Add `memory-stats` as a dependency to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! memory-stats = "1.0.0"
//! ```
//!
//! ## Example
//!
//! Here's an example that prints out the current memory usage:
//!
//! ```
//! use memory_stats::memory_stats;
//!
//! if let Some(usage) = memory_stats() {
//!     println!("Current physical memory usage: {}", usage.physical_mem);
//!     println!("Current virtual memory usage: {}", usage.virtual_mem);
//! } else {
//!     println!("Couldn't get the current memory usage :(");
//! }
//! ```
//!
//! ## Caveats
//!
//! Getting accurate memory usage on Linux is fairly expensive and not always possible. This crate always attempts to use the statistics from
//! [`/proc/self/smaps`](https://man7.org/linux/man-pages/man5/proc.5.html#:~:text=See%20user_namespaces%287%29.-,/proc/%5Bpid%5D/smaps,-%28since%20Linux%202.6.14)
//! if avaliable. However, since support for `/proc/self/smaps` might not be compiled in on all kernels, this crate will also use the faster but less accurate statistics from
//! [`/proc/self/statm`](https://man7.org/linux/man-pages/man5/proc.5.html#:~:text=by%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20waitpid%282%29.-,/proc/%5Bpid%5D/statm,-Provides%20information%20about)
//! as a fallback.

#[cfg_attr(target_os = "windows", path = "windows.rs")]
#[cfg_attr(any(target_os = "linux", target_os = "android"), path = "linux.rs")]
#[cfg_attr(any(target_os = "macos", target_os = "ios"), path = "darwin.rs")]
mod platform;

#[cfg(not(any(
    target_os = "windows",
    target_os = "linux",
    target_os = "android",
    target_os = "macos",
    target_os = "ios",
)))]
mod platform {
    pub fn memory_stats() -> Option<MemoryStats> {
        None
    }
}

/// Statistics on the memory used by the current process.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct MemoryStats {
    /// The "physical" memory used by this process.
    /// This corresponds to the following
    /// metric on each platform:
    /// - **Linux, Android, MacOS, iOS**: Resident Set Size
    /// - **Windows**: Working Set
    pub physical_mem: usize,

    /// The "virtual" memory used by this process.
    /// This corresponds to the following
    /// metric on each platform:
    /// - **Linux, Android, MacOS, iOS**: Virtual Size
    /// - **Windows**: Pagefile Usage
    pub virtual_mem: usize,
}

/// Returns a snapshot of the the memory used by the
/// current process.
///
/// # Errors
///
/// If the current memory usage cannot be queried
/// or `memory_stats` is run on a unsupported platform,
/// `None` is returned.
#[cfg_attr(
    not(any(
        target_os = "windows",
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
    )),
    deprecated("memory-stats doesn't support this platform!")
)]
pub fn memory_stats() -> Option<MemoryStats> {
    platform::memory_stats()
}
