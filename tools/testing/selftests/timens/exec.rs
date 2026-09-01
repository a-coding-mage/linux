// SPDX-License-Identifier: GPL-2.0
// C dependencies: errno.h, fcntl.h, sched.h, stdio.h, stdbool.h,
// sys/stat.h, sys/syscall.h, sys/types.h, sys/wait.h, time.h, unistd.h,
// string.h, "log.h", and "timens.h".

#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_long};
use core::ptr;

const OFFSET: c_long = 36000;
const CLOCK_MONOTONIC: c_int = 1;

type pid_t = c_int;

#[repr(C)]
struct timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

unsafe extern "C" {
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn labs(j: c_long) -> c_long;
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn fork() -> pid_t;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn execve(
        pathname: *const c_char,
        argv: *const *mut c_char,
        envp: *const *mut c_char,
    ) -> c_int;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;

    fn pr_perror(format: *const c_char, ...) -> c_int;
    fn pr_fail(format: *const c_char, ...) -> c_int;
    fn ksft_print_header();
    fn ksft_set_plan(cnt: c_int);
    fn ksft_exit_fail() -> !;
    fn ksft_test_result_pass(format: *const c_char, ...);
    fn ksft_exit_pass() -> !;

    fn nscheck();
    fn unshare_timens() -> c_int;
    fn _settime(clockid: c_int, offset: c_long) -> c_int;
    fn _gettime(clockid: c_int, ts: *mut timespec, vvar: c_int);
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut now = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut tst = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut status: c_int = 0;
    let mut i: c_int;
    let pid: pid_t;

    if argc > 1 {
        if sscanf(*argv.add(1), b"%ld\0".as_ptr() as *const c_char, &mut now.tv_sec) != 1 {
            return pr_perror(b"sscanf\0".as_ptr() as *const c_char);
        }

        i = 0;
        while i < 2 {
            _gettime(CLOCK_MONOTONIC, &mut tst, i);
            if labs(tst.tv_sec - now.tv_sec) > 5 {
                return pr_fail(
                    b"%ld %ld\n\0".as_ptr() as *const c_char,
                    now.tv_sec,
                    tst.tv_sec,
                );
            }
            i += 1;
        }
        return 0;
    }

    ksft_print_header();

    nscheck();

    ksft_set_plan(1);

    clock_gettime(CLOCK_MONOTONIC, &mut now);

    if unshare_timens() != 0 {
        return 1;
    }

    if _settime(CLOCK_MONOTONIC, OFFSET) != 0 {
        return 1;
    }

    i = 0;
    while i < 2 {
        _gettime(CLOCK_MONOTONIC, &mut tst, i);
        if labs(tst.tv_sec - now.tv_sec) > 5 {
            return pr_fail(
                b"%ld %ld\n\0".as_ptr() as *const c_char,
                now.tv_sec,
                tst.tv_sec,
            );
        }
        i += 1;
    }

    if argc > 1 {
        return 0;
    }

    pid = fork();
    if pid < 0 {
        return pr_perror(b"fork\0".as_ptr() as *const c_char);
    }

    if pid == 0 {
        let mut now_str = [0 as c_char; 64];
        let mut cargv = [
            b"exec\0".as_ptr() as *mut c_char,
            now_str.as_mut_ptr(),
            ptr::null_mut(),
        ];
        let cenv = [ptr::null_mut::<c_char>()];

        /* Check that a child process is in the new timens. */
        i = 0;
        while i < 2 {
            _gettime(CLOCK_MONOTONIC, &mut tst, i);
            if labs(tst.tv_sec - now.tv_sec - OFFSET) > 5 {
                return pr_fail(
                    b"%ld %ld\n\0".as_ptr() as *const c_char,
                    now.tv_sec + OFFSET,
                    tst.tv_sec,
                );
            }
            i += 1;
        }

        /* Check for proper vvar offsets after execve. */
        snprintf(
            now_str.as_mut_ptr(),
            now_str.len(),
            b"%ld\0".as_ptr() as *const c_char,
            now.tv_sec + OFFSET,
        );
        execve(
            b"/proc/self/exe\0".as_ptr() as *const c_char,
            cargv.as_mut_ptr(),
            cenv.as_ptr(),
        );
        return pr_perror(b"execve\0".as_ptr() as *const c_char);
    }

    if waitpid(pid, &mut status, 0) != pid {
        return pr_perror(b"waitpid\0".as_ptr() as *const c_char);
    }

    if status != 0 {
        ksft_exit_fail();
    }

    ksft_test_result_pass(b"exec\n\0".as_ptr() as *const c_char);
    ksft_exit_pass();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
