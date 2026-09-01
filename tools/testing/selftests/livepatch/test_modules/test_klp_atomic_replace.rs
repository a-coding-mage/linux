// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Joe Lawrence <joe.lawrence@redhat.com>

// C pr_fmt(fmt): KBUILD_MODNAME ": " fmt
// Original dependencies: <linux/module.h>, <linux/kernel.h>,
// <linux/livepatch.h>, <linux/seq_file.h>

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct module {
    pub name: *const c_char,
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct klp_func {
    pub old_name: *const c_char,
    pub new_func: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int>,
}

#[repr(C)]
pub struct klp_object {
    pub funcs: *mut klp_func,
}

#[repr(C)]
pub struct klp_patch {
    pub mod_: *mut module,
    pub objs: *mut klp_object,
    pub replace: c_int,
}

unsafe extern "C" {
    #[link_name = "__this_module"]
    static mut THIS_MODULE: module;

    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...) -> c_int;
    fn klp_enable_patch(patch: *mut klp_patch) -> c_int;
}

static mut replace: c_int = 0;
// module_param(replace, int, 0644);
// MODULE_PARM_DESC(replace, "replace (default=0)");

unsafe extern "C" fn livepatch_meminfo_proc_show(
    m: *mut seq_file,
    _v: *mut c_void,
) -> c_int {
    unsafe {
        seq_printf(
            m,
            c"%s: %s\n".as_ptr(),
            THIS_MODULE.name,
            c"this has been live patched".as_ptr(),
        );
    }

    0
}

#[used]
static mut funcs: [klp_func; 2] = [
    klp_func {
        old_name: c"meminfo_proc_show".as_ptr(),
        new_func: Some(livepatch_meminfo_proc_show),
    },
    klp_func {
        old_name: core::ptr::null(),
        new_func: None,
    },
];

#[used]
static mut objs: [klp_object; 2] = [
    klp_object {
        // name being NULL means vmlinux
        funcs: unsafe { core::ptr::addr_of_mut!(funcs) as *mut klp_func },
    },
    klp_object {
        funcs: core::ptr::null_mut(),
    },
];

#[used]
static mut patch: klp_patch = klp_patch {
    mod_: unsafe { core::ptr::addr_of_mut!(THIS_MODULE) },
    objs: unsafe { core::ptr::addr_of_mut!(objs) as *mut klp_object },
    // set .replace in the init function below for demo purposes
    replace: 0,
};

unsafe extern "C" fn test_klp_atomic_replace_init() -> c_int {
    unsafe {
        patch.replace = replace;
        klp_enable_patch(core::ptr::addr_of_mut!(patch))
    }
}

unsafe extern "C" fn test_klp_atomic_replace_exit() {}

// module_init(test_klp_atomic_replace_init);
// module_exit(test_klp_atomic_replace_exit);
// MODULE_LICENSE("GPL");
// MODULE_INFO(livepatch, "Y");
// MODULE_AUTHOR("Joe Lawrence <joe.lawrence@redhat.com>");
// MODULE_DESCRIPTION("Livepatch test: atomic replace");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
