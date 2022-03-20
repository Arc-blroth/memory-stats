#![doc = include_str!("../README.md")]

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
/// current process. If the current memory usage
/// cannot be queried, `None` is returned.
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
