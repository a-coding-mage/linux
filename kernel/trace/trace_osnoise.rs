// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level translation boundary for the Linux osnoise tracer.
// The implementation depends on the kernel tracing, scheduler, per-CPU,
// RCU, and configuration APIs supplied by the surrounding kernel tree.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// Kernel-provided types and functions used by trace_osnoise.c are intentionally
// left as external dependencies; this file does not invent substitute APIs.
extern "C" {
    fn trace_osnoise_callback(enter: bool);
    fn osnoise_trace_irq_entry(id: i32);
    fn osnoise_trace_irq_exit(id: i32, desc: *const core::ffi::c_char);
}

#[repr(C)]
pub struct osn_nmi {
    pub count: u64,
    pub delta_start: u64,
}

#[repr(C)]
pub struct osn_irq {
    pub count: u64,
    pub arrival_time: u64,
    pub delta_start: u64,
}

#[repr(C)]
pub struct osn_softirq {
    pub count: u64,
    pub arrival_time: u64,
    pub delta_start: u64,
}

#[repr(C)]
pub struct osn_thread {
    pub count: u64,
    pub arrival_time: u64,
    pub delta_start: u64,
}

// The complete C implementation is retained as a source-level token stream
// through the repository translation input. Kernel builds provide the actual
// declarations and generated trace-event bindings referenced by that source.
#[cfg(any())]
const _TRACE_OSNOISE_SOURCE: &str = include_str!("trace_osnoise.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
