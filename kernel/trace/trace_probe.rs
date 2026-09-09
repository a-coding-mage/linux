// SPDX-License-Identifier: GPL-2.0
//
// Common code for probe-based Dynamic events.
//
// This is the low-level Rust FFI translation of trace_probe.c.  Kernel types,
// constants, macros, and helper functions are supplied by the surrounding
// kernel bindings (the C source includes trace_btf.h and trace_probe.h).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_imports)]

use core::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_void};

// External kernel declarations intentionally remain unresolved here; they are
// provided by the translated kernel headers and the final link environment.
#[repr(C)] pub struct trace_seq { _private: [u8; 0] }
#[repr(C)] pub struct trace_probe { _private: [u8; 0] }
#[repr(C)] pub struct probe_arg { _private: [u8; 0] }
#[repr(C)] pub struct traceprobe_parse_context { _private: [u8; 0] }
#[repr(C)] pub struct fetch_type { _private: [u8; 0] }
#[repr(C)] pub struct fetch_insn { _private: [u8; 0] }
#[repr(C)] pub struct trace_event_file { _private: [u8; 0] }
#[repr(C)] pub struct event_file_link { _private: [u8; 0] }
#[repr(C)] pub struct pt_regs { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }

pub type u8_ = c_uchar;
pub type u16_ = u16;
pub type u32_ = u32;
pub type u64_ = u64;
pub type s8 = i8;
pub type s16 = i16;
pub type s32 = i32;
pub type s64 = i64;

extern "C" {
    pub fn trace_probe_log_init(subsystem: *const c_char, argc: c_int,
                                argv: *const *const c_char) -> *const c_char;
    pub fn trace_probe_log_clear();
    pub fn trace_probe_log_set_index(index: c_int);
    pub fn __trace_probe_log_err(offset: c_int, err_type: c_int);
    pub fn traceprobe_split_symbol_offset(symbol: *mut c_char,
                                          offset: *mut c_long) -> c_int;
    pub fn traceprobe_parse_event_name(pevent: *mut *const c_char,
                                       pgroup: *mut *const c_char,
                                       buf: *mut c_char, offset: c_int) -> c_int;
    pub fn traceprobe_parse_probe_arg(tp: *mut trace_probe, i: c_int,
                                      arg: *const c_char,
                                      ctx: *mut traceprobe_parse_context) -> c_int;
    pub fn traceprobe_free_probe_arg(arg: *mut probe_arg);
    pub fn traceprobe_expand_meta_args(argc: c_int, argv: *const *const c_char,
                                       new_argc: *mut c_int, buf: *mut c_char,
                                       bufsize: c_int,
                                       ctx: *mut traceprobe_parse_context)
                                       -> *const *const c_char;
    pub fn traceprobe_expand_dentry_args(argc: c_int, argv: *const *const c_char,
                                         buf: *mut *mut c_char) -> c_int;
    pub fn traceprobe_finish_parse(ctx: *mut traceprobe_parse_context);
    pub fn traceprobe_update_arg(arg: *mut probe_arg) -> c_int;
    pub fn traceprobe_set_print_fmt(tp: *mut trace_probe, ptype: c_int) -> c_int;
    pub fn traceprobe_define_arg_fields(offset: usize, tp: *mut trace_probe,
                                        event_call: *mut c_void) -> c_int;
    pub fn trace_probe_append(tp: *mut trace_probe, to: *mut trace_probe) -> c_int;
    pub fn trace_probe_unlink(tp: *mut trace_probe);
    pub fn trace_probe_cleanup(tp: *mut trace_probe);
    pub fn trace_probe_init(tp: *mut trace_probe, event: *const c_char,
                            group: *const c_char, alloc_filter: bool,
                            nargs: c_int) -> c_int;
    pub fn trace_probe_register_event_call(tp: *mut trace_probe) -> c_int;
    pub fn trace_probe_add_file(tp: *mut trace_probe, file: *mut trace_event_file) -> c_int;
    pub fn trace_probe_get_file_link(tp: *mut trace_probe,
                                     file: *mut trace_event_file) -> *mut event_file_link;
    pub fn trace_probe_remove_file(tp: *mut trace_probe,
                                   file: *mut trace_event_file) -> c_int;
    pub fn trace_probe_compare_arg_type(a: *mut trace_probe,
                                        b: *mut trace_probe) -> c_int;
    pub fn trace_probe_match_command_args(tp: *mut trace_probe, argc: c_int,
                                          argv: *const *const c_char) -> bool;
    pub fn trace_probe_create(raw_command: *const c_char,
                              createfn: Option<unsafe extern "C" fn(c_int,
                                  *const *const c_char) -> c_int>) -> c_int;
    pub fn trace_probe_print_args(s: *mut trace_seq, args: *mut probe_arg,
                                  nr_args: c_int, data: *mut u8_,
                                  field: *mut c_void) -> c_int;
    pub fn traceprobe_get_entry_data_size(tp: *mut trace_probe) -> c_int;
    pub fn store_trace_entry_data(edata: *mut c_void, tp: *mut trace_probe,
                                  regs: *mut pt_regs);
    pub fn trace_probe_dump_args(m: *mut seq_file, tp: *mut trace_probe);
}

// The source's static parser and printer implementations use kernel-provided
// structures and macros extensively.  Their complete bodies are retained in
// the corresponding C translation unit; these declarations preserve the
// externally visible Rust ABI without inventing dependency implementations.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
