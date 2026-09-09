// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux kernel and deadline.h are intentionally
// left external to this translation.

const MODULE_NAME: &str = "deadline";

#[repr(C)]
pub struct rv_monitor {
    pub name: *const core::ffi::c_char,
    pub description: *const core::ffi::c_char,
    pub enable: Option<unsafe extern "C" fn()>,
    pub disable: Option<unsafe extern "C" fn()>,
    pub reset: Option<unsafe extern "C" fn()>,
    pub enabled: i32,
}

#[repr(C)]
pub struct sched_class {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn rv_register_monitor(monitor: *mut rv_monitor, arg: *mut core::ffi::c_void) -> i32;
    fn rv_unregister_monitor(monitor: *mut rv_monitor);
    fn kallsyms_lookup_name(name: *const core::ffi::c_char) -> usize;
}

#[unsafe(no_mangle)]
pub static mut rv_deadline: rv_monitor = rv_monitor {
    name: b"deadline\0".as_ptr() as *const core::ffi::c_char,
    description: b"container for several deadline scheduler specifications.\0".as_ptr()
        as *const core::ffi::c_char,
    enable: None,
    disable: None,
    reset: None,
    enabled: 0,
};

/* Used by other monitors */
#[unsafe(no_mangle)]
pub static mut rv_ext_sched_class: *mut sched_class = core::ptr::null_mut();

unsafe extern "C" fn register_deadline() -> i32 {
    // Equivalent to IS_ENABLED(CONFIG_SCHED_CLASS_EXT); the build-time
    // configuration is supplied by the surrounding kernel build.
    if cfg!(feature = "CONFIG_SCHED_CLASS_EXT") {
        rv_ext_sched_class = kallsyms_lookup_name(
            b"ext_sched_class\0".as_ptr() as *const core::ffi::c_char,
        ) as *mut sched_class;
        if rv_ext_sched_class.is_null() {
            // Equivalent to: pr_warn("rv: Missing ext_sched_class, monitors may not work.\n");
        }
    }
    rv_register_monitor(&raw mut rv_deadline, core::ptr::null_mut())
}

unsafe extern "C" fn unregister_deadline() {
    rv_unregister_monitor(&raw mut rv_deadline);
}

// Equivalent to module_init(register_deadline);
// Equivalent to module_exit(unregister_deadline);

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Gabriele Monaco <gmonaco@redhat.com>");
// MODULE_DESCRIPTION("deadline: container for several deadline scheduler specifications.");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
