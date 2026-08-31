// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
/* Dependencies from C source:
 * #include <test_progs.h>
 * #include "test_subprogs.skel.h"
 * #include "test_subprogs_unused.skel.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
struct toggler_ctx {
    fd: c_int,
    stop: bool,
}

#[repr(C)]
struct test_subprogs_bss {
    res1: c_int,
    res2: c_int,
    res3: c_int,
    res4: c_int,
}

#[repr(C)]
struct test_subprogs {
    bss: *mut test_subprogs_bss,
}

#[repr(C)]
struct test_subprogs_unused {
    _private: [u8; 0],
}

type pthread_t = usize;
type off_t = isize;
type size_t = usize;
type ssize_t = isize;

const SEEK_SET: c_int = 0;
const O_RDWR: c_int = 0o2;

extern "C" {
    fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn test_subprogs__open_and_load() -> *mut test_subprogs;
    fn test_subprogs__attach(skel: *mut test_subprogs) -> c_int;
    fn test_subprogs__destroy(skel: *mut test_subprogs);
    fn test_subprogs_unused__open_and_load() -> *mut test_subprogs_unused;
    fn test_subprogs_unused__destroy(skel: *mut test_subprogs_unused);

    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
}

unsafe extern "C" fn toggle_jit_harden(arg: *mut c_void) -> *mut c_void {
    let ctx: *mut toggler_ctx = arg as *mut toggler_ctx;
    let two: c_char = b'2' as c_char;
    let zero: c_char = b'0' as c_char;

    while !(*ctx).stop {
        lseek((*ctx).fd, 0, SEEK_SET);
        write(
            (*ctx).fd,
            &two as *const c_char as *const c_void,
            core::mem::size_of_val(&two),
        );
        lseek((*ctx).fd, 0, SEEK_SET);
        write(
            (*ctx).fd,
            &zero as *const c_char as *const c_void,
            core::mem::size_of_val(&zero),
        );
    }

    core::ptr::null_mut()
}

unsafe fn test_subprogs_with_jit_harden_toggling() {
    let mut ctx: toggler_ctx = core::mem::zeroed();
    let mut toggler: pthread_t = core::mem::zeroed();
    let mut err: c_int;
    let mut i: c_uint;
    let loop_: c_uint = 10;

    ctx.fd = open(c"/proc/sys/net/core/bpf_jit_harden".as_ptr(), O_RDWR);
    if !ASSERT_GE(ctx.fd, 0, c"open bpf_jit_harden".as_ptr()) {
        return;
    }

    ctx.stop = false;
    err = pthread_create(
        &mut toggler,
        core::ptr::null(),
        toggle_jit_harden,
        &mut ctx as *mut toggler_ctx as *mut c_void,
    );
    if !ASSERT_OK(err, c"new toggler".as_ptr()) {
        close(ctx.fd);
        return;
    }

    /* Make toggler thread to run */
    usleep(1);

    i = 0;
    while i < loop_ {
        let skel: *mut test_subprogs = test_subprogs__open_and_load();

        if !ASSERT_OK_PTR(skel as *const c_void, c"skel open".as_ptr()) {
            break;
        }
        test_subprogs__destroy(skel);
        i = i.wrapping_add(1);
    }

    ctx.stop = true;
    pthread_join(toggler, core::ptr::null_mut());
    close(ctx.fd);
}

unsafe fn test_subprogs_alone() {
    let mut skel: *mut test_subprogs;
    let skel2: *mut test_subprogs_unused;
    let mut err: c_int;

    skel = test_subprogs__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
        return;
    }

    err = test_subprogs__attach(skel);
    if !ASSERT_OK(err, c"skel attach".as_ptr()) {
        test_subprogs__destroy(skel);
        return;
    }

    usleep(1);

    ASSERT_EQ((*(*skel).bss).res1, 12, c"res1".as_ptr());
    ASSERT_EQ((*(*skel).bss).res2, 17, c"res2".as_ptr());
    ASSERT_EQ((*(*skel).bss).res3, 19, c"res3".as_ptr());
    ASSERT_EQ((*(*skel).bss).res4, 36, c"res4".as_ptr());

    skel2 = test_subprogs_unused__open_and_load();
    ASSERT_OK_PTR(skel2 as *const c_void, c"unused_progs_skel".as_ptr());
    test_subprogs_unused__destroy(skel2);

    test_subprogs__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_subprogs() {
    if test__start_subtest(c"subprogs_alone".as_ptr()) {
        test_subprogs_alone();
    }
    if test__start_subtest(c"subprogs_and_jit_harden".as_ptr()) {
        test_subprogs_with_jit_harden_toggling();
    }
}
