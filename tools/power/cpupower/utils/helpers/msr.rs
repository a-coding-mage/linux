// SPDX-License-Identifier: GPL-2.0
// Translated from C. Original code is enabled only for __i386__ or __x86_64__.

#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};

const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const SEEK_CUR: c_int = 1;

/* Intel specific MSRs */
const MSR_IA32_PERF_STATUS: c_uint = 0x198;
const MSR_IA32_MISC_ENABLES: c_uint = 0x1a0;
const MSR_NEHALEM_TURBO_RATIO_LIMIT: c_uint = 0x1ad;

extern "C" {
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn lseek(fd: c_int, offset: i64, whence: c_int) -> i64;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;

    static cpupower_cpu_info: cpupower_cpu_info_t;
}

#[repr(C)]
pub struct cpupower_cpu_info_t {
    pub caps: c_uint,
}

extern "C" {
    static CPUPOWER_CAP_HAS_TURBO_RATIO: c_uint;
}

/*
 * read_msr
 *
 * Will return 0 on success and -1 on failure.
 * Possible errno values could be:
 * EFAULT -If the read/write did not fully complete
 * EIO    -If the CPU does not support MSRs
 * ENXIO  -If the CPU does not exist
 */

#[no_mangle]
pub unsafe extern "C" fn read_msr(
    cpu: c_int,
    idx: c_uint,
    val: *mut c_ulonglong,
) -> c_int {
    let fd: c_int;
    let mut msr_file_name: [c_char; 64] = [0; 64];

    sprintf(
        msr_file_name.as_mut_ptr(),
        b"/dev/cpu/%d/msr\0".as_ptr() as *const c_char,
        cpu,
    );
    fd = open(msr_file_name.as_ptr(), O_RDONLY);
    if fd < 0 {
        return -1;
    }
    if lseek(fd, idx as i64, SEEK_CUR) == -1 {
        close(fd);
        return -1;
    }
    if read(
        fd,
        val as *mut c_void,
        core::mem::size_of::<c_ulonglong>(),
    ) != core::mem::size_of::<c_ulonglong>() as isize
    {
        close(fd);
        return -1;
    }
    close(fd);
    0
}

/*
 * write_msr
 *
 * Will return 0 on success and -1 on failure.
 * Possible errno values could be:
 * EFAULT -If the read/write did not fully complete
 * EIO    -If the CPU does not support MSRs
 * ENXIO  -If the CPU does not exist
 */
#[no_mangle]
pub unsafe extern "C" fn write_msr(
    cpu: c_int,
    idx: c_uint,
    val: c_ulonglong,
) -> c_int {
    let fd: c_int;
    let mut msr_file_name: [c_char; 64] = [0; 64];

    sprintf(
        msr_file_name.as_mut_ptr(),
        b"/dev/cpu/%d/msr\0".as_ptr() as *const c_char,
        cpu,
    );
    fd = open(msr_file_name.as_ptr(), O_WRONLY);
    if fd < 0 {
        return -1;
    }
    if lseek(fd, idx as i64, SEEK_CUR) == -1 {
        close(fd);
        return -1;
    }
    if write(
        fd,
        &val as *const c_ulonglong as *const c_void,
        core::mem::size_of::<c_ulonglong>(),
    ) != core::mem::size_of::<c_ulonglong>() as isize
    {
        close(fd);
        return -1;
    }
    close(fd);
    0
}

#[no_mangle]
pub unsafe extern "C" fn msr_intel_get_turbo_ratio(cpu: c_uint) -> c_ulonglong {
    let mut val: c_ulonglong = 0;
    let ret: c_int;

    if (cpupower_cpu_info.caps & CPUPOWER_CAP_HAS_TURBO_RATIO) == 0 {
        return -1i32 as c_ulonglong;
    }

    ret = read_msr(cpu as c_int, MSR_NEHALEM_TURBO_RATIO_LIMIT, &mut val);
    if ret != 0 {
        return ret as c_ulonglong;
    }
    val
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
