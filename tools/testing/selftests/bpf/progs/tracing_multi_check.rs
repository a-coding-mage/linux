// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/bpf/progs/tracing_multi_check.c
// C includes removed: vmlinux.h, bpf/bpf_helpers.h, bpf/bpf_tracing.h.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type __u64 = u64;

// SEC("license")
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut pid: i32 = 0;
#[no_mangle]
pub static mut test_cookies: bool = false;

extern "C" {
    fn bpf_get_func_ip(ctx: *mut __u64) -> __u64;
    fn bpf_get_current_pid_tgid() -> __u64;
    fn bpf_get_func_ret(ctx: *mut __u64, ret: *mut __u64) -> i64;
    fn bpf_get_attach_cookie(ctx: *mut __u64) -> __u64;
    fn bpf_get_func_arg(ctx: *mut __u64, n: u32, value: *mut __u64) -> i64;

    /* bpf_fentry_test1 is exported as kfunc via vmlinux.h */
    static bpf_fentry_test1: core::ffi::c_void;
    static bpf_fentry_test2: core::ffi::c_void;
    static bpf_fentry_test3: core::ffi::c_void;
    static bpf_fentry_test4: core::ffi::c_void;
    static bpf_fentry_test5: core::ffi::c_void;
    static bpf_fentry_test6: core::ffi::c_void;
    static bpf_fentry_test7: core::ffi::c_void;
    static bpf_fentry_test8: core::ffi::c_void;
    static bpf_fentry_test9: core::ffi::c_void;
    static bpf_fentry_test10: core::ffi::c_void;

    static bpf_testmod_fentry_test1: core::ffi::c_void;
    static bpf_testmod_fentry_test2: core::ffi::c_void;
    static bpf_testmod_fentry_test3: core::ffi::c_void;
    static bpf_testmod_fentry_test7: core::ffi::c_void;
    static bpf_testmod_fentry_test11: core::ffi::c_void;
}

#[no_mangle]
pub unsafe extern "C" fn tracing_multi_arg_check(
    ctx: *mut __u64,
    test_result: *mut __u64,
    is_return: bool,
) -> i32 {
    let ip: *mut core::ffi::c_void = bpf_get_func_ip(ctx) as *mut core::ffi::c_void;
    let mut value: __u64 = 0;
    let mut ret: __u64 = 0;
    let mut cookie: __u64 = 0;
    let mut err: i64 = 0;

    if (bpf_get_current_pid_tgid() >> 32) as i32 != pid {
        return 1;
    }

    if is_return {
        err |= bpf_get_func_ret(ctx, &mut ret);
    }
    if test_cookies {
        cookie = bpf_get_attach_cookie(ctx);
    }

    if ip == (&bpf_fentry_test1 as *const core::ffi::c_void as *mut core::ffi::c_void) {
        let a: i32;

        err |= bpf_get_func_arg(ctx, 0, &mut value);
        a = value as i32;

        err |= if is_return { (ret != 2) as i64 } else { 0 };
        err |= if test_cookies { (cookie != 8) as i64 } else { 0 };

        *test_result = (*test_result).wrapping_add((err == 0 && a == 1) as __u64);
    } else if ip == (&bpf_fentry_test2 as *const core::ffi::c_void as *mut core::ffi::c_void) {
        let b: __u64;
        let a: i32;

        err |= bpf_get_func_arg(ctx, 0, &mut value);
        a = value as i32;
        err |= bpf_get_func_arg(ctx, 1, &mut value);
        b = value;

        err |= if is_return { (ret != 5) as i64 } else { 0 };
        err |= if test_cookies { (cookie != 9) as i64 } else { 0 };

        *test_result = (*test_result).wrapping_add((err == 0 && a == 2 && b == 3) as __u64);
    } else if ip == (&bpf_fentry_test3 as *const core::ffi::c_void as *mut core::ffi::c_void) {
        let c: __u64;
        let a: i8;
        let b: i32;

        err |= bpf_get_func_arg(ctx, 0, &mut value);
        a = value as i8;
        err |= bpf_get_func_arg(ctx, 1, &mut value);
        b = value as i32;
        err |= bpf_get_func_arg(ctx, 2, &mut value);
        c = value;

        err |= if is_return { (ret != 15) as i64 } else { 0 };
        err |= if test_cookies { (cookie != 7) as i64 } else { 0 };

        *test_result = (*test_result).wrapping_add((err == 0 && a == 4 && b == 5 && c == 6) as __u64);
    } else if ip == (&bpf_fentry_test4 as *const core::ffi::c_void as *mut core::ffi::c_void) {
        let a: *mut core::ffi::c_void;
        let b: i8;
        let c: i32;
        let d: __u64;

        err |= bpf_get_func_arg(ctx, 0, &mut value);
        a = value as *mut core::ffi::c_void;
        err |= bpf_get_func_arg(ctx, 1, &mut value);
        b = value as i8;
        err |= bpf_get_func_arg(ctx, 2, &mut value);
        c = value as i32;
        err |= bpf_get_func_arg(ctx, 3, &mut value);
        d = value;

        err |= if is_return { (ret != 34) as i64 } else { 0 };
        err |= if test_cookies { (cookie != 5) as i64 } else { 0 };

        *test_result = (*test_result).wrapping_add(
            (err == 0 && a == 7usize as *mut core::ffi::c_void && b == 8 && c == 9 && d == 10)
                as __u64,
        );
    } else if ip == (&bpf_fentry_test5 as *const core::ffi::c_void as *mut core::ffi::c_void) {
        let a: __u64;
        let b: *mut core::ffi::c_void;
        let c: i16;
        let d: i32;
        let e: __u64;

        err |= bpf_get_func_arg(ctx, 0, &mut value);
        a = value;
        err |= bpf_get_func_arg(ctx, 1, &mut value);
        b = value as *mut core::ffi::c_void;
        err |= bpf_get_func_arg(ctx, 2, &mut value);
        c = value as i16;
        err |= bpf_get_func_arg(ctx, 3, &mut value);
        d = value as i32;
        err |= bpf_get_func_arg(ctx, 4, &mut value);
        e = value;

        err |= if is_return { (ret != 65) as i64 } else { 0 };
        err |= if test_cookies { (cookie != 4) as i64 } else { 0 };

        *test_result = (*test_result).wrapping_add(
            (err == 0
                && a == 11
                && b == 12usize as *mut core::ffi::c_void
                && c == 13
                && d == 14
                && e == 15) as __u64,
        );
    } else if ip == (&bpf_fentry_test6 as *const core::ffi::c_void as *mut core::ffi::c_void) {
        let a: __u64;
        let b: *mut core::ffi::c_void;
        let c: i16;
        let d: i32;
        let e: *mut core::ffi::c_void;
        let f: __u64;

        err |= bpf_get_func_arg(ctx, 0, &mut value);
        a = value;
        err |= bpf_get_func_arg(ctx, 1, &mut value);
        b = value as *mut core::ffi::c_void;
        err |= bpf_get_func_arg(ctx, 2, &mut value);
        c = value as i16;
        err |= bpf_get_func_arg(ctx, 3, &mut value);
        d = value as i32;
        err |= bpf_get_func_arg(ctx, 4, &mut value);
        e = value as *mut core::ffi::c_void;
        err |= bpf_get_func_arg(ctx, 5, &mut value);
        f = value;

        err |= if is_return { (ret != 111) as i64 } else { 0 };
        err |= if test_cookies { (cookie != 2) as i64 } else { 0 };

        *test_result = (*test_result).wrapping_add(
            (err == 0
                && a == 16
                && b == 17usize as *mut core::ffi::c_void
                && c == 18
                && d == 19
                && e == 20usize as *mut core::ffi::c_void
                && f == 21) as __u64,
        );
    } else if ip == (&bpf_fentry_test7 as *const core::ffi::c_void as *mut core::ffi::c_void) {
        err |= if is_return { (ret != 0) as i64 } else { 0 };
        err |= if test_cookies { (cookie != 3) as i64 } else { 0 };

        *test_result = (*test_result).wrapping_add(if err == 0 { 1 } else { 0 });
    } else if ip == (&bpf_fentry_test8 as *const core::ffi::c_void as *mut core::ffi::c_void) {
        err |= if is_return { (ret != 0) as i64 } else { 0 };
        err |= if test_cookies { (cookie != 1) as i64 } else { 0 };

        *test_result = (*test_result).wrapping_add(if err == 0 { 1 } else { 0 });
    } else if ip == (&bpf_fentry_test9 as *const core::ffi::c_void as *mut core::ffi::c_void) {
        err |= if is_return { (ret != 0) as i64 } else { 0 };
        err |= if test_cookies { (cookie != 10) as i64 } else { 0 };

        *test_result = (*test_result).wrapping_add(if err == 0 { 1 } else { 0 });
    } else if ip == (&bpf_fentry_test10 as *const core::ffi::c_void as *mut core::ffi::c_void) {
        err |= if is_return { (ret != 0) as i64 } else { 0 };
        err |= if test_cookies { (cookie != 6) as i64 } else { 0 };

        *test_result = (*test_result).wrapping_add(if err == 0 { 1 } else { 0 });
    } else if ip == (&bpf_testmod_fentry_test1 as *const core::ffi::c_void as *mut core::ffi::c_void) {
        let a: i32;

        err |= bpf_get_func_arg(ctx, 0, &mut value);
        a = value as i32;

        err |= if is_return { (ret != 2) as i64 } else { 0 };

        *test_result = (*test_result).wrapping_add((err == 0 && a == 1) as __u64);
    } else if ip == (&bpf_testmod_fentry_test2 as *const core::ffi::c_void as *mut core::ffi::c_void) {
        let a: i32;
        let b: __u64;

        err |= bpf_get_func_arg(ctx, 0, &mut value);
        a = value as i32;
        err |= bpf_get_func_arg(ctx, 1, &mut value);
        b = value as __u64;

        err |= if is_return { (ret != 5) as i64 } else { 0 };

        *test_result = (*test_result).wrapping_add((err == 0 && a == 2 && b == 3) as __u64);
    } else if ip == (&bpf_testmod_fentry_test3 as *const core::ffi::c_void as *mut core::ffi::c_void) {
        let a: i8;
        let b: i32;
        let c: __u64;

        err |= bpf_get_func_arg(ctx, 0, &mut value);
        a = value as i8;
        err |= bpf_get_func_arg(ctx, 1, &mut value);
        b = value as i32;
        err |= bpf_get_func_arg(ctx, 2, &mut value);
        c = value as __u64;

        err |= if is_return { (ret != 15) as i64 } else { 0 };

        *test_result = (*test_result).wrapping_add((err == 0 && a == 4 && b == 5 && c == 6) as __u64);
    } else if ip == (&bpf_testmod_fentry_test7 as *const core::ffi::c_void as *mut core::ffi::c_void) {
        err |= if is_return { (ret != 133) as i64 } else { 0 };

        *test_result = (*test_result).wrapping_add((err == 0) as __u64);
    } else if ip == (&bpf_testmod_fentry_test11 as *const core::ffi::c_void as *mut core::ffi::c_void) {
        err |= if is_return { (ret != 231) as i64 } else { 0 };

        *test_result = (*test_result).wrapping_add((err == 0) as __u64);
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
