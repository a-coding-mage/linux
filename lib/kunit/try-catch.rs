// SPDX-License-Identifier: GPL-2.0
/*
 * An API to allow a function, that may fail, to be executed, and recover in a
 * controlled manner.
 *
 * Copyright (C) 2019, Google LLC.
 * Author: Brendan Higgins <brendanhiggins@google.com>
 */

// Dependencies are supplied by the surrounding kernel/KUnit translation unit.

use core::ffi::c_void;

#[repr(C)]
pub struct kunit_last_seen {
    pub file: *const core::ffi::c_char,
    pub line: i32,
}

#[repr(C)]
pub struct kunit {
    pub last_seen: kunit_last_seen,
}

#[repr(C)]
pub struct task_struct {
    pub vfork_done: *mut completion,
}

#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}

pub type kunit_try_fn = unsafe extern "C" fn(*mut c_void);

#[repr(C)]
pub struct kunit_try_catch {
    pub test: *mut kunit,
    pub try_result: i32,
    pub context: *mut c_void,
    pub timeout: i32,
    pub try_: kunit_try_fn,
    pub catch: kunit_try_fn,
}

extern "C" {
    fn kthread_exit(code: i32) -> !;
    fn kthread_create(
        threadfn: unsafe extern "C" fn(*mut c_void) -> i32,
        data: *mut c_void,
        name: *const core::ffi::c_char,
    ) -> *mut task_struct;
    fn get_task_struct(task: *mut task_struct);
    fn wake_up_process(task: *mut task_struct);
    fn wait_for_completion_timeout(done: *mut completion, timeout: i32) -> i32;
    fn kthread_stop(task: *mut task_struct);
    fn put_task_struct(task: *mut task_struct);
    fn kunit_err(test: *mut kunit, fmt: *const core::ffi::c_char, ...);
}

const EFAULT: i32 = 14;
const EINTR: i32 = 4;
const ETIMEDOUT: i32 = 110;

pub unsafe extern "C" fn kunit_try_catch_throw(try_catch: *mut kunit_try_catch) -> ! {
    (*try_catch).try_result = -EFAULT;
    kthread_exit(0);
}

unsafe extern "C" fn kunit_generic_run_threadfn_adapter(data: *mut c_void) -> i32 {
    let try_catch = data as *mut kunit_try_catch;

    (*try_catch).try_result = -EINTR;
    ((*try_catch).try_)((*try_catch).context);
    if (*try_catch).try_result == -EINTR {
        (*try_catch).try_result = 0;
    }

    0
}

pub unsafe extern "C" fn kunit_try_catch_run(
    try_catch: *mut kunit_try_catch,
    context: *mut c_void,
) {
    let test = (*try_catch).test;
    let task_struct: *mut task_struct;
    let task_done: *mut completion;
    let exit_code: i32;
    let time_remaining: i32;

    (*try_catch).context = context;
    (*try_catch).try_result = 0;
    task_struct = kthread_create(
        kunit_generic_run_threadfn_adapter,
        try_catch as *mut c_void,
        b"kunit_try_catch_thread\0".as_ptr() as *const core::ffi::c_char,
    );
    // IS_ERR(task_struct)
    if (task_struct as isize) < 0 {
        (*try_catch).try_result = task_struct as isize as i32;
        ((*try_catch).catch)((*try_catch).context);
        return;
    }
    get_task_struct(task_struct);
    /*
     * As for a vfork(2), task_struct->vfork_done (pointing to the
     * underlying kthread->exited) can be used to wait for the end of a
     * kernel thread. It is set to NULL when the thread exits, so we
     * keep a copy here.
     */
    task_done = (*task_struct).vfork_done;
    wake_up_process(task_struct);

    time_remaining = wait_for_completion_timeout(task_done, (*try_catch).timeout);
    if time_remaining == 0 {
        (*try_catch).try_result = -ETIMEDOUT;
        kthread_stop(task_struct);
    }

    put_task_struct(task_struct);
    exit_code = (*try_catch).try_result;

    if exit_code == 0 {
        return;
    }

    if exit_code == -EFAULT {
        (*try_catch).try_result = 0;
    } else if exit_code == -EINTR {
        if !(*test).last_seen.file.is_null() {
            kunit_err(
                test,
                b"try faulted: last line seen %s:%d\n\0".as_ptr() as *const core::ffi::c_char,
                (*test).last_seen.file,
                (*test).last_seen.line,
            );
        } else {
            kunit_err(test, b"try faulted\n\0".as_ptr() as *const core::ffi::c_char);
        }
    } else if exit_code == -ETIMEDOUT {
        kunit_err(test, b"try timed out\n\0".as_ptr() as *const core::ffi::c_char);
    } else if exit_code != 0 {
        kunit_err(
            test,
            b"Unknown error: %d\n\0".as_ptr() as *const core::ffi::c_char,
            exit_code,
        );
    }

    ((*try_catch).catch)((*try_catch).context);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
