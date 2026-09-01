// SPDX-License-Identifier: GPL-2.0
//
// Translated from testing/selftests/net/tcp_ao/lib/utils.c.
// Dependencies from "aolib.h", libc, and system socket headers are expected to
// be supplied by the surrounding translation unit/crate.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void, VaListImpl};

extern "C" {
    fn rand() -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fwrite(ptr: *const c_void, size: usize, nmemb: usize, stream: *mut FILE) -> usize;
    fn strlen(s: *const c_char) -> usize;
    fn free(ptr: *mut c_void);
    fn test_snprintf(fmt: *const c_char, args: VaListImpl<'_>) -> *mut c_char;

    static mut errno: c_int;
}

extern "C" {
    type FILE;
}

pub unsafe extern "C" fn randomize_buffer(buf: *mut c_void, buflen: usize) {
    let mut p = buf as *mut c_int;
    let mut words = buflen / core::mem::size_of::<c_int>();
    let leftover = buflen % core::mem::size_of::<c_int>();

    if buflen == 0 {
        return;
    }

    while words != 0 {
        words -= 1;
        *p = rand();
        p = p.add(1);
    }

    if leftover != 0 {
        let tmp: c_int = rand();

        memcpy(
            (buf as *mut u8).add(buflen - leftover) as *mut c_void,
            &tmp as *const c_int as *const c_void,
            leftover,
        );
    }
}

// C attribute preserved from source: __printf(3, 4)
pub unsafe extern "C" fn test_echo(
    fname: *const c_char,
    append: bool,
    fmt: *const c_char,
    mut vargs: ...
) -> c_int {
    let len: usize;
    let written: usize;
    let msg: *mut c_char;
    let f: *mut FILE;

    f = fopen(fname, if append { b"a\0".as_ptr() } else { b"w\0".as_ptr() } as *const c_char);
    if f.is_null() {
        return -errno;
    }

    msg = test_snprintf(fmt, vargs.as_va_list());
    if msg.is_null() {
        fclose(f);
        return -1;
    }
    len = strlen(msg);
    written = fwrite(msg as *const c_void, 1, len, f);
    fclose(f);
    free(msg as *mut c_void);
    if written == len { 0 } else { -1 }
}

pub static addr_any6: sockaddr_in6 = sockaddr_in6 {
    sin6_family: AF_INET6,
    ..unsafe { core::mem::zeroed() }
};

pub static addr_any4: sockaddr_in = sockaddr_in {
    sin_family: AF_INET,
    ..unsafe { core::mem::zeroed() }
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
