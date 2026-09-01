// SPDX-License-Identifier: GPL-2.0
// C includes translated as external declarations below:
// errno.h, fcntl.h, sched.h, stdio.h, stdbool.h, sys/stat.h,
// sys/syscall.h, sys/types.h, sys/wait.h, time.h, unistd.h,
// string.h, pthread.h, "log.h", "timens.h"

use std::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use std::ptr;

const OFFSET: c_long = 36000;
const CLOCK_MONOTONIC: clockid_t = 1;

type clockid_t = c_int;
type pid_t = c_int;
type pthread_t = c_ulong;

#[repr(C)]
struct timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

#[repr(C)]
struct thread_args {
    tst_name: *mut c_char,
    now: *mut timespec,
}

#[repr(C)]
struct ksft_count {
    ksft_pass: c_int,
}

unsafe extern "C" {
    static mut ksft_cnt: ksft_count;

    fn _gettime(clk_id: clockid_t, tp: *mut timespec, vdso_clock: c_int) -> c_int;
    fn _settime(clk_id: clockid_t, offset: c_long) -> c_int;
    fn unshare_timens() -> c_int;
    fn nscheck();

    fn pr_fail(fmt: *const c_char, ...) -> c_int;
    fn pr_perror(fmt: *const c_char, ...) -> c_int;
    fn ksft_print_header();
    fn ksft_set_plan(cnt: c_int);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_exit_fail() -> !;
    fn ksft_inc_pass_cnt();
    fn ksft_exit_pass() -> !;

    fn labs(j: c_long) -> c_long;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn clock_gettime(clk_id: clockid_t, tp: *mut timespec) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn vfork() -> pid_t;
    fn snprintf(str: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn execve(
        pathname: *const c_char,
        argv: *const *mut c_char,
        envp: *const *mut c_char,
    ) -> c_int;
    fn _exit(status: c_int) -> !;
    fn waitpid(pid: pid_t, stat_loc: *mut c_int, options: c_int) -> pid_t;
}

unsafe extern "C" fn tcheck(_args: *mut c_void) -> *mut c_void {
    let args: *mut thread_args = _args as *mut thread_args;
    let now: *mut timespec = (*args).now;
    let mut tst: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut i: c_int;

    i = 0;
    while i < 2 {
        _gettime(CLOCK_MONOTONIC, &mut tst, i);
        if labs(tst.tv_sec - (*now).tv_sec) > 5 {
            pr_fail(
                c"%s: in-thread: unexpected value: %ld (%ld)\n".as_ptr(),
                (*args).tst_name,
                tst.tv_sec,
                (*now).tv_sec,
            );
            return 1usize as *mut c_void;
        }
        i += 1;
    }
    ptr::null_mut()
}

unsafe fn check_in_thread(tst_name: *mut c_char, now: *mut timespec) -> c_int {
    let mut args: thread_args = thread_args { tst_name, now };
    let mut th: pthread_t = 0;
    let mut retval: *mut c_void = ptr::null_mut();

    if pthread_create(
        &mut th,
        ptr::null(),
        tcheck,
        &mut args as *mut thread_args as *mut c_void,
    ) != 0
    {
        return pr_perror(c"thread".as_ptr());
    }
    if pthread_join(th, &mut retval) != 0 {
        return pr_perror(c"pthread_join".as_ptr());
    }
    (retval != ptr::null_mut()) as c_int
}

unsafe fn check(tst_name: *mut c_char, now: *mut timespec) -> c_int {
    let mut tst: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut i: c_int;

    i = 0;
    while i < 2 {
        _gettime(CLOCK_MONOTONIC, &mut tst, i);
        if labs(tst.tv_sec - (*now).tv_sec) > 5 {
            return pr_fail(
                c"%s: unexpected value: %ld (%ld)\n".as_ptr(),
                tst_name,
                tst.tv_sec,
                (*now).tv_sec,
            );
        }
        i += 1;
    }
    if check_in_thread(tst_name, now) != 0 {
        return 1;
    }
    ksft_test_result_pass(c"%s\n".as_ptr(), tst_name);
    0
}

unsafe fn main_impl(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut status: c_int = 0;
    let mut pid: pid_t;

    if argc > 1 {
        let mut endptr: *mut c_char = ptr::null_mut();

        ksft_cnt.ksft_pass = 1;
        now.tv_sec = strtoul(*argv.add(1), &mut endptr, 0) as c_long;
        if *endptr != 0 {
            return pr_perror(c"strtoul".as_ptr());
        }

        return check(c"child after exec".as_ptr() as *mut c_char, &mut now);
    }

    ksft_print_header();

    nscheck();

    ksft_set_plan(4);

    clock_gettime(CLOCK_MONOTONIC, &mut now);

    if unshare_timens() != 0 {
        return 1;
    }

    if _settime(CLOCK_MONOTONIC, OFFSET) != 0 {
        return 1;
    }

    if check(c"parent before vfork".as_ptr() as *mut c_char, &mut now) != 0 {
        return 1;
    }

    pid = vfork();
    if pid < 0 {
        return pr_perror(c"fork".as_ptr());
    }

    if pid == 0 {
        let mut now_str: [c_char; 64] = [0; 64];
        let mut cargv: [*mut c_char; 3] = [
            c"exec".as_ptr() as *mut c_char,
            now_str.as_mut_ptr(),
            ptr::null_mut(),
        ];
        let cenv: [*mut c_char; 1] = [ptr::null_mut()];

        /* Check for proper vvar offsets after execve. */
        snprintf(
            now_str.as_mut_ptr(),
            now_str.len(),
            c"%ld".as_ptr(),
            now.tv_sec + OFFSET,
        );
        execve(c"/proc/self/exe".as_ptr(), cargv.as_mut_ptr(), cenv.as_ptr());
        pr_perror(c"execve".as_ptr());
        _exit(1);
    }

    if waitpid(pid, &mut status, 0) != pid {
        return pr_perror(c"waitpid".as_ptr());
    }

    if status != 0 {
        ksft_exit_fail();
    }
    ksft_inc_pass_cnt();
    ksft_test_result_pass(c"wait for child\n".as_ptr());

    /* Check that we are still in the source timens. */
    if check(c"parent after vfork".as_ptr() as *mut c_char, &mut now) != 0 {
        return 1;
    }

    ksft_exit_pass();
}

fn main() {
    let mut args: Vec<*mut c_char> = std::env::args()
        .map(|arg| {
            std::ffi::CString::new(arg)
                .expect("argument contains interior NUL")
                .into_raw()
        })
        .collect();
    args.push(ptr::null_mut());

    let ret = unsafe { main_impl((args.len() - 1) as c_int, args.as_mut_ptr()) };

    for arg in args.into_iter().take_while(|arg| !arg.is_null()) {
        unsafe {
            let _ = std::ffi::CString::from_raw(arg);
        }
    }

    std::process::exit(ret);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
