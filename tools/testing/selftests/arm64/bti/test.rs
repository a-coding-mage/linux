// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019,2021  Arm Limited
 * Original author: Dave Martin <Dave.Martin@arm.com>
 */

// C dependencies translated as external declarations:
// "system.h", <stdbool.h>, <stddef.h>, <linux/errno.h>,
// <linux/auxvec.h>, <linux/signal.h>, <asm/sigcontext.h>,
// <asm/ucontext.h>, "btitest.h", "signal.h"

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

#[repr(C)]
pub struct sigset_t {
    _data: [u64; 16],
}

pub type sighandler_t = Option<unsafe extern "C" fn(c_int)>;

#[repr(C)]
pub struct sigaction {
    pub sa_handler: sighandler_t,
    pub sa_flags: c_ulong,
    pub sa_restorer: Option<unsafe extern "C" fn()>,
    pub sa_mask: sigset_t,
}

#[repr(C)]
pub struct siginfo_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mcontext_t {
    pub fault_address: u64,
    pub regs: [u64; 31],
    pub sp: u64,
    pub pc: u64,
    pub pstate: u64,
}

#[repr(C)]
pub struct ucontext {
    pub uc_flags: c_ulong,
    pub uc_link: *mut ucontext,
    pub uc_stack: stack_t,
    pub uc_sigmask: sigset_t,
    pub uc_mcontext: mcontext_t,
}

#[repr(C)]
pub struct stack_t {
    pub ss_sp: *mut c_void,
    pub ss_flags: c_int,
    pub ss_size: usize,
}

pub type ucontext_t = ucontext;

const EXPECTED_TESTS: u32 = 18;

unsafe extern "C" {
    static BTI: c_int;

    static PSR_BTYPE_MASK: u64;
    static PSR_BTYPE_SHIFT: u32;
    static AT_NULL: c_ulong;
    static AT_HWCAP: c_ulong;
    static AT_HWCAP2: c_ulong;
    static HWCAP_PACA: c_ulong;
    static HWCAP2_BTI: c_ulong;
    static SA_SIGINFO: c_ulong;
    static SIGILL: c_int;
    static SIG_UNBLOCK: c_int;

    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn exit(status: c_int) -> !;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn sigaddset(set: *mut sigset_t, signum: c_int) -> c_int;
    fn sigprocmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int;

    fn call_using_br_x0(fn_: Option<unsafe extern "C" fn()>);
    fn call_using_br_x16(fn_: Option<unsafe extern "C" fn()>);
    fn call_using_blr(fn_: Option<unsafe extern "C" fn()>);

    fn nohint_func();
    fn bti_none_func();
    fn bti_c_func();
    fn bti_j_func();
    fn bti_jc_func();
    fn paciasp_func();
}

static mut test_num: u32 = 1;
static mut test_passed: u32 = 0;
static mut test_failed: u32 = 0;
static mut test_skipped: u32 = 0;

unsafe fn fdputs(fd: c_int, str_: *const c_char) {
    let mut len: usize = 0;
    let mut p = str_;

    while *p != 0 {
        p = p.add(1);
        len += 1;
    }

    write(fd, str_ as *const c_void, len);
}

unsafe fn putstr(str_: *const c_char) {
    fdputs(1, str_);
}

unsafe fn putnum(num: u32) {
    let c: c_char;

    if num / 10 != 0 {
        putnum(num / 10);
    }

    c = (b'0' + (num % 10) as u8) as c_char;
    write(1, &c as *const c_char as *const c_void, 1);
}

unsafe fn puttestname(test_name: *const c_char, trampoline_name: *const c_char) {
    putstr(test_name);
    putstr(c"/".as_ptr());
    putstr(trampoline_name);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn print_summary() {
    putstr(c"# Totals: pass:".as_ptr());
    putnum(test_passed);
    putstr(c" fail:".as_ptr());
    putnum(test_failed);
    putstr(c" xfail:0 xpass:0 skip:".as_ptr());
    putnum(test_skipped);
    putstr(c" error:0\n".as_ptr());
}

static mut current_test_name: *const c_char = ptr::null();
static mut current_trampoline_name: *const c_char = ptr::null();
static mut sigill_expected: c_int = 0;
static mut sigill_received: c_int = 0;

unsafe extern "C" fn handler(n: c_int, si: *mut siginfo_t, uc_: *mut c_void) {
    let uc: *mut ucontext_t = uc_ as *mut ucontext_t;
    let _ = si;

    putstr(c"# \t[SIGILL in ".as_ptr());
    puttestname(current_test_name, current_trampoline_name);
    putstr(c", BTYPE=".as_ptr());
    write(
        1,
        c"00011011"
            .as_ptr()
            .add(((((*uc).uc_mcontext.pstate & PSR_BTYPE_MASK) >> PSR_BTYPE_SHIFT) * 2) as usize)
            as *const c_void,
        2,
    );
    if sigill_expected == 0 {
        putstr(c"]\n".as_ptr());
        putstr(c"not ok ".as_ptr());
        putnum(test_num);
        putstr(c" ".as_ptr());
        puttestname(current_test_name, current_trampoline_name);
        putstr(c"(unexpected SIGILL)\n".as_ptr());
        print_summary();
        exit(128 + n);
    }

    putstr(c" (expected)]\n".as_ptr());
    sigill_received = 1;
    /* zap BTYPE so that resuming the faulting code will work */
    (*uc).uc_mcontext.pstate &= !PSR_BTYPE_MASK;
}

/* Does the system have BTI? */
static mut have_bti: bool = false;

unsafe fn __do_test(
    trampoline: unsafe extern "C" fn(Option<unsafe extern "C" fn()>),
    fn_: Option<unsafe extern "C" fn()>,
    trampoline_name: *const c_char,
    name: *const c_char,
    mut expect_sigill: c_int,
) {
    /*
     * Branch Target exceptions should only happen for BTI
     * binaries running on a system with BTI:
     */
    if BTI == 0 || !have_bti {
        expect_sigill = 0;
    }

    sigill_expected = expect_sigill;
    sigill_received = 0;
    current_test_name = name;
    current_trampoline_name = trampoline_name;

    trampoline(fn_);

    if expect_sigill != 0 && sigill_received == 0 {
        putstr(c"not ok ".as_ptr());
        test_failed += 1;
    } else {
        putstr(c"ok ".as_ptr());
        test_passed += 1;
    }
    putnum(test_num);
    test_num += 1;
    putstr(c" ".as_ptr());
    puttestname(name, trampoline_name);
    putstr(c"\n".as_ptr());
}

unsafe fn do_test(
    expect_sigill_br_x0: c_int,
    expect_sigill_br_x16: c_int,
    expect_sigill_blr: c_int,
    name: Option<unsafe extern "C" fn()>,
    name_str: *const c_char,
) {
    __do_test(
        call_using_br_x0,
        name,
        c"call_using_br_x0".as_ptr(),
        name_str,
        expect_sigill_br_x0,
    );
    __do_test(
        call_using_br_x16,
        name,
        c"call_using_br_x16".as_ptr(),
        name_str,
        expect_sigill_br_x16,
    );
    __do_test(
        call_using_blr,
        name,
        c"call_using_blr".as_ptr(),
        name_str,
        expect_sigill_blr,
    );
}

#[repr(C)]
struct auxv_entry {
    type_: c_ulong,
    val: c_ulong,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn start(argcp: *mut c_int) {
    let mut sa: sigaction = core::mem::zeroed();
    let mut p: *const *const c_void;
    let mut auxv: *const auxv_entry;
    let mut hwcap: c_ulong = 0;
    let mut hwcap2: c_ulong = 0;

    putstr(c"TAP version 13\n".as_ptr());
    putstr(c"1..".as_ptr());
    putnum(EXPECTED_TESTS);
    putstr(c"\n".as_ptr());

    /* Gross hack for finding AT_HWCAP2 from the initial process stack: */
    p = (argcp as *const *const c_void).add(1 + *argcp as usize + 1); /* start of environment */
    /* step over environment */
    while !(*p).is_null() {
        p = p.add(1);
    }
    p = p.add(1);
    auxv = p as *const auxv_entry;
    while (*auxv).type_ != AT_NULL {
        match (*auxv).type_ {
            x if x == AT_HWCAP => {
                hwcap = (*auxv).val;
            }
            x if x == AT_HWCAP2 => {
                hwcap2 = (*auxv).val;
            }
            _ => {}
        }
        auxv = auxv.add(1);
    }

    if hwcap & HWCAP_PACA != 0 {
        putstr(c"# HWCAP_PACA present\n".as_ptr());
    } else {
        putstr(c"# HWCAP_PACA not present\n".as_ptr());
    }

    if hwcap2 & HWCAP2_BTI != 0 {
        putstr(c"# HWCAP2_BTI present\n".as_ptr());
        if !(hwcap & HWCAP_PACA != 0) {
            putstr(c"# Bad hardware?  Expect problems.\n".as_ptr());
        }
        have_bti = true;
    } else {
        putstr(c"# HWCAP2_BTI not present\n".as_ptr());
        have_bti = false;
    }

    putstr(c"# Test binary".as_ptr());
    if BTI == 0 {
        putstr(c" not".as_ptr());
    }
    putstr(c" built for BTI\n".as_ptr());

    sa.sa_handler = core::mem::transmute::<
        unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void),
        sighandler_t,
    >(handler);
    sa.sa_flags = SA_SIGINFO;
    sigemptyset(&mut sa.sa_mask);
    sigaction(SIGILL, &sa, ptr::null_mut());
    sigaddset(&mut sa.sa_mask, SIGILL);
    sigprocmask(SIG_UNBLOCK, &sa.sa_mask, ptr::null_mut());

    do_test(1, 1, 1, Some(nohint_func), c"nohint_func".as_ptr());
    do_test(1, 1, 1, Some(bti_none_func), c"bti_none_func".as_ptr());
    do_test(1, 0, 0, Some(bti_c_func), c"bti_c_func".as_ptr());
    do_test(0, 0, 1, Some(bti_j_func), c"bti_j_func".as_ptr());
    do_test(0, 0, 0, Some(bti_jc_func), c"bti_jc_func".as_ptr());
    do_test(1, 0, 0, Some(paciasp_func), c"paciasp_func".as_ptr());

    print_summary();

    if test_num - 1 != EXPECTED_TESTS {
        putstr(c"# WARNING - EXPECTED TEST COUNT WRONG\n".as_ptr());
    }

    if test_failed != 0 {
        exit(1);
    } else {
        exit(0);
    }
}
