// SPDX-License-Identifier: GPL-2.0-only

/*
 * Copyright 2020 Google LLC.
 */

// C dependencies translated as external declarations:
// <test_progs.h>, <cgroup_helpers.h>, <network_helpers.h>
// "metadata_unused.skel.h", "metadata_used.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type __u32 = u32;
type __u64 = u64;

const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;

#[repr(C)]
pub struct bpf_prog_info {
    pub nr_map_ids: __u32,
    pub map_ids: __u64,
}

#[repr(C)]
pub struct bpf_map_info {
    pub id: __u32,
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
pub struct metadata_unused {
    pub progs: metadata_unused__progs,
    pub maps: metadata_unused__maps,
    pub rodata: *mut metadata_unused__rodata,
}

#[repr(C)]
pub struct metadata_unused__progs {
    pub prog: *mut bpf_program,
}

#[repr(C)]
pub struct metadata_unused__maps {
    pub rodata: *mut bpf_map,
}

#[repr(C)]
pub struct metadata_unused__rodata {
    pub bpf_metadata_a: [c_char; 4],
    pub bpf_metadata_b: c_int,
}

#[repr(C)]
pub struct metadata_used {
    pub progs: metadata_used__progs,
    pub maps: metadata_used__maps,
    pub rodata: *mut metadata_used__rodata,
}

#[repr(C)]
pub struct metadata_used__progs {
    pub prog: *mut bpf_program,
}

#[repr(C)]
pub struct metadata_used__maps {
    pub rodata: *mut bpf_map,
}

#[repr(C)]
pub struct metadata_used__rodata {
    pub bpf_metadata_a: [c_char; 4],
    pub bpf_metadata_b: c_int,
}

static mut duration: c_int = 0;

unsafe extern "C" {
    static mut errno: c_int;

    fn bpf_map_get_info_by_fd(
        fd: c_int,
        info: *mut bpf_map_info,
        info_len: *mut __u32,
    ) -> c_int;
    fn bpf_prog_get_info_by_fd(
        fd: c_int,
        info: *mut bpf_prog_info,
        info_len: *mut __u32,
    ) -> c_int;
    fn bpf_prog_bind_map(prog_fd: c_int, map_fd: c_int, opts: *const c_void) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;

    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;

    fn metadata_unused__open_and_load() -> *mut metadata_unused;
    fn metadata_unused__destroy(obj: *mut metadata_unused);
    fn metadata_used__open_and_load() -> *mut metadata_used;
    fn metadata_used__destroy(obj: *mut metadata_used);

    fn test__start_subtest(name: *const c_char) -> bool;
    fn CHECK(condition: bool, name: *const c_char, fmt: *const c_char, ...) -> bool;
}

#[inline]
unsafe fn ptr_to_u64<T>(ptr: *mut T) -> __u64 {
    ptr as __u64
}

unsafe fn prog_holds_map(prog_fd: c_int, map_fd: c_int) -> c_int {
    let mut prog_info: bpf_prog_info = mem::zeroed();
    let mut map_info: bpf_map_info = mem::zeroed();
    let mut prog_info_len: __u32;
    let mut map_info_len: __u32;
    let map_ids: *mut __u32;
    let nr_maps: c_int;
    let mut ret: c_int;
    let mut i: c_int;

    map_info_len = mem::size_of_val(&map_info) as __u32;
    ret = bpf_map_get_info_by_fd(map_fd, &mut map_info, &mut map_info_len);
    if ret != 0 {
        return -errno;
    }

    prog_info_len = mem::size_of_val(&prog_info) as __u32;
    ret = bpf_prog_get_info_by_fd(prog_fd, &mut prog_info, &mut prog_info_len);
    if ret != 0 {
        return -errno;
    }

    map_ids = calloc(prog_info.nr_map_ids as usize, mem::size_of::<__u32>()) as *mut __u32;
    if map_ids.is_null() {
        return -ENOMEM;
    }

    nr_maps = prog_info.nr_map_ids as c_int;
    memset(
        &mut prog_info as *mut bpf_prog_info as *mut c_void,
        0,
        mem::size_of_val(&prog_info),
    );
    prog_info.nr_map_ids = nr_maps as __u32;
    prog_info.map_ids = ptr_to_u64(map_ids);
    prog_info_len = mem::size_of_val(&prog_info) as __u32;

    ret = bpf_prog_get_info_by_fd(prog_fd, &mut prog_info, &mut prog_info_len);
    if ret != 0 {
        ret = -errno;
        free(map_ids as *mut c_void);
        return ret;
    }

    ret = -ENOENT;
    i = 0;
    while i < prog_info.nr_map_ids as c_int {
        if *map_ids.offset(i as isize) == map_info.id {
            ret = 0;
            break;
        }
        i += 1;
    }

    free(map_ids as *mut c_void);
    ret
}

unsafe fn test_metadata_unused() {
    let obj: *mut metadata_unused;
    let mut err: c_int;

    obj = metadata_unused__open_and_load();
    if CHECK(
        obj.is_null(),
        c"skel-load".as_ptr(),
        c"errno %d".as_ptr(),
        errno,
    ) {
        return;
    }

    err = prog_holds_map(
        bpf_program__fd((*obj).progs.prog),
        bpf_map__fd((*obj).maps.rodata),
    );
    if CHECK(
        err != 0,
        c"prog-holds-rodata".as_ptr(),
        c"errno: %d".as_ptr(),
        err,
    ) {
        return;
    }

    /* Assert that we can access the metadata in skel and the values are
     * what we expect.
     */
    if CHECK(
        strncmp(
            (*(*obj).rodata).bpf_metadata_a.as_ptr(),
            c"foo".as_ptr(),
            mem::size_of_val(&(*(*obj).rodata).bpf_metadata_a),
        ) != 0,
        c"bpf_metadata_a".as_ptr(),
        c"expected \"foo\", value differ".as_ptr(),
    ) {
        metadata_unused__destroy(obj);
        return;
    }
    if CHECK(
        (*(*obj).rodata).bpf_metadata_b != 1,
        c"bpf_metadata_b".as_ptr(),
        c"expected 1, got %d".as_ptr(),
        (*(*obj).rodata).bpf_metadata_b,
    ) {
        metadata_unused__destroy(obj);
        return;
    }

    /* Assert that binding metadata map to prog again succeeds. */
    err = bpf_prog_bind_map(
        bpf_program__fd((*obj).progs.prog),
        bpf_map__fd((*obj).maps.rodata),
        ptr::null(),
    );
    CHECK(
        err != 0,
        c"rebind_map".as_ptr(),
        c"errno %d, expected 0".as_ptr(),
        errno,
    );

    metadata_unused__destroy(obj);
}

unsafe fn test_metadata_used() {
    let obj: *mut metadata_used;
    let mut err: c_int;

    obj = metadata_used__open_and_load();
    if CHECK(
        obj.is_null(),
        c"skel-load".as_ptr(),
        c"errno %d".as_ptr(),
        errno,
    ) {
        return;
    }

    err = prog_holds_map(
        bpf_program__fd((*obj).progs.prog),
        bpf_map__fd((*obj).maps.rodata),
    );
    if CHECK(
        err != 0,
        c"prog-holds-rodata".as_ptr(),
        c"errno: %d".as_ptr(),
        err,
    ) {
        return;
    }

    /* Assert that we can access the metadata in skel and the values are
     * what we expect.
     */
    if CHECK(
        strncmp(
            (*(*obj).rodata).bpf_metadata_a.as_ptr(),
            c"bar".as_ptr(),
            mem::size_of_val(&(*(*obj).rodata).bpf_metadata_a),
        ) != 0,
        c"metadata_a".as_ptr(),
        c"expected \"bar\", value differ".as_ptr(),
    ) {
        metadata_used__destroy(obj);
        return;
    }
    if CHECK(
        (*(*obj).rodata).bpf_metadata_b != 2,
        c"metadata_b".as_ptr(),
        c"expected 2, got %d".as_ptr(),
        (*(*obj).rodata).bpf_metadata_b,
    ) {
        metadata_used__destroy(obj);
        return;
    }

    /* Assert that binding metadata map to prog again succeeds. */
    err = bpf_prog_bind_map(
        bpf_program__fd((*obj).progs.prog),
        bpf_map__fd((*obj).maps.rodata),
        ptr::null(),
    );
    CHECK(
        err != 0,
        c"rebind_map".as_ptr(),
        c"errno %d, expected 0".as_ptr(),
        errno,
    );

    metadata_used__destroy(obj);
}

pub unsafe extern "C" fn test_metadata() {
    if test__start_subtest(c"unused".as_ptr()) {
        test_metadata_unused();
    }

    if test__start_subtest(c"used".as_ptr()) {
        test_metadata_used();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
