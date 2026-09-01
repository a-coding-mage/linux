// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2019 Facebook */
/* Dependencies in the C source:
 * #include <test_progs.h>
 * #include <linux/bpf.h>
 * #include "test_pe_preserve_elems.skel.h"
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::mem;
use core::ptr;
use std::os::raw::{c_char, c_int, c_long, c_ulong};

const PERF_TYPE_SOFTWARE: u32 = 1;
const PERF_COUNT_SW_CPU_CLOCK: u64 = 0;
const BPF_ANY: u64 = 0;
const ENOENT: c_int = 2;

/* __NR_perf_event_open is architecture-specific in Linux headers. */
#[cfg(target_arch = "x86_64")]
const __NR_perf_event_open: c_long = 298;
#[cfg(target_arch = "aarch64")]
const __NR_perf_event_open: c_long = 241;

static mut duration: c_int = 0;

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub retval: u32,
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
}

#[repr(C)]
pub struct test_pe_preserve_elems {
    pub maps: test_pe_preserve_elems_maps,
    pub progs: test_pe_preserve_elems_progs,
}

#[repr(C)]
pub struct test_pe_preserve_elems_maps {
    pub array_1: *mut bpf_map,
    pub array_2: *mut bpf_map,
}

#[repr(C)]
pub struct test_pe_preserve_elems_progs {
    pub read_array_1: *mut bpf_program,
    pub read_array_2: *mut bpf_program,
}

unsafe extern "C" {
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_int,
        value: *const c_int,
        flags: u64,
    ) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn test_pe_preserve_elems__open_and_load() -> *mut test_pe_preserve_elems;
    fn test_pe_preserve_elems__destroy(skel: *mut test_pe_preserve_elems);
    fn close(fd: c_int) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn CHECK(condition: bool, name: *const c_char, fmt: *const c_char, ...) -> bool;
}

unsafe fn test_one_map(map: *mut bpf_map, prog: *mut bpf_program, has_share_pe: bool) {
    let mut err: c_int;
    let key: c_int = 0;
    let mut pfd: c_int = -1;
    let mfd: c_int = bpf_map__fd(map);
    let mut opts: bpf_test_run_opts = mem::zeroed();
    opts.sz = mem::size_of::<bpf_test_run_opts>();
    let mut attr = perf_event_attr {
        size: mem::size_of::<perf_event_attr>() as u32,
        type_: PERF_TYPE_SOFTWARE,
        config: PERF_COUNT_SW_CPU_CLOCK,
    };

    pfd = syscall(
        __NR_perf_event_open,
        &mut attr as *mut perf_event_attr,
        0 as c_int,  /* pid */
        -1 as c_int, /* cpu 0 */
        -1 as c_int, /* group id */
        0 as c_ulong, /* flags */
    ) as c_int;
    if CHECK(
        pfd < 0,
        b"perf_event_open\0".as_ptr() as *const c_char,
        b"failed\n\0".as_ptr() as *const c_char,
    ) {
        return;
    }

    err = bpf_map_update_elem(
        mfd,
        &key as *const c_int,
        &pfd as *const c_int,
        BPF_ANY,
    );
    close(pfd);
    if CHECK(
        err < 0,
        b"bpf_map_update_elem\0".as_ptr() as *const c_char,
        b"failed\n\0".as_ptr() as *const c_char,
    ) {
        return;
    }

    err = bpf_prog_test_run_opts(bpf_program__fd(prog), &mut opts as *mut bpf_test_run_opts);
    if CHECK(
        err < 0,
        b"bpf_prog_test_run_opts\0".as_ptr() as *const c_char,
        b"failed\n\0".as_ptr() as *const c_char,
    ) {
        return;
    }
    if CHECK(
        opts.retval != 0,
        b"bpf_perf_event_read_value\0".as_ptr() as *const c_char,
        b"failed with %d\n\0".as_ptr() as *const c_char,
        opts.retval as c_int,
    ) {
        return;
    }

    /* closing mfd, prog still holds a reference on map */
    close(mfd);

    err = bpf_prog_test_run_opts(bpf_program__fd(prog), &mut opts as *mut bpf_test_run_opts);
    if CHECK(
        err < 0,
        b"bpf_prog_test_run_opts\0".as_ptr() as *const c_char,
        b"failed\n\0".as_ptr() as *const c_char,
    ) {
        return;
    }

    if has_share_pe {
        CHECK(
            opts.retval != 0,
            b"bpf_perf_event_read_value\0".as_ptr() as *const c_char,
            b"failed with %d\n\0".as_ptr() as *const c_char,
            opts.retval as c_int,
        );
    } else {
        CHECK(
            opts.retval as c_int != -ENOENT,
            b"bpf_perf_event_read_value\0".as_ptr() as *const c_char,
            b"should have failed with %d, but got %d\n\0".as_ptr() as *const c_char,
            -ENOENT,
            opts.retval as c_int,
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_pe_preserve_elems() {
    let mut skel: *mut test_pe_preserve_elems;

    skel = test_pe_preserve_elems__open_and_load();
    if CHECK(
        skel == ptr::null_mut(),
        b"skel_open\0".as_ptr() as *const c_char,
        b"failed to open skeleton\n\0".as_ptr() as *const c_char,
    ) {
        return;
    }

    test_one_map((*skel).maps.array_1, (*skel).progs.read_array_1, false);
    test_one_map((*skel).maps.array_2, (*skel).progs.read_array_2, true);

    test_pe_preserve_elems__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
