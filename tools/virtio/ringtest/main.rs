// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2016 Red Hat, Inc.
 * Author: Michael S. Tsirkin <mst@redhat.com>
 *
 * Command line processing and common functions for ring benchmarking.
 */

use std::ffi::c_void;
use std::os::raw::{c_char, c_int, c_long, c_ulong};
use std::ptr;

const INT_MAX: c_int = 2147483647;
const CPU_SETSIZE: c_long = 1024;
const NO_ARGUMENT: c_int = 0;
const REQUIRED_ARGUMENT: c_int = 1;

#[repr(C)]
pub struct cpu_set_t {
    __bits: [c_ulong; 16],
}

#[repr(C)]
pub struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

type pthread_t = c_ulong;

unsafe extern "C" {
    static mut optarg: *mut c_char;
    static mut optind: c_int;

    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn eventfd(initval: c_int, flags: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn pthread_self() -> pthread_t;
    fn pthread_setaffinity_np(thread: pthread_t, cpusetsize: usize, cpuset: *const cpu_set_t) -> c_int;
    fn pthread_exit(value_ptr: *mut c_void) -> !;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    static mut stderr: *mut c_void;

    fn vmexit();
    fn vmentry();
    fn used_empty() -> bool;
    fn busy_wait();
    fn disable_call();
    fn add_inbuf(id: c_int, buf: *const c_char, data: *const c_char) -> c_int;
    fn kick_available();
    fn get_buf(len: *mut u32, buf: *mut *mut c_void) -> bool;
    fn enable_call() -> bool;
    fn avail_empty() -> bool;
    fn enable_kick() -> bool;
    fn disable_kick();
    fn use_buf(len: *mut u32, buf: *mut *mut c_void) -> bool;
    fn call_used();
    fn smp_acquire();
    fn smp_release();
    fn smp_mb();
    fn alloc_ring();
}

#[no_mangle]
pub static mut runcycles: c_int = 10000000;
#[no_mangle]
pub static mut max_outstanding: c_int = INT_MAX;
#[no_mangle]
pub static mut batch: c_int = 1;
#[no_mangle]
pub static mut param: c_int = 0;

#[no_mangle]
pub static mut do_sleep: bool = false;
#[no_mangle]
pub static mut do_relax: bool = false;
#[no_mangle]
pub static mut do_exit: bool = true;

#[no_mangle]
pub static mut ring_size: u32 = 256;

static mut kickfd: c_int = -1;
static mut callfd: c_int = -1;

unsafe fn CPU_ZERO(set: *mut cpu_set_t) {
    for word in (*set).__bits.iter_mut() {
        *word = 0;
    }
}

unsafe fn CPU_SET(cpu: c_long, set: *mut cpu_set_t) {
    let bits_per_word = (8 * std::mem::size_of::<c_ulong>()) as c_long;
    let idx = (cpu / bits_per_word) as usize;
    let bit = (cpu % bits_per_word) as u32;
    (*set).__bits[idx] |= (1 as c_ulong) << bit;
}

#[no_mangle]
pub unsafe extern "C" fn notify(fd: c_int) {
    let mut v: u64 = 1;
    let r: c_int;

    vmexit();
    r = write(
        fd,
        &v as *const u64 as *const c_void,
        std::mem::size_of_val(&v),
    ) as c_int;
    assert!(r == std::mem::size_of_val(&v) as c_int);
    vmentry();
}

#[no_mangle]
pub unsafe extern "C" fn wait_for_notify(fd: c_int) {
    let mut v: u64 = 1;
    let r: c_int;

    vmexit();
    r = read(
        fd,
        &mut v as *mut u64 as *mut c_void,
        std::mem::size_of_val(&v),
    ) as c_int;
    assert!(r == std::mem::size_of_val(&v) as c_int);
    vmentry();
}

#[no_mangle]
pub unsafe extern "C" fn kick() {
    notify(kickfd);
}

#[no_mangle]
pub unsafe extern "C" fn wait_for_kick() {
    wait_for_notify(kickfd);
}

#[no_mangle]
pub unsafe extern "C" fn call() {
    notify(callfd);
}

#[no_mangle]
pub unsafe extern "C" fn wait_for_call() {
    wait_for_notify(callfd);
}

#[no_mangle]
pub unsafe extern "C" fn set_affinity(arg: *const c_char) {
    let mut cpuset = cpu_set_t { __bits: [0; 16] };
    let ret: c_int;
    let self_: pthread_t;
    let cpu: c_long;
    let mut endptr: *mut c_char = ptr::null_mut();

    if arg.is_null() {
        return;
    }

    cpu = strtol(arg, &mut endptr, 0);
    assert!(*endptr == 0);

    assert!(cpu >= 0 && cpu < CPU_SETSIZE);

    self_ = pthread_self();
    CPU_ZERO(&mut cpuset);
    CPU_SET(cpu, &mut cpuset);

    ret = pthread_setaffinity_np(self_, std::mem::size_of::<cpu_set_t>(), &cpuset);
    assert!(ret == 0);
}

#[no_mangle]
pub unsafe extern "C" fn poll_used() {
    while used_empty() {
        busy_wait();
    }
}

unsafe fn run_guest() {
    let mut completed_before: c_int;
    let mut completed: c_int = 0;
    let mut started: c_int = 0;
    let bufs: c_int = runcycles;
    let mut spurious: c_int = 0;
    let mut r: c_int;
    let mut len: u32 = 0;
    let mut buf: *mut c_void = ptr::null_mut();
    let mut tokick: c_int = batch;

    loop {
        if do_sleep {
            disable_call();
        }
        completed_before = completed;
        loop {
            if started < bufs && started - completed < max_outstanding {
                r = add_inbuf(
                    0,
                    b"Buffer\n\0".as_ptr() as *const c_char,
                    b"Hello, world!\0".as_ptr() as *const c_char,
                );
                if r == 0 {
                    started += 1;
                    tokick -= 1;
                    if tokick == 0 {
                        tokick = batch;
                        if do_sleep {
                            kick_available();
                        }
                    }
                }
            } else {
                r = -1;
            }

            /* Flush out completed bufs if any */
            if get_buf(&mut len, &mut buf) {
                completed += 1;
                if completed == bufs {
                    return;
                }
                r = 0;
            }

            if r != 0 {
                break;
            }
        }
        if completed == completed_before {
            spurious += 1;
        }
        assert!(completed <= bufs);
        assert!(started <= bufs);
        if do_sleep {
            if used_empty() && enable_call() {
                wait_for_call();
            }
        } else {
            poll_used();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn poll_avail() {
    while avail_empty() {
        busy_wait();
    }
}

unsafe fn run_host() {
    let mut completed_before: c_int;
    let mut completed: c_int = 0;
    let mut spurious: c_int = 0;
    let bufs: c_int = runcycles;
    let mut len: u32 = 0;
    let mut buf: *mut c_void = ptr::null_mut();

    loop {
        if do_sleep {
            if avail_empty() && enable_kick() {
                wait_for_kick();
            }
        } else {
            poll_avail();
        }
        if do_sleep {
            disable_kick();
        }
        completed_before = completed;
        while use_buf(&mut len, &mut buf) {
            if do_sleep {
                call_used();
            }
            completed += 1;
            if completed == bufs {
                return;
            }
        }
        if completed == completed_before {
            spurious += 1;
        }
        assert!(completed <= bufs);
        if completed == bufs {
            break;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn start_guest(arg: *mut c_void) -> *mut c_void {
    set_affinity(arg as *const c_char);
    run_guest();
    pthread_exit(ptr::null_mut());
}

#[no_mangle]
pub unsafe extern "C" fn start_host(arg: *mut c_void) -> *mut c_void {
    set_affinity(arg as *const c_char);
    run_host();
    pthread_exit(ptr::null_mut());
}

static optstring: &[u8] = b"\0";
static longopts: [option; 12] = [
    option {
        name: b"help\0".as_ptr() as *const c_char,
        has_arg: NO_ARGUMENT,
        flag: ptr::null_mut(),
        val: b'h' as c_int,
    },
    option {
        name: b"host-affinity\0".as_ptr() as *const c_char,
        has_arg: REQUIRED_ARGUMENT,
        flag: ptr::null_mut(),
        val: b'H' as c_int,
    },
    option {
        name: b"guest-affinity\0".as_ptr() as *const c_char,
        has_arg: REQUIRED_ARGUMENT,
        flag: ptr::null_mut(),
        val: b'G' as c_int,
    },
    option {
        name: b"ring-size\0".as_ptr() as *const c_char,
        has_arg: REQUIRED_ARGUMENT,
        flag: ptr::null_mut(),
        val: b'R' as c_int,
    },
    option {
        name: b"run-cycles\0".as_ptr() as *const c_char,
        has_arg: REQUIRED_ARGUMENT,
        flag: ptr::null_mut(),
        val: b'C' as c_int,
    },
    option {
        name: b"outstanding\0".as_ptr() as *const c_char,
        has_arg: REQUIRED_ARGUMENT,
        flag: ptr::null_mut(),
        val: b'o' as c_int,
    },
    option {
        name: b"batch\0".as_ptr() as *const c_char,
        has_arg: REQUIRED_ARGUMENT,
        flag: ptr::null_mut(),
        val: b'b' as c_int,
    },
    option {
        name: b"param\0".as_ptr() as *const c_char,
        has_arg: REQUIRED_ARGUMENT,
        flag: ptr::null_mut(),
        val: b'p' as c_int,
    },
    option {
        name: b"sleep\0".as_ptr() as *const c_char,
        has_arg: NO_ARGUMENT,
        flag: ptr::null_mut(),
        val: b's' as c_int,
    },
    option {
        name: b"relax\0".as_ptr() as *const c_char,
        has_arg: NO_ARGUMENT,
        flag: ptr::null_mut(),
        val: b'x' as c_int,
    },
    option {
        name: b"exit\0".as_ptr() as *const c_char,
        has_arg: NO_ARGUMENT,
        flag: ptr::null_mut(),
        val: b'e' as c_int,
    },
    option {
        name: ptr::null(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: 0,
    },
];

unsafe fn help() {
    fprintf(
        stderr,
        b"Usage: <test> [--help] [--host-affinity H] [--guest-affinity G] [--ring-size R (default: %u)] [--run-cycles C (default: %d)] [--batch b] [--outstanding o] [--param p] [--sleep] [--relax] [--exit]\n\0"
            .as_ptr() as *const c_char,
        ring_size,
        runcycles,
    );
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut ret: c_int;
    let mut host: pthread_t = 0;
    let mut guest: pthread_t = 0;
    let mut tret: *mut c_void = ptr::null_mut();
    let mut host_arg: *mut c_char = ptr::null_mut();
    let mut guest_arg: *mut c_char = ptr::null_mut();
    let mut endptr: *mut c_char = ptr::null_mut();
    let mut c: c_long;

    kickfd = eventfd(0, 0);
    assert!(kickfd >= 0);
    callfd = eventfd(0, 0);
    assert!(callfd >= 0);

    loop {
        let o = getopt_long(argc, argv, optstring.as_ptr() as *const c_char, longopts.as_ptr(), ptr::null_mut());
        match o {
            -1 => break,
            x if x == b'?' as c_int => {
                help();
                exit(2);
            }
            x if x == b'H' as c_int => {
                host_arg = optarg;
            }
            x if x == b'G' as c_int => {
                guest_arg = optarg;
            }
            x if x == b'R' as c_int => {
                ring_size = strtol(optarg, &mut endptr, 0) as u32;
                assert!(ring_size != 0 && (ring_size & (ring_size - 1)) == 0);
                assert!(*endptr == 0);
            }
            x if x == b'C' as c_int => {
                c = strtol(optarg, &mut endptr, 0);
                assert!(*endptr == 0);
                assert!(c > 0 && c < INT_MAX as c_long);
                runcycles = c as c_int;
            }
            x if x == b'o' as c_int => {
                c = strtol(optarg, &mut endptr, 0);
                assert!(*endptr == 0);
                assert!(c > 0 && c < INT_MAX as c_long);
                max_outstanding = c as c_int;
            }
            x if x == b'p' as c_int => {
                c = strtol(optarg, &mut endptr, 0);
                assert!(*endptr == 0);
                assert!(c > 0 && c < INT_MAX as c_long);
                param = c as c_int;
            }
            x if x == b'b' as c_int => {
                c = strtol(optarg, &mut endptr, 0);
                assert!(*endptr == 0);
                assert!(c > 0 && c < INT_MAX as c_long);
                batch = c as c_int;
            }
            x if x == b's' as c_int => {
                do_sleep = true;
            }
            x if x == b'x' as c_int => {
                do_relax = true;
            }
            x if x == b'e' as c_int => {
                do_exit = true;
            }
            _ => {
                help();
                exit(4);
            }
        }
    }

    /* does nothing here, used to make sure all smp APIs compile */
    smp_acquire();
    smp_release();
    smp_mb();

    if batch > max_outstanding {
        batch = max_outstanding;
    }

    if optind < argc {
        help();
        exit(4);
    }
    alloc_ring();

    ret = pthread_create(
        &mut host,
        ptr::null(),
        start_host,
        host_arg as *mut c_void,
    );
    assert!(ret == 0);
    ret = pthread_create(
        &mut guest,
        ptr::null(),
        start_guest,
        guest_arg as *mut c_void,
    );
    assert!(ret == 0);

    ret = pthread_join(guest, &mut tret);
    assert!(ret == 0);
    ret = pthread_join(host, &mut tret);
    assert!(ret == 0);
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
