// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024-2025 Meta Platforms, Inc. and affiliates. */
/*
 * C dependencies:
 * - <test_progs.h>
 * - <network_helpers.h>
 * - <sys/sysinfo.h>
 * - <sys/syscall.h>
 * - <linux/perf_event.h>
 * - "res_spin_lock.skel.h"
 * - "res_spin_lock_fail.skel.h"
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = c_uint;
type pthread_t = usize;

const PERF_TYPE_HARDWARE: u32 = 0;
const PERF_COUNT_HW_CPU_CYCLES: u64 = 0;
const __NR_perf_event_open: c_long = 298;
const ENOENT: c_int = 2;
const EOPNOTSUPP: c_int = 95;
const EDEADLK: c_int = 35;

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub data_in: *const c_void,
    pub data_size_in: u32,
    pub repeat: u32,
    pub retval: i32,
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
}

#[repr(C)]
pub struct res_spin_lock_bss {
    pub err: c_int,
}

#[repr(C)]
pub struct res_spin_lock_progs {
    pub res_spin_lock_test: *mut bpf_program,
    pub res_spin_lock_test_held_lock_max: *mut bpf_program,
    pub res_spin_lock_test_AB: *mut bpf_program,
    pub res_spin_lock_test_BA: *mut bpf_program,
}

#[repr(C)]
pub struct res_spin_lock {
    pub progs: res_spin_lock_progs,
    pub bss: *mut res_spin_lock_bss,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

unsafe extern "C" {
    static pkt_v4: [u8; 0];
    static mut errno: c_int;

    fn RUN_TESTS(name: *const c_char);
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn test__skip();
    fn get_nprocs() -> c_int;
    fn libbpf_num_possible_cpus() -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn res_spin_lock__open_and_load() -> *mut res_spin_lock;
    fn res_spin_lock__destroy(skel: *mut res_spin_lock);
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_exit(retval: *mut c_void) -> !;
    fn syscall(num: c_long, ...) -> c_long;
    fn close(fd: c_int) -> c_int;
    fn load_module(name: *const c_char, verbose: bool) -> c_int;
    fn unload_module(name: *const c_char, verbose: bool);
    fn sleep(seconds: c_uint) -> c_uint;
}

unsafe fn READ_ONCE<T: Copy>(p: *const T) -> T {
    ptr::read_volatile(p)
}

unsafe fn WRITE_ONCE<T>(p: *mut T, v: T) {
    ptr::write_volatile(p, v);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_res_spin_lock_failure() {
    unsafe {
        RUN_TESTS(c"res_spin_lock_fail".as_ptr());
    }
}

static mut skip: c_int = 0;

unsafe extern "C" fn spin_lock_thread(arg: *mut c_void) -> *mut c_void {
    let mut err: c_int;
    let prog_fd: c_int = unsafe { *(arg as *mut u32) as c_int };
    let mut topts = bpf_test_run_opts {
        sz: size_of::<bpf_test_run_opts>(),
        data_in: unsafe { pkt_v4.as_ptr() as *const c_void },
        data_size_in: size_of_val_pkt_v4() as u32,
        repeat: 10000,
        retval: 0,
    };

    while unsafe { READ_ONCE(&raw const skip) } == 0 {
        err = unsafe { bpf_prog_test_run_opts(prog_fd, &mut topts) };
        if err != 0 || topts.retval != 0 {
            unsafe {
                ASSERT_OK(err, c"test_run".as_ptr());
                ASSERT_OK(topts.retval, c"test_run retval".as_ptr());
            }
            break;
        }
    }
    unsafe { pthread_exit(arg) };
}

fn size_of_val_pkt_v4() -> usize {
    size_of::<[u8; 0]>()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_res_spin_lock_success() {
    let mut topts = bpf_test_run_opts {
        sz: size_of::<bpf_test_run_opts>(),
        data_in: unsafe { pkt_v4.as_ptr() as *const c_void },
        data_size_in: size_of_val_pkt_v4() as u32,
        repeat: 1,
        retval: 0,
    };
    let skel: *mut res_spin_lock;
    let mut thread_id: [pthread_t; 16] = [0; 16];
    let mut prog_fd: c_int;
    let mut i: c_int;
    let mut err: c_int;
    let mut ret: *mut c_void = ptr::null_mut();

    if unsafe { get_nprocs() } < 2 {
        unsafe {
            test__skip();
        }
        return;
    }

    skel = unsafe { res_spin_lock__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(skel as *const c_void, c"res_spin_lock__open_and_load".as_ptr()) } {
        return;
    }
    /* AA deadlock */
    prog_fd = unsafe { bpf_program__fd((*skel).progs.res_spin_lock_test) };
    err = unsafe { bpf_prog_test_run_opts(prog_fd, &mut topts) };
    unsafe {
        ASSERT_OK(err, c"error".as_ptr());
        ASSERT_OK(topts.retval, c"retval".as_ptr());
    }

    prog_fd = unsafe { bpf_program__fd((*skel).progs.res_spin_lock_test_held_lock_max) };
    err = unsafe { bpf_prog_test_run_opts(prog_fd, &mut topts) };
    unsafe {
        ASSERT_OK(err, c"error".as_ptr());
        ASSERT_OK(topts.retval, c"retval".as_ptr());
    }

    /* Multi-threaded ABBA deadlock. */

    prog_fd = unsafe { bpf_program__fd((*skel).progs.res_spin_lock_test_AB) };
    i = 0;
    while i < 16 {
        let err: c_int;

        err = unsafe {
            pthread_create(
                &mut thread_id[i as usize],
                ptr::null(),
                spin_lock_thread,
                &mut prog_fd as *mut c_int as *mut c_void,
            )
        };
        if !unsafe { ASSERT_OK(err, c"pthread_create".as_ptr()) } {
            goto_end(skel);
            return;
        }
        i += 1;
    }

    topts.retval = 0;
    topts.repeat = 1000;
    let fd: c_int = unsafe { bpf_program__fd((*skel).progs.res_spin_lock_test_BA) };
    while topts.retval == 0
        && err == 0
        && unsafe { READ_ONCE(&raw const (*(*skel).bss).err) } == 0
    {
        err = unsafe { bpf_prog_test_run_opts(fd, &mut topts) };
    }

    unsafe {
        WRITE_ONCE(&raw mut skip, true as c_int);
    }

    i = 0;
    while i < 16 {
        if !unsafe { ASSERT_OK(pthread_join(thread_id[i as usize], &mut ret), c"pthread_join".as_ptr()) } {
            goto_end(skel);
            return;
        }
        if !unsafe {
            ASSERT_EQ(
                ret,
                &mut prog_fd as *mut c_int as *mut c_void,
                c"ret == prog_fd".as_ptr(),
            )
        } {
            goto_end(skel);
            return;
        }
        i += 1;
    }

    unsafe {
        ASSERT_EQ(READ_ONCE(&raw const (*(*skel).bss).err), -EDEADLK, c"timeout err".as_ptr());
        ASSERT_OK(err, c"err".as_ptr());
        ASSERT_EQ(topts.retval, -EDEADLK, c"timeout".as_ptr());
    }
    goto_end(skel);
}

unsafe fn goto_end(skel: *mut res_spin_lock) {
    unsafe {
        res_spin_lock__destroy(skel);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn serial_test_res_spin_lock_stress() {
    let mut attr = perf_event_attr {
        size: size_of::<perf_event_attr>() as u32,
        type_: PERF_TYPE_HARDWARE,
        config: PERF_COUNT_HW_CPU_CYCLES,
    };
    let pmu_fd: c_int;

    if unsafe { libbpf_num_possible_cpus() } < 3 {
        unsafe {
            test__skip();
        }
        return;
    }

    pmu_fd = unsafe {
        syscall(
            __NR_perf_event_open,
            &mut attr as *mut perf_event_attr,
            0,
            -1,
            -1,
            0,
        ) as c_int
    };
    if pmu_fd < 0 {
        if unsafe { errno == ENOENT || errno == EOPNOTSUPP } {
            unsafe {
                test__skip();
            }
            return;
        }
        unsafe {
            ASSERT_OK(-errno, c"perf_event_open pmu probe".as_ptr());
        }
        return;
    }
    unsafe {
        close(pmu_fd);
    }

    unsafe {
        ASSERT_OK(load_module(c"bpf_test_rqspinlock.ko".as_ptr(), false), c"load module AA".as_ptr());
        sleep(5);
        unload_module(c"bpf_test_rqspinlock".as_ptr(), false);
    }
    /*
     * Insert bpf_test_rqspinlock.ko manually with test_mode=[1|2] to test
     * other cases (ABBA, ABBCCA).
     */
}
