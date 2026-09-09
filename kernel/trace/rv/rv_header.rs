/* SPDX-License-Identifier: GPL-2.0 */

/* C dependency: <linux/mutex.h> */
/* C dependency: ../trace.h */
/* C dependency: <linux/tracefs.h> */
/* C dependency: <linux/rv.h> */

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct rv_interface {
    pub root_dir: *mut dentry,
    pub monitors_dir: *mut dentry,
}

/* Opaque types supplied by the corresponding C dependencies. */
pub enum dentry {}
pub enum mutex {}
pub enum list_head {}
pub enum rv_monitor {}

/* These constants are supplied by the corresponding trace headers. */
pub const RV_MODE_WRITE: _ = TRACE_MODE_WRITE;
pub const RV_MODE_READ: _ = TRACE_MODE_READ;

macro_rules! rv_create_dir {
    ($($arg:tt)*) => { tracefs_create_dir!($($arg)*) };
}

macro_rules! rv_create_file {
    ($($arg:tt)*) => { tracefs_create_file!($($arg)*) };
}

macro_rules! rv_remove {
    ($($arg:tt)*) => { tracefs_remove!($($arg)*) };
}

/* C DEFINE_FREE(rv_remove, struct dentry *, if (_T) rv_remove(_T)); */
#[inline]
pub unsafe fn rv_remove_cleanup(value: *mut dentry) {
    if !value.is_null() {
        rv_remove!(value);
    }
}

pub const MAX_RV_MONITOR_NAME_SIZE: usize = 32;
pub const MAX_RV_REACTOR_NAME_SIZE: usize = 32;

unsafe extern "C" {
    pub static mut rv_interface_lock: mutex;
    pub static mut rv_monitors_list: list_head;

    pub fn get_monitors_root() -> *mut dentry;
    pub fn rv_disable_monitor(mon: *mut rv_monitor) -> c_int;
    pub fn rv_enable_monitor(mon: *mut rv_monitor) -> c_int;
    pub fn rv_is_container_monitor(mon: *mut rv_monitor) -> bool;
    pub fn rv_is_nested_monitor(mon: *mut rv_monitor) -> bool;
}

#[cfg(CONFIG_RV_REACTORS)]
unsafe extern "C" {
    pub fn reactor_populate_monitor(mon: *mut rv_monitor, root: *mut dentry) -> c_int;
    pub fn init_rv_reactors(root_dir: *mut dentry) -> c_int;
}

#[cfg(not(CONFIG_RV_REACTORS))]
#[inline]
pub unsafe fn reactor_populate_monitor(_mon: *mut rv_monitor, _root: *mut dentry) -> c_int {
    0
}

#[cfg(not(CONFIG_RV_REACTORS))]
#[inline]
pub unsafe fn init_rv_reactors(_root_dir: *mut dentry) -> c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
