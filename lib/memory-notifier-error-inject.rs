// SPDX-License-Identifier: GPL-2.0-only
//
// C dependencies: <linux/kernel.h>, <linux/module.h>, <linux/memory.h>, and
// "notifier-error-inject.h" are supplied by the surrounding kernel sources.

use core::ffi::{c_char, c_int, c_void};

// The following types, constants, and functions are supplied by the kernel
// headers and notifier-error-inject.h.
#[repr(C)]
pub struct NotifierBlock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct NotifierErrInjectAction {
    pub action: c_int,
}

#[repr(C)]
pub struct NotifierErrInject {
    pub actions: [NotifierErrInjectAction; 3],
    pub nb: NotifierBlock,
}

#[repr(C)]
pub struct Dentry {
    _private: [u8; 0],
}

extern "C" {
    pub static mut notifier_err_inject_dir: *mut Dentry;
    pub fn notifier_err_inject_init(
        name: *const c_char,
        parent: *mut Dentry,
        inject: *mut NotifierErrInject,
        priority: c_int,
    ) -> *mut Dentry;
    pub fn register_memory_notifier(nb: *mut NotifierBlock) -> c_int;
    pub fn unregister_memory_notifier(nb: *mut NotifierBlock);
    pub fn debugfs_remove_recursive(dir: *mut Dentry);
    pub fn ptr_err(ptr: *mut c_void) -> c_int;
    pub fn is_err(ptr: *mut Dentry) -> bool;
}

// MEM_GOING_ONLINE and MEM_GOING_OFFLINE are supplied by <linux/memory.h>.
// NOTIFIER_ERR_INJECT_ACTION is supplied by "notifier-error-inject.h".
extern "C" {
    pub static MEM_GOING_ONLINE: c_int;
    pub static MEM_GOING_OFFLINE: c_int;
}

static mut priority: c_int = 0;

static mut memory_notifier_err_inject: NotifierErrInject = NotifierErrInject {
    actions: [
        NotifierErrInjectAction { action: 0 }, // NOTIFIER_ERR_INJECT_ACTION(MEM_GOING_ONLINE)
        NotifierErrInjectAction { action: 0 }, // NOTIFIER_ERR_INJECT_ACTION(MEM_GOING_OFFLINE)
        NotifierErrInjectAction { action: 0 },
    ],
    nb: NotifierBlock { _private: [] },
};

static mut dir: *mut Dentry = core::ptr::null_mut();

unsafe fn err_inject_init() -> c_int {
    let mut err: c_int;

    dir = notifier_err_inject_init(
        b"memory\0".as_ptr() as *const c_char,
        notifier_err_inject_dir,
        &mut memory_notifier_err_inject,
        priority,
    );
    if is_err(dir) {
        return ptr_err(dir as *mut c_void);
    }

    err = register_memory_notifier(&mut memory_notifier_err_inject.nb);
    if err != 0 {
        debugfs_remove_recursive(dir);
    }

    err
}

unsafe fn err_inject_exit() {
    unregister_memory_notifier(&mut memory_notifier_err_inject.nb);
    debugfs_remove_recursive(dir);
}

// module_init(err_inject_init);
// module_exit(err_inject_exit);
// MODULE_DESCRIPTION("memory notifier error injection module");
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Akinobu Mita <akinobu.mita@gmail.com>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
