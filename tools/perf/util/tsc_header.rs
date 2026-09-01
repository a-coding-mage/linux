/* SPDX-License-Identifier: GPL-2.0 */

// C dependency intent:
// - <linux/types.h> supplies fixed-width integer and bool aliases.
// - "event.h" supplies union perf_event.

#[repr(C)]
pub struct perf_tsc_conversion {
    pub time_shift: u16,
    pub time_mult: u32,
    pub time_zero: u64,
    pub time_cycles: u64,
    pub time_mask: u64,

    pub cap_user_time_zero: bool,
    pub cap_user_time_short: bool,
}

#[repr(C)]
pub struct perf_event_mmap_page {
    _private: [u8; 0],
}

// Opaque Rust stand-in for C's union perf_event, declared in "event.h".
#[repr(C)]
pub struct perf_event {
    _private: [u8; 0],
}

// Opaque Rust stand-in for C's FILE.
#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn perf_read_tsc_conversion(
        pc: *const perf_event_mmap_page,
        tc: *mut perf_tsc_conversion,
    ) -> ::std::os::raw::c_int;

    pub fn perf_time_to_tsc(ns: u64, tc: *mut perf_tsc_conversion) -> u64;
    pub fn tsc_to_perf_time(cyc: u64, tc: *mut perf_tsc_conversion) -> u64;
    pub fn rdtsc() -> u64;
    pub fn arch_get_tsc_freq() -> u64;

    pub fn perf_event__fprintf_time_conv(event: *mut perf_event, fp: *mut FILE) -> usize;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
