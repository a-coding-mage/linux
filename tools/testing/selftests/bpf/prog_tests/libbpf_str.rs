// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

use core::ffi::{c_char, c_int, c_uint, c_void};

/* C dependencies from:
 * #include <ctype.h>
 * #include <test_progs.h>
 * #include <bpf/btf.h>
 */

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_type {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_enum {
    pub name_off: c_uint,
    pub val: c_int,
}

pub type bpf_attach_type = c_uint;
pub type bpf_link_type = c_uint;
pub type bpf_map_type = c_uint;
pub type bpf_prog_type = c_uint;

unsafe extern "C" {
    fn toupper(c: c_int) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    fn btf__parse(path: *const c_char, opts: *const c_void) -> *mut btf;
    fn btf__find_by_name_kind(btf: *const btf, name: *const c_char, kind: c_uint) -> c_int;
    fn btf__type_by_id(btf: *const btf, id: c_uint) -> *const btf_type;
    fn btf_enum(t: *const btf_type) -> *const btf_enum;
    fn btf_vlen(t: *const btf_type) -> c_int;
    fn btf__str_by_offset(btf: *const btf, offset: c_uint) -> *const c_char;
    fn btf__free(btf: *mut btf);

    fn libbpf_bpf_attach_type_str(t: bpf_attach_type) -> *const c_char;
    fn libbpf_bpf_link_type_str(t: bpf_link_type) -> *const c_char;
    fn libbpf_bpf_map_type_str(t: bpf_map_type) -> *const c_char;
    fn libbpf_bpf_prog_type_str(t: bpf_prog_type) -> *const c_char;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_GT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_STREQ(actual: *const c_char, expected: *const c_char, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;

    static __MAX_BPF_ATTACH_TYPE: bpf_attach_type;
    static __MAX_BPF_LINK_TYPE: bpf_link_type;
    static __MAX_BPF_MAP_TYPE: bpf_map_type;
    static __MAX_BPF_PROG_TYPE: bpf_prog_type;
}

const BTF_KIND_ENUM: c_uint = 6;

/*
 * Utility function uppercasing an entire string.
 */
unsafe fn uppercase(mut s: *mut c_char) {
    unsafe {
        while *s != b'\0' as c_char {
            *s = toupper(*s as c_int) as c_char;
            s = s.add(1);
        }
    }
}

/*
 * Test case to check that all bpf_attach_type variants are covered by
 * libbpf_bpf_attach_type_str.
 */
unsafe fn test_libbpf_bpf_attach_type_str() {
    let btf: *mut btf;
    let t: *const btf_type;
    let mut e: *const btf_enum;
    let mut i: c_int;
    let n: c_int;
    let id: c_int;

    unsafe {
        btf = btf__parse(c"/sys/kernel/btf/vmlinux".as_ptr(), core::ptr::null());
        if !ASSERT_OK_PTR(btf as *const c_void, c"btf_parse".as_ptr()) {
            return;
        }

        /* find enum bpf_attach_type and enumerate each value */
        id = btf__find_by_name_kind(btf, c"bpf_attach_type".as_ptr(), BTF_KIND_ENUM);
        if !ASSERT_GT(id, 0, c"bpf_attach_type_id".as_ptr()) {
            btf__free(btf);
            return;
        }
        t = btf__type_by_id(btf, id as c_uint);
        e = btf_enum(t);
        n = btf_vlen(t);
        i = 0;
        while i < n {
            let attach_type: bpf_attach_type = (*e).val as bpf_attach_type;
            let attach_type_name: *const c_char;
            let attach_type_str: *const c_char;
            let mut buf = [0 as c_char; 256];

            if attach_type != __MAX_BPF_ATTACH_TYPE {
                attach_type_name = btf__str_by_offset(btf, (*e).name_off);
                attach_type_str = libbpf_bpf_attach_type_str(attach_type);
                ASSERT_OK_PTR(attach_type_str as *const c_void, attach_type_name);

                snprintf(
                    buf.as_mut_ptr(),
                    buf.len(),
                    c"BPF_%s".as_ptr(),
                    attach_type_str,
                );
                uppercase(buf.as_mut_ptr());

                ASSERT_STREQ(buf.as_ptr(), attach_type_name, c"exp_str_value".as_ptr());
            }

            e = e.add(1);
            i += 1;
        }

        btf__free(btf);
    }
}

/*
 * Test case to check that all bpf_link_type variants are covered by
 * libbpf_bpf_link_type_str.
 */
unsafe fn test_libbpf_bpf_link_type_str() {
    let btf: *mut btf;
    let t: *const btf_type;
    let mut e: *const btf_enum;
    let mut i: c_int;
    let n: c_int;
    let id: c_int;

    unsafe {
        btf = btf__parse(c"/sys/kernel/btf/vmlinux".as_ptr(), core::ptr::null());
        if !ASSERT_OK_PTR(btf as *const c_void, c"btf_parse".as_ptr()) {
            return;
        }

        /* find enum bpf_link_type and enumerate each value */
        id = btf__find_by_name_kind(btf, c"bpf_link_type".as_ptr(), BTF_KIND_ENUM);
        if !ASSERT_GT(id, 0, c"bpf_link_type_id".as_ptr()) {
            btf__free(btf);
            return;
        }
        t = btf__type_by_id(btf, id as c_uint);
        e = btf_enum(t);
        n = btf_vlen(t);
        i = 0;
        while i < n {
            let link_type: bpf_link_type = (*e).val as bpf_link_type;
            let link_type_name: *const c_char;
            let link_type_str: *const c_char;
            let mut buf = [0 as c_char; 256];

            if link_type != __MAX_BPF_LINK_TYPE {
                link_type_name = btf__str_by_offset(btf, (*e).name_off);
                link_type_str = libbpf_bpf_link_type_str(link_type);
                ASSERT_OK_PTR(link_type_str as *const c_void, link_type_name);

                snprintf(
                    buf.as_mut_ptr(),
                    buf.len(),
                    c"BPF_LINK_TYPE_%s".as_ptr(),
                    link_type_str,
                );
                uppercase(buf.as_mut_ptr());

                ASSERT_STREQ(buf.as_ptr(), link_type_name, c"exp_str_value".as_ptr());
            }

            e = e.add(1);
            i += 1;
        }

        btf__free(btf);
    }
}

/*
 * Test case to check that all bpf_map_type variants are covered by
 * libbpf_bpf_map_type_str.
 */
unsafe fn test_libbpf_bpf_map_type_str() {
    let btf: *mut btf;
    let t: *const btf_type;
    let mut e: *const btf_enum;
    let mut i: c_int;
    let n: c_int;
    let id: c_int;

    unsafe {
        btf = btf__parse(c"/sys/kernel/btf/vmlinux".as_ptr(), core::ptr::null());
        if !ASSERT_OK_PTR(btf as *const c_void, c"btf_parse".as_ptr()) {
            return;
        }

        /* find enum bpf_map_type and enumerate each value */
        id = btf__find_by_name_kind(btf, c"bpf_map_type".as_ptr(), BTF_KIND_ENUM);
        if !ASSERT_GT(id, 0, c"bpf_map_type_id".as_ptr()) {
            btf__free(btf);
            return;
        }
        t = btf__type_by_id(btf, id as c_uint);
        e = btf_enum(t);
        n = btf_vlen(t);
        i = 0;
        while i < n {
            let map_type: bpf_map_type = (*e).val as bpf_map_type;
            let map_type_name: *const c_char;
            let map_type_str: *const c_char;
            let mut buf = [0 as c_char; 256];

            if map_type != __MAX_BPF_MAP_TYPE {
                map_type_name = btf__str_by_offset(btf, (*e).name_off);
                map_type_str = libbpf_bpf_map_type_str(map_type);
                ASSERT_OK_PTR(map_type_str as *const c_void, map_type_name);

                snprintf(
                    buf.as_mut_ptr(),
                    buf.len(),
                    c"BPF_MAP_TYPE_%s".as_ptr(),
                    map_type_str,
                );
                uppercase(buf.as_mut_ptr());

                /* Special case for map_type_name BPF_MAP_TYPE_CGROUP_STORAGE_DEPRECATED
                 * where it and BPF_MAP_TYPE_CGROUP_STORAGE have the same enum value
                 * (map_type). For this enum value, libbpf_bpf_map_type_str() picks
                 * BPF_MAP_TYPE_CGROUP_STORAGE. The same for
                 * BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE_DEPRECATED and
                 * BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE.
                 */
                if strcmp(
                    map_type_name,
                    c"BPF_MAP_TYPE_CGROUP_STORAGE_DEPRECATED".as_ptr(),
                ) == 0
                {
                    e = e.add(1);
                    i += 1;
                    continue;
                }
                if strcmp(
                    map_type_name,
                    c"BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE_DEPRECATED".as_ptr(),
                ) == 0
                {
                    e = e.add(1);
                    i += 1;
                    continue;
                }

                ASSERT_STREQ(buf.as_ptr(), map_type_name, c"exp_str_value".as_ptr());
            }

            e = e.add(1);
            i += 1;
        }

        btf__free(btf);
    }
}

/*
 * Test case to check that all bpf_prog_type variants are covered by
 * libbpf_bpf_prog_type_str.
 */
unsafe fn test_libbpf_bpf_prog_type_str() {
    let btf: *mut btf;
    let t: *const btf_type;
    let mut e: *const btf_enum;
    let mut i: c_int;
    let n: c_int;
    let id: c_int;

    unsafe {
        btf = btf__parse(c"/sys/kernel/btf/vmlinux".as_ptr(), core::ptr::null());
        if !ASSERT_OK_PTR(btf as *const c_void, c"btf_parse".as_ptr()) {
            return;
        }

        /* find enum bpf_prog_type and enumerate each value */
        id = btf__find_by_name_kind(btf, c"bpf_prog_type".as_ptr(), BTF_KIND_ENUM);
        if !ASSERT_GT(id, 0, c"bpf_prog_type_id".as_ptr()) {
            btf__free(btf);
            return;
        }
        t = btf__type_by_id(btf, id as c_uint);
        e = btf_enum(t);
        n = btf_vlen(t);
        i = 0;
        while i < n {
            let prog_type: bpf_prog_type = (*e).val as bpf_prog_type;
            let prog_type_name: *const c_char;
            let prog_type_str: *const c_char;
            let mut buf = [0 as c_char; 256];

            if prog_type != __MAX_BPF_PROG_TYPE {
                prog_type_name = btf__str_by_offset(btf, (*e).name_off);
                prog_type_str = libbpf_bpf_prog_type_str(prog_type);
                ASSERT_OK_PTR(prog_type_str as *const c_void, prog_type_name);

                snprintf(
                    buf.as_mut_ptr(),
                    buf.len(),
                    c"BPF_PROG_TYPE_%s".as_ptr(),
                    prog_type_str,
                );
                uppercase(buf.as_mut_ptr());

                ASSERT_STREQ(buf.as_ptr(), prog_type_name, c"exp_str_value".as_ptr());
            }

            e = e.add(1);
            i += 1;
        }

        btf__free(btf);
    }
}

/*
 * Run all libbpf str conversion tests.
 */
#[no_mangle]
pub unsafe extern "C" fn test_libbpf_str() {
    unsafe {
        if test__start_subtest(c"bpf_attach_type_str".as_ptr()) {
            test_libbpf_bpf_attach_type_str();
        }

        if test__start_subtest(c"bpf_link_type_str".as_ptr()) {
            test_libbpf_bpf_link_type_str();
        }

        if test__start_subtest(c"bpf_map_type_str".as_ptr()) {
            test_libbpf_bpf_map_type_str();
        }

        if test__start_subtest(c"bpf_prog_type_str".as_ptr()) {
            test_libbpf_bpf_prog_type_str();
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
