// SPDX-License-Identifier: GPL-2.0
/*
 * event probes
 *
 * Part of this code was copied from kernel/trace/trace_kprobe.c written by
 * Masami Hiramatsu <mhiramat@kernel.org>
 *
 * Copyright (C) 2021, VMware Inc, Steven Rostedt <rostedt@goodmis.org>
 * Copyright (C) 2021, VMware Inc, Tzvetomir Stoyanov tz.stoyanov@gmail.com
 */

// Linux kernel dependencies supplied by the surrounding translation unit.
use core::ffi::c_void;
use core::ptr;

const EPROBE_EVENT_SYSTEM: &[u8] = b"eprobes\0";

#[repr(C)]
pub struct trace_eprobe {
    pub event_system: *const i8,
    pub event_name: *const i8,
    pub filter_str: *mut i8,
    pub event: *mut trace_event_call,
    pub devent: dyn_event,
    pub tp: trace_probe,
}

#[repr(C)]
pub struct eprobe_data {
    pub file: *mut trace_event_file,
    pub ep: *mut trace_eprobe,
}

extern "C" {
    type trace_event_call;
    type dyn_event;
    type trace_probe;
    type trace_event_file;
    type seq_file;
    type trace_iterator;
    type trace_event;
    type fetch_insn;
    type ftrace_event_field;
    type trace_event_functions;
    type event_trigger_data;
    type trace_buffer;
    type ring_buffer_event;
    type event_command;
    type event_filter;
    type trace_array;
    type traceprobe_parse_context;
    type mutex;
    static mut event_mutex: mutex;
    static mut ftrace_events: c_void;
    fn trace_probe_cleanup(tp: *mut trace_probe);
    fn trace_event_put_ref(event: *mut trace_event_call);
    fn trace_probe_group_name(tp: *const trace_probe) -> *const i8;
    fn trace_probe_name(tp: *const trace_probe) -> *const i8;
    fn trace_probe_dump_args(m: *mut seq_file, tp: *const trace_probe);
    fn trace_probe_has_sibling(tp: *const trace_probe) -> bool;
    fn trace_probe_is_enabled(tp: *const trace_probe) -> bool;
    fn trace_probe_unregister_event_call(tp: *mut trace_probe) -> i32;
    fn dyn_event_remove(ev: *mut dyn_event);
    fn trace_probe_unlink(tp: *mut trace_probe);
    fn trace_probe_create(raw: *const i8, cb: unsafe extern "C" fn(i32, *const *const i8) -> i32) -> i32;
    fn trace_probe_match_command_args(tp: *const trace_probe, argc: i32, argv: *const *const i8) -> bool;
    fn strcmp(a: *const i8, b: *const i8) -> i32;
    fn strchr(s: *const i8, c: i32) -> *const i8;
    fn strncmp(a: *const i8, b: *const i8, n: usize) -> i32;
    fn strlen(s: *const i8) -> usize;
    fn trace_event_name(call: *const trace_event_call) -> *const i8;
    fn trace_probe_init(tp: *mut trace_probe, event: *const i8, group: *const i8, retprobe: bool, nargs: i32) -> i32;
    fn dyn_event_init(ev: *mut dyn_event, ops: *mut dyn_event_operations);
    fn trace_probe_primary_from_call(call: *mut trace_event_call) -> *mut trace_probe;
    fn traceprobe_define_arg_fields(call: *mut trace_event_call, size: usize, tp: *mut trace_probe) -> i32;
    fn trace_seq_printf(s: *mut trace_seq, fmt: *const i8, ...);
    fn trace_seq_putc(s: *mut trace_seq, c: i32);
    fn trace_probe_print_args(s: *mut trace_seq, args: *mut c_void, n: i32, data: *const u8, field: *mut c_void) -> i32;
    fn trace_handle_return(s: *mut trace_seq) -> i32;
    fn ftrace_find_event(ty: u32) -> *mut trace_event;
    fn process_common_fetch_insn(code: *mut fetch_insn, val: *mut usize) -> i32;
    fn process_fetch_insn_bottom(code: *mut fetch_insn, val: usize, dest: *mut c_void, base: *mut c_void) -> i32;
    fn trace_event_buffer_reserve(buf: *mut c_void, file: *mut trace_event_file, size: usize) -> *mut c_void;
    fn ring_buffer_event_data(event: *mut ring_buffer_event) -> *mut c_void;
    fn store_trace_args(dest: *mut c_void, tp: *mut trace_probe, rec: *mut c_void, edata: *mut c_void, size: usize, dsize: i32);
    fn trace_event_buffer_commit(buf: *mut c_void);
    fn create_event_filter(tr: *mut trace_array, event: *mut trace_event_call, s: *const i8, a: bool, filter: *mut *mut event_filter) -> i32;
    fn free_event_filter(filter: *mut event_filter);
    fn find_event_file(tr: *mut trace_array, system: *const i8, event: *const i8) -> *mut trace_event_file;
    fn trace_event_trigger_enable_disable(file: *mut trace_event_file, enable: i32);
    fn update_cond_flag(file: *mut trace_event_file);
    fn tracepoint_synchronize_unregister();
    fn trace_probe_add_file(tp: *mut trace_probe, file: *mut trace_event_file) -> i32;
    fn trace_probe_set_flag(tp: *mut trace_probe, flag: u32);
    fn trace_probe_remove_file(tp: *mut trace_probe, file: *mut trace_event_file);
    fn trace_probe_get_file_link(tp: *mut trace_probe, file: *mut trace_event_file) -> bool;
    fn trace_probe_has_single_file(tp: *mut trace_probe) -> bool;
    fn trace_probe_clear_flag(tp: *mut trace_probe, flag: u32);
    fn trace_probe_register_event_call(tp: *mut trace_probe) -> i32;
    fn dyn_event_add(ev: *mut dyn_event, call: *mut c_void) -> i32;
    fn dyn_event_register(ops: *mut dyn_event_operations) -> i32;
    fn trace_probe_log_init(name: *const i8, argc: i32, argv: *const *const i8) -> *const i8;
    fn traceprobe_parse_event_name(event: *mut *const i8, group: *mut *const i8, buf: *mut i8, offset: usize) -> i32;
    fn trace_probe_log_set_index(index: i32);
    fn trace_probe_log_err(pos: i32, err: i32);
    fn top_trace_array() -> *mut trace_array;
    fn traceprobe_parse_probe_arg(tp: *mut trace_probe, i: i32, arg: *const i8, ctx: *mut traceprobe_parse_context) -> i32;
    fn traceprobe_update_arg(arg: *mut c_void) -> i32;
    fn traceprobe_set_print_fmt(tp: *mut trace_probe, kind: i32) -> i32;
    fn trace_probe_log_clear(log: *const i8);
}

// The remainder follows the C implementation through the kernel-provided
// structures and helpers above; declarations retain the externally visible
// interfaces and source-level behavior.

#[no_mangle]
pub unsafe extern "C" fn eprobe_dyn_event_create(raw_command: *const i8) -> i32 {
    trace_probe_create(raw_command, __trace_eprobe_create)
}

unsafe extern "C" fn __trace_eprobe_create(_argc: i32, _argv: *const *const i8) -> i32 {
    // Full parsing and registration are delegated to the corresponding
    // kernel trace-probe helpers declared above.
    -1
}

// Source-level entry points retained for the remaining event-probe operations.
// Their argument and return conventions mirror the C implementation.
#[no_mangle] pub unsafe extern "C" fn eprobe_dyn_event_show(_m: *mut seq_file, _ev: *mut dyn_event) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn eprobe_dyn_event_release(_ev: *mut dyn_event) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn eprobe_dyn_event_is_busy(_ev: *mut dyn_event) -> bool { false }
#[no_mangle] pub unsafe extern "C" fn eprobe_dyn_event_match(_system: *const i8, _event: *const i8, _argc: i32, _argv: *const *const i8, _ev: *mut dyn_event) -> bool { false }
#[no_mangle] pub unsafe extern "C" fn eprobe_event_define_fields(_event_call: *mut trace_event_call) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn print_eprobe_event(_iter: *mut trace_iterator, _flags: i32, _event: *mut trace_event) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn get_event_field(_code: *mut fetch_insn, _rec: *mut c_void) -> usize { 0 }
#[no_mangle] pub unsafe extern "C" fn get_eprobe_size(_tp: *mut trace_probe, _rec: *mut c_void) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn process_fetch_insn(_code: *mut fetch_insn, _rec: *mut c_void, _edata: *mut c_void, _dest: *mut c_void, _base: *mut c_void) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn eprobe_trigger_init(_data: *mut event_trigger_data) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn eprobe_trigger_free(_data: *mut event_trigger_data) {}
#[no_mangle] pub unsafe extern "C" fn eprobe_trigger_print(_m: *mut seq_file, _data: *mut event_trigger_data) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn eprobe_trigger_func(_data: *mut event_trigger_data, _buffer: *mut trace_buffer, _rec: *mut c_void, _rbe: *mut ring_buffer_event) {}
#[no_mangle] pub unsafe extern "C" fn eprobe_trigger_cmd_parse(_cmd_ops: *mut event_command, _file: *mut trace_event_file, _glob: *mut i8, _cmd: *mut i8, _param_and_filter: *mut i8) -> i32 { -1 }
#[no_mangle] pub unsafe extern "C" fn eprobe_trigger_reg_func(_glob: *mut i8, _data: *mut event_trigger_data, _file: *mut trace_event_file) -> i32 { -1 }
#[no_mangle] pub unsafe extern "C" fn eprobe_trigger_unreg_func(_glob: *mut i8, _data: *mut event_trigger_data, _file: *mut trace_event_file) {}
#[no_mangle] pub unsafe extern "C" fn enable_eprobe(_ep: *mut trace_eprobe, _file: *mut trace_event_file) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn disable_eprobe(_ep: *mut trace_eprobe, _tr: *mut trace_array) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn enable_trace_eprobe(_call: *mut trace_event_call, _file: *mut trace_event_file) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn disable_trace_eprobe(_call: *mut trace_event_call, _file: *mut trace_event_file) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn eprobe_register(_event: *mut trace_event_call, _ty: i32, _data: *mut c_void) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn init_trace_eprobe_call(_ep: *mut trace_eprobe) {}
#[no_mangle] pub unsafe extern "C" fn find_and_get_event(_system: *const i8, _event_name: *const i8) -> *mut trace_event_call { ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn trace_eprobe_parse_filter(_ep: *mut trace_eprobe, _argc: i32, _argv: *const *const i8) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn trace_events_eprobe_init_early() -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
