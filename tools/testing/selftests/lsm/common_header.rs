/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Linux Security Module infrastructure tests
 *
 * Copyright © 2023 Casey Schaufler <casey@schaufler-ca.com>
 */

use core::ffi::{c_char, c_int, c_long, c_uint};

unsafe extern "C" {
    pub fn syscall(num: c_long, ...) -> c_long;

    pub fn read_proc_attr(attr: *const c_char, value: *mut c_char, size: size_t) -> c_int;
    pub fn read_sysfs_lsms(lsms: *mut c_char, size: size_t) -> c_int;
    pub fn attr_lsm_count() -> c_int;
}

/* C conditional intent: defined only when lsm_get_self_attr is not a macro. */
pub unsafe fn lsm_get_self_attr(
    attr: c_uint,
    ctx: *mut lsm_ctx,
    size: *mut __u32,
    flags: __u32,
) -> c_int {
    unsafe { syscall(__NR_lsm_get_self_attr as c_long, attr, ctx, size, flags) as c_int }
}

/* C conditional intent: defined only when lsm_set_self_attr is not a macro. */
pub unsafe fn lsm_set_self_attr(
    attr: c_uint,
    ctx: *mut lsm_ctx,
    size: __u32,
    flags: __u32,
) -> c_int {
    unsafe { syscall(__NR_lsm_set_self_attr as c_long, attr, ctx, size, flags) as c_int }
}

/* C conditional intent: defined only when lsm_list_modules is not a macro. */
pub unsafe fn lsm_list_modules(ids: *mut __u64, size: *mut __u32, flags: __u32) -> c_int {
    unsafe { syscall(__NR_lsm_list_modules as c_long, ids, size, flags) as c_int }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
