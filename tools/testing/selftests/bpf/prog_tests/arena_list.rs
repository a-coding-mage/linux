// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

/*
 * Translated from C. External libbpf/test harness declarations and generated
 * skeleton types are supplied by the surrounding test environment.
 */

type __u64 = u64;
type u64 = u64;

#[repr(C)]
pub struct arena_list_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct arena_list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct elem {
    pub node: arena_list_node,
    pub value: __u64,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub retval: i32,
}

#[repr(C)]
pub struct arena_list_rodata {
    pub nonsleepable: bool,
}

#[repr(C)]
pub struct arena_list_bss {
    pub cnt: i32,
    pub skip: bool,
    pub list_head: *mut arena_list_head,
    pub list_sum: i32,
}

#[repr(C)]
pub struct arena_list_arena {
    pub arena_sum: i32,
    pub test_val: i32,
}

#[repr(C)]
pub struct arena_list_progs {
    pub arena_list_add: *mut bpf_program,
    pub arena_list_del: *mut bpf_program,
}

#[repr(C)]
pub struct arena_list {
    pub rodata: *mut arena_list_rodata,
    pub bss: *mut arena_list_bss,
    pub arena: *mut arena_list_arena,
    pub progs: arena_list_progs,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn arena_list__open() -> *mut arena_list;
    fn arena_list__load(skel: *mut arena_list) -> i32;
    fn arena_list__destroy(skel: *mut arena_list);
    fn bpf_program__fd(prog: *mut bpf_program) -> i32;
    fn bpf_prog_test_run_opts(fd: i32, opts: *mut bpf_test_run_opts) -> i32;

    fn ASSERT_OK_PTR(ptr: *const core::ffi::c_void, name: *const i8) -> bool;
    fn ASSERT_OK(ret: i32, name: *const i8) -> bool;
    fn ASSERT_EQ(actual: i32, expected: i32, name: *const i8) -> bool;
    fn test__skip();
    fn test__start_subtest(name: *const i8) -> bool;
    fn printf(fmt: *const i8, ...);
}

/*
 * C used list_for_each_entry(n, head, node) over arena_list_head. The concrete
 * intrusive-list iterator is provided by bpf_arena_list.h in the original
 * source, so this declaration preserves the file-local use without
 * reimplementing that dependency here.
 */
unsafe extern "C" {
    fn arena_list_first_entry(head: *mut arena_list_head) -> *mut elem;
    fn arena_list_next_entry(n: *mut elem, head: *mut arena_list_head) -> *mut elem;
}

unsafe fn list_sum(head: *mut arena_list_head) -> i32 {
    let mut n: *mut elem;
    let mut sum: i32 = 0;

    n = arena_list_first_entry(head);
    while !n.is_null() {
        sum += (*n).value as i32;
        n = arena_list_next_entry(n, head);
    }
    sum
}

unsafe fn test_arena_list_add_del(cnt: i32, nonsleepable: bool) {
    let mut opts: bpf_test_run_opts = core::mem::zeroed();
    let skel: *mut arena_list;
    let expected_sum: i32 = (cnt as u64 * (cnt - 1) as u64 / 2) as i32;
    let mut ret: i32;
    let mut sum: i32;

    skel = arena_list__open();
    if !ASSERT_OK_PTR(skel as *const core::ffi::c_void, c"arena_list__open".as_ptr()) {
        return;
    }

    (*(*skel).rodata).nonsleepable = nonsleepable;

    ret = arena_list__load(skel);
    if !ASSERT_OK(ret, c"arena_list__load".as_ptr()) {
        arena_list__destroy(skel);
        return;
    }

    (*(*skel).bss).cnt = cnt;
    ret = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.arena_list_add), &mut opts);
    ASSERT_OK(ret, c"ret_add".as_ptr());
    ASSERT_OK(opts.retval, c"retval".as_ptr());
    if (*(*skel).bss).skip {
        printf(
            c"%s:SKIP:compiler doesn't support arena_cast\n".as_ptr(),
            c"test_arena_list_add_del".as_ptr(),
        );
        test__skip();
        arena_list__destroy(skel);
        return;
    }
    sum = list_sum((*(*skel).bss).list_head);
    ASSERT_EQ(sum, expected_sum, c"sum of elems".as_ptr());
    ASSERT_EQ(
        (*(*skel).arena).arena_sum,
        expected_sum,
        c"__arena sum of elems".as_ptr(),
    );
    ASSERT_EQ((*(*skel).arena).test_val, cnt + 1, c"num of elems".as_ptr());

    ret = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.arena_list_del), &mut opts);
    ASSERT_OK(ret, c"ret_del".as_ptr());
    sum = list_sum((*(*skel).bss).list_head);
    ASSERT_EQ(sum, 0, c"sum of list elems after del".as_ptr());
    ASSERT_EQ(
        (*(*skel).bss).list_sum,
        expected_sum,
        c"sum of list elems computed by prog".as_ptr(),
    );
    ASSERT_EQ(
        (*(*skel).arena).arena_sum,
        expected_sum,
        c"__arena sum of elems".as_ptr(),
    );

    arena_list__destroy(skel);
}

pub unsafe fn serial_test_arena_list() {
    if test__start_subtest(c"arena_list_1".as_ptr()) {
        test_arena_list_add_del(1, false);
    }
    if test__start_subtest(c"arena_list_1000".as_ptr()) {
        test_arena_list_add_del(1000, false);
    }
    if test__start_subtest(c"arena_list_1_nonsleepable".as_ptr()) {
        test_arena_list_add_del(1, true);
    }
    if test__start_subtest(c"arena_list_1000_nonsleepable".as_ptr()) {
        test_arena_list_add_del(1000, true);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
