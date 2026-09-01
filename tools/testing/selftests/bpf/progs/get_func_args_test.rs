// SPDX-License-Identifier: GPL-2.0
// C dependencies: vmlinux.h, bpf/bpf_helpers.h, bpf/bpf_tracing.h, errno.h

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

type __u64 = u64;
type __s64 = i64;

const EINVAL: __s64 = 22;
const EOPNOTSUPP: __s64 = 95;

extern "C" {
    fn bpf_get_func_arg_cnt(ctx: *mut core::ffi::c_void) -> __u64;
    fn bpf_get_func_arg(ctx: *mut core::ffi::c_void, n: __u64, value: *mut __u64) -> __s64;
    fn bpf_get_func_ret(ctx: *mut core::ffi::c_void, value: *mut __u64) -> __s64;
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut test1_result: __u64 = 0;

#[link_section = "fentry/bpf_fentry_test1"]
#[no_mangle]
pub unsafe extern "C" fn test1(ctx: *mut core::ffi::c_void) -> i32 {
    let cnt: __u64 = bpf_get_func_arg_cnt(ctx);
    let mut a: __u64 = 0;
    let mut z: __u64 = 0;
    let mut ret: __u64 = 0;
    let mut err: __s64;

    test1_result = (cnt == 1) as __u64;

    /* valid arguments */
    err = bpf_get_func_arg(ctx, 0, &mut a);

    /* We need to cast access to traced function argument values with
     * proper type cast, because trampoline uses type specific instruction
     * to save it, like for 'int a' with 32-bit mov like:
     *
     *   mov %edi,-0x8(%rbp)
     *
     * so the upper 4 bytes are not zeroed.
     */
    test1_result &= (err == 0 && (a as i32) == 1) as __u64;

    /* not valid argument */
    err = bpf_get_func_arg(ctx, 1, &mut z);
    test1_result &= (err == -EINVAL) as __u64;

    /* return value fails in fentry */
    err = bpf_get_func_ret(ctx, &mut ret);
    test1_result &= (err == -EOPNOTSUPP) as __u64;
    0
}

#[no_mangle]
pub static mut test2_result: __u64 = 0;

#[link_section = "fexit/bpf_fentry_test2"]
#[no_mangle]
pub unsafe extern "C" fn test2(ctx: *mut core::ffi::c_void) -> i32 {
    let cnt: __u64 = bpf_get_func_arg_cnt(ctx);
    let mut a: __u64 = 0;
    let mut b: __u64 = 0;
    let mut z: __u64 = 0;
    let mut ret: __u64 = 0;
    let mut err: __s64;

    test2_result = (cnt == 2) as __u64;

    /* valid arguments */
    err = bpf_get_func_arg(ctx, 0, &mut a);
    test2_result &= (err == 0 && (a as i32) == 2) as __u64;

    err = bpf_get_func_arg(ctx, 1, &mut b);
    test2_result &= (err == 0 && b == 3) as __u64;

    /* not valid argument */
    err = bpf_get_func_arg(ctx, 2, &mut z);
    test2_result &= (err == -EINVAL) as __u64;

    /* return value */
    err = bpf_get_func_ret(ctx, &mut ret);
    test2_result &= (err == 0 && ret == 5) as __u64;
    0
}

#[no_mangle]
pub static mut test3_result: __u64 = 0;

#[link_section = "fmod_ret/bpf_modify_return_test"]
#[no_mangle]
pub unsafe extern "C" fn fmod_ret_test(
    ctx: *mut core::ffi::c_void,
    _a: i32,
    _b: *mut i32,
    _ret: i32,
) -> i32 {
    let cnt: __u64 = bpf_get_func_arg_cnt(ctx);
    let mut a: __u64 = 0;
    let mut b: __u64 = 0;
    let mut z: __u64 = 0;
    let mut ret: __u64 = 0;
    let mut err: __s64;

    test3_result = (cnt == 2) as __u64;

    /* valid arguments */
    err = bpf_get_func_arg(ctx, 0, &mut a);
    test3_result &= (err == 0 && (a as i32) == 1) as __u64;

    err = bpf_get_func_arg(ctx, 1, &mut b);
    test3_result &= (err == 0 && (b as *mut i32) == _b) as __u64;

    /* not valid argument */
    err = bpf_get_func_arg(ctx, 2, &mut z);
    test3_result &= (err == -EINVAL) as __u64;

    /* return value */
    err = bpf_get_func_ret(ctx, &mut ret);
    test3_result &= (err == 0 && ret == 0) as __u64;

    /* change return value, it's checked in fexit_test program */
    1234
}

#[no_mangle]
pub static mut test4_result: __u64 = 0;

#[link_section = "fexit/bpf_modify_return_test"]
#[no_mangle]
pub unsafe extern "C" fn fexit_test(
    ctx: *mut core::ffi::c_void,
    _a: i32,
    _b: *mut i32,
    _ret: i32,
) -> i32 {
    let cnt: __u64 = bpf_get_func_arg_cnt(ctx);
    let mut a: __u64 = 0;
    let mut b: __u64 = 0;
    let mut z: __u64 = 0;
    let mut ret: __u64 = 0;
    let mut err: __s64;

    test4_result = (cnt == 2) as __u64;

    /* valid arguments */
    err = bpf_get_func_arg(ctx, 0, &mut a);
    test4_result &= (err == 0 && (a as i32) == 1) as __u64;

    err = bpf_get_func_arg(ctx, 1, &mut b);
    test4_result &= (err == 0 && (b as *mut i32) == _b) as __u64;

    /* not valid argument */
    err = bpf_get_func_arg(ctx, 2, &mut z);
    test4_result &= (err == -EINVAL) as __u64;

    /* return value */
    err = bpf_get_func_ret(ctx, &mut ret);
    test4_result &= (err == 0 && ret == 1234) as __u64;
    0
}

#[no_mangle]
pub static mut test5_result: __u64 = 0;

#[link_section = "tp_btf/bpf_testmod_fentry_test1_tp"]
#[no_mangle]
pub unsafe extern "C" fn tp_test1(ctx: *mut core::ffi::c_void) -> i32 {
    let cnt: __u64 = bpf_get_func_arg_cnt(ctx);
    let mut a: __u64 = 0;
    let mut z: __u64 = 0;
    let mut err: __s64;

    test5_result = (cnt == 1) as __u64;

    err = bpf_get_func_arg(ctx, 0, &mut a);
    test5_result &= (err == 0 && (a as i32) == 1) as __u64;

    /* not valid argument */
    err = bpf_get_func_arg(ctx, 1, &mut z);
    test5_result &= (err == -EINVAL) as __u64;

    0
}

#[no_mangle]
pub static mut test6_result: __u64 = 0;

#[link_section = "tp_btf/bpf_testmod_fentry_test2_tp"]
#[no_mangle]
pub unsafe extern "C" fn tp_test2(ctx: *mut core::ffi::c_void) -> i32 {
    let cnt: __u64 = bpf_get_func_arg_cnt(ctx);
    let mut a: __u64 = 0;
    let mut b: __u64 = 0;
    let mut z: __u64 = 0;
    let mut err: __s64;

    test6_result = (cnt == 2) as __u64;

    /* valid arguments */
    err = bpf_get_func_arg(ctx, 0, &mut a);
    test6_result &= (err == 0 && (a as i32) == 2) as __u64;

    err = bpf_get_func_arg(ctx, 1, &mut b);
    test6_result &= (err == 0 && b == 3) as __u64;

    /* not valid argument */
    err = bpf_get_func_arg(ctx, 2, &mut z);
    test6_result &= (err == -EINVAL) as __u64;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
