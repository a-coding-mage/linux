// SPDX-License-Identifier: GPL-2.0-only
//
// C dependencies supplied by the kernel and notifier-error-inject.h are
// intentionally represented as external Rust declarations.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

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
    pub actions: [notifier_err_inject_action; 4],
    pub nb: notifier_block,
}

// Values and layout supplied by the kernel and notifier-error-inject.h.
unsafe extern "C" {
    static mut notifier_err_inject_dir: *mut dentry;

    fn notifier_err_inject_init(
        name: *const c_char,
        parent: *mut dentry,
        inject: *mut notifier_err_inject,
        priority: c_int,
    ) -> *mut dentry;
    fn register_pm_notifier(nb: *mut notifier_block) -> c_int;
    fn unregister_pm_notifier(nb: *mut notifier_block);
    fn debugfs_remove_recursive(dentry: *mut dentry);
    fn IS_ERR(ptr: *mut dentry) -> bool;
    fn PTR_ERR(ptr: *mut dentry) -> c_int;
}

// Build-time constants supplied by <linux/suspend.h>.
const PM_HIBERNATION_PREPARE: c_int = 0;
const PM_SUSPEND_PREPARE: c_int = 1;
const PM_RESTORE_PREPARE: c_int = 2;

// Direct equivalent of NOTIFIER_ERR_INJECT_ACTION(action).
const fn notifier_err_inject_action(action: c_int) -> notifier_err_inject_action {
    notifier_err_inject_action { action }
}

static mut priority: c_int = 0;

static mut pm_notifier_err_inject: notifier_err_inject = notifier_err_inject {
    actions: [
        notifier_err_inject_action(PM_HIBERNATION_PREPARE),
        notifier_err_inject_action(PM_SUSPEND_PREPARE),
        notifier_err_inject_action(PM_RESTORE_PREPARE),
        notifier_err_inject_action(0),
    ],
    nb: notifier_block { _private: [] },
};

static mut dir: *mut dentry = core::ptr::null_mut();

unsafe fn err_inject_init() -> c_int {
    let mut err: c_int;

    dir = notifier_err_inject_init(
        c"pm".as_ptr(),
        notifier_err_inject_dir,
        &raw mut pm_notifier_err_inject,
        priority,
    );
    if IS_ERR(dir) {
        return PTR_ERR(dir);
    }

    err = register_pm_notifier(&raw mut pm_notifier_err_inject.nb);
    if err != 0 {
        debugfs_remove_recursive(dir);
    }

    err
}

unsafe fn err_inject_exit() {
    unregister_pm_notifier(&raw mut pm_notifier_err_inject.nb);
    debugfs_remove_recursive(dir);
}

// Equivalent to module_init(err_inject_init) and module_exit(err_inject_exit).
#[used]
static MODULE_INIT: unsafe fn() -> c_int = err_inject_init;
#[used]
static MODULE_EXIT: unsafe fn() = err_inject_exit;

// MODULE_DESCRIPTION("PM notifier error injection module");
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Akinobu Mita <akinobu.mita@gmail.com>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
