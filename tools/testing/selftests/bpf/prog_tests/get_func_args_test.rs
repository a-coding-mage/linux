// SPDX-License-Identifier: GPL-2.0
// C dependencies translated as external declarations:
// <test_progs.h>
// "get_func_args_test.skel.h"
// "get_func_args_fsession_test.skel.h"

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub retval: u32,
}

impl Default for bpf_test_run_opts {
    fn default() -> Self {
        Self { retval: 0 }
    }
}

#[repr(C)]
pub struct get_func_args_test {
    pub progs: get_func_args_test__progs,
    pub bss: *mut get_func_args_test__bss,
}

#[repr(C)]
pub struct get_func_args_test__progs {
    pub test1: *mut bpf_program,
    pub fmod_ret_test: *mut bpf_program,
}

#[repr(C)]
pub struct get_func_args_test__bss {
    pub test1_result: c_int,
    pub test2_result: c_int,
    pub test3_result: c_int,
    pub test4_result: c_int,
    pub test5_result: c_int,
    pub test6_result: c_int,
}

#[repr(C)]
pub struct get_func_args_fsession_test {
    pub progs: get_func_args_fsession_test__progs,
    pub bss: *mut get_func_args_fsession_test__bss,
}

#[repr(C)]
pub struct get_func_args_fsession_test__progs {
    pub test1: *mut bpf_program,
}

#[repr(C)]
pub struct get_func_args_fsession_test__bss {
    pub test1_result: c_int,
}

unsafe extern "C" {
    fn get_func_args_test__open_and_load() -> *mut get_func_args_test;
    fn get_func_args_test__attach(skel: *mut get_func_args_test) -> c_int;
    fn get_func_args_test__destroy(skel: *mut get_func_args_test);

    fn get_func_args_fsession_test__open_and_load() -> *mut get_func_args_fsession_test;
    fn get_func_args_fsession_test__attach(skel: *mut get_func_args_fsession_test) -> c_int;
    fn get_func_args_fsession_test__destroy(skel: *mut get_func_args_fsession_test);

    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn trigger_module_test_read(arg: c_int) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(left: u64, right: u64, name: *const c_char) -> bool;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_get_func_args_test() {
    let mut skel: *mut get_func_args_test = core::ptr::null_mut();
    let mut err: c_int;
    let mut prog_fd: c_int;
    let mut topts = bpf_test_run_opts::default();

    skel = unsafe { get_func_args_test__open_and_load() };
    if !unsafe {
        ASSERT_OK_PTR(
            skel as *const c_void,
            c"get_func_args_test__open_and_load".as_ptr(),
        )
    } {
        return;
    }

    err = unsafe { get_func_args_test__attach(skel) };
    if !unsafe { ASSERT_OK(err, c"get_func_args_test__attach".as_ptr()) } {
        unsafe { get_func_args_test__destroy(skel) };
        return;
    }

    /* This runs bpf_fentry_test* functions and triggers
     * fentry/fexit programs.
     */
    prog_fd = unsafe { bpf_program__fd((*skel).progs.test1) };
    err = unsafe { bpf_prog_test_run_opts(prog_fd, &mut topts) };
    unsafe { ASSERT_OK(err, c"test_run".as_ptr()) };
    unsafe { ASSERT_EQ(topts.retval as u64, 0, c"test_run".as_ptr()) };

    /* This runs bpf_modify_return_test function and triggers
     * fmod_ret_test and fexit_test programs.
     */
    prog_fd = unsafe { bpf_program__fd((*skel).progs.fmod_ret_test) };
    err = unsafe { bpf_prog_test_run_opts(prog_fd, &mut topts) };
    unsafe { ASSERT_OK(err, c"test_run".as_ptr()) };

    unsafe { ASSERT_EQ((topts.retval >> 16) as u64, 1, c"test_run".as_ptr()) };
    unsafe { ASSERT_EQ((topts.retval & 0xffff) as u64, (1234 + 29) as u64, c"test_run".as_ptr()) };
    unsafe { ASSERT_OK(trigger_module_test_read(1), c"trigger_read".as_ptr()) };

    unsafe { ASSERT_EQ((*(*skel).bss).test1_result as u64, 1, c"test1_result".as_ptr()) };
    unsafe { ASSERT_EQ((*(*skel).bss).test2_result as u64, 1, c"test2_result".as_ptr()) };
    unsafe { ASSERT_EQ((*(*skel).bss).test3_result as u64, 1, c"test3_result".as_ptr()) };
    unsafe { ASSERT_EQ((*(*skel).bss).test4_result as u64, 1, c"test4_result".as_ptr()) };
    unsafe { ASSERT_EQ((*(*skel).bss).test5_result as u64, 1, c"test5_result".as_ptr()) };
    unsafe { ASSERT_EQ((*(*skel).bss).test6_result as u64, 1, c"test6_result".as_ptr()) };

    unsafe { get_func_args_test__destroy(skel) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_get_func_args_fsession_test() {
    let mut skel: *mut get_func_args_fsession_test = core::ptr::null_mut();
    let mut err: c_int;
    let mut topts = bpf_test_run_opts::default();

    skel = unsafe { get_func_args_fsession_test__open_and_load() };
    if !unsafe {
        ASSERT_OK_PTR(
            skel as *const c_void,
            c"get_func_args_fsession_test__open_and_load".as_ptr(),
        )
    } {
        return;
    }

    err = unsafe { get_func_args_fsession_test__attach(skel) };
    if !unsafe { ASSERT_OK(err, c"get_func_args_fsession_test__attach".as_ptr()) } {
        unsafe { get_func_args_fsession_test__destroy(skel) };
        return;
    }

    err = unsafe { bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.test1), &mut topts) };
    unsafe { ASSERT_OK(err, c"test_run".as_ptr()) };
    unsafe { ASSERT_EQ(topts.retval as u64, 0, c"test_run".as_ptr()) };

    unsafe { ASSERT_EQ((*(*skel).bss).test1_result as u64, 1, c"test1_result".as_ptr()) };

    unsafe { get_func_args_fsession_test__destroy(skel) };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
