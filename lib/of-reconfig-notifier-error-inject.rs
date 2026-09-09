// SPDX-License-Identifier: GPL-2.0-only
//
// C dependencies:
// #include <linux/kernel.h>
// #include <linux/module.h>
// #include <linux/of.h>
// #include "notifier-error-inject.h"

use core::ffi::{c_char, c_int, c_void};

// These types, constants, and functions are supplied by the surrounding
// kernel sources represented by the C includes above.
#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct notifier_err_inject_action {
    pub action: c_int,
}

#[repr(C)]
pub struct notifier_err_inject {
    pub actions: [notifier_err_inject_action; 6],
    pub nb: notifier_block,
}

extern "C" {
    static mut notifier_err_inject_dir: *mut c_void;
    fn notifier_err_inject_init(
        name: *const c_char,
        parent: *mut c_void,
        inject: *mut notifier_err_inject,
        priority: c_int,
    ) -> *mut c_void;
    fn of_reconfig_notifier_register(nb: *mut notifier_block) -> c_int;
    fn of_reconfig_notifier_unregister(nb: *mut notifier_block);
    fn debugfs_remove_recursive(dir: *mut c_void);
}

// Build-time kernel macros/constants retained as external symbols here.
extern "C" {
    static OF_RECONFIG_ATTACH_NODE: c_int;
    static OF_RECONFIG_DETACH_NODE: c_int;
    static OF_RECONFIG_ADD_PROPERTY: c_int;
    static OF_RECONFIG_REMOVE_PROPERTY: c_int;
    static OF_RECONFIG_UPDATE_PROPERTY: c_int;
}

static mut priority: c_int = 0;
static mut reconfig_err_inject: notifier_err_inject = notifier_err_inject {
    actions: [
        notifier_err_inject_action { action: 0 },
        notifier_err_inject_action { action: 0 },
        notifier_err_inject_action { action: 0 },
        notifier_err_inject_action { action: 0 },
        notifier_err_inject_action { action: 0 },
        notifier_err_inject_action { action: 0 },
    ],
    nb: notifier_block { _private: [] },
};

static mut dir: *mut c_void = core::ptr::null_mut();

unsafe fn err_inject_init() -> c_int {
    let err: c_int;

    let name = b"OF-reconfig\0";
    dir = notifier_err_inject_init(
        name.as_ptr() as *const c_char,
        notifier_err_inject_dir,
        &mut reconfig_err_inject,
        priority,
    );
    // IS_ERR(dir) / PTR_ERR(dir) are kernel error-pointer operations.
    if (dir as usize) >= (!4095usize) {
        return dir as isize as c_int;
    }

    err = of_reconfig_notifier_register(&mut reconfig_err_inject.nb);
    if err != 0 {
        debugfs_remove_recursive(dir);
    }

    err
}

unsafe fn err_inject_exit() {
    of_reconfig_notifier_unregister(&mut reconfig_err_inject.nb);
    debugfs_remove_recursive(dir);
}

// module_param(priority, int, 0);
// MODULE_PARM_DESC(priority, "specify OF reconfig notifier priority");
// module_init(err_inject_init);
// module_exit(err_inject_exit);
// MODULE_DESCRIPTION("OF reconfig notifier error injection module");
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Akinobu Mita <akinobu.mita@gmail.com>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
