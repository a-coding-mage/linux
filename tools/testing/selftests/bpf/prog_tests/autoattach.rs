// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Google */

// C dependencies:
// #include <test_progs.h>
// #include "test_autoattach.skel.h"

#[repr(C)]
pub struct test_autoattach {
    pub progs: test_autoattach__progs,
    pub bss: *mut test_autoattach__bss,
}

#[repr(C)]
pub struct test_autoattach__progs {
    pub prog1: *mut bpf_program,
    pub prog2: *mut bpf_program,
}

#[repr(C)]
pub struct test_autoattach__bss {
    pub prog1_called: bool,
    pub prog2_called: bool,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn test_autoattach__open_and_load() -> *mut test_autoattach;
    fn test_autoattach__attach(skel: *mut test_autoattach) -> i32;
    fn test_autoattach__destroy(skel: *mut test_autoattach);

    fn bpf_program__set_autoattach(prog: *mut bpf_program, autoattach: bool);
    fn bpf_program__autoattach(prog: *mut bpf_program) -> bool;

    fn ASSERT_OK_PTR(ptr: *mut test_autoattach, name: *const ::core::ffi::c_char) -> bool;
    fn ASSERT_OK(res: i32, name: *const ::core::ffi::c_char) -> bool;
    fn ASSERT_TRUE(cond: bool, name: *const ::core::ffi::c_char) -> bool;
    fn ASSERT_FALSE(cond: bool, name: *const ::core::ffi::c_char) -> bool;

    fn usleep(usec: ::core::ffi::c_uint) -> ::core::ffi::c_int;
}

pub unsafe extern "C" fn test_autoattach() {
    let mut skel: *mut test_autoattach;

    skel = unsafe { test_autoattach__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(skel, c"skel_open_and_load".as_ptr()) } {
        goto_cleanup(skel);
        return;
    }

    /* disable auto-attach for prog2 */
    unsafe {
        bpf_program__set_autoattach((*skel).progs.prog2, false);
    }
    unsafe {
        ASSERT_TRUE(
            bpf_program__autoattach((*skel).progs.prog1),
            c"autoattach_prog1".as_ptr(),
        );
        ASSERT_FALSE(
            bpf_program__autoattach((*skel).progs.prog2),
            c"autoattach_prog2".as_ptr(),
        );
    }
    if !unsafe { ASSERT_OK(test_autoattach__attach(skel), c"skel_attach".as_ptr()) } {
        goto_cleanup(skel);
        return;
    }

    unsafe {
        usleep(1);
    }

    unsafe {
        ASSERT_TRUE((*(*skel).bss).prog1_called, c"attached_prog1".as_ptr());
        ASSERT_FALSE((*(*skel).bss).prog2_called, c"attached_prog2".as_ptr());
    }

    goto_cleanup(skel);
}

unsafe fn goto_cleanup(skel: *mut test_autoattach) {
    unsafe {
        test_autoattach__destroy(skel);
    }
}
