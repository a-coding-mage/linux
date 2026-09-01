// SPDX-License-Identifier: GPL-2.0
// C dependencies: <stdarg.h>, <stdio.h>, <perf/cpumap.h>,
// <internal/tests.h>, "tests.h"

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub enum libperf_print_level {
    LIBPERF_ERR,
    LIBPERF_WARN,
    LIBPERF_INFO,
    LIBPERF_DEBUG,
    LIBPERF_DEBUG2,
}

pub type va_list = *mut core::ffi::c_void;

unsafe extern "C" {
    static mut stderr: *mut core::ffi::c_void;
    static mut tests_failed: c_int;

    fn vfprintf(
        stream: *mut core::ffi::c_void,
        format: *const c_char,
        ap: va_list,
    ) -> c_int;

    fn libperf_init(
        print_fn: Option<
            unsafe extern "C" fn(
                level: libperf_print_level,
                fmt: *const c_char,
                ap: va_list,
            ) -> c_int,
        >,
    );

    fn perf_cpu_map__new_any_cpu() -> *mut perf_cpu_map;
    fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
    fn perf_cpu_map__get(cpus: *mut perf_cpu_map);
    fn perf_cpu_map__put(cpus: *mut perf_cpu_map);

    fn perf_cpu_map__nr(cpus: *const perf_cpu_map) -> c_int;
    fn perf_cpu_map__cpu(cpus: *const perf_cpu_map, idx: c_int) -> perf_cpu;
}

unsafe extern "C" fn libperf_print(
    _level: libperf_print_level,
    fmt: *const c_char,
    ap: va_list,
) -> c_int {
    unsafe { vfprintf(stderr, fmt, ap) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_cpumap(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut cpus: *mut perf_cpu_map;
    let mut cpu: perf_cpu;
    let mut idx: c_int;

    // __T_START;

    unsafe { libperf_init(Some(libperf_print)) };

    cpus = unsafe { perf_cpu_map__new_any_cpu() };
    if cpus.is_null() {
        return -1;
    }

    unsafe { perf_cpu_map__get(cpus) };
    unsafe { perf_cpu_map__put(cpus) };
    unsafe { perf_cpu_map__put(cpus) };

    cpus = unsafe { perf_cpu_map__new_online_cpus() };
    if cpus.is_null() {
        return -1;
    }

    idx = 0;
    while idx < unsafe { perf_cpu_map__nr(cpus) } {
        cpu = unsafe { perf_cpu_map__cpu(cpus, idx) };
        // __T("wrong cpu number", cpu.cpu != -1);
        if !(cpu.cpu != -1) {
            unsafe {
                tests_failed += 1;
            }
        }
        idx += 1;
    }

    unsafe { perf_cpu_map__put(cpus) };

    // __T_END;
    if unsafe { tests_failed } == 0 { 0 } else { -1 }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
