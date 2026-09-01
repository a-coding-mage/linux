// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Bytedance */

/* Depends on test_progs.h and test_map_lookup_percpu_elem.skel.h. */

use core::ffi::{c_int, c_long, c_uint, c_void};

type __u64 = u64;

const __NR_getuid: c_long = 102;

#[repr(C)]
pub struct test_map_lookup_percpu_elem {
    pub rodata: *mut test_map_lookup_percpu_elem_rodata,
    pub bss: *mut test_map_lookup_percpu_elem_bss,
    pub maps: test_map_lookup_percpu_elem_maps,
}

#[repr(C)]
pub struct test_map_lookup_percpu_elem_rodata {
    pub my_pid: c_int,
    pub nr_cpus: c_int,
}

#[repr(C)]
pub struct test_map_lookup_percpu_elem_bss {
    pub percpu_array_elem_sum: __u64,
    pub percpu_hash_elem_sum: __u64,
    pub percpu_lru_hash_elem_sum: __u64,
}

#[repr(C)]
pub struct test_map_lookup_percpu_elem_maps {
    pub percpu_array_map: *mut bpf_map,
    pub percpu_hash_map: *mut bpf_map,
    pub percpu_lru_hash_map: *mut bpf_map,
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn libbpf_num_possible_cpus() -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn getpid() -> c_int;
    fn syscall(number: c_long, ...) -> c_long;

    fn test_map_lookup_percpu_elem__open() -> *mut test_map_lookup_percpu_elem;
    fn test_map_lookup_percpu_elem__load(skel: *mut test_map_lookup_percpu_elem) -> c_int;
    fn test_map_lookup_percpu_elem__attach(skel: *mut test_map_lookup_percpu_elem) -> c_int;
    fn test_map_lookup_percpu_elem__detach(skel: *mut test_map_lookup_percpu_elem);
    fn test_map_lookup_percpu_elem__destroy(skel: *mut test_map_lookup_percpu_elem);

    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: c_uint) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const u8) -> bool;
    fn ASSERT_OK(ret: c_int, name: *const u8) -> bool;
    fn ASSERT_EQ(actual: __u64, expected: __u64, name: *const u8) -> bool;
}

pub unsafe fn test_map_lookup_percpu_elem() {
    let mut skel: *mut test_map_lookup_percpu_elem;
    let mut key: __u64 = 0;
    let sum: __u64;
    let mut ret: c_int;
    let mut i: c_int;
    let nr_cpus: c_int = libbpf_num_possible_cpus();
    let buf: *mut __u64;

    buf = malloc((nr_cpus as usize) * core::mem::size_of::<__u64>()) as *mut __u64;
    if !ASSERT_OK_PTR(buf as *const c_void, c"malloc".as_ptr() as *const u8) {
        return;
    }

    i = 0;
    while i < nr_cpus {
        *buf.offset(i as isize) = i as __u64;
        i += 1;
    }
    sum = ((nr_cpus - 1) * nr_cpus / 2) as __u64;

    skel = test_map_lookup_percpu_elem__open();
    if !ASSERT_OK_PTR(
        skel as *const c_void,
        c"test_map_lookup_percpu_elem__open".as_ptr() as *const u8,
    ) {
        goto_exit(buf);
        return;
    }

    (*(*skel).rodata).my_pid = getpid();
    (*(*skel).rodata).nr_cpus = nr_cpus;

    ret = test_map_lookup_percpu_elem__load(skel);
    if !ASSERT_OK(ret, c"test_map_lookup_percpu_elem__load".as_ptr() as *const u8) {
        goto_cleanup(skel, buf);
        return;
    }

    ret = test_map_lookup_percpu_elem__attach(skel);
    if !ASSERT_OK(ret, c"test_map_lookup_percpu_elem__attach".as_ptr() as *const u8) {
        goto_cleanup(skel, buf);
        return;
    }

    ret = bpf_map_update_elem(
        bpf_map__fd((*skel).maps.percpu_array_map),
        &key as *const __u64 as *const c_void,
        buf as *const c_void,
        0,
    );
    ASSERT_OK(ret, c"percpu_array_map update".as_ptr() as *const u8);

    ret = bpf_map_update_elem(
        bpf_map__fd((*skel).maps.percpu_hash_map),
        &key as *const __u64 as *const c_void,
        buf as *const c_void,
        0,
    );
    ASSERT_OK(ret, c"percpu_hash_map update".as_ptr() as *const u8);

    ret = bpf_map_update_elem(
        bpf_map__fd((*skel).maps.percpu_lru_hash_map),
        &key as *const __u64 as *const c_void,
        buf as *const c_void,
        0,
    );
    ASSERT_OK(ret, c"percpu_lru_hash_map update".as_ptr() as *const u8);

    syscall(__NR_getuid);

    test_map_lookup_percpu_elem__detach(skel);

    ASSERT_EQ(
        (*(*skel).bss).percpu_array_elem_sum,
        sum,
        c"percpu_array lookup percpu elem".as_ptr() as *const u8,
    );
    ASSERT_EQ(
        (*(*skel).bss).percpu_hash_elem_sum,
        sum,
        c"percpu_hash lookup percpu elem".as_ptr() as *const u8,
    );
    ASSERT_EQ(
        (*(*skel).bss).percpu_lru_hash_elem_sum,
        sum,
        c"percpu_lru_hash lookup percpu elem".as_ptr() as *const u8,
    );

    goto_cleanup(skel, buf);
}

unsafe fn goto_cleanup(skel: *mut test_map_lookup_percpu_elem, buf: *mut __u64) {
    test_map_lookup_percpu_elem__destroy(skel);
    goto_exit(buf);
}

unsafe fn goto_exit(buf: *mut __u64) {
    free(buf as *mut c_void);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
