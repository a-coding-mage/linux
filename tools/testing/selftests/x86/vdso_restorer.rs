// SPDX-License-Identifier: GPL-2.0-only
/*
 * vdso_restorer.c - tests vDSO-based signal restore
 * Copyright (c) 2015 Andrew Lutomirski
 *
 * This makes sure that sa_restorer == NULL keeps working on 32-bit
 * configurations.  Modern glibc doesn't use it under any circumstances,
 * so it's easy to overlook breakage.
 *
 * 64-bit userspace has never supported sa_restorer == NULL, so this is
 * 32-bit only.
 */

/* C dependencies: err.h, stdio.h, dlfcn.h, string.h, signal.h, unistd.h,
 * syscall.h, sys/syscall.h.
 */

use libc::{
    c_char, c_int, c_long, c_uint, c_ulong, c_void, sig_atomic_t, siginfo_t, size_t, SIGUSR1,
    SA_SIGINFO, RTLD_LAZY, RTLD_LOCAL, RTLD_NOLOAD, SYS_rt_sigaction, SYS_sigaction,
};

unsafe extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn printf(format: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn syscall(number: c_long, ...) -> c_long;
    fn raise(sig: c_int) -> c_int;
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
}

/* Open-code this -- the headers are too messy to easily use them. */
#[repr(C)]
struct real_sigaction {
    handler: *mut c_void,
    flags: c_ulong,
    restorer: *mut c_void,
    mask: [c_uint; 2],
}

static mut handler_called: sig_atomic_t = 0;

unsafe extern "C" fn handler_with_siginfo(
    _sig: c_int,
    _info: *mut siginfo_t,
    _ctx_void: *mut c_void,
) {
    unsafe {
        handler_called = 1;
    }
}

unsafe extern "C" fn handler_without_siginfo(_sig: c_int) {
    unsafe {
        handler_called = 1;
    }
}

fn main() {
    unsafe {
        let mut nerrs: c_int = 0;
        let mut sa: real_sigaction = core::mem::zeroed();

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
            printf(c"[SKIP]\tFailed to find vDSO.  Tests are not expected to work.\n".as_ptr());
            std::process::exit(0);
        }

        memset(
            &mut sa as *mut real_sigaction as *mut c_void,
            0,
            core::mem::size_of_val(&sa),
        );
        sa.handler = handler_with_siginfo as *mut c_void;
        sa.flags = SA_SIGINFO as c_ulong;
        sa.restorer = core::ptr::null_mut(); /* request kernel-provided restorer */

        printf(c"[RUN]\tRaise a signal, SA_SIGINFO, sa.restorer == NULL\n".as_ptr());

        if syscall(
            SYS_rt_sigaction as c_long,
            SIGUSR1,
            &sa as *const real_sigaction,
            core::ptr::null_mut::<c_void>(),
            8,
        ) != 0
        {
            err(1, c"raw rt_sigaction syscall".as_ptr());
        }

        raise(SIGUSR1);

        if handler_called != 0 {
            printf(c"[OK]\tSA_SIGINFO handler returned successfully\n".as_ptr());
        } else {
            printf(c"[FAIL]\tSA_SIGINFO handler was not called\n".as_ptr());
            nerrs += 1;
        }

        printf(c"[RUN]\tRaise a signal, !SA_SIGINFO, sa.restorer == NULL\n".as_ptr());

        sa.flags = 0;
        sa.handler = handler_without_siginfo as *mut c_void;
        if syscall(
            SYS_sigaction as c_long,
            SIGUSR1,
            &sa as *const real_sigaction,
            0,
        ) != 0
        {
            err(1, c"raw sigaction syscall".as_ptr());
        }
        handler_called = 0;

        raise(SIGUSR1);

        if handler_called != 0 {
            printf(c"[OK]\t!SA_SIGINFO handler returned successfully\n".as_ptr());
        } else {
            printf(c"[FAIL]\t!SA_SIGINFO handler was not called\n".as_ptr());
            nerrs += 1;
        }

        std::process::exit(nerrs);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
