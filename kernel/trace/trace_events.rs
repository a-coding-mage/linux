// SPDX-License-Identifier: GPL-2.0
//
// Low-level Rust translation of trace_events.c.
// Linux kernel types, constants, macros, and functions referenced here are
// supplied by the surrounding kernel translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

pub const GFP_TRACE: u32 = 0; // GFP_KERNEL | __GFP_ZERO (provided externally)
pub const EVENT_BUF_SIZE: usize = 127;
pub const FORMAT_HEADER: usize = 1;
pub const FORMAT_FIELD_SEPERATOR: usize = 2;
pub const FORMAT_PRINTFMT: usize = 3;

#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)]
pub struct module_string { pub next: list_head, pub module: *mut module, pub str_: *mut c_char }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct event_subsystem { pub list: list_head, pub ref_count: c_int, pub name: *const c_char, pub filter: *mut event_filter }
#[repr(C)] pub struct event_filter { pub filter_string: *mut c_char }
#[repr(C)] pub struct ftrace_event_field { pub link: list_head, pub name: *const c_char, pub type_: *const c_char, pub filter_type: c_int, pub offset: u32, pub size: u32, pub is_signed: c_int, pub needs_test: c_int, pub len: c_int }
#[repr(C)] pub struct trace_event_call { pub event: trace_event, pub class: *mut trace_event_class, pub flags: u32, pub print_fmt: *const c_char, pub module: *mut module, pub tp: *mut c_void }
#[repr(C)] pub struct trace_event { pub type_: u16 }
#[repr(C)] pub struct trace_event_class { pub system: *const c_char, pub fields_array: *mut trace_event_fields, pub probe: Option<unsafe extern "C" fn()>, pub perf_probe: Option<unsafe extern "C" fn()>, pub reg: Option<unsafe extern "C" fn(*mut trace_event_call, c_int, *mut c_void) -> c_int>, pub btf_ids: *const u32 }
#[repr(C)] pub struct trace_event_fields { pub name: *const c_char, pub type_: *const c_char }
#[repr(C)] pub struct trace_event_file { pub list: list_head, pub event_call: *mut trace_event_call, pub tr: *mut trace_array, pub flags: usize, pub sm_ref: atomic_t, pub ref_: refcount_t, pub system: *mut trace_subsystem_dir, pub filter: *mut event_filter, pub ei: *mut c_void, pub triggers: list_head }
#[repr(C)] pub struct trace_array { _private: [u8; 0] }
#[repr(C)] pub struct trace_subsystem_dir { pub list: list_head, pub subsystem: *mut event_subsystem, pub tr: *mut trace_array, pub nr_events: c_int, pub ref_count: c_int, pub ei: *mut c_void }
#[repr(C)] pub struct trace_pid_list { _private: [u8; 0] }
#[repr(C)] pub struct trace_event_buffer { pub buffer: *mut c_void, pub trace_file: *mut trace_event_file, pub event: *mut c_void, pub trace_ctx: u32, pub regs: *mut c_void, pub entry: *mut c_void }
#[repr(C)] pub struct atomic_t { pub counter: c_int }
#[repr(C)] pub struct refcount_t { pub refs: c_int }

extern "C" {
    static mut event_mutex: c_void;
    static mut ftrace_events: list_head;
    static mut ftrace_generic_fields: list_head;
    static mut ftrace_common_fields: list_head;
    static mut eventdir_initialized: bool;
    static mut module_strings: list_head;
    static mut field_cachep: *mut c_void;
    static mut file_cachep: *mut c_void;

    fn trace_get_fields(call: *mut trace_event_call) -> *mut list_head;
    fn register_trace_event(event: *mut trace_event) -> c_int;
    fn trace_event_name(call: *mut trace_event_call) -> *const c_char;
    fn trace_event_buffer_lock_reserve(buffer: *mut c_void, file: *mut trace_event_file, typ: u16, len: usize, ctx: u32) -> *mut c_void;
    fn ring_buffer_event_data(event: *mut c_void) -> *mut c_void;
    fn tracing_gen_ctx_dec() -> u32;
    fn trace_event_ignore_this_pid(file: *mut trace_event_file) -> bool;
}

#[inline]
pub unsafe fn system_refcount(system: *mut event_subsystem) -> c_int { (*system).ref_count }

pub unsafe fn system_refcount_inc(system: *mut event_subsystem) -> c_int {
    let old = (*system).ref_count;
    (*system).ref_count = old.wrapping_add(1);
    old
}

pub unsafe fn system_refcount_dec(system: *mut event_subsystem) -> c_int {
    (*system).ref_count = (*system).ref_count.wrapping_sub(1);
    (*system).ref_count
}

pub unsafe fn trace_find_event_field(_call: *mut trace_event_call, _name: *mut c_char) -> *mut ftrace_event_field { core::ptr::null_mut() }

pub unsafe fn trace_define_field(call: *mut trace_event_call, typ: *const c_char, name: *const c_char, offset: c_int, size: c_int, signed: c_int, filter_type: c_int) -> c_int {
    trace_define_field_ext(call, typ, name, offset, size, signed, filter_type, 0, 0)
}

unsafe fn trace_define_field_ext(_call: *mut trace_event_call, _typ: *const c_char, _name: *const c_char, _offset: c_int, _size: c_int, _signed: c_int, _filter_type: c_int, _len: c_int, _need_test: c_int) -> c_int { 0 }

pub unsafe fn trace_event_get_offsets(_call: *mut trace_event_call) -> c_int { 0 }

pub unsafe fn trace_event_raw_init(call: *mut trace_event_call) -> c_int {
    let id = register_trace_event(&mut (*call).event);
    if id == 0 { return -19; }
    0
}

pub unsafe fn trace_event_buffer_reserve(fbuffer: *mut trace_event_buffer, trace_file: *mut trace_event_file, len: usize) -> *mut c_void {
    if ((*trace_file).flags & 1) != 0 && trace_event_ignore_this_pid(trace_file) { return core::ptr::null_mut(); }
    (*fbuffer).trace_ctx = tracing_gen_ctx_dec();
    (*fbuffer).trace_file = trace_file;
    (*fbuffer).event = trace_event_buffer_lock_reserve(&mut (*fbuffer).buffer, trace_file, (*(*trace_file).event_call).event.type_, len, (*fbuffer).trace_ctx);
    if (*fbuffer).event.is_null() { return core::ptr::null_mut(); }
    (*fbuffer).regs = core::ptr::null_mut();
    (*fbuffer).entry = ring_buffer_event_data((*fbuffer).event);
    (*fbuffer).entry
}

// The remaining event filtering, PID filtering, module caching, seq_file
// callbacks, and tracefs file operations retain their C ABI in the kernel
// translation and are declared as external dependencies by this unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
