// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2020 Collabora Ltd.
 *
 * Benchmark and test syscall user dispatch
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use core::arch::asm;
use core::ffi::c_void;

const PR_SET_SYSCALL_USER_DISPATCH: libc::c_int = 59;
const PR_SYS_DISPATCH_OFF: libc::c_int = 0;
const PR_SYS_DISPATCH_ON: libc::c_int = 1;
const SYSCALL_DISPATCH_FILTER_ALLOW: libc::c_int = 0;
const SYSCALL_DISPATCH_FILTER_BLOCK: libc::c_int = 1;

/*
 * MAGIC_SYSCALL_1 is defined from __NR_syscalls when available in C;
 * otherwise the C source falls back to 0xff00.
 */
const MAGIC_SYSCALL_1: libc::c_long = 0xff00;

/*
 * To test returning from a sigsys with selector blocked, the test
 * requires some per-architecture support (i.e. knowledge about the
 * signal trampoline address).  On i386, we know it is on the vdso, and
 * a small trampoline is open-coded for x86_64.  Other architectures
 * that have a trampoline in the vdso will support TEST_BLOCKED_RETURN
 * out of the box, but don't enable them until they support syscall user
 * dispatch.
 */

#[cfg(target_arch = "x86_64")]
extern "C" {
    fn syscall_dispatcher_start() -> *mut c_void;
    fn syscall_dispatcher_end() -> *mut c_void;
}

#[cfg(not(target_arch = "x86_64"))]
static mut syscall_dispatcher_start: libc::c_ulong = 0;
#[cfg(not(target_arch = "x86_64"))]
static mut syscall_dispatcher_end: libc::c_ulong = 0;

static mut trapped_call_count: libc::c_ulong = 0;
static mut native_call_count: libc::c_ulong = 0;

static mut selector: libc::c_char = 0;

const CALIBRATION_STEP: libc::c_int = 100000;
const CALIBRATE_TO_SECS: libc::c_int = 5;
static mut factor: libc::c_int = 0;

unsafe fn SYSCALL_BLOCK() {
    selector = SYSCALL_DISPATCH_FILTER_BLOCK as libc::c_char;
}

unsafe fn SYSCALL_UNBLOCK() {
    selector = SYSCALL_DISPATCH_FILTER_ALLOW as libc::c_char;
}

unsafe fn one_sysinfo_step() -> f64 {
    let mut t1: libc::timespec = core::mem::zeroed();
    let mut t2: libc::timespec = core::mem::zeroed();
    let mut i: libc::c_int;
    let mut info: libc::sysinfo = core::mem::zeroed();

    libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut t1);
    i = 0;
    while i < CALIBRATION_STEP {
        libc::sysinfo(&mut info);
        i += 1;
    }
    libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut t2);
    (t2.tv_sec - t1.tv_sec) as f64 + 1.0e-9f64 * (t2.tv_nsec - t1.tv_nsec) as f64
}

unsafe fn calibrate_set() {
    let mut elapsed: f64 = 0.0;

    libc::printf(
        b"Calibrating test set to last ~%d seconds...\n\0".as_ptr() as *const libc::c_char,
        CALIBRATE_TO_SECS,
    );

    while elapsed < 1.0 {
        elapsed += one_sysinfo_step();
        factor += CALIBRATE_TO_SECS;
    }

    libc::printf(
        b"test iterations = %d\n\0".as_ptr() as *const libc::c_char,
        CALIBRATION_STEP * factor,
    );
}

unsafe fn perf_syscall() -> f64 {
    let mut i: libc::c_uint;
    let mut partial: f64 = 0.0;

    i = 0;
    while i < factor as libc::c_uint {
        partial += one_sysinfo_step() / (CALIBRATION_STEP * factor) as f64;
        i += 1;
    }
    partial
}

unsafe extern "C" fn handle_sigsys(
    _sig: libc::c_int,
    info: *mut libc::siginfo_t,
    _ucontext: *mut c_void,
) {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let len: libc::c_int;

    SYSCALL_UNBLOCK();

    /* printf and friends are not signal-safe. */
    len = libc::snprintf(
        buf.as_mut_ptr(),
        1024,
        b"Caught sys_%x\n\0".as_ptr() as *const libc::c_char,
        (*info).si_syscall(),
    );
    libc::write(1, buf.as_ptr() as *const c_void, len as libc::size_t);

    if (*info).si_syscall() as libc::c_long == MAGIC_SYSCALL_1 {
        trapped_call_count += 1;
    } else {
        native_call_count += 1;
    }

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        SYSCALL_BLOCK();
    }

    #[cfg(target_arch = "x86_64")]
    {
        asm!("mov rax, 0xf", options(nostack, preserves_flags));
        asm!("leave", options(preserves_flags));
        asm!("add rsp, 0x8", options(nostack, preserves_flags));
        asm!("syscall_dispatcher_start:");
        asm!("syscall");
        asm!("nop"); /* Landing pad within dispatcher area */
        asm!("syscall_dispatcher_end:");
    }
}

fn main() {
    unsafe {
        let mut act: libc::sigaction = core::mem::zeroed();
        let mut time1: f64;
        let mut time2: f64;
        let mut ret: libc::c_int;
        let mut mask: libc::sigset_t = core::mem::zeroed();

        libc::memset(
            &mut act as *mut libc::sigaction as *mut c_void,
            0,
            core::mem::size_of::<libc::sigaction>(),
        );
        libc::sigemptyset(&mut mask);

        act.sa_sigaction = handle_sigsys as usize;
        act.sa_flags = libc::SA_SIGINFO;
        act.sa_mask = mask;

        calibrate_set();

        time1 = perf_syscall();
        libc::printf(
            b"Avg syscall time %.0lfns.\n\0".as_ptr() as *const libc::c_char,
            time1 * 1.0e9f64,
        );

        ret = libc::sigaction(libc::SIGSYS, &act, core::ptr::null_mut());
        if ret != 0 {
            libc::perror(b"Error sigaction:\0".as_ptr() as *const libc::c_char);
            libc::exit(-1);
        }

        libc::fprintf(
            libc::stderr,
            b"Enabling syscall trapping.\n\0".as_ptr() as *const libc::c_char,
        );

        #[cfg(target_arch = "x86_64")]
        {
            if libc::prctl(
                PR_SET_SYSCALL_USER_DISPATCH,
                PR_SYS_DISPATCH_ON,
                syscall_dispatcher_start,
                syscall_dispatcher_end as usize - syscall_dispatcher_start as usize + 1,
                &mut selector as *mut libc::c_char,
            ) != 0
            {
                libc::perror(b"prctl failed\n\0".as_ptr() as *const libc::c_char);
                libc::exit(-1);
            }
        }

        #[cfg(not(target_arch = "x86_64"))]
        {
            if libc::prctl(
                PR_SET_SYSCALL_USER_DISPATCH,
                PR_SYS_DISPATCH_ON,
                syscall_dispatcher_start,
                syscall_dispatcher_end - syscall_dispatcher_start + 1,
                &mut selector as *mut libc::c_char,
            ) != 0
            {
                libc::perror(b"prctl failed\n\0".as_ptr() as *const libc::c_char);
                libc::exit(-1);
            }
        }

        SYSCALL_BLOCK();
        libc::syscall(MAGIC_SYSCALL_1);

        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        {
            if selector == SYSCALL_DISPATCH_FILTER_ALLOW as libc::c_char {
                libc::fprintf(
                    libc::stderr,
                    b"Failed to return with selector blocked.\n\0".as_ptr()
                        as *const libc::c_char,
                );
                libc::exit(-1);
            }
        }

        SYSCALL_UNBLOCK();

        if trapped_call_count == 0 {
            libc::fprintf(
                libc::stderr,
                b"syscall trapping does not work.\n\0".as_ptr() as *const libc::c_char,
            );
            libc::exit(-1);
        }

        time2 = perf_syscall();

        if native_call_count != 0 {
            libc::perror(
                b"syscall trapping intercepted more syscalls than expected\n\0".as_ptr()
                    as *const libc::c_char,
            );
            libc::exit(-1);
        }

        libc::printf(
            b"trapped_call_count %lu, native_call_count %lu.\n\0".as_ptr()
                as *const libc::c_char,
            trapped_call_count,
            native_call_count,
        );
        libc::printf(
            b"Avg syscall time %.0lfns.\n\0".as_ptr() as *const libc::c_char,
            time2 * 1.0e9f64,
        );
        libc::printf(
            b"Interception overhead: %.1lf%% (+%.0lfns).\n\0".as_ptr()
                as *const libc::c_char,
            100.0f64 * (time2 / time1 - 1.0f64),
            1.0e9f64 * (time2 - time1),
        );
        libc::exit(0);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
