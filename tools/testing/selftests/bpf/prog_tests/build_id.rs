// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */
// C dependencies translated from:
// #include <test_progs.h>
// #include "test_build_id.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};

const BPF_BUILD_ID_SIZE: usize = 20;
const BPF_STACK_BUILD_ID_EMPTY: c_int = 0;
const BPF_STACK_BUILD_ID_VALID: c_int = 1;
const BPF_STACK_BUILD_ID_IP: c_int = 2;
const VERBOSE_NORMAL: c_int = 1;

#[repr(C)]
pub struct bpf_stack_build_id {
    pub status: c_int,
    pub build_id: [u8; BPF_BUILD_ID_SIZE],
    pub offset: u64,
    pub ip: u64,
}

#[repr(C)]
pub struct test_build_id {
    pub progs: test_build_id_progs,
    pub links: test_build_id_links,
    pub bss: *mut test_build_id_bss,
}

#[repr(C)]
pub struct test_build_id_progs {
    pub uprobe_nofault: *mut bpf_program,
    pub uprobe_sleepable: *mut bpf_program,
}

#[repr(C)]
pub struct test_build_id_links {
    pub uprobe_nofault: *mut bpf_link,
    pub uprobe_sleepable: *mut bpf_link,
}

#[repr(C)]
pub struct test_build_id_bss {
    pub res_nofault: c_int,
    pub stack_nofault: *mut bpf_stack_build_id,
    pub res_sleepable: c_int,
    pub stack_sleepable: *mut bpf_stack_build_id,
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
pub struct test_env {
    pub verbosity: c_int,
}

static mut build_id: [c_char; BPF_BUILD_ID_SIZE] = [0; BPF_BUILD_ID_SIZE];
static mut build_id_sz: c_int = 0;

unsafe extern "C" {
    static mut env: test_env;

    fn printf(format: *const c_char, ...) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;

    fn read_build_id(filename: *const c_char, build_id: *mut c_char, size: usize) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;

    fn test_build_id__open_and_load() -> *mut test_build_id;
    fn test_build_id__destroy(obj: *mut test_build_id);
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_ulonglong, expected: c_ulonglong, name: *const c_char) -> bool;
}

unsafe fn print_stack(stack: *mut bpf_stack_build_id, frame_cnt: c_int) {
    let mut i: c_int;
    let mut j: c_int;

    i = 0;
    while i < frame_cnt {
        printf(b"FRAME #%02d: \0".as_ptr() as *const c_char, i);
        match (*stack.add(i as usize)).status {
            BPF_STACK_BUILD_ID_EMPTY => {
                printf(b"<EMPTY>\n\0".as_ptr() as *const c_char);
            }
            BPF_STACK_BUILD_ID_VALID => {
                printf(b"BUILD ID = \0".as_ptr() as *const c_char);
                j = 0;
                while j < BPF_BUILD_ID_SIZE as c_int {
                    printf(
                        b"%02hhx\0".as_ptr() as *const c_char,
                        (*stack.add(i as usize)).build_id[j as usize] as c_uint,
                    );
                    j += 1;
                }
                printf(
                    b" OFFSET = %llx\0".as_ptr() as *const c_char,
                    (*stack.add(i as usize)).offset as c_ulonglong,
                );
            }
            BPF_STACK_BUILD_ID_IP => {
                printf(
                    b"IP = %llx\0".as_ptr() as *const c_char,
                    (*stack.add(i as usize)).ip as c_ulonglong,
                );
            }
            _ => {
                printf(
                    b"UNEXPECTED STATUS %d \0".as_ptr() as *const c_char,
                    (*stack.add(i as usize)).status,
                );
            }
        }
        printf(b"\n\0".as_ptr() as *const c_char);
        i += 1;
    }
}

unsafe fn subtest_nofault(build_id_resident: bool) {
    let skel: *mut test_build_id;
    let stack: *mut bpf_stack_build_id;
    let frame_cnt: c_int;

    skel = test_build_id__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, b"skel_open\0".as_ptr() as *const c_char) {
        return;
    }

    (*skel).links.uprobe_nofault = bpf_program__attach((*skel).progs.uprobe_nofault);
    if !ASSERT_OK_PTR(
        (*skel).links.uprobe_nofault as *const c_void,
        b"link\0".as_ptr() as *const c_char,
    ) {
        test_build_id__destroy(skel);
        return;
    }

    if build_id_resident {
        ASSERT_OK(
            system(b"./uprobe_multi uprobe-paged-in\0".as_ptr() as *const c_char),
            b"trigger_uprobe\0".as_ptr() as *const c_char,
        );
    } else {
        ASSERT_OK(
            system(b"./uprobe_multi uprobe-paged-out\0".as_ptr() as *const c_char),
            b"trigger_uprobe\0".as_ptr() as *const c_char,
        );
    }

    if !ASSERT_GT((*(*skel).bss).res_nofault, 0, b"res\0".as_ptr() as *const c_char) {
        test_build_id__destroy(skel);
        return;
    }

    stack = (*(*skel).bss).stack_nofault;
    frame_cnt = (*(*skel).bss).res_nofault / core::mem::size_of::<bpf_stack_build_id>() as c_int;
    if env.verbosity >= VERBOSE_NORMAL {
        print_stack(stack, frame_cnt);
    }

    if build_id_resident {
        ASSERT_EQ(
            (*stack.add(0)).status as c_ulonglong,
            BPF_STACK_BUILD_ID_VALID as c_ulonglong,
            b"build_id_status\0".as_ptr() as *const c_char,
        );
        ASSERT_EQ(
            memcmp(
                (*stack.add(0)).build_id.as_ptr() as *const c_void,
                build_id.as_ptr() as *const c_void,
                build_id_sz as usize,
            ) as c_ulonglong,
            0,
            b"build_id_match\0".as_ptr() as *const c_char,
        );
    } else {
        ASSERT_EQ(
            (*stack.add(0)).status as c_ulonglong,
            BPF_STACK_BUILD_ID_IP as c_ulonglong,
            b"build_id_status\0".as_ptr() as *const c_char,
        );
    }

    test_build_id__destroy(skel);
}

unsafe fn subtest_sleepable() {
    let skel: *mut test_build_id;
    let stack: *mut bpf_stack_build_id;
    let frame_cnt: c_int;

    skel = test_build_id__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, b"skel_open\0".as_ptr() as *const c_char) {
        return;
    }

    (*skel).links.uprobe_sleepable = bpf_program__attach((*skel).progs.uprobe_sleepable);
    if !ASSERT_OK_PTR(
        (*skel).links.uprobe_sleepable as *const c_void,
        b"link\0".as_ptr() as *const c_char,
    ) {
        test_build_id__destroy(skel);
        return;
    }

    /* force build ID to not be paged in */
    ASSERT_OK(
        system(b"./uprobe_multi uprobe-paged-out\0".as_ptr() as *const c_char),
        b"trigger_uprobe\0".as_ptr() as *const c_char,
    );

    if !ASSERT_GT((*(*skel).bss).res_sleepable, 0, b"res\0".as_ptr() as *const c_char) {
        test_build_id__destroy(skel);
        return;
    }

    stack = (*(*skel).bss).stack_sleepable;
    frame_cnt = (*(*skel).bss).res_sleepable / core::mem::size_of::<bpf_stack_build_id>() as c_int;
    if env.verbosity >= VERBOSE_NORMAL {
        print_stack(stack, frame_cnt);
    }

    ASSERT_EQ(
        (*stack.add(0)).status as c_ulonglong,
        BPF_STACK_BUILD_ID_VALID as c_ulonglong,
        b"build_id_status\0".as_ptr() as *const c_char,
    );
    ASSERT_EQ(
        memcmp(
            (*stack.add(0)).build_id.as_ptr() as *const c_void,
            build_id.as_ptr() as *const c_void,
            build_id_sz as usize,
        ) as c_ulonglong,
        0,
        b"build_id_match\0".as_ptr() as *const c_char,
    );

    test_build_id__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn serial_test_build_id() {
    build_id_sz = read_build_id(
        b"uprobe_multi\0".as_ptr() as *const c_char,
        build_id.as_mut_ptr(),
        core::mem::size_of_val(&build_id),
    );
    ASSERT_EQ(
        build_id_sz as c_ulonglong,
        BPF_BUILD_ID_SIZE as c_ulonglong,
        b"parse_build_id\0".as_ptr() as *const c_char,
    );

    if test__start_subtest(b"nofault-paged-out\0".as_ptr() as *const c_char) {
        subtest_nofault(false /* not resident */);
    }
    if test__start_subtest(b"nofault-paged-in\0".as_ptr() as *const c_char) {
        subtest_nofault(true /* resident */);
    }
    if test__start_subtest(b"sleepable\0".as_ptr() as *const c_char) {
        subtest_sleepable();
    }
}
