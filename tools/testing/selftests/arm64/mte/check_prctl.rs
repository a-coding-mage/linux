// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2022 ARM Limited

use core::ffi::{c_char, c_int, c_ulong};

// C dependencies from <sys/auxv.h>, <sys/prctl.h>, <asm/hwcap.h>, and
// "kselftest.h" are represented as FFI declarations or local macro
// equivalents where this file provided the value.
const AT_HWCAP2: c_ulong = 26;
const AT_HWCAP3: c_ulong = 29;

const PR_SET_TAGGED_ADDR_CTRL: c_int = 55;
const PR_GET_TAGGED_ADDR_CTRL: c_int = 56;

const PR_MTE_TCF_NONE: c_int = 0;
const PR_MTE_TCF_SYNC: c_int = 1 << 1;
const PR_MTE_TCF_ASYNC: c_int = 1 << 2;
const PR_MTE_TCF_MASK: c_int = PR_MTE_TCF_SYNC | PR_MTE_TCF_ASYNC;
const PR_MTE_STORE_ONLY: c_int = 1 << 3;

const HWCAP2_MTE: c_int = 1 << 18;
const HWCAP3_MTE_STORE_ONLY: c_int = 1 << 0;

unsafe extern "C" {
    fn prctl(option: c_int, ...) -> c_int;
    fn getauxval(type_: c_ulong) -> c_ulong;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn __errno_location() -> *mut c_int;

    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_print_cnts();
}

unsafe fn errno_value() -> c_int {
    unsafe { *__errno_location() }
}

unsafe fn set_tagged_addr_ctrl(val: c_int) -> c_int {
    let ret: c_int;

    ret = unsafe { prctl(PR_SET_TAGGED_ADDR_CTRL, val, 0, 0, 0) };
    if ret < 0 {
        let err = unsafe { errno_value() };
        unsafe {
            ksft_print_msg(
                c"PR_SET_TAGGED_ADDR_CTRL: failed %d %d (%s)\n".as_ptr(),
                ret,
                err,
                strerror(err),
            );
        }
    }
    ret
}

unsafe fn get_tagged_addr_ctrl() -> c_int {
    let ret: c_int;

    ret = unsafe { prctl(PR_GET_TAGGED_ADDR_CTRL, 0, 0, 0, 0) };
    if ret < 0 {
        let err = unsafe { errno_value() };
        unsafe {
            ksft_print_msg(
                c"PR_GET_TAGGED_ADDR_CTRL failed: %d %d (%s)\n".as_ptr(),
                ret,
                err,
                strerror(err),
            );
        }
    }
    ret
}

/*
 * Read the current mode without having done any configuration, should
 * run first.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_basic_read() {
    let ret: c_int;

    ret = unsafe { get_tagged_addr_ctrl() };
    if ret < 0 {
        unsafe {
            ksft_test_result_fail(c"check_basic_read\n".as_ptr());
        }
        return;
    }

    if ret & PR_MTE_TCF_SYNC != 0 {
        unsafe {
            ksft_print_msg(c"SYNC enabled\n".as_ptr());
        }
    }
    if ret & PR_MTE_TCF_ASYNC != 0 {
        unsafe {
            ksft_print_msg(c"ASYNC enabled\n".as_ptr());
        }
    }

    /* Any configuration is valid */
    unsafe {
        ksft_test_result_pass(c"check_basic_read\n".as_ptr());
    }
}

/*
 * Attempt to set a specified combination of modes.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_mode_test(
    name: *const c_char,
    hwcap2: c_int,
    hwcap3: c_int,
    mask: c_int,
) {
    let ret: c_int;

    if unsafe { getauxval(AT_HWCAP2) } & hwcap2 as c_ulong != hwcap2 as c_ulong {
        unsafe {
            ksft_test_result_skip(c"%s\n".as_ptr(), name);
        }
        return;
    }

    if unsafe { getauxval(AT_HWCAP3) } & hwcap3 as c_ulong != hwcap3 as c_ulong {
        unsafe {
            ksft_test_result_skip(c"%s\n".as_ptr(), name);
        }
        return;
    }

    ret = unsafe { set_tagged_addr_ctrl(mask) };
    if ret < 0 {
        unsafe {
            ksft_test_result_fail(c"%s\n".as_ptr(), name);
        }
        return;
    }

    ret = unsafe { get_tagged_addr_ctrl() };
    if ret < 0 {
        unsafe {
            ksft_test_result_fail(c"%s\n".as_ptr(), name);
        }
        return;
    }

    if ret & (PR_MTE_TCF_MASK | PR_MTE_STORE_ONLY) == mask {
        unsafe {
            ksft_test_result_pass(c"%s\n".as_ptr(), name);
        }
    } else {
        unsafe {
            ksft_print_msg(
                c"Got %x, expected %x\n".as_ptr(),
                ret & PR_MTE_TCF_MASK as c_int,
                mask,
            );
            ksft_test_result_fail(c"%s\n".as_ptr(), name);
        }
    }
}

#[repr(C)]
struct mte_mode {
    mask: c_int,
    hwcap2: c_int,
    hwcap3: c_int,
    name: *const c_char,
}

static mut mte_modes: [mte_mode; 7] = [
    mte_mode {
        mask: PR_MTE_TCF_NONE,
        hwcap2: 0,
        hwcap3: 0,
        name: c"NONE".as_ptr(),
    },
    mte_mode {
        mask: PR_MTE_TCF_SYNC,
        hwcap2: HWCAP2_MTE,
        hwcap3: 0,
        name: c"SYNC".as_ptr(),
    },
    mte_mode {
        mask: PR_MTE_TCF_ASYNC,
        hwcap2: HWCAP2_MTE,
        hwcap3: 0,
        name: c"ASYNC".as_ptr(),
    },
    mte_mode {
        mask: PR_MTE_TCF_SYNC | PR_MTE_TCF_ASYNC,
        hwcap2: HWCAP2_MTE,
        hwcap3: 0,
        name: c"SYNC+ASYNC".as_ptr(),
    },
    mte_mode {
        mask: PR_MTE_TCF_SYNC | PR_MTE_STORE_ONLY,
        hwcap2: HWCAP2_MTE,
        hwcap3: HWCAP3_MTE_STORE_ONLY,
        name: c"SYNC+STONLY".as_ptr(),
    },
    mte_mode {
        mask: PR_MTE_TCF_ASYNC | PR_MTE_STORE_ONLY,
        hwcap2: HWCAP2_MTE,
        hwcap3: HWCAP3_MTE_STORE_ONLY,
        name: c"ASYNC+STONLY".as_ptr(),
    },
    mte_mode {
        mask: PR_MTE_TCF_SYNC | PR_MTE_TCF_ASYNC | PR_MTE_STORE_ONLY,
        hwcap2: HWCAP2_MTE,
        hwcap3: HWCAP3_MTE_STORE_ONLY,
        name: c"SYNC+ASYNC+STONLY".as_ptr(),
    },
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> c_int {
    let mut i: c_int;

    unsafe {
        ksft_print_header();
        ksft_set_plan(mte_modes.len() as c_int + 1);
    }

    unsafe {
        check_basic_read();
    }
    i = 0;
    while i < unsafe { mte_modes.len() as c_int } {
        unsafe {
            set_mode_test(
                mte_modes[i as usize].name,
                mte_modes[i as usize].hwcap2,
                mte_modes[i as usize].hwcap3,
                mte_modes[i as usize].mask,
            );
        }
        i += 1;
    }

    unsafe {
        ksft_print_cnts();
    }

    0
}
