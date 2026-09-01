// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

use core::ffi::{c_char, c_int, c_long, c_void};
use core::ptr;
use core::sync::atomic::{AtomicI32, Ordering};

#[repr(C)]
struct test_data {
    prog_fd: c_int,
    exit: AtomicI32,
}

#[repr(C)]
struct task_work_stress {
    obj: *mut bpf_object,
    bss: *mut task_work_stress_bss,
}

#[repr(C)]
struct task_work_stress_bss {
    callback_scheduled: c_long,
    schedule_error: c_long,
    delete_success: c_long,
    callback_success: c_long,
}

#[repr(C)]
struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_test_run_opts {
    _private: [u8; 0],
}

type pthread_t = libc::pthread_t;

unsafe extern "C" {
    static mut errno: c_int;

    fn getenv(name: *const c_char) -> *mut c_char;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn fprintf(stream: *mut libc::FILE, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut libc::FILE;
    fn sleep(seconds: libc::c_uint) -> libc::c_uint;

    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const libc::pthread_attr_t,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;

    fn task_work_stress__open() -> *mut task_work_stress;
    fn task_work_stress__load(skel: *mut task_work_stress) -> c_int;
    fn task_work_stress__destroy(skel: *mut task_work_stress);

    fn test__start_subtest(name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(left: c_long, right: c_long, name: *const c_char) -> bool;
    fn ASSERT_LT(left: c_long, right: c_long, name: *const c_char) -> bool;
    fn ASSERT_EQ(left: c_long, right: c_long, name: *const c_char) -> bool;
}

unsafe extern "C" fn runner(test_data: *mut c_void) -> *mut c_void {
    let td = test_data as *mut test_data;
    let mut err: c_int = 0;
    let mut opts: bpf_test_run_opts = core::mem::zeroed();

    while err == 0 && (*td).exit.load(Ordering::SeqCst) == 0 {
        err = bpf_prog_test_run_opts((*td).prog_fd, &mut opts);
    }

    ptr::null_mut()
}

unsafe fn get_env_int(str_: *const c_char, def: c_int) -> c_int {
    let s = getenv(str_);
    let mut end: *mut c_char = ptr::null_mut();
    let retval: c_int;

    if s.is_null() || *s == 0 {
        return def;
    }
    errno = 0;
    retval = strtol(s, &mut end, 10) as c_int;
    if errno != 0 || *end != 0 || retval < 0 {
        return def;
    }
    retval
}

unsafe fn task_work_run(enable_delete: bool) {
    let mut skel: *mut task_work_stress;
    let scheduler: *mut bpf_program;
    let deleter: *mut bpf_program;
    const NTHREADS: usize = 16;
    let nthreads: usize = NTHREADS;
    let test_time_s: c_int = get_env_int(c"BPF_TASK_WORK_TEST_TIME".as_ptr(), 1);
    let mut tid: [pthread_t; NTHREADS] = core::mem::zeroed();
    let mut tid_del: pthread_t = core::mem::zeroed();
    let mut started: [bool; NTHREADS] = [false; NTHREADS];
    let mut started_del: bool = false;
    let mut td_sched = test_data {
        prog_fd: 0,
        exit: AtomicI32::new(0),
    };
    let mut td_del = test_data {
        prog_fd: 0,
        exit: AtomicI32::new(1),
    };
    let mut i: usize;
    let mut err: c_int;

    skel = task_work_stress__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"task_work__open".as_ptr()) {
        return;
    }

    scheduler = bpf_object__find_program_by_name((*skel).obj, c"schedule_task_work".as_ptr());
    bpf_program__set_autoload(scheduler, true);

    deleter = bpf_object__find_program_by_name((*skel).obj, c"delete_task_work".as_ptr());
    bpf_program__set_autoload(deleter, true);

    err = task_work_stress__load(skel);
    if !ASSERT_OK(err, c"skel_load".as_ptr()) {
        task_work_stress__destroy(skel);
        return;
    }

    i = 0;
    while i < nthreads {
        started[i] = false;
        i += 1;
    }

    td_sched.prog_fd = bpf_program__fd(scheduler);
    i = 0;
    while i < nthreads {
        if pthread_create(
            &mut tid[i],
            ptr::null(),
            runner,
            &mut td_sched as *mut test_data as *mut c_void,
        ) != 0
        {
            fprintf(stderr, c"could not start thread".as_ptr());
            break;
        }
        started[i] = true;
        i += 1;
    }

    if i == nthreads {
        if enable_delete {
            td_del.exit.store(0, Ordering::SeqCst);
        }

        td_del.prog_fd = bpf_program__fd(deleter);
        if pthread_create(
            &mut tid_del,
            ptr::null(),
            runner,
            &mut td_del as *mut test_data as *mut c_void,
        ) != 0
        {
            fprintf(stderr, c"could not start thread".as_ptr());
        } else {
            started_del = true;

            /* Run stress test for some time */
            sleep(test_time_s as libc::c_uint);
        }
    }

    td_sched.exit.store(1, Ordering::SeqCst);
    td_del.exit.store(1, Ordering::SeqCst);
    i = 0;
    while i < nthreads {
        if started[i] {
            pthread_join(tid[i], ptr::null_mut());
        }
        i += 1;
    }

    if started_del {
        pthread_join(tid_del, ptr::null_mut());
    }

    ASSERT_GT((*(*skel).bss).callback_scheduled, 0, c"work scheduled".as_ptr());
    /* Some scheduling attempts should have failed due to contention */
    ASSERT_GT((*(*skel).bss).schedule_error, 0, c"schedule error".as_ptr());

    if enable_delete {
        /* If delete thread is enabled, it has cancelled some callbacks */
        ASSERT_GT((*(*skel).bss).delete_success, 0, c"delete success".as_ptr());
        ASSERT_LT(
            (*(*skel).bss).callback_success,
            (*(*skel).bss).callback_scheduled,
            c"callbacks".as_ptr(),
        );
    } else {
        /* Without delete thread number of scheduled callbacks is the same as fired */
        ASSERT_EQ(
            (*(*skel).bss).callback_success,
            (*(*skel).bss).callback_scheduled,
            c"callbacks".as_ptr(),
        );
    }

    task_work_stress__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_task_work_stress() {
    if test__start_subtest(c"no_delete".as_ptr()) {
        task_work_run(false);
    }
    if test__start_subtest(c"with_delete".as_ptr()) {
        task_work_run(true);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
