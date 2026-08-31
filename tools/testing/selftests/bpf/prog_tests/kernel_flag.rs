// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Microsoft */

// C dependencies:
// #include <test_progs.h>
// #include "kfunc_call_test.skel.h"
// #include "kfunc_call_test.lskel.h"
// #include "test_kernel_flag.skel.h"

#[repr(C)]
pub struct test_kernel_flag {
    pub bss: *mut test_kernel_flag_bss,
}

#[repr(C)]
pub struct test_kernel_flag_bss {
    pub monitored_tid: i32,
}

#[repr(C)]
pub struct kfunc_call_test {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kfunc_call_test_lskel {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn test_kernel_flag__open_and_load() -> *mut test_kernel_flag;
    fn test_kernel_flag__attach(skel: *mut test_kernel_flag) -> i32;
    fn test_kernel_flag__destroy(skel: *mut test_kernel_flag);

    fn kfunc_call_test__open_and_load() -> *mut kfunc_call_test;
    fn kfunc_call_test__destroy(skel: *mut kfunc_call_test);

    fn kfunc_call_test_lskel__open_and_load() -> *mut kfunc_call_test_lskel;
    fn kfunc_call_test_lskel__destroy(skel: *mut kfunc_call_test_lskel);

    fn sys_gettid() -> i32;

    fn ASSERT_OK_PTR(ptr: *const core::ffi::c_void, name: *const core::ffi::c_char) -> bool;
    fn ASSERT_OK(ret: i32, name: *const core::ffi::c_char) -> bool;
    fn ASSERT_ERR_PTR(ptr: *const core::ffi::c_void, name: *const core::ffi::c_char) -> bool;
}

pub unsafe fn test_kernel_flag() {
    let lsm_skel: *mut test_kernel_flag;
    let mut skel: *mut kfunc_call_test = core::ptr::null_mut();
    let mut lskel: *mut kfunc_call_test_lskel = core::ptr::null_mut();
    let mut ret: i32;

    lsm_skel = unsafe { test_kernel_flag__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(lsm_skel as *const core::ffi::c_void, c"lsm_skel".as_ptr()) } {
        return;
    }

    unsafe {
        (*(*lsm_skel).bss).monitored_tid = sys_gettid();
    }

    ret = unsafe { test_kernel_flag__attach(lsm_skel) };
    if !unsafe { ASSERT_OK(ret, c"test_kernel_flag__attach".as_ptr()) } {
        unsafe {
            (*(*lsm_skel).bss).monitored_tid = 0;
            test_kernel_flag__destroy(lsm_skel);
        }
        return;
    }

    /* Test with skel. This should pass the gatekeeper */
    skel = unsafe { kfunc_call_test__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(skel as *const core::ffi::c_void, c"skel".as_ptr()) } {
        unsafe {
            (*(*lsm_skel).bss).monitored_tid = 0;
            test_kernel_flag__destroy(lsm_skel);
        }
        return;
    }

    /* Test with lskel. This should fail due to blocking kernel-based bpf() invocations */
    lskel = unsafe { kfunc_call_test_lskel__open_and_load() };
    if !unsafe { ASSERT_ERR_PTR(lskel as *const core::ffi::c_void, c"lskel".as_ptr()) } {
        unsafe {
            if !skel.is_null() {
                kfunc_call_test__destroy(skel);
            }
            if !lskel.is_null() {
                kfunc_call_test_lskel__destroy(lskel);
            }

            (*(*lsm_skel).bss).monitored_tid = 0;
            test_kernel_flag__destroy(lsm_skel);
        }
        return;
    }

    unsafe {
        if !skel.is_null() {
            kfunc_call_test__destroy(skel);
        }
        if !lskel.is_null() {
            kfunc_call_test_lskel__destroy(lskel);
        }

        (*(*lsm_skel).bss).monitored_tid = 0;
        test_kernel_flag__destroy(lsm_skel);
    }
}
