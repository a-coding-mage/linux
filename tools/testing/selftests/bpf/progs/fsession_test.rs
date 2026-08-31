// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 ChinaTelecom */
/* Depends on vmlinux.h, bpf_helpers.h, and bpf_tracing.h definitions. */

use core::ffi::c_void;
use core::ptr::{read_volatile, write_volatile};

#[repr(C)]
pub struct bpf_fentry_test_t {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_session_is_return(ctx: *mut c_void) -> bool;
    fn bpf_get_func_ip(ctx: *mut c_void) -> u64;
    fn bpf_session_cookie(ctx: *mut c_void) -> *mut u64;
    fn bpf_fentry_test1();
}

/* SEC("license") */
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut test1_entry_result: u64 = 0;
#[unsafe(no_mangle)]
pub static mut test1_exit_result: u64 = 0;

/* SEC("fsession/bpf_fentry_test1") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test1(ctx: *mut c_void, a: i32, ret: i32) -> i32 {
    let is_exit: bool = unsafe { bpf_session_is_return(ctx) };

    if !is_exit {
        unsafe {
            test1_entry_result = (a == 1 && ret == 0) as u64;
        }
        return 0;
    }

    unsafe {
        test1_exit_result = (a == 1 && ret == 2) as u64;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test2_entry_result: u64 = 0;
#[unsafe(no_mangle)]
pub static mut test2_exit_result: u64 = 0;

/* SEC("fsession/bpf_fentry_test3") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test2(ctx: *mut c_void, a: i8, b: i32, c: u64, ret: i32) -> i32 {
    let is_exit: bool = unsafe { bpf_session_is_return(ctx) };

    if !is_exit {
        unsafe {
            test2_entry_result = (a == 4 && b == 5 && c == 6 && ret == 0) as u64;
        }
        return 0;
    }

    unsafe {
        test2_exit_result = (a == 4 && b == 5 && c == 6 && ret == 15) as u64;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test3_entry_result: u64 = 0;
#[unsafe(no_mangle)]
pub static mut test3_exit_result: u64 = 0;

/* SEC("fsession/bpf_fentry_test4") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test3(
    ctx: *mut c_void,
    a: *mut c_void,
    b: i8,
    c: i32,
    d: u64,
    ret: i32,
) -> i32 {
    let is_exit: bool = unsafe { bpf_session_is_return(ctx) };

    if !is_exit {
        unsafe {
            test3_entry_result =
                (a == 7usize as *mut c_void && b == 8 && c == 9 && d == 10 && ret == 0) as u64;
        }
        return 0;
    }

    unsafe {
        test3_exit_result =
            (a == 7usize as *mut c_void && b == 8 && c == 9 && d == 10 && ret == 34) as u64;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test4_entry_result: u64 = 0;
#[unsafe(no_mangle)]
pub static mut test4_exit_result: u64 = 0;

/* SEC("fsession/bpf_fentry_test5") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test4(
    ctx: *mut c_void,
    a: u64,
    b: *mut c_void,
    c: i16,
    d: i32,
    e: u64,
    ret: i32,
) -> i32 {
    let is_exit: bool = unsafe { bpf_session_is_return(ctx) };

    if !is_exit {
        unsafe {
            test4_entry_result = (a == 11
                && b == 12usize as *mut c_void
                && c == 13
                && d == 14
                && e == 15
                && ret == 0) as u64;
        }
        return 0;
    }

    unsafe {
        test4_exit_result = (a == 11
            && b == 12usize as *mut c_void
            && c == 13
            && d == 14
            && e == 15
            && ret == 65) as u64;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test5_entry_result: u64 = 0;
#[unsafe(no_mangle)]
pub static mut test5_exit_result: u64 = 0;

/* SEC("fsession/bpf_fentry_test7") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test5(
    ctx: *mut c_void,
    arg: *mut bpf_fentry_test_t,
    ret: i32,
) -> i32 {
    let is_exit: bool = unsafe { bpf_session_is_return(ctx) };

    if !is_exit {
        if arg.is_null() {
            unsafe {
                test5_entry_result = (ret == 0) as u64;
            }
        }
        return 0;
    }

    if arg.is_null() {
        unsafe {
            test5_exit_result = 1;
        }
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test6_entry_result: u64 = 0;
#[unsafe(no_mangle)]
pub static mut test6_exit_result: u64 = 0;

/* SEC("fsession/bpf_fentry_test1") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test6(ctx: *mut c_void, _a: i32) -> i32 {
    let addr: u64 = unsafe { bpf_get_func_ip(ctx) };

    if unsafe { bpf_session_is_return(ctx) } {
        unsafe {
            test6_exit_result = ((addr as *const c_void) == (bpf_fentry_test1 as *const c_void)) as u64;
        }
    } else {
        unsafe {
            test6_entry_result = ((addr as *const c_void) == (bpf_fentry_test1 as *const c_void)) as u64;
        }
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test7_entry_ok: u64 = 0;
#[unsafe(no_mangle)]
pub static mut test7_exit_ok: u64 = 0;

/* SEC("fsession/bpf_fentry_test1") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test7(ctx: *mut c_void, _a: i32) -> i32 {
    let cookie: *mut u64 = unsafe { bpf_session_cookie(ctx) };

    if !unsafe { bpf_session_is_return(ctx) } {
        unsafe {
            write_volatile(cookie, 0xAAAABBBBCCCCDDDDu64);
            test7_entry_ok = (read_volatile(cookie) == 0xAAAABBBBCCCCDDDDu64) as u64;
        }
        return 0;
    }

    unsafe {
        test7_exit_ok = (read_volatile(cookie) == 0xAAAABBBBCCCCDDDDu64) as u64;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test8_entry_ok: u64 = 0;
#[unsafe(no_mangle)]
pub static mut test8_exit_ok: u64 = 0;

/* SEC("fsession/bpf_fentry_test1") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test8(ctx: *mut c_void, _a: i32) -> i32 {
    let cookie: *mut u64 = unsafe { bpf_session_cookie(ctx) };

    if !unsafe { bpf_session_is_return(ctx) } {
        unsafe {
            write_volatile(cookie, 0x1111222233334444u64);
            test8_entry_ok = (read_volatile(cookie) == 0x1111222233334444u64) as u64;
        }
        return 0;
    }

    unsafe {
        test8_exit_ok = (read_volatile(cookie) == 0x1111222233334444u64) as u64;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test9_entry_result: u64 = 0;
#[unsafe(no_mangle)]
pub static mut test9_exit_result: u64 = 0;

/* SEC("fsession/bpf_fentry_test1") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test9(ctx: *mut c_void, a: i32, ret: i32) -> i32 {
    let cookie: *mut u64 = unsafe { bpf_session_cookie(ctx) };

    if !unsafe { bpf_session_is_return(ctx) } {
        unsafe {
            test9_entry_result = (a == 1 && ret == 0) as u64;
            *cookie = 0x123456u64;
        }
        return 0;
    }

    unsafe {
        test9_exit_result = (a == 1 && ret == 2 && *cookie == 0x123456u64) as u64;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test10_result: u64 = 0;

/* SEC("fexit/bpf_fentry_test1") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test10(_ctx: *mut c_void, a: i32, ret: i32) -> i32 {
    unsafe {
        test10_result = (a == 1 && ret == 2) as u64;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test11_result: u64 = 0;

/* SEC("fentry/bpf_fentry_test1") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test11(_ctx: *mut c_void, a: i32) -> i32 {
    unsafe {
        test11_result = (a == 1) as u64;
    }
    0
}
