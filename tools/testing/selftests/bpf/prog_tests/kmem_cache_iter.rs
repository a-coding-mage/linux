// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Google */

/*
 * Translated from:
 *   testing/selftests/bpf/prog_tests/kmem_cache_iter.c
 *
 * C dependencies intentionally left as external bindings:
 *   <test_progs.h>, <bpf/libbpf.h>, <bpf/btf.h>,
 *   "kmem_cache_iter.skel.h"
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

const SLAB_NAME_MAX: usize = 32;

#[repr(C)]
pub struct kmem_cache_result {
    pub name: [c_char; SLAB_NAME_MAX],
    pub obj_size: c_long,
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub flags: u32,
    pub retval: u32,
}

#[repr(C)]
pub struct kmem_cache_iter {
    pub progs: kmem_cache_iter_progs,
    pub maps: kmem_cache_iter_maps,
    pub links: kmem_cache_iter_links,
    pub bss: *mut kmem_cache_iter_bss,
}

#[repr(C)]
pub struct kmem_cache_iter_progs {
    pub check_task_struct: *mut c_void,
    pub open_coded_iter: *mut c_void,
}

#[repr(C)]
pub struct kmem_cache_iter_maps {
    pub slab_result: *mut c_void,
}

#[repr(C)]
pub struct kmem_cache_iter_links {
    pub slab_info_collector: *mut c_void,
}

#[repr(C)]
pub struct kmem_cache_iter_bss {
    pub task_struct_found: c_int,
    pub kmem_cache_seen: c_int,
    pub open_coded_seen: c_int,
}

unsafe extern "C" {
    fn bpf_program__fd(prog: *mut c_void) -> c_int;
    fn bpf_map__fd(map: *mut c_void) -> c_int;
    fn bpf_link__fd(link: *mut c_void) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_iter_create(link_fd: c_int) -> c_int;

    fn kmem_cache_iter__open_and_load() -> *mut kmem_cache_iter;
    fn kmem_cache_iter__attach(skel: *mut kmem_cache_iter) -> c_int;
    fn kmem_cache_iter__destroy(skel: *mut kmem_cache_iter);

    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ<T: Copy, U: Copy>(actual: T, expected: U, name: *const c_char) -> bool;
    fn ASSERT_GE<T: Copy, U: Copy>(actual: T, expected: U, name: *const c_char) -> bool;
    fn ASSERT_STRNEQ(
        actual: *const c_char,
        expected: *const c_char,
        len: usize,
        name: *const c_char,
    ) -> bool;

    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
}

unsafe fn subtest_kmem_cache_iter_check_task_struct(skel: *mut kmem_cache_iter) {
    let mut opts = bpf_test_run_opts {
        sz: core::mem::size_of::<bpf_test_run_opts>(),
        flags: 0, /* Run it with the current task */
        retval: 0,
    };
    let prog_fd = bpf_program__fd((*skel).progs.check_task_struct);

    /* Get task_struct and check it if's from a slab cache */
    ASSERT_OK(
        bpf_prog_test_run_opts(prog_fd, &mut opts),
        c"prog_test_run".as_ptr(),
    );

    /* The BPF program should set 'found' variable */
    ASSERT_EQ(
        (*(*skel).bss).task_struct_found,
        1,
        c"task_struct_found".as_ptr(),
    );
}

unsafe fn subtest_kmem_cache_iter_check_slabinfo(skel: *mut kmem_cache_iter) {
    let fp: *mut FILE;
    let map_fd: c_int;
    let mut name = [0 as c_char; SLAB_NAME_MAX];
    let mut objsize: c_ulong = 0;
    let mut rest_of_line = [0 as c_char; 1000];
    let mut r = kmem_cache_result {
        name: [0 as c_char; SLAB_NAME_MAX],
        obj_size: 0,
    };
    let mut seen: c_int = 0;

    fp = fopen(c"/proc/slabinfo".as_ptr(), c"r".as_ptr());
    if fp.is_null() {
        /* CONFIG_SLUB_DEBUG is not enabled */
        return;
    }

    map_fd = bpf_map__fd((*skel).maps.slab_result);

    /* Ignore first two lines for header */
    fscanf(fp, c"slabinfo - version: %*d.%*d\n".as_ptr());
    fscanf(
        fp,
        c"# %*s %*s %*s %*s %*s %*s : %[^\n]\n".as_ptr(),
        rest_of_line.as_mut_ptr(),
    );

    /* Compare name and objsize only - others can be changes frequently */
    while fscanf(
        fp,
        c"%s %*u %*u %lu %*u %*u : %[^\n]\n".as_ptr(),
        name.as_mut_ptr(),
        &mut objsize,
        rest_of_line.as_mut_ptr(),
    ) == 3
    {
        let ret = bpf_map_lookup_elem(
            map_fd,
            &seen as *const c_int as *const c_void,
            &mut r as *mut kmem_cache_result as *mut c_void,
        );

        if !ASSERT_OK(ret, c"kmem_cache_lookup".as_ptr()) {
            break;
        }

        ASSERT_STRNEQ(
            r.name.as_ptr(),
            name.as_ptr(),
            core::mem::size_of_val(&r.name) - 1,
            c"kmem_cache_name".as_ptr(),
        );
        ASSERT_EQ(r.obj_size, objsize, c"kmem_cache_objsize".as_ptr());

        seen += 1;
    }

    ASSERT_EQ(
        (*(*skel).bss).kmem_cache_seen,
        seen,
        c"kmem_cache_seen_eq".as_ptr(),
    );

    fclose(fp);
}

unsafe fn subtest_kmem_cache_iter_open_coded(skel: *mut kmem_cache_iter) {
    let mut topts = bpf_test_run_opts {
        sz: core::mem::size_of::<bpf_test_run_opts>(),
        flags: 0,
        retval: 0,
    };
    let err: c_int;
    let fd: c_int;

    /* No need to attach it, just run it directly */
    fd = bpf_program__fd((*skel).progs.open_coded_iter);

    err = bpf_prog_test_run_opts(fd, &mut topts);
    if !ASSERT_OK(err, c"test_run_opts err".as_ptr()) {
        return;
    }
    if !ASSERT_OK(topts.retval as c_int, c"test_run_opts retval".as_ptr()) {
        return;
    }

    /* It should be same as we've seen from the explicit iterator */
    ASSERT_EQ(
        (*(*skel).bss).open_coded_seen,
        (*(*skel).bss).kmem_cache_seen,
        c"open_code_seen_eq".as_ptr(),
    );
}

#[no_mangle]
pub unsafe extern "C" fn test_kmem_cache_iter() {
    let mut skel: *mut kmem_cache_iter = core::ptr::null_mut();
    let mut buf = [0 as c_char; 256];
    let iter_fd: c_int;

    skel = kmem_cache_iter__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"kmem_cache_iter__open_and_load".as_ptr()) {
        return;
    }

    if !ASSERT_OK(kmem_cache_iter__attach(skel), c"skel_attach".as_ptr()) {
        kmem_cache_iter__destroy(skel);
        return;
    }

    iter_fd = bpf_iter_create(bpf_link__fd((*skel).links.slab_info_collector));
    if !ASSERT_GE(iter_fd, 0, c"iter_create".as_ptr()) {
        kmem_cache_iter__destroy(skel);
        return;
    }

    while read(iter_fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf)) > 0 {
        /* Read out all contents */
    }

    /* Next reads should return 0 */
    ASSERT_EQ(
        read(
            iter_fd,
            buf.as_mut_ptr() as *mut c_void,
            core::mem::size_of_val(&buf),
        ),
        0,
        c"read".as_ptr(),
    );

    if test__start_subtest(c"check_task_struct".as_ptr()) {
        subtest_kmem_cache_iter_check_task_struct(skel);
    }
    if test__start_subtest(c"check_slabinfo".as_ptr()) {
        subtest_kmem_cache_iter_check_slabinfo(skel);
    }
    if test__start_subtest(c"open_coded_iter".as_ptr()) {
        subtest_kmem_cache_iter_open_coded(skel);
    }

    close(iter_fd);

    kmem_cache_iter__destroy(skel);
}
