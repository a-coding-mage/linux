// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2018, Breno Leitao, Gustavo Romero, IBM Corp.
 *
 * A test case that creates a signal and starts a suspended transaction
 * inside the signal handler.
 *
 * It returns from the signal handler with the CPU at suspended state, but
 * without setting usercontext MSR Transaction State (TS) fields.
 */

// C dependencies: _GNU_SOURCE, <stdio.h>, <stdlib.h>, <signal.h>, "utils.h",
// and "tm.h".

use core::arch::asm;
use core::ptr;

#[no_mangle]
pub unsafe extern "C" fn trap_signal_handler(
    _signo: libc::c_int,
    _si: *mut libc::siginfo_t,
    uc: *mut libc::c_void,
) {
    let ucp: *mut libc::ucontext_t = uc as *mut libc::ucontext_t;

    asm!("tbegin.; tsuspend.;");

    /* Skip 'trap' instruction if it succeed */
    (*(*ucp).uc_mcontext.regs).nip += 4;
}

#[no_mangle]
pub unsafe extern "C" fn tm_signal_sigreturn_nt() -> libc::c_int {
    let mut trap_sa: libc::sigaction = core::mem::zeroed();

    SKIP_IF!(!have_htm());
    SKIP_IF!(htm_is_synthetic());

    trap_sa.sa_flags = libc::SA_SIGINFO;
    trap_sa.sa_sigaction = trap_signal_handler as usize;

    libc::sigaction(libc::SIGTRAP, &trap_sa, ptr::null_mut());

    libc::raise(libc::SIGTRAP);

    libc::EXIT_SUCCESS
}

#[no_mangle]
pub unsafe extern "C" fn main(_argc: libc::c_int, _argv: *mut *mut libc::c_char) -> libc::c_int {
    test_harness(
        Some(tm_signal_sigreturn_nt),
        b"tm_signal_sigreturn_nt\0".as_ptr() as *const libc::c_char,
    )
}
