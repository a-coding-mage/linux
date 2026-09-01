// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Joe Lawrence <joe.lawrence@redhat.com>

// pr_fmt(fmt) was defined as: KBUILD_MODNAME ": " fmt
// C dependencies: <linux/module.h>, <linux/kernel.h>, <linux/livepatch.h>

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut THIS_MODULE: *mut module;

    fn klp_enable_patch(patch: *mut klp_patch) -> c_int;
    fn pr_info(fmt: *const c_char, ...);
}

#[repr(C)]
pub struct module {
    pub name: *const c_char,
    pub state: usize,
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct klp_callbacks {
    pub pre_patch: Option<unsafe extern "C" fn(obj: *mut klp_object) -> c_int>,
    pub post_patch: Option<unsafe extern "C" fn(obj: *mut klp_object)>,
    pub pre_unpatch: Option<unsafe extern "C" fn(obj: *mut klp_object)>,
    pub post_unpatch: Option<unsafe extern "C" fn(obj: *mut klp_object)>,
}

#[repr(C)]
pub struct klp_func {
    pub old_name: *const c_char,
    pub new_func: *const c_void,
}

#[repr(C)]
pub struct klp_object {
    pub name: *const c_char,
    pub funcs: *mut klp_func,
    pub callbacks: klp_callbacks,
    pub mod_: *mut module,
}

#[repr(C)]
pub struct klp_patch {
    pub mod_: *mut module,
    pub objs: *mut klp_object,
}

const MODULE_STATE_LIVE: usize = 0;
const MODULE_STATE_COMING: usize = 1;
const MODULE_STATE_GOING: usize = 2;
const MODULE_STATE_UNFORMED: usize = 3;

static mut PRE_PATCH_RET: c_int = 0;
// module_param(pre_patch_ret, int, 0644);
// MODULE_PARM_DESC(pre_patch_ret, "pre_patch_ret (default=0)");

static MODULE_STATE: [*const c_char; 4] = [
    b"[MODULE_STATE_LIVE] Normal state\0".as_ptr() as *const c_char,
    b"[MODULE_STATE_COMING] Full formed, running module_init\0".as_ptr() as *const c_char,
    b"[MODULE_STATE_GOING] Going away\0".as_ptr() as *const c_char,
    b"[MODULE_STATE_UNFORMED] Still setting it up\0".as_ptr() as *const c_char,
];

unsafe extern "C" fn callback_info(callback: *const c_char, obj: *mut klp_object) {
    if !(*obj).mod_.is_null() {
        pr_info(
            b"%s: %s -> %s\n\0".as_ptr() as *const c_char,
            callback,
            (*(*obj).mod_).name,
            MODULE_STATE[(*(*obj).mod_).state],
        );
    } else {
        pr_info(
            b"%s: vmlinux\n\0".as_ptr() as *const c_char,
            callback,
        );
    }
}

/* Executed on object patching (ie, patch enablement) */
unsafe extern "C" fn pre_patch_callback(obj: *mut klp_object) -> c_int {
    callback_info(b"pre_patch_callback\0".as_ptr() as *const c_char, obj);
    PRE_PATCH_RET
}

/* Executed on object unpatching (ie, patch disablement) */
unsafe extern "C" fn post_patch_callback(obj: *mut klp_object) {
    callback_info(b"post_patch_callback\0".as_ptr() as *const c_char, obj);
}

/* Executed on object unpatching (ie, patch disablement) */
unsafe extern "C" fn pre_unpatch_callback(obj: *mut klp_object) {
    callback_info(b"pre_unpatch_callback\0".as_ptr() as *const c_char, obj);
}

/* Executed on object unpatching (ie, patch disablement) */
unsafe extern "C" fn post_unpatch_callback(obj: *mut klp_object) {
    callback_info(b"post_unpatch_callback\0".as_ptr() as *const c_char, obj);
}

unsafe extern "C" fn patched_work_func(_work: *mut work_struct) {
    pr_info(
        b"%s\n\0".as_ptr() as *const c_char,
        b"patched_work_func\0".as_ptr() as *const c_char,
    );
}

static mut NO_FUNCS: [klp_func; 1] = [
    klp_func {
        old_name: core::ptr::null(),
        new_func: core::ptr::null(),
    },
];

static mut BUSYMOD_FUNCS: [klp_func; 2] = [
    klp_func {
        old_name: b"busymod_work_func\0".as_ptr() as *const c_char,
        new_func: patched_work_func as *const c_void,
    },
    klp_func {
        old_name: core::ptr::null(),
        new_func: core::ptr::null(),
    },
];

static mut OBJS: [klp_object; 4] = [
    klp_object {
        name: core::ptr::null(), /* vmlinux */
        funcs: unsafe { NO_FUNCS.as_mut_ptr() },
        callbacks: klp_callbacks {
            pre_patch: Some(pre_patch_callback),
            post_patch: Some(post_patch_callback),
            pre_unpatch: Some(pre_unpatch_callback),
            post_unpatch: Some(post_unpatch_callback),
        },
        mod_: core::ptr::null_mut(),
    },
    klp_object {
        name: b"test_klp_callbacks_mod\0".as_ptr() as *const c_char,
        funcs: unsafe { NO_FUNCS.as_mut_ptr() },
        callbacks: klp_callbacks {
            pre_patch: Some(pre_patch_callback),
            post_patch: Some(post_patch_callback),
            pre_unpatch: Some(pre_unpatch_callback),
            post_unpatch: Some(post_unpatch_callback),
        },
        mod_: core::ptr::null_mut(),
    },
    klp_object {
        name: b"test_klp_callbacks_busy\0".as_ptr() as *const c_char,
        funcs: unsafe { BUSYMOD_FUNCS.as_mut_ptr() },
        callbacks: klp_callbacks {
            pre_patch: Some(pre_patch_callback),
            post_patch: Some(post_patch_callback),
            pre_unpatch: Some(pre_unpatch_callback),
            post_unpatch: Some(post_unpatch_callback),
        },
        mod_: core::ptr::null_mut(),
    },
    klp_object {
        name: core::ptr::null(),
        funcs: core::ptr::null_mut(),
        callbacks: klp_callbacks {
            pre_patch: None,
            post_patch: None,
            pre_unpatch: None,
            post_unpatch: None,
        },
        mod_: core::ptr::null_mut(),
    },
];

static mut PATCH: klp_patch = klp_patch {
    mod_: unsafe { THIS_MODULE },
    objs: unsafe { OBJS.as_mut_ptr() },
};

unsafe extern "C" fn test_klp_callbacks_demo_init() -> c_int {
    klp_enable_patch(core::ptr::addr_of_mut!(PATCH))
}

unsafe extern "C" fn test_klp_callbacks_demo_exit() {}

// module_init(test_klp_callbacks_demo_init);
// module_exit(test_klp_callbacks_demo_exit);
// MODULE_LICENSE("GPL");
// MODULE_INFO(livepatch, "Y");
// MODULE_AUTHOR("Joe Lawrence <joe.lawrence@redhat.com>");
// MODULE_DESCRIPTION("Livepatch test: livepatch demo");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
