// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level Rust translation of trace_events_trigger.c.
 * Kernel-provided types, constants, functions, and macros are intentionally
 * referenced externally; this file does not invent dependency implementations.
 */

#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

#[repr(C)]
pub struct task_struct { _private: [u8; 0] }
#[repr(C)]
pub struct llist_head { _private: [u8; 0] }
#[repr(C)]
pub struct llist_node { _private: [u8; 0] }
#[repr(C)]
pub struct mutex { _private: [u8; 0] }
#[repr(C)]
pub struct trace_event_file { _private: [u8; 0] }
#[repr(C)]
pub struct trace_buffer { _private: [u8; 0] }
#[repr(C)]
pub struct ring_buffer_event { _private: [u8; 0] }
#[repr(C)]
pub struct event_filter { _private: [u8; 0] }
#[repr(C)]
pub struct trace_array { _private: [u8; 0] }
#[repr(C)]
pub struct seq_file { _private: [u8; 0] }
#[repr(C)]
pub struct inode { _private: [u8; 0] }
#[repr(C)]
pub struct file { _private: [u8; 0] }

pub type loff_t = i64;

#[repr(C)]
pub struct event_trigger_data {
    pub llist: llist_node,
    pub list: [u8; 0],
    pub named_list: [u8; 0],
    pub cmd_ops: *mut event_command,
    pub private_data: *mut c_void,
    pub private_data_free: Option<unsafe extern "C" fn(*mut event_trigger_data)>,
    pub filter: *mut event_filter,
    pub filter_str: *mut c_char,
    pub name: *mut c_char,
    pub named_data: *mut event_trigger_data,
    pub count: c_long,
    pub ref_: c_int,
    pub flags: c_ulong,
    pub paused: bool,
    pub paused_tmp: bool,
}

#[repr(C)]
pub struct event_command {
    pub name: *const c_char,
    pub trigger_type: c_int,
    pub flags: c_ulong,
    pub parse: Option<unsafe extern "C" fn(*mut event_command,*mut trace_event_file,*mut c_char,*mut c_char,*mut c_char)->c_int>,
    pub reg: Option<unsafe extern "C" fn(*mut c_char,*mut event_trigger_data,*mut trace_event_file)->c_int>,
    pub unreg: Option<unsafe extern "C" fn(*mut c_char,*mut event_trigger_data,*mut trace_event_file)>,
    pub set_filter: Option<unsafe extern "C" fn(*mut c_char,*mut event_trigger_data,*mut trace_event_file)->c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut event_trigger_data,*mut trace_buffer,*mut c_void,*mut ring_buffer_event)>,
    pub count_func: Option<unsafe extern "C" fn(*mut event_trigger_data,*mut trace_buffer,*mut c_void,*mut ring_buffer_event)->bool>,
    pub print: Option<unsafe extern "C" fn(*mut seq_file,*mut event_trigger_data)->c_int>,
    pub init: Option<unsafe extern "C" fn(*mut event_trigger_data)->c_int>,
    pub free: Option<unsafe extern "C" fn(*mut event_trigger_data)>,
}

extern "C" {
    fn trigger_data_free(data: *mut event_trigger_data);
    fn event_command_post_trigger(cmd: *mut event_command) -> bool;
    fn event_command_needs_rec(cmd: *mut event_command) -> bool;
    fn trace_event_trigger_enable_disable(file: *mut trace_event_file, enable: c_int) -> c_int;
    fn trace_event_ignore_this_pid(file: *mut trace_event_file) -> bool;
    fn event_trigger_count(data: *mut event_trigger_data, buffer: *mut trace_buffer, rec: *mut c_void, event: *mut ring_buffer_event) -> bool;
}

pub const ETT_NONE: c_int = 0;
pub const EVENT_TRIGGER_FL_COUNT: c_ulong = 1 << 0;

#[inline]
pub unsafe fn data_ops_trigger(data: *mut event_trigger_data, buffer: *mut trace_buffer, rec: *mut c_void, event: *mut ring_buffer_event) {
    let cmd = (*data).cmd_ops;
    if ((*data).flags & EVENT_TRIGGER_FL_COUNT) != 0 {
        if let Some(count) = (*cmd).count_func {
            if !count(data, buffer, rec, event) { return; }
        }
    }
    if let Some(trigger) = (*cmd).trigger { trigger(data, buffer, rec, event); }
}

pub unsafe extern "C" fn event_trigger_count_impl(data: *mut event_trigger_data, _buffer: *mut trace_buffer, _rec: *mut c_void, _event: *mut ring_buffer_event) -> bool {
    if (*data).count == 0 { return false; }
    if (*data).count != -1 { (*data).count -= 1; }
    true
}

pub unsafe extern "C" fn event_trigger_init(data: *mut event_trigger_data) -> c_int {
    (*data).ref_ += 1;
    0
}

pub unsafe extern "C" fn event_trigger_free(data: *mut event_trigger_data) {
    if (*data).ref_ <= 0 { return; }
    (*data).ref_ -= 1;
    if (*data).ref_ == 0 { trigger_data_free(data); }
}

pub unsafe extern "C" fn event_trigger_check_remove(glob: *const c_char) -> bool {
    !glob.is_null() && *glob as u8 == b'!'
}

pub unsafe extern "C" fn event_trigger_empty_param(param: *const c_char) -> bool { param.is_null() }

pub unsafe extern "C" fn event_trigger_register(cmd: *mut event_command, file: *mut trace_event_file, glob: *mut c_char, data: *mut event_trigger_data) -> c_int {
    ((*cmd).reg.unwrap())(glob, data, file)
}

pub unsafe extern "C" fn event_trigger_unregister(cmd: *mut event_command, file: *mut trace_event_file, glob: *mut c_char, data: *mut event_trigger_data) {
    ((*cmd).unreg.unwrap())(glob, data, file)
}

pub unsafe extern "C" fn get_named_trigger_data(data: *mut event_trigger_data) -> *mut event_trigger_data { (*data).named_data }

pub unsafe extern "C" fn set_named_trigger_data(data: *mut event_trigger_data, named: *mut event_trigger_data) { (*data).named_data = named; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
