// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * livepatch-sample.c - Kernel Live Patching Sample Module
 *
 * Copyright (C) 2014 Seth Jennings <sjenning@redhat.com>
 */

// pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// Kernel dependencies supplied by the surrounding build are intentionally
// referenced here rather than reimplemented.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct klp_func {
    pub old_name: *const c_char,
    pub new_func: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> c_int>,
}

#[repr(C)]
pub struct klp_object {
    pub name: *const c_char,
    pub funcs: *mut klp_func,
}

#[repr(C)]
pub struct klp_patch {
    pub mod_: *mut c_void,
    pub objs: *mut klp_object,
}

extern "C" {
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...) -> c_int;
    fn klp_enable_patch(patch: *mut klp_patch) -> c_int;
}

// THIS_MODULE is supplied by the kernel module environment.
extern "C" {
    static mut THIS_MODULE: c_void;
}

/*
 * This (dumb) live patch overrides the function that prints the
 * kernel boot cmdline when /proc/cmdline is read.
 *
 * Example:
 *
 * $ cat /proc/cmdline
 * <your cmdline>
 *
 * $ insmod livepatch-sample.ko
 * $ cat /proc/cmdline
 * this has been live patched
 *
 * $ echo 0 > /sys/kernel/livepatch/livepatch_sample/enabled
 * $ cat /proc/cmdline
 * <your cmdline>
 */

unsafe extern "C" fn livepatch_cmdline_proc_show(m: *mut c_void, _v: *mut c_void) -> c_int {
    static FORMAT: &[u8] = b"%s\n\0";
    static MESSAGE: &[u8] = b"this has been live patched\0";
    seq_printf(
        m as *mut seq_file,
        FORMAT.as_ptr() as *const c_char,
        MESSAGE.as_ptr() as *const c_char,
    );
    0
}

static mut funcs: [klp_func; 2] = [
    klp_func {
        old_name: b"cmdline_proc_show\0".as_ptr() as *const c_char,
        new_func: Some(livepatch_cmdline_proc_show),
    },
    klp_func {
        old_name: core::ptr::null(),
        new_func: None,
    },
];

static mut objs: [klp_object; 2] = [
    klp_object {
        // name being NULL means vmlinux
        name: core::ptr::null(),
        funcs: core::ptr::addr_of_mut!(funcs) as *mut klp_func,
    },
    klp_object {
        name: core::ptr::null(),
        funcs: core::ptr::null_mut(),
    },
];

static mut patch: klp_patch = klp_patch {
    mod_: core::ptr::addr_of_mut!(THIS_MODULE),
    objs: core::ptr::addr_of_mut!(objs) as *mut klp_object,
};

unsafe extern "C" fn livepatch_init() -> c_int {
    klp_enable_patch(core::ptr::addr_of_mut!(patch))
}

unsafe extern "C" fn livepatch_exit() {
}

// module_init(livepatch_init);
// module_exit(livepatch_exit);
// MODULE_DESCRIPTION("Kernel Live Patching Sample Module");
// MODULE_LICENSE("GPL");
// MODULE_INFO(livepatch, "Y");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
