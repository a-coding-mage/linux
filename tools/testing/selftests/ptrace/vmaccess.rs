// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (c) 2020 Bernd Edlinger <bernd.edlinger@hotmail.de>
 * All rights reserved.
 *
 * Check whether /proc/$pid/mem can be accessed without causing deadlocks
 * when de_thread is blocked with ->cred_guard_mutex held.
 */

// C dependencies: "kselftest_harness.h", stdio.h, fcntl.h, pthread.h,
// signal.h, unistd.h, sys/ptrace.h.

use libc::{
    c_char, c_int, c_long, c_void, close, execlp, fork, kill, open, pthread_create, pthread_join,
    pthread_t, ptrace, sleep, sprintf, waitpid, EAGAIN, ECHILD, O_RDONLY, PTRACE_ATTACH,
    PTRACE_DETACH, PTRACE_TRACEME, SIGCONT, SIGSTOP, WEXITSTATUS, WIFEXITED, WIFSTOPPED,
    WNOHANG, WSTOPSIG,
};

unsafe extern "C" {
    static mut errno: c_int;
}

unsafe extern "C" fn thread(_arg: *mut c_void) -> *mut c_void {
    unsafe {
        ptrace(
            PTRACE_TRACEME,
            0,
            0 as c_long,
            0 as c_long,
        );
    }
    core::ptr::null_mut()
}

// Original C harness declaration: TEST(vmaccess)
unsafe fn vmaccess() {
    unsafe {
        let mut f: c_int;
        let pid: c_int = fork();
        let mut mm: [c_char; 64] = [0; 64];

        if pid == 0 {
            let mut pt: pthread_t = core::mem::zeroed();

            pthread_create(
                &mut pt,
                core::ptr::null(),
                thread,
                core::ptr::null_mut(),
            );
            pthread_join(pt, core::ptr::null_mut());
            execlp(c"true".as_ptr(), c"true".as_ptr(), core::ptr::null::<c_char>());
        }

        sleep(1);
        sprintf(mm.as_mut_ptr(), c"/proc/%d/mem".as_ptr(), pid);
        f = open(mm.as_ptr(), O_RDONLY);
        assert!(f >= 0);
        close(f);
        f = kill(pid, SIGCONT);
        assert_eq!(f, 0);
    }
}

// Original C harness declaration: TEST(attach)
unsafe fn attach() {
    unsafe {
        let mut s: c_int = 0;
        let mut k: c_int;
        let pid: c_int = fork();

        if pid == 0 {
            let mut pt: pthread_t = core::mem::zeroed();

            pthread_create(
                &mut pt,
                core::ptr::null(),
                thread,
                core::ptr::null_mut(),
            );
            pthread_join(pt, core::ptr::null_mut());
            execlp(
                c"sleep".as_ptr(),
                c"sleep".as_ptr(),
                c"2".as_ptr(),
                core::ptr::null::<c_char>(),
            );
        }

        sleep(1);
        k = ptrace(PTRACE_ATTACH, pid, 0 as c_long, 0 as c_long);
        assert_eq!(errno, EAGAIN);
        assert_eq!(k, -1);
        k = waitpid(-1, &mut s, WNOHANG);
        assert_ne!(k, -1);
        assert_ne!(k, 0);
        assert_ne!(k, pid);
        assert_eq!(WIFEXITED(s), 1);
        assert_eq!(WEXITSTATUS(s), 0);
        sleep(1);
        k = ptrace(PTRACE_ATTACH, pid, 0 as c_long, 0 as c_long);
        assert_eq!(k, 0);
        k = waitpid(-1, &mut s, 0);
        assert_eq!(k, pid);
        assert_eq!(WIFSTOPPED(s), 1);
        assert_eq!(WSTOPSIG(s), SIGSTOP);
        k = ptrace(PTRACE_DETACH, pid, 0 as c_long, 0 as c_long);
        assert_eq!(k, 0);
        k = waitpid(-1, &mut s, 0);
        assert_eq!(k, pid);
        assert_eq!(WIFEXITED(s), 1);
        assert_eq!(WEXITSTATUS(s), 0);
        k = waitpid(-1, core::ptr::null_mut(), 0);
        assert_eq!(k, -1);
        assert_eq!(errno, ECHILD);
    }
}

// Original C harness declaration: TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
