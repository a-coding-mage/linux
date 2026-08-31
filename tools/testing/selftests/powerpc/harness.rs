// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2013, Michael Ellerman, IBM Corp.
 */

// C dependencies translated as Rust dependencies/import intent:
// errno.h, signal.h, stdbool.h, stdio.h, stdlib.h, sys/types.h, sys/wait.h,
// unistd.h, elf.h, fcntl.h, link.h, sys/stat.h
// Local dependencies: "subunit.h", "utils.h"

use core::ffi::{c_char, c_int};

const KILL_TIMEOUT: libc::c_uint = 5;

extern "C" {
    fn test_start(name: *const c_char);
    fn test_set_git_version(version: *const c_char);
    fn test_error(name: *const c_char);
    fn test_skip(name: *const c_char);
    fn test_finish(name: *const c_char, rc: c_int);

    static GIT_VERSION: c_char;
    static MAGIC_SKIP_RETURN_VALUE: c_int;
}

/* Setting timeout to -1 disables the alarm */
static mut timeout: u64 = 120;

#[inline]
unsafe fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

#[inline]
unsafe fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

#[inline]
unsafe fn wifsignaled(status: c_int) -> bool {
    (((status & 0x7f) + 1) >> 1) > 0
}

#[inline]
unsafe fn wtermsig(status: c_int) -> c_int {
    status & 0x7f
}

#[no_mangle]
pub unsafe extern "C" fn run_test(
    test_function: Option<unsafe extern "C" fn() -> c_int>,
    name: *const c_char,
) -> c_int {
    let mut terminated: bool;
    let mut rc: c_int;
    let mut status: c_int = 0;
    let pid: libc::pid_t;

    /* Make sure output is flushed before forking */
    libc::fflush(libc::stdout);

    pid = libc::fork();
    if pid == 0 {
        libc::setpgid(0, 0);
        libc::exit(test_function.unwrap()());
    } else if pid == -1 {
        libc::perror(b"fork\0".as_ptr() as *const c_char);
        return 1;
    }

    libc::setpgid(pid, pid);

    if timeout != (-1i32 as u64) {
        /* Wake us up in timeout seconds */
        libc::alarm(timeout as libc::c_uint);
    }
    terminated = false;

    loop {
        rc = libc::waitpid(pid, &mut status as *mut c_int, 0);
        if rc == -1 {
            if *libc::__errno_location() != libc::EINTR {
                libc::printf(b"unknown error from waitpid\n\0".as_ptr() as *const c_char);
                return 1;
            }

            if terminated {
                libc::printf(b"!! force killing %s\n\0".as_ptr() as *const c_char, name);
                libc::kill(-pid, libc::SIGKILL);
                return 1;
            } else {
                libc::printf(b"!! killing %s\n\0".as_ptr() as *const c_char, name);
                libc::kill(-pid, libc::SIGTERM);
                terminated = true;
                libc::alarm(KILL_TIMEOUT);
                continue;
            }
        }
        break;
    }

    /* Kill anything else in the process group that is still running */
    libc::kill(-pid, libc::SIGTERM);

    if wifexited(status) {
        status = wexitstatus(status);
    } else {
        if wifsignaled(status) {
            libc::printf(
                b"!! child died by signal %d\n\0".as_ptr() as *const c_char,
                wtermsig(status),
            );
        } else {
            libc::printf(b"!! child died by unknown cause\n\0".as_ptr() as *const c_char);
        }

        status = 1; /* Signal or other */
    }

    status
}

unsafe extern "C" fn sig_handler(_signum: c_int) {
    /* Just wake us up from waitpid */
}

static mut sig_action: libc::sigaction = unsafe { core::mem::zeroed() };

#[no_mangle]
pub unsafe extern "C" fn test_harness_set_timeout(time: u64) {
    timeout = time;
}

#[no_mangle]
pub unsafe extern "C" fn test_harness(
    test_function: Option<unsafe extern "C" fn() -> c_int>,
    name: *const c_char,
) -> c_int {
    let mut rc: c_int;

    sig_action.sa_sigaction = sig_handler as usize;

    test_start(name);
    test_set_git_version(&GIT_VERSION as *const c_char);

    if libc::sigaction(libc::SIGINT, &sig_action as *const libc::sigaction, core::ptr::null_mut()) != 0
    {
        libc::perror(b"sigaction (sigint)\0".as_ptr() as *const c_char);
        test_error(name);
        return 1;
    }

    if libc::sigaction(
        libc::SIGALRM,
        &sig_action as *const libc::sigaction,
        core::ptr::null_mut(),
    ) != 0
    {
        libc::perror(b"sigaction (sigalrm)\0".as_ptr() as *const c_char);
        test_error(name);
        return 1;
    }

    rc = run_test(test_function, name);

    if rc == MAGIC_SKIP_RETURN_VALUE {
        test_skip(name);
        /* so that skipped test is not marked as failed */
        rc = 0;
    } else {
        test_finish(name, rc);
    }

    rc
}
