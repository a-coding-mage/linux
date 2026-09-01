// SPDX-License-Identifier: GPL-2.0

// C dependencies removed from executable Rust:
// <vmlinux.h>, <errno.h>, <bpf/bpf_helpers.h>, and "task_local_data.bpf.h"
// are expected to provide the BPF helper declarations, task-local-data types,
// key symbols, and section handling used below.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};
use core::mem::{size_of, MaybeUninit};

type __u64 = u64;

extern "C" {
    type task_struct;
    type tld_object;
}

type tld_key_t = u64;

#[repr(C)]
pub struct tld_keys {
    pub value0: tld_key_t,
    pub value1: tld_key_t,
    pub value2: tld_key_t,
    pub value_not_exist: tld_key_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_tld_struct {
    pub a: __u64,
    pub b: __u64,
    pub c: __u64,
    pub d: __u64,
}

#[no_mangle]
pub static mut test_value0: c_int = 0;
#[no_mangle]
pub static mut test_value1: c_int = 0;
#[no_mangle]
pub static mut test_value2: MaybeUninit<test_tld_struct> = MaybeUninit::uninit();

extern "C" {
    static value0: tld_key_t;
    static value1: tld_key_t;
    static value2: tld_key_t;
    static value_not_exist: tld_key_t;

    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn tld_object_init(task: *mut task_struct, obj: *mut tld_object) -> c_int;
    fn tld_get_data(
        obj: *mut tld_object,
        key: tld_key_t,
        name: *const c_char,
        size: usize,
    ) -> *mut c_void;
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn task_main(_ctx: *mut c_void) -> c_int {
    let mut tld_obj = MaybeUninit::<tld_object>::uninit();
    let mut struct_p: *mut test_tld_struct;
    let mut task: *mut task_struct;
    let mut err: c_int;
    let mut int_p: *mut c_int;

    task = bpf_get_current_task_btf();
    err = tld_object_init(task, tld_obj.as_mut_ptr());
    if err != 0 {
        return 1;
    }

    int_p = tld_get_data(
        tld_obj.as_mut_ptr(),
        value0,
        b"value0\0".as_ptr() as *const c_char,
        size_of::<c_int>(),
    ) as *mut c_int;
    if !int_p.is_null() {
        test_value0 = *int_p;
    } else {
        return 2;
    }

    int_p = tld_get_data(
        tld_obj.as_mut_ptr(),
        value1,
        b"value1\0".as_ptr() as *const c_char,
        size_of::<c_int>(),
    ) as *mut c_int;
    if !int_p.is_null() {
        test_value1 = *int_p;
    } else {
        return 3;
    }

    struct_p = tld_get_data(
        tld_obj.as_mut_ptr(),
        value2,
        b"value2\0".as_ptr() as *const c_char,
        size_of::<test_tld_struct>(),
    ) as *mut test_tld_struct;
    if !struct_p.is_null() {
        test_value2.write(*struct_p);
    } else {
        return 4;
    }

    int_p = tld_get_data(
        tld_obj.as_mut_ptr(),
        value_not_exist,
        b"value_not_exist\0".as_ptr() as *const c_char,
        size_of::<c_int>(),
    ) as *mut c_int;
    if !int_p.is_null() {
        return 5;
    }

    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
