// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2024 ARM Ltd.
 *
 * Author: Dev Jain <dev.jain@arm.com>
 *
 * Test describing a clear distinction between signal states - delivered and
 * blocked, and their relation with ucontext.
 *
 * A process can request blocking of a signal by masking it into its set of
 * blocked signals; such a signal, when sent to the process by the kernel,
 * will get blocked by the process and it may later unblock it and take an
 * action. At that point, the signal will be delivered.
 *
 * We test the following functionalities of the kernel:
 *
 * ucontext_t describes the interrupted context of the thread; this implies
 * that, in case of registering a handler and catching the corresponding
 * signal, that state is before what was jumping into the handler.
 *
 * The thread's mask of blocked signals can be permanently changed, i.e, not
 * just during the execution of the handler, by mangling with uc_sigmask
 * from inside the handler.
 *
 * Assume that we block the set of signals, S1, by sigaction(), and say, the
 * signal for which the handler was installed, is S2. When S2 is sent to the
 * program, it will be considered "delivered", since we will act on the
 * signal and jump to the handler. Any instances of S1 or S2 raised, while the
 * program is executing inside the handler, will be blocked; they will be
 * delivered immediately upon termination of the handler.
 *
 * For standard signals (also see real-time signals in the man page), multiple
 * blocked instances of the same signal are not queued; such a signal will
 * be delivered just once.
 */

// C includes translated as external dependencies:
// <stdio.h>, <stdlib.h>, <signal.h>, <ucontext.h>, and "kselftest.h".

use libc::{
    c_char, c_int, c_void, raise, sigaction, sigaddset, sigemptyset, siginfo_t, sigismember,
    sigprocmask, sigset_t, ucontext_t, SA_SIGINFO, SIGSEGV, SIGUSR1, SIGUSR2, SIG_BLOCK,
};

extern "C" {
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_test_result(condition: bool, fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_exit_fail_perror(msg: *const c_char) -> !;
    fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
    fn ksft_finished() -> !;
}

unsafe extern "C" fn handler_verify_ucontext(
    _signo: c_int,
    _info: *mut siginfo_t,
    uc: *mut c_void,
) {
    let mut ret: c_int;

    /* Kernel dumps ucontext with USR2 blocked */
    ret = sigismember(&mut (*(uc as *mut ucontext_t)).uc_sigmask, SIGUSR2);
    ksft_test_result(ret == 1, b"USR2 blocked in ucontext\n\0".as_ptr() as *const c_char);

    /*
     * USR2 is blocked; can be delivered neither here, nor after
     * exit from handler
     */
    if raise(SIGUSR2) != 0 {
        ksft_exit_fail_perror(b"raise\0".as_ptr() as *const c_char);
    }
}

unsafe extern "C" fn handler_segv(_signo: c_int, _info: *mut siginfo_t, _uc: *mut c_void) {
    /*
     * Three cases possible:
     * 1. Program already terminated due to segmentation fault.
     * 2. SEGV was blocked even after returning from handler_usr.
     * 3. SEGV was delivered on returning from handler_usr.
     * The last option must happen.
     */
    ksft_test_result_pass(b"SEGV delivered\n\0".as_ptr() as *const c_char);
}

static mut CNT: c_int = 0;

unsafe extern "C" fn handler_usr(_signo: c_int, _info: *mut siginfo_t, uc: *mut c_void) {
    let mut ret: c_int;

    /*
     * Break out of infinite recursion caused by raise(SIGUSR1) invoked
     * from inside the handler
     */
    CNT += 1;
    if CNT > 1 {
        return;
    }

    /* SEGV blocked during handler execution, delivered on return */
    if raise(SIGSEGV) != 0 {
        ksft_exit_fail_perror(b"raise\0".as_ptr() as *const c_char);
    }

    ksft_print_msg(b"SEGV bypassed successfully\n\0".as_ptr() as *const c_char);

    /*
     * Signal responsible for handler invocation is blocked by default;
     * delivered on return, leading to recursion
     */
    if raise(SIGUSR1) != 0 {
        ksft_exit_fail_perror(b"raise\0".as_ptr() as *const c_char);
    }

    ksft_test_result(
        CNT == 1,
        b"USR1 is blocked, cannot invoke handler right now\n\0".as_ptr() as *const c_char,
    );

    /* Raise USR1 again; only one instance must be delivered upon exit */
    if raise(SIGUSR1) != 0 {
        ksft_exit_fail_perror(b"raise\0".as_ptr() as *const c_char);
    }

    /* SEGV has been blocked in sa_mask, but ucontext is empty */
    ret = sigismember(&mut (*(uc as *mut ucontext_t)).uc_sigmask, SIGSEGV);
    ksft_test_result(ret == 0, b"SEGV not blocked in ucontext\n\0".as_ptr() as *const c_char);

    /* USR1 has been blocked, but ucontext is empty */
    ret = sigismember(&mut (*(uc as *mut ucontext_t)).uc_sigmask, SIGUSR1);
    ksft_test_result(ret == 0, b"USR1 not blocked in ucontext\n\0".as_ptr() as *const c_char);

    /*
     * Mangle ucontext; this will be copied back into &current->blocked
     * on return from the handler.
     */
    if sigaddset(&mut (*(uc as *mut ucontext_t)).uc_sigmask, SIGUSR2) != 0 {
        ksft_exit_fail_perror(b"sigaddset\0".as_ptr() as *const c_char);
    }
}

unsafe fn set_sa_sigaction(
    act: *mut sigaction,
    handler: unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void),
) {
    (*act).sa_sigaction = handler as usize;
}

fn main() {
    unsafe {
        let mut act: sigaction = std::mem::zeroed();
        let mut act2: sigaction = std::mem::zeroed();
        let mut set: sigset_t = std::mem::zeroed();
        let mut oldset: sigset_t = std::mem::zeroed();

        ksft_print_header();
        ksft_set_plan(7);

        act.sa_flags = SA_SIGINFO;
        set_sa_sigaction(&mut act, handler_usr);

        /* Add SEGV to blocked mask */
        if sigemptyset(&mut act.sa_mask) != 0
            || sigaddset(&mut act.sa_mask, SIGSEGV) != 0
            || sigismember(&mut act.sa_mask, SIGSEGV) != 1
        {
            ksft_exit_fail_msg(b"Cannot add SEGV to blocked mask\n\0".as_ptr() as *const c_char);
        }

        if sigaction(SIGUSR1, &act, std::ptr::null_mut()) != 0 {
            ksft_exit_fail_perror(b"Cannot install handler\0".as_ptr() as *const c_char);
        }

        act2.sa_flags = SA_SIGINFO;
        set_sa_sigaction(&mut act2, handler_segv);

        if sigaction(SIGSEGV, &act2, std::ptr::null_mut()) != 0 {
            ksft_exit_fail_perror(b"Cannot install handler\0".as_ptr() as *const c_char);
        }

        /* Invoke handler */
        if raise(SIGUSR1) != 0 {
            ksft_exit_fail_perror(b"raise\0".as_ptr() as *const c_char);
        }

        /* USR1 must not be queued */
        ksft_test_result(CNT == 2, b"handler invoked only twice\n\0".as_ptr() as *const c_char);

        /* Mangled ucontext implies USR2 is blocked for current thread */
        if raise(SIGUSR2) != 0 {
            ksft_exit_fail_perror(b"raise\0".as_ptr() as *const c_char);
        }

        ksft_print_msg(b"USR2 bypassed successfully\n\0".as_ptr() as *const c_char);

        set_sa_sigaction(&mut act, handler_verify_ucontext);
        if sigaction(SIGUSR1, &act, std::ptr::null_mut()) != 0 {
            ksft_exit_fail_perror(b"Cannot install handler\0".as_ptr() as *const c_char);
        }

        if raise(SIGUSR1) != 0 {
            ksft_exit_fail_perror(b"raise\0".as_ptr() as *const c_char);
        }

        /*
         * Raising USR2 in handler_verify_ucontext is redundant since it
         * is blocked
         */
        ksft_print_msg(b"USR2 still blocked on return from handler\n\0".as_ptr() as *const c_char);

        /* Confirm USR2 blockage by sigprocmask() too */
        if sigemptyset(&mut set) != 0 {
            ksft_exit_fail_perror(b"sigemptyset\0".as_ptr() as *const c_char);
        }

        if sigprocmask(SIG_BLOCK, &set, &mut oldset) != 0 {
            ksft_exit_fail_perror(b"sigprocmask\0".as_ptr() as *const c_char);
        }

        ksft_test_result(
            sigismember(&mut oldset, SIGUSR2) == 1,
            b"USR2 present in &current->blocked\n\0".as_ptr() as *const c_char,
        );

        ksft_finished();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
