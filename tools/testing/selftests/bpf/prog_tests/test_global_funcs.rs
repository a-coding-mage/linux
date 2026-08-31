// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
/*
 * Translated from C. Original dependencies:
 * - <test_progs.h>
 * - test_global_func{1..17}.skel.h
 * - test_global_func_deep_stack.skel.h
 * - test_global_func_ctx_args.skel.h
 * - bpf/libbpf_internal.h
 * - btf_helpers.h
 */

use core::ffi::{c_char, c_int, c_void};

type __u32 = u32;
type __u64 = u64;

const BTF_KIND_PTR: c_int = 1;
const BTF_KIND_STRUCT: c_int = 4;
const BTF_KIND_FUNC: c_int = 12;

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_param {
    pub name_off: __u32,
    pub type_: __u32,
}

#[repr(C)]
pub struct btf_type {
    pub name_off: __u32,
    pub info: __u32,
    pub type_: __u32,
}

#[repr(C)]
pub struct bpf_func_info_min {
    pub insn_off: __u32,
    pub type_id: __u32,
}

#[repr(C)]
pub struct bpf_prog_info {
    pub type_: __u32,
    pub id: __u32,
    pub tag: [u8; 8],
    pub jited_prog_len: __u32,
    pub xlated_prog_len: __u32,
    pub jited_prog_insns: __u64,
    pub xlated_prog_insns: __u64,
    pub load_time: __u64,
    pub created_by_uid: __u32,
    pub nr_map_ids: __u32,
    pub map_ids: __u64,
    pub name: [c_char; 16],
    pub ifindex: __u32,
    pub gpl_compatible: __u32,
    pub netns_dev: __u64,
    pub netns_ino: __u64,
    pub nr_jited_ksyms: __u32,
    pub nr_jited_func_lens: __u32,
    pub jited_ksyms: __u64,
    pub jited_func_lens: __u64,
    pub btf_id: __u32,
    pub func_info_rec_size: __u32,
    pub func_info: __u64,
    pub nr_func_info: __u32,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_global_func_ctx_args_progs {
    pub arg_tag_ctx_perf: *mut bpf_program,
}

#[repr(C)]
pub struct test_global_func_ctx_args {
    pub progs: test_global_func_ctx_args_progs,
}

#[repr(align(8))]
struct AlignedFuncInfoBuf([u8; 1024]);

unsafe extern "C" {
    fn btf__type_by_id(btf: *const btf, type_id: __u32) -> *const btf_type;
    fn btf_kind(t: *const btf_type) -> c_int;
    fn btf_type_raw_dump(btf: *const btf, type_id: __u32) -> *const c_char;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_HAS_SUBSTR(s: *const c_char, substr: *const c_char, name: *const c_char) -> bool;
    fn btf__load_vmlinux_btf() -> *mut btf;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn btf__find_by_name_kind(btf: *const btf, name: *const c_char, kind: c_int) -> c_int;
    fn test__skip();
    fn btf__free(btf: *mut btf);
    fn test_global_func_ctx_args__open() -> *mut test_global_func_ctx_args;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn test_global_func_ctx_args__load(skel: *mut test_global_func_ctx_args) -> c_int;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ptr_to_u64(ptr: *const c_void) -> __u64;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_get_info_by_fd(fd: c_int, info: *mut bpf_prog_info, info_len: *mut __u32) -> c_int;
    fn btf__load_from_kernel_by_id(id: __u32) -> *mut btf;
    fn btf__name_by_offset(btf: *const btf, offset: __u32) -> *const c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn btf_vlen(t: *const btf_type) -> c_int;
    fn btf_params(t: *const btf_type) -> *const btf_param;
    fn ASSERT_FAIL(fmt: *const c_char, ...) -> bool;
    fn test_global_func_ctx_args__destroy(skel: *mut test_global_func_ctx_args);
    fn RUN_TESTS(name: *const c_char);
    fn test__start_subtest(name: *const c_char) -> bool;
}

unsafe fn check_ctx_arg_type(btf: *const btf, p: *const btf_param) {
    let t: *const btf_type;
    let s: *const c_char;

    t = btf__type_by_id(btf, (*p).type_);
    if !ASSERT_EQ(btf_kind(t), BTF_KIND_PTR, b"ptr_t\0".as_ptr() as *const c_char) {
        return;
    }

    s = btf_type_raw_dump(btf, (*t).type_);
    if !ASSERT_HAS_SUBSTR(
        s,
        b"STRUCT 'bpf_perf_event_data' size=0 vlen=0\0".as_ptr() as *const c_char,
        b"ctx_struct_t\0".as_ptr() as *const c_char,
    ) {
        return;
    }
}

unsafe fn subtest_ctx_arg_rewrite() {
    let mut skel: *mut test_global_func_ctx_args = core::ptr::null_mut();
    let mut info: bpf_prog_info = core::mem::zeroed();
    let mut func_info_buf = AlignedFuncInfoBuf([0; 1024]);
    let mut rec: *mut bpf_func_info_min;
    let mut btf: *mut btf = core::ptr::null_mut();
    let mut info_len: __u32 = core::mem::size_of::<bpf_prog_info>() as __u32;
    let mut err: c_int;
    let fd: c_int;
    let mut i: c_int;
    let mut kern_btf: *mut btf = core::ptr::null_mut();

    kern_btf = btf__load_vmlinux_btf();
    if !ASSERT_OK_PTR(kern_btf as *const c_void, b"kern_btf_load\0".as_ptr() as *const c_char) {
        return;
    }

    /* simple detection of kernel native arg:ctx tag support */
    if btf__find_by_name_kind(
        kern_btf,
        b"bpf_subprog_arg_info\0".as_ptr() as *const c_char,
        BTF_KIND_STRUCT,
    ) > 0
    {
        test__skip();
        btf__free(kern_btf);
        return;
    }
    btf__free(kern_btf);

    skel = test_global_func_ctx_args__open();
    if !ASSERT_OK_PTR(skel as *const c_void, b"skel_open\0".as_ptr() as *const c_char) {
        return;
    }

    bpf_program__set_autoload((*skel).progs.arg_tag_ctx_perf, true);

    err = test_global_func_ctx_args__load(skel);
    if !ASSERT_OK(err, b"skel_load\0".as_ptr() as *const c_char) {
        goto_out(btf, skel);
        return;
    }

    core::ptr::write_bytes(
        &mut info as *mut bpf_prog_info as *mut u8,
        0,
        core::mem::size_of::<bpf_prog_info>(),
    );
    info.func_info = ptr_to_u64(&func_info_buf.0 as *const [u8; 1024] as *const c_void);
    info.nr_func_info = 3;
    info.func_info_rec_size = core::mem::size_of::<bpf_func_info_min>() as __u32;

    fd = bpf_program__fd((*skel).progs.arg_tag_ctx_perf);
    err = bpf_prog_get_info_by_fd(fd, &mut info, &mut info_len);
    if !ASSERT_OK(err, b"prog_info\0".as_ptr() as *const c_char) {
        goto_out(btf, skel);
        return;
    }

    if !ASSERT_EQ(info.nr_func_info as c_int, 3, b"nr_func_info\0".as_ptr() as *const c_char) {
        goto_out(btf, skel);
        return;
    }

    btf = btf__load_from_kernel_by_id(info.btf_id);
    if !ASSERT_OK_PTR(btf as *const c_void, b"obj_kern_btf\0".as_ptr() as *const c_char) {
        goto_out(btf, skel);
        return;
    }

    rec = func_info_buf.0.as_mut_ptr() as *mut bpf_func_info_min;
    i = 0;
    while i < info.nr_func_info as c_int {
        let fn_t: *const btf_type;
        let proto_t: *const btf_type;
        let name: *const c_char;

        if (*rec).insn_off == 0 {
            i += 1;
            rec = (rec as *mut c_void as *mut u8).add(info.func_info_rec_size as usize)
                as *mut bpf_func_info_min;
            continue; /* main prog, skip */
        }

        fn_t = btf__type_by_id(btf, (*rec).type_id);
        if !ASSERT_OK_PTR(fn_t as *const c_void, b"fn_type\0".as_ptr() as *const c_char) {
            goto_out(btf, skel);
            return;
        }
        if !ASSERT_EQ(btf_kind(fn_t), BTF_KIND_FUNC, b"fn_type_kind\0".as_ptr() as *const c_char) {
            goto_out(btf, skel);
            return;
        }
        proto_t = btf__type_by_id(btf, (*fn_t).type_);
        if !ASSERT_OK_PTR(proto_t as *const c_void, b"proto_type\0".as_ptr() as *const c_char) {
            goto_out(btf, skel);
            return;
        }

        name = btf__name_by_offset(btf, (*fn_t).name_off);
        if strcmp(name, b"subprog_ctx_tag\0".as_ptr() as *const c_char) == 0 {
            /* int subprog_ctx_tag(void *ctx __arg_ctx) */
            if !ASSERT_EQ(btf_vlen(proto_t), 1, b"arg_cnt\0".as_ptr() as *const c_char) {
                goto_out(btf, skel);
                return;
            }

            /* arg 0 is PTR -> STRUCT bpf_perf_event_data */
            check_ctx_arg_type(btf, btf_params(proto_t).add(0));
        } else if strcmp(name, b"subprog_multi_ctx_tags\0".as_ptr() as *const c_char) == 0 {
            /* int subprog_multi_ctx_tags(void *ctx1 __arg_ctx,
             *			      struct my_struct *mem,
             *			      void *ctx2 __arg_ctx)
             */
            if !ASSERT_EQ(btf_vlen(proto_t), 3, b"arg_cnt\0".as_ptr() as *const c_char) {
                goto_out(btf, skel);
                return;
            }

            /* arg 0 is PTR -> STRUCT bpf_perf_event_data */
            check_ctx_arg_type(btf, btf_params(proto_t).add(0));
            /* arg 2 is PTR -> STRUCT bpf_perf_event_data */
            check_ctx_arg_type(btf, btf_params(proto_t).add(2));
        } else {
            ASSERT_FAIL(b"unexpected subprog %s\0".as_ptr() as *const c_char, name);
            goto_out(btf, skel);
            return;
        }

        i += 1;
        rec = (rec as *mut c_void as *mut u8).add(info.func_info_rec_size as usize)
            as *mut bpf_func_info_min;
    }

    goto_out(btf, skel);
}

unsafe fn goto_out(btf: *mut btf, skel: *mut test_global_func_ctx_args) {
    btf__free(btf);
    test_global_func_ctx_args__destroy(skel);
}

pub unsafe extern "C" fn test_test_global_funcs() {
    RUN_TESTS(b"test_global_func1\0".as_ptr() as *const c_char);
    RUN_TESTS(b"test_global_func2\0".as_ptr() as *const c_char);
    RUN_TESTS(b"test_global_func3\0".as_ptr() as *const c_char);
    RUN_TESTS(b"test_global_func4\0".as_ptr() as *const c_char);
    RUN_TESTS(b"test_global_func5\0".as_ptr() as *const c_char);
    RUN_TESTS(b"test_global_func6\0".as_ptr() as *const c_char);
    RUN_TESTS(b"test_global_func7\0".as_ptr() as *const c_char);
    RUN_TESTS(b"test_global_func8\0".as_ptr() as *const c_char);
    RUN_TESTS(b"test_global_func9\0".as_ptr() as *const c_char);
    RUN_TESTS(b"test_global_func10\0".as_ptr() as *const c_char);
    RUN_TESTS(b"test_global_func11\0".as_ptr() as *const c_char);
    RUN_TESTS(b"test_global_func12\0".as_ptr() as *const c_char);
    RUN_TESTS(b"test_global_func13\0".as_ptr() as *const c_char);
    RUN_TESTS(b"test_global_func14\0".as_ptr() as *const c_char);
    RUN_TESTS(b"test_global_func15\0".as_ptr() as *const c_char);
    RUN_TESTS(b"test_global_func16\0".as_ptr() as *const c_char);
    RUN_TESTS(b"test_global_func17\0".as_ptr() as *const c_char);
    RUN_TESTS(b"test_global_func_deep_stack\0".as_ptr() as *const c_char);
    RUN_TESTS(b"test_global_func_ctx_args\0".as_ptr() as *const c_char);

    if test__start_subtest(b"ctx_arg_rewrite\0".as_ptr() as *const c_char) {
        subtest_ctx_arg_rewrite();
    }
}
