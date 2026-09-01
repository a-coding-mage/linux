// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023. Huawei Technologies Co., Ltd */

// C dependencies removed from executable Rust:
// sched.h, pthread.h, stdbool.h, bpf/btf.h, test_progs.h,
// and the generated "test_bpf_ma.skel.h".

use core::ffi::{c_char, c_int, c_uint, c_void};

const BTF_KIND_STRUCT: c_uint = 4;

// Generated skeleton array sizes come from "test_bpf_ma.skel.h" through
// ARRAY_SIZE() in the C source and are not defined in this isolated file.
const TEST_BPF_MA_DATA_SIZES_LEN: usize = 0;
const TEST_BPF_MA_PERCPU_DATA_SIZES_LEN: usize = 0;

#[repr(C)]
pub struct test_bpf_ma {
    pub obj: *mut bpf_object,
    pub rodata: *mut test_bpf_ma_rodata,
    pub bss: *mut test_bpf_ma_bss,
}

#[repr(C)]
pub struct test_bpf_ma_rodata {
    pub data_sizes: *mut c_uint,
    pub data_btf_ids: *mut c_int,
    pub percpu_data_sizes: *mut c_uint,
    pub percpu_data_btf_ids: *mut c_int,
}

#[repr(C)]
pub struct test_bpf_ma_bss {
    pub pid: c_int,
    pub err: c_int,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn test_bpf_ma__open() -> *mut test_bpf_ma;
    fn test_bpf_ma__load(skel: *mut test_bpf_ma) -> c_int;
    fn test_bpf_ma__attach(skel: *mut test_bpf_ma) -> c_int;
    fn test_bpf_ma__destroy(skel: *mut test_bpf_ma);

    fn bpf_object__btf(obj: *mut bpf_object) -> *mut btf;
    fn btf__find_by_name_kind(btf: *mut btf, name: *const c_char, kind: c_uint) -> c_int;
    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);

    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_GT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;

    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn getpid() -> c_int;
    fn usleep(usec: c_uint) -> c_int;
}

unsafe fn do_bpf_ma_test(name: *const c_char) {
    let mut skel: *mut test_bpf_ma;
    let mut prog: *mut bpf_program;
    let mut btf: *mut btf;
    let mut i: c_int;
    let mut err: c_int;
    let mut id: c_int;
    let mut tname = [0 as c_char; 32];

    skel = test_bpf_ma__open();
    if !ASSERT_OK_PTR(skel as *mut c_void, c"open".as_ptr()) {
        return;
    }

    btf = bpf_object__btf((*skel).obj);
    if !ASSERT_OK_PTR(btf as *mut c_void, c"btf".as_ptr()) {
        goto_out(skel);
        return;
    }

    i = 0;
    while (i as usize) < TEST_BPF_MA_DATA_SIZES_LEN {
        snprintf(
            tname.as_mut_ptr(),
            tname.len(),
            c"bin_data_%u".as_ptr(),
            *(*(*skel).rodata).data_sizes.add(i as usize),
        );
        id = btf__find_by_name_kind(btf, tname.as_ptr(), BTF_KIND_STRUCT);
        if !ASSERT_GT(id, 0, tname.as_ptr()) {
            goto_out(skel);
            return;
        }
        *(*(*skel).rodata).data_btf_ids.add(i as usize) = id;
        i += 1;
    }

    i = 0;
    while (i as usize) < TEST_BPF_MA_PERCPU_DATA_SIZES_LEN {
        snprintf(
            tname.as_mut_ptr(),
            tname.len(),
            c"percpu_bin_data_%u".as_ptr(),
            *(*(*skel).rodata).percpu_data_sizes.add(i as usize),
        );
        id = btf__find_by_name_kind(btf, tname.as_ptr(), BTF_KIND_STRUCT);
        if !ASSERT_GT(id, 0, tname.as_ptr()) {
            goto_out(skel);
            return;
        }
        *(*(*skel).rodata).percpu_data_btf_ids.add(i as usize) = id;
        i += 1;
    }

    prog = bpf_object__find_program_by_name((*skel).obj, name);
    if !ASSERT_OK_PTR(prog as *mut c_void, c"invalid prog name".as_ptr()) {
        goto_out(skel);
        return;
    }
    bpf_program__set_autoload(prog, true);

    err = test_bpf_ma__load(skel);
    if !ASSERT_OK(err, c"load".as_ptr()) {
        goto_out(skel);
        return;
    }

    err = test_bpf_ma__attach(skel);
    if !ASSERT_OK(err, c"attach".as_ptr()) {
        goto_out(skel);
        return;
    }

    (*(*skel).bss).pid = getpid();
    usleep(1);
    ASSERT_OK((*(*skel).bss).err, c"test error".as_ptr());

    goto_out(skel);
}

unsafe fn goto_out(skel: *mut test_bpf_ma) {
    test_bpf_ma__destroy(skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_test_bpf_ma() {
    if test__start_subtest(c"batch_alloc_free".as_ptr()) {
        do_bpf_ma_test(c"test_batch_alloc_free".as_ptr());
    }
    if test__start_subtest(c"free_through_map_free".as_ptr()) {
        do_bpf_ma_test(c"test_free_through_map_free".as_ptr());
    }
    if test__start_subtest(c"batch_percpu_alloc_free".as_ptr()) {
        do_bpf_ma_test(c"test_batch_percpu_alloc_free".as_ptr());
    }
    if test__start_subtest(c"percpu_free_through_map_free".as_ptr()) {
        do_bpf_ma_test(c"test_percpu_free_through_map_free".as_ptr());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
