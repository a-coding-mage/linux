// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017 Joe Lawrence <joe.lawrence@redhat.com>
 */

/*
 * livepatch-callbacks-busymod.c - (un)patching callbacks demo support module
 *
 * Purpose
 * -------
 *
 * Simple module to demonstrate livepatch (un)patching callbacks.
 *
 * This module is not intended to be standalone.  See the "Usage"
 * section of livepatch-callbacks-mod.c.
 */

use core::ffi::{c_char, c_int, c_void};

// The following declarations are supplied by the kernel headers/build.
#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    _private: [u8; 0],
}

extern "C" {
    fn pr_info(fmt: *const c_char, ...);
    fn msleep(msecs: c_uint);
    fn schedule_delayed_work(work: *mut delayed_work, delay: c_ulong) -> c_int;
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> c_int;
}

type c_uint = u32;
type c_ulong = usize;

static mut sleep_secs: c_int = 0;

// module_param(sleep_secs, int, 0644);
// MODULE_PARM_DESC(sleep_secs, "sleep_secs (default=0)");

unsafe extern "C" fn busymod_work_func(_work: *mut work_struct);

// DECLARE_DELAYED_WORK(work, busymod_work_func);
static mut work: delayed_work = delayed_work { _private: [] };

unsafe extern "C" fn busymod_work_func(_work: *mut work_struct) {
    pr_info(
        b"%s, sleeping %d seconds ...\n\0".as_ptr() as *const c_char,
        b"busymod_work_func\0".as_ptr() as *const c_char,
        sleep_secs,
    );
    msleep((sleep_secs * 1000) as c_uint);
    pr_info(
        b"%s exit\n\0".as_ptr() as *const c_char,
        b"busymod_work_func\0".as_ptr() as *const c_char,
    );
}

unsafe extern "C" fn livepatch_callbacks_mod_init() -> c_int {
    pr_info(
        b"%s\n\0".as_ptr() as *const c_char,
        b"livepatch_callbacks_mod_init\0".as_ptr() as *const c_char,
    );
    schedule_delayed_work(&mut work, 0);
    0
}

unsafe extern "C" fn livepatch_callbacks_mod_exit() {
    cancel_delayed_work_sync(&mut work);
    pr_info(
        b"%s\n\0".as_ptr() as *const c_char,
        b"livepatch_callbacks_mod_exit\0".as_ptr() as *const c_char,
    );
}

// module_init(livepatch_callbacks_mod_init);
// module_exit(livepatch_callbacks_mod_exit);
// MODULE_DESCRIPTION("Live patching demo for (un)patching callbacks, support module");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
