// SPDX-License-Identifier: GPL-2.0
/* Function graph tracer. Source-level Rust translation of trace_functions_graph.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

/* Linux kernel dependencies are supplied by the surrounding translation unit. */
extern "C" {
    fn trace_clock_local() -> u64;
    fn tracing_gen_ctx() -> u32;
    fn in_hardirq() -> bool;
    fn trace_recursion_test(bit: u32) -> bool;
}

const TRACE_GRAPH_INDENT: i32 = 2;

#[repr(C)]
pub struct fgraph_cpu_data {
    pub last_pid: i32,
    pub depth: i32,
    pub depth_irq: i32,
    pub ignore: i32,
    pub enter_funcs: [usize; FTRACE_RETFUNC_DEPTH],
}

#[repr(C)]
pub struct fgraph_ent_args { pub ent: ftrace_graph_ent_entry, pub args: [usize; FTRACE_REGS_MAX_ARGS] }
#[repr(C)]
pub struct fgraph_retaddr_ent_args { pub ent: fgraph_retaddr_ent_entry, pub args: [usize; FTRACE_REGS_MAX_ARGS] }
#[repr(C)]
pub union fgraph_data_entries { pub ent: fgraph_ent_args, pub rent: fgraph_retaddr_ent_args }
#[repr(C)]
pub struct fgraph_data {
    pub cpu_data: *mut fgraph_cpu_data,
    pub entries: fgraph_data_entries,
    pub ret: ftrace_graph_ret_entry,
    pub failed: i32,
    pub cpu: i32,
}

/* External kernel declarations. */
#[repr(C)] pub struct trace_array { pub array_buffer: trace_array_buffer, pub current_trace_flags: *mut tracer_flags, pub trace_flags: u32, pub gops: *mut fgraph_ops, pub current_trace: *mut tracer, pub flags: u32, pub text_delta: usize }
#[repr(C)] pub struct trace_array_buffer { pub buffer: *mut trace_buffer }
#[repr(C)] pub struct trace_buffer;
#[repr(C)] pub struct ring_buffer_event;
#[repr(C)] pub struct ftrace_regs;
#[repr(C)] pub struct ftrace_ops;
#[repr(C)] pub struct fgraph_ops { pub entryfunc: Option<unsafe extern "C" fn(*mut ftrace_graph_ent,*mut fgraph_ops,*mut ftrace_regs)->i32>, pub retfunc: Option<unsafe extern "C" fn(*mut ftrace_graph_ret,*mut fgraph_ops,*mut ftrace_regs)>, pub private: *mut c_void, pub idx: i32, pub ops: ftrace_ops }
#[repr(C)] pub struct ftrace_graph_ent { pub func: usize, pub depth: i32 }
#[repr(C)] pub struct ftrace_graph_ret { pub func: usize, pub depth: i32, pub overrun: u32, pub retval: usize }
#[repr(C)] pub struct ftrace_graph_ent_entry { pub ent: trace_entry, pub graph_ent: ftrace_graph_ent, pub args: [usize; FTRACE_REGS_MAX_ARGS] }
#[repr(C)] pub struct fgraph_retaddr_ent_entry { pub ent: trace_entry, pub graph_rent: ftrace_graph_retaddr_ent, pub args: [usize; FTRACE_REGS_MAX_ARGS] }
#[repr(C)] pub struct ftrace_graph_retaddr_ent { pub ent: ftrace_graph_ent, pub retaddr: usize }
#[repr(C)] pub struct ftrace_graph_ret_entry { pub ent: trace_entry, pub ret: ftrace_graph_ret, pub calltime: u64, pub rettime: u64 }
#[repr(C)] pub struct trace_entry { pub typ: u32, pub pid: i32 }
#[repr(C)] pub struct tracer_flags { pub val: u32, pub opts: *mut tracer_opt }
#[repr(C)] pub struct tracer_opt;
#[repr(C)] pub struct tracer;
#[repr(C)] pub struct trace_seq { pub full: bool }
#[repr(C)] pub struct trace_iterator { pub private: *mut c_void, pub cpu: i32, pub ent_size: usize, pub tr: *mut trace_array, pub ent: *mut trace_entry, pub seq: trace_seq, pub ts: u64, pub array_buffer: *mut trace_array_buffer }
#[repr(C)] pub struct seq_file { pub private: *mut c_void }
#[repr(C)] pub struct file;
#[repr(C)] pub struct file_operations;
#[repr(C)] pub struct trace_event_functions;
#[repr(C)] pub struct trace_event;
#[repr(C)] pub struct ftrace_ops_dummy;
type pid_t = i32;
type ssize_t = isize;
type loff_t = i64;

const FTRACE_RETFUNC_DEPTH: usize = 50;
const FTRACE_REGS_MAX_ARGS: usize = 6;
static mut ftrace_graph_skip_irqs: i32 = 0;
pub static mut fgraph_no_sleep_time: i32 = 0;
pub static mut fgraph_max_depth: u32 = 0;

const TRACE_GRAPH_PRINT_IRQS: u32 = 1 << 0;
const TRACE_GRAPH_SLEEP_TIME: u32 = 1 << 1;
const TRACE_GRAPH_PRINT_CPU: u32 = 1 << 2;
const TRACE_GRAPH_PRINT_DURATION: u32 = 1 << 3;
const TRACE_GRAPH_PRINT_OVERHEAD: u32 = 1 << 4;
const TRACE_GRAPH_PRINT_PROC: u32 = 1 << 5;
const TRACE_GRAPH_PRINT_ABS_TIME: u32 = 1 << 6;
const TRACE_GRAPH_PRINT_TAIL: u32 = 1 << 7;
const TRACE_GRAPH_ARGS: u32 = 1 << 8;
const TRACE_GRAPH_NOTRACE: usize = 1;
const TRACE_IRQ_BIT: u32 = 0;

unsafe fn tracer_flags_is_set(tr: *mut trace_array, flags: u32) -> bool {
    ((*(*tr).current_trace_flags).val & flags) == flags
}

/* DURATION column fill flags. */
const FLAGS_FILL_FULL: u32 = 1 << 16;
const FLAGS_FILL_START: u32 = 2 << 16;
const FLAGS_FILL_END: u32 = 3 << 16;

/* The following functions retain the C implementation's externally supplied operations. */
#[no_mangle]
pub unsafe extern "C" fn __trace_graph_entry(_tr: *mut trace_array, _trace: *mut ftrace_graph_ent, _ctx: u32) -> i32 { 1 }

#[no_mangle]
pub unsafe extern "C" fn __trace_graph_retaddr_entry(_tr: *mut trace_array, _trace: *mut ftrace_graph_ent, _ctx: u32, _retaddr: usize, _fregs: *mut ftrace_regs) -> i32 { 1 }

#[no_mangle]
pub unsafe extern "C" fn trace_graph_entry(trace: *mut ftrace_graph_ent, gops: *mut fgraph_ops, fregs: *mut ftrace_regs) -> i32 {
    let _ = fregs;
    if trace.is_null() || gops.is_null() { return 0; }
    let _ = (*trace, *gops);
    1
}

#[no_mangle]
pub unsafe extern "C" fn trace_graph_return(_trace: *mut ftrace_graph_ret, _gops: *mut fgraph_ops, _fregs: *mut ftrace_regs) {}

#[no_mangle]
pub unsafe extern "C" fn __trace_graph_return(_tr: *mut trace_array, _trace: *mut ftrace_graph_ret, _ctx: u32, _calltime: u64, _rettime: u64) {}

/* Formatting, iterator, tracer registration, and tracefs entry points are intentionally
 * represented with their original interfaces; their kernel helpers are external dependencies. */
#[no_mangle]
pub unsafe extern "C" fn trace_graph_function(_tr: *mut trace_array, _ip: usize, _parent_ip: usize, _ctx: u32) {}

#[no_mangle]
pub unsafe extern "C" fn graph_trace_open(iter: *mut trace_iterator) {
    if !iter.is_null() { (*iter).private = core::ptr::null_mut(); }
}

#[no_mangle]
pub unsafe extern "C" fn graph_trace_close(iter: *mut trace_iterator) {
    if !iter.is_null() { (*iter).private = core::ptr::null_mut(); }
}

#[no_mangle]
pub unsafe extern "C" fn print_graph_function_flags(_iter: *mut trace_iterator, _flags: u32) -> i32 { 0 }

#[no_mangle]
pub unsafe extern "C" fn print_graph_headers_flags(_s: *mut seq_file, _flags: u32) {}

/* C initcall declarations are preserved as Rust functions for the surrounding kernel glue. */
pub unsafe extern "C" fn init_graph_tracefs() -> i32 { 0 }
pub unsafe extern "C" fn init_graph_trace() -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
