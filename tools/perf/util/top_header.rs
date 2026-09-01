// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/util/top.h.
// Original dependencies included:
// "tool.h", "evswitch.h", "annotate.h", "mutex.h", "ordered-events.h",
// "record.h", <linux/types.h>, <stddef.h>, <stdbool.h>, <sys/ioctl.h>

use core::ffi::c_char;

#[repr(C)]
pub struct perf_top_qe {
    pub in_: *mut ordered_events,
    pub data: [ordered_events; 2],
    pub rotate: bool,
    pub mutex: mutex,
    pub cond: cond,
}

#[repr(C)]
pub struct perf_top {
    pub tool: perf_tool,
    pub evlist: *mut evlist,
    pub sb_evlist: *mut evlist,
    pub record_opts: record_opts,
    pub evswitch: evswitch,
    /*
     * Symbols will be added here in perf_event__process_sample and will
     * get out after decayed.
     */
    pub samples: u64,
    pub lost: u64,
    pub lost_total: u64,
    pub drop: u64,
    pub drop_total: u64,
    pub kernel_samples: u64,
    pub us_samples: u64,
    pub exact_samples: u64,
    pub guest_us_samples: u64,
    pub guest_kernel_samples: u64,
    pub print_entries: i32,
    pub count_filter: i32,
    pub delay_secs: i32,
    pub max_stack: i32,
    pub hide_kernel_symbols: bool,
    pub hide_user_symbols: bool,
    pub zero: bool,
    // Present in C only when HAVE_SLANG_SUPPORT is defined.
    #[cfg(HAVE_SLANG_SUPPORT)]
    pub use_tui: bool,
    pub use_stdio: bool,
    pub vmlinux_warned: bool,
    pub dump_symtab: bool,
    pub stitch_lbr: bool,
    pub sym_filter_entry: *mut hist_entry,
    pub sym_evsel: *mut evsel,
    pub session: *mut perf_session,
    pub winsize: winsize,
    pub realtime_prio: i32,
    pub sym_filter: *const c_char,
    pub min_percent: f32,
    pub nr_threads_synthesize: u32,
    pub uid_str: *const c_char,

    pub qe: perf_top_qe,
}

pub const CONSOLE_CLEAR: &[u8] = b"\x1B[H\x1B[2J\0";

unsafe extern "C" {
    pub fn perf_top__header_snprintf(top: *mut perf_top, bf: *mut c_char, size: usize) -> usize;
    pub fn perf_top__reset_sample_counters(top: *mut perf_top);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
