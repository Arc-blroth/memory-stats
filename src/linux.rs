use std::fs;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crate::MemoryStats;

const SMAPS: &str = "/proc/self/smaps";
const STATM: &str = "/proc/self/statm";

static SMAPS_CHECKED: AtomicBool = AtomicBool::new(false);
static SMAPS_EXIST: AtomicBool = AtomicBool::new(false);
static PAGE_SIZE: AtomicUsize = AtomicUsize::new(0);

pub fn memory_stats() -> Option<MemoryStats> {
    // If possible, we try to use /proc/self/smaps to retrieve
    // accurate memory usage, but it's not avaliable on all
    // kernels. We use the inaccurate /proc/self/statm stats
    // as a fallback in case smaps isn't avaliable.

    if let Ok(false) = SMAPS_CHECKED.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed) {
        let smaps_exist = fs::metadata(SMAPS).is_ok();

        if !smaps_exist && PAGE_SIZE.load(Ordering::Relaxed) == 0 {
            let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
            if page_size == -1 {
                // sysconf returned error
                return None;
            } else {
                PAGE_SIZE.store(page_size as usize, Ordering::Relaxed);
            }
        }

        // store SMAPS_EXIST last to prevent code from loading a PAGE_SIZE of 0
        SMAPS_EXIST.store(smaps_exist, Ordering::Relaxed);
    }

    if SMAPS_EXIST.load(Ordering::Relaxed) {
        match fs::read_to_string(SMAPS) {
            Ok(smap_info) => {
                // smaps returns a list of different areas of memory
                // and the sizes of each, in kB. smaps_rollup doesn't
                // include a Size row, so we'll add up the sizes
                // ourselves.

                let mut total_size_kb: usize = 0;
                let mut total_rss_kb: usize = 0;

                for line in smap_info.lines() {
                    if line.starts_with("Size:") {
                        total_size_kb += scan_int(&line[5..]).0;
                    } else if line.starts_with("Rss:") {
                        total_rss_kb += scan_int(&line[4..]).0;
                    }
                }

                // note: "kB" actually means 1024 bytes, see
                // https://github.com/torvalds/linux/blob/0014404f9c18dd360a1b8bb4243643c679ce99bf/fs/proc/task_mmu.c#L802
                Some(MemoryStats {
                    physical_mem: total_rss_kb << 10,
                    virtual_mem: total_size_kb << 10,
                })
            }
            Err(_) => None,
        }
    } else {
        match fs::read_to_string(STATM) {
            Ok(statm_info) => {
                // statm returns the virtual size and rss, in
                // multiples of the page size, as the first
                // two columns of output.

                let page_size = PAGE_SIZE.load(Ordering::Relaxed);
                let (total_size_pages, idx) = scan_int(&statm_info);
                let (total_rss_pages, _) = scan_int(&statm_info[idx..]);
                Some(MemoryStats {
                    physical_mem: total_rss_pages * page_size,
                    virtual_mem: total_size_pages * page_size,
                })
            }
            Err(_) => None,
        }
    }
}

/// Extracts a positive integer from a string that
/// may contain leading spaces and trailing chars.
/// Returns the extracted number and the index of
/// the next character in the string.
fn scan_int(string: &str) -> (usize, usize) {
    let mut out = 0;
    let mut idx = 0;
    let mut chars = string.chars().peekable();
    while let Some(' ') = chars.next_if_eq(&' ') {
        idx += 1;
    }
    while let Some(n) = chars.next() {
        idx += 1;
        if n >= '0' && n <= '9' {
            out *= 10;
            out += n as usize - '0' as usize;
        } else {
            break;
        }
    }
    (out, idx)
}
