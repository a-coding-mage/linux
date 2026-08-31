// SPDX-License-Identifier: GPL-2.0
// Dependencies from the original C source:
// linux/kernel.h, linux/module.h, linux/init.h, linux/rv.h, and "test_container.h".

pub const MODULE_NAME: &[u8] = b"test_container\0";

#[repr(C)]
pub struct rv_monitor {
    pub name: *const core::ffi::c_char,
    pub description: *const core::ffi::c_char,
    pub enable: Option<unsafe extern "C" fn(*mut rv_monitor) -> core::ffi::c_int>,
    pub disable: Option<unsafe extern "C" fn(*mut rv_monitor)>,
    pub reset: Option<unsafe extern "C" fn(*mut rv_monitor)>,
    pub enabled: core::ffi::c_int,
}

unsafe extern "C" {
    fn rv_register_monitor(
        monitor: *mut rv_monitor,
        parent: *mut rv_monitor,
    ) -> core::ffi::c_int;
    fn rv_unregister_monitor(monitor: *mut rv_monitor);
}

#[unsafe(no_mangle)]
pub static mut rv_test_container: rv_monitor = rv_monitor {
    name: b"test_container\0".as_ptr() as *const core::ffi::c_char,
    description: b"Test container for grouping monitors\0".as_ptr() as *const core::ffi::c_char,
    enable: None,
    disable: None,
    reset: None,
    enabled: 0,
};

unsafe extern "C" fn register_test_container() -> core::ffi::c_int {
    unsafe { rv_register_monitor(&raw mut rv_test_container, core::ptr::null_mut()) }
}

unsafe extern "C" fn unregister_test_container() {
    unsafe {
        rv_unregister_monitor(&raw mut rv_test_container);
    }
}

// Original C module wiring:
// module_init(register_test_container);
// module_exit(unregister_test_container);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("rvgen: auto-generated");
// MODULE_DESCRIPTION("test_container: Test container for grouping monitors");
