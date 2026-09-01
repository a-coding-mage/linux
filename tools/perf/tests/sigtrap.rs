// SPDX-License-Identifier: GPL-2.0
/*
 * Basic test for sigtrap support.
 *
 * Copyright (C) 2021, Google LLC.
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem;
use core::ptr;
use core::sync::atomic::{AtomicI32, Ordering};

const NUM_THREADS: c_int = 5;

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
    pub sample_period: u64,
    pub disabled: u64,
    pub bp_addr: u64,
    pub bp_type: u32,
    pub bp_len: u32,
    pub inherit: u64,
    pub inherit_thread: u64,
    pub remove_on_exec: u64,
    pub sigtrap: u64,
    pub sig_data: u64,
    pub exclude_kernel: u64,
    pub exclude_hv: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siginfo_t {
    pub si_addr: *mut c_void,
}

#[repr(C)]
pub struct sigset_t {
    _private: [u64; 16],
}

#[repr(C)]
pub struct sigaction {
    pub sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
    pub sa_mask: sigset_t,
    pub sa_flags: c_int,
}

#[repr(C)]
pub struct pthread_barrier_t {
    _private: [c_char; 32],
}

pub type pthread_t = c_ulong;
pub type pid_t = c_int;

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_member {
    pub type_: u32,
}

#[repr(C)]
pub struct btf_type {
    pub name_off: u32,
}

#[repr(C)]
struct Ctx {
    tids_want_signal: AtomicI32, /* Which threads still want a signal. */
    signal_count: AtomicI32,     /* Sanity check number of signals received. */
    iterate_on: c_int,           /* Variable to set breakpoint on. */
    first_siginfo: siginfo_t,    /* First observed siginfo_t. */
}

static mut ctx: Ctx = Ctx {
    tids_want_signal: AtomicI32::new(0),
    signal_count: AtomicI32::new(0),
    iterate_on: 0,
    first_siginfo: siginfo_t {
        si_addr: ptr::null_mut(),
    },
};

unsafe fn TEST_SIG_DATA() -> c_ulong {
    !((&raw const ctx.iterate_on) as c_ulong)
}

const PERF_TYPE_BREAKPOINT: u32 = 5;
const PERF_TYPE_SOFTWARE: u32 = 1;
const PERF_COUNT_SW_DUMMY: u64 = 9;
const HW_BREAKPOINT_RW: u32 = 3;
const HW_BREAKPOINT_LEN_1: u32 = 1;
const BTF_KIND_STRUCT: c_int = 4;
const PERF_EVENT_IOC_ENABLE: c_ulong = 0x2400;
const PERF_EVENT_IOC_DISABLE: c_ulong = 0x2401;
const SYS_GETTID: c_long = 186;
const SA_SIGINFO: c_int = 4;
const SA_NODEFER: c_int = 0x40000000;
const SIGTRAP: c_int = 5;
const STRERR_BUFSIZE: usize = 128;
const TEST_OK: c_int = 0;
const TEST_FAIL: c_int = -1;
const TEST_SKIP: c_int = -2;
const BP_SIGNAL_IS_SUPPORTED: bool = true;

extern "C" {
    fn syscall(num: c_long, ...) -> c_long;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn pthread_barrier_init(
        barrier: *mut pthread_barrier_t,
        attr: *const c_void,
        count: c_uint,
    ) -> c_int;
    fn pthread_barrier_wait(barrier: *mut pthread_barrier_t) -> c_int;
    fn pthread_barrier_destroy(barrier: *mut pthread_barrier_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn sys_perf_event_open(
        attr: *mut perf_event_attr,
        pid: pid_t,
        cpu: c_int,
        group_fd: c_int,
        flags: c_ulong,
    ) -> c_int;
    fn perf_event_open_cloexec_flag() -> c_ulong;
    fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: usize) -> *mut c_char;
    fn pr_debug(fmt: *const c_char, ...);
    fn TEST_ASSERT_EQUAL(msg: *const c_char, val: c_int, expected: c_int);
    fn TEST_ASSERT_VAL(msg: *const c_char, val: bool);
    fn btf__load_vmlinux_btf() -> *mut btf;
    fn btf__free(btf: *mut btf);
    fn btf__find_by_name_kind(btf: *mut btf, name: *const c_char, kind: c_int) -> c_int;
    fn __btf_type__find_member_by_name(
        btf: *mut btf,
        id: c_int,
        name: *const c_char,
    ) -> *const btf_member;
    fn btf__type_by_id(btf: *mut btf, id: u32) -> *const btf_type;
    fn btf_is_struct(type_: *const btf_type) -> bool;
    fn btf__name_by_offset(btf: *mut btf, offset: u32) -> *const c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

pub type c_uint = u32;

unsafe fn make_event_attr() -> perf_event_attr {
    let attr = perf_event_attr {
        type_: PERF_TYPE_BREAKPOINT,
        size: mem::size_of::<perf_event_attr>() as u32,
        config: 0,
        sample_period: 1,
        disabled: 1,
        bp_addr: (&raw const ctx.iterate_on) as u64,
        bp_type: HW_BREAKPOINT_RW,
        bp_len: HW_BREAKPOINT_LEN_1,
        inherit: 1,        /* Children inherit events ... */
        inherit_thread: 1, /* ... but only cloned with CLONE_THREAD. */
        remove_on_exec: 1, /* Required by sigtrap. */
        sigtrap: 1,        /* Request synchronous SIGTRAP on event. */
        sig_data: TEST_SIG_DATA() as u64,
        exclude_kernel: 1, /* To allow */
        exclude_hv: 1,     /* running as !root */
    };
    attr
}

/* #ifdef HAVE_BPF_SKEL */
#[cfg(HAVE_BPF_SKEL)]
static mut btf: *mut btf = ptr::null_mut();

#[cfg(HAVE_BPF_SKEL)]
unsafe fn btf__available() -> bool {
    if btf.is_null() {
        btf = btf__load_vmlinux_btf();
    }

    !btf.is_null()
}

#[cfg(HAVE_BPF_SKEL)]
unsafe fn btf__exit() {
    btf__free(btf);
    btf = ptr::null_mut();
}

#[cfg(HAVE_BPF_SKEL)]
unsafe fn attr_has_sigtrap() -> bool {
    let id: c_int;

    if !btf__available() {
        /* should be an old kernel */
        return false;
    }

    id = btf__find_by_name_kind(btf, b"perf_event_attr\0".as_ptr() as *const c_char, BTF_KIND_STRUCT);
    if id < 0 {
        return false;
    }

    !__btf_type__find_member_by_name(btf, id, b"sigtrap\0".as_ptr() as *const c_char).is_null()
}

#[cfg(HAVE_BPF_SKEL)]
unsafe fn kernel_with_sleepable_spinlocks() -> bool {
    let member: *const btf_member;
    let type_: *const btf_type;
    let type_name: *const c_char;
    let id: c_int;

    if !btf__available() {
        return false;
    }

    id = btf__find_by_name_kind(btf, b"spinlock\0".as_ptr() as *const c_char, BTF_KIND_STRUCT);
    if id < 0 {
        return false;
    }

    // Only RT has a "lock" member for "struct spinlock"
    member = __btf_type__find_member_by_name(btf, id, b"lock\0".as_ptr() as *const c_char);
    if member.is_null() {
        return false;
    }

    // But check its type as well
    type_ = btf__type_by_id(btf, (*member).type_);
    if type_.is_null() || !btf_is_struct(type_) {
        return false;
    }

    type_name = btf__name_by_offset(btf, (*type_).name_off);
    !type_name.is_null() && strcmp(type_name, b"rt_mutex_base\0".as_ptr() as *const c_char) != 0
}

/* #else  !HAVE_BPF_SKEL */
#[cfg(not(HAVE_BPF_SKEL))]
unsafe fn attr_has_sigtrap() -> bool {
    let mut attr = perf_event_attr {
        type_: PERF_TYPE_SOFTWARE,
        size: mem::size_of::<perf_event_attr>() as u32,
        config: PERF_COUNT_SW_DUMMY,
        sample_period: 0,
        disabled: 0,
        bp_addr: 0,
        bp_type: 0,
        bp_len: 0,
        inherit: 0,
        inherit_thread: 0,
        remove_on_exec: 1, /* Required by sigtrap. */
        sigtrap: 1,        /* Request synchronous SIGTRAP on event. */
        sig_data: 0,
        exclude_kernel: 0,
        exclude_hv: 0,
    };
    let fd: c_int;
    let mut ret = false;

    fd = sys_perf_event_open(&mut attr, 0, -1, -1, perf_event_open_cloexec_flag());
    if fd >= 0 {
        ret = true;
        close(fd);
    }

    ret
}

#[cfg(not(HAVE_BPF_SKEL))]
unsafe fn kernel_with_sleepable_spinlocks() -> bool {
    false
}

#[cfg(not(HAVE_BPF_SKEL))]
unsafe fn btf__exit() {}
/* #endif  HAVE_BPF_SKEL */

unsafe extern "C" fn sigtrap_handler(
    _signum: c_int,
    info: *mut siginfo_t,
    _ucontext: *mut c_void,
) {
    if ctx.signal_count.fetch_add(1, Ordering::Relaxed) == 0 {
        ctx.first_siginfo = *info;
    }
    ctx.tids_want_signal.fetch_sub(syscall(SYS_GETTID) as c_int, Ordering::Relaxed);
}

unsafe extern "C" fn test_thread(arg: *mut c_void) -> *mut c_void {
    let barrier = arg as *mut pthread_barrier_t;
    let tid: pid_t = syscall(SYS_GETTID) as pid_t;
    let mut i: c_int;

    pthread_barrier_wait(barrier);

    ctx.tids_want_signal.fetch_add(tid, Ordering::Relaxed);
    i = 0;
    while i < ctx.iterate_on - 1 {
        ctx.tids_want_signal.fetch_add(tid, Ordering::Relaxed);
        i += 1;
    }

    ptr::null_mut()
}

unsafe fn run_test_threads(threads: *mut pthread_t, barrier: *mut pthread_barrier_t) -> c_int {
    let mut i: c_int;

    pthread_barrier_wait(barrier);
    i = 0;
    while i < NUM_THREADS {
        TEST_ASSERT_EQUAL(
            b"pthread_join() failed\0".as_ptr() as *const c_char,
            pthread_join(*threads.add(i as usize), ptr::null_mut()),
            0,
        );
        i += 1;
    }

    TEST_OK
}

unsafe fn run_stress_test(
    fd: c_int,
    threads: *mut pthread_t,
    barrier: *mut pthread_barrier_t,
) -> c_int {
    let ret: c_int;
    let expected_sigtraps: c_int;

    ctx.iterate_on = 3000;

    TEST_ASSERT_EQUAL(
        b"misfired signal?\0".as_ptr() as *const c_char,
        ctx.signal_count.load(Ordering::Relaxed),
        0,
    );
    TEST_ASSERT_EQUAL(
        b"enable failed\0".as_ptr() as *const c_char,
        ioctl(fd, PERF_EVENT_IOC_ENABLE, 0),
        0,
    );
    ret = run_test_threads(threads, barrier);
    TEST_ASSERT_EQUAL(
        b"disable failed\0".as_ptr() as *const c_char,
        ioctl(fd, PERF_EVENT_IOC_DISABLE, 0),
        0,
    );

    expected_sigtraps = NUM_THREADS * ctx.iterate_on;

    if ctx.signal_count.load(Ordering::Relaxed) < expected_sigtraps
        && kernel_with_sleepable_spinlocks()
    {
        pr_debug(
            b"Expected %d sigtraps, got %d, running on a kernel with sleepable spinlocks.\n\0"
                .as_ptr() as *const c_char,
            expected_sigtraps,
            ctx.signal_count.load(Ordering::Relaxed),
        );
        pr_debug(
            b"See https://lore.kernel.org/all/e368f2c848d77fbc8d259f44e2055fe469c219cf.camel@gmx.de/\n\0"
                .as_ptr() as *const c_char,
        );
        return TEST_SKIP;
    } else {
        TEST_ASSERT_EQUAL(
            b"unexpected sigtraps\0".as_ptr() as *const c_char,
            ctx.signal_count.load(Ordering::Relaxed),
            expected_sigtraps,
        );
    }

    TEST_ASSERT_EQUAL(
        b"missing signals or incorrectly delivered\0".as_ptr() as *const c_char,
        ctx.tids_want_signal.load(Ordering::Relaxed),
        0,
    );
    TEST_ASSERT_VAL(
        b"unexpected si_addr\0".as_ptr() as *const c_char,
        ctx.first_siginfo.si_addr == (&raw mut ctx.iterate_on) as *mut c_void,
    );
    /*
     * #if 0
     * FIXME: enable when libc's signal.h has si_perf_{type,data}
     * TEST_ASSERT_EQUAL("unexpected si_perf_type", ctx.first_siginfo.si_perf_type,
     *                   PERF_TYPE_BREAKPOINT);
     * TEST_ASSERT_EQUAL("unexpected si_perf_data", ctx.first_siginfo.si_perf_data,
     *                   TEST_SIG_DATA);
     * #endif
     */

    ret
}

unsafe fn test__sigtrap(_test: *mut test_suite, _subtest: c_int) -> c_int {
    let mut attr = make_event_attr();
    let mut action: sigaction = mem::zeroed();
    let mut oldact: sigaction = mem::zeroed();
    let mut threads: [pthread_t; NUM_THREADS as usize] = [0; NUM_THREADS as usize];
    let mut barrier: pthread_barrier_t = mem::zeroed();
    let mut sbuf: [c_char; STRERR_BUFSIZE] = [0; STRERR_BUFSIZE];
    let mut i: c_int;
    let fd: c_int;
    let mut ret: c_int = TEST_FAIL;

    if !BP_SIGNAL_IS_SUPPORTED {
        pr_debug(b"Test not supported on this architecture\0".as_ptr() as *const c_char);
        return TEST_SKIP;
    }

    pthread_barrier_init(&mut barrier, ptr::null(), (NUM_THREADS + 1) as c_uint);

    action.sa_flags = SA_SIGINFO | SA_NODEFER;
    action.sa_sigaction = Some(sigtrap_handler);
    sigemptyset(&mut action.sa_mask);
    if sigaction(SIGTRAP, &action, &mut oldact) != 0 {
        pr_debug(
            b"FAILED sigaction(): %s\n\0".as_ptr() as *const c_char,
            str_error_r(errno(), sbuf.as_mut_ptr(), sbuf.len()),
        );
        pthread_barrier_destroy(&mut barrier);
        btf__exit();
        return ret;
    }

    fd = sys_perf_event_open(&mut attr, 0, -1, -1, perf_event_open_cloexec_flag());
    if fd < 0 {
        if attr_has_sigtrap() {
            pr_debug(
                b"FAILED sys_perf_event_open(): %s\n\0".as_ptr() as *const c_char,
                str_error_r(errno(), sbuf.as_mut_ptr(), sbuf.len()),
            );
        } else {
            pr_debug(b"perf_event_attr doesn't have sigtrap\n\0".as_ptr() as *const c_char);
            ret = TEST_SKIP;
        }
        sigaction(SIGTRAP, &oldact, ptr::null_mut());
        pthread_barrier_destroy(&mut barrier);
        btf__exit();
        return ret;
    }

    i = 0;
    while i < NUM_THREADS {
        if pthread_create(
            &mut threads[i as usize],
            ptr::null(),
            test_thread,
            &mut barrier as *mut pthread_barrier_t as *mut c_void,
        ) != 0
        {
            pr_debug(
                b"FAILED pthread_create(): %s\n\0".as_ptr() as *const c_char,
                str_error_r(errno(), sbuf.as_mut_ptr(), sbuf.len()),
            );
            close(fd);
            sigaction(SIGTRAP, &oldact, ptr::null_mut());
            pthread_barrier_destroy(&mut barrier);
            btf__exit();
            return ret;
        }
        i += 1;
    }

    ret = run_stress_test(fd, threads.as_mut_ptr(), &mut barrier);

    close(fd);
    sigaction(SIGTRAP, &oldact, ptr::null_mut());
    pthread_barrier_destroy(&mut barrier);
    btf__exit();
    ret
}

extern "C" {
    fn errno() -> c_int;
}

/* DEFINE_SUITE("Sigtrap", sigtrap); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
