// SPDX-License-Identifier: GPL-2.0
// C source used _GNU_SOURCE and included system/libc headers plus "log.h" and "timens.h".

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_long, c_void};

type clockid_t = c_int;
type clock_t = c_long;
type time_t = c_long;

#[repr(C)]
#[derive(Copy, Clone)]
struct timespec {
    tv_sec: time_t,
    tv_nsec: c_long,
}

type vgettime_t = Option<unsafe extern "C" fn(clockid_t, *mut timespec) -> c_int>;

static mut vdso_clock_gettime: vgettime_t = None;

const RTLD_LAZY: c_int = 0x00001;
const RTLD_LOCAL: c_int = 0;
const RTLD_NOLOAD: c_int = 0x00004;

const O_RDONLY: c_int = 0;
const CLONE_NEWTIME: c_int = 0x00000080;

const CLOCK_MONOTONIC: clock_t = 1;
const CLOCK_MONOTONIC_RAW: clock_t = 4;
const CLOCK_MONOTONIC_COARSE: clock_t = 6;
const CLOCK_BOOTTIME: clock_t = 7;

unsafe extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn setns(fd: c_int, nstype: c_int) -> c_int;

    fn pr_err(fmt: *const c_char, ...);
    fn pr_perror(fmt: *const c_char, ...) -> c_int;
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_exit_pass();
    fn nscheck();
    fn unshare_timens() -> c_int;
    fn _settime(clockid: clockid_t, offset: time_t) -> c_int;
}

unsafe fn fill_function_pointers() {
    let mut vdso = dlopen(
        c"linux-vdso.so.1".as_ptr(),
        RTLD_LAZY | RTLD_LOCAL | RTLD_NOLOAD,
    );
    if vdso.is_null() {
        vdso = dlopen(
            c"linux-gate.so.1".as_ptr(),
            RTLD_LAZY | RTLD_LOCAL | RTLD_NOLOAD,
        );
    }
    if vdso.is_null() {
        vdso = dlopen(
            c"linux-vdso32.so.1".as_ptr(),
            RTLD_LAZY | RTLD_LOCAL | RTLD_NOLOAD,
        );
    }
    if vdso.is_null() {
        vdso = dlopen(
            c"linux-vdso64.so.1".as_ptr(),
            RTLD_LAZY | RTLD_LOCAL | RTLD_NOLOAD,
        );
    }
    if vdso.is_null() {
        pr_err(c"[WARN]\tfailed to find vDSO\n".as_ptr());
        return;
    }

    vdso_clock_gettime = core::mem::transmute::<*mut c_void, vgettime_t>(dlsym(
        vdso,
        c"__vdso_clock_gettime".as_ptr(),
    ));
    if vdso_clock_gettime.is_none() {
        vdso_clock_gettime = core::mem::transmute::<*mut c_void, vgettime_t>(dlsym(
            vdso,
            c"__kernel_clock_gettime".as_ptr(),
        ));
    }
    if vdso_clock_gettime.is_none() {
        pr_err(c"Warning: failed to find clock_gettime in vDSO\n".as_ptr());
    }
}

unsafe fn test(clockid: clock_t, clockstr: *mut c_char, in_ns: bool) {
    let mut tp: timespec;
    let start: timespec;
    let mut i: c_long = 0;
    const timeout: c_int = 3;

    let gettime = vdso_clock_gettime.unwrap();
    let mut start_tmp = core::mem::MaybeUninit::<timespec>::uninit();
    gettime(clockid as clockid_t, start_tmp.as_mut_ptr());
    start = start_tmp.assume_init();
    tp = start;
    while start.tv_sec + timeout as time_t > tp.tv_sec
        || (start.tv_sec + timeout as time_t == tp.tv_sec && start.tv_nsec > tp.tv_nsec)
    {
        gettime(clockid as clockid_t, &mut tp);
        i += 1;
    }

    ksft_test_result_pass(
        c"%s:\tclock: %10s\tcycles:\t%10ld\n".as_ptr(),
        if in_ns {
            c"ns".as_ptr()
        } else {
            c"host".as_ptr()
        },
        clockstr,
        i,
    );
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let offset: time_t = 10;
    let nsfd: c_int;

    ksft_print_header();

    ksft_set_plan(8);

    fill_function_pointers();

    test(CLOCK_MONOTONIC, c"monotonic".as_ptr() as *mut c_char, false);
    test(
        CLOCK_MONOTONIC_COARSE,
        c"monotonic-coarse".as_ptr() as *mut c_char,
        false,
    );
    test(
        CLOCK_MONOTONIC_RAW,
        c"monotonic-raw".as_ptr() as *mut c_char,
        false,
    );
    test(CLOCK_BOOTTIME, c"boottime".as_ptr() as *mut c_char, false);

    nscheck();

    if unshare_timens() != 0 {
        return 1;
    }

    nsfd = open(c"/proc/self/ns/time_for_children".as_ptr(), O_RDONLY);
    if nsfd < 0 {
        return pr_perror(c"Can't open a time namespace".as_ptr());
    }

    if _settime(CLOCK_MONOTONIC as clockid_t, offset) != 0 {
        return 1;
    }
    if _settime(CLOCK_BOOTTIME as clockid_t, offset) != 0 {
        return 1;
    }

    if setns(nsfd, CLONE_NEWTIME) != 0 {
        return pr_perror(c"setns".as_ptr());
    }

    test(CLOCK_MONOTONIC, c"monotonic".as_ptr() as *mut c_char, true);
    test(
        CLOCK_MONOTONIC_COARSE,
        c"monotonic-coarse".as_ptr() as *mut c_char,
        true,
    );
    test(
        CLOCK_MONOTONIC_RAW,
        c"monotonic-raw".as_ptr() as *mut c_char,
        true,
    );
    test(CLOCK_BOOTTIME, c"boottime".as_ptr() as *mut c_char, true);

    ksft_exit_pass();
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
