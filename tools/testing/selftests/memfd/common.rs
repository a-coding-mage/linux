// SPDX-License-Identifier: GPL-2.0
// C source defined _GNU_SOURCE and __EXPORTED_HEADERS__ before including libc
// and Linux memfd/syscall headers.

use core::ffi::{c_char, c_int, c_ulong, c_uint, c_void};

pub type FILE = c_void;

unsafe extern "C" {
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn getline(lineptr: *mut *mut c_char, n: *mut usize, stream: *mut FILE) -> isize;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn free(ptr: *mut c_void);
    fn fclose(stream: *mut FILE) -> c_int;
    fn syscall(number: c_long, ...) -> c_long;
}

pub type c_long = core::ffi::c_long;

pub static mut hugetlbfs_test: c_int = 0;

/*
 * Copied from mlock2-tests.c
 */
pub unsafe fn default_huge_page_size() -> c_ulong {
    let mut hps: c_ulong = 0;
    let mut line: *mut c_char = core::ptr::null_mut();
    let mut linelen: usize = 0;
    let f: *mut FILE = unsafe { fopen(c"/proc/meminfo".as_ptr(), c"r".as_ptr()) };

    if f.is_null() {
        return 0;
    }
    while unsafe { getline(&mut line, &mut linelen, f) } > 0 {
        if unsafe { sscanf(line, c"Hugepagesize:       %lu kB".as_ptr(), &mut hps) } == 1 {
            hps <<= 10;
            break;
        }
    }

    unsafe { free(line as *mut c_void) };
    unsafe { fclose(f) };
    hps
}

pub unsafe fn sys_memfd_create(name: *const c_char, mut flags: c_uint) -> c_int {
    if unsafe { hugetlbfs_test } != 0 {
        flags |= libc::MFD_HUGETLB;
    }

    unsafe { syscall(libc::SYS_memfd_create, name, flags) as c_int }
}
