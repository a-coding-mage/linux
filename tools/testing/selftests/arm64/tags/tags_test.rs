// SPDX-License-Identifier: GPL-2.0

// Translated from C source using stdio/stdlib/unistd/stdint, sys/prctl,
// sys/utsname, and kselftest dependencies.

use core::ffi::{c_char, c_int, c_ulong, c_void};

const PR_SET_TAGGED_ADDR_CTRL: c_int = 55;
const PR_TAGGED_ADDR_ENABLE: c_ulong = 1 << 0;

const fn SHIFT_TAG(tag: u64) -> u64 {
    (tag as u64) << 56
}

const fn SET_TAG(ptr: *mut c_void, tag: u64) -> u64 {
    ((ptr as u64) & !SHIFT_TAG(0xff)) | SHIFT_TAG(tag)
}

#[repr(C)]
pub struct utsname {
    pub sysname: [c_char; 65],
    pub nodename: [c_char; 65],
    pub release: [c_char; 65],
    pub version: [c_char; 65],
    pub machine: [c_char; 65],
    pub domainname: [c_char; 65],
}

unsafe extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn prctl(option: c_int, ...) -> c_int;
    fn uname(buf: *mut utsname) -> c_int;

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_exit_fail_perror(msg: *const c_char) -> !;
    fn ksft_test_result(condition: bool, fmt: *const c_char, ...);
    fn ksft_finished();
}

fn main() {
    static mut tbi_enabled: c_int = 0;
    let mut tag: c_ulong = 0;
    let mut ptr: *mut utsname;

    unsafe {
        ksft_print_header();
        ksft_set_plan(1);

        if prctl(
            PR_SET_TAGGED_ADDR_CTRL,
            PR_TAGGED_ADDR_ENABLE,
            0 as c_ulong,
            0 as c_ulong,
            0 as c_ulong,
        ) == 0
        {
            tbi_enabled = 1;
        }

        ptr = malloc(core::mem::size_of::<utsname>()) as *mut utsname;
        if ptr.is_null() {
            ksft_exit_fail_perror(c"Failed to allocate utsname buffer".as_ptr());
        }

        if tbi_enabled != 0 {
            tag = 0x42;
        }
        ptr = SET_TAG(ptr as *mut c_void, tag as u64) as *mut utsname;
        ksft_test_result(
            uname(ptr) == 0,
            c"Syscall successful with tagged address\n".as_ptr(),
        );
        free(ptr as *mut c_void);

        ksft_finished();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
