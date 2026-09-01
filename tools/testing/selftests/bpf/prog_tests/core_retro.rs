// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook
// Original C dependencies:
// #define _GNU_SOURCE
// #include <test_progs.h>
// #include "test_core_retro.skel.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

type c_int = i32;
type size_t = usize;
type useconds_t = u32;

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_core_retro__maps {
    pub exp_tgid_map: *mut bpf_map,
    pub results: *mut bpf_map,
}

#[repr(C)]
pub struct test_core_retro {
    pub maps: test_core_retro__maps,
}

unsafe extern "C" {
    fn getpid() -> c_int;
    fn usleep(usec: useconds_t) -> c_int;

    fn test_core_retro__open_and_load() -> *mut test_core_retro;
    fn test_core_retro__attach(skel: *mut test_core_retro) -> c_int;
    fn test_core_retro__destroy(skel: *mut test_core_retro);

    fn bpf_map__update_elem(
        map: *mut bpf_map,
        key: *const core::ffi::c_void,
        key_sz: size_t,
        value: *const core::ffi::c_void,
        value_sz: size_t,
        flags: u64,
    ) -> c_int;
    fn bpf_map__lookup_elem(
        map: *mut bpf_map,
        key: *const core::ffi::c_void,
        key_sz: size_t,
        value: *mut core::ffi::c_void,
        value_sz: size_t,
        flags: u64,
    ) -> c_int;

    fn ASSERT_OK_PTR(ptr: *mut core::ffi::c_void, name: *const u8) -> bool;
    fn ASSERT_OK(err: c_int, name: *const u8) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const u8) -> bool;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_core_retro() {
    let mut err: c_int;
    let zero: c_int = 0;
    let mut res: c_int = 0;
    let my_pid: c_int = unsafe { getpid() };
    let skel: *mut test_core_retro;

    /* load program */
    skel = unsafe { test_core_retro__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(skel as *mut core::ffi::c_void, c"skel_load".as_ptr() as *const u8) } {
        unsafe { test_core_retro__destroy(skel) };
        return;
    }

    err = unsafe {
        bpf_map__update_elem(
            (*skel).maps.exp_tgid_map,
            &zero as *const _ as *const core::ffi::c_void,
            core::mem::size_of_val(&zero),
            &my_pid as *const _ as *const core::ffi::c_void,
            core::mem::size_of_val(&my_pid),
            0,
        )
    };
    if !unsafe { ASSERT_OK(err, c"map_update".as_ptr() as *const u8) } {
        unsafe { test_core_retro__destroy(skel) };
        return;
    }

    /* attach probe */
    err = unsafe { test_core_retro__attach(skel) };
    if !unsafe { ASSERT_OK(err, c"attach_kprobe".as_ptr() as *const u8) } {
        unsafe { test_core_retro__destroy(skel) };
        return;
    }

    /* trigger */
    unsafe {
        usleep(1);
    }

    err = unsafe {
        bpf_map__lookup_elem(
            (*skel).maps.results,
            &zero as *const _ as *const core::ffi::c_void,
            core::mem::size_of_val(&zero),
            &mut res as *mut _ as *mut core::ffi::c_void,
            core::mem::size_of_val(&res),
            0,
        )
    };
    if !unsafe { ASSERT_OK(err, c"map_lookup".as_ptr() as *const u8) } {
        unsafe { test_core_retro__destroy(skel) };
        return;
    }

    unsafe {
        ASSERT_EQ(res, my_pid, c"pid_check".as_ptr() as *const u8);
    }

    unsafe {
        test_core_retro__destroy(skel);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
