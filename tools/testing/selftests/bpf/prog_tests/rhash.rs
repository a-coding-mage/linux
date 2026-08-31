// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */
/* Translated from C source using test_progs/libbpf, generated skeletons, and Linux BPF APIs. */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};

type __u32 = u32;
type __u64 = u64;
type u32 = u32;
type u64 = u64;

const BPF_F_NO_PREALLOC: __u32 = 1;
const BPF_NOEXIST: __u64 = 1;
const BPF_MAP_TYPE_RHASH: bpf_map_type = 37;

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

type bpf_map_type = c_uint;

#[repr(C)]
pub struct rhash_bss {
    pub err: c_int,
}

#[repr(C)]
pub struct rhash {
    pub obj: *mut bpf_object,
    pub bss: *mut rhash_bss,
}

#[repr(C)]
pub struct bpf_iter_bpf_rhash_map_bss {
    pub key_sum: u32,
    pub elem_count: u32,
}

#[repr(C)]
pub struct bpf_iter_bpf_rhash_map_maps {
    pub rhashmap: *mut bpf_map,
}

#[repr(C)]
pub struct bpf_iter_bpf_rhash_map_progs {
    pub dump_bpf_rhash_map: *mut bpf_program,
}

#[repr(C)]
pub struct bpf_iter_bpf_rhash_map {
    pub maps: bpf_iter_bpf_rhash_map_maps,
    pub progs: bpf_iter_bpf_rhash_map_progs,
    pub bss: *mut bpf_iter_bpf_rhash_map_bss,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub retval: __u32,
}

#[repr(C)]
pub struct bpf_map_create_opts {
    pub sz: usize,
    pub map_flags: __u32,
    pub map_extra: __u64,
}

#[repr(C)]
pub struct bpf_map_info {
    pub map_extra: __u64,
}

#[repr(C)]
pub struct bpf_iter_attach_opts {
    pub sz: usize,
    pub link_info: *mut bpf_iter_link_info,
    pub link_info_len: __u32,
}

#[repr(C)]
pub struct bpf_iter_link_info_map {
    pub map_fd: __u32,
}

#[repr(C)]
pub union bpf_iter_link_info {
    pub map: bpf_iter_link_info_map,
}

unsafe extern "C" {
    fn rhash__open() -> *mut rhash;
    fn rhash__load(skel: *mut rhash) -> c_int;
    fn rhash__destroy(skel: *mut rhash);

    fn bpf_iter_bpf_rhash_map__open() -> *mut bpf_iter_bpf_rhash_map;
    fn bpf_iter_bpf_rhash_map__load(skel: *mut bpf_iter_bpf_rhash_map) -> c_int;
    fn bpf_iter_bpf_rhash_map__destroy(skel: *mut bpf_iter_bpf_rhash_map);

    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_map_create(
        map_type: bpf_map_type,
        map_name: *const c_char,
        key_size: __u32,
        value_size: __u32,
        max_entries: __u32,
        opts: *mut bpf_map_create_opts,
    ) -> c_int;
    fn bpf_map_get_info_by_fd(fd: c_int, info: *mut bpf_map_info, info_len: *__u32) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: __u64,
    ) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_program__attach_iter(
        prog: *mut bpf_program,
        opts: *mut bpf_iter_attach_opts,
    ) -> *mut bpf_link;
    fn bpf_iter_create(link_fd: c_int) -> c_int;
    fn bpf_link__fd(link: *mut bpf_link) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_LT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: __u64, expected: __u64, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;

    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
}

unsafe fn rhash_run(prog_name: *const c_char) {
    let mut skel: *mut rhash;
    let mut prog: *mut bpf_program;
    let mut opts: bpf_test_run_opts = bpf_test_run_opts {
        sz: size_of::<bpf_test_run_opts>(),
        retval: 0,
    };
    let mut err: c_int;

    skel = rhash__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"rhash__open".as_ptr()) {
        return;
    }

    prog = bpf_object__find_program_by_name((*skel).obj, prog_name);
    if !ASSERT_OK_PTR(
        prog as *const c_void,
        c"bpf_object__find_program_by_name".as_ptr(),
    ) {
        rhash__destroy(skel);
        return;
    }
    bpf_program__set_autoload(prog, true);

    err = rhash__load(skel);
    if !ASSERT_OK(err, c"skel_load".as_ptr()) {
        rhash__destroy(skel);
        return;
    }

    err = bpf_prog_test_run_opts(bpf_program__fd(prog), &mut opts);
    if !ASSERT_OK(err, c"prog run".as_ptr()) {
        rhash__destroy(skel);
        return;
    }

    if !ASSERT_OK(opts.retval as c_int, c"prog retval".as_ptr()) {
        rhash__destroy(skel);
        return;
    }

    if !ASSERT_OK((*(*skel).bss).err, c"bss->err".as_ptr()) {
        rhash__destroy(skel);
        return;
    }

    rhash__destroy(skel);
}

unsafe fn rhash_map_create(max_entries: __u32, map_extra: __u64) -> c_int {
    let mut opts: bpf_map_create_opts = bpf_map_create_opts {
        sz: size_of::<bpf_map_create_opts>(),
        map_flags: BPF_F_NO_PREALLOC,
        map_extra,
    };

    bpf_map_create(
        BPF_MAP_TYPE_RHASH,
        c"rhash_extra".as_ptr(),
        size_of::<__u32>() as __u32,
        size_of::<__u64>() as __u32,
        max_entries,
        &mut opts,
    )
}

unsafe fn rhash_map_extra_presize() {
    const MAX_ENTRIES: __u32 = 1024;
    const NELEM_HINT: __u32 = 256;
    let mut info: bpf_map_info = zeroed();
    let mut info_len: __u32 = size_of::<bpf_map_info>() as __u32;
    let val: __u64 = 0;
    let mut key: __u32;
    let mut fd: c_int;
    let mut i: c_int;

    fd = rhash_map_create(MAX_ENTRIES, NELEM_HINT as __u64);
    if !ASSERT_GE(fd, 0, c"rhash_map_create presize".as_ptr()) {
        return;
    }

    if !ASSERT_OK(
        bpf_map_get_info_by_fd(fd, &mut info, &mut info_len),
        c"info".as_ptr(),
    ) {
        close(fd);
        return;
    }
    ASSERT_EQ(info.map_extra, NELEM_HINT as __u64, c"info.map_extra".as_ptr());

    i = 0;
    while i < NELEM_HINT as c_int {
        key = i as __u32;
        if !ASSERT_OK(
            bpf_map_update_elem(
                fd,
                &key as *const __u32 as *const c_void,
                &val as *const __u64 as *const c_void,
                BPF_NOEXIST,
            ),
            c"update".as_ptr(),
        ) {
            close(fd);
            return;
        }
        i += 1;
    }

    close(fd);
}

unsafe fn rhash_map_extra_too_big() {
    let mut fd: c_int;

    fd = rhash_map_create(1u32 << 20, 0x10000);
    if !ASSERT_LT(fd, 0, c"rhash_map_create hint > U16_MAX".as_ptr()) {
        close(fd);
    }
}

unsafe fn rhash_iter_test() {
    let mut opts: bpf_iter_attach_opts = bpf_iter_attach_opts {
        sz: size_of::<bpf_iter_attach_opts>(),
        link_info: core::ptr::null_mut(),
        link_info_len: 0,
    };
    let mut skel: *mut bpf_iter_bpf_rhash_map;
    let mut err: c_int;
    let mut i: c_int;
    let mut len: isize;
    let mut map_fd: c_int;
    let mut iter_fd: c_int;
    let mut linfo: bpf_iter_link_info = zeroed();
    let mut expected_key_sum: u32 = 0;
    let mut key: u32;
    let mut link: *mut bpf_link;
    let val: u64 = 0;
    let mut buf: [c_char; 64] = [0; 64];

    skel = bpf_iter_bpf_rhash_map__open();
    if !ASSERT_OK_PTR(
        skel as *const c_void,
        c"bpf_iter_bpf_rhash_map__open".as_ptr(),
    ) {
        return;
    }

    err = bpf_iter_bpf_rhash_map__load(skel);
    if !ASSERT_OK(err, c"bpf_iter_bpf_rhash_map__load".as_ptr()) {
        bpf_iter_bpf_rhash_map__destroy(skel);
        return;
    }

    map_fd = bpf_map__fd((*skel).maps.rhashmap);

    /* Populate map with test data */
    i = 0;
    while i < 64 {
        key = (i + 1) as u32;
        expected_key_sum = expected_key_sum.wrapping_add(key);

        err = bpf_map_update_elem(
            map_fd,
            &key as *const u32 as *const c_void,
            &val as *const u64 as *const c_void,
            BPF_NOEXIST,
        );
        if !ASSERT_OK(err, c"map_update".as_ptr()) {
            bpf_iter_bpf_rhash_map__destroy(skel);
            return;
        }
        i += 1;
    }

    memset(
        &mut linfo as *mut bpf_iter_link_info as *mut c_void,
        0,
        size_of::<bpf_iter_link_info>(),
    );
    linfo.map.map_fd = map_fd as __u32;
    opts.link_info = &mut linfo;
    opts.link_info_len = size_of::<bpf_iter_link_info>() as __u32;

    link = bpf_program__attach_iter((*skel).progs.dump_bpf_rhash_map, &mut opts);
    if !ASSERT_OK_PTR(link as *const c_void, c"attach_iter".as_ptr()) {
        bpf_iter_bpf_rhash_map__destroy(skel);
        return;
    }

    iter_fd = bpf_iter_create(bpf_link__fd(link));
    if !ASSERT_GE(iter_fd, 0, c"create_iter".as_ptr()) {
        bpf_link__destroy(link);
        bpf_iter_bpf_rhash_map__destroy(skel);
        return;
    }

    loop {
        len = read(
            iter_fd,
            buf.as_mut_ptr() as *mut c_void,
            size_of::<[c_char; 64]>(),
        );
        if len <= 0 {
            break;
        }
    }

    ASSERT_EQ(
        (*(*skel).bss).key_sum as __u64,
        expected_key_sum as __u64,
        c"key_sum".as_ptr(),
    );
    ASSERT_EQ((*(*skel).bss).elem_count as __u64, 64, c"elem_count".as_ptr());

    close(iter_fd);

    bpf_link__destroy(link);
    bpf_iter_bpf_rhash_map__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_rhash() {
    if test__start_subtest(c"test_rhash_lookup_update".as_ptr()) {
        rhash_run(c"test_rhash_lookup_update".as_ptr());
    }

    if test__start_subtest(c"test_rhash_update_delete".as_ptr()) {
        rhash_run(c"test_rhash_update_delete".as_ptr());
    }

    if test__start_subtest(c"test_rhash_update_elements".as_ptr()) {
        rhash_run(c"test_rhash_update_elements".as_ptr());
    }

    if test__start_subtest(c"test_rhash_update_exist".as_ptr()) {
        rhash_run(c"test_rhash_update_exist".as_ptr());
    }

    if test__start_subtest(c"test_rhash_update_any".as_ptr()) {
        rhash_run(c"test_rhash_update_any".as_ptr());
    }

    if test__start_subtest(c"test_rhash_noexist_duplicate".as_ptr()) {
        rhash_run(c"test_rhash_noexist_duplicate".as_ptr());
    }

    if test__start_subtest(c"test_rhash_delete_nonexistent".as_ptr()) {
        rhash_run(c"test_rhash_delete_nonexistent".as_ptr());
    }

    if test__start_subtest(c"test_rhash_map_extra_presize".as_ptr()) {
        rhash_map_extra_presize();
    }

    if test__start_subtest(c"test_rhash_map_extra_too_big".as_ptr()) {
        rhash_map_extra_too_big();
    }

    if test__start_subtest(c"test_rhash_iter".as_ptr()) {
        rhash_iter_test();
    }
}
