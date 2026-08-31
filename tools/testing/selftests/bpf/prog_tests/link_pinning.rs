// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

/* Translated from C. Original includes:
 * <test_progs.h>
 * <sys/stat.h>
 * "test_link_pinning.skel.h"
 */

use std::ffi::c_char;
use std::os::raw::c_int;
use std::ptr;

static mut duration: c_int = 0;

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stat {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_link_pinning__bss {
    pub in_: c_int,
    pub out: c_int,
}

#[repr(C)]
pub struct test_link_pinning__progs {
    pub raw_tp_prog: *mut bpf_program,
    pub tp_btf_prog: *mut bpf_program,
}

#[repr(C)]
pub struct test_link_pinning {
    pub progs: test_link_pinning__progs,
    pub bss: *mut test_link_pinning__bss,
}

unsafe extern "C" {
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_link__pin(link: *mut bpf_link, path: *const c_char) -> c_int;
    fn bpf_link__pin_path(link: *mut bpf_link) -> *const c_char;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_link__open(path: *const c_char) -> *mut bpf_link;
    fn bpf_link__unpin(link: *mut bpf_link) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn stat(path: *const c_char, statbuf: *mut stat) -> c_int;
    fn usleep(usec: u32) -> c_int;
    fn test_link_pinning__open_and_load() -> *mut test_link_pinning;
    fn test_link_pinning__destroy(skel: *mut test_link_pinning);
    fn test__start_subtest(name: *const c_char) -> bool;

    static mut errno: c_int;
}

pub unsafe fn test_link_pinning_subtest(
    prog: *mut bpf_program,
    bss: *mut test_link_pinning__bss,
) {
    let link_pin_path = c"/sys/fs/bpf/pinned_link_test".as_ptr();
    let mut statbuf: stat = unsafe { std::mem::zeroed() };
    let mut link: *mut bpf_link;
    let mut err: c_int;
    let mut i: c_int;

    'body: {
        link = unsafe { bpf_program__attach(prog) };
        if !ASSERT_OK_PTR!(link, c"link_attach".as_ptr()) {
            break 'body;
        }

        unsafe {
            (*bss).in_ = 1;
            usleep(1);
        }
        CHECK!(
            unsafe { (*bss).out != 1 },
            c"res_check1".as_ptr(),
            c"exp %d, got %d\n".as_ptr(),
            1,
            unsafe { (*bss).out }
        );

        /* pin link */
        err = unsafe { bpf_link__pin(link, link_pin_path) };
        if CHECK!(err, c"link_pin".as_ptr(), c"err: %d\n".as_ptr(), err) {
            break 'body;
        }

        CHECK!(
            unsafe { strcmp(link_pin_path, bpf_link__pin_path(link)) },
            c"pin_path1".as_ptr(),
            c"exp %s, got %s\n".as_ptr(),
            link_pin_path,
            unsafe { bpf_link__pin_path(link) }
        );

        /* check that link was pinned */
        err = unsafe { stat(link_pin_path, &mut statbuf) };
        if CHECK!(
            err,
            c"stat_link".as_ptr(),
            c"err %d errno %d\n".as_ptr(),
            err,
            unsafe { errno }
        ) {
            break 'body;
        }

        unsafe {
            (*bss).in_ = 2;
            usleep(1);
        }
        CHECK!(
            unsafe { (*bss).out != 2 },
            c"res_check2".as_ptr(),
            c"exp %d, got %d\n".as_ptr(),
            2,
            unsafe { (*bss).out }
        );

        /* destroy link, pinned link should keep program attached */
        unsafe {
            bpf_link__destroy(link);
        }
        link = ptr::null_mut();

        unsafe {
            (*bss).in_ = 3;
            usleep(1);
        }
        CHECK!(
            unsafe { (*bss).out != 3 },
            c"res_check3".as_ptr(),
            c"exp %d, got %d\n".as_ptr(),
            3,
            unsafe { (*bss).out }
        );

        /* re-open link from BPFFS */
        link = unsafe { bpf_link__open(link_pin_path) };
        if !ASSERT_OK_PTR!(link, c"link_open".as_ptr()) {
            break 'body;
        }

        CHECK!(
            unsafe { strcmp(link_pin_path, bpf_link__pin_path(link)) },
            c"pin_path2".as_ptr(),
            c"exp %s, got %s\n".as_ptr(),
            link_pin_path,
            unsafe { bpf_link__pin_path(link) }
        );

        /* unpin link from BPFFS, program still attached */
        err = unsafe { bpf_link__unpin(link) };
        if CHECK!(err, c"link_unpin".as_ptr(), c"err: %d\n".as_ptr(), err) {
            break 'body;
        }

        /* still active, as we have FD open now */
        unsafe {
            (*bss).in_ = 4;
            usleep(1);
        }
        CHECK!(
            unsafe { (*bss).out != 4 },
            c"res_check4".as_ptr(),
            c"exp %d, got %d\n".as_ptr(),
            4,
            unsafe { (*bss).out }
        );

        unsafe {
            bpf_link__destroy(link);
        }
        link = ptr::null_mut();

        /* Validate it's finally detached.
         * Actual detachment might get delayed a bit, so there is no reliable
         * way to validate it immediately here, let's count up for long enough
         * and see if eventually output stops being updated
         */
        i = 5;
        while i < 10000 {
            unsafe {
                (*bss).in_ = i;
                usleep(1);
                if (*bss).out == i - 1 {
                    break;
                }
            }
            i += 1;
        }
        CHECK!(
            i == 10000,
            c"link_attached".as_ptr(),
            c"got to iteration #%d\n".as_ptr(),
            i
        );
    }

    unsafe {
        bpf_link__destroy(link);
    }
}

pub unsafe fn test_link_pinning() {
    let skel: *mut test_link_pinning;

    skel = unsafe { test_link_pinning__open_and_load() };
    if CHECK!(
        skel.is_null(),
        c"skel_open".as_ptr(),
        c"failed to open skeleton\n".as_ptr()
    ) {
        return;
    }

    if unsafe { test__start_subtest(c"pin_raw_tp".as_ptr()) } {
        unsafe {
            test_link_pinning_subtest((*skel).progs.raw_tp_prog, (*skel).bss);
        }
    }
    if unsafe { test__start_subtest(c"pin_tp_btf".as_ptr()) } {
        unsafe {
            test_link_pinning_subtest((*skel).progs.tp_btf_prog, (*skel).bss);
        }
    }

    unsafe {
        test_link_pinning__destroy(skel);
    }
}
