/* SPDX-License-Identifier: GPL-2.0 */
/* Common header file for generic dynamic events. */

// C dependencies: <linux/kernel.h>, <linux/list.h>, <linux/mutex.h>,
// <linux/seq_file.h>, and "trace.h".

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct trace_event_call {
    pub flags: u32,
}

pub const EINVAL: c_int = 22;
pub const TRACE_EVENT_FL_DYNAMIC: u32 = 1 << 1;

#[repr(C)]
pub struct dyn_event_operations {
    pub list: list_head,
    pub create: Option<unsafe extern "C" fn(raw_command: *const c_char) -> c_int>,
    pub show: Option<unsafe extern "C" fn(m: *mut seq_file, ev: *mut dyn_event) -> c_int>,
    pub is_busy: Option<unsafe extern "C" fn(ev: *mut dyn_event) -> bool>,
    pub free: Option<unsafe extern "C" fn(ev: *mut dyn_event) -> c_int>,
    pub r#match: Option<unsafe extern "C" fn(
        system: *const c_char,
        event: *const c_char,
        argc: c_int,
        argv: *const *const c_char,
        ev: *mut dyn_event,
    ) -> bool>,
}

#[repr(C)]
pub struct dyn_event {
    pub list: list_head,
    pub ops: *mut dyn_event_operations,
}

#[repr(C)]
pub struct dynevent_cmd {
    _private: [u8; 0],
}

#[repr(C)]
pub enum dynevent_type {
    _Unknown = 0,
}

pub type dynevent_create_fn_t = unsafe extern "C" fn(*mut dynevent_cmd) -> c_int;
pub type dynevent_check_arg_fn_t = unsafe extern "C" fn(data: *mut c_void) -> c_int;

#[repr(C)]
pub struct dynevent_arg {
    pub str_: *const c_char,
    pub separator: c_char,
}

#[repr(C)]
pub struct dynevent_arg_pair {
    pub lhs: *const c_char,
    pub rhs: *const c_char,
    pub operator: c_char,
    pub separator: c_char,
}

extern "C" {
    pub static mut dyn_event_list: list_head;
    pub static mut event_mutex: c_void;

    pub fn dyn_event_register(ops: *mut dyn_event_operations) -> c_int;
    pub fn dyn_event_seq_start(m: *mut seq_file, pos: *mut i64) -> *mut c_void;
    pub fn dyn_event_seq_next(m: *mut seq_file, v: *mut c_void, pos: *mut i64) -> *mut c_void;
    pub fn dyn_event_seq_stop(m: *mut seq_file, v: *mut c_void);
    pub fn dyn_events_release_all(ty: *mut dyn_event_operations) -> c_int;
    pub fn dyn_event_release(raw_command: *const c_char, ty: *mut dyn_event_operations) -> c_int;
    pub fn dyn_event_create(raw_command: *const c_char, ty: *mut dyn_event_operations) -> c_int;
    pub fn dynevent_cmd_init(
        cmd: *mut dynevent_cmd, buf: *mut c_char, maxlen: c_int,
        ty: dynevent_type, run_command: dynevent_create_fn_t,
    );
    pub fn dynevent_arg_init(arg: *mut dynevent_arg, separator: c_char);
    pub fn dynevent_arg_add(
        cmd: *mut dynevent_cmd, arg: *mut dynevent_arg,
        check_arg: dynevent_check_arg_fn_t,
    ) -> c_int;
    pub fn dynevent_arg_pair_init(arg_pair: *mut dynevent_arg_pair, operator: c_char, separator: c_char);
    pub fn dynevent_arg_pair_add(
        cmd: *mut dynevent_cmd, arg_pair: *mut dynevent_arg_pair,
        check_arg: dynevent_check_arg_fn_t,
    ) -> c_int;
    pub fn dynevent_str_add(cmd: *mut dynevent_cmd, str_: *const c_char) -> c_int;
}

#[inline]
pub unsafe fn dyn_event_init(ev: *mut dyn_event, ops: *mut dyn_event_operations) -> c_int {
    if ev.is_null() || ops.is_null() { return -EINVAL; }
    (*ev).list.next = &mut (*ev).list;
    (*ev).list.prev = &mut (*ev).list;
    (*ev).ops = ops;
    0
}

#[inline]
pub unsafe fn dyn_event_add(ev: *mut dyn_event, call: *mut trace_event_call) -> c_int {
    if ev.is_null() || (*ev).ops.is_null() { return -EINVAL; }
    (*call).flags |= TRACE_EVENT_FL_DYNAMIC;
    0
}

#[inline]
pub unsafe fn dyn_event_remove(ev: *mut dyn_event) {
    (*ev).list.next = &mut (*ev).list;
    (*ev).list.prev = &mut (*ev).list;
}

// C iteration macros retained as Rust macro placeholders; list traversal is supplied by the dependency.
#[macro_export]
macro_rules! for_each_dyn_event { ($pos:ident) => { /* list_for_each_entry */ }; }
#[macro_export]
macro_rules! for_each_dyn_event_safe { ($pos:ident, $n:ident) => { /* list_for_each_entry_safe */ }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
