// SPDX-License-Identifier: GPL-2.0
/*
 * Test for perf events with SIGTRAP across all threads.
 *
 * Copyright (C) 2021, Google LLC.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

/* Original C dependencies:
 * _GNU_SOURCE, sys/types.h, asm/siginfo.h, linux/hw_breakpoint.h,
 * linux/perf_event.h, pthread.h, signal.h, sys/ioctl.h, sys/syscall.h,
 * unistd.h, and kselftest_harness.h.
 *
 * This file intentionally references those ABI items as external dependencies.
 */

use core::ffi::{c_int, c_long, c_ulong, c_void};
use core::mem;
use core::ptr;
use core::sync::atomic::{AtomicI32, Ordering};

const NUM_THREADS: usize = 5;

type pid_t = c_int;
type size_t = usize;
type pthread_t = c_ulong;

#[repr(C)]
pub struct pthread_barrier_t {
    __private: [u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigset_t {
    __private: [c_ulong; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sigaction_handler {
    pub sa_handler: Option<unsafe extern "C" fn(c_int)>,
    pub sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigaction {
    pub __sigaction_handler: sigaction_handler,
    pub sa_mask: sigset_t,
    pub sa_flags: c_int,
    pub sa_restorer: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siginfo_t {
    pub si_signo: c_int,
    pub si_errno: c_int,
    pub si_code: c_int,
    pub si_addr: *mut c_void,
    pub si_perf_data: u64,
    pub si_perf_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
    pub sample_period: u64,
    pub sample_type: u64,
    pub read_format: u64,
    pub flags: u64,
    pub wakeup_events: u32,
    pub bp_type: u32,
    pub bp_addr: u64,
    pub bp_len: u64,
    pub branch_sample_type: u64,
    pub sample_regs_user: u64,
    pub sample_stack_user: u32,
    pub clockid: c_int,
    pub sample_regs_intr: u64,
    pub aux_watermark: u32,
    pub sample_max_stack: u16,
    pub __reserved_2: u16,
    pub aux_sample_size: u32,
    pub __reserved_3: u32,
    pub sig_data: u64,
}

const PERF_TYPE_BREAKPOINT: u32 = 5;
const HW_BREAKPOINT_RW: u32 = 3;
const HW_BREAKPOINT_LEN_1: u64 = 1;
const PERF_FLAG_FD_CLOEXEC: c_ulong = 8;
const PERF_EVENT_IOC_ENABLE: c_ulong = 9216;
const PERF_EVENT_IOC_DISABLE: c_ulong = 9217;
const PERF_EVENT_IOC_MODIFY_ATTRIBUTES: c_ulong = 1074275339;
const SA_SIGINFO: c_int = 4;
const SA_NODEFER: c_int = 0x40000000;
const SIGTRAP: c_int = 5;
const TRAP_PERF: c_int = 6;
const __NR_gettid: c_long = 186;
const __NR_perf_event_open: c_long = 298;

const PERF_ATTR_DISABLED: u64 = 1 << 0;
const PERF_ATTR_INHERIT: u64 = 1 << 1;
const PERF_ATTR_EXCLUDE_KERNEL: u64 = 1 << 5;
const PERF_ATTR_EXCLUDE_HV: u64 = 1 << 6;
const PERF_ATTR_INHERIT_THREAD: u64 = 1 << 36;
const PERF_ATTR_REMOVE_ON_EXEC: u64 = 1 << 37;
const PERF_ATTR_SIGTRAP: u64 = 1 << 38;

extern "C" {
    fn fprintf(stream: *mut c_void, format: *const u8, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn syscall(number: c_long, ...) -> c_long;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn pthread_barrier_init(
        barrier: *mut pthread_barrier_t,
        attr: *const c_void,
        count: u32,
    ) -> c_int;
    fn pthread_barrier_wait(barrier: *mut pthread_barrier_t) -> c_int;
    fn pthread_barrier_destroy(barrier: *mut pthread_barrier_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
}

/* Data shared between test body, threads, and signal handler. */
#[repr(C)]
struct Ctx {
    tids_want_signal: AtomicI32, /* Which threads still want a signal. */
    signal_count: AtomicI32,     /* Sanity check number of signals received. */
    iterate_on: i32,             /* Variable to set breakpoint on. */
    first_siginfo: siginfo_t,    /* First observed siginfo_t. */
}

static mut ctx: Ctx = Ctx {
    tids_want_signal: AtomicI32::new(0),
    signal_count: AtomicI32::new(0),
    iterate_on: 0,
    first_siginfo: siginfo_t {
        si_signo: 0,
        si_errno: 0,
        si_code: 0,
        si_addr: ptr::null_mut(),
        si_perf_data: 0,
        si_perf_type: 0,
    },
};

/* Unique value to check si_perf_data is correctly set from perf_event_attr::sig_data. */
fn TEST_SIG_DATA(addr: *const c_void, id: c_ulong) -> u64 {
    (!(addr as c_ulong)).wrapping_add(id) as u64
}

unsafe fn make_event_attr(enabled: bool, addr: *mut c_void, id: c_ulong) -> perf_event_attr {
    let mut attr: perf_event_attr = mem::zeroed();
    attr.type_ = PERF_TYPE_BREAKPOINT;
    attr.size = mem::size_of::<perf_event_attr>() as u32;
    attr.sample_period = 1;
    if !enabled {
        attr.flags |= PERF_ATTR_DISABLED;
    }
    attr.bp_addr = addr as u64;
    attr.bp_type = HW_BREAKPOINT_RW;
    attr.bp_len = HW_BREAKPOINT_LEN_1;
    attr.flags |= PERF_ATTR_INHERIT; /* Children inherit events ... */
    attr.flags |= PERF_ATTR_INHERIT_THREAD; /* ... but only cloned with CLONE_THREAD. */
    attr.flags |= PERF_ATTR_REMOVE_ON_EXEC; /* Required by sigtrap. */
    attr.flags |= PERF_ATTR_SIGTRAP; /* Request synchronous SIGTRAP on event. */
    attr.sig_data = TEST_SIG_DATA(addr, id);
    attr.flags |= PERF_ATTR_EXCLUDE_KERNEL; /* To allow */
    attr.flags |= PERF_ATTR_EXCLUDE_HV; /* running as !root */
    attr
}

unsafe extern "C" fn sigtrap_handler(signum: c_int, info: *mut siginfo_t, ucontext: *mut c_void) {
    let _ = signum;
    let _ = ucontext;

    if (*info).si_code != TRAP_PERF {
        fprintf(
            stderr,
            b"%s: unexpected si_code %d\n\0".as_ptr(),
            b"sigtrap_handler\0".as_ptr(),
            (*info).si_code,
        );
        return;
    }

    /*
     * The data in siginfo_t we're interested in should all be the same
     * across threads.
     */
    if ctx.signal_count.fetch_add(1, Ordering::Relaxed) == 0 {
        ctx.first_siginfo = *info;
    }
    ctx.tids_want_signal
        .fetch_sub(syscall(__NR_gettid) as i32, Ordering::Relaxed);
}

unsafe extern "C" fn test_thread(arg: *mut c_void) -> *mut c_void {
    let barrier: *mut pthread_barrier_t = arg as *mut pthread_barrier_t;
    let tid: pid_t = syscall(__NR_gettid) as pid_t;
    let mut iter: c_int;
    let mut i: c_int;

    pthread_barrier_wait(barrier);

    ctx.tids_want_signal.fetch_add(tid, Ordering::Relaxed);
    iter = core::ptr::read_volatile(core::ptr::addr_of!(ctx.iterate_on)); /* read */
    if iter >= 0 {
        i = 0;
        while i < iter - 1 {
            ctx.tids_want_signal.fetch_add(tid, Ordering::Relaxed);
            core::ptr::write_volatile(core::ptr::addr_of_mut!(ctx.iterate_on), iter); /* idempotent write */
            i += 1;
        }
    } else {
        while core::ptr::read_volatile(core::ptr::addr_of!(ctx.iterate_on)) != 0 {}
    }

    ptr::null_mut()
}

#[repr(C)]
struct sigtrap_threads {
    oldact: sigaction,
    threads: [pthread_t; NUM_THREADS],
    barrier: pthread_barrier_t,
    fd: c_int,
}

unsafe fn sigtrap_threads_setup(self_: *mut sigtrap_threads) {
    let attr = make_event_attr(
        false,
        core::ptr::addr_of_mut!(ctx.iterate_on) as *mut c_void,
        0,
    );
    let mut action: sigaction = mem::zeroed();
    let mut i: c_int;

    ctx = Ctx {
        tids_want_signal: AtomicI32::new(0),
        signal_count: AtomicI32::new(0),
        iterate_on: 0,
        first_siginfo: mem::zeroed(),
    };

    /* Initialize sigtrap handler. */
    action.sa_flags = SA_SIGINFO | SA_NODEFER;
    action.__sigaction_handler.sa_sigaction = Some(sigtrap_handler);
    sigemptyset(&mut action.sa_mask);
    ASSERT_EQ(sigaction(SIGTRAP, &action, &mut (*self_).oldact), 0);

    /* Initialize perf event. */
    (*self_).fd = syscall(
        __NR_perf_event_open,
        &attr as *const perf_event_attr,
        0,
        -1,
        -1,
        PERF_FLAG_FD_CLOEXEC,
    ) as c_int;
    ASSERT_NE((*self_).fd, -1);

    /* Spawn threads inheriting perf event. */
    pthread_barrier_init(&mut (*self_).barrier, ptr::null(), (NUM_THREADS + 1) as u32);
    i = 0;
    while i < NUM_THREADS as c_int {
        ASSERT_EQ(
            pthread_create(
                &mut (*self_).threads[i as usize],
                ptr::null(),
                Some(test_thread),
                &mut (*self_).barrier as *mut pthread_barrier_t as *mut c_void,
            ),
            0,
        );
        i += 1;
    }
}

unsafe fn sigtrap_threads_teardown(self_: *mut sigtrap_threads) {
    pthread_barrier_destroy(&mut (*self_).barrier);
    close((*self_).fd);
    sigaction(SIGTRAP, &(*self_).oldact, ptr::null_mut());
}

unsafe fn run_test_threads(_metadata: *mut __test_metadata, self_: *mut sigtrap_threads) {
    let mut i: c_int;

    pthread_barrier_wait(&mut (*self_).barrier);
    i = 0;
    while i < NUM_THREADS as c_int {
        ASSERT_EQ(pthread_join((*self_).threads[i as usize], ptr::null_mut()), 0);
        i += 1;
    }
}

unsafe fn remain_disabled(_metadata: *mut __test_metadata, self_: *mut sigtrap_threads) {
    run_test_threads(_metadata, self_);
    EXPECT_EQ(ctx.signal_count.load(Ordering::Relaxed), 0);
    EXPECT_NE(ctx.tids_want_signal.load(Ordering::Relaxed), 0);
}

unsafe fn enable_event(_metadata: *mut __test_metadata, self_: *mut sigtrap_threads) {
    EXPECT_EQ(ioctl((*self_).fd, PERF_EVENT_IOC_ENABLE, 0), 0);
    run_test_threads(_metadata, self_);

    EXPECT_EQ(ctx.signal_count.load(Ordering::Relaxed), NUM_THREADS as i32);
    EXPECT_EQ(ctx.tids_want_signal.load(Ordering::Relaxed), 0);
    EXPECT_EQ(
        ctx.first_siginfo.si_addr,
        core::ptr::addr_of_mut!(ctx.iterate_on) as *mut c_void,
    );
    EXPECT_EQ(ctx.first_siginfo.si_perf_type, PERF_TYPE_BREAKPOINT);
    EXPECT_EQ(
        ctx.first_siginfo.si_perf_data,
        TEST_SIG_DATA(core::ptr::addr_of!(ctx.iterate_on) as *const c_void, 0),
    );

    /* Check enabled for parent. */
    core::ptr::write_volatile(core::ptr::addr_of_mut!(ctx.iterate_on), 0);
    EXPECT_EQ(ctx.signal_count.load(Ordering::Relaxed), NUM_THREADS as i32 + 1);
}

/* Test that modification propagates to all inherited events. */
unsafe fn modify_and_enable_event(_metadata: *mut __test_metadata, self_: *mut sigtrap_threads) {
    let new_attr = make_event_attr(
        true,
        core::ptr::addr_of_mut!(ctx.iterate_on) as *mut c_void,
        42,
    );

    EXPECT_EQ(
        ioctl(
            (*self_).fd,
            PERF_EVENT_IOC_MODIFY_ATTRIBUTES,
            &new_attr as *const perf_event_attr,
        ),
        0,
    );
    run_test_threads(_metadata, self_);

    EXPECT_EQ(ctx.signal_count.load(Ordering::Relaxed), NUM_THREADS as i32);
    EXPECT_EQ(ctx.tids_want_signal.load(Ordering::Relaxed), 0);
    EXPECT_EQ(
        ctx.first_siginfo.si_addr,
        core::ptr::addr_of_mut!(ctx.iterate_on) as *mut c_void,
    );
    EXPECT_EQ(ctx.first_siginfo.si_perf_type, PERF_TYPE_BREAKPOINT);
    EXPECT_EQ(
        ctx.first_siginfo.si_perf_data,
        TEST_SIG_DATA(core::ptr::addr_of!(ctx.iterate_on) as *const c_void, 42),
    );

    /* Check enabled for parent. */
    core::ptr::write_volatile(core::ptr::addr_of_mut!(ctx.iterate_on), 0);
    EXPECT_EQ(ctx.signal_count.load(Ordering::Relaxed), NUM_THREADS as i32 + 1);
}

/* Stress test event + signal handling. */
unsafe fn signal_stress(_metadata: *mut __test_metadata, self_: *mut sigtrap_threads) {
    core::ptr::write_volatile(core::ptr::addr_of_mut!(ctx.iterate_on), 3000);

    EXPECT_EQ(ioctl((*self_).fd, PERF_EVENT_IOC_ENABLE, 0), 0);
    run_test_threads(_metadata, self_);
    EXPECT_EQ(ioctl((*self_).fd, PERF_EVENT_IOC_DISABLE, 0), 0);

    EXPECT_EQ(
        ctx.signal_count.load(Ordering::Relaxed),
        NUM_THREADS as i32 * core::ptr::read_volatile(core::ptr::addr_of!(ctx.iterate_on)),
    );
    EXPECT_EQ(ctx.tids_want_signal.load(Ordering::Relaxed), 0);
    EXPECT_EQ(
        ctx.first_siginfo.si_addr,
        core::ptr::addr_of_mut!(ctx.iterate_on) as *mut c_void,
    );
    EXPECT_EQ(ctx.first_siginfo.si_perf_type, PERF_TYPE_BREAKPOINT);
    EXPECT_EQ(
        ctx.first_siginfo.si_perf_data,
        TEST_SIG_DATA(core::ptr::addr_of!(ctx.iterate_on) as *const c_void, 0),
    );
}

unsafe fn signal_stress_with_disable(_metadata: *mut __test_metadata, self_: *mut sigtrap_threads) {
    let target_count: c_int = NUM_THREADS as c_int * 3000;
    let mut i: c_int;

    core::ptr::write_volatile(core::ptr::addr_of_mut!(ctx.iterate_on), -1);

    EXPECT_EQ(ioctl((*self_).fd, PERF_EVENT_IOC_ENABLE, 0), 0);
    pthread_barrier_wait(&mut (*self_).barrier);
    while ctx.signal_count.load(Ordering::Relaxed) < target_count {
        EXPECT_EQ(ioctl((*self_).fd, PERF_EVENT_IOC_DISABLE, 0), 0);
        EXPECT_EQ(ioctl((*self_).fd, PERF_EVENT_IOC_ENABLE, 0), 0);
    }
    core::ptr::write_volatile(core::ptr::addr_of_mut!(ctx.iterate_on), 0);
    i = 0;
    while i < NUM_THREADS as c_int {
        ASSERT_EQ(pthread_join((*self_).threads[i as usize], ptr::null_mut()), 0);
        i += 1;
    }
    EXPECT_EQ(ioctl((*self_).fd, PERF_EVENT_IOC_DISABLE, 0), 0);

    EXPECT_EQ(
        ctx.first_siginfo.si_addr,
        core::ptr::addr_of_mut!(ctx.iterate_on) as *mut c_void,
    );
    EXPECT_EQ(ctx.first_siginfo.si_perf_type, PERF_TYPE_BREAKPOINT);
    EXPECT_EQ(
        ctx.first_siginfo.si_perf_data,
        TEST_SIG_DATA(core::ptr::addr_of!(ctx.iterate_on) as *const c_void, 0),
    );
}

extern "C" {
    type __test_metadata;
}

extern "Rust" {
    fn ASSERT_EQ<T: PartialEq + core::fmt::Debug>(left: T, right: T);
    fn ASSERT_NE<T: PartialEq + core::fmt::Debug>(left: T, right: T);
    fn EXPECT_EQ<T: PartialEq + core::fmt::Debug>(left: T, right: T);
    fn EXPECT_NE<T: PartialEq + core::fmt::Debug>(left: T, right: T);
}

/* TEST_HARNESS_MAIN */
