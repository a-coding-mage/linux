// SPDX-License-Identifier: GPL-2.0
// Translated from C source depending on test_progs.h and network_helpers.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type __u8 = u8;
type __u32 = u32;
type __u64 = u64;
type uint32_t = u32;

const ENOMEM: c_int = 12;
const EPERM: c_int = 1;
const BPF_PROG_TYPE_SCHED_CLS: c_int = 3;

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub data_in: *mut c_void,
    pub data_size_in: u32,
    pub repeat: u32,
    pub retval: u32,
    pub duration: u32,
}

unsafe extern "C" {
    static mut pkt_v4: c_void;
    static mut errno: c_int;

    fn bpf_find_map(test: *const c_char, obj: *mut bpf_object, name: *const c_char) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_object__find_map_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_map;
    fn bpf_map__is_internal(map: *mut bpf_map) -> bool;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map__value_size(map: *mut bpf_map) -> usize;
    fn bpf_prog_test_load(
        file: *const c_char,
        prog_type: c_int,
        obj: *mut *mut bpf_object,
        prog_fd: *mut c_int,
    ) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;

    fn CHECK_FAIL(cond: bool) -> bool;
    fn CHECK(cond: bool, name: *const c_char, fmt: *const c_char, ...) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut bpf_map, name: *const c_char) -> bool;
    fn ASSERT_TRUE(cond: bool, name: *const c_char) -> bool;
    fn ASSERT_EQ(left: *mut bpf_map, right: *mut bpf_map, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
}

#[repr(C)]
struct foo {
    a: __u8,
    b: __u32,
    c: __u64,
}

#[repr(C)]
struct number_test {
    name: *const c_char,
    key: uint32_t,
    num: __u64,
}

#[repr(C)]
struct string_test {
    name: *const c_char,
    key: uint32_t,
    str_: [c_char; 32],
}

#[repr(C)]
struct struct_test {
    name: *const c_char,
    key: uint32_t,
    val: foo,
}

const fn cstr32(bytes: &[u8]) -> [c_char; 32] {
    let mut out = [0 as c_char; 32];
    let mut i = 0;
    while i < bytes.len() && i < 32 {
        out[i] = bytes[i] as c_char;
        i += 1;
    }
    out
}

unsafe fn test_global_data_number(obj: *mut bpf_object, _duration: __u32) {
    let mut num: __u64 = 0;

    let map_fd = bpf_find_map(
        b"test_global_data_number\0".as_ptr() as *const c_char,
        obj,
        b"result_number\0".as_ptr() as *const c_char,
    );
    if CHECK_FAIL(map_fd < 0) {
        return;
    }

    let tests = [
        number_test { name: b"relocate .bss reference\0".as_ptr() as *const c_char, key: 0, num: 0 },
        number_test { name: b"relocate .data reference\0".as_ptr() as *const c_char, key: 1, num: 42 },
        number_test { name: b"relocate .rodata reference\0".as_ptr() as *const c_char, key: 2, num: 24 },
        number_test { name: b"relocate .bss reference\0".as_ptr() as *const c_char, key: 3, num: 0 },
        number_test { name: b"relocate .data reference\0".as_ptr() as *const c_char, key: 4, num: 0xffeeff },
        number_test { name: b"relocate .rodata reference\0".as_ptr() as *const c_char, key: 5, num: 0xabab },
        number_test { name: b"relocate .bss reference\0".as_ptr() as *const c_char, key: 6, num: 1234 },
        number_test { name: b"relocate .bss reference\0".as_ptr() as *const c_char, key: 7, num: 0 },
        number_test { name: b"relocate .rodata reference\0".as_ptr() as *const c_char, key: 8, num: 0xab },
        number_test { name: b"relocate .rodata reference\0".as_ptr() as *const c_char, key: 9, num: 0x1111111111111111 },
        number_test { name: b"relocate .rodata reference\0".as_ptr() as *const c_char, key: 10, num: !0u64 },
    ];

    let mut i = 0usize;
    while i < tests.len() {
        let err = bpf_map_lookup_elem(
            map_fd,
            &tests[i].key as *const _ as *const c_void,
            &mut num as *mut _ as *mut c_void,
        );
        CHECK(
            err != 0 || num != tests[i].num,
            tests[i].name,
            b"err %d result %llx expected %llx\n\0".as_ptr() as *const c_char,
            err,
            num,
            tests[i].num,
        );
        i += 1;
    }
}

unsafe fn test_global_data_string(obj: *mut bpf_object, _duration: __u32) {
    let mut str_ = [0 as c_char; 32];

    let map_fd = bpf_find_map(
        b"test_global_data_string\0".as_ptr() as *const c_char,
        obj,
        b"result_string\0".as_ptr() as *const c_char,
    );
    if CHECK_FAIL(map_fd < 0) {
        return;
    }

    let tests = [
        string_test { name: b"relocate .rodata reference\0".as_ptr() as *const c_char, key: 0, str_: cstr32(b"abcdefghijklmnopqrstuvwxyz") },
        string_test { name: b"relocate .data reference\0".as_ptr() as *const c_char, key: 1, str_: cstr32(b"abcdefghijklmnopqrstuvwxyz") },
        string_test { name: b"relocate .bss reference\0".as_ptr() as *const c_char, key: 2, str_: cstr32(b"") },
        string_test { name: b"relocate .data reference\0".as_ptr() as *const c_char, key: 3, str_: cstr32(b"abcdexghijklmnopqrstuvwxyz") },
        string_test { name: b"relocate .bss reference\0".as_ptr() as *const c_char, key: 4, str_: cstr32(b"\0\0hello") },
    ];

    let mut i = 0usize;
    while i < tests.len() {
        let err = bpf_map_lookup_elem(
            map_fd,
            &tests[i].key as *const _ as *const c_void,
            str_.as_mut_ptr() as *mut c_void,
        );
        CHECK(
            err != 0
                || memcmp(
                    str_.as_ptr() as *const c_void,
                    tests[i].str_.as_ptr() as *const c_void,
                    core::mem::size_of_val(&str_),
                ) != 0,
            tests[i].name,
            b"err %d result '%s' expected '%s'\n\0".as_ptr() as *const c_char,
            err,
            str_.as_ptr(),
            tests[i].str_.as_ptr(),
        );
        i += 1;
    }
}

unsafe fn test_global_data_struct(obj: *mut bpf_object, _duration: __u32) {
    let mut val = foo { a: 0, b: 0, c: 0 };

    let map_fd = bpf_find_map(
        b"test_global_data_struct\0".as_ptr() as *const c_char,
        obj,
        b"result_struct\0".as_ptr() as *const c_char,
    );
    if CHECK_FAIL(map_fd < 0) {
        return;
    }

    let tests = [
        struct_test { name: b"relocate .rodata reference\0".as_ptr() as *const c_char, key: 0, val: foo { a: 42, b: 0xfefeefef, c: 0x1111111111111111 } },
        struct_test { name: b"relocate .bss reference\0".as_ptr() as *const c_char, key: 1, val: foo { a: 0, b: 0, c: 0 } },
        struct_test { name: b"relocate .rodata reference\0".as_ptr() as *const c_char, key: 2, val: foo { a: 0, b: 0, c: 0 } },
        struct_test { name: b"relocate .data reference\0".as_ptr() as *const c_char, key: 3, val: foo { a: 41, b: 0xeeeeefef, c: 0x2111111111111111 } },
    ];

    let mut i = 0usize;
    while i < tests.len() {
        let err = bpf_map_lookup_elem(
            map_fd,
            &tests[i].key as *const _ as *const c_void,
            &mut val as *mut _ as *mut c_void,
        );
        CHECK(
            err != 0
                || memcmp(
                    &val as *const _ as *const c_void,
                    &tests[i].val as *const _ as *const c_void,
                    core::mem::size_of_val(&val),
                ) != 0,
            tests[i].name,
            b"err %d result { %u, %u, %llu } expected { %u, %u, %llu }\n\0".as_ptr() as *const c_char,
            err,
            val.a as c_uint,
            val.b as c_uint,
            val.c,
            tests[i].val.a as c_uint,
            tests[i].val.b as c_uint,
            tests[i].val.c,
        );
        i += 1;
    }
}

unsafe fn test_global_data_rdonly(obj: *mut bpf_object, _duration: __u32) {
    let mut err: c_int = -ENOMEM;
    let zero: c_int = 0;

    let map = bpf_object__find_map_by_name(obj, b"test_glo.rodata\0".as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(map, b"map\0".as_ptr() as *const c_char) {
        return;
    }
    if !ASSERT_TRUE(bpf_map__is_internal(map), b"is_internal\0".as_ptr() as *const c_char) {
        return;
    }

    /* ensure we can lookup internal maps by their ELF names */
    let map2 = bpf_object__find_map_by_name(obj, b".rodata\0".as_ptr() as *const c_char);
    if !ASSERT_EQ(map, map2, b"same_maps\0".as_ptr() as *const c_char) {
        return;
    }

    let map_fd = bpf_map__fd(map);
    if CHECK_FAIL(map_fd < 0) {
        return;
    }

    let buff = malloc(bpf_map__value_size(map)) as *mut __u8;
    if !buff.is_null() {
        err = bpf_map_update_elem(
            map_fd,
            &zero as *const _ as *const c_void,
            buff as *const c_void,
            0,
        );
    }
    free(buff as *mut c_void);
    CHECK(
        err == 0 || errno != EPERM,
        b"test .rodata read-only map\0".as_ptr() as *const c_char,
        b"err %d errno %d\n\0".as_ptr() as *const c_char,
        err,
        errno,
    );
}

#[no_mangle]
pub unsafe extern "C" fn test_global_data() {
    let file = b"./test_global_data.bpf.o\0".as_ptr() as *const c_char;
    let mut obj: *mut bpf_object = core::ptr::null_mut();
    let mut prog_fd: c_int = 0;
    let mut topts = bpf_test_run_opts {
        data_in: &mut pkt_v4 as *mut _ as *mut c_void,
        data_size_in: core::mem::size_of_val(&pkt_v4) as u32,
        repeat: 1,
        retval: 0,
        duration: 0,
    };

    let mut err = bpf_prog_test_load(
        file,
        BPF_PROG_TYPE_SCHED_CLS,
        &mut obj,
        &mut prog_fd,
    );
    if !ASSERT_OK(err, b"load program\0".as_ptr() as *const c_char) {
        return;
    }

    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, b"pass global data run err\0".as_ptr() as *const c_char);
    ASSERT_OK(topts.retval as c_int, b"pass global data run retval\0".as_ptr() as *const c_char);

    test_global_data_number(obj, topts.duration);
    test_global_data_string(obj, topts.duration);
    test_global_data_struct(obj, topts.duration);
    test_global_data_rdonly(obj, topts.duration);

    bpf_object__close(obj);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
