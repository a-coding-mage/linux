// SPDX-License-Identifier: LGPL-2.1
// C dependencies translated as external Rust dependency surface:
// assert.h, pthread.h, sched.h, signal.h, stdbool.h, stdio.h, string.h,
// syscall.h, unistd.h, linux/prctl.h, sys/prctl.h, sys/time.h, "rseq.h",
// and "../kselftest_harness.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type c_int = i32;
type c_long = i64;
type c_ulong = u64;
type int64_t = i64;
type pthread_t = c_ulong;
type size_t = usize;

const __NR_rseq_slice_yield: c_long = 471;

const BITS_PER_INT: c_int = 32;
const BITS_PER_BYTE: c_int = 8;

const PR_RSEQ_SLICE_EXTENSION: c_int = 79;
const PR_RSEQ_SLICE_EXTENSION_GET: c_int = 1;
const PR_RSEQ_SLICE_EXTENSION_SET: c_int = 2;
const PR_RSEQ_SLICE_EXT_ENABLE: c_int = 0x01;

const RSEQ_SLICE_EXT_REQUEST_BIT: c_int = 0;
const RSEQ_SLICE_EXT_GRANTED_BIT: c_int = 1;

// C asm_inline compatibility macro has no file-local Rust equivalent.

const NSEC_PER_SEC: c_long = 1000000000;
const NSEC_PER_USEC: c_long = 1000;

#[repr(C)]
struct timespec {
    tv_sec: int64_t,
    tv_nsec: int64_t,
}

#[repr(C)]
struct cpu_set_t {
    // Opaque external libc layout placeholder for calls translated from C.
    __private: [c_ulong; 16],
}

#[repr(C)]
struct rseq_slice_ctrl {
    request: c_int,
    granted: c_int,
}

#[repr(C)]
struct rseq_abi {
    slice_ctrl: rseq_slice_ctrl,
}

#[repr(C)]
struct noise_params {
    noise_nsecs: int64_t,
    sleep_nsecs: int64_t,
    run: int64_t,
}

#[repr(C)]
struct slice_ext {
    noise_thread: pthread_t,
    noise_params: noise_params,
}

#[repr(C)]
struct slice_ext_variant {
    total_nsecs: int64_t,
    slice_nsecs: int64_t,
    noise_nsecs: int64_t,
    sleep_nsecs: int64_t,
    no_yield: bool,
}

static slice_ext_variant_n2_2_50: slice_ext_variant = slice_ext_variant {
    total_nsecs: 5i64 * NSEC_PER_SEC,
    slice_nsecs: 2i64 * NSEC_PER_USEC,
    noise_nsecs: 2i64 * NSEC_PER_USEC,
    sleep_nsecs: 50i64 * NSEC_PER_USEC,
    no_yield: false,
};

static slice_ext_variant_n50_2_50: slice_ext_variant = slice_ext_variant {
    total_nsecs: 5i64 * NSEC_PER_SEC,
    slice_nsecs: 50i64 * NSEC_PER_USEC,
    noise_nsecs: 2i64 * NSEC_PER_USEC,
    sleep_nsecs: 50i64 * NSEC_PER_USEC,
    no_yield: false,
};

static slice_ext_variant_n2_2_50_no_yield: slice_ext_variant = slice_ext_variant {
    total_nsecs: 5i64 * NSEC_PER_SEC,
    slice_nsecs: 2i64 * NSEC_PER_USEC,
    noise_nsecs: 2i64 * NSEC_PER_USEC,
    sleep_nsecs: 50i64 * NSEC_PER_USEC,
    no_yield: true,
};

const CLOCK_MONOTONIC: c_int = 1;
const CPU_SETSIZE: c_int = 1024;
const __NR_getpid: c_long = 39;

extern "C" {
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn clock_nanosleep(
        clockid: c_int,
        flags: c_int,
        request: *const timespec,
        remain: *mut timespec,
    ) -> c_int;
    fn prctl(option: c_int, arg2: c_ulong, arg3: c_ulong, arg4: c_ulong, arg5: c_ulong) -> c_int;
    fn sched_getaffinity(pid: c_int, cpusetsize: size_t, mask: *mut cpu_set_t) -> c_int;
    fn sched_setaffinity(pid: c_int, cpusetsize: size_t, mask: *const cpu_set_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const (),
        start_routine: unsafe extern "C" fn(*mut ()) -> *mut (),
        arg: *mut (),
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut ()) -> c_int;
    fn syscall(number: c_long, ...) -> c_long;
    fn printf(format: *const u8, ...) -> c_int;

    fn __rseq_register_current_thread(register: bool, unregister: bool) -> c_int;
    fn rseq_get_abi() -> *mut rseq_abi;
}

extern "Rust" {
    fn CPU_ISSET(cpu: c_int, set: *const cpu_set_t) -> bool;
    fn CPU_ZERO(set: *mut cpu_set_t);
    fn CPU_SET(cpu: c_int, set: *mut cpu_set_t);
}

macro_rules! RSEQ_READ_ONCE {
    ($x:expr) => {
        unsafe { core::ptr::read_volatile(core::ptr::addr_of!($x)) }
    };
}

macro_rules! RSEQ_WRITE_ONCE {
    ($x:expr, $v:expr) => {
        unsafe {
            core::ptr::write_volatile(core::ptr::addr_of_mut!($x), $v);
        }
    };
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! ASSERT_NE {
    ($left:expr, $right:expr) => {
        assert_ne!($left, $right)
    };
}

macro_rules! SKIP {
    (return, $msg:expr) => {{
        printf($msg.as_ptr());
        return;
    }};
}

#[inline]
unsafe fn elapsed(start: *mut timespec, now: *mut timespec, span: int64_t) -> bool {
    let mut delta: int64_t = (*now).tv_sec - (*start).tv_sec;

    delta *= NSEC_PER_SEC;
    delta += (*now).tv_nsec - (*start).tv_nsec;
    delta >= span
}

unsafe extern "C" fn noise_thread(arg: *mut ()) -> *mut () {
    let p: *mut noise_params = arg as *mut noise_params;

    while RSEQ_READ_ONCE!((*p).run) != 0 {
        let mut ts_start: timespec = core::mem::zeroed();
        let mut ts_now: timespec = core::mem::zeroed();

        clock_gettime(CLOCK_MONOTONIC, &mut ts_start);
        loop {
            clock_gettime(CLOCK_MONOTONIC, &mut ts_now);
            if elapsed(&mut ts_start, &mut ts_now, (*p).noise_nsecs) {
                break;
            }
        }

        ts_start.tv_sec = 0;
        ts_start.tv_nsec = (*p).sleep_nsecs;
        clock_nanosleep(CLOCK_MONOTONIC, 0, &ts_start, core::ptr::null_mut());
    }
    core::ptr::null_mut()
}

unsafe fn slice_ext_setup(self_: *mut slice_ext, variant: *const slice_ext_variant) {
    let mut affinity: cpu_set_t = core::mem::zeroed();

    if __rseq_register_current_thread(true, false) != 0 {
        SKIP!(return, "RSEQ not supported\n\0");
    }

    if prctl(
        PR_RSEQ_SLICE_EXTENSION,
        PR_RSEQ_SLICE_EXTENSION_SET as c_ulong,
        PR_RSEQ_SLICE_EXT_ENABLE as c_ulong,
        0,
        0,
    ) != 0
    {
        SKIP!(return, "Time slice extension not supported\n\0");
    }

    ASSERT_EQ!(
        sched_getaffinity(0, core::mem::size_of_val(&affinity), &mut affinity),
        0
    );

    /* Pin it on a single CPU. Avoid CPU 0 */
    let mut i: c_int = 1;
    while i < CPU_SETSIZE {
        if !CPU_ISSET(i, &affinity) {
            i += 1;
            continue;
        }

        CPU_ZERO(&mut affinity);
        CPU_SET(i, &mut affinity);
        ASSERT_EQ!(
            sched_setaffinity(0, core::mem::size_of_val(&affinity), &affinity),
            0
        );
        break;
    }

    (*self_).noise_params.noise_nsecs = (*variant).noise_nsecs;
    (*self_).noise_params.sleep_nsecs = (*variant).sleep_nsecs;
    (*self_).noise_params.run = 1;

    ASSERT_EQ!(
        pthread_create(
            &mut (*self_).noise_thread,
            core::ptr::null(),
            noise_thread,
            &mut (*self_).noise_params as *mut noise_params as *mut (),
        ),
        0
    );
}

unsafe fn slice_ext_teardown(self_: *mut slice_ext) {
    (*self_).noise_params.run = 0;
    pthread_join((*self_).noise_thread, core::ptr::null_mut());
}

unsafe fn slice_ext_slice_test(_self: *mut slice_ext, variant: *const slice_ext_variant) {
    let mut success: c_ulong = 0;
    let mut yielded: c_ulong = 0;
    let mut scheduled: c_ulong = 0;
    let mut raced: c_ulong = 0;
    let mut total: c_ulong = 0;
    let mut aborted: c_ulong = 0;
    let rs: *mut rseq_abi = rseq_get_abi();
    let mut ts_start: timespec = core::mem::zeroed();
    let mut ts_now: timespec = core::mem::zeroed();

    ASSERT_NE!(rs, core::ptr::null_mut());

    clock_gettime(CLOCK_MONOTONIC, &mut ts_start);
    loop {
        let mut ts_cs: timespec = core::mem::zeroed();
        let mut req: bool = false;

        clock_gettime(CLOCK_MONOTONIC, &mut ts_cs);

        total += 1;
        RSEQ_WRITE_ONCE!((*rs).slice_ctrl.request, 1);
        loop {
            clock_gettime(CLOCK_MONOTONIC, &mut ts_now);
            if elapsed(&mut ts_cs, &mut ts_now, (*variant).slice_nsecs) {
                break;
            }
        }

        /*
         * request can be cleared unconditionally, but for making
         * the stats work this is actually checking it first
         */
        if RSEQ_READ_ONCE!((*rs).slice_ctrl.request) != 0 {
            RSEQ_WRITE_ONCE!((*rs).slice_ctrl.request, 0);
            /* Race between check and clear! */
            req = true;
            success += 1;
        }

        if RSEQ_READ_ONCE!((*rs).slice_ctrl.granted) != 0 {
            /* The above raced against a late grant */
            if req {
                success -= 1;
            }
            if (*variant).no_yield {
                syscall(__NR_getpid);
                aborted += 1;
            } else {
                yielded += 1;
                if syscall(__NR_rseq_slice_yield) == 0 {
                    raced += 1;
                }
            }
        } else if !req {
            scheduled += 1;
        }

        clock_gettime(CLOCK_MONOTONIC, &mut ts_now);
        if elapsed(&mut ts_start, &mut ts_now, (*variant).total_nsecs) {
            break;
        }
    }

    printf(b"# Total     %12ld\n\0".as_ptr(), total);
    printf(b"# Success   %12ld\n\0".as_ptr(), success);
    printf(b"# Yielded   %12ld\n\0".as_ptr(), yielded);
    printf(b"# Aborted   %12ld\n\0".as_ptr(), aborted);
    printf(b"# Scheduled %12ld\n\0".as_ptr(), scheduled);
    printf(b"# Raced     %12ld\n\0".as_ptr(), raced);
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
