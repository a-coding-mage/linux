/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: linux/atomic.h, linux/debugfs.h, linux/notifier.h

#[repr(C)]
pub struct notifier_err_inject_action {
    pub val: ::core::ffi::c_ulong,
    pub error: ::core::ffi::c_int,
    pub name: *const ::core::ffi::c_char,
}

// Equivalent of: .name = #action, .val = (action),
#[macro_export]
macro_rules! NOTIFIER_ERR_INJECT_ACTION {
    ($action:expr) => {
        .name = stringify!($action),
        .val = $action,
    };
}

#[repr(C)]
pub struct notifier_err_inject {
    pub nb: notifier_block,
    pub actions: [notifier_err_inject_action; 0],
    // The last slot must be terminated with zero sentinel.
}

extern "C" {
    pub static mut notifier_err_inject_dir: *mut dentry;

    pub fn notifier_err_inject_init(
        name: *const ::core::ffi::c_char,
        parent: *mut dentry,
        err_inject: *mut notifier_err_inject,
        priority: ::core::ffi::c_int,
    ) -> *mut dentry;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
