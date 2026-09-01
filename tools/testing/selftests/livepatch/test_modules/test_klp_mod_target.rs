// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2026 Pablo Hugen <phugen@redhat.com>

// pr_fmt(fmt) was defined as: KBUILD_MODNAME ": " fmt
// Dependencies in the C source:
// #include <linux/module.h>
// #include <linux/kernel.h>
// #include <linux/proc_fs.h>
// #include <linux/seq_file.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct proc_dir_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    pub name: *const c_char,
}

const ENOMEM: c_int = 12;

static mut pde: *mut proc_dir_entry = core::ptr::null_mut();

unsafe extern "C" {
    static THIS_MODULE: *mut module;

    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...) -> c_int;
    fn proc_create_single(
        name: *const c_char,
        mode: c_uint,
        parent: *mut proc_dir_entry,
        show: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int>,
    ) -> *mut proc_dir_entry;
    fn proc_remove(de: *mut proc_dir_entry);
    fn pr_info(fmt: *const c_char, ...);
}

#[inline(never)]
unsafe extern "C" fn test_klp_mod_target_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    unsafe {
        seq_printf(
            m,
            c"%s: %s\n".as_ptr(),
            (*THIS_MODULE).name,
            c"original output".as_ptr(),
        );
    }
    0
}

unsafe extern "C" fn test_klp_mod_target_init() -> c_int {
    unsafe {
        pr_info(c"%s\n".as_ptr(), c"test_klp_mod_target_init".as_ptr());
        pde = proc_create_single(
            c"test_klp_mod_target".as_ptr(),
            0,
            core::ptr::null_mut(),
            Some(test_klp_mod_target_show),
        );
        if pde.is_null() {
            return -ENOMEM;
        }
    }
    0
}

unsafe extern "C" fn test_klp_mod_target_exit() {
    unsafe {
        pr_info(c"%s\n".as_ptr(), c"test_klp_mod_target_exit".as_ptr());
        proc_remove(pde);
    }
}

// module_init(test_klp_mod_target_init);
// module_exit(test_klp_mod_target_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Pablo Hugen <phugen@redhat.com>");
// MODULE_DESCRIPTION("Livepatch test: target module with proc entry");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
