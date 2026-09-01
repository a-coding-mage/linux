// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

/* Dependencies from the original C source:
 * #include <test_progs.h>
 * #include <sys/syscall.h>
 * #include "linked_funcs.skel.h"
 */

use std::os::raw::{c_char, c_int, c_long, c_void};

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct linked_funcs {
    pub progs: linked_funcs_progs,
    pub rodata: *mut linked_funcs_rodata,
    pub bss: *mut linked_funcs_bss,
}

#[repr(C)]
pub struct linked_funcs_progs {
    pub handler1: *mut bpf_program,
    pub handler2: *mut bpf_program,
}

#[repr(C)]
pub struct linked_funcs_rodata {
    pub my_tid: c_int,
}

#[repr(C)]
pub struct linked_funcs_bss {
    pub syscall_id: c_long,
    pub output_val1: c_int,
    pub output_ctx1: c_long,
    pub output_weak1: c_int,
    pub output_val2: c_int,
    pub output_ctx2: c_long,
    pub output_weak2: c_int,
}

unsafe extern "C" {
    fn linked_funcs__open() -> *mut linked_funcs;
    fn linked_funcs__load(skel: *mut linked_funcs) -> c_int;
    fn linked_funcs__attach(skel: *mut linked_funcs) -> c_int;
    fn linked_funcs__destroy(skel: *mut linked_funcs);

    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn sys_gettid() -> c_int;
    fn syscall(number: c_long, ...) -> c_long;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_long, expected: c_long, name: *const c_char) -> bool;

    /* SYS_getpgid is supplied by <sys/syscall.h> as a C preprocessor constant. */
    static SYS_getpgid: c_long;
}

pub unsafe fn test_linked_funcs() {
    let mut err: c_int;
    let skel: *mut linked_funcs;

    skel = unsafe { linked_funcs__open() };
    if !unsafe { ASSERT_OK_PTR(skel as *const c_void, b"skel_open\0".as_ptr() as *const c_char) } {
        return;
    }

    /* handler1 and handler2 are marked as SEC("?raw_tp/sys_enter") and
     * are set to not autoload by default
     */
    unsafe {
        bpf_program__set_autoload((*skel).progs.handler1, true);
        bpf_program__set_autoload((*skel).progs.handler2, true);
    }

    unsafe {
        (*(*skel).rodata).my_tid = sys_gettid();
        (*(*skel).bss).syscall_id = SYS_getpgid;
    }

    err = unsafe { linked_funcs__load(skel) };
    if !unsafe { ASSERT_OK(err, b"skel_load\0".as_ptr() as *const c_char) } {
        goto_cleanup(skel);
        return;
    }

    err = unsafe { linked_funcs__attach(skel) };
    if !unsafe { ASSERT_OK(err, b"skel_attach\0".as_ptr() as *const c_char) } {
        goto_cleanup(skel);
        return;
    }

    /* trigger */
    unsafe {
        syscall(SYS_getpgid);
    }

    unsafe {
        ASSERT_EQ(
            (*(*skel).bss).output_val1 as c_long,
            (2000 + 2000) as c_long,
            b"output_val1\0".as_ptr() as *const c_char,
        );
        ASSERT_EQ(
            (*(*skel).bss).output_ctx1,
            SYS_getpgid,
            b"output_ctx1\0".as_ptr() as *const c_char,
        );
        ASSERT_EQ(
            (*(*skel).bss).output_weak1 as c_long,
            42,
            b"output_weak1\0".as_ptr() as *const c_char,
        );

        ASSERT_EQ(
            (*(*skel).bss).output_val2 as c_long,
            (2 * 1000 + 2 * (2 * 1000)) as c_long,
            b"output_val2\0".as_ptr() as *const c_char,
        );
        ASSERT_EQ(
            (*(*skel).bss).output_ctx2,
            SYS_getpgid,
            b"output_ctx2\0".as_ptr() as *const c_char,
        );
        /* output_weak2 should never be updated */
        ASSERT_EQ(
            (*(*skel).bss).output_weak2 as c_long,
            0,
            b"output_weak2\0".as_ptr() as *const c_char,
        );
    }

    goto_cleanup(skel);
}

unsafe fn goto_cleanup(skel: *mut linked_funcs) {
    unsafe {
        linked_funcs__destroy(skel);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
