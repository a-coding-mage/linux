// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Joe Lawrence <joe.lawrence@redhat.com>

// C preprocessor/module dependency intent:
// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// #include <linux/module.h>
// #include <linux/kernel.h>
// #include <linux/livepatch.h>

use core::ffi::{c_char, c_int, c_void};

const MODULE_STATE_LIVE: usize = 0;
const MODULE_STATE_COMING: usize = 1;
const MODULE_STATE_GOING: usize = 2;
const MODULE_STATE_UNFORMED: usize = 3;

#[repr(C)]
pub struct module {
    pub state: usize,
    pub name: *const c_char,
}

#[repr(C)]
pub struct klp_object {
    pub name: *const c_char,
    pub funcs: *mut klp_func,
    pub callbacks: klp_callbacks,
    pub mod_: *mut module,
}

#[repr(C)]
pub struct klp_func {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct klp_callbacks {
    pub pre_patch: Option<unsafe extern "C" fn(*mut klp_object) -> c_int>,
    pub post_patch: Option<unsafe extern "C" fn(*mut klp_object)>,
    pub pre_unpatch: Option<unsafe extern "C" fn(*mut klp_object)>,
    pub post_unpatch: Option<unsafe extern "C" fn(*mut klp_object)>,
}

#[repr(C)]
pub struct klp_patch {
    pub mod_: *mut c_void,
    pub objs: *mut klp_object,
    pub replace: c_int,
}

unsafe extern "C" {
    static mut THIS_MODULE: c_void;

    fn klp_enable_patch(patch: *mut klp_patch) -> c_int;
    fn pr_info(fmt: *const c_char, ...);
}

static mut replace: c_int = 0;
// module_param(replace, int, 0644);
// MODULE_PARM_DESC(replace, "replace (default=0)");

static MODULE_STATE_LIVE_STR: &[u8] = b"[MODULE_STATE_LIVE] Normal state\0";
static MODULE_STATE_COMING_STR: &[u8] =
    b"[MODULE_STATE_COMING] Full formed, running module_init\0";
static MODULE_STATE_GOING_STR: &[u8] = b"[MODULE_STATE_GOING] Going away\0";
static MODULE_STATE_UNFORMED_STR: &[u8] = b"[MODULE_STATE_UNFORMED] Still setting it up\0";

static module_state: [*const c_char; 4] = [
    MODULE_STATE_LIVE_STR.as_ptr() as *const c_char,
    MODULE_STATE_COMING_STR.as_ptr() as *const c_char,
    MODULE_STATE_GOING_STR.as_ptr() as *const c_char,
    MODULE_STATE_UNFORMED_STR.as_ptr() as *const c_char,
];

unsafe fn callback_info(callback: *const c_char, obj: *mut klp_object) {
    if !(*obj).mod_.is_null() {
        pr_info(
            b"%s: %s -> %s\n\0".as_ptr() as *const c_char,
            callback,
            (*(*obj).mod_).name,
            module_state[(*(*obj).mod_).state],
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
    0
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

static mut no_funcs: [klp_func; 1] = [klp_func { _unused: [] }];

static mut objs: [klp_object; 2] = [
    klp_object {
        name: core::ptr::null(), /* vmlinux */
        funcs: unsafe { no_funcs.as_mut_ptr() },
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

static mut patch: klp_patch = klp_patch {
    mod_: unsafe { &raw mut THIS_MODULE },
    objs: unsafe { objs.as_mut_ptr() },
    /* set .replace in the init function below for demo purposes */
    replace: 0,
};

unsafe extern "C" fn test_klp_callbacks_demo2_init() -> c_int {
    patch.replace = replace;
    klp_enable_patch(&raw mut patch)
}

unsafe extern "C" fn test_klp_callbacks_demo2_exit() {}

// module_init(test_klp_callbacks_demo2_init);
// module_exit(test_klp_callbacks_demo2_exit);
#[used]
static MODULE_LICENSE: &[u8] = b"GPL\0";
#[used]
static MODULE_INFO_LIVEPATCH: &[u8] = b"Y\0";
#[used]
static MODULE_AUTHOR: &[u8] = b"Joe Lawrence <joe.lawrence@redhat.com>\0";
#[used]
static MODULE_DESCRIPTION: &[u8] = b"Livepatch test: livepatch demo2\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
