// SPDX-License-Identifier: GPL-2.0-only

// C dependencies:
// #include <stdio.h>
// #include <errno.h>
// #include <stdlib.h>
// #include <string.h>
// #include <sys/types.h>
// #include <sys/stat.h>
// #include <fcntl.h>
// #include <unistd.h>
//
// #include "cpupower_intern.h"
// #include "acpi_cppc.h"

use core::ffi::{c_char, c_int, c_ulong, c_ulonglong};

type size_t = usize;

unsafe extern "C" {
    static PATH_TO_CPU: *const c_char;
    static ERANGE: c_int;

    fn cpupower_read_sysfs(path: *const c_char, buf: *mut c_char, buflen: size_t) -> c_uint;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulonglong;
    fn __errno_location() -> *mut c_int;
}

type c_uint = u32;

// Header-provided constants from cpupower_intern.h.
const SYSFS_PATH_MAX: usize = 255;
const MAX_LINE_LEN: usize = 4096;

const HIGHEST_PERF: usize = 0;
const LOWEST_PERF: usize = 1;
const NOMINAL_PERF: usize = 2;
const LOWEST_NONLINEAR_PERF: usize = 3;
const LOWEST_FREQ: usize = 4;
const NOMINAL_FREQ: usize = 5;
const REFERENCE_PERF: usize = 6;
const WRAPAROUND_TIME: usize = 7;
const MAX_CPPC_VALUE_FILES: usize = 8;

#[repr(C)]
pub enum acpi_cppc_value {
    HIGHEST_PERF = HIGHEST_PERF as isize,
    LOWEST_PERF = LOWEST_PERF as isize,
    NOMINAL_PERF = NOMINAL_PERF as isize,
    LOWEST_NONLINEAR_PERF = LOWEST_NONLINEAR_PERF as isize,
    LOWEST_FREQ = LOWEST_FREQ as isize,
    NOMINAL_FREQ = NOMINAL_FREQ as isize,
    REFERENCE_PERF = REFERENCE_PERF as isize,
    WRAPAROUND_TIME = WRAPAROUND_TIME as isize,
    MAX_CPPC_VALUE_FILES = MAX_CPPC_VALUE_FILES as isize,
}

/* ACPI CPPC sysfs access ***********************************************/

unsafe fn acpi_cppc_read_file(
    cpu: c_uint,
    fname: *const c_char,
    buf: *mut c_char,
    buflen: size_t,
) -> c_int {
    let mut path = [0 as c_char; SYSFS_PATH_MAX];

    snprintf(
        path.as_mut_ptr(),
        path.len(),
        b"%scpu%u/acpi_cppc/%s\0".as_ptr() as *const c_char,
        PATH_TO_CPU,
        cpu,
        fname,
    );
    cpupower_read_sysfs(path.as_ptr(), buf, buflen) as c_int
}

static acpi_cppc_value_files: [&[u8]; MAX_CPPC_VALUE_FILES] = [
    b"highest_perf\0",
    b"lowest_perf\0",
    b"nominal_perf\0",
    b"lowest_nonlinear_perf\0",
    b"lowest_freq\0",
    b"nominal_freq\0",
    b"reference_perf\0",
    b"wraparound_time\0",
];

#[no_mangle]
pub unsafe extern "C" fn acpi_cppc_get_data(cpu: c_uint, which: acpi_cppc_value) -> c_ulong {
    let mut value: c_ulonglong;
    let len: c_uint;
    let mut linebuf = [0 as c_char; MAX_LINE_LEN];
    let mut endp: *mut c_char = core::ptr::null_mut();
    let which = which as usize;

    if which >= MAX_CPPC_VALUE_FILES {
        return 0;
    }

    len = acpi_cppc_read_file(
        cpu,
        acpi_cppc_value_files[which].as_ptr() as *const c_char,
        linebuf.as_mut_ptr(),
        linebuf.len(),
    ) as c_uint;
    if len == 0 {
        return 0;
    }

    value = strtoull(linebuf.as_ptr(), &mut endp, 0);

    if endp == linebuf.as_mut_ptr() || *__errno_location() == ERANGE {
        return 0;
    }

    value as c_ulong
}
