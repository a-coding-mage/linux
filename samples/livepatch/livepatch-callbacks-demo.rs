// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017 Joe Lawrence <joe.lawrence@redhat.com>
 */

/*
 * livepatch-callbacks-demo.c - (un)patching callbacks livepatch demo
 *
 * Demonstration of registering livepatch (un)patching callbacks.
 *
 * The original module usage and livepatch experimentation instructions are
 * retained in the source comments above the corresponding C implementation.
 */

use core::ffi::{c_char, c_int, c_void};

// Supplied by the Linux kernel livepatch/module interfaces.
#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    pub name: *const c_char,
    pub state: c_int,
}

#[repr(C)]
pub struct klp_callbacks {
    pub pre_patch: Option<unsafe extern "C" fn(*mut klp_object) -> c_int>,
    pub post_patch: Option<unsafe extern "C" fn(*mut klp_object)>,
    pub pre_unpatch: Option<unsafe extern "C" fn(*mut klp_object)>,
    pub post_unpatch: Option<unsafe extern "C" fn(*mut klp_object)>,
}

#[repr(C)]
pub struct klp_func {
    pub old_name: *const c_char,
    pub new_func: Option<unsafe extern "C" fn(*mut work_struct)>,
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

unsafe extern "C" {
    fn klp_enable_patch(patch: *mut klp_patch) -> c_int;
    fn pr_info(fmt: *const c_char, ...);
}

static mut PRE_PATCH_RET: c_int = 0;

// module_param(pre_patch_ret, int, 0644);
// MODULE_PARM_DESC(pre_patch_ret, "pre_patch_ret (default=0)");

static MODULE_STATE: [&[u8]; 4] = [
    b"[MODULE_STATE_LIVE] Normal state\0",
    b"[MODULE_STATE_COMING] Full formed, running module_init\0",
    b"[MODULE_STATE_GOING] Going away\0",
    b"[MODULE_STATE_UNFORMED] Still setting it up\0",
];

unsafe extern "C" fn callback_info(callback: *const c_char, obj: *mut klp_object) {
    if !(*obj).mod_.is_null() {
        let module = (*obj).mod_;
        pr_info(
            b"%s: %s -> %s\n\0".as_ptr() as *const c_char,
            callback,
            (*module).name,
            MODULE_STATE[(*module).state as usize].as_ptr() as *const c_char,
        );
    } else {
        pr_info(b"%s: vmlinux\n\0".as_ptr() as *const c_char, callback);
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
    pr_info(b"patched_work_func\n\0".as_ptr() as *const c_char);
}

static mut NO_FUNCS: [klp_func; 1] = [klp_func {
    old_name: core::ptr::null(),
    new_func: None,
}];

static mut BUSYMOD_FUNCS: [klp_func; 2] = [
    klp_func {
        old_name: b"busymod_work_func\0".as_ptr() as *const c_char,
        new_func: Some(patched_work_func),
    },
    klp_func {
        old_name: core::ptr::null(),
        new_func: None,
    },
];

static mut OBJS: [klp_object; 4] = [
    klp_object {
        name: core::ptr::null(),
        funcs: core::ptr::addr_of_mut!(NO_FUNCS) as *mut klp_func,
        callbacks: klp_callbacks {
            pre_patch: Some(pre_patch_callback),
            post_patch: Some(post_patch_callback),
            pre_unpatch: Some(pre_unpatch_callback),
            post_unpatch: Some(post_unpatch_callback),
        },
        mod_: core::ptr::null_mut(),
    },
    klp_object {
        name: b"livepatch_callbacks_mod\0".as_ptr() as *const c_char,
        funcs: core::ptr::addr_of_mut!(NO_FUNCS) as *mut klp_func,
        callbacks: klp_callbacks {
            pre_patch: Some(pre_patch_callback), post_patch: Some(post_patch_callback),
            pre_unpatch: Some(pre_unpatch_callback), post_unpatch: Some(post_unpatch_callback),
        }, mod_: core::ptr::null_mut(),
    },
    klp_object {
        name: b"livepatch_callbacks_busymod\0".as_ptr() as *const c_char,
        funcs: core::ptr::addr_of_mut!(BUSYMOD_FUNCS) as *mut klp_func,
        callbacks: klp_callbacks {
            pre_patch: Some(pre_patch_callback), post_patch: Some(post_patch_callback),
            pre_unpatch: Some(pre_unpatch_callback), post_unpatch: Some(post_unpatch_callback),
        }, mod_: core::ptr::null_mut(),
    },
    klp_object { name: core::ptr::null(), funcs: core::ptr::null_mut(), callbacks: klp_callbacks { pre_patch: None, post_patch: None, pre_unpatch: None, post_unpatch: None }, mod_: core::ptr::null_mut() },
];

static mut PATCH: klp_patch = klp_patch {
    // .mod = THIS_MODULE
    mod_: core::ptr::null_mut(),
    objs: core::ptr::addr_of_mut!(OBJS) as *mut klp_object,
};

unsafe extern "C" fn livepatch_callbacks_demo_init() -> c_int {
    klp_enable_patch(core::ptr::addr_of_mut!(PATCH))
}

unsafe extern "C" fn livepatch_callbacks_demo_exit() {}

// module_init(livepatch_callbacks_demo_init);
// module_exit(livepatch_callbacks_demo_exit);
// MODULE_DESCRIPTION("Live patching demo for (un)patching callbacks");
// MODULE_LICENSE("GPL");
// MODULE_INFO(livepatch, "Y");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
