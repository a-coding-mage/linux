// SPDX-License-Identifier: GPL-2.0
//
// Translated from C source:
//   lib/perf/tests/test-threadmap.c
//
// External declarations correspond to symbols/macros/types provided by:
//   <stdarg.h>, <stdio.h>, <perf/threadmap.h>, <internal/tests.h>, "tests.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};

pub type pid_t = c_int;

#[repr(C)]
pub struct perf_thread_map {
    _private: [u8; 0],
}

#[repr(C)]
pub enum libperf_print_level {
    LIBPERF_PRINT_LEVEL__NONE = 0,
    LIBPERF_PRINT_LEVEL__ERROR = 1,
    LIBPERF_PRINT_LEVEL__WARNING = 2,
    LIBPERF_PRINT_LEVEL__INFO = 3,
    LIBPERF_PRINT_LEVEL__DEBUG = 4,
    LIBPERF_PRINT_LEVEL__DEBUG2 = 5,
    LIBPERF_PRINT_LEVEL__DEBUG3 = 6,
}

pub type va_list = *mut c_void;

unsafe extern "C" {
    static mut stderr: *mut c_void;
    static mut tests_failed: c_int;

    fn vfprintf(stream: *mut c_void, format: *const c_char, ap: va_list) -> c_int;

    fn libperf_init(
        print_fn: Option<
            unsafe extern "C" fn(
                level: libperf_print_level,
                fmt: *const c_char,
                ap: va_list,
            ) -> c_int,
        >,
    );

    fn perf_thread_map__new_array(nr: c_int, array: *mut pid_t) -> *mut perf_thread_map;
    fn perf_thread_map__new_dummy() -> *mut perf_thread_map;
    fn perf_thread_map__nr(threads: *mut perf_thread_map) -> c_int;
    fn perf_thread_map__pid(threads: *mut perf_thread_map, idx: c_int) -> pid_t;
    fn perf_thread_map__set_pid(threads: *mut perf_thread_map, idx: c_int, pid: pid_t);
    fn perf_thread_map__get(threads: *mut perf_thread_map);
    fn perf_thread_map__put(threads: *mut perf_thread_map);

    // C test helpers are macros in the original headers. They are represented
    // here as external hooks so the source-level test flow remains explicit.
    fn __T(msg: *const c_char, cond: bool);
    fn __T_START();
    fn __T_END();
}

unsafe extern "C" fn libperf_print(
    _level: libperf_print_level,
    fmt: *const c_char,
    ap: va_list,
) -> c_int {
    unsafe { vfprintf(stderr, fmt, ap) }
}

unsafe fn test_threadmap_array(nr: c_int, array: *mut pid_t) -> c_int {
    let threads: *mut perf_thread_map;
    let mut i: c_int;

    threads = unsafe { perf_thread_map__new_array(nr, array) };
    unsafe {
        __T(
            b"Failed to allocate new thread map\0".as_ptr() as *const c_char,
            !threads.is_null(),
        );
    }

    unsafe {
        __T(
            b"Unexpected number of threads\0".as_ptr() as *const c_char,
            perf_thread_map__nr(threads) == nr,
        );
    }

    i = 0;
    while i < nr {
        unsafe {
            __T(
                b"Unexpected initial value of thread\0".as_ptr() as *const c_char,
                perf_thread_map__pid(threads, i)
                    == if !array.is_null() {
                        *array.offset(i as isize)
                    } else {
                        -1
                    },
            );
        }
        i += 1;
    }

    i = 1;
    while i < nr {
        unsafe {
            perf_thread_map__set_pid(threads, i, i * 100);
        }
        i += 1;
    }

    unsafe {
        __T(
            b"Unexpected value of thread 0\0".as_ptr() as *const c_char,
            perf_thread_map__pid(threads, 0)
                == if !array.is_null() {
                    *array.offset(0)
                } else {
                    -1
                },
        );
    }

    i = 1;
    while i < nr {
        unsafe {
            __T(
                b"Unexpected thread value\0".as_ptr() as *const c_char,
                perf_thread_map__pid(threads, i) == i * 100,
            );
        }
        i += 1;
    }

    unsafe {
        perf_thread_map__put(threads);
    }

    0
}

const THREADS_NR: usize = 10;

#[no_mangle]
pub unsafe extern "C" fn test_threadmap(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let threads: *mut perf_thread_map;
    let mut thr_array: [pid_t; THREADS_NR] = [0; THREADS_NR];
    let mut i: c_int;

    unsafe {
        __T_START();
    }

    unsafe {
        libperf_init(Some(libperf_print));
    }

    threads = unsafe { perf_thread_map__new_dummy() };
    if threads.is_null() {
        return -1;
    }

    unsafe {
        perf_thread_map__get(threads);
        perf_thread_map__put(threads);
        perf_thread_map__put(threads);
    }

    unsafe {
        test_threadmap_array(THREADS_NR as c_int, core::ptr::null_mut());
    }

    i = 0;
    while i < THREADS_NR as c_int {
        thr_array[i as usize] = i + 100;
        i += 1;
    }

    unsafe {
        test_threadmap_array(THREADS_NR as c_int, thr_array.as_mut_ptr());
    }

    unsafe {
        __T_END();
        if tests_failed == 0 {
            0
        } else {
            -1
        }
    }
}
