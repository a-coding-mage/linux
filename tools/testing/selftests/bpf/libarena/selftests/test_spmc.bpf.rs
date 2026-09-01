// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause

// Dependencies from the original C source:
// #include <libarena/common.h>
// #include <libarena/asan.h>
// #include <libarena/spmc.h>

/*
 * NOTE: These selftests only test for the single-threaded use case, which for
 * Lev-Chase queues is obviously the simplest one. Still, it is important to
 * exercise the API to ensure it passes verification and basic checks.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int};

type u64 = u64;

const ENOENT: c_int = 2;

#[repr(C)]
pub struct spmc {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut can_loop: bool;

    fn spmc_create() -> *mut spmc;
    fn spmc_destroy(spmc: *mut spmc);
    fn spmc_owned_add(spmc: *mut spmc, val: u64) -> c_int;
    fn spmc_owned_remove(spmc: *mut spmc, val: *mut u64) -> c_int;
    fn spmc_steal(spmc: *mut spmc, val: *mut u64) -> c_int;

    fn arena_stderr(fmt: *const c_char, ...) -> c_int;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "syscall")]
pub unsafe extern "C" fn test_spmc_remove_empty() -> c_int {
    let mut val: u64 = 0;
    let mut ret: c_int;

    let spmc: *mut spmc = unsafe { spmc_create() };

    if spmc.is_null() {
        return 1;
    }

    ret = unsafe { spmc_owned_remove(spmc, &mut val) };
    if ret != -ENOENT {
        return 1;
    }

    unsafe { spmc_destroy(spmc) };

    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "syscall")]
pub unsafe extern "C" fn test_spmc_steal_empty() -> c_int {
    let mut val: u64 = 0;
    let mut ret: c_int;

    let spmc: *mut spmc = unsafe { spmc_create() };

    if spmc.is_null() {
        return 1;
    }

    ret = unsafe { spmc_steal(spmc, &mut val) };
    if ret != -ENOENT {
        return 1;
    }

    unsafe { spmc_destroy(spmc) };

    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "syscall")]
pub unsafe extern "C" fn test_spmc_steal_one() -> c_int {
    let mut val: u64;
    let mut newval: u64 = 0;
    let mut ret: c_int;
    let mut i: c_int;

    let spmc: *mut spmc = unsafe { spmc_create() };

    if spmc.is_null() {
        return 1;
    }

    i = 0;
    while i < 10 && unsafe { can_loop } {
        val = i as u64;

        ret = unsafe { spmc_owned_add(spmc, val) };
        if ret != 0 {
            return 1;
        }

        ret = unsafe { spmc_steal(spmc, &mut newval) };
        if ret != 0 {
            return 2;
        }

        if val != newval {
            return 3;
        }

        i += 1;
    }

    unsafe { spmc_destroy(spmc) };

    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "syscall")]
pub unsafe extern "C" fn test_spmc_remove_one() -> c_int {
    let mut val: u64;
    let mut newval: u64 = 0;
    let mut ret: c_int;
    let mut i: c_int;

    let spmc: *mut spmc = unsafe { spmc_create() };

    if spmc.is_null() {
        return 1;
    }

    i = 0;
    while i < 10 && unsafe { can_loop } {
        val = i as u64;

        ret = unsafe { spmc_owned_add(spmc, val) };
        if ret != 0 {
            return 1;
        }

        ret = unsafe { spmc_owned_remove(spmc, &mut newval) };
        if ret != 0 {
            return 2;
        }

        if val != newval {
            return 3;
        }

        i += 1;
    }

    unsafe { spmc_destroy(spmc) };

    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "syscall")]
pub unsafe extern "C" fn test_spmc_remove_many() -> c_int {
    let mut val: u64;
    let mut newval: u64 = 0;
    let mut ret: c_int;
    let mut i: c_int;
    let mut expected: u64;

    let spmc: *mut spmc = unsafe { spmc_create() };

    if spmc.is_null() {
        return 1;
    }

    i = 0;
    while i < 500 && unsafe { can_loop } {
        val = i as u64;

        ret = unsafe { spmc_owned_add(spmc, val) };
        if ret != 0 {
            unsafe {
                arena_stderr(
                    b"%s:%d error %d\n\0".as_ptr() as *const c_char,
                    b"test_spmc_remove_many\0".as_ptr() as *const c_char,
                    106 as c_int,
                    ret,
                );
            }
            return 1;
        }

        i += 1;
    }

    i = 0;
    while i < 500 && unsafe { can_loop } {
        ret = unsafe { spmc_owned_remove(spmc, &mut newval) };
        if ret != 0 {
            unsafe {
                arena_stderr(
                    b"%s:%d error %d\n\0".as_ptr() as *const c_char,
                    b"test_spmc_remove_many\0".as_ptr() as *const c_char,
                    115 as c_int,
                    ret,
                );
            }
            return 1;
        }

        expected = (500 - 1 - i) as u64;
        if newval != expected {
            unsafe {
                arena_stderr(
                    b"%s:%d expected %llu found %llu\n\0".as_ptr() as *const c_char,
                    b"test_spmc_remove_many\0".as_ptr() as *const c_char,
                    122 as c_int,
                    expected,
                    newval,
                );
            }
            return 1;
        }

        i += 1;
    }

    unsafe { spmc_destroy(spmc) };

    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "syscall")]
pub unsafe extern "C" fn test_spmc_steal_many() -> c_int {
    let mut val: u64;
    let mut newval: u64 = 0;
    let mut ret: c_int;
    let mut i: c_int;

    let spmc: *mut spmc = unsafe { spmc_create() };

    if spmc.is_null() {
        return 1;
    }

    i = 0;
    while i < 500 && unsafe { can_loop } {
        val = i as u64;

        ret = unsafe { spmc_owned_add(spmc, val) };
        if ret != 0 {
            unsafe {
                arena_stderr(
                    b"%s:%d error %d\n\0".as_ptr() as *const c_char,
                    b"test_spmc_steal_many\0".as_ptr() as *const c_char,
                    145 as c_int,
                    ret,
                );
            }
            return 1;
        }

        i += 1;
    }

    i = 0;
    while i < 500 && unsafe { can_loop } {
        ret = unsafe { spmc_steal(spmc, &mut newval) };
        if ret != 0 {
            unsafe {
                arena_stderr(
                    b"%s:%d error %d\n\0".as_ptr() as *const c_char,
                    b"test_spmc_steal_many\0".as_ptr() as *const c_char,
                    154 as c_int,
                    ret,
                );
            }
            return 1;
        }

        if newval != i as u64 {
            unsafe {
                arena_stderr(
                    b"%s:%d expected %d found %llu\n\0".as_ptr() as *const c_char,
                    b"test_spmc_steal_many\0".as_ptr() as *const c_char,
                    160 as c_int,
                    i,
                    newval,
                );
            }
            return 1;
        }

        i += 1;
    }

    unsafe { spmc_destroy(spmc) };

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
