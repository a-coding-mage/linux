// SPDX-License-Identifier: GPL-2.0
// C dependencies: <test_progs.h>, "test_probe_read_user_str.skel.h"

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::{size_of, size_of_val};
use core::ptr;

#[repr(C)]
pub struct test_probe_read_user_str_bss {
    pub user_ptr: *mut c_char,
    pub ret: c_long,
    pub buf: [c_char; 256],
    pub pid: c_int,
}

#[repr(C)]
pub struct test_probe_read_user_str {
    pub bss: *mut test_probe_read_user_str_bss,
}

unsafe extern "C" {
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn getpid() -> c_int;

    fn test_probe_read_user_str__open_and_load() -> *mut test_probe_read_user_str;
    fn test_probe_read_user_str__attach(skel: *mut test_probe_read_user_str) -> c_int;
    fn test_probe_read_user_str__destroy(skel: *mut test_probe_read_user_str);
}

unsafe extern "C" {
    fn CHECK(condition: c_int, name: *const c_char, fmt: *const c_char, ...) -> c_int;
}

static str1: &[u8; 9] = b"mestring\0";
static str2: &[u8; 23] = b"mestringalittlebigger\0";
static str3: &[u8; 31] = b"mestringblubblubblubblubblub\0";

unsafe fn test_one_str(
    skel: *mut test_probe_read_user_str,
    str_: *const c_char,
    len: usize,
) -> c_int {
    let mut err: c_int;
    let mut duration: c_int = 0;
    let mut buf: [c_char; 256] = [0; 256];

    /* Ensure bytes after string are ones */
    memset(buf.as_mut_ptr() as *mut c_void, 1, size_of::<[c_char; 256]>());
    memcpy(buf.as_mut_ptr() as *mut c_void, str_ as *const c_void, len);

    /* Give prog our userspace pointer */
    (*(*skel).bss).user_ptr = buf.as_mut_ptr();

    /* Trigger tracepoint */
    usleep(1);

    /* Did helper fail? */
    if CHECK(
        ((*(*skel).bss).ret < 0) as c_int,
        c"prog_ret".as_ptr(),
        c"prog returned: %ld\n".as_ptr(),
        (*(*skel).bss).ret,
    ) != 0 {
        return 1;
    }

    /* Check that string was copied correctly */
    err = memcmp((*(*skel).bss).buf.as_ptr() as *const c_void, str_ as *const c_void, len);
    if CHECK((err != 0) as c_int, c"memcmp".as_ptr(), c"prog copied wrong string".as_ptr()) != 0 {
        return 1;
    }

    /* Now check that no extra trailing bytes were copied */
    memset(buf.as_mut_ptr() as *mut c_void, 0, size_of::<[c_char; 256]>());
    err = memcmp(
        (*(*skel).bss).buf.as_ptr().add(len) as *const c_void,
        buf.as_ptr() as *const c_void,
        size_of::<[c_char; 256]>() - len,
    );
    if CHECK(
        (err != 0) as c_int,
        c"memcmp".as_ptr(),
        c"trailing bytes were not stripped".as_ptr(),
    ) != 0 {
        return 1;
    }

    let _ = duration;
    return 0;
}

pub unsafe fn test_probe_read_user_str() {
    let mut skel: *mut test_probe_read_user_str;
    let mut err: c_int;
    let mut duration: c_int = 0;

    skel = test_probe_read_user_str__open_and_load();
    if CHECK(
        skel.is_null() as c_int,
        c"test_probe_read_user_str__open_and_load".as_ptr(),
        c"skeleton open and load failed\n".as_ptr(),
    ) != 0 {
        return;
    }

    /* Give pid to bpf prog so it doesn't read from anyone else */
    (*(*skel).bss).pid = getpid();

    err = test_probe_read_user_str__attach(skel);
    if CHECK(
        (err != 0) as c_int,
        c"test_probe_read_user_str__attach".as_ptr(),
        c"skeleton attach failed: %d\n".as_ptr(),
        err,
    ) != 0 {
        test_probe_read_user_str__destroy(skel);
        return;
    }

    if test_one_str(skel, str1.as_ptr() as *const c_char, size_of_val(str1)) != 0 {
        test_probe_read_user_str__destroy(skel);
        return;
    }
    if test_one_str(skel, str2.as_ptr() as *const c_char, size_of_val(str2)) != 0 {
        test_probe_read_user_str__destroy(skel);
        return;
    }
    if test_one_str(skel, str3.as_ptr() as *const c_char, size_of_val(str3)) != 0 {
        test_probe_read_user_str__destroy(skel);
        return;
    }

    test_probe_read_user_str__destroy(skel);
    let _ = duration;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
