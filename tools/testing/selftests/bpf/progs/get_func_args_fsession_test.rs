// SPDX-License-Identifier: GPL-2.0
// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>, <errno.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

type __u64 = u64;
type __s64 = i64;

const EINVAL: i32 = 22;

unsafe extern "C" {
    fn bpf_get_func_arg_cnt(ctx: *mut core::ffi::c_void) -> __u64;
    fn bpf_get_func_arg(ctx: *mut core::ffi::c_void, n: __u64, value: *mut __u64) -> __s64;
    fn bpf_get_func_ret(ctx: *mut core::ffi::c_void, value: *mut __u64) -> __s64;
    fn bpf_session_is_return(ctx: *mut core::ffi::c_void) -> i32;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut test1_result: __u64 = 0;

#[unsafe(link_section = "fsession/bpf_fentry_test1")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test1(ctx: *mut core::ffi::c_void) -> i32 {
    let cnt: __u64 = unsafe { bpf_get_func_arg_cnt(ctx) };
    let mut a: __u64 = 0;
    let mut z: __u64 = 0;
    let mut ret: __u64 = 0;
    let mut err: __s64;

    unsafe {
        test1_result = (cnt == 1) as __u64;
    }

    /* valid arguments */
    err = unsafe { bpf_get_func_arg(ctx, 0, &mut a) };
    unsafe {
        test1_result &= (err == 0 && (a as i32) == 1) as __u64;
    }

    /* not valid argument */
    err = unsafe { bpf_get_func_arg(ctx, 1, &mut z) };
    unsafe {
        test1_result &= (err == -(EINVAL as __s64)) as __u64;
    }

    if unsafe { bpf_session_is_return(ctx) } != 0 {
        err = unsafe { bpf_get_func_ret(ctx, &mut ret) };
        unsafe {
            test1_result &= (err == 0 && ret == 2) as __u64;
        }
    } else {
        err = unsafe { bpf_get_func_ret(ctx, &mut ret) };
        unsafe {
            test1_result &= (err == 0 && ret == 0) as __u64;
        }
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
