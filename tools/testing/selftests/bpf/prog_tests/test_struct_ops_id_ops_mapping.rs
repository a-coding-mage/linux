// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <test_progs.h>
// #include "struct_ops_id_ops_mapping1.skel.h"
// #include "struct_ops_id_ops_mapping2.skel.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type __u32 = u32;

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map_info {
    pub type_: __u32,
    pub id: __u32,
}

#[repr(C)]
pub struct struct_ops_id_ops_mapping1_maps {
    pub st_ops_map: *mut bpf_map,
}

#[repr(C)]
pub struct struct_ops_id_ops_mapping1_progs {
    pub syscall_prog: *mut bpf_program,
}

#[repr(C)]
pub struct struct_ops_id_ops_mapping1_bss {
    pub st_ops_id: __u32,
    pub test_pid: c_int,
    pub test_err: c_int,
}

#[repr(C)]
pub struct struct_ops_id_ops_mapping1 {
    pub maps: struct_ops_id_ops_mapping1_maps,
    pub progs: struct_ops_id_ops_mapping1_progs,
    pub bss: *mut struct_ops_id_ops_mapping1_bss,
}

#[repr(C)]
pub struct struct_ops_id_ops_mapping2_maps {
    pub st_ops_map: *mut bpf_map,
}

#[repr(C)]
pub struct struct_ops_id_ops_mapping2_progs {
    pub syscall_prog: *mut bpf_program,
}

#[repr(C)]
pub struct struct_ops_id_ops_mapping2_bss {
    pub st_ops_id: __u32,
    pub test_pid: c_int,
    pub test_err: c_int,
}

#[repr(C)]
pub struct struct_ops_id_ops_mapping2 {
    pub maps: struct_ops_id_ops_mapping2_maps,
    pub progs: struct_ops_id_ops_mapping2_progs,
    pub bss: *mut struct_ops_id_ops_mapping2_bss,
}

unsafe extern "C" {
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;

    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_map_get_info_by_fd(fd: c_int, info: *mut bpf_map_info, len: *mut __u32) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut c_void) -> c_int;

    fn getpid() -> c_int;
    fn sys_gettid() -> c_int;

    fn struct_ops_id_ops_mapping1__open_and_load() -> *mut struct_ops_id_ops_mapping1;
    fn struct_ops_id_ops_mapping1__attach(skel: *mut struct_ops_id_ops_mapping1) -> c_int;
    fn struct_ops_id_ops_mapping1__destroy(skel: *mut struct_ops_id_ops_mapping1);

    fn struct_ops_id_ops_mapping2__open_and_load() -> *mut struct_ops_id_ops_mapping2;
    fn struct_ops_id_ops_mapping2__attach(skel: *mut struct_ops_id_ops_mapping2) -> c_int;
    fn struct_ops_id_ops_mapping2__destroy(skel: *mut struct_ops_id_ops_mapping2);
}

unsafe fn test_st_ops_id_ops_mapping() {
    let mut skel1: *mut struct_ops_id_ops_mapping1 = ptr::null_mut();
    let mut skel2: *mut struct_ops_id_ops_mapping2 = ptr::null_mut();
    let mut info: bpf_map_info = core::mem::zeroed();
    let mut len: __u32 = size_of::<bpf_map_info>() as __u32;
    let mut err: c_int;
    let pid: c_int;
    let prog1_fd: c_int;
    let prog2_fd: c_int;

    skel1 = struct_ops_id_ops_mapping1__open_and_load();
    if !ASSERT_OK_PTR(
        skel1 as *const c_void,
        c"struct_ops_id_ops_mapping1__open".as_ptr(),
    ) {
        goto_out(skel1, skel2);
        return;
    }

    skel2 = struct_ops_id_ops_mapping2__open_and_load();
    if !ASSERT_OK_PTR(
        skel2 as *const c_void,
        c"struct_ops_id_ops_mapping2__open".as_ptr(),
    ) {
        goto_out(skel1, skel2);
        return;
    }

    err = bpf_map_get_info_by_fd(
        bpf_map__fd((*skel1).maps.st_ops_map),
        &mut info,
        &mut len,
    );
    if !ASSERT_OK(err, c"bpf_map_get_info_by_fd".as_ptr()) {
        goto_out(skel1, skel2);
        return;
    }

    (*(*skel1).bss).st_ops_id = info.id;

    err = bpf_map_get_info_by_fd(
        bpf_map__fd((*skel2).maps.st_ops_map),
        &mut info,
        &mut len,
    );
    if !ASSERT_OK(err, c"bpf_map_get_info_by_fd".as_ptr()) {
        goto_out(skel1, skel2);
        return;
    }

    (*(*skel2).bss).st_ops_id = info.id;

    err = struct_ops_id_ops_mapping1__attach(skel1);
    if !ASSERT_OK(err, c"struct_ops_id_ops_mapping1__attach".as_ptr()) {
        goto_out(skel1, skel2);
        return;
    }

    err = struct_ops_id_ops_mapping2__attach(skel2);
    if !ASSERT_OK(err, c"struct_ops_id_ops_mapping2__attach".as_ptr()) {
        goto_out(skel1, skel2);
        return;
    }

    /* run tracing prog that calls .test_1 and checks return */
    pid = getpid();
    (*(*skel1).bss).test_pid = pid;
    (*(*skel2).bss).test_pid = pid;
    sys_gettid();
    (*(*skel1).bss).test_pid = 0;
    (*(*skel2).bss).test_pid = 0;

    /* run syscall_prog that calls .test_1 and checks return */
    prog1_fd = bpf_program__fd((*skel1).progs.syscall_prog);
    err = bpf_prog_test_run_opts(prog1_fd, ptr::null_mut());
    ASSERT_OK(err, c"bpf_prog_test_run_opts".as_ptr());

    prog2_fd = bpf_program__fd((*skel2).progs.syscall_prog);
    err = bpf_prog_test_run_opts(prog2_fd, ptr::null_mut());
    ASSERT_OK(err, c"bpf_prog_test_run_opts".as_ptr());

    ASSERT_EQ(
        (*(*skel1).bss).test_err,
        0,
        c"skel1->bss->test_err".as_ptr(),
    );
    ASSERT_EQ(
        (*(*skel2).bss).test_err,
        0,
        c"skel2->bss->test_err".as_ptr(),
    );

    goto_out(skel1, skel2);
}

unsafe fn goto_out(
    skel1: *mut struct_ops_id_ops_mapping1,
    skel2: *mut struct_ops_id_ops_mapping2,
) {
    struct_ops_id_ops_mapping1__destroy(skel1);
    struct_ops_id_ops_mapping2__destroy(skel2);
}

#[no_mangle]
pub unsafe extern "C" fn test_struct_ops_id_ops_mapping() {
    if test__start_subtest(c"st_ops_id_ops_mapping".as_ptr()) {
        test_st_ops_id_ops_mapping();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
