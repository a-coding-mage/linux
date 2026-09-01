// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook */
/* Translated from testing/selftests/bpf/prog_tests/fexit_bpf2bpf.c. */
/* C includes removed: test_progs.h, network_helpers.h, bpf/btf.h,
 * bind4_prog.skel.h, freplace_progmap.skel.h, fentry_sleepable.skel.h,
 * xdp_dummy.skel.h.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type __u32 = u32;
type __u64 = u64;
type __s32 = i32;
type bool_t = bool;
type test_cb = unsafe extern "C" fn(obj: *mut bpf_object) -> c_int;

const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const EINVAL: c_int = 22;
const BPF_PROG_TYPE_UNSPEC: c_int = 0;
const BPF_PROG_TYPE_TRACING: c_int = 26;
const BPF_TRACE_FENTRY: c_int = 2;
const BTF_KIND_FUNC: c_int = 12;
const VERBOSE_NONE: c_int = 0;
const BPF_REG_0: c_int = 0;

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
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
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_insn {
    pub code: u8,
    pub dst_src: u8,
    pub off: i16,
    pub imm: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_info {
    pub id: __u32,
    pub btf_id: __u32,
    pub attach_btf_id: __u32,
    pub attach_btf_obj_id: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_link_info_tracing {
    pub attach_type: __u32,
    pub target_obj_id: __u32,
    pub target_btf_id: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_link_info {
    pub tracing: bpf_link_info_tracing,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub data_in: *const c_void,
    pub data_size_in: __u32,
    pub repeat: __u32,
    pub retval: __u32,
}

#[repr(C)]
pub struct bpf_prog_load_opts {
    pub sz: usize,
    pub expected_attach_type: c_int,
    pub attach_prog_fd: c_int,
    pub attach_btf_id: c_int,
}

#[repr(C)]
pub struct bpf_cpumap_val_bpf_prog {
    pub fd: c_int,
}

#[repr(C)]
pub struct bpf_cpumap_val {
    pub qsize: __u32,
    pub bpf_prog: bpf_cpumap_val_bpf_prog,
}

#[repr(C)]
pub struct bind4_prog_progs {
    pub bind_v4_prog: *mut bpf_program,
}

#[repr(C)]
pub struct bind4_prog_links {
    pub bind_v4_prog: *mut bpf_link,
}

#[repr(C)]
pub struct bind4_prog {
    pub progs: bind4_prog_progs,
    pub links: bind4_prog_links,
}

#[repr(C)]
pub struct freplace_progmap_progs {
    pub xdp_cpumap_prog: *mut bpf_program,
    pub xdp_drop_prog: *mut bpf_program,
}

#[repr(C)]
pub struct freplace_progmap_maps {
    pub cpu_map: *mut bpf_map,
}

#[repr(C)]
pub struct freplace_progmap {
    pub progs: freplace_progmap_progs,
    pub maps: freplace_progmap_maps,
}

#[repr(C)]
pub struct fentry_sleepable_bss {
    pub user_ptr: *mut c_char,
    pub retval: c_int,
}

#[repr(C)]
pub struct fentry_sleepable_progs {
    pub fentry_xdp: *mut bpf_program,
}

#[repr(C)]
pub struct fentry_sleepable_links {
    pub fentry_xdp: *mut bpf_link,
}

#[repr(C)]
pub struct fentry_sleepable {
    pub bss: *mut fentry_sleepable_bss,
    pub progs: fentry_sleepable_progs,
    pub links: fentry_sleepable_links,
}

#[repr(C)]
pub struct xdp_dummy_progs {
    pub xdp_dummy_prog: *mut bpf_program,
    pub __x64_sys_nop: *mut bpf_program,
}

#[repr(C)]
pub struct xdp_dummy_links {
    pub __x64_sys_nop: *mut bpf_link,
}

#[repr(C)]
pub struct xdp_dummy {
    pub progs: xdp_dummy_progs,
    pub links: xdp_dummy_links,
}

#[repr(C)]
pub struct test_env {
    pub verbosity: c_int,
}

unsafe extern "C" {
    static pkt_v6: [u8; 0];
    static mut env: test_env;
    static mut errno: c_int;

    fn malloc(size: usize) -> *mut c_void;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;

    fn bpf_object__next_map(obj: *mut bpf_object, prev: *mut bpf_map) -> *mut bpf_map;
    fn bpf_map__is_internal(map: *mut bpf_map) -> bool_t;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: __u64) -> c_int;
    fn bpf_prog_test_load(file: *const c_char, ty: c_int, obj: *mut *mut bpf_object, fd: *mut c_int) -> c_int;
    fn bpf_prog_get_info_by_fd(fd: c_int, info: *mut bpf_prog_info, info_len: *mut __u32) -> c_int;
    fn bpf_object__btf(obj: *mut bpf_object) -> *mut btf;
    fn bpf_object__open_file(path: *const c_char, opts: *const c_void) -> *mut bpf_object;
    fn bpf_object__next_program(obj: *mut bpf_object, prev: *mut bpf_program) -> *mut bpf_program;
    fn bpf_program__set_attach_target(prog: *mut bpf_program, fd: c_int, name: *const c_char) -> c_int;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_program__section_name(prog: *mut bpf_program) -> *const c_char;
    fn bpf_program__attach_trace(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_link__fd(link: *mut bpf_link) -> c_int;
    fn bpf_link_get_info_by_fd(fd: c_int, info: *mut bpf_link_info, info_len: *mut __u32) -> c_int;
    fn bpf_program__expected_attach_type(prog: *mut bpf_program) -> __u32;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_object__close(obj: *mut bpf_object);
    fn bpf_object__find_program_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_program;
    fn bpf_program__attach_freplace(prog: *mut bpf_program, target_fd: c_int, attach_func_name: *const c_char) -> *mut bpf_link;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_program__set_log_buf(prog: *mut bpf_program, log_buf: *mut c_char, log_size: usize);
    fn bpf_program__set_log_level(prog: *mut bpf_program, level: c_int);
    fn btf__find_by_name_kind(btf: *mut btf, name: *const c_char, kind: c_int) -> __s32;
    fn btf__load_from_kernel_by_id(id: __u32) -> *mut btf;
    fn libbpf_get_error(ptr: *const c_void) -> c_int;
    fn btf__free(btf: *mut btf);
    fn bpf_prog_load(prog_type: c_int, prog_name: *const c_char, license: *const c_char, insns: *const bpf_insn, insn_cnt: usize, opts: *const bpf_prog_load_opts) -> c_int;
    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;
    fn bind4_prog__open_and_load() -> *mut bind4_prog;
    fn bind4_prog__destroy(obj: *mut bind4_prog);
    fn freplace_progmap__open() -> *mut freplace_progmap;
    fn freplace_progmap__load(obj: *mut freplace_progmap) -> c_int;
    fn freplace_progmap__destroy(obj: *mut freplace_progmap);
    fn xdp_dummy__open_and_load() -> *mut xdp_dummy;
    fn xdp_dummy__destroy(obj: *mut xdp_dummy);
    fn fentry_sleepable__open() -> *mut fentry_sleepable;
    fn fentry_sleepable__load(obj: *mut fentry_sleepable) -> c_int;
    fn fentry_sleepable__destroy(obj: *mut fentry_sleepable);
    fn bpf_program__attach_xdp(prog: *mut bpf_program, ifindex: c_int) -> *mut bpf_link;

    fn CHECK(condition: bool_t, name: *const c_char, fmt: *const c_char, ...) -> bool_t;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool_t;
    fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool_t;
    fn ASSERT_EQ<T>(actual: T, expected: T, name: *const c_char) -> bool_t;
    fn ASSERT_GE<T>(actual: T, expected: T, name: *const c_char) -> bool_t;
    fn ASSERT_GT<T>(actual: T, expected: T, name: *const c_char) -> bool_t;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool_t;
    fn ASSERT_HAS_SUBSTR(str_: *const c_char, substr: *const c_char, name: *const c_char) -> bool_t;
    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool_t;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn array_size<T, const N: usize>(_: &[T; N]) -> c_int {
    N as c_int
}

unsafe fn bpf_mov64_imm(dst: c_int, imm: i32) -> bpf_insn {
    bpf_insn {
        code: 0xb7,
        dst_src: dst as u8,
        off: 0,
        imm,
    }
}

unsafe fn bpf_exit_insn() -> bpf_insn {
    bpf_insn {
        code: 0x95,
        dst_src: 0,
        off: 0,
        imm: 0,
    }
}

unsafe fn check_data_map(obj: *mut bpf_object, prog_cnt: c_int, reset: bool_t) -> c_int {
    let mut data_map: *mut bpf_map = ptr::null_mut();
    let mut map: *mut bpf_map = ptr::null_mut();
    let mut result: *mut __u64 = ptr::null_mut();
    let zero: c_int = 0;
    let _duration: __u32 = 0;
    let mut ret: c_int = -1;
    let mut i: c_int;

    result = malloc(((prog_cnt + 32) as usize) * mem::size_of::<__u64>()) as *mut __u64;
    if CHECK(result.is_null(), cstr!("alloc_memory"), cstr!("failed to alloc memory")) {
        return -ENOMEM;
    }

    loop {
        map = bpf_object__next_map(obj, map);
        if map.is_null() {
            break;
        }
        if bpf_map__is_internal(map) {
            data_map = map;
            break;
        }
    }
    if CHECK(data_map.is_null(), cstr!("find_data_map"), cstr!("data map not found\n")) {
        goto_out(result, ret);
        return ret;
    }

    ret = bpf_map_lookup_elem(
        bpf_map__fd(data_map),
        &zero as *const _ as *const c_void,
        result as *mut c_void,
    );
    if CHECK(ret != 0, cstr!("get_result"), cstr!("failed to get output data: %d\n"), ret) {
        goto_out(result, ret);
        return ret;
    }

    i = 0;
    while i < prog_cnt {
        if CHECK(
            *result.add(i as usize) != 1,
            cstr!("result"),
            cstr!("fexit_bpf2bpf result[%d] failed err %llu\n"),
            i,
            *result.add(i as usize),
        ) {
            goto_out(result, ret);
            return ret;
        }
        *result.add(i as usize) = 0;
        i += 1;
    }
    if reset {
        ret = bpf_map_update_elem(
            bpf_map__fd(data_map),
            &zero as *const _ as *const c_void,
            result as *const c_void,
            0,
        );
        if CHECK(ret != 0, cstr!("reset_result"), cstr!("failed to reset result\n")) {
            goto_out(result, ret);
            return ret;
        }
    }

    ret = 0;
    goto_out(result, ret);
    ret
}

unsafe fn goto_out(result: *mut __u64, _ret: c_int) {
    free(result as *mut c_void);
}

unsafe fn test_fexit_bpf2bpf_common(
    obj_file: *const c_char,
    target_obj_file: *const c_char,
    prog_cnt: c_int,
    prog_name: *const *const c_char,
    run_prog: bool_t,
    cb: Option<test_cb>,
) {
    let mut obj: *mut bpf_object = ptr::null_mut();
    let mut tgt_obj: *mut bpf_object = ptr::null_mut();
    let mut tgt_prog_id: __u32;
    let mut info_len: __u32;
    let mut prog_info: bpf_prog_info = mem::zeroed();
    let mut prog: *mut *mut bpf_program = ptr::null_mut();
    let mut p: *mut bpf_program;
    let mut link: *mut *mut bpf_link = ptr::null_mut();
    let mut err: c_int;
    let mut tgt_fd: c_int = 0;
    let mut i: c_int;
    let btf: *mut btf;
    let mut topts = bpf_test_run_opts {
        sz: mem::size_of::<bpf_test_run_opts>(),
        data_in: pkt_v6.as_ptr() as *const c_void,
        data_size_in: mem::size_of_val(&pkt_v6) as __u32,
        repeat: 1,
        retval: 0,
    };

    err = bpf_prog_test_load(target_obj_file, BPF_PROG_TYPE_UNSPEC, &mut tgt_obj, &mut tgt_fd);
    if !ASSERT_OK(err, cstr!("tgt_prog_load")) {
        return;
    }

    info_len = mem::size_of::<bpf_prog_info>() as __u32;
    err = bpf_prog_get_info_by_fd(tgt_fd, &mut prog_info, &mut info_len);
    if !ASSERT_OK(err, cstr!("tgt_fd_get_info")) {
        goto_close_prog(prog_cnt, link, obj, tgt_obj, prog);
        return;
    }

    tgt_prog_id = prog_info.id;
    btf = bpf_object__btf(tgt_obj);

    link = calloc(mem::size_of::<*mut bpf_link>(), prog_cnt as usize) as *mut *mut bpf_link;
    if !ASSERT_OK_PTR(link, cstr!("link_ptr")) {
        goto_close_prog(prog_cnt, link, obj, tgt_obj, prog);
        return;
    }

    prog = calloc(mem::size_of::<*mut bpf_program>(), prog_cnt as usize) as *mut *mut bpf_program;
    if !ASSERT_OK_PTR(prog, cstr!("prog_ptr")) {
        goto_close_prog(prog_cnt, link, obj, tgt_obj, prog);
        return;
    }

    obj = bpf_object__open_file(obj_file, ptr::null());
    if !ASSERT_OK_PTR(obj, cstr!("obj_open")) {
        goto_close_prog(prog_cnt, link, obj, tgt_obj, prog);
        return;
    }

    p = ptr::null_mut();
    loop {
        p = bpf_object__next_program(obj, p);
        if p.is_null() {
            break;
        }
        err = bpf_program__set_attach_target(p, tgt_fd, ptr::null());
        ASSERT_OK(err, cstr!("set_attach_target"));
    }

    err = bpf_object__load(obj);
    if !ASSERT_OK(err, cstr!("obj_load")) {
        goto_close_prog(prog_cnt, link, obj, tgt_obj, prog);
        return;
    }

    i = 0;
    while i < prog_cnt {
        let mut link_info: bpf_link_info = mem::zeroed();
        let mut pos: *mut bpf_program;
        let mut pos_sec_name: *const c_char;
        let tgt_name: *const c_char;
        let btf_id: __s32;

        tgt_name = strstr(*prog_name.add(i as usize), cstr!("/"));
        if !ASSERT_OK_PTR(tgt_name, cstr!("tgt_name")) {
            goto_close_prog(prog_cnt, link, obj, tgt_obj, prog);
            return;
        }
        btf_id = btf__find_by_name_kind(btf, tgt_name.add(1), BTF_KIND_FUNC);

        *prog.add(i as usize) = ptr::null_mut();
        pos = ptr::null_mut();
        loop {
            pos = bpf_object__next_program(obj, pos);
            if pos.is_null() {
                break;
            }
            pos_sec_name = bpf_program__section_name(pos);
            if !pos_sec_name.is_null() && strcmp(pos_sec_name, *prog_name.add(i as usize)) == 0 {
                *prog.add(i as usize) = pos;
                break;
            }
        }
        if !ASSERT_OK_PTR(*prog.add(i as usize), *prog_name.add(i as usize)) {
            goto_close_prog(prog_cnt, link, obj, tgt_obj, prog);
            return;
        }

        *link.add(i as usize) = bpf_program__attach_trace(*prog.add(i as usize));
        if !ASSERT_OK_PTR(*link.add(i as usize), cstr!("attach_trace")) {
            goto_close_prog(prog_cnt, link, obj, tgt_obj, prog);
            return;
        }

        info_len = mem::size_of::<bpf_link_info>() as __u32;
        memset(
            &mut link_info as *mut _ as *mut c_void,
            0,
            mem::size_of::<bpf_link_info>(),
        );
        err = bpf_link_get_info_by_fd(bpf_link__fd(*link.add(i as usize)), &mut link_info, &mut info_len);
        ASSERT_OK(err, cstr!("link_fd_get_info"));
        ASSERT_EQ(
            link_info.tracing.attach_type,
            bpf_program__expected_attach_type(*prog.add(i as usize)),
            cstr!("link_attach_type"),
        );
        ASSERT_EQ(link_info.tracing.target_obj_id, tgt_prog_id, cstr!("link_tgt_obj_id"));
        ASSERT_EQ(link_info.tracing.target_btf_id, btf_id as __u32, cstr!("link_tgt_btf_id"));
        i += 1;
    }

    if let Some(cb_fn) = cb {
        err = cb_fn(obj);
        if err != 0 {
            goto_close_prog(prog_cnt, link, obj, tgt_obj, prog);
            return;
        }
    }

    if !run_prog {
        goto_close_prog(prog_cnt, link, obj, tgt_obj, prog);
        return;
    }

    err = bpf_prog_test_run_opts(tgt_fd, &mut topts);
    ASSERT_OK(err, cstr!("prog_run"));
    ASSERT_EQ(topts.retval, 0, cstr!("prog_run_ret"));

    if check_data_map(obj, prog_cnt, false) != 0 {
        goto_close_prog(prog_cnt, link, obj, tgt_obj, prog);
        return;
    }

    goto_close_prog(prog_cnt, link, obj, tgt_obj, prog);
}

unsafe fn goto_close_prog(
    prog_cnt: c_int,
    link: *mut *mut bpf_link,
    obj: *mut bpf_object,
    tgt_obj: *mut bpf_object,
    prog: *mut *mut bpf_program,
) {
    let mut i = 0;
    while i < prog_cnt {
        if !link.is_null() {
            bpf_link__destroy(*link.add(i as usize));
        }
        i += 1;
    }
    bpf_object__close(obj);
    bpf_object__close(tgt_obj);
    free(link as *mut c_void);
    free(prog as *mut c_void);
}

unsafe fn test_target_no_callees() {
    let prog_name = [cstr!("fexit/test_pkt_md_access")];
    test_fexit_bpf2bpf_common(
        cstr!("./fexit_bpf2bpf_simple.bpf.o"),
        cstr!("./test_pkt_md_access.bpf.o"),
        array_size(&prog_name),
        prog_name.as_ptr(),
        true,
        None,
    );
}

unsafe fn test_target_yes_callees() {
    let prog_name = [
        cstr!("fexit/test_pkt_access"),
        cstr!("fexit/test_pkt_access_subprog1"),
        cstr!("fexit/test_pkt_access_subprog2"),
        cstr!("fexit/test_pkt_access_subprog3"),
    ];
    test_fexit_bpf2bpf_common(
        cstr!("./fexit_bpf2bpf.bpf.o"),
        cstr!("./test_pkt_access.bpf.o"),
        array_size(&prog_name),
        prog_name.as_ptr(),
        true,
        None,
    );
}

unsafe fn test_func_replace() {
    let prog_name = [
        cstr!("fexit/test_pkt_access"),
        cstr!("fexit/test_pkt_access_subprog1"),
        cstr!("fexit/test_pkt_access_subprog2"),
        cstr!("fexit/test_pkt_access_subprog3"),
        cstr!("freplace/get_skb_len"),
        cstr!("freplace/get_skb_ifindex"),
        cstr!("freplace/get_constant"),
        cstr!("freplace/test_pkt_write_access_subprog"),
    ];
    test_fexit_bpf2bpf_common(
        cstr!("./fexit_bpf2bpf.bpf.o"),
        cstr!("./test_pkt_access.bpf.o"),
        array_size(&prog_name),
        prog_name.as_ptr(),
        true,
        None,
    );
}

unsafe fn test_func_replace_verify() {
    let prog_name = [cstr!("freplace/do_bind")];
    test_fexit_bpf2bpf_common(
        cstr!("./freplace_connect4.bpf.o"),
        cstr!("./connect4_prog.bpf.o"),
        array_size(&prog_name),
        prog_name.as_ptr(),
        false,
        None,
    );
}

unsafe extern "C" fn test_second_attach(obj: *mut bpf_object) -> c_int {
    let prog_name = cstr!("security_new_get_constant");
    let tgt_name = cstr!("get_constant");
    let tgt_obj_file = cstr!("./test_pkt_access.bpf.o");
    let mut prog: *mut bpf_program = ptr::null_mut();
    let mut tgt_obj: *mut bpf_object = ptr::null_mut();
    let mut link: *mut bpf_link = ptr::null_mut();
    let mut err: c_int = 0;
    let mut tgt_fd: c_int = 0;
    let mut topts = bpf_test_run_opts {
        sz: mem::size_of::<bpf_test_run_opts>(),
        data_in: pkt_v6.as_ptr() as *const c_void,
        data_size_in: mem::size_of_val(&pkt_v6) as __u32,
        repeat: 1,
        retval: 0,
    };

    prog = bpf_object__find_program_by_name(obj, prog_name);
    if !ASSERT_OK_PTR(prog, cstr!("find_prog")) {
        return -ENOENT;
    }

    err = bpf_prog_test_load(tgt_obj_file, BPF_PROG_TYPE_UNSPEC, &mut tgt_obj, &mut tgt_fd);
    if !ASSERT_OK(err, cstr!("second_prog_load")) {
        return err;
    }

    link = bpf_program__attach_freplace(prog, tgt_fd, tgt_name);
    if !ASSERT_OK_PTR(link, cstr!("second_link")) {
        bpf_link__destroy(link);
        bpf_object__close(tgt_obj);
        return err;
    }

    err = bpf_prog_test_run_opts(tgt_fd, &mut topts);
    if !ASSERT_OK(err, cstr!("ipv6 test_run")) {
        bpf_link__destroy(link);
        bpf_object__close(tgt_obj);
        return err;
    }
    if !ASSERT_OK(topts.retval as c_int, cstr!("ipv6 retval")) {
        bpf_link__destroy(link);
        bpf_object__close(tgt_obj);
        return err;
    }

    err = check_data_map(obj, 1, true);
    bpf_link__destroy(link);
    bpf_object__close(tgt_obj);
    err
}

unsafe fn test_func_replace_multi() {
    let prog_name = [cstr!("freplace/get_constant")];
    test_fexit_bpf2bpf_common(
        cstr!("./freplace_get_constant.bpf.o"),
        cstr!("./test_pkt_access.bpf.o"),
        array_size(&prog_name),
        prog_name.as_ptr(),
        true,
        Some(test_second_attach),
    );
}

unsafe fn test_fmod_ret_freplace() {
    let mut freplace_obj: *mut bpf_object = ptr::null_mut();
    let mut pkt_obj: *mut bpf_object = ptr::null_mut();
    let mut fmod_obj: *mut bpf_object = ptr::null_mut();
    let freplace_name = cstr!("./freplace_get_constant.bpf.o");
    let fmod_ret_name = cstr!("./fmod_ret_freplace.bpf.o");
    /* DECLARE_LIBBPF_OPTS(bpf_object_open_opts, opts); */
    let tgt_name = cstr!("./test_pkt_access.bpf.o");
    let mut freplace_link: *mut bpf_link = ptr::null_mut();
    let mut prog: *mut bpf_program;
    let _duration: __u32 = 0;
    let mut err: c_int;
    let mut pkt_fd: c_int = 0;
    let attach_prog_fd: c_int;

    err = bpf_prog_test_load(tgt_name, BPF_PROG_TYPE_UNSPEC, &mut pkt_obj, &mut pkt_fd);
    /* the target prog should load fine */
    if CHECK(err != 0, cstr!("tgt_prog_load"), cstr!("file %s err %d errno %d\n"), tgt_name, err, errno) {
        return;
    }

    freplace_obj = bpf_object__open_file(freplace_name, ptr::null());
    if !ASSERT_OK_PTR(freplace_obj, cstr!("freplace_obj_open")) {
        goto_fmod_out(freplace_link, freplace_obj, fmod_obj, pkt_obj);
        return;
    }

    prog = bpf_object__next_program(freplace_obj, ptr::null_mut());
    err = bpf_program__set_attach_target(prog, pkt_fd, ptr::null());
    ASSERT_OK(err, cstr!("freplace__set_attach_target"));

    err = bpf_object__load(freplace_obj);
    if CHECK(err != 0, cstr!("freplace_obj_load"), cstr!("err %d\n"), err) {
        goto_fmod_out(freplace_link, freplace_obj, fmod_obj, pkt_obj);
        return;
    }

    freplace_link = bpf_program__attach_trace(prog);
    if !ASSERT_OK_PTR(freplace_link, cstr!("freplace_attach_trace")) {
        goto_fmod_out(freplace_link, freplace_obj, fmod_obj, pkt_obj);
        return;
    }

    fmod_obj = bpf_object__open_file(fmod_ret_name, ptr::null());
    if !ASSERT_OK_PTR(fmod_obj, cstr!("fmod_obj_open")) {
        goto_fmod_out(freplace_link, freplace_obj, fmod_obj, pkt_obj);
        return;
    }

    attach_prog_fd = bpf_program__fd(prog);
    prog = bpf_object__next_program(fmod_obj, ptr::null_mut());
    err = bpf_program__set_attach_target(prog, attach_prog_fd, ptr::null());
    ASSERT_OK(err, cstr!("fmod_ret_set_attach_target"));

    err = bpf_object__load(fmod_obj);
    if CHECK(err == 0, cstr!("fmod_obj_load"), cstr!("loading fmod_ret should fail\n")) {
        goto_fmod_out(freplace_link, freplace_obj, fmod_obj, pkt_obj);
        return;
    }

    goto_fmod_out(freplace_link, freplace_obj, fmod_obj, pkt_obj);
}

unsafe fn goto_fmod_out(
    freplace_link: *mut bpf_link,
    freplace_obj: *mut bpf_object,
    fmod_obj: *mut bpf_object,
    pkt_obj: *mut bpf_object,
) {
    bpf_link__destroy(freplace_link);
    bpf_object__close(freplace_obj);
    bpf_object__close(fmod_obj);
    bpf_object__close(pkt_obj);
}

unsafe fn test_func_replace_void() {
    let prog_name = [cstr!("freplace/foo")];
    test_fexit_bpf2bpf_common(
        cstr!("./freplace_void.bpf.o"),
        cstr!("./test_global_func7.bpf.o"),
        array_size(&prog_name),
        prog_name.as_ptr(),
        false,
        None,
    );
}

unsafe fn test_obj_load_failure_common(
    obj_file: *const c_char,
    target_obj_file: *const c_char,
    exp_msg: *const c_char,
) {
    /*
     * standalone test that asserts failure to load freplace prog
     * because of invalid return code.
     */
    let mut obj: *mut bpf_object = ptr::null_mut();
    let mut pkt_obj: *mut bpf_object = ptr::null_mut();
    let mut prog: *mut bpf_program;
    let mut log_buf = [0 as c_char; 64 * 1024];
    let mut err: c_int;
    let mut pkt_fd: c_int = 0;
    let _duration: __u32 = 0;

    err = bpf_prog_test_load(target_obj_file, BPF_PROG_TYPE_UNSPEC, &mut pkt_obj, &mut pkt_fd);
    /* the target prog should load fine */
    if CHECK(err != 0, cstr!("tgt_prog_load"), cstr!("file %s err %d errno %d\n"), target_obj_file, err, errno) {
        return;
    }

    obj = bpf_object__open_file(obj_file, ptr::null());
    if !ASSERT_OK_PTR(obj, cstr!("obj_open")) {
        goto_obj_load_close(obj, pkt_obj);
        return;
    }

    prog = bpf_object__next_program(obj, ptr::null_mut());
    err = bpf_program__set_attach_target(prog, pkt_fd, ptr::null());
    ASSERT_OK(err, cstr!("set_attach_target"));

    log_buf[0] = 0;
    if !exp_msg.is_null() {
        bpf_program__set_log_buf(prog, log_buf.as_mut_ptr(), mem::size_of_val(&log_buf));
    }
    if env.verbosity > VERBOSE_NONE {
        bpf_program__set_log_level(prog, 2);
    }

    /* It should fail to load the program */
    err = bpf_object__load(obj);
    if env.verbosity > VERBOSE_NONE && !exp_msg.is_null() {
        printf(cstr!("VERIFIER LOG:\n================\n%s\n================\n"), log_buf.as_ptr());
    }
    if CHECK(err == 0, cstr!("bpf_obj_load should fail"), cstr!("err %d\n"), err) {
        goto_obj_load_close(obj, pkt_obj);
        return;
    }

    if !exp_msg.is_null() {
        ASSERT_HAS_SUBSTR(log_buf.as_ptr(), exp_msg, cstr!("fail_msg"));
    }
    goto_obj_load_close(obj, pkt_obj);
}

unsafe fn goto_obj_load_close(obj: *mut bpf_object, pkt_obj: *mut bpf_object) {
    bpf_object__close(obj);
    bpf_object__close(pkt_obj);
}

unsafe fn test_func_replace_return_code() {
    /* test invalid return code in the replaced program */
    test_obj_load_failure_common(cstr!("./freplace_connect_v4_prog.bpf.o"), cstr!("./connect4_prog.bpf.o"), ptr::null());
}

unsafe fn test_func_map_prog_compatibility() {
    /* test with spin lock map value in the replaced program */
    test_obj_load_failure_common(cstr!("./freplace_attach_probe.bpf.o"), cstr!("./test_attach_probe.bpf.o"), ptr::null());
}

unsafe fn test_func_replace_unreliable() {
    /* freplace'ing unreliable main prog should fail with error
     * "Cannot replace static functions"
     */
    test_obj_load_failure_common(
        cstr!("freplace_unreliable_prog.bpf.o"),
        cstr!("./verifier_btf_unreliable_prog.bpf.o"),
        cstr!("Cannot replace static functions"),
    );
}

unsafe fn test_func_replace_global_func() {
    let prog_name = [cstr!("freplace/test_pkt_access")];

    test_fexit_bpf2bpf_common(
        cstr!("./freplace_global_func.bpf.o"),
        cstr!("./test_pkt_access.bpf.o"),
        array_size(&prog_name),
        prog_name.as_ptr(),
        false,
        None,
    );
}

unsafe fn test_func_replace_int_with_void() {
    /* Make sure we can't freplace with the wrong type */
    test_obj_load_failure_common(
        cstr!("freplace_int_with_void.bpf.o"),
        cstr!("./test_global_func2.bpf.o"),
        cstr!("Return type UNKNOWN of test_freplace_int_with_void() doesn't match type INT of global_func2()"),
    );
}

unsafe fn find_prog_btf_id(name: *const c_char, attach_prog_fd: __u32) -> c_int {
    let mut info: bpf_prog_info = mem::zeroed();
    let mut info_len: __u32 = mem::size_of::<bpf_prog_info>() as __u32;
    let mut btf: *mut btf;
    let mut ret: c_int;

    ret = bpf_prog_get_info_by_fd(attach_prog_fd as c_int, &mut info, &mut info_len);
    if ret != 0 {
        return ret;
    }

    if info.btf_id == 0 {
        return -EINVAL;
    }

    btf = btf__load_from_kernel_by_id(info.btf_id);
    ret = libbpf_get_error(btf as *const c_void);
    if ret != 0 {
        return ret;
    }

    ret = btf__find_by_name_kind(btf, name, BTF_KIND_FUNC);
    btf__free(btf);
    ret
}

unsafe fn load_fentry(attach_prog_fd: c_int, attach_btf_id: c_int) -> c_int {
    let opts = bpf_prog_load_opts {
        sz: mem::size_of::<bpf_prog_load_opts>(),
        expected_attach_type: BPF_TRACE_FENTRY,
        attach_prog_fd,
        attach_btf_id,
    };
    let insns = [
        bpf_mov64_imm(BPF_REG_0, 0),
        bpf_exit_insn(),
    ];

    bpf_prog_load(
        BPF_PROG_TYPE_TRACING,
        cstr!("bind4_fentry"),
        cstr!("GPL"),
        insns.as_ptr(),
        insns.len(),
        &opts,
    )
}

unsafe fn test_fentry_to_cgroup_bpf() {
    let mut skel: *mut bind4_prog = ptr::null_mut();
    let mut info: bpf_prog_info = mem::zeroed();
    let mut info_len: __u32 = mem::size_of::<bpf_prog_info>() as __u32;
    let mut cgroup_fd: c_int = -1;
    let mut fentry_fd: c_int = -1;
    let mut btf_id: c_int;

    cgroup_fd = test__join_cgroup(cstr!("/fentry_to_cgroup_bpf"));
    if !ASSERT_GE(cgroup_fd, 0, cstr!("cgroup_fd")) {
        return;
    }

    skel = bind4_prog__open_and_load();
    if !ASSERT_OK_PTR(skel, cstr!("skel")) {
        goto_fentry_cleanup(cgroup_fd, fentry_fd, skel);
        return;
    }

    (*skel).links.bind_v4_prog = bpf_program__attach_cgroup((*skel).progs.bind_v4_prog, cgroup_fd);
    if !ASSERT_OK_PTR((*skel).links.bind_v4_prog, cstr!("bpf_program__attach_cgroup")) {
        goto_fentry_cleanup(cgroup_fd, fentry_fd, skel);
        return;
    }

    btf_id = find_prog_btf_id(cstr!("bind_v4_prog"), bpf_program__fd((*skel).progs.bind_v4_prog) as __u32);
    if !ASSERT_GE(btf_id, 0, cstr!("find_prog_btf_id")) {
        goto_fentry_cleanup(cgroup_fd, fentry_fd, skel);
        return;
    }

    fentry_fd = load_fentry(bpf_program__fd((*skel).progs.bind_v4_prog), btf_id);
    if !ASSERT_GE(fentry_fd, 0, cstr!("load_fentry")) {
        goto_fentry_cleanup(cgroup_fd, fentry_fd, skel);
        return;
    }

    /* Make sure bpf_prog_get_info_by_fd works correctly when attaching
     * to another BPF program.
     */
    ASSERT_OK(
        bpf_prog_get_info_by_fd(fentry_fd, &mut info, &mut info_len),
        cstr!("bpf_prog_get_info_by_fd"),
    );

    ASSERT_EQ(info.btf_id, 0, cstr!("info.btf_id"));
    ASSERT_EQ(info.attach_btf_id, btf_id as __u32, cstr!("info.attach_btf_id"));
    ASSERT_GT(info.attach_btf_obj_id, 0, cstr!("info.attach_btf_obj_id"));

    goto_fentry_cleanup(cgroup_fd, fentry_fd, skel);
}

unsafe fn goto_fentry_cleanup(cgroup_fd: c_int, fentry_fd: c_int, skel: *mut bind4_prog) {
    if cgroup_fd >= 0 {
        close(cgroup_fd);
    }
    if fentry_fd >= 0 {
        close(fentry_fd);
    }
    bind4_prog__destroy(skel);
}

unsafe fn test_func_replace_progmap() {
    let mut value = bpf_cpumap_val {
        qsize: 1,
        bpf_prog: bpf_cpumap_val_bpf_prog { fd: 0 },
    };
    let mut skel: *mut freplace_progmap = ptr::null_mut();
    let mut tgt_skel: *mut xdp_dummy = ptr::null_mut();
    let key: __u32 = 0;
    let mut err: c_int;

    skel = freplace_progmap__open();
    if !ASSERT_OK_PTR(skel, cstr!("prog_open")) {
        return;
    }

    tgt_skel = xdp_dummy__open_and_load();
    if !ASSERT_OK_PTR(tgt_skel, cstr!("tgt_prog_load")) {
        goto_progmap_out(tgt_skel, skel);
        return;
    }

    err = bpf_program__set_attach_target(
        (*skel).progs.xdp_cpumap_prog,
        bpf_program__fd((*tgt_skel).progs.xdp_dummy_prog),
        cstr!("xdp_dummy_prog"),
    );
    if !ASSERT_OK(err, cstr!("set_attach_target")) {
        goto_progmap_out(tgt_skel, skel);
        return;
    }

    err = freplace_progmap__load(skel);
    if !ASSERT_OK(err, cstr!("obj_load")) {
        goto_progmap_out(tgt_skel, skel);
        return;
    }

    /* Prior to fixing the kernel, loading the PROG_TYPE_EXT 'redirect'
     * program above will cause the map owner type of 'cpumap' to be set to
     * PROG_TYPE_EXT. This in turn will cause the bpf_map_update_elem()
     * below to fail, because the program we are inserting into the map is
     * of PROG_TYPE_XDP. After fixing the kernel, the initial ownership will
     * be correctly resolved to the *target* of the PROG_TYPE_EXT program
     * (i.e., PROG_TYPE_XDP) and the map update will succeed.
     */
    value.bpf_prog.fd = bpf_program__fd((*skel).progs.xdp_drop_prog);
    err = bpf_map_update_elem(
        bpf_map__fd((*skel).maps.cpu_map),
        &key as *const _ as *const c_void,
        &value as *const _ as *const c_void,
        0,
    );
    ASSERT_OK(err, cstr!("map_update"));

    goto_progmap_out(tgt_skel, skel);
}

unsafe fn goto_progmap_out(tgt_skel: *mut xdp_dummy, skel: *mut freplace_progmap) {
    xdp_dummy__destroy(tgt_skel);
    freplace_progmap__destroy(skel);
}

unsafe fn test_sleepable_fentry_to_xdp() {
    let mut skel: *mut fentry_sleepable = ptr::null_mut();
    let mut skel_xdp: *mut xdp_dummy = ptr::null_mut();
    let mut ifindex: c_int;
    let mut prog_fd: c_int;
    let mut err: c_int;
    let mut buff = [0 as c_char; 64];

    /* #ifndef __x86_64__: test is skipped on non-x86_64 builds. */
    #[cfg(not(target_arch = "x86_64"))]
    {
        test__skip();
        return;
    }

    ifindex = if_nametoindex(cstr!("lo")) as c_int;
    if !ASSERT_GT(ifindex, 0, cstr!("if_nametoindex")) {
        return;
    }

    skel_xdp = xdp_dummy__open_and_load();
    if !ASSERT_OK_PTR(skel_xdp, cstr!("xdp_dummy__open_and_load")) {
        return;
    }

    skel = fentry_sleepable__open();
    if !ASSERT_OK_PTR(skel, cstr!("fentry_sleepable__open")) {
        goto_sleepable_out(skel, skel_xdp);
        return;
    }

    (*(*skel).bss).user_ptr = buff.as_mut_ptr();

    prog_fd = bpf_program__fd((*skel_xdp).progs.__x64_sys_nop);
    err = bpf_program__set_attach_target((*skel).progs.fentry_xdp, prog_fd, cstr!("__x64_sys_nop"));
    if !ASSERT_OK(err, cstr!("bpf_program__set_attach_target")) {
        goto_sleepable_out(skel, skel_xdp);
        return;
    }

    err = fentry_sleepable__load(skel);
    ASSERT_ERR(err, cstr!("fentry_sleepable__load"));
    if err != 0 {
        goto_sleepable_out(skel, skel_xdp);
        return;
    }

    (*skel).links.fentry_xdp = bpf_program__attach_trace((*skel).progs.fentry_xdp);
    if !ASSERT_OK_PTR((*skel).links.fentry_xdp, cstr!("bpf_program__attach_trace")) {
        goto_sleepable_out(skel, skel_xdp);
        return;
    }

    (*skel_xdp).links.__x64_sys_nop =
        bpf_program__attach_xdp((*skel_xdp).progs.__x64_sys_nop, ifindex);
    if !ASSERT_OK_PTR((*skel_xdp).links.__x64_sys_nop, cstr!("bpf_program__attach_xdp")) {
        goto_sleepable_out(skel, skel_xdp);
        return;
    }

    err = system(cstr!("ping -q -c 1 -W 1 127.0.0.1 > /dev/null"));
    ASSERT_OK(err, cstr!("ping"));
    ASSERT_ERR((*(*skel).bss).retval, cstr!("retval"));

    goto_sleepable_out(skel, skel_xdp);
}

unsafe fn goto_sleepable_out(skel: *mut fentry_sleepable, skel_xdp: *mut xdp_dummy) {
    fentry_sleepable__destroy(skel);
    xdp_dummy__destroy(skel_xdp);
}

/* NOTE: affect other tests, must run in serial mode */
#[no_mangle]
pub unsafe extern "C" fn serial_test_fexit_bpf2bpf() {
    if test__start_subtest(cstr!("target_no_callees")) {
        test_target_no_callees();
    }
    if test__start_subtest(cstr!("target_yes_callees")) {
        test_target_yes_callees();
    }
    if test__start_subtest(cstr!("func_replace")) {
        test_func_replace();
    }
    if test__start_subtest(cstr!("func_replace_verify")) {
        test_func_replace_verify();
    }
    if test__start_subtest(cstr!("func_replace_return_code")) {
        test_func_replace_return_code();
    }
    if test__start_subtest(cstr!("func_map_prog_compatibility")) {
        test_func_map_prog_compatibility();
    }
    if test__start_subtest(cstr!("func_replace_unreliable")) {
        test_func_replace_unreliable();
    }
    if test__start_subtest(cstr!("func_replace_multi")) {
        test_func_replace_multi();
    }
    if test__start_subtest(cstr!("fmod_ret_freplace")) {
        test_fmod_ret_freplace();
    }
    if test__start_subtest(cstr!("func_replace_global_func")) {
        test_func_replace_global_func();
    }
    if test__start_subtest(cstr!("fentry_to_cgroup_bpf")) {
        test_fentry_to_cgroup_bpf();
    }
    if test__start_subtest(cstr!("func_replace_progmap")) {
        test_func_replace_progmap();
    }
    if test__start_subtest(cstr!("freplace_int_with_void")) {
        test_func_replace_int_with_void();
    }
    if test__start_subtest(cstr!("freplace_void")) {
        test_func_replace_void();
    }
    if test__start_subtest(cstr!("sleepable_fentry_to_xdp")) {
        test_sleepable_fentry_to_xdp();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
