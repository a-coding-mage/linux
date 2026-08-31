// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2019 SUSE

// pr_fmt(fmt) was defined as: KBUILD_MODNAME ": " fmt
// C dependencies: linux/slab.h, linux/module.h, linux/kernel.h,
// linux/printk.h, linux/livepatch.h

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

const CONSOLE_LOGLEVEL_STATE: c_int = 1;
/* Version 1 does not support migration. */
const CONSOLE_LOGLEVEL_STATE_VERSION: c_int = 1;

const MODULE_STATE_LIVE: usize = 0;
const MODULE_STATE_COMING: usize = 1;
const MODULE_STATE_GOING: usize = 2;
const MODULE_STATE_UNFORMED: usize = 3;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

extern "C" {
    static mut console_loglevel: c_int;
    static CONSOLE_LOGLEVEL_MOTORMOUTH: c_int;
    static THIS_MODULE: *mut module;
    static GFP_KERNEL: gfp_t;

    fn pr_info(fmt: *const c_char, ...);
    fn kzalloc(size: usize, flags: gfp_t) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn klp_get_state(patch: *mut klp_patch, id: c_int) -> *mut klp_state;
    fn klp_enable_patch(patch: *mut klp_patch) -> c_int;
}

type gfp_t = u32;

#[repr(C)]
pub struct module {
    pub name: *const c_char,
    pub state: usize,
}

#[repr(C)]
pub struct klp_state {
    pub id: c_int,
    pub version: c_int,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct klp_func {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct klp_callbacks {
    pub pre_patch: Option<unsafe extern "C" fn(obj: *mut klp_object) -> c_int>,
    pub post_patch: Option<unsafe extern "C" fn(obj: *mut klp_object)>,
    pub pre_unpatch: Option<unsafe extern "C" fn(obj: *mut klp_object)>,
    pub post_unpatch: Option<unsafe extern "C" fn(obj: *mut klp_object)>,
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
    pub states: *mut klp_state,
    pub replace: bool,
}

static MODULE_STATE_LIVE_STR: &[u8] = b"[MODULE_STATE_LIVE] Normal state\0";
static MODULE_STATE_COMING_STR: &[u8] = b"[MODULE_STATE_COMING] Full formed, running module_init\0";
static MODULE_STATE_GOING_STR: &[u8] = b"[MODULE_STATE_GOING] Going away\0";
static MODULE_STATE_UNFORMED_STR: &[u8] = b"[MODULE_STATE_UNFORMED] Still setting it up\0";

static module_state: [*const c_char; 4] = [
    MODULE_STATE_LIVE_STR.as_ptr() as *const c_char,
    MODULE_STATE_COMING_STR.as_ptr() as *const c_char,
    MODULE_STATE_GOING_STR.as_ptr() as *const c_char,
    MODULE_STATE_UNFORMED_STR.as_ptr() as *const c_char,
];

unsafe extern "C" fn callback_info(callback: *const c_char, obj: *mut klp_object) {
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

// C forward declaration: static struct klp_patch patch;

unsafe extern "C" fn allocate_loglevel_state() -> c_int {
    let loglevel_state: *mut klp_state;

    loglevel_state = klp_get_state(&mut patch, CONSOLE_LOGLEVEL_STATE);
    if loglevel_state.is_null() {
        return -EINVAL;
    }

    (*loglevel_state).data = kzalloc(core::mem::size_of_val(&console_loglevel), GFP_KERNEL);
    if (*loglevel_state).data.is_null() {
        return -ENOMEM;
    }

    pr_info(
        b"%s: allocating space to store console_loglevel\n\0".as_ptr() as *const c_char,
        b"allocate_loglevel_state\0".as_ptr() as *const c_char,
    );
    0
}

unsafe extern "C" fn fix_console_loglevel() {
    let loglevel_state: *mut klp_state;

    loglevel_state = klp_get_state(&mut patch, CONSOLE_LOGLEVEL_STATE);
    if loglevel_state.is_null() {
        return;
    }

    pr_info(
        b"%s: fixing console_loglevel\n\0".as_ptr() as *const c_char,
        b"fix_console_loglevel\0".as_ptr() as *const c_char,
    );
    *((*loglevel_state).data as *mut c_int) = console_loglevel;
    console_loglevel = CONSOLE_LOGLEVEL_MOTORMOUTH;
}

unsafe extern "C" fn restore_console_loglevel() {
    let loglevel_state: *mut klp_state;

    loglevel_state = klp_get_state(&mut patch, CONSOLE_LOGLEVEL_STATE);
    if loglevel_state.is_null() {
        return;
    }

    pr_info(
        b"%s: restoring console_loglevel\n\0".as_ptr() as *const c_char,
        b"restore_console_loglevel\0".as_ptr() as *const c_char,
    );
    console_loglevel = *((*loglevel_state).data as *mut c_int);
}

unsafe extern "C" fn free_loglevel_state() {
    let loglevel_state: *mut klp_state;

    loglevel_state = klp_get_state(&mut patch, CONSOLE_LOGLEVEL_STATE);
    if loglevel_state.is_null() {
        return;
    }

    pr_info(
        b"%s: freeing space for the stored console_loglevel\n\0".as_ptr() as *const c_char,
        b"free_loglevel_state\0".as_ptr() as *const c_char,
    );
    kfree((*loglevel_state).data);
}

/* Executed on object patching (ie, patch enablement) */
unsafe extern "C" fn pre_patch_callback(obj: *mut klp_object) -> c_int {
    callback_info(b"pre_patch_callback\0".as_ptr() as *const c_char, obj);
    allocate_loglevel_state()
}

/* Executed on object unpatching (ie, patch disablement) */
unsafe extern "C" fn post_patch_callback(obj: *mut klp_object) {
    callback_info(b"post_patch_callback\0".as_ptr() as *const c_char, obj);
    fix_console_loglevel();
}

/* Executed on object unpatching (ie, patch disablement) */
unsafe extern "C" fn pre_unpatch_callback(obj: *mut klp_object) {
    callback_info(b"pre_unpatch_callback\0".as_ptr() as *const c_char, obj);
    restore_console_loglevel();
}

/* Executed on object unpatching (ie, patch disablement) */
unsafe extern "C" fn post_unpatch_callback(obj: *mut klp_object) {
    callback_info(b"post_unpatch_callback\0".as_ptr() as *const c_char, obj);
    free_loglevel_state();
}

static mut no_funcs: [klp_func; 1] = [
    klp_func {
        _unused: [],
    },
];

static mut objs: [klp_object; 2] = [
    klp_object {
        name: ptr::null(), /* vmlinux */
        funcs: unsafe { no_funcs.as_mut_ptr() },
        callbacks: klp_callbacks {
            pre_patch: Some(pre_patch_callback),
            post_patch: Some(post_patch_callback),
            pre_unpatch: Some(pre_unpatch_callback),
            post_unpatch: Some(post_unpatch_callback),
        },
        mod_: ptr::null_mut(),
    },
    klp_object {
        name: ptr::null(),
        funcs: ptr::null_mut(),
        callbacks: klp_callbacks {
            pre_patch: None,
            post_patch: None,
            pre_unpatch: None,
            post_unpatch: None,
        },
        mod_: ptr::null_mut(),
    },
];

static mut states: [klp_state; 2] = [
    klp_state {
        id: CONSOLE_LOGLEVEL_STATE,
        version: CONSOLE_LOGLEVEL_STATE_VERSION,
        data: ptr::null_mut(),
    },
    klp_state {
        id: 0,
        version: 0,
        data: ptr::null_mut(),
    },
];

static mut patch: klp_patch = klp_patch {
    mod_: unsafe { THIS_MODULE },
    objs: unsafe { objs.as_mut_ptr() },
    states: unsafe { states.as_mut_ptr() },
    replace: true,
};

unsafe extern "C" fn test_klp_callbacks_demo_init() -> c_int {
    klp_enable_patch(&mut patch)
}

unsafe extern "C" fn test_klp_callbacks_demo_exit() {}

// module_init(test_klp_callbacks_demo_init);
// module_exit(test_klp_callbacks_demo_exit);
// MODULE_LICENSE("GPL");
// MODULE_INFO(livepatch, "Y");
// MODULE_AUTHOR("Petr Mladek <pmladek@suse.com>");
// MODULE_DESCRIPTION("Livepatch test: system state modification");
