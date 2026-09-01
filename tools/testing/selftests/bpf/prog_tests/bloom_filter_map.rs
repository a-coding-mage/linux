// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// C dependencies: <sys/syscall.h>, <limits.h>, <test_progs.h>,
// "bloom_filter_map.skel.h"

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

const NUMA_NO_NODE: c_int = -1;
const INT32_MAX: c_int = 2147483647;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const SYS_getpgid: c_long = 121;

const BPF_MAP_TYPE_BLOOM_FILTER: c_uint = 30;
const BPF_F_NO_PREALLOC: u64 = 1;
const BPF_F_NUMA_NODE: u64 = 4;
const BPF_F_LOCK: u64 = 4;
const BPF_F_ZERO_SEED: u64 = 64;
const BPF_ANY: u64 = 0;
const BPF_NOEXIST: u64 = 1;
const BPF_EXIST: u64 = 2;

type __u32 = u32;

#[repr(C)]
pub struct bpf_map_create_opts {
    pub sz: usize,
    pub map_flags: u32,
    pub numa_node: u32,
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bloom_filter_map {
    pub maps: bloom_filter_map_maps,
    pub progs: bloom_filter_map_progs,
    pub bss: *mut bloom_filter_map_bss,
}

#[repr(C)]
pub struct bloom_filter_map_maps {
    pub outer_map: *mut bpf_map,
    pub map_random_data: *mut bpf_map,
    pub map_bloom: *mut bpf_map,
}

#[repr(C)]
pub struct bloom_filter_map_progs {
    pub check_bloom: *mut bpf_program,
    pub inner_map: *mut bpf_program,
}

#[repr(C)]
pub struct bloom_filter_map_bss {
    pub error: c_int,
}

unsafe extern "C" {
    fn bpf_map_create(
        map_type: c_uint,
        map_name: *const c_char,
        key_size: c_uint,
        value_size: c_uint,
        max_entries: c_uint,
        opts: *const bpf_map_create_opts,
    ) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64)
        -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map__max_entries(map: *mut bpf_map) -> __u32;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bloom_filter_map__open_and_load() -> *mut bloom_filter_map;
    fn bloom_filter_map__destroy(skel: *mut bloom_filter_map);
    fn ASSERT_LT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn close(fd: c_int) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn rand() -> c_int;
}

unsafe fn test_fail_cases() {
    let mut opts: bpf_map_create_opts = core::mem::zeroed();
    opts.sz = core::mem::size_of::<bpf_map_create_opts>();
    let value: __u32 = 0;
    let mut fd: c_int;
    let mut err: c_int;

    /* Invalid key size */
    fd = bpf_map_create(
        BPF_MAP_TYPE_BLOOM_FILTER,
        core::ptr::null(),
        4,
        core::mem::size_of_val(&value) as c_uint,
        100,
        core::ptr::null(),
    );
    if !ASSERT_LT(
        fd,
        0,
        c"bpf_map_create bloom filter invalid key size".as_ptr(),
    ) {
        close(fd);
    }

    /* Invalid value size */
    fd = bpf_map_create(
        BPF_MAP_TYPE_BLOOM_FILTER,
        core::ptr::null(),
        0,
        0,
        100,
        core::ptr::null(),
    );
    if !ASSERT_LT(
        fd,
        0,
        c"bpf_map_create bloom filter invalid value size 0".as_ptr(),
    ) {
        close(fd);
    }

    /* Invalid value size: too big */
    fd = bpf_map_create(
        BPF_MAP_TYPE_BLOOM_FILTER,
        core::ptr::null(),
        0,
        INT32_MAX as c_uint,
        100,
        core::ptr::null(),
    );
    if !ASSERT_LT(
        fd,
        0,
        c"bpf_map_create bloom filter invalid value too large".as_ptr(),
    ) {
        close(fd);
    }

    /* Invalid max entries size */
    fd = bpf_map_create(
        BPF_MAP_TYPE_BLOOM_FILTER,
        core::ptr::null(),
        0,
        core::mem::size_of_val(&value) as c_uint,
        0,
        core::ptr::null(),
    );
    if !ASSERT_LT(
        fd,
        0,
        c"bpf_map_create bloom filter invalid max entries size".as_ptr(),
    ) {
        close(fd);
    }

    /* Bloom filter maps do not support BPF_F_NO_PREALLOC */
    opts.map_flags = BPF_F_NO_PREALLOC as u32;
    fd = bpf_map_create(
        BPF_MAP_TYPE_BLOOM_FILTER,
        core::ptr::null(),
        0,
        core::mem::size_of_val(&value) as c_uint,
        100,
        &opts,
    );
    if !ASSERT_LT(fd, 0, c"bpf_map_create bloom filter invalid flags".as_ptr()) {
        close(fd);
    }

    fd = bpf_map_create(
        BPF_MAP_TYPE_BLOOM_FILTER,
        core::ptr::null(),
        0,
        core::mem::size_of_val(&value) as c_uint,
        100,
        core::ptr::null(),
    );
    if !ASSERT_GE(fd, 0, c"bpf_map_create bloom filter".as_ptr()) {
        return;
    }

    /* Test invalid flags */
    err = bpf_map_update_elem(fd, core::ptr::null(), &value as *const _ as *const c_void, -1i64 as u64);
    ASSERT_EQ(
        err,
        -EINVAL,
        c"bpf_map_update_elem bloom filter invalid flags".as_ptr(),
    );

    err = bpf_map_update_elem(
        fd,
        core::ptr::null(),
        &value as *const _ as *const c_void,
        BPF_EXIST,
    );
    ASSERT_EQ(
        err,
        -EINVAL,
        c"bpf_map_update_elem bloom filter invalid flags".as_ptr(),
    );

    err = bpf_map_update_elem(
        fd,
        core::ptr::null(),
        &value as *const _ as *const c_void,
        BPF_F_LOCK,
    );
    ASSERT_EQ(
        err,
        -EINVAL,
        c"bpf_map_update_elem bloom filter invalid flags".as_ptr(),
    );

    err = bpf_map_update_elem(
        fd,
        core::ptr::null(),
        &value as *const _ as *const c_void,
        BPF_NOEXIST,
    );
    ASSERT_EQ(
        err,
        -EINVAL,
        c"bpf_map_update_elem bloom filter invalid flags".as_ptr(),
    );

    err = bpf_map_update_elem(
        fd,
        core::ptr::null(),
        &value as *const _ as *const c_void,
        10000,
    );
    ASSERT_EQ(
        err,
        -EINVAL,
        c"bpf_map_update_elem bloom filter invalid flags".as_ptr(),
    );

    close(fd);
}

unsafe fn test_success_cases() {
    let mut opts: bpf_map_create_opts = core::mem::zeroed();
    opts.sz = core::mem::size_of::<bpf_map_create_opts>();
    let mut value: [c_char; 11] = [0; 11];
    let fd: c_int;
    let mut err: c_int;

    /* Create a map */
    opts.map_flags = (BPF_F_ZERO_SEED | BPF_F_NUMA_NODE) as u32;
    opts.numa_node = NUMA_NO_NODE as u32;
    fd = bpf_map_create(
        BPF_MAP_TYPE_BLOOM_FILTER,
        core::ptr::null(),
        0,
        core::mem::size_of_val(&value) as c_uint,
        100,
        &opts,
    );
    if !ASSERT_GE(
        fd,
        0,
        c"bpf_map_create bloom filter success case".as_ptr(),
    ) {
        return;
    }

    /* Add a value to the bloom filter */
    err = bpf_map_update_elem(fd, core::ptr::null(), &value as *const _ as *const c_void, 0);
    if !ASSERT_OK(
        err,
        c"bpf_map_update_elem bloom filter success case".as_ptr(),
    ) {
        close(fd);
        return;
    }

    /* Lookup a value in the bloom filter */
    err = bpf_map_lookup_elem(fd, core::ptr::null(), &mut value as *mut _ as *mut c_void);
    ASSERT_OK(
        err,
        c"bpf_map_update_elem bloom filter success case".as_ptr(),
    );

    close(fd);
}

unsafe fn check_bloom(skel: *mut bloom_filter_map) {
    let link: *mut bpf_link;

    link = bpf_program__attach((*skel).progs.check_bloom);
    if !ASSERT_OK_PTR(link as *const c_void, c"link".as_ptr()) {
        return;
    }

    syscall(SYS_getpgid);

    ASSERT_EQ((*(*skel).bss).error, 0, c"error".as_ptr());

    bpf_link__destroy(link);
}

unsafe fn test_inner_map(
    skel: *mut bloom_filter_map,
    rand_vals: *const __u32,
    nr_rand_vals: __u32,
) {
    let outer_map_fd: c_int;
    let mut inner_map_fd: c_int;
    let mut err: c_int;
    let mut i: c_int;
    let key: c_int = 0;
    let link: *mut bpf_link;

    /* Create a bloom filter map that will be used as the inner map */
    inner_map_fd = bpf_map_create(
        BPF_MAP_TYPE_BLOOM_FILTER,
        core::ptr::null(),
        0,
        core::mem::size_of::<__u32>() as c_uint,
        nr_rand_vals,
        core::ptr::null(),
    );
    if !ASSERT_GE(
        inner_map_fd,
        0,
        c"bpf_map_create bloom filter inner map".as_ptr(),
    ) {
        return;
    }

    i = 0;
    while i < nr_rand_vals as c_int {
        err = bpf_map_update_elem(
            inner_map_fd,
            core::ptr::null(),
            rand_vals.add(i as usize) as *const c_void,
            BPF_ANY,
        );
        if !ASSERT_OK(err, c"Add random value to inner_map_fd".as_ptr()) {
            close(inner_map_fd);
            return;
        }
        i += 1;
    }

    /* Add the bloom filter map to the outer map */
    outer_map_fd = bpf_map__fd((*skel).maps.outer_map);
    err = bpf_map_update_elem(
        outer_map_fd,
        &key as *const _ as *const c_void,
        &inner_map_fd as *const _ as *const c_void,
        BPF_ANY,
    );
    if !ASSERT_OK(err, c"Add bloom filter map to outer map".as_ptr()) {
        close(inner_map_fd);
        return;
    }

    /* Attach the bloom_filter_inner_map prog */
    link = bpf_program__attach((*skel).progs.inner_map);
    if ASSERT_OK_PTR(link as *const c_void, c"link".as_ptr()) {
        syscall(SYS_getpgid);

        ASSERT_EQ((*(*skel).bss).error, 0, c"error".as_ptr());

        bpf_link__destroy(link);
    }

    /* Ensure the inner bloom filter map can be deleted */
    err = bpf_map_delete_elem(outer_map_fd, &key as *const _ as *const c_void);
    ASSERT_OK(err, c"Delete inner bloom filter map".as_ptr());

    close(inner_map_fd);
}

unsafe fn setup_progs(
    out_skel: *mut *mut bloom_filter_map,
    out_rand_vals: *mut *mut __u32,
    out_nr_rand_vals: *mut __u32,
) -> c_int {
    let skel: *mut bloom_filter_map;
    let random_data_fd: c_int;
    let bloom_fd: c_int;
    let mut rand_vals: *mut __u32 = core::ptr::null_mut();
    let map_size: __u32;
    let mut val: __u32;
    let mut err: c_int;
    let mut i: c_int;

    /* Set up a bloom filter map skeleton */
    skel = bloom_filter_map__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *const c_void,
        c"bloom_filter_map__open_and_load".as_ptr(),
    ) {
        return -EINVAL;
    }

    /* Set up rand_vals */
    map_size = bpf_map__max_entries((*skel).maps.map_random_data);
    rand_vals = malloc(core::mem::size_of::<__u32>() * map_size as usize) as *mut __u32;
    if rand_vals.is_null() {
        err = -ENOMEM;
        bloom_filter_map__destroy(skel);
        return err;
    }

    /* Generate random values and populate both skeletons */
    random_data_fd = bpf_map__fd((*skel).maps.map_random_data);
    bloom_fd = bpf_map__fd((*skel).maps.map_bloom);
    i = 0;
    while i < map_size as c_int {
        val = rand() as __u32;

        err = bpf_map_update_elem(
            random_data_fd,
            &i as *const _ as *const c_void,
            &val as *const _ as *const c_void,
            BPF_ANY,
        );
        if !ASSERT_OK(err, c"Add random value to map_random_data".as_ptr()) {
            bloom_filter_map__destroy(skel);
            if !rand_vals.is_null() {
                free(rand_vals as *mut c_void);
            }
            return err;
        }

        err = bpf_map_update_elem(
            bloom_fd,
            core::ptr::null(),
            &val as *const _ as *const c_void,
            BPF_ANY,
        );
        if !ASSERT_OK(err, c"Add random value to map_bloom".as_ptr()) {
            bloom_filter_map__destroy(skel);
            if !rand_vals.is_null() {
                free(rand_vals as *mut c_void);
            }
            return err;
        }

        *rand_vals.add(i as usize) = val;
        i += 1;
    }

    *out_skel = skel;
    *out_rand_vals = rand_vals;
    *out_nr_rand_vals = map_size;

    0
}

#[no_mangle]
pub unsafe extern "C" fn test_bloom_filter_map() {
    let mut rand_vals: *mut __u32 = core::ptr::null_mut();
    let mut nr_rand_vals: __u32 = 0;
    let mut skel: *mut bloom_filter_map = core::ptr::null_mut();
    let err: c_int;

    test_fail_cases();
    test_success_cases();

    err = setup_progs(&mut skel, &mut rand_vals, &mut nr_rand_vals);
    if err != 0 {
        return;
    }

    test_inner_map(skel, rand_vals, nr_rand_vals);
    free(rand_vals as *mut c_void);

    check_bloom(skel);

    bloom_filter_map__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
