// Rust translation of perf/util/ftrace.h.
// C includes removed:
// - <linux/list.h> provides struct list_head.
// - "target.h" provides struct target.
//
// Forward declarations from the C header:
// struct evlist;
// struct hashmap;
// struct stats;

use std::os::raw::{c_char, c_int, c_uint, c_ulong};

#[repr(C)]
pub struct perf_ftrace {
    pub evlist: *mut evlist,
    pub target: target,
    pub tracer: *const c_char,
    pub filters: list_head,
    pub notrace: list_head,
    pub graph_funcs: list_head,
    pub nograph_funcs: list_head,
    pub event_pair: list_head,
    pub profile_hash: *mut hashmap,
    pub percpu_buffer_size: c_ulong,
    pub inherit: bool,
    pub use_nsec: bool,
    pub bucket_range: c_uint,
    pub min_latency: c_uint,
    pub max_latency: c_uint,
    pub bucket_num: c_uint,
    pub hide_empty: bool,
    pub graph_depth: c_int,
    pub func_stack_trace: c_int,
    pub func_irq_info: c_int,
    pub graph_args: c_int,
    pub graph_retval: c_int,
    pub graph_retval_hex: c_int,
    pub graph_retaddr: c_int,
    pub graph_nosleep_time: c_int,
    pub graph_noirqs: c_int,
    pub graph_verbose: c_int,
    pub graph_thresh: c_int,
    pub graph_tail: c_int,
}

#[repr(C)]
pub struct filter_entry {
    pub list: list_head,
    pub name: [c_char; 0],
}

pub const NUM_BUCKET: c_int = 22; /* 20 + 2 (for outliers in both direction) */

// C conditional: #ifdef HAVE_BPF_SKEL
#[cfg(HAVE_BPF_SKEL)]
extern "C" {
    pub fn perf_ftrace__latency_prepare_bpf(ftrace: *mut perf_ftrace) -> c_int;
    pub fn perf_ftrace__latency_start_bpf(ftrace: *mut perf_ftrace) -> c_int;
    pub fn perf_ftrace__latency_stop_bpf(ftrace: *mut perf_ftrace) -> c_int;
    pub fn perf_ftrace__latency_read_bpf(
        ftrace: *mut perf_ftrace,
        buckets: *mut c_int,
        stats: *mut stats,
    ) -> c_int;
    pub fn perf_ftrace__latency_cleanup_bpf(ftrace: *mut perf_ftrace) -> c_int;
}

// C conditional: #else  /* !HAVE_BPF_SKEL */
#[cfg(not(HAVE_BPF_SKEL))]
#[inline]
pub unsafe fn perf_ftrace__latency_prepare_bpf(_ftrace: *mut perf_ftrace) -> c_int {
    -1
}

#[cfg(not(HAVE_BPF_SKEL))]
#[inline]
pub unsafe fn perf_ftrace__latency_start_bpf(_ftrace: *mut perf_ftrace) -> c_int {
    -1
}

#[cfg(not(HAVE_BPF_SKEL))]
#[inline]
pub unsafe fn perf_ftrace__latency_stop_bpf(_ftrace: *mut perf_ftrace) -> c_int {
    -1
}

#[cfg(not(HAVE_BPF_SKEL))]
#[inline]
pub unsafe fn perf_ftrace__latency_read_bpf(
    _ftrace: *mut perf_ftrace,
    _buckets: *mut c_int,
    _stats: *mut stats,
) -> c_int {
    -1
}

#[cfg(not(HAVE_BPF_SKEL))]
#[inline]
pub unsafe fn perf_ftrace__latency_cleanup_bpf(_ftrace: *mut perf_ftrace) -> c_int {
    -1
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
