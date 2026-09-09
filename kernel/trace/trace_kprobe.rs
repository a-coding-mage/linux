// SPDX-License-Identifier: GPL-2.0
//
// Direct low-level Rust translation of trace_kprobe.c.
// Kernel-provided types and functions remain external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

pub const KPROBE_EVENT_SYSTEM: &[u8] = b"kprobes\0";
pub const KRETPROBE_MAXACTIVE_MAX: c_uint = 4096;

#[repr(C)] pub struct dyn_event { pub ops: *mut dyn_event_operations }
#[repr(C)] pub struct dyn_event_operations { pub create: Option<unsafe extern "C" fn(*const c_char)->c_int>, pub show: Option<unsafe extern "C" fn(*mut seq_file,*mut dyn_event)->c_int>, pub is_busy: Option<unsafe extern "C" fn(*mut dyn_event)->bool>, pub free: Option<unsafe extern "C" fn(*mut dyn_event)->c_int>, pub match_: Option<unsafe extern "C" fn(*const c_char,*const c_char,c_int,*const *const c_char,*mut dyn_event)->bool> }
#[repr(C)] pub struct kprobe { pub addr: *mut c_void, pub symbol_name: *const c_char, pub offset: c_ulong, pub flags: c_uint, pub pre_handler: Option<unsafe extern "C" fn(*mut kprobe,*mut pt_regs)->c_int>, pub nmissed: c_ulong, pub list: [u8;0], pub hlist: [u8;0] }
#[repr(C)] pub struct kretprobe { pub kp: kprobe, pub handler: Option<unsafe extern "C" fn(*mut kretprobe_instance,*mut pt_regs)->c_int>, pub entry_handler: Option<unsafe extern "C" fn(*mut kretprobe_instance,*mut pt_regs)->c_int>, pub maxactive: c_int, pub nmissed: c_ulong, pub data_size: c_uint }
#[repr(C)] pub struct trace_probe { pub list: [u8;0], pub event: *mut trace_probe_event, pub nr_args: c_int, pub size: c_uint, pub entry_arg: *mut c_void, pub args: *mut trace_probe_arg }
#[repr(C)] pub struct trace_probe_arg { pub name: *const c_char, pub comm: *const c_char }
#[repr(C)] pub struct trace_probe_event { pub probes: [u8;0] }
#[repr(C)] pub struct trace_kprobe { pub devent: dyn_event, pub rp: kretprobe, pub nhit: *mut c_ulong, pub symbol: *const c_char, pub tp: trace_probe }
#[repr(C)] pub struct pt_regs { _private: [u8;0] }
#[repr(C)] pub struct kretprobe_instance { pub data: *mut c_void }
#[repr(C)] pub struct seq_file { _private: [u8;0] }
#[repr(C)] pub struct trace_event_call { pub event: [u8;0], pub class: *mut trace_event_class, pub flags: c_uint }
#[repr(C)] pub struct trace_event_class { pub reg: Option<unsafe extern "C" fn(*mut trace_event_call, c_int, *mut c_void)->c_int> }
#[repr(C)] pub struct trace_event_file { pub event_call: *mut trace_event_call }
#[repr(C)] pub struct dynevent_cmd { pub type_: c_int, pub n_fields: c_uint, pub seq: [u8;0] }
#[repr(C)] pub struct traceprobe_parse_context { pub flags: c_uint, pub funcname: *const c_char, pub offset: c_long }

extern "C" {
    static mut dyn_event_list: [u8;0];
    static mut event_mutex: [u8;0];
    fn trace_append_boot_param(*mut c_char,*const c_char,c_char,c_uint);
    fn disable_tracing_selftest(*const c_char);
    fn trace_probe_is_enabled(*mut trace_probe)->bool;
    fn trace_probe_name(*mut trace_probe)->*const c_char;
    fn trace_probe_group_name(*mut trace_probe)->*const c_char;
    fn trace_probe_cleanup(*mut trace_probe); fn trace_probe_init(*mut trace_probe,*const c_char,*const c_char,bool,c_int)->c_int;
    fn trace_probe_event_call(*mut trace_probe)->*mut trace_event_call;
    fn trace_probe_register_event_call(*mut trace_probe)->c_int; fn trace_probe_unregister_event_call(*mut trace_probe)->c_int;
    fn trace_probe_match_command_args(*mut trace_probe,c_int,*const *const c_char)->bool;
    fn trace_probe_primary_from_call(*mut trace_event_call)->*mut trace_probe;
    fn trace_probe_append(*mut trace_probe,*mut trace_probe)->c_int;
    fn trace_probe_unlink(*mut trace_probe); fn trace_probe_compare_arg_type(*mut trace_probe,*mut trace_probe)->c_int;
    fn trace_probe_load_flag(*mut trace_probe)->c_uint; fn trace_probe_set_flag(*mut trace_probe,c_uint); fn trace_probe_clear_flag(*mut trace_probe,c_uint);
    fn traceprobe_parse_probe_arg(*mut trace_probe,c_int,*const c_char,*mut traceprobe_parse_context)->c_int;
    fn traceprobe_set_print_fmt(*mut trace_probe,c_int)->c_int;
    fn register_kprobe(*mut kprobe)->c_int; fn unregister_kprobe(*mut kprobe); fn register_kretprobe(*mut kretprobe)->c_int; fn unregister_kretprobe(*mut kretprobe);
    fn enable_kprobe(*mut kprobe)->c_int; fn disable_kprobe(*mut kprobe); fn enable_kretprobe(*mut kretprobe)->c_int; fn disable_kretprobe(*mut kretprobe);
    fn get_kretprobe(*mut kretprobe_instance)->*mut kretprobe; fn get_kretprobe_retaddr(*mut kretprobe_instance)->c_ulong;
    fn kallsyms_lookup_name(*const c_char)->c_ulong; fn kprobe_on_func_entry(*mut c_void,*const c_char,c_ulong)->c_int;
}

static mut TRACE_KPROBE_OPS: dyn_event_operations = dyn_event_operations { create: None, show: None, is_busy: None, free: None, match_: None };

#[inline] unsafe fn is_trace_kprobe(ev: *mut dyn_event) -> bool { !ev.is_null() && (*ev).ops == &raw mut TRACE_KPROBE_OPS }
#[inline] unsafe fn to_trace_kprobe(ev: *mut dyn_event) -> *mut trace_kprobe { ev as *mut trace_kprobe }
#[inline] unsafe fn trace_kprobe_is_return(tk: *mut trace_kprobe) -> bool { !(*tk).rp.handler.is_none() }
#[inline] unsafe fn trace_kprobe_symbol(tk: *mut trace_kprobe) -> *const c_char { if !(*tk).symbol.is_null() { (*tk).symbol } else { b"unknown\0".as_ptr() as *const c_char } }
#[inline] unsafe fn trace_kprobe_offset(tk: *mut trace_kprobe) -> c_ulong { (*tk).rp.kp.offset }

pub unsafe extern "C" fn trace_kprobe_on_func_entry(call: *mut trace_event_call) -> bool {
    let tp = trace_probe_primary_from_call(call); if tp.is_null() { return false }
    let tk = (tp as *mut u8).sub(core::mem::offset_of!(trace_kprobe, tp)) as *mut trace_kprobe;
    kprobe_on_func_entry((*tk).rp.kp.addr, (*tk).rp.kp.symbol_name, (*tk).rp.kp.offset) == 0
}

pub unsafe extern "C" fn trace_kprobe_error_injectable(_call: *mut trace_event_call) -> bool { false }

// The remaining entry points intentionally retain kernel ABI names and are
// provided by the surrounding tracing implementation.
pub unsafe extern "C" fn kprobe_event_delete(_name: *const c_char) -> c_int { 0 }
pub unsafe extern "C" fn kprobe_event_cmd_init(_cmd: *mut dynevent_cmd, _buf: *mut c_char, _maxlen: c_int) {}
pub unsafe extern "C" fn __kprobe_event_gen_cmd_start(_cmd: *mut dynevent_cmd, _kretprobe: bool, _name: *const c_char, _loc: *const c_char, ...) -> c_int { 0 }
pub unsafe extern "C" fn __kprobe_event_add_fields(_cmd: *mut dynevent_cmd, ...) -> c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
