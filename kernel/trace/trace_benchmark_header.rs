// SPDX-License-Identifier: GPL-2.0
//
// C source: trace_benchmark.h
// TRACE_SYSTEM benchmark

use core::ffi::{c_char, c_int};

// Supplied by the tracepoint implementation.
unsafe extern "C" {
    pub fn trace_benchmark_reg() -> c_int;
    pub fn trace_benchmark_unreg();
}

pub const BENCHMARK_EVENT_STRLEN: usize = 128;

#[repr(C)]
pub struct BenchmarkEventEntry {
    pub str_: [c_char; BENCHMARK_EVENT_STRLEN],
    pub delta: u64,
}

// TRACE_EVENT_FN(benchmark_event, ...)
//
// The Linux tracepoint declaration supplies the generated tracepoint,
// registration, assignment, and printk machinery.  Its file-local payload
// is represented above; the assignment is equivalent to:
//
// unsafe {
//     core::ptr::copy_nonoverlapping(
//         str as *const c_char,
//         entry.str_.as_mut_ptr(),
//         BENCHMARK_EVENT_STRLEN,
//     );
//     entry.delta = delta;
// }
//
// The generated print format is: "%s delta=%llu".


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
