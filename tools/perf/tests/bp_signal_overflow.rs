// SPDX-License-Identifier: GPL-2.0
/*
 * Originally done by Vince Weaver <vincent.weaver@maine.edu> for
 * perf_event_tests (git://github.com/deater/perf_event_tests)
 */

/*
 * Powerpc needs __SANE_USERSPACE_TYPES__ before <linux/types.h> to select
 * 'int-ll64.h' and avoid compile warnings when printing __u64 with %llu.
 *
 * C source defined __SANE_USERSPACE_TYPES__ before including Linux headers.
 */

use core::ffi::{c_int, c_long, c_ulong, c_void};

static mut overflows: c_int = 0;

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct siginfo_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sigaction {
    pub sa_sigaction: usize,
    pub sa_flags: c_int,
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
    pub sample_period: u64,
    pub sample_type: u64,
    pub wakeup_events: u32,
    pub bp_type: u32,
    pub bp_addr: u64,
    pub bp_len: u64,
    pub disabled: u64,
    pub exclude_kernel: u64,
    pub exclude_hv: u64,
}

const TEST_FAIL: c_int = -1;
const TEST_SKIP: c_int = -2;
const TEST_OK: c_int = 0;

const SIGIO: c_int = 29;
const SA_SIGINFO: c_int = 4;
const F_SETFL: c_int = 4;
const F_SETOWN: c_int = 8;
const F_SETSIG: c_int = 10;
const O_RDWR: c_int = 0o00000002;
const O_NONBLOCK: c_int = 0o00004000;
const O_ASYNC: c_int = 0o00020000;

const PERF_TYPE_BREAKPOINT: u32 = 5;
const PERF_SAMPLE_IP: u64 = 1;
const PERF_EVENT_IOC_ENABLE: c_ulong = 9216;
const PERF_EVENT_IOC_DISABLE: c_ulong = 9217;
const PERF_EVENT_IOC_RESET: c_ulong = 9219;
const HW_BREAKPOINT_X: u32 = 4;

const EXECUTIONS: c_int = 10000;
const THRESHOLD: c_int = 100;

unsafe extern "C" {
    static BP_SIGNAL_IS_SUPPORTED: bool;

    fn time(tloc: *mut c_long) -> c_long;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn getpid() -> c_int;
    fn close(fd: c_int) -> c_int;

    fn default_breakpoint_len() -> u64;
    fn sys_perf_event_open(
        attr: *mut perf_event_attr,
        pid: c_int,
        cpu: c_int,
        group_fd: c_int,
        flags: c_ulong,
    ) -> c_int;
    fn perf_event_open_cloexec_flag() -> c_ulong;
    fn pr_debug(fmt: *const i8, ...);
}

#[inline(never)]
unsafe extern "C" fn test_function() -> c_int {
    time(core::ptr::null_mut()) as c_int
}

unsafe extern "C" fn sig_handler(
    _signum: c_int,
    _oh: *mut siginfo_t,
    _uc: *mut c_void,
) {
    overflows += 1;
}

unsafe fn bp_count(fd: c_int) -> i64 {
    let mut count: i64 = 0;
    let ret: c_int = read(
        fd,
        &mut count as *mut i64 as *mut c_void,
        core::mem::size_of::<i64>(),
    ) as c_int;

    if ret != core::mem::size_of::<i64>() as c_int {
        pr_debug(c"failed to read: %d\n".as_ptr());
        return TEST_FAIL as i64;
    }

    count
}

unsafe fn test__bp_signal_overflow(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut pe: perf_event_attr = core::mem::zeroed();
    let mut sa: sigaction = core::mem::zeroed();
    let mut count: i64;
    let fd: c_int;
    let mut i: c_int;
    let mut fails: c_int = 0;

    if !BP_SIGNAL_IS_SUPPORTED {
        pr_debug(c"Test not supported on this architecture".as_ptr());
        return TEST_SKIP;
    }

    /* setup SIGIO signal handler */
    memset(
        &mut sa as *mut sigaction as *mut c_void,
        0,
        core::mem::size_of::<sigaction>(),
    );
    sa.sa_sigaction = sig_handler as usize;
    sa.sa_flags = SA_SIGINFO;

    if sigaction(SIGIO, &sa, core::ptr::null_mut()) < 0 {
        pr_debug(c"failed setting up signal handler\n".as_ptr());
        return TEST_FAIL;
    }

    memset(
        &mut pe as *mut perf_event_attr as *mut c_void,
        0,
        core::mem::size_of::<perf_event_attr>(),
    );
    pe.type_ = PERF_TYPE_BREAKPOINT;
    pe.size = core::mem::size_of::<perf_event_attr>() as u32;

    pe.config = 0;
    pe.bp_type = HW_BREAKPOINT_X;
    pe.bp_addr = test_function as usize as c_ulong as u64;
    pe.bp_len = default_breakpoint_len();

    pe.sample_period = THRESHOLD as u64;
    pe.sample_type = PERF_SAMPLE_IP;
    pe.wakeup_events = 1;

    pe.disabled = 1;
    pe.exclude_kernel = 1;
    pe.exclude_hv = 1;

    fd = sys_perf_event_open(
        &mut pe,
        0,
        -1,
        -1,
        perf_event_open_cloexec_flag(),
    );
    if fd < 0 {
        pr_debug(c"failed opening event %llx\n".as_ptr());
        return TEST_FAIL;
    }

    fcntl(fd, F_SETFL, O_RDWR | O_NONBLOCK | O_ASYNC);
    fcntl(fd, F_SETSIG, SIGIO);
    fcntl(fd, F_SETOWN, getpid());

    ioctl(fd, PERF_EVENT_IOC_RESET, 0);
    ioctl(fd, PERF_EVENT_IOC_ENABLE, 0);

    i = 0;
    while i < EXECUTIONS {
        test_function();
        i += 1;
    }

    ioctl(fd, PERF_EVENT_IOC_DISABLE, 0);

    count = bp_count(fd);

    close(fd);

    pr_debug(c"count %lld, overflow %d\n".as_ptr());

    if count != EXECUTIONS as i64 {
        pr_debug(c"\tWrong number of executions %lld != %d\n".as_ptr());
        fails += 1;
    }

    if overflows != EXECUTIONS / THRESHOLD {
        pr_debug(c"\tWrong number of overflows %d != %d\n".as_ptr());
        fails += 1;
    }

    if fails != 0 {
        TEST_FAIL
    } else {
        TEST_OK
    }
}

/* DEFINE_SUITE("Breakpoint overflow sampling", bp_signal_overflow); */
