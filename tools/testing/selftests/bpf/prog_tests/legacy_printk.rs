// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */
/* Dependencies from the original C includes:
 * <test_progs.h>
 * "test_legacy_printk.skel.h"
 */

use core::ffi::{c_int, c_uint, c_void};

type bool_ = bool;

const BPF_ANY: c_uint = 0;
const EINVAL: c_int = 22;

#[repr(C)]
pub struct test_legacy_printk {
    pub progs: test_legacy_printk__progs,
    pub maps: test_legacy_printk__maps,
    pub bss: *mut test_legacy_printk__bss,
}

#[repr(C)]
pub struct test_legacy_printk__progs {
    pub handle_legacy: *mut bpf_program,
    pub handle_modern: *mut bpf_program,
}

#[repr(C)]
pub struct test_legacy_printk__maps {
    pub my_pid_map: *mut bpf_map,
    pub res_map: *mut bpf_map,
}

#[repr(C)]
pub struct test_legacy_printk__bss {
    pub my_pid_var: c_int,
    pub res_var: c_int,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn test_legacy_printk__open() -> *mut test_legacy_printk;
    fn test_legacy_printk__load(skel: *mut test_legacy_printk) -> c_int;
    fn test_legacy_printk__attach(skel: *mut test_legacy_printk) -> c_int;
    fn test_legacy_printk__destroy(skel: *mut test_legacy_printk);

    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool_);
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: c_uint,
    ) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;

    fn getpid() -> c_int;
    fn usleep(usec: c_uint) -> c_int;

    fn __errno_location() -> *mut c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const u8) -> bool_;
    fn ASSERT_OK(err: c_int, name: *const u8) -> bool_;
    fn ASSERT_GT(actual: c_int, expected: c_int, name: *const u8) -> bool_;
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

unsafe fn execute_one_variant(legacy: bool_) -> c_int {
    let mut skel: *mut test_legacy_printk;
    let mut err: c_int;
    let mut zero: c_int = 0;
    let my_pid: c_int = getpid();
    let mut res: c_int = 0;
    let mut map_fd: c_int;

    skel = test_legacy_printk__open();
    if !ASSERT_OK_PTR(skel as *const c_void, b"skel_open\0".as_ptr()) {
        return -errno();
    }

    bpf_program__set_autoload((*skel).progs.handle_legacy, legacy);
    bpf_program__set_autoload((*skel).progs.handle_modern, !legacy);

    err = test_legacy_printk__load(skel);
    /* no ASSERT_OK, we expect one of two variants can fail here */
    if err != 0 {
        test_legacy_printk__destroy(skel);
        return err;
    }

    if legacy {
        map_fd = bpf_map__fd((*skel).maps.my_pid_map);
        err = bpf_map_update_elem(
            map_fd,
            &zero as *const _ as *const c_void,
            &my_pid as *const _ as *const c_void,
            BPF_ANY,
        );
        if !ASSERT_OK(err, b"my_pid_map_update\0".as_ptr()) {
            test_legacy_printk__destroy(skel);
            return err;
        }
        err = bpf_map_lookup_elem(
            map_fd,
            &zero as *const _ as *const c_void,
            &mut res as *mut _ as *mut c_void,
        );
    } else {
        (*(*skel).bss).my_pid_var = my_pid;
    }

    err = test_legacy_printk__attach(skel);
    if !ASSERT_OK(err, b"skel_attach\0".as_ptr()) {
        test_legacy_printk__destroy(skel);
        return err;
    }

    usleep(1); /* trigger */

    if legacy {
        map_fd = bpf_map__fd((*skel).maps.res_map);
        err = bpf_map_lookup_elem(
            map_fd,
            &zero as *const _ as *const c_void,
            &mut res as *mut _ as *mut c_void,
        );
        if !ASSERT_OK(err, b"res_map_lookup\0".as_ptr()) {
            test_legacy_printk__destroy(skel);
            return err;
        }
    } else {
        res = (*(*skel).bss).res_var;
    }

    if !ASSERT_GT(res, 0, b"res\0".as_ptr()) {
        err = -EINVAL;
        test_legacy_printk__destroy(skel);
        return err;
    }

    test_legacy_printk__destroy(skel);
    err
}

#[no_mangle]
pub unsafe extern "C" fn test_legacy_printk() {
    /* legacy variant should work everywhere */
    ASSERT_OK(execute_one_variant(true /* legacy */), b"legacy_case\0".as_ptr());

    /* execute modern variant, can fail the load on old kernels */
    execute_one_variant(false);
}
