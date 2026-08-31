// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

/*
 * C dependencies from the original source:
 * - <sched.h>
 * - <pthread.h>
 * - <test_progs.h>
 * - "timer_start_delete_race.skel.h"
 */

/*
 * Test for race between bpf_timer_start() and map element deletion.
 *
 * The race scenario:
 * - CPU 1: bpf_timer_start() proceeds to bpf_async_process() and is about
 *          to call hrtimer_start() but hasn't yet
 * - CPU 2: map_delete_elem() calls __bpf_async_cancel_and_free(), since
 *          timer is not scheduled yet hrtimer_try_to_cancel() is a nop,
 *          then calls bpf_async_refcount_put() dropping refcnt to zero
 *          and scheduling call_rcu_tasks_trace()
 * - CPU 1: continues and calls hrtimer_start()
 * - After RCU tasks trace grace period: memory is freed
 * - Timer callback fires on freed memory: UAF!
 *
 * This test stresses this race by having two threads:
 * - Thread 1: repeatedly starts timers
 * - Thread 2: repeatedly deletes map elements
 *
 * KASAN should detect use-after-free.
 */

use core::ffi::{c_int, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null_mut, read_volatile, write_volatile};

const ITERATIONS: c_int = 1000;

#[repr(C)]
pub struct timer_start_delete_race {
    pub progs: timer_start_delete_race__progs,
}

#[repr(C)]
pub struct timer_start_delete_race__progs {
    pub start_timer: *mut bpf_program,
    pub delete_elem: *mut bpf_program,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub retval: u32,
}

#[repr(C)]
pub struct cpu_set_t {
    _private: [u8; 128],
}

type pthread_t = usize;

#[repr(C)]
struct ctx {
    skel: *mut timer_start_delete_race,
    start: bool,
    stop: bool,
    errors: c_int,
}

unsafe extern "C" {
    fn CPU_ZERO(set: *mut cpu_set_t);
    fn CPU_SET(cpu: c_int, set: *mut cpu_set_t);
    fn pthread_self() -> pthread_t;
    fn pthread_setaffinity_np(thread: pthread_t, cpusetsize: usize, cpuset: *const cpu_set_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn usleep(usec: u32) -> c_int;

    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn timer_start_delete_race__open_and_load() -> *mut timer_start_delete_race;
    fn timer_start_delete_race__destroy(skel: *mut timer_start_delete_race);

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const u8) -> bool;
    fn ASSERT_OK(err: c_int, name: *const u8) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const u8) -> bool;
}

unsafe extern "C" fn start_timer_thread(arg: *mut c_void) -> *mut c_void {
    let ctx = arg as *mut ctx;
    let mut cpuset: cpu_set_t = unsafe { zeroed() };
    let fd: c_int;
    let mut i: c_int;

    unsafe {
        CPU_ZERO(&mut cpuset);
        CPU_SET(0, &mut cpuset);
        pthread_setaffinity_np(pthread_self(), size_of::<cpu_set_t>(), &cpuset);

        while !read_volatile(&(*ctx).start) && !read_volatile(&(*ctx).stop) {
            usleep(1);
        }
        if read_volatile(&(*ctx).stop) {
            return null_mut();
        }

        fd = bpf_program__fd((*(*ctx).skel).progs.start_timer);

        i = 0;
        while i < ITERATIONS && !read_volatile(&(*ctx).stop) {
            let mut opts: bpf_test_run_opts = zeroed();
            opts.sz = size_of::<bpf_test_run_opts>();
            let err: c_int;

            err = bpf_prog_test_run_opts(fd, &mut opts);
            if err != 0 || opts.retval != 0 {
                (*ctx).errors += 1;
                break;
            }

            i += 1;
        }
    }

    null_mut()
}

unsafe extern "C" fn delete_elem_thread(arg: *mut c_void) -> *mut c_void {
    let ctx = arg as *mut ctx;
    let mut cpuset: cpu_set_t = unsafe { zeroed() };
    let fd: c_int;
    let mut i: c_int;

    unsafe {
        CPU_ZERO(&mut cpuset);
        CPU_SET(1, &mut cpuset);
        pthread_setaffinity_np(pthread_self(), size_of::<cpu_set_t>(), &cpuset);

        while !read_volatile(&(*ctx).start) && !read_volatile(&(*ctx).stop) {
            usleep(1);
        }
        if read_volatile(&(*ctx).stop) {
            return null_mut();
        }

        fd = bpf_program__fd((*(*ctx).skel).progs.delete_elem);

        i = 0;
        while i < ITERATIONS && !read_volatile(&(*ctx).stop) {
            let mut opts: bpf_test_run_opts = zeroed();
            opts.sz = size_of::<bpf_test_run_opts>();
            let err: c_int;

            err = bpf_prog_test_run_opts(fd, &mut opts);
            if err != 0 || opts.retval != 0 {
                (*ctx).errors += 1;
                break;
            }

            i += 1;
        }
    }

    null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_timer_start_delete_race() {
    let skel: *mut timer_start_delete_race;
    let mut threads: [pthread_t; 2] = unsafe { zeroed() };
    let mut ctx: ctx = unsafe { zeroed() };
    let mut err: c_int;

    unsafe {
        skel = timer_start_delete_race__open_and_load();
        if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open_and_load".as_ptr() as *const u8) {
            return;
        }

        ctx.skel = skel;

        err = pthread_create(
            &mut threads[0],
            core::ptr::null(),
            start_timer_thread,
            &mut ctx as *mut ctx as *mut c_void,
        );
        if !ASSERT_OK(err, c"create start_timer_thread".as_ptr() as *const u8) {
            write_volatile(&mut ctx.stop, true);
            timer_start_delete_race__destroy(skel);
            return;
        }

        err = pthread_create(
            &mut threads[1],
            core::ptr::null(),
            delete_elem_thread,
            &mut ctx as *mut ctx as *mut c_void,
        );
        if !ASSERT_OK(err, c"create delete_elem_thread".as_ptr() as *const u8) {
            write_volatile(&mut ctx.stop, true);
            pthread_join(threads[0], null_mut());
            timer_start_delete_race__destroy(skel);
            return;
        }

        write_volatile(&mut ctx.start, true);

        pthread_join(threads[0], null_mut());
        pthread_join(threads[1], null_mut());

        ASSERT_EQ(ctx.errors, 0, c"thread_errors".as_ptr() as *const u8);

        /* Either KASAN will catch UAF or kernel will crash or nothing happens */
        timer_start_delete_race__destroy(skel);
    }
}
