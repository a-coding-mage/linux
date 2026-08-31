// SPDX-License-Identifier: GPL-2.0
// C dependencies translated from:
// #include <linux/kernel.h>
// #include <linux/module.h>
// #include <linux/init.h>
// #include <linux/rv.h>
// #include "%%MODEL_NAME%%.h"

pub const MODULE_NAME: &str = "%%MODEL_NAME%%";

#[repr(C)]
pub struct rv_monitor {
    pub name: *const ::core::ffi::c_char,
    pub description: *const ::core::ffi::c_char,
    pub enable: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub disable: Option<unsafe extern "C" fn()>,
    pub reset: Option<unsafe extern "C" fn()>,
    pub enabled: ::core::ffi::c_int,
}

extern "C" {
    pub fn rv_register_monitor(
        monitor: *mut rv_monitor,
        parent: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    pub fn rv_unregister_monitor(monitor: *mut rv_monitor);
}

#[no_mangle]
pub static mut rv_%%MODEL_NAME%%: rv_monitor = rv_monitor {
    name: b"%%MODEL_NAME%%\0".as_ptr() as *const ::core::ffi::c_char,
    description: b"%%DESCRIPTION%%\0".as_ptr() as *const ::core::ffi::c_char,
    enable: None,
    disable: None,
    reset: None,
    enabled: 0,
};

// Original C used: static int __init register_%%MODEL_NAME%%(void)
unsafe extern "C" fn register_%%MODEL_NAME%%() -> ::core::ffi::c_int {
    rv_register_monitor(
        &mut rv_%%MODEL_NAME%% as *mut rv_monitor,
        ::core::ptr::null_mut(),
    )
}

// Original C used: static void __exit unregister_%%MODEL_NAME%%(void)
unsafe extern "C" fn unregister_%%MODEL_NAME%%() {
    rv_unregister_monitor(&mut rv_%%MODEL_NAME%% as *mut rv_monitor);
}

// Original C module metadata/macros:
// module_init(register_%%MODEL_NAME%%);
// module_exit(unregister_%%MODEL_NAME%%);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("rvgen: auto-generated");
// MODULE_DESCRIPTION("%%MODEL_NAME%%: %%DESCRIPTION%%");
