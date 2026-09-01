// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

/* Dependencies from the original C source:
 * #include <vmlinux.h>
 * #include <bpf/bpf_helpers.h>
 * #include <bpf/bpf_tracing.h>
 * #include <errno.h>
 */

pub type __u64 = u64;
pub type __s64 = i64;

pub const EINVAL: i32 = 22;
pub const EOPNOTSUPP: i32 = 95;

unsafe extern "C" {
    #[linkage = "extern_weak"]
    pub fn bpf_kfunc_implicit_arg(a: i32) -> i32;

    pub fn bpf_get_func_arg_cnt(ctx: *mut core::ffi::c_void) -> __u64;
    pub fn bpf_get_func_arg(ctx: *mut core::ffi::c_void, n: __u32, value: *mut __u64) -> __s64;
    pub fn bpf_get_func_ret(ctx: *mut core::ffi::c_void, value: *mut __u64) -> __s64;
}

pub type __u32 = u32;

/* SEC("license") */
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

/* Shared arg checks; reports arg count and aux, returns 1 on success. */
#[inline(always)]
unsafe fn check_implicit_args(
    ctx: *mut core::ffi::c_void,
    arg_cnt: *mut __u64,
    aux_arg: *mut __u64,
) -> __u64 {
    let mut a: __u64 = 0;
    let mut aux: __u64 = 0;
    let mut z: __u64 = 0;
    let mut result: __u64;
    let mut err: __s64;

    *arg_cnt = bpf_get_func_arg_cnt(ctx);
    result = (*arg_cnt == 2) as __u64;

    err = bpf_get_func_arg(ctx, 0, &mut a);
    result &= (err == 0 && a as i32 == 5) as __u64;

    err = bpf_get_func_arg(ctx, 1, &mut aux);
    *aux_arg = aux;
    result &= (err == 0 && aux != 0) as __u64;

    err = bpf_get_func_arg(ctx, 2, &mut z);
    result &= (err == -(EINVAL as __s64)) as __u64;

    result
}

#[no_mangle]
pub static mut fentry_result: __u64 = 0;
#[no_mangle]
pub static mut fentry_arg_cnt: __u64 = 0;
#[no_mangle]
pub static mut fentry_aux_arg: __u64 = 0;

/* SEC("fentry/bpf_kfunc_implicit_arg") */
#[no_mangle]
pub unsafe extern "C" fn trace_implicit_arg_fentry(ctx: *mut core::ffi::c_void) -> i32 {
    let mut ret: __u64 = 0;
    let mut err: __s64;

    fentry_result = check_implicit_args(ctx, &mut fentry_arg_cnt, &mut fentry_aux_arg);

    err = bpf_get_func_ret(ctx, &mut ret);
    fentry_result &= (err == -(EOPNOTSUPP as __s64)) as __u64;

    0
}

#[no_mangle]
pub static mut fexit_result: __u64 = 0;
#[no_mangle]
pub static mut fexit_arg_cnt: __u64 = 0;
#[no_mangle]
pub static mut fexit_aux_arg: __u64 = 0;

/* SEC("fexit/bpf_kfunc_implicit_arg") */
#[no_mangle]
pub unsafe extern "C" fn trace_implicit_arg_fexit(ctx: *mut core::ffi::c_void) -> i32 {
    let mut ret: __u64 = 0;
    let mut err: __s64;

    fexit_result = check_implicit_args(ctx, &mut fexit_arg_cnt, &mut fexit_aux_arg);

    err = bpf_get_func_ret(ctx, &mut ret);
    fexit_result &= (err == 0 && ret == 5) as __u64;

    0
}

/* SEC("syscall") */
#[no_mangle]
pub unsafe extern "C" fn trigger_implicit_arg(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;

    bpf_kfunc_implicit_arg(5)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
