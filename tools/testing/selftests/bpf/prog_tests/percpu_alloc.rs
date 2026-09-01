// SPDX-License-Identifier: GPL-2.0
// C dependencies translated as external declarations:
// <test_progs.h>, "cgroup_helpers.h", and generated *.skel.h headers.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};

type size_t = usize;
type u32 = u32;
type u64 = u64;

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
struct percpu_alloc_array_progs {
    test_array_map_1: *mut bpf_program,
    test_array_map_2: *mut bpf_program,
    test_array_map_3: *mut bpf_program,
    test_array_map_4: *mut bpf_program,
    test_array_map_10: *mut bpf_program,
    cgroup_egress: *mut bpf_program,
}

#[repr(C)]
struct percpu_alloc_array_maps {
    percpu: *mut bpf_map,
    percpu_cgroup_storage: *mut bpf_map,
}

#[repr(C)]
struct percpu_alloc_array_bss {
    my_pid: c_int,
    cpu0_field_d: u32,
    sum_field_c: u32,
}

#[repr(C)]
struct percpu_alloc_array_rodata {
    nr_cpus: c_int,
}

#[repr(C)]
struct percpu_alloc_array {
    progs: percpu_alloc_array_progs,
    maps: percpu_alloc_array_maps,
    bss: *mut percpu_alloc_array_bss,
    rodata: *mut percpu_alloc_array_rodata,
}

#[repr(C)]
struct percpu_alloc_cgrp_local_storage_progs {
    test_cgrp_local_storage_1: *mut bpf_program,
}

#[repr(C)]
struct percpu_alloc_cgrp_local_storage_bss {
    my_pid: c_int,
    cpu0_field_d: u32,
    sum_field_c: u32,
}

#[repr(C)]
struct percpu_alloc_cgrp_local_storage_rodata {
    nr_cpus: c_int,
}

#[repr(C)]
struct percpu_alloc_cgrp_local_storage {
    progs: percpu_alloc_cgrp_local_storage_progs,
    bss: *mut percpu_alloc_cgrp_local_storage_bss,
    rodata: *mut percpu_alloc_cgrp_local_storage_rodata,
}

#[repr(C)]
struct bpf_test_run_opts {
    _private: [u8; 0],
    retval: u32,
}

#[repr(C)]
struct bpf_map_batch_opts {
    elem_flags: u64,
}

#[repr(C)]
struct bpf_cgroup_storage_key {
    _private: [u8; 0],
}

type bpf_map_type = c_uint;

const BPF_F_CPU: u64 = 1;
const BPF_F_ALL_CPUS: u64 = 2;
const BPF_F_LOCK: u64 = 4;
const BPF_MAP_TYPE_PERCPU_ARRAY: bpf_map_type = 0;
const BPF_MAP_TYPE_PERCPU_HASH: bpf_map_type = 1;
const BPF_MAP_TYPE_LRU_PERCPU_HASH: bpf_map_type = 2;
const BPF_MAP_TYPE_ARRAY: bpf_map_type = 3;
const BPF_MAP_TYPE_HASH: bpf_map_type = 4;
const BPF_CGROUP_INET_EGRESS: c_int = 1;
const ERANGE: c_int = 34;
const ENOENT: c_int = 2;

unsafe extern "C" {
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn close(fd: c_int) -> c_int;
    fn getpid() -> c_int;

    fn libbpf_num_possible_cpus() -> c_int;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map__set_type(map: *mut bpf_map, map_type: bpf_map_type);
    fn bpf_map__set_max_entries(map: *mut bpf_map, max_entries: u32);
    fn bpf_map_create(
        map_type: bpf_map_type,
        name: *const c_char,
        key_size: c_uint,
        value_size: c_uint,
        max_entries: c_uint,
        opts: *const c_void,
    ) -> c_int;
    fn bpf_map_lookup_elem_flags(
        fd: c_int,
        key: *const c_void,
        value: *mut c_void,
        flags: u64,
    ) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64)
        -> c_int;
    fn bpf_map__lookup_elem(
        map: *mut bpf_map,
        key: *const c_void,
        key_sz: size_t,
        value: *mut c_void,
        value_sz: size_t,
        flags: u64,
    ) -> c_int;
    fn bpf_map__update_elem(
        map: *mut bpf_map,
        key: *const c_void,
        key_sz: size_t,
        value: *const c_void,
        value_sz: size_t,
        flags: u64,
    ) -> c_int;
    fn bpf_map_update_batch(
        fd: c_int,
        keys: *const c_void,
        values: *const c_void,
        count: *mut u32,
        opts: *mut bpf_map_batch_opts,
    ) -> c_int;
    fn bpf_map_lookup_batch(
        fd: c_int,
        in_batch: *mut c_void,
        out_batch: *mut u64,
        keys: *mut c_void,
        values: *mut c_void,
        count: *mut u32,
        opts: *mut bpf_map_batch_opts,
    ) -> c_int;
    fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> c_int;
    fn bpf_prog_attach(prog_fd: c_int, target_fd: c_int, attach_type: c_int, flags: c_uint)
        -> c_int;
    fn bpf_prog_detach2(prog_fd: c_int, target_fd: c_int, attach_type: c_int) -> c_int;

    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn setup_cgroup_environment() -> c_int;
    fn cleanup_cgroup_environment();
    fn create_and_get_cgroup(path: *const c_char) -> c_int;
    fn join_cgroup(path: *const c_char) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;

    fn percpu_alloc_array__open() -> *mut percpu_alloc_array;
    fn percpu_alloc_array__open_and_load() -> *mut percpu_alloc_array;
    fn percpu_alloc_array__load(skel: *mut percpu_alloc_array) -> c_int;
    fn percpu_alloc_array__attach(skel: *mut percpu_alloc_array) -> c_int;
    fn percpu_alloc_array__destroy(skel: *mut percpu_alloc_array);
    fn percpu_alloc_cgrp_local_storage__open() -> *mut percpu_alloc_cgrp_local_storage;
    fn percpu_alloc_cgrp_local_storage__load(skel: *mut percpu_alloc_cgrp_local_storage)
        -> c_int;
    fn percpu_alloc_cgrp_local_storage__attach(skel: *mut percpu_alloc_cgrp_local_storage)
        -> c_int;
    fn percpu_alloc_cgrp_local_storage__destroy(skel: *mut percpu_alloc_cgrp_local_storage);

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_ulonglong, expected: c_ulonglong, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_TRUE(cond: bool, name: *const c_char) -> bool;
    fn RUN_TESTS_percpu_alloc_fail();
}

const fn roundup(x: size_t, y: size_t) -> size_t {
    ((x + y - 1) / y) * y
}

unsafe fn test_array() {
    let mut err: c_int;
    let prog_fd: c_int;
    let mut topts: bpf_test_run_opts = core::mem::zeroed();

    let skel = percpu_alloc_array__open();
    if !ASSERT_OK_PTR(skel as *const c_void, b"percpu_alloc_array__open\0".as_ptr() as *const c_char) {
        return;
    }

    bpf_program__set_autoload((*skel).progs.test_array_map_1, true);
    bpf_program__set_autoload((*skel).progs.test_array_map_2, true);
    bpf_program__set_autoload((*skel).progs.test_array_map_3, true);
    bpf_program__set_autoload((*skel).progs.test_array_map_4, true);

    (*(*skel).bss).my_pid = getpid();
    (*(*skel).rodata).nr_cpus = libbpf_num_possible_cpus();

    err = percpu_alloc_array__load(skel);
    if !ASSERT_OK(err, b"percpu_alloc_array__load\0".as_ptr() as *const c_char) {
        percpu_alloc_array__destroy(skel);
        return;
    }

    err = percpu_alloc_array__attach(skel);
    if !ASSERT_OK(err, b"percpu_alloc_array__attach\0".as_ptr() as *const c_char) {
        percpu_alloc_array__destroy(skel);
        return;
    }

    prog_fd = bpf_program__fd((*skel).progs.test_array_map_1);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, b"test_run array_map 1-4\0".as_ptr() as *const c_char);
    ASSERT_EQ(topts.retval as c_ulonglong, 0, b"test_run array_map 1-4\0".as_ptr() as *const c_char);
    ASSERT_EQ((*(*skel).bss).cpu0_field_d as c_ulonglong, 2, b"cpu0_field_d\0".as_ptr() as *const c_char);
    ASSERT_EQ((*(*skel).bss).sum_field_c as c_ulonglong, 1, b"sum_field_c\0".as_ptr() as *const c_char);
    percpu_alloc_array__destroy(skel);
}

unsafe fn test_array_sleepable() {
    let mut err: c_int;
    let prog_fd: c_int;
    let mut topts: bpf_test_run_opts = core::mem::zeroed();

    let skel = percpu_alloc_array__open();
    if !ASSERT_OK_PTR(skel as *const c_void, b"percpu_alloc__open\0".as_ptr() as *const c_char) {
        return;
    }

    bpf_program__set_autoload((*skel).progs.test_array_map_10, true);

    (*(*skel).bss).my_pid = getpid();
    (*(*skel).rodata).nr_cpus = libbpf_num_possible_cpus();

    err = percpu_alloc_array__load(skel);
    if !ASSERT_OK(err, b"percpu_alloc_array__load\0".as_ptr() as *const c_char) {
        percpu_alloc_array__destroy(skel);
        return;
    }

    err = percpu_alloc_array__attach(skel);
    if !ASSERT_OK(err, b"percpu_alloc_array__attach\0".as_ptr() as *const c_char) {
        percpu_alloc_array__destroy(skel);
        return;
    }

    prog_fd = bpf_program__fd((*skel).progs.test_array_map_10);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, b"test_run array_map_10\0".as_ptr() as *const c_char);
    ASSERT_EQ(topts.retval as c_ulonglong, 0, b"test_run array_map_10\0".as_ptr() as *const c_char);
    ASSERT_EQ((*(*skel).bss).cpu0_field_d as c_ulonglong, 2, b"cpu0_field_d\0".as_ptr() as *const c_char);
    ASSERT_EQ((*(*skel).bss).sum_field_c as c_ulonglong, 1, b"sum_field_c\0".as_ptr() as *const c_char);
    percpu_alloc_array__destroy(skel);
}

unsafe fn test_cgrp_local_storage() {
    let mut err: c_int;
    let cgroup_fd: c_int;
    let prog_fd: c_int;
    let mut topts: bpf_test_run_opts = core::mem::zeroed();

    cgroup_fd = test__join_cgroup(b"/percpu_alloc\0".as_ptr() as *const c_char);
    if !ASSERT_GE(cgroup_fd, 0, b"join_cgroup /percpu_alloc\0".as_ptr() as *const c_char) {
        return;
    }

    let skel = percpu_alloc_cgrp_local_storage__open();
    if !ASSERT_OK_PTR(skel as *const c_void, b"percpu_alloc_cgrp_local_storage__open\0".as_ptr() as *const c_char) {
        close(cgroup_fd);
        return;
    }

    (*(*skel).bss).my_pid = getpid();
    (*(*skel).rodata).nr_cpus = libbpf_num_possible_cpus();

    err = percpu_alloc_cgrp_local_storage__load(skel);
    if ASSERT_OK(err, b"percpu_alloc_cgrp_local_storage__load\0".as_ptr() as *const c_char) {
        err = percpu_alloc_cgrp_local_storage__attach(skel);
        if ASSERT_OK(err, b"percpu_alloc_cgrp_local_storage__attach\0".as_ptr() as *const c_char) {
            prog_fd = bpf_program__fd((*skel).progs.test_cgrp_local_storage_1);
            err = bpf_prog_test_run_opts(prog_fd, &mut topts);
            ASSERT_OK(err, b"test_run cgrp_local_storage 1-3\0".as_ptr() as *const c_char);
            ASSERT_EQ(topts.retval as c_ulonglong, 0, b"test_run cgrp_local_storage 1-3\0".as_ptr() as *const c_char);
            ASSERT_EQ((*(*skel).bss).cpu0_field_d as c_ulonglong, 2, b"cpu0_field_d\0".as_ptr() as *const c_char);
            ASSERT_EQ((*(*skel).bss).sum_field_c as c_ulonglong, 1, b"sum_field_c\0".as_ptr() as *const c_char);
        }
    }

    percpu_alloc_cgrp_local_storage__destroy(skel);
    close(cgroup_fd);
}

unsafe fn test_failure() {
    RUN_TESTS_percpu_alloc_fail();
}

unsafe fn test_percpu_map_op_cpu_flag(
    map: *mut bpf_map,
    keys: *mut c_void,
    key_sz: size_t,
    entries: u32,
    nr_cpus: c_int,
    test_batch: bool,
) {
    let value_sz: size_t = core::mem::size_of::<u32>();
    let mut value_sz_cpus: size_t;
    let mut value_sz_total: size_t;
    let mut values: *mut u32 = core::ptr::null_mut();
    let mut values_percpu: *mut u32 = core::ptr::null_mut();
    let value: u32 = 0xDEADC0DE;
    let mut i: c_int;
    let mut j: c_int;
    let mut cpu: c_int;
    let map_fd: c_int;
    let mut err: c_int;
    let mut batch: u64 = 0;
    let mut flags: u64;
    let mut values_row: *mut c_void;
    let mut count: u32;
    let mut v: u32;
    let mut batch_opts: bpf_map_batch_opts = core::mem::zeroed();

    value_sz_cpus = value_sz * nr_cpus as size_t;
    values = calloc(entries as size_t, value_sz_cpus) as *mut u32;
    if !ASSERT_OK_PTR(values as *const c_void, b"calloc values\0".as_ptr() as *const c_char) {
        return;
    }

    values_percpu = calloc(entries as size_t, roundup(value_sz, 8) * nr_cpus as size_t) as *mut u32;
    if !ASSERT_OK_PTR(values_percpu as *const c_void, b"calloc values_percpu\0".as_ptr() as *const c_char) {
        free(values as *mut c_void);
        return;
    }

    value_sz_total = value_sz_cpus * entries as size_t;
    memset(values as *mut c_void, 0, value_sz_total);

    map_fd = bpf_map__fd(map);
    flags = BPF_F_CPU | BPF_F_ALL_CPUS;
    err = bpf_map_lookup_elem_flags(map_fd, keys, values as *mut c_void, flags);
    if !ASSERT_ERR(err, b"bpf_map_lookup_elem_flags cpu|all_cpus\0".as_ptr() as *const c_char) {
        free(values_percpu as *mut c_void);
        free(values as *mut c_void);
        return;
    }

    err = bpf_map_update_elem(map_fd, keys, values as *const c_void, flags);
    if !ASSERT_ERR(err, b"bpf_map_update_elem cpu|all_cpus\0".as_ptr() as *const c_char) {
        free(values_percpu as *mut c_void);
        free(values as *mut c_void);
        return;
    }

    flags = BPF_F_ALL_CPUS;
    err = bpf_map_lookup_elem_flags(map_fd, keys, values as *mut c_void, flags);
    if !ASSERT_ERR(err, b"bpf_map_lookup_elem_flags all_cpus\0".as_ptr() as *const c_char) {
        free(values_percpu as *mut c_void);
        free(values as *mut c_void);
        return;
    }

    flags = BPF_F_LOCK | BPF_F_CPU;
    err = bpf_map_lookup_elem_flags(map_fd, keys, values as *mut c_void, flags);
    if !ASSERT_ERR(err, b"bpf_map_lookup_elem_flags BPF_F_LOCK\0".as_ptr() as *const c_char) {
        free(values_percpu as *mut c_void);
        free(values as *mut c_void);
        return;
    }

    flags = BPF_F_LOCK | BPF_F_ALL_CPUS;
    err = bpf_map_update_elem(map_fd, keys, values as *const c_void, flags);
    if !ASSERT_ERR(err, b"bpf_map_update_elem BPF_F_LOCK\0".as_ptr() as *const c_char) {
        free(values_percpu as *mut c_void);
        free(values as *mut c_void);
        return;
    }

    flags = ((nr_cpus as u64) << 32) | BPF_F_CPU;
    err = bpf_map_update_elem(map_fd, keys, values as *const c_void, flags);
    if !ASSERT_EQ(err as c_ulonglong, (-ERANGE) as c_ulonglong, b"bpf_map_update_elem -ERANGE\0".as_ptr() as *const c_char) {
        free(values_percpu as *mut c_void);
        free(values as *mut c_void);
        return;
    }

    err = bpf_map__update_elem(map, keys, key_sz, values as *const c_void, value_sz, flags);
    if !ASSERT_EQ(err as c_ulonglong, (-ERANGE) as c_ulonglong, b"bpf_map__update_elem -ERANGE\0".as_ptr() as *const c_char) {
        free(values_percpu as *mut c_void);
        free(values as *mut c_void);
        return;
    }

    err = bpf_map_lookup_elem_flags(map_fd, keys, values as *mut c_void, flags);
    if !ASSERT_EQ(err as c_ulonglong, (-ERANGE) as c_ulonglong, b"bpf_map_lookup_elem_flags -ERANGE\0".as_ptr() as *const c_char) {
        free(values_percpu as *mut c_void);
        free(values as *mut c_void);
        return;
    }

    err = bpf_map__lookup_elem(map, keys, key_sz, values as *mut c_void, value_sz, flags);
    if !ASSERT_EQ(err as c_ulonglong, (-ERANGE) as c_ulonglong, b"bpf_map__lookup_elem -ERANGE\0".as_ptr() as *const c_char) {
        free(values_percpu as *mut c_void);
        free(values as *mut c_void);
        return;
    }

    cpu = 0;
    while cpu < nr_cpus {
        /* clear value on all cpus */
        *values.add(0) = 0;
        flags = BPF_F_ALL_CPUS;
        i = 0;
        while i < entries as c_int {
            err = bpf_map__update_elem(
                map,
                (keys as *mut u8).add(i as size_t * key_sz) as *const c_void,
                key_sz,
                values as *const c_void,
                value_sz,
                flags,
            );
            if !ASSERT_OK(err, b"bpf_map__update_elem all_cpus\0".as_ptr() as *const c_char) {
                free(values_percpu as *mut c_void);
                free(values as *mut c_void);
                return;
            }
            i += 1;
        }

        /* update value on specified cpu */
        i = 0;
        while i < entries as c_int {
            *values.add(0) = value;
            flags = ((cpu as u64) << 32) | BPF_F_CPU;
            err = bpf_map__update_elem(
                map,
                (keys as *mut u8).add(i as size_t * key_sz) as *const c_void,
                key_sz,
                values as *const c_void,
                value_sz,
                flags,
            );
            if !ASSERT_OK(err, b"bpf_map__update_elem specified cpu\0".as_ptr() as *const c_char) {
                free(values_percpu as *mut c_void);
                free(values as *mut c_void);
                return;
            }

            /* lookup then check value on CPUs */
            j = 0;
            while j < nr_cpus {
                flags = ((j as u64) << 32) | BPF_F_CPU;
                err = bpf_map__lookup_elem(
                    map,
                    (keys as *mut u8).add(i as size_t * key_sz) as *const c_void,
                    key_sz,
                    values as *mut c_void,
                    value_sz,
                    flags,
                );
                if !ASSERT_OK(err, b"bpf_map__lookup_elem specified cpu\0".as_ptr() as *const c_char) {
                    free(values_percpu as *mut c_void);
                    free(values as *mut c_void);
                    return;
                }
                if !ASSERT_EQ(
                    *values.add(0) as c_ulonglong,
                    (if j != cpu { 0 } else { value }) as c_ulonglong,
                    b"bpf_map__lookup_elem value on specified cpu\0".as_ptr() as *const c_char,
                ) {
                    free(values_percpu as *mut c_void);
                    free(values as *mut c_void);
                    return;
                }
                j += 1;
            }
            i += 1;
        }
        cpu += 1;
    }

    if !test_batch {
        free(values_percpu as *mut c_void);
        free(values as *mut c_void);
        return;
    }

    count = entries;
    batch_opts.elem_flags = ((nr_cpus as u64) << 32) | BPF_F_CPU;
    err = bpf_map_update_batch(map_fd, keys, values as *const c_void, &mut count, &mut batch_opts);
    if !ASSERT_EQ(err as c_ulonglong, (-ERANGE) as c_ulonglong, b"bpf_map_update_batch -ERANGE\0".as_ptr() as *const c_char) {
        free(values_percpu as *mut c_void);
        free(values as *mut c_void);
        return;
    }

    cpu = 0;
    while cpu < nr_cpus {
        memset(values as *mut c_void, 0, value_sz_total);

        /* clear values across all CPUs */
        count = entries;
        batch_opts.elem_flags = BPF_F_ALL_CPUS;
        err = bpf_map_update_batch(map_fd, keys, values as *const c_void, &mut count, &mut batch_opts);
        if !ASSERT_OK(err, b"bpf_map_update_batch all_cpus\0".as_ptr() as *const c_char) {
            break;
        }
        if !ASSERT_EQ(count as c_ulonglong, entries as c_ulonglong, b"bpf_map_update_batch count\0".as_ptr() as *const c_char) {
            break;
        }

        /* update values on specified CPU */
        i = 0;
        while i < entries as c_int {
            *values.add(i as size_t) = value;
            i += 1;
        }

        count = entries;
        batch_opts.elem_flags = ((cpu as u64) << 32) | BPF_F_CPU;
        err = bpf_map_update_batch(map_fd, keys, values as *const c_void, &mut count, &mut batch_opts);
        if !ASSERT_OK(err, b"bpf_map_update_batch specified cpu\0".as_ptr() as *const c_char) {
            break;
        }
        if !ASSERT_EQ(count as c_ulonglong, entries as c_ulonglong, b"bpf_map_update_batch count\0".as_ptr() as *const c_char) {
            break;
        }

        /* lookup values on specified CPU */
        batch = 0;
        count = entries;
        memset(values as *mut c_void, 0, entries as size_t * value_sz);
        err = bpf_map_lookup_batch(map_fd, core::ptr::null_mut(), &mut batch, keys, values as *mut c_void, &mut count, &mut batch_opts);
        if !ASSERT_TRUE(err == 0 || err == -ENOENT, b"bpf_map_lookup_batch specified cpu\0".as_ptr() as *const c_char) {
            break;
        }
        if !ASSERT_EQ(count as c_ulonglong, entries as c_ulonglong, b"bpf_map_lookup_batch count\0".as_ptr() as *const c_char) {
            break;
        }

        i = 0;
        while i < entries as c_int {
            if !ASSERT_EQ((*values.add(i as size_t)) as c_ulonglong, value as c_ulonglong, b"bpf_map_lookup_batch value on specified cpu\0".as_ptr() as *const c_char) {
                break;
            }
            i += 1;
        }
        if i < entries as c_int {
            break;
        }

        /* lookup values from all CPUs */
        batch = 0;
        count = entries;
        batch_opts.elem_flags = 0;
        memset(values_percpu as *mut c_void, 0, roundup(value_sz, 8) * nr_cpus as size_t * entries as size_t);
        err = bpf_map_lookup_batch(map_fd, core::ptr::null_mut(), &mut batch, keys, values_percpu as *mut c_void, &mut count, &mut batch_opts);
        if !ASSERT_TRUE(err == 0 || err == -ENOENT, b"bpf_map_lookup_batch all_cpus\0".as_ptr() as *const c_char) {
            break;
        }
        if !ASSERT_EQ(count as c_ulonglong, entries as c_ulonglong, b"bpf_map_lookup_batch count\0".as_ptr() as *const c_char) {
            break;
        }

        i = 0;
        while i < entries as c_int {
            values_row = (values_percpu as *mut u8).add(roundup(value_sz, 8) * i as size_t * nr_cpus as size_t) as *mut c_void;
            j = 0;
            while j < nr_cpus {
                v = *((values_row as *mut u8).add(roundup(value_sz, 8) * j as size_t) as *mut u32);
                if !ASSERT_EQ(
                    v as c_ulonglong,
                    (if j != cpu { 0 } else { value }) as c_ulonglong,
                    b"bpf_map_lookup_batch value all_cpus\0".as_ptr() as *const c_char,
                ) {
                    break;
                }
                j += 1;
            }
            if j < nr_cpus {
                break;
            }
            i += 1;
        }
        if i < entries as c_int {
            break;
        }
        cpu += 1;
    }

    free(values_percpu as *mut c_void);
    free(values as *mut c_void);
}

unsafe fn test_percpu_map_cpu_flag(map_type: bpf_map_type) {
    let key_sz: size_t = core::mem::size_of::<c_int>();
    let keys: *mut c_int;
    let mut nr_cpus: c_int;
    let mut i: c_int;
    let mut err: c_int;
    let map: *mut bpf_map;
    let max_entries: u32;

    nr_cpus = libbpf_num_possible_cpus();
    if !ASSERT_GT(nr_cpus, 0, b"libbpf_num_possible_cpus\0".as_ptr() as *const c_char) {
        return;
    }

    max_entries = (nr_cpus * 2) as u32;
    keys = calloc(max_entries as size_t, key_sz) as *mut c_int;
    if !ASSERT_OK_PTR(keys as *const c_void, b"calloc keys\0".as_ptr() as *const c_char) {
        return;
    }

    i = 0;
    while i < max_entries as c_int {
        *keys.add(i as size_t) = i;
        i += 1;
    }

    let skel = percpu_alloc_array__open();
    if !ASSERT_OK_PTR(skel as *const c_void, b"percpu_alloc_array__open\0".as_ptr() as *const c_char) {
        free(keys as *mut c_void);
        return;
    }

    map = (*skel).maps.percpu;
    bpf_map__set_type(map, map_type);
    bpf_map__set_max_entries(map, max_entries);

    err = percpu_alloc_array__load(skel);
    if ASSERT_OK(err, b"test_percpu_alloc__load\0".as_ptr() as *const c_char) {
        test_percpu_map_op_cpu_flag(map, keys as *mut c_void, key_sz, nr_cpus as u32, nr_cpus, true);
    }
    percpu_alloc_array__destroy(skel);
    free(keys as *mut c_void);
}

unsafe fn test_percpu_array_cpu_flag() {
    test_percpu_map_cpu_flag(BPF_MAP_TYPE_PERCPU_ARRAY);
}

unsafe fn test_percpu_hash_cpu_flag() {
    test_percpu_map_cpu_flag(BPF_MAP_TYPE_PERCPU_HASH);
}

unsafe fn test_lru_percpu_hash_cpu_flag() {
    test_percpu_map_cpu_flag(BPF_MAP_TYPE_LRU_PERCPU_HASH);
}

unsafe fn test_percpu_cgroup_storage_cpu_flag() {
    let mut skel: *mut percpu_alloc_array = core::ptr::null_mut();
    let mut key: bpf_cgroup_storage_key = core::mem::zeroed();
    let cgroup: c_int;
    let prog_fd: c_int;
    let nr_cpus: c_int;
    let mut err: c_int;
    let map: *mut bpf_map;

    nr_cpus = libbpf_num_possible_cpus();
    if !ASSERT_GT(nr_cpus, 0, b"libbpf_num_possible_cpus\0".as_ptr() as *const c_char) {
        return;
    }

    err = setup_cgroup_environment();
    if !ASSERT_OK(err, b"setup_cgroup_environment\0".as_ptr() as *const c_char) {
        return;
    }

    cgroup = create_and_get_cgroup(b"/cg_percpu\0".as_ptr() as *const c_char);
    if !ASSERT_GE(cgroup, 0, b"create_and_get_cgroup\0".as_ptr() as *const c_char) {
        cleanup_cgroup_environment();
        return;
    }

    err = join_cgroup(b"/cg_percpu\0".as_ptr() as *const c_char);
    if ASSERT_OK(err, b"join_cgroup\0".as_ptr() as *const c_char) {
        skel = percpu_alloc_array__open_and_load();
        if ASSERT_OK_PTR(skel as *const c_void, b"percpu_alloc_array__open_and_load\0".as_ptr() as *const c_char) {
            prog_fd = bpf_program__fd((*skel).progs.cgroup_egress);
            err = bpf_prog_attach(prog_fd, cgroup, BPF_CGROUP_INET_EGRESS, 0);
            if ASSERT_OK(err, b"bpf_prog_attach\0".as_ptr() as *const c_char) {
                map = (*skel).maps.percpu_cgroup_storage;
                err = bpf_map_get_next_key(bpf_map__fd(map), core::ptr::null(), &mut key as *mut _ as *mut c_void);
                if ASSERT_OK(err, b"bpf_map_get_next_key\0".as_ptr() as *const c_char) {
                    test_percpu_map_op_cpu_flag(
                        map,
                        &mut key as *mut _ as *mut c_void,
                        core::mem::size_of::<bpf_cgroup_storage_key>(),
                        1,
                        nr_cpus,
                        false,
                    );
                }
            }
        }
    }

    bpf_prog_detach2(-1, cgroup, BPF_CGROUP_INET_EGRESS);
    close(cgroup);
    cleanup_cgroup_environment();
    percpu_alloc_array__destroy(skel);
}

unsafe fn test_map_op_cpu_flag(map_type: bpf_map_type) {
    let max_entries: u32 = 1;
    let mut count: u32 = max_entries;
    let mut flags: u64;
    let mut batch: u64 = 0;
    let mut val: u64 = 0;
    let mut err: c_int;
    let map_fd: c_int;
    let mut key: c_int = 0;
    let mut batch_opts: bpf_map_batch_opts = core::mem::zeroed();

    map_fd = bpf_map_create(
        map_type,
        b"test_cpu_flag\0".as_ptr() as *const c_char,
        core::mem::size_of::<c_int>() as c_uint,
        core::mem::size_of::<u64>() as c_uint,
        max_entries,
        core::ptr::null(),
    );
    if !ASSERT_GE(map_fd, 0, b"bpf_map_create\0".as_ptr() as *const c_char) {
        return;
    }

    flags = BPF_F_ALL_CPUS;
    err = bpf_map_update_elem(map_fd, &key as *const _ as *const c_void, &val as *const _ as *const c_void, flags);
    ASSERT_ERR(err, b"bpf_map_update_elem all_cpus\0".as_ptr() as *const c_char);

    batch_opts.elem_flags = BPF_F_ALL_CPUS;
    err = bpf_map_update_batch(map_fd, &key as *const _ as *const c_void, &val as *const _ as *const c_void, &mut count, &mut batch_opts);
    ASSERT_ERR(err, b"bpf_map_update_batch all_cpus\0".as_ptr() as *const c_char);

    flags = BPF_F_CPU;
    err = bpf_map_lookup_elem_flags(map_fd, &key as *const _ as *const c_void, &mut val as *mut _ as *mut c_void, flags);
    ASSERT_ERR(err, b"bpf_map_lookup_elem_flags cpu\0".as_ptr() as *const c_char);

    batch_opts.elem_flags = BPF_F_CPU;
    err = bpf_map_lookup_batch(
        map_fd,
        core::ptr::null_mut(),
        &mut batch,
        &mut key as *mut _ as *mut c_void,
        &mut val as *mut _ as *mut c_void,
        &mut count,
        &mut batch_opts,
    );
    ASSERT_ERR(err, b"bpf_map_lookup_batch cpu\0".as_ptr() as *const c_char);

    close(map_fd);
}

unsafe fn test_array_cpu_flag() {
    test_map_op_cpu_flag(BPF_MAP_TYPE_ARRAY);
}

unsafe fn test_hash_cpu_flag() {
    test_map_op_cpu_flag(BPF_MAP_TYPE_HASH);
}

#[no_mangle]
pub unsafe extern "C" fn test_percpu_alloc() {
    if test__start_subtest(b"array\0".as_ptr() as *const c_char) {
        test_array();
    }
    if test__start_subtest(b"array_sleepable\0".as_ptr() as *const c_char) {
        test_array_sleepable();
    }
    if test__start_subtest(b"cgrp_local_storage\0".as_ptr() as *const c_char) {
        test_cgrp_local_storage();
    }
    if test__start_subtest(b"failure_tests\0".as_ptr() as *const c_char) {
        test_failure();
    }
    if test__start_subtest(b"cpu_flag_percpu_array\0".as_ptr() as *const c_char) {
        test_percpu_array_cpu_flag();
    }
    if test__start_subtest(b"cpu_flag_percpu_hash\0".as_ptr() as *const c_char) {
        test_percpu_hash_cpu_flag();
    }
    if test__start_subtest(b"cpu_flag_lru_percpu_hash\0".as_ptr() as *const c_char) {
        test_lru_percpu_hash_cpu_flag();
    }
    if test__start_subtest(b"cpu_flag_percpu_cgroup_storage\0".as_ptr() as *const c_char) {
        test_percpu_cgroup_storage_cpu_flag();
    }
    if test__start_subtest(b"cpu_flag_array\0".as_ptr() as *const c_char) {
        test_array_cpu_flag();
    }
    if test__start_subtest(b"cpu_flag_hash\0".as_ptr() as *const c_char) {
        test_hash_cpu_flag();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
