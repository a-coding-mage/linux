// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// C dependencies:
// #include <test_progs.h>
// #include <sys/syscall.h>
// #include "linked_maps.skel.h"

use core::ffi::{c_char, c_int, c_long};

#[repr(C)]
pub struct linked_maps__bss {
    pub output_first1: c_int,
    pub output_second1: c_int,
    pub output_weak1: c_int,
}

#[repr(C)]
pub struct linked_maps {
    pub bss: *mut linked_maps__bss,
}

unsafe extern "C" {
    fn linked_maps__open_and_load() -> *mut linked_maps;
    fn linked_maps__attach(skel: *mut linked_maps) -> c_int;
    fn linked_maps__destroy(skel: *mut linked_maps);
    fn syscall(num: c_long, ...) -> c_long;
}

// SYS_getpgid is provided by <sys/syscall.h> in the original C source.

pub unsafe fn test_linked_maps() {
    let mut err: c_int;
    let skel: *mut linked_maps;

    skel = unsafe { linked_maps__open_and_load() };
    if !ASSERT_OK_PTR!(skel, c"skel_open".as_ptr() as *const c_char) {
        return;
    }

    err = unsafe { linked_maps__attach(skel) };
    if !ASSERT_OK!(err, c"skel_attach".as_ptr() as *const c_char) {
        goto_cleanup(skel);
        return;
    }

    /* trigger */
    unsafe {
        syscall(SYS_getpgid);
    }

    unsafe {
        ASSERT_EQ!(
            (*(*skel).bss).output_first1,
            2000,
            c"output_first1".as_ptr() as *const c_char
        );
        ASSERT_EQ!(
            (*(*skel).bss).output_second1,
            2,
            c"output_second1".as_ptr() as *const c_char
        );
        ASSERT_EQ!(
            (*(*skel).bss).output_weak1,
            2,
            c"output_weak1".as_ptr() as *const c_char
        );
    }

    goto_cleanup(skel);
}

unsafe fn goto_cleanup(skel: *mut linked_maps) {
    unsafe {
        linked_maps__destroy(skel);
    }
}
