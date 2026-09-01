// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2026 Pablo Hugen <phugen@redhat.com>

// pr_fmt(fmt) was defined in C as: KBUILD_MODNAME ": " fmt

// C includes translated as external dependency intent:
// <linux/module.h>
// <linux/kernel.h>
// <linux/livepatch.h>
// <linux/seq_file.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    pub name: *const c_char,
}

#[repr(C)]
pub struct klp_func {
    pub old_name: *const c_char,
    pub new_func: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int>,
}

#[repr(C)]
pub struct klp_object {
    pub name: *const c_char,
    pub funcs: *mut klp_func,
}

#[repr(C)]
pub struct klp_patch {
    pub mod_: *mut module,
    pub objs: *mut klp_object,
}

extern "C" {
    static mut THIS_MODULE: *mut module;

    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...) -> c_int;
    fn klp_enable_patch(patch: *mut klp_patch) -> c_int;
}

unsafe extern "C" fn livepatch_mod_target_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    seq_printf(
        m,
        b"%s: %s\n\0".as_ptr() as *const c_char,
        (*THIS_MODULE).name,
        b"this has been live patched\0".as_ptr() as *const c_char,
    );
    0
}

static mut funcs: [klp_func; 2] = [
    klp_func {
        old_name: b"test_klp_mod_target_show\0".as_ptr() as *const c_char,
        new_func: Some(livepatch_mod_target_show),
    },
    klp_func {
        old_name: core::ptr::null(),
        new_func: None,
    },
];

static mut objs: [klp_object; 2] = [
    klp_object {
        name: b"test_klp_mod_target\0".as_ptr() as *const c_char,
        funcs: unsafe { funcs.as_mut_ptr() },
    },
    klp_object {
        name: core::ptr::null(),
        funcs: core::ptr::null_mut(),
    },
];

static mut patch: klp_patch = klp_patch {
    mod_: unsafe { THIS_MODULE },
    objs: unsafe { objs.as_mut_ptr() },
};

unsafe extern "C" fn test_klp_mod_patch_init() -> c_int {
    klp_enable_patch(core::ptr::addr_of_mut!(patch))
}

unsafe extern "C" fn test_klp_mod_patch_exit() {}

// module_init(test_klp_mod_patch_init);
// module_exit(test_klp_mod_patch_exit);
// MODULE_LICENSE("GPL");
// MODULE_INFO(livepatch, "Y");
// MODULE_AUTHOR("Pablo Hugen <phugen@redhat.com>");
// MODULE_DESCRIPTION("Livepatch test: patch for module-provided function");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
