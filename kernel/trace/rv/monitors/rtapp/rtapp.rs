// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux kernel and rtapp.h are external to this translation.

use core::ffi::{c_char, c_void};

pub const MODULE_NAME: &str = "rtapp";

#[repr(C)]
pub struct rv_monitor {
    pub name: *const c_char,
    pub description: *const c_char,
}

unsafe extern "C" {
    pub fn rv_register_monitor(monitor: *mut rv_monitor, data: *mut c_void) -> i32;
    pub fn rv_unregister_monitor(monitor: *mut rv_monitor);
}

#[no_mangle]
pub static mut rv_rtapp: rv_monitor = rv_monitor {
    name: b"rtapp\0".as_ptr() as *const c_char,
    description: b"Collection of monitors for detecting problems with real-time applications\0"
        .as_ptr() as *const c_char,
};

// C __init function.
#[no_mangle]
pub unsafe extern "C" fn register_rtapp() -> i32 {
    rv_register_monitor(&raw mut rv_rtapp, core::ptr::null_mut())
}

// C __exit function.
#[no_mangle]
pub unsafe extern "C" fn unregister_rtapp() {
    rv_unregister_monitor(&raw mut rv_rtapp);
}

// C module_init(register_rtapp);
// C module_exit(unregister_rtapp);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Nam Cao <namcao@linutronix.de>");
// MODULE_DESCRIPTION("Collection of monitors for detecting problems with real-time applications");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
