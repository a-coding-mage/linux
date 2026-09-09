// SPDX-License-Identifier: GPL-2.0
//
// Rust FFI translation of trace.h.  Types supplied by the Linux headers are
// intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_void};

pub const TRACE_MODE_WRITE: u32 = 0o640;
pub const TRACE_MODE_READ: u32 = 0o440;
pub const TRACE_BUF_SIZE: usize = 1024;
pub const TRACE_FLAGS_MAX_SIZE: usize = 64;
pub const HIST_STACKTRACE_DEPTH: usize = 31;
pub const HIST_STACKTRACE_SKIP: usize = 5;
pub const SYSCALL_FAULT_USER_MAX: usize = 165;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum trace_type {
    __TRACE_FIRST_TYPE = 0,
    TRACE_FN, TRACE_CTX, TRACE_WAKE, TRACE_STACK, TRACE_PRINT, TRACE_BPRINT,
    TRACE_MMIO_RW, TRACE_MMIO_MAP, TRACE_BRANCH, TRACE_GRAPH_RET,
    TRACE_GRAPH_ENT, TRACE_GRAPH_RETADDR_ENT, TRACE_USER_STACK, TRACE_BLK,
    TRACE_BPUTS, TRACE_HWLAT, TRACE_OSNOISE, TRACE_TIMERLAT, TRACE_RAW_DATA,
    TRACE_FUNC_REPEATS, __TRACE_LAST_TYPE,
}

#[repr(C)]
pub struct trace_entry { _private: [u8; 0] }
#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)]
pub struct trace_array_cpu {
    pub disabled: c_int,
    pub entries: usize, pub saved_latency: usize, pub critical_start: usize,
    pub critical_end: usize, pub critical_sequence: usize, pub nice: usize,
    pub policy: usize, pub rt_priority: usize, pub skipped_entries: usize,
    pub preempt_timestamp: u64, pub pid: c_int, pub uid: c_int,
    pub comm: [c_char; 16],
    pub ignore_pid: bool,
}
#[repr(C)]
pub struct array_buffer {
    pub tr: *mut trace_array, pub buffer: *mut trace_buffer,
    pub data: *mut trace_array_cpu, pub time_start: u64, pub cpu: c_int,
}
#[repr(C)]
pub struct trace_options { pub tracer: *mut tracer, pub topts: *mut trace_option_dentry }
#[repr(C)]
pub struct trace_option_dentry {
    pub opt: *mut tracer_opt, pub flags: *mut tracer_flags,
    pub tr: *mut trace_array, pub entry: *mut dentry,
}
#[repr(C)]
pub struct tracer_opt { pub name: *const c_char, pub bit: u32 }
#[repr(C)]
pub struct tracer_flags { pub val: u32, pub opts: *mut tracer_opt, pub trace: *mut tracer }

#[repr(C)]
pub struct trace_array {
    pub list: list_head, pub name: *mut c_char, pub array_buffer: array_buffer,
    pub mapped: u32, pub range_addr_start: usize, pub range_addr_size: usize,
    pub range_name: *mut c_char, pub text_delta: c_long,
    pub module_delta: *mut trace_module_delta, pub scratch: *mut c_void,
    pub scratch_size: c_int, pub buffer_disabled: c_int,
    pub filtered_pids: *mut trace_pid_list, pub filtered_no_pids: *mut trace_pid_list,
    pub max_lock: [u8; 0], pub stop_count: c_int, pub clock_id: c_int,
    pub nr_topts: c_int, pub clear_trace: bool, pub buffer_percent: c_int,
    pub n_err_log_entries: u32, pub current_trace: *mut tracer,
    pub current_trace_flags: *mut tracer_flags, pub trace_flags: u64,
    pub trace_flags_index: [u8; TRACE_FLAGS_MAX_SIZE], pub flags: u32,
}
#[repr(C)] pub struct trace_module_delta { pub rcu: [u8; 0], pub delta: [c_long; 0] }
#[repr(C)] pub struct trace_pid_list { _private: [u8; 0] }
#[repr(C)] pub struct trace_buffer { _private: [u8; 0] }
#[repr(C)] pub struct ring_buffer_event { pub array: [u32; 0] }
#[repr(C)] pub struct trace_iterator { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct ftrace_regs { _private: [u8; 0] }
#[repr(C)] pub struct fgraph_ops { _private: [u8; 0] }
#[repr(C)] pub struct ftrace_graph_ent { pub func: usize, pub depth: c_int }
#[repr(C)] pub struct ftrace_graph_ret { pub depth: c_int }
#[repr(C)] pub struct ftrace_hash { pub size_bits: usize, pub buckets: *mut c_void, pub count: usize, pub flags: usize }

pub type cond_update_fn_t = unsafe extern "C" fn(*mut trace_array, *mut c_void) -> bool;
pub type ftrace_mapper_func = unsafe extern "C" fn(*mut c_void) -> c_int;

pub const TRACE_ARRAY_FL_GLOBAL: u32 = 1 << 0;
pub const TRACE_ARRAY_FL_BOOT: u32 = 1 << 1;
pub const TRACE_ARRAY_FL_LAST_BOOT: u32 = 1 << 2;
pub const TRACE_ARRAY_FL_MOD_INIT: u32 = 1 << 3;
pub const TRACE_ARRAY_FL_MEMMAP: u32 = 1 << 4;
pub const TRACE_ARRAY_FL_VMALLOC: u32 = 1 << 5;
pub const TRACE_ARRAY_FL_RDONLY: u32 = 1 << 6;
pub const TRACE_GRAPH_FL: usize = 1;
pub const TRACE_GRAPH_NOTRACE: usize = 1 << 3;
pub const FTRACE_HASH_FL_MOD: usize = 1;
pub const FILTER_PRED_INVALID: u16 = u16::MAX;
pub const FILTER_PRED_IS_RIGHT: u16 = 1 << 15;
pub const FILTER_PRED_FOLD: u16 = 1 << 15;
pub const MAX_FILTER_PRED: usize = 16384;
pub const MAX_EVENT_NAME_LEN: usize = 64;

#[repr(C)] pub struct cond_snapshot { pub cond_data: *mut c_void, pub update: cond_update_fn_t }
#[repr(C)] pub struct trace_func_repeats { pub ip: usize, pub parent_ip: usize, pub count: usize, pub ts_last_call: u64 }
#[repr(C)] pub struct trace_min_max_param { pub lock: *mut c_void, pub val: *mut u64, pub min: *mut u64, pub max: *mut u64 }

extern "C" {
    pub static mut ftrace_trace_arrays: list_head;
    pub fn trace_pid_list_alloc() -> *mut trace_pid_list;
    pub fn trace_pid_list_free(pid_list: *mut trace_pid_list);
    pub fn trace_pid_list_is_set(pid_list: *mut trace_pid_list, pid: u32) -> bool;
    pub fn trace_pid_list_set(pid_list: *mut trace_pid_list, pid: u32) -> c_int;
    pub fn trace_pid_list_clear(pid_list: *mut trace_pid_list, pid: u32) -> c_int;
    pub fn trace_array_get(tr: *mut trace_array) -> c_int;
    pub fn trace_array_find(instance: *const c_char) -> *mut trace_array;
    pub fn trace_array_find_get(instance: *const c_char) -> *mut trace_array;
    pub fn tracing_set_clock(tr: *mut trace_array, clockstr: *const c_char) -> c_int;
    pub fn trace_clock_in_ns(tr: *mut trace_array) -> bool;
    pub fn trace_adjust_address(tr: *mut trace_array, addr: usize) -> usize;
    pub fn tracer_tracing_is_on(tr: *mut trace_array) -> bool;
    pub fn tracer_tracing_on(tr: *mut trace_array);
    pub fn tracer_tracing_off(tr: *mut trace_array);
    pub fn register_tracer(tr: *mut tracer) -> c_int;
    pub fn tracing_is_enabled() -> c_int;
    pub fn tracing_reset_cpu(buf: *mut array_buffer, cpu: c_int);
    pub fn tracing_update_buffers(tr: *mut trace_array) -> c_int;
    pub fn trace_set_options(tr: *mut trace_array, option: *mut c_char) -> c_int;
    pub fn tracing_set_tracer(tr: *mut trace_array, buf: *const c_char) -> c_int;
    pub fn trace_event_init();
}

#[inline]
pub unsafe fn trace_array_is_readonly(tr: *const trace_array) -> bool { (*tr).flags & TRACE_ARRAY_FL_RDONLY != 0 }
#[inline]
pub unsafe fn ftrace_hash_empty(hash: *const ftrace_hash) -> bool { hash.is_null() || ((*hash).count == 0 && (*hash).flags & FTRACE_HASH_FL_MOD == 0) }
#[inline]
pub unsafe fn ftrace_graph_depth(task_var: *const usize) -> usize { (*task_var >> 1) & 3 }
#[inline]
pub unsafe fn ftrace_graph_set_depth(task_var: *mut usize, depth: c_int) { *task_var = (*task_var & !(3 << 1)) | ((depth as usize & 3) << 1); }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
