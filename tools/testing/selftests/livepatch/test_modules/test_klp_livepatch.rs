// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2014 Seth Jennings <sjenning@redhat.com>

use core::ffi::{c_char, c_int, c_void};

// C source included:
// #include <linux/module.h>
// #include <linux/kernel.h>
// #include <linux/livepatch.h>
// #include <linux/seq_file.h>
//
// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt

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
    pub new_func: Option<unsafe extern "C" fn(m: *mut seq_file, v: *mut c_void) -> c_int>,
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

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;

    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...) -> c_int;
    fn klp_enable_patch(patch: *mut klp_patch) -> c_int;
}

const LIVEPATCH_CMDLINE_PROC_SHOW_TEXT: *const c_char =
    b"this has been live patched\0".as_ptr() as *const c_char;
const LIVEPATCH_CMDLINE_PROC_SHOW_FMT: *const c_char = b"%s: %s\n\0".as_ptr() as *const c_char;
const CMDLINE_PROC_SHOW_NAME: *const c_char = b"cmdline_proc_show\0".as_ptr() as *const c_char;

unsafe extern "C" fn livepatch_cmdline_proc_show(
    m: *mut seq_file,
    _v: *mut c_void,
) -> c_int {
    unsafe {
        seq_printf(
            m,
            LIVEPATCH_CMDLINE_PROC_SHOW_FMT,
            (*THIS_MODULE).name,
            LIVEPATCH_CMDLINE_PROC_SHOW_TEXT,
        );
    }
    0
}

static mut FUNCS: [klp_func; 2] = [
    klp_func {
        old_name: CMDLINE_PROC_SHOW_NAME,
        new_func: Some(livepatch_cmdline_proc_show),
    },
    klp_func {
        old_name: core::ptr::null(),
        new_func: None,
    },
];

static mut OBJS: [klp_object; 2] = [
    klp_object {
        /* name being NULL means vmlinux */
        name: core::ptr::null(),
        funcs: unsafe { FUNCS.as_mut_ptr() },
    },
    klp_object {
        name: core::ptr::null(),
        funcs: core::ptr::null_mut(),
    },
];

static mut PATCH: klp_patch = klp_patch {
    mod_: unsafe { THIS_MODULE },
    objs: unsafe { OBJS.as_mut_ptr() },
};

unsafe extern "C" fn test_klp_livepatch_init() -> c_int {
    unsafe { klp_enable_patch(&raw mut PATCH) }
}

unsafe extern "C" fn test_klp_livepatch_exit() {}

// module_init(test_klp_livepatch_init);
// module_exit(test_klp_livepatch_exit);
// MODULE_LICENSE("GPL");
// MODULE_INFO(livepatch, "Y");
// MODULE_AUTHOR("Seth Jennings <sjenning@redhat.com>");
// MODULE_DESCRIPTION("Livepatch test: livepatch module");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
