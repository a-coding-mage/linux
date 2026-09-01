// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

// Translated from testing/selftests/bpf/prog_tests/autoload.c.
// External dependencies correspond to test_progs.h, time.h, and
// test_autoload.skel.h.

use core::ffi::{c_int, c_uint};

#[repr(C)]
pub struct test_autoload {
    pub progs: test_autoload__progs,
    pub bss: *mut test_autoload__bss,
}

#[repr(C)]
pub struct test_autoload__progs {
    pub prog3: *mut bpf_program,
}

#[repr(C)]
pub struct test_autoload__bss {
    pub prog1_called: bool,
    pub prog2_called: bool,
    pub prog3_called: bool,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn test_autoload__open_and_load() -> *mut test_autoload;
    fn test_autoload__open() -> *mut test_autoload;
    fn test_autoload__load(skel: *mut test_autoload) -> c_int;
    fn test_autoload__attach(skel: *mut test_autoload) -> c_int;
    fn test_autoload__destroy(skel: *mut test_autoload);
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn usleep(usec: c_uint) -> c_int;
}

pub unsafe fn test_autoload() {
    let duration: c_int = 0;
    let mut err: c_int;
    let mut skel: *mut test_autoload;

    'cleanup: loop {
        skel = test_autoload__open_and_load();
        /* prog3 should be broken */
        if CHECK!(skel, "skel_open_and_load", "unexpected success\n") {
            break 'cleanup;
        }

        skel = test_autoload__open();
        if CHECK!(skel.is_null(), "skel_open", "failed to open skeleton\n") {
            break 'cleanup;
        }

        /* don't load prog3 */
        bpf_program__set_autoload((*skel).progs.prog3, false);

        err = test_autoload__load(skel);
        if CHECK!(err, "skel_load", "failed to load skeleton: %d\n", err) {
            break 'cleanup;
        }

        err = test_autoload__attach(skel);
        if CHECK!(err, "skel_attach", "skeleton attach failed: %d\n", err) {
            break 'cleanup;
        }

        usleep(1);

        CHECK!(
            !(*(*skel).bss).prog1_called,
            "prog1",
            "not called\n"
        );
        CHECK!(
            !(*(*skel).bss).prog2_called,
            "prog2",
            "not called\n"
        );
        CHECK!((*(*skel).bss).prog3_called, "prog3", "called?!\n");

        break 'cleanup;
    }

    test_autoload__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
