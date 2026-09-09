/* SPDX-License-Identifier: GPL-2.0-only */

// The C header guard is omitted; Rust items are scoped by the containing module.

pub const MEM_RANGE_CHUNK_SZ: usize = 2048; /* Memory ranges size chunk */

extern "C" {
    pub fn sort_memory_ranges(mrngs: *mut crate::crash_mem, merge: bool);
    pub fn realloc_mem_ranges(mem_ranges: *mut *mut crate::crash_mem) -> *mut crate::crash_mem;
    pub fn add_mem_range(mem_ranges: *mut *mut crate::crash_mem, base: u64, size: u64) -> i32;
    pub fn get_exclude_memory_ranges(mem_ranges: *mut *mut crate::crash_mem) -> i32;
    pub fn get_reserved_memory_ranges(mem_ranges: *mut *mut crate::crash_mem) -> i32;
    pub fn get_crash_memory_ranges(mem_ranges: *mut *mut crate::crash_mem) -> i32;
    pub fn get_usable_memory_ranges(mem_ranges: *mut *mut crate::crash_mem) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
