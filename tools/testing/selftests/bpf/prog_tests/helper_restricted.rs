// SPDX-License-Identifier: GPL-2.0

// Dependencies from C includes:
// <test_progs.h>
// "test_helper_restricted.skel.h"

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_helper_restricted {
    pub skeleton: *mut bpf_object_skeleton,
}

#[repr(C)]
pub struct bpf_object_skeleton {
    pub prog_cnt: c_int,
    pub progs: *mut bpf_prog_skeleton,
}

#[repr(C)]
pub struct bpf_prog_skeleton {
    pub prog: *mut *mut bpf_program,
}

unsafe extern "C" {
    fn test_helper_restricted__open() -> *mut test_helper_restricted;
    fn test_helper_restricted__load(obj: *mut test_helper_restricted) -> c_int;
    fn test_helper_restricted__destroy(obj: *mut test_helper_restricted);

    fn ASSERT_OK_PTR(ptr: *const test_helper_restricted, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_helper_restricted() {
    let mut prog_i: c_int = 0;
    let mut prog_cnt: c_int;

    loop {
        let test: *mut test_helper_restricted;
        let err: c_int;

        test = unsafe { test_helper_restricted__open() };
        if !unsafe { ASSERT_OK_PTR(test, c"open".as_ptr()) } {
            return;
        }

        prog_cnt = unsafe { (*(*test).skeleton).prog_cnt };

        let mut j: c_int = 0;
        while j < prog_cnt {
            let prog: *mut bpf_program =
                unsafe { *(*((*(*test).skeleton).progs.add(j as usize))).prog };

            unsafe { bpf_program__set_autoload(prog, true) };
            j += 1;
        }

        err = unsafe { test_helper_restricted__load(test) };
        unsafe { ASSERT_ERR(err, c"load_should_fail".as_ptr()) };

        unsafe { test_helper_restricted__destroy(test) };

        prog_i += 1;
        if prog_i >= prog_cnt {
            break;
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
