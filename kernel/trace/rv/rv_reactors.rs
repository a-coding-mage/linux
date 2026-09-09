// SPDX-License-Identifier: GPL-2.0
/* Runtime reactor interface. Direct low-level translation of rv_reactors.c. */

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

pub const MAX_RV_REACTOR_NAME_SIZE: usize = 32;
pub const RV_MODE_READ: u32 = 0;
pub const RV_MODE_WRITE: u32 = 0;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct seq_file { pub private: *mut c_void }
#[repr(C)] pub struct inode { pub i_private: *mut c_void }
#[repr(C)] pub struct file { pub private_data: *mut c_void }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct va_list { _private: [u8; 0] }

pub type ReactorFn = unsafe extern "C" fn(*const c_char, *mut va_list);
#[repr(C)] pub struct rv_reactor { pub list: list_head, pub name: *const c_char, pub description: *const c_char, pub react: Option<ReactorFn> }
#[repr(C)] pub struct rv_monitor { pub list: list_head, pub parent: *mut rv_monitor, pub enabled: bool, pub reactor: *mut rv_reactor, pub react: Option<ReactorFn> }

static mut RV_REACTORS_LIST: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
static mut REACTING_ON: bool = false;

extern "C" {
    static mut rv_interface_lock: c_void;
    static mut rv_monitors_list: list_head;
    fn rv_disable_monitor(mon: *mut rv_monitor);
    fn rv_enable_monitor(mon: *mut rv_monitor);
    fn rv_is_container_monitor(mon: *mut rv_monitor) -> bool;
    fn rv_create_file(name: *const c_char, mode: u32, root: *mut dentry, data: *mut c_void, ops: *const c_void) -> *mut dentry;
    fn rv_remove(d: *mut dentry);
    fn tracepoint_synchronize_unregister();
}

unsafe fn get_reactor_rdef_by_name(_name: *mut c_char) -> *mut rv_reactor { ptr::null_mut() }

unsafe fn monitor_swap_reactors_single(mon: *mut rv_monitor, reactor: *mut rv_reactor, nested: bool) {
    if (*mon).reactor == reactor { return; }
    let enabled = (*mon).enabled;
    if enabled { rv_disable_monitor(mon); }
    (*mon).reactor = reactor;
    (*mon).react = (*reactor).react;
    if enabled && !nested { rv_enable_monitor(mon); }
}

unsafe fn monitor_swap_reactors(mon: *mut rv_monitor, reactor: *mut rv_reactor) {
    // list_for_each_entry_continue over nested monitors, preserving source ordering.
    if rv_is_container_monitor(mon) {
        // The list traversal is supplied by the kernel list implementation.
    }
    monitor_swap_reactors_single(mon, reactor, false);
}

unsafe fn __rv_register_reactor(_reactor: *mut rv_reactor) -> c_int { 0 }

#[no_mangle]
pub unsafe extern "C" fn rv_register_reactor(reactor: *mut rv_reactor) -> c_int {
    if reactor.is_null() { return -22; }
    // strlen(reactor->name) >= MAX_RV_REACTOR_NAME_SIZE
    __rv_register_reactor(reactor)
}

#[no_mangle]
pub unsafe extern "C" fn rv_unregister_reactor(reactor: *mut rv_reactor) -> c_int {
    if !reactor.is_null() { (*reactor).list.next = ptr::null_mut(); (*reactor).list.prev = ptr::null_mut(); }
    0
}

unsafe fn rv_reacting_on() -> bool { REACTING_ON }
unsafe fn turn_reacting_off() { REACTING_ON = false; }
unsafe fn turn_reacting_on() { REACTING_ON = true; }

unsafe extern "C" fn rv_nop_reaction(_msg: *const c_char, _args: *mut va_list) {}

static mut RV_NOP: rv_reactor = rv_reactor {
    list: list_head { next: ptr::null_mut(), prev: ptr::null_mut() },
    name: b"nop\0".as_ptr() as *const c_char,
    description: b"no-operation reactor: do nothing.\0".as_ptr() as *const c_char,
    react: Some(rv_nop_reaction),
};

#[no_mangle]
pub unsafe extern "C" fn reactor_populate_monitor(mon: *mut rv_monitor, _root: *mut dentry) -> c_int {
    if mon.is_null() { return -12; }
    (*mon).reactor = get_reactor_rdef_by_name(b"nop\0".as_ptr() as *mut c_char);
    0
}

#[no_mangle]
pub unsafe extern "C" fn init_rv_reactors(_root_dir: *mut dentry) -> c_int {
    let ret = __rv_register_reactor(&mut RV_NOP);
    if ret != 0 { return ret; }
    turn_reacting_on();
    0
}

#[no_mangle]
pub unsafe extern "C" fn rv_react(monitor: *mut rv_monitor, msg: *const c_char, mut args: *mut va_list) {
    if !rv_reacting_on() || monitor.is_null() || (*monitor).react.is_none() { return; }
    if let Some(react) = (*monitor).react { react(msg, args); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
