// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */
/*
 * Translated from C source:
 *   testing/selftests/bpf/prog_tests/timer.c
 *
 * C dependencies removed from executable Rust:
 *   <sched.h>, <test_progs.h>, <linux/perf_event.h>, <sys/syscall.h>,
 *   "timer.skel.h", "timer_failure.skel.h", "timer_interrupt.skel.h"
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const NUM_THR: usize = 8;

const PERF_TYPE_HARDWARE: u32 = 0;
const PERF_COUNT_HW_CPU_CYCLES: u64 = 0;
const EOPNOTSUPP: c_int = 95;
const ENOENT: c_int = 2;
const __NR_perf_event_open: c_long = 298;

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
    pub sample_period: u64,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub retval: u32,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timer {
    pub data: *mut timer_data,
    pub bss: *mut timer_bss,
    pub progs: timer_progs,
}

#[repr(C)]
pub struct timer_data {
    pub callback_check: c_int,
    pub callback2_check: c_int,
}

#[repr(C)]
pub struct timer_bss {
    pub async_cancel: bool,
    pub pinned_callback_check: c_int,
    pub test_hits: c_int,
    pub update_hits: c_int,
    pub cancel_hits: c_int,
    pub bss_data: c_int,
    pub abs_data: c_int,
    pub err: c_int,
    pub ok: c_int,
}

#[repr(C)]
pub struct timer_progs {
    pub race: *mut bpf_program,
    pub nmi_race: *mut bpf_program,
    pub nmi_update: *mut bpf_program,
    pub nmi_cancel: *mut bpf_program,
    pub test1: *mut bpf_program,
    pub test_async_cancel_succeed: *mut bpf_program,
}

#[repr(C)]
pub struct timer_interrupt {
    pub bss: *mut timer_interrupt_bss,
    pub progs: timer_interrupt_progs,
}

#[repr(C)]
pub struct timer_interrupt_bss {
    pub in_interrupt: c_int,
    pub preempt_count: c_int,
    pub in_interrupt_cb: c_int,
}

#[repr(C)]
pub struct timer_interrupt_progs {
    pub test_timer_interrupt: *mut bpf_program,
}

type pthread_t = c_ulong;

unsafe extern "C" {
    static mut errno: c_int;

    fn syscall(num: c_long, ...) -> c_long;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_exit(retval: *mut c_void) -> !;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn fork() -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn usleep(usec: c_uint) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;

    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_program__attach_perf_event(prog: *mut bpf_program, pfd: c_int) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);

    fn timer__open_and_load() -> *mut timer;
    fn timer__attach(skel: *mut timer) -> c_int;
    fn timer__detach(skel: *mut timer);
    fn timer__destroy(skel: *mut timer);

    fn timer_interrupt__open_and_load() -> *mut timer_interrupt;
    fn timer_interrupt__attach(skel: *mut timer_interrupt) -> c_int;
    fn timer_interrupt__destroy(skel: *mut timer_interrupt);

    fn test__skip();

    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ(left: c_long, right: c_long, name: *const c_char) -> bool;
    fn ASSERT_NEQ(left: c_long, right: c_long, name: *const c_char) -> bool;
    fn ASSERT_GE(left: c_long, right: c_long, name: *const c_char) -> bool;
    fn ASSERT_GT(left: c_long, right: c_long, name: *const c_char) -> bool;
    fn RUN_TESTS_timer_failure();
}

fn libbpf_opts_bpf_test_run_opts() -> bpf_test_run_opts {
    bpf_test_run_opts {
        sz: size_of::<bpf_test_run_opts>(),
        retval: 0,
    }
}

unsafe fn perf_event_open(type_: u32, config: u64, pid: c_int, cpu: c_int) -> c_int {
    let mut attr = perf_event_attr {
        type_,
        config,
        size: size_of::<perf_event_attr>() as u32,
        sample_period: 10000,
    };

    syscall(
        __NR_perf_event_open,
        &mut attr as *mut perf_event_attr,
        pid,
        cpu,
        -1,
        0,
    ) as c_int
}

unsafe extern "C" fn spin_lock_thread(arg: *mut c_void) -> *mut c_void {
    let mut i: c_int;
    let mut err: c_int;
    let prog_fd: c_int = *(arg as *mut c_int);
    let mut topts = libbpf_opts_bpf_test_run_opts();

    i = 0;
    while i < 10000 {
        err = bpf_prog_test_run_opts(prog_fd, &mut topts);
        if !ASSERT_OK(err, c"test_run_opts err".as_ptr())
            || !ASSERT_OK(topts.retval as c_int, c"test_run_opts retval".as_ptr())
        {
            break;
        }
        i += 1;
    }

    pthread_exit(arg);
}

unsafe fn timer_stress_runner(timer_skel: *mut timer, async_cancel: bool) -> c_int {
    let mut i: c_int;
    let mut err: c_int = 1;
    let prog_fd: c_int;
    let mut _topts = libbpf_opts_bpf_test_run_opts();
    let mut thread_id: [pthread_t; NUM_THR] = [0; NUM_THR];
    let mut ret: *mut c_void = ptr::null_mut();

    (*(*timer_skel).bss).async_cancel = async_cancel;
    prog_fd = bpf_program__fd((*timer_skel).progs.race);
    i = 0;
    while i < NUM_THR as c_int {
        err = pthread_create(
            &mut thread_id[i as usize],
            ptr::null(),
            spin_lock_thread,
            &prog_fd as *const c_int as *mut c_void,
        );
        if !ASSERT_OK(err, c"pthread_create".as_ptr()) {
            break;
        }
        i += 1;
    }

    while i != 0 {
        i -= 1;
        err = pthread_join(thread_id[i as usize], &mut ret);
        if ASSERT_OK(err, c"pthread_join".as_ptr()) {
            ASSERT_EQ(
                ret as c_long,
                (&prog_fd as *const c_int as *mut c_void) as c_long,
                c"pthread_join".as_ptr(),
            );
        }
    }
    err
}

unsafe fn timer_stress(timer_skel: *mut timer) -> c_int {
    timer_stress_runner(timer_skel, false)
}

unsafe fn timer_stress_async_cancel(timer_skel: *mut timer) -> c_int {
    timer_stress_runner(timer_skel, true)
}

unsafe extern "C" fn nmi_cpu_worker(_arg: *mut c_void) -> *mut c_void {
    let mut num: u64 = 1;
    let mut i: c_int;

    i = 0;
    while i < 500000000 {
        num = num.wrapping_mul(((i % 7) + 1) as u64);
        i += 1;
    }
    let _ = core::ptr::read_volatile(&num);

    ptr::null_mut()
}

unsafe fn run_nmi_test(timer_skel: *mut timer, prog: *mut bpf_program) -> c_int {
    let mut link: *mut bpf_link = ptr::null_mut();
    let mut pe_fd: c_int = -1;
    let mut pipefd: [c_int; 2] = [-1, -1];
    let mut pid: c_int = 0;
    let mut status: c_int = 0;
    let mut buf: c_char = 0;
    let mut ret: c_int = -1;

    if !ASSERT_OK(pipe(pipefd.as_mut_ptr()), c"pipe".as_ptr()) {
        // goto cleanup
    } else {
        pid = fork();
        if pid == 0 {
            /* Child: spawn multiple threads to consume multiple CPUs */
            let mut threads: [pthread_t; NUM_THR] = [0; NUM_THR];
            let mut i: c_int;

            close(pipefd[1]);
            read(pipefd[0], &mut buf as *mut c_char as *mut c_void, 1);
            close(pipefd[0]);

            i = 0;
            while i < NUM_THR as c_int {
                pthread_create(
                    &mut threads[i as usize],
                    ptr::null(),
                    nmi_cpu_worker,
                    ptr::null_mut(),
                );
                i += 1;
            }
            i = 0;
            while i < NUM_THR as c_int {
                pthread_join(threads[i as usize], ptr::null_mut());
                i += 1;
            }
            exit(0);
        }

        if ASSERT_GE(pid as c_long, 0, c"fork".as_ptr()) {
            /* Open perf event for child process across all CPUs */
            pe_fd = perf_event_open(
                PERF_TYPE_HARDWARE,
                PERF_COUNT_HW_CPU_CYCLES,
                pid, /* measure child process */
                -1,  /* on any CPU */
            );
            if pe_fd < 0 {
                if errno == ENOENT || errno == EOPNOTSUPP {
                    printf(c"SKIP:no PERF_COUNT_HW_CPU_CYCLES\n".as_ptr());
                    test__skip();
                    ret = EOPNOTSUPP;
                } else {
                    ASSERT_GE(pe_fd as c_long, 0, c"perf_event_open".as_ptr());
                }
            } else {
                link = bpf_program__attach_perf_event(prog, pe_fd);
                if ASSERT_OK_PTR(link as *const c_void, c"attach_perf_event".as_ptr()) {
                    pe_fd = -1; /* Ownership transferred to link */

                    /* Signal child to start CPU work */
                    close(pipefd[0]);
                    pipefd[0] = -1;
                    write(pipefd[1], &buf as *const c_char as *const c_void, 1);
                    close(pipefd[1]);
                    pipefd[1] = -1;

                    waitpid(pid, &mut status, 0);
                    pid = 0;

                    /* Verify NMI context was hit */
                    ASSERT_GT((*(*timer_skel).bss).test_hits as c_long, 0, c"test_hits".as_ptr());
                    ret = 0;
                }
            }
        }
    }

    bpf_link__destroy(link);
    if pe_fd >= 0 {
        close(pe_fd);
    }
    if pid > 0 {
        write(pipefd[1], &buf as *const c_char as *const c_void, 1);
        waitpid(pid, &mut status, 0);
    }
    if pipefd[0] >= 0 {
        close(pipefd[0]);
    }
    if pipefd[1] >= 0 {
        close(pipefd[1]);
    }
    ret
}

unsafe fn timer_stress_nmi_race(timer_skel: *mut timer) -> c_int {
    let mut err: c_int;

    err = run_nmi_test(timer_skel, (*timer_skel).progs.nmi_race);
    if err == EOPNOTSUPP {
        return 0;
    }
    err
}

unsafe fn timer_stress_nmi_update(timer_skel: *mut timer) -> c_int {
    let mut err: c_int;

    err = run_nmi_test(timer_skel, (*timer_skel).progs.nmi_update);
    if err == EOPNOTSUPP {
        return 0;
    }
    if err != 0 {
        return err;
    }
    ASSERT_GT((*(*timer_skel).bss).update_hits as c_long, 0, c"update_hits".as_ptr());
    0
}

unsafe fn timer_stress_nmi_cancel(timer_skel: *mut timer) -> c_int {
    let mut err: c_int;

    err = run_nmi_test(timer_skel, (*timer_skel).progs.nmi_cancel);
    if err == EOPNOTSUPP {
        return 0;
    }
    if err != 0 {
        return err;
    }
    ASSERT_GT((*(*timer_skel).bss).cancel_hits as c_long, 0, c"cancel_hits".as_ptr());
    0
}

unsafe fn timer(timer_skel: *mut timer) -> c_int {
    let mut err: c_int;
    let prog_fd: c_int;
    let mut topts = libbpf_opts_bpf_test_run_opts();

    err = timer__attach(timer_skel);
    if !ASSERT_OK(err, c"timer_attach".as_ptr()) {
        return err;
    }

    ASSERT_EQ((*(*timer_skel).data).callback_check as c_long, 52, c"callback_check1".as_ptr());
    ASSERT_EQ(
        (*(*timer_skel).data).callback2_check as c_long,
        52,
        c"callback2_check1".as_ptr(),
    );
    ASSERT_EQ(
        (*(*timer_skel).bss).pinned_callback_check as c_long,
        0,
        c"pinned_callback_check1".as_ptr(),
    );

    prog_fd = bpf_program__fd((*timer_skel).progs.test1);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c"test_run".as_ptr());
    ASSERT_EQ(topts.retval as c_long, 0, c"test_run".as_ptr());
    timer__detach(timer_skel);

    usleep(50); /* 10 usecs should be enough, but give it extra */
    /* check that timer_cb1() was executed 10+10 times */
    ASSERT_EQ((*(*timer_skel).data).callback_check as c_long, 42, c"callback_check2".as_ptr());
    ASSERT_EQ(
        (*(*timer_skel).data).callback2_check as c_long,
        42,
        c"callback2_check2".as_ptr(),
    );

    /* check that timer_cb2() was executed twice */
    ASSERT_EQ((*(*timer_skel).bss).bss_data as c_long, 10, c"bss_data".as_ptr());

    /* check that timer_cb3() was executed twice */
    ASSERT_EQ((*(*timer_skel).bss).abs_data as c_long, 12, c"abs_data".as_ptr());

    /* check that timer_cb_pinned() was executed twice */
    ASSERT_EQ(
        (*(*timer_skel).bss).pinned_callback_check as c_long,
        2,
        c"pinned_callback_check".as_ptr(),
    );

    /* check that there were no errors in timer execution */
    ASSERT_EQ((*(*timer_skel).bss).err as c_long, 0, c"err".as_ptr());

    /* check that code paths completed */
    ASSERT_EQ((*(*timer_skel).bss).ok as c_long, (1 | 2 | 4), c"ok".as_ptr());

    0
}

unsafe fn timer_cancel_async(timer_skel: *mut timer) -> c_int {
    let mut err: c_int;
    let prog_fd: c_int;
    let mut topts = libbpf_opts_bpf_test_run_opts();

    prog_fd = bpf_program__fd((*timer_skel).progs.test_async_cancel_succeed);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c"test_run".as_ptr());
    ASSERT_EQ(topts.retval as c_long, 0, c"test_run".as_ptr());

    usleep(500);
    /* check that there were no errors in timer execution */
    ASSERT_EQ((*(*timer_skel).bss).err as c_long, 0, c"err".as_ptr());

    /* check that code paths completed */
    ASSERT_EQ((*(*timer_skel).bss).ok as c_long, (1 | 2 | 4), c"ok".as_ptr());

    0
}

unsafe fn test_timer(timer_test_fn: unsafe fn(*mut timer) -> c_int) {
    let mut timer_skel: *mut timer = ptr::null_mut();
    let mut err: c_int;

    timer_skel = timer__open_and_load();
    if timer_skel.is_null() && errno == EOPNOTSUPP {
        test__skip();
        return;
    }
    if !ASSERT_OK_PTR(timer_skel as *const c_void, c"timer_skel_load".as_ptr()) {
        return;
    }

    err = timer_test_fn(timer_skel);
    ASSERT_OK(err, c"timer".as_ptr());
    timer__destroy(timer_skel);
}

pub unsafe fn serial_test_timer() {
    test_timer(timer);

    RUN_TESTS_timer_failure();
}

pub unsafe fn serial_test_timer_stress() {
    test_timer(timer_stress);
}

pub unsafe fn serial_test_timer_stress_async_cancel() {
    test_timer(timer_stress_async_cancel);
}

pub unsafe fn serial_test_timer_async_cancel() {
    test_timer(timer_cancel_async);
}

pub unsafe fn serial_test_timer_stress_nmi_race() {
    test_timer(timer_stress_nmi_race);
}

pub unsafe fn serial_test_timer_stress_nmi_update() {
    test_timer(timer_stress_nmi_update);
}

pub unsafe fn serial_test_timer_stress_nmi_cancel() {
    test_timer(timer_stress_nmi_cancel);
}

pub unsafe fn test_timer_interrupt() {
    let mut skel: *mut timer_interrupt = ptr::null_mut();
    let mut err: c_int;
    let prog_fd: c_int;
    let mut opts = libbpf_opts_bpf_test_run_opts();

    skel = timer_interrupt__open_and_load();
    if skel.is_null() && errno == EOPNOTSUPP {
        test__skip();
        return;
    }
    if !ASSERT_OK_PTR(skel as *const c_void, c"timer_interrupt__open_and_load".as_ptr()) {
        return;
    }

    err = timer_interrupt__attach(skel);
    if !ASSERT_OK(err, c"timer_interrupt__attach".as_ptr()) {
        timer_interrupt__destroy(skel);
        return;
    }

    prog_fd = bpf_program__fd((*skel).progs.test_timer_interrupt);
    err = bpf_prog_test_run_opts(prog_fd, &mut opts);
    if !ASSERT_OK(err, c"bpf_prog_test_run_opts".as_ptr()) {
        timer_interrupt__destroy(skel);
        return;
    }

    usleep(50);

    ASSERT_EQ((*(*skel).bss).in_interrupt as c_long, 0, c"in_interrupt".as_ptr());
    if (*(*skel).bss).preempt_count != 0 {
        ASSERT_NEQ(
            (*(*skel).bss).in_interrupt_cb as c_long,
            0,
            c"in_interrupt_cb".as_ptr(),
        );
    }

    timer_interrupt__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
