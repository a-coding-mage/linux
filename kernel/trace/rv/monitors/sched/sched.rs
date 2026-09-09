// SPDX-License-Identifier: GPL-2.0
//
// C dependencies:
// #include <linux/kernel.h>
// #include <linux/module.h>
// #include <linux/init.h>
// #include <linux/rv.h>
// #include "sched.h"

use core::ffi::c_char;

const MODULE_NAME: &str = "sched";

#[repr(C)]
pub struct rv_monitor {
    pub name: *const c_char,
    pub description: *const c_char,
    pub enable: Option<unsafe extern "C" fn()>,
    pub disable: Option<unsafe extern "C" fn()>,
    pub reset: Option<unsafe extern "C" fn()>,
    pub enabled: i32,
}

unsafe extern "C" {
    fn rv_register_monitor(monitor: *mut rv_monitor, data: *mut core::ffi::c_void) -> i32;
    fn rv_unregister_monitor(monitor: *mut rv_monitor);
}

#[no_mangle]
pub static mut rv_sched: rv_monitor = rv_monitor {
    name: b"sched\0".as_ptr() as *const c_char,
    description: b"container for several scheduler monitor specifications.\0".as_ptr()
        as *const c_char,
    enable: None,
    disable: None,
    reset: None,
    enabled: 0,
};

unsafe extern "C" fn register_sched() -> i32 {
    rv_register_monitor(&raw mut rv_sched, core::ptr::null_mut())
}

unsafe extern "C" fn unregister_sched() {
    rv_unregister_monitor(&raw mut rv_sched);
}

// module_init(register_sched);
// module_exit(unregister_sched);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Gabriele Monaco <gmonaco@redhat.com>");
// MODULE_DESCRIPTION("sched: container for several scheduler monitor specifications.");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
