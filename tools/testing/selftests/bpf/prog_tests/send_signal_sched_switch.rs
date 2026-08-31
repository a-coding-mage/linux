// SPDX-License-Identifier: GPL-2.0
// Translated from C implementation source:
// testing/selftests/bpf/prog_tests/send_signal_sched_switch.c
//
// Original dependencies:
// #include <test_progs.h>
// #include <stdio.h>
// #include <stdlib.h>
// #include <sys/mman.h>
// #include <pthread.h>
// #include <sys/types.h>
// #include <sys/stat.h>
// #include <fcntl.h>
// #include "test_send_signal_kern.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type u32 = c_uint;
type pthread_t = usize;

const SIGUSR1: c_int = 10;
const THREAD_COUNT: usize = 100;

#[repr(C)]
pub struct test_send_signal_kern_bss {
    pub pid: c_int,
    pub sig: c_int,
}

#[repr(C)]
pub struct test_send_signal_kern {
    pub bss: *mut test_send_signal_kern_bss,
}

unsafe extern "C" {
    fn signal(signum: c_int, handler: extern "C" fn(c_int)) -> extern "C" fn(c_int);
    fn test_send_signal_kern__open_and_load() -> *mut test_send_signal_kern;
    fn test_send_signal_kern__attach(skel: *mut test_send_signal_kern) -> c_int;
    fn test_send_signal_kern__destroy(skel: *mut test_send_signal_kern);
    fn getpid() -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;

    static mut errno: c_int;

    fn CHECK(condition: bool, tag: *const c_char, format: *const c_char, ...) -> bool;
}

extern "C" fn sigusr1_handler(_signum: c_int) {}

extern "C" fn worker(_p: *mut c_void) -> *mut c_void {
    let mut i: c_int;

    i = 0;
    while i < 1000 {
        unsafe {
            usleep(1);
        }
        i += 1;
    }

    ptr::null_mut()
}

/* NOTE: cause events loss */
pub unsafe fn serial_test_send_signal_sched_switch() {
    let mut skel: *mut test_send_signal_kern;
    let mut threads: [pthread_t; THREAD_COUNT] = [0; THREAD_COUNT];
    let duration: u32 = 0;
    let mut i: c_int;
    let mut err: c_int;

    let _ = duration;

    signal(SIGUSR1, sigusr1_handler);

    skel = test_send_signal_kern__open_and_load();
    if CHECK(
        skel.is_null(),
        c"skel_open_and_load".as_ptr(),
        c"skeleton open_and_load failed\n".as_ptr(),
    ) {
        return;
    }

    (*(*skel).bss).pid = getpid();
    (*(*skel).bss).sig = SIGUSR1;

    err = test_send_signal_kern__attach(skel);
    if CHECK(
        err != 0,
        c"skel_attach".as_ptr(),
        c"skeleton attach failed\n".as_ptr(),
    ) {
        test_send_signal_kern__destroy(skel);
        return;
    }

    i = 0;
    while i < THREAD_COUNT as c_int {
        err = pthread_create(
            threads.as_mut_ptr().add(i as usize),
            ptr::null(),
            worker,
            ptr::null_mut(),
        );
        if CHECK(
            err != 0,
            c"pthread_create".as_ptr(),
            c"Error creating thread, %s\n".as_ptr(),
            strerror(errno),
        ) {
            test_send_signal_kern__destroy(skel);
            return;
        }
        i += 1;
    }

    i = 0;
    while i < THREAD_COUNT as c_int {
        pthread_join(threads[i as usize], ptr::null_mut());
        i += 1;
    }

    test_send_signal_kern__destroy(skel);
}
