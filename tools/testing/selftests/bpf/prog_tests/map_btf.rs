// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023. Huawei Technologies Co., Ltd */

// C dependencies:
// #include <test_progs.h>
// #include "normal_map_btf.skel.h"
// #include "map_in_map_btf.skel.h"

#[repr(C)]
pub struct normal_map_btf {
    pub bss: *mut normal_map_btf_bss,
    pub maps: normal_map_btf_maps,
}

#[repr(C)]
pub struct normal_map_btf_bss {
    pub pid: i32,
    pub done: bool,
}

#[repr(C)]
pub struct normal_map_btf_maps {
    pub array: *mut bpf_map,
}

#[repr(C)]
pub struct map_in_map_btf {
    pub bss: *mut map_in_map_btf_bss,
    pub maps: map_in_map_btf_maps,
}

#[repr(C)]
pub struct map_in_map_btf_bss {
    pub pid: i32,
    pub done: bool,
}

#[repr(C)]
pub struct map_in_map_btf_maps {
    pub inner_array: *mut bpf_map,
    pub outer_array: *mut bpf_map,
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn normal_map_btf__open_and_load() -> *mut normal_map_btf;
    fn normal_map_btf__attach(skel: *mut normal_map_btf) -> i32;
    fn normal_map_btf__destroy(skel: *mut normal_map_btf);

    fn map_in_map_btf__open_and_load() -> *mut map_in_map_btf;
    fn map_in_map_btf__attach(skel: *mut map_in_map_btf) -> i32;
    fn map_in_map_btf__destroy(skel: *mut map_in_map_btf);

    fn ASSERT_OK_PTR(ptr: *const core::ffi::c_void, name: *const core::ffi::c_char) -> bool;
    fn ASSERT_OK(err: i32, name: *const core::ffi::c_char) -> bool;
    fn ASSERT_TRUE(value: bool, name: *const core::ffi::c_char) -> bool;
    fn test__start_subtest(name: *const core::ffi::c_char) -> bool;

    fn getpid() -> i32;
    fn usleep(usec: u32) -> i32;
    fn bpf_map_create(
        map_type: u32,
        map_name: *const core::ffi::c_char,
        key_size: u32,
        value_size: u32,
        max_entries: u32,
        opts: *const core::ffi::c_void,
    ) -> i32;
    fn bpf_map__fd(map: *mut bpf_map) -> i32;
    fn dup(oldfd: i32) -> i32;
    fn kern_sync_rcu();
    fn close(fd: i32) -> i32;
    fn bpf_map__delete_elem(
        map: *mut bpf_map,
        key: *const core::ffi::c_void,
        key_sz: usize,
        flags: u64,
    ) -> i32;
}

const BPF_MAP_TYPE_PERCPU_ARRAY: u32 = 6;

unsafe fn do_test_normal_map_btf() {
    let mut skel: *mut normal_map_btf;
    let mut i: usize;
    let mut err: i32;
    let mut new_fd: i32 = -1;
    let mut map_fd_arr: [i32; 64] = [0; 64];

    skel = normal_map_btf__open_and_load();
    if !ASSERT_OK_PTR(skel as *const core::ffi::c_void, c"open_load".as_ptr()) {
        return;
    }

    err = normal_map_btf__attach(skel);
    if !ASSERT_OK(err, c"attach".as_ptr()) {
        goto_out_normal_map_btf(skel, new_fd, &mut map_fd_arr);
        return;
    }

    (*(*skel).bss).pid = getpid();
    usleep(1);
    ASSERT_TRUE((*(*skel).bss).done, c"done".as_ptr());

    /* Use percpu_array to slow bpf_map_free_deferred() down.
     * The memory allocation may fail, so doesn't check the returned fd.
     */
    i = 0;
    while i < map_fd_arr.len() {
        map_fd_arr[i] = bpf_map_create(
            BPF_MAP_TYPE_PERCPU_ARRAY,
            core::ptr::null(),
            4,
            4,
            256,
            core::ptr::null(),
        );
        i += 1;
    }

    /* Close array fd later */
    new_fd = dup(bpf_map__fd((*skel).maps.array));

    goto_out_normal_map_btf(skel, new_fd, &mut map_fd_arr);
}

unsafe fn goto_out_normal_map_btf(
    skel: *mut normal_map_btf,
    new_fd: i32,
    map_fd_arr: &mut [i32; 64],
) {
    let mut i: usize;

    normal_map_btf__destroy(skel);
    if new_fd < 0 {
        return;
    }
    /* Use kern_sync_rcu() to wait for the start of the free of the bpf
     * program and use an assumed delay to wait for the release of the map
     * btf which is held by other maps (e.g, bss). After that, array map
     * holds the last reference of map btf.
     */
    kern_sync_rcu();
    usleep(4000);
    /* Spawn multiple kworkers to delay the invocation of
     * bpf_map_free_deferred() for array map.
     */
    i = 0;
    while i < map_fd_arr.len() {
        if map_fd_arr[i] < 0 {
            i += 1;
            continue;
        }
        close(map_fd_arr[i]);
        i += 1;
    }
    close(new_fd);
}

unsafe fn do_test_map_in_map_btf() {
    let mut err: i32;
    let zero: i32 = 0;
    let mut new_fd: i32 = -1;
    let mut skel: *mut map_in_map_btf;

    skel = map_in_map_btf__open_and_load();
    if !ASSERT_OK_PTR(skel as *const core::ffi::c_void, c"open_load".as_ptr()) {
        return;
    }

    err = map_in_map_btf__attach(skel);
    if !ASSERT_OK(err, c"attach".as_ptr()) {
        goto_out_map_in_map_btf(skel, new_fd);
        return;
    }

    (*(*skel).bss).pid = getpid();
    usleep(1);
    ASSERT_TRUE((*(*skel).bss).done, c"done".as_ptr());

    /* Close inner_array fd later */
    new_fd = dup(bpf_map__fd((*skel).maps.inner_array));
    /* Defer the free of inner_array */
    err = bpf_map__delete_elem(
        (*skel).maps.outer_array,
        &zero as *const i32 as *const core::ffi::c_void,
        core::mem::size_of_val(&zero),
        0,
    );
    ASSERT_OK(err, c"delete inner map".as_ptr());

    goto_out_map_in_map_btf(skel, new_fd);
}

unsafe fn goto_out_map_in_map_btf(skel: *mut map_in_map_btf, new_fd: i32) {
    map_in_map_btf__destroy(skel);
    if new_fd < 0 {
        return;
    }
    /* Use kern_sync_rcu() to wait for the start of the free of the bpf
     * program and use an assumed delay to wait for the free of the outer
     * map and the release of map btf. After that, inner map holds the last
     * reference of map btf.
     */
    kern_sync_rcu();
    usleep(10000);
    close(new_fd);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_map_btf() {
    if test__start_subtest(c"array_btf".as_ptr()) {
        do_test_normal_map_btf();
    }
    if test__start_subtest(c"inner_array_btf".as_ptr()) {
        do_test_map_in_map_btf();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
