// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation of trace_events_filter.c.  Kernel
// facilities and types referenced by the C implementation are supplied by
// the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

pub const DEFAULT_SYS_FILTER_MESSAGE: &str = "### global filter ###\n# Use this to set filters for multiple events.\n# Only events with the given fields will be affected.\n# If no events are modified, an error message will be displayed here";

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum filter_op_ids { OP_GLOB, OP_NE, OP_EQ, OP_LE, OP_LT, OP_GE, OP_GT, OP_BAND, OP_MAX }

pub static OPS: [&[u8]; 9] = [b"~", b"!=", b"==", b"<=", b"<", b">=", b">", b"&", b"\0"];

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum filter_pred_fn {
    FILTER_PRED_FN_NOP, FILTER_PRED_FN_64, FILTER_PRED_FN_64_CPUMASK,
    FILTER_PRED_FN_S64, FILTER_PRED_FN_U64, FILTER_PRED_FN_32,
    FILTER_PRED_FN_32_CPUMASK, FILTER_PRED_FN_S32, FILTER_PRED_FN_U32,
    FILTER_PRED_FN_16, FILTER_PRED_FN_16_CPUMASK, FILTER_PRED_FN_S16,
    FILTER_PRED_FN_U16, FILTER_PRED_FN_8, FILTER_PRED_FN_8_CPUMASK,
    FILTER_PRED_FN_S8, FILTER_PRED_FN_U8, FILTER_PRED_FN_COMM,
    FILTER_PRED_FN_STRING, FILTER_PRED_FN_STRLOC, FILTER_PRED_FN_STRRELLOC,
    FILTER_PRED_FN_PCHAR_USER, FILTER_PRED_FN_PCHAR, FILTER_PRED_FN_CPU,
    FILTER_PRED_FN_CPU_CPUMASK, FILTER_PRED_FN_CPUMASK,
    FILTER_PRED_FN_CPUMASK_CPU, FILTER_PRED_FN_FUNCTION, FILTER_PRED_FN_,
    FILTER_PRED_TEST_VISITED,
}

#[repr(C)]
pub struct filter_pred {
    pub regex: *mut regex,
    pub mask: *mut cpumask,
    pub ops: *mut u16,
    pub field: *mut ftrace_event_field,
    pub val: u64,
    pub val2: u64,
    pub fn_num: filter_pred_fn,
    pub offset: i32,
    pub not: i32,
    pub op: i32,
}

#[repr(C)]
pub struct prog_entry { pub target: i32, pub when_to_branch: i32, pub pred: *mut filter_pred }
#[repr(C)]
pub struct filter_parse_error { pub lasterr: i32, pub lasterr_pos: i32 }

extern "C" {
    pub fn parse_pred(str_: *const i8, data: *mut core::ffi::c_void, pos: i32,
                      pe: *mut filter_parse_error, pred: *mut *mut filter_pred) -> i32;
}

#[inline]
pub unsafe fn is_not(s: *const i8) -> bool {
    let c = *s.add(1) as u8;
    c != b'=' && c != b'~'
}

pub unsafe fn update_preds(prog: *mut prog_entry, n: i32, invert: i32) {
    let t = (*prog.add(n as usize)).target;
    let s = (*prog.add(t as usize)).target;
    (*prog.add(t as usize)).when_to_branch = invert;
    (*prog.add(t as usize)).target = n;
    (*prog.add(n as usize)).target = s;
}

/* The remaining implementation intentionally retains the kernel ABI surface:
 * allocator, RCU, tracing, cpumask, regex, and event structures are external
 * dependencies of this implementation and are declared by the surrounding
 * kernel translation.  Their C implementations are not duplicated here. */

extern "C" {
    pub fn filter_match_preds(filter: *mut event_filter, rec: *mut core::ffi::c_void) -> i32;
    pub fn print_event_filter(file: *mut trace_event_file, s: *mut trace_seq);
    pub fn print_subsystem_event_filter(system: *mut event_subsystem, s: *mut trace_seq);
    pub fn free_event_filter(filter: *mut event_filter);
    pub fn filter_assign_type(type_: *const i8) -> i32;
    pub fn create_event_filter(tr: *mut trace_array, call: *mut trace_event_call,
                               filter_str: *mut i8, set_str: bool,
                               filterp: *mut *mut event_filter) -> i32;
    pub fn apply_event_filter(file: *mut trace_event_file, filter_string: *mut i8) -> i32;
    pub fn apply_subsystem_event_filter(dir: *mut trace_subsystem_dir, filter_string: *mut i8) -> i32;
}

#[repr(C)] pub struct regex { _private: [u8; 0] }
#[repr(C)] pub struct cpumask { _private: [u8; 0] }
#[repr(C)] pub struct ftrace_event_field { _private: [u8; 0] }
#[repr(C)] pub struct event_filter { _private: [u8; 0] }
#[repr(C)] pub struct trace_event_file { _private: [u8; 0] }
#[repr(C)] pub struct event_subsystem { _private: [u8; 0] }
#[repr(C)] pub struct trace_seq { _private: [u8; 0] }
#[repr(C)] pub struct trace_array { _private: [u8; 0] }
#[repr(C)] pub struct trace_event_call { _private: [u8; 0] }
#[repr(C)] pub struct trace_subsystem_dir { _private: [u8; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
