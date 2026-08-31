// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include <test_progs.h>
// #include "struct_ops_arena.skel.h"
// #include "struct_ops_arena_attach.skel.h"
// #include "struct_ops_arena_fail.skel.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

const EOPNOTSUPP: c_int = 95;

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub retval: u32,
}

impl Default for bpf_test_run_opts {
    fn default() -> Self {
        Self {
            sz: core::mem::size_of::<Self>(),
            retval: 0,
        }
    }
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct struct_ops_arena_maps {
    pub testmod_arena: *mut bpf_map,
}

#[repr(C)]
pub struct struct_ops_arena_progs {
    pub trigger: *mut bpf_program,
    pub test_arena_cb: *mut bpf_program,
}

#[repr(C)]
pub struct struct_ops_arena {
    pub obj: *mut bpf_object,
    pub maps: struct_ops_arena_maps,
    pub progs: struct_ops_arena_progs,
}

#[repr(C)]
pub struct struct_ops_arena_attach {
    pub obj: *mut bpf_object,
}

#[repr(C)]
pub struct struct_ops_arena_fail {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn struct_ops_arena__open_and_load() -> *mut struct_ops_arena;
    fn struct_ops_arena__destroy(skel: *mut struct_ops_arena);

    fn struct_ops_arena_fail__open_and_load() -> *mut struct_ops_arena_fail;
    fn struct_ops_arena_fail__destroy(skel: *mut struct_ops_arena_fail);

    fn struct_ops_arena_attach__open() -> *mut struct_ops_arena_attach;
    fn struct_ops_arena_attach__load(skel: *mut struct_ops_arena_attach) -> c_int;
    fn struct_ops_arena_attach__destroy(skel: *mut struct_ops_arena_attach);

    fn bpf_map__attach_struct_ops(map: *mut bpf_map) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_program__set_attach_target(
        prog: *mut bpf_program,
        target_fd: c_int,
        attach_func_name: *const c_char,
    ) -> c_int;
    fn bpf_program__set_log_buf(prog: *mut bpf_program, log_buf: *mut c_char, log_size: usize);

    fn test__start_subtest(name: *const c_char) -> bool;
    fn test__skip();

    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_ERR_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_HAS_SUBSTR(str_: *const c_char, substr: *const c_char, name: *const c_char) -> bool;
}

// bpf_object__for_each_program() is supplied by libbpf headers in C. Keep it as
// an external iterator-shaped dependency for this source-level translation.
unsafe extern "C" {
    fn bpf_object__next_program(obj: *mut bpf_object, prev: *mut bpf_program) -> *mut bpf_program;
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
/*
 * Attach callbacks with __arena and __arena__nullable arguments and drive
 * them through the bpf_testmod_ops3_call_test_arena*() kfuncs.
 */
unsafe fn arena_arg() {
    let mut topts: bpf_test_run_opts = bpf_test_run_opts::default();
    let skel: *mut struct_ops_arena;
    let mut link: *mut bpf_link = ptr::null_mut();
    let err: c_int;

    skel = struct_ops_arena__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *mut c_void,
        c"struct_ops_arena__open_and_load".as_ptr(),
    ) {
        return;
    }

    link = bpf_map__attach_struct_ops((*skel).maps.testmod_arena);
    if !ASSERT_OK_PTR(link as *mut c_void, c"attach_struct_ops".as_ptr()) {
        goto_out_arena_arg(link, skel);
        return;
    }

    err = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.trigger), &mut topts);
    ASSERT_OK(err, c"test_run".as_ptr());
    ASSERT_EQ(topts.retval as c_int, 0, c"trigger_retval".as_ptr());

    goto_out_arena_arg(link, skel);
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
unsafe fn goto_out_arena_arg(link: *mut bpf_link, skel: *mut struct_ops_arena) {
    bpf_link__destroy(link);
    struct_ops_arena__destroy(skel);
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
/*
 * A program with no arena cannot attach to a member with an __arena
 * argument.
 */
unsafe fn arena_arg_fail() {
    let skel: *mut struct_ops_arena_fail;

    skel = struct_ops_arena_fail__open_and_load();
    if ASSERT_ERR_PTR(
        skel as *mut c_void,
        c"struct_ops_arena_fail__open_and_load".as_ptr(),
    ) {
        return;
    }

    struct_ops_arena_fail__destroy(skel);
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
unsafe fn arena_arg_attach_one(target_fd: c_int, prog_name: *const c_char) {
    let skel: *mut struct_ops_arena_attach;
    let prog: *mut bpf_program;
    let mut pos: *mut bpf_program;
    let mut log_buf: [c_char; 64 * 1024] = [0; 64 * 1024];
    let mut err: c_int;

    skel = struct_ops_arena_attach__open();
    if !ASSERT_OK_PTR(
        skel as *mut c_void,
        c"struct_ops_arena_attach__open".as_ptr(),
    ) {
        return;
    }

    prog = bpf_object__find_program_by_name((*skel).obj, prog_name);
    if !ASSERT_OK_PTR(prog as *mut c_void, prog_name) {
        goto_out_arena_arg_attach_one(skel);
        return;
    }

    pos = bpf_object__next_program((*skel).obj, ptr::null_mut());
    while !pos.is_null() {
        bpf_program__set_autoload(pos, pos == prog);
        pos = bpf_object__next_program((*skel).obj, pos);
    }

    err = bpf_program__set_attach_target(prog, target_fd, c"test_arena_cb".as_ptr());
    if !ASSERT_OK(err, c"set_attach_target".as_ptr()) {
        goto_out_arena_arg_attach_one(skel);
        return;
    }

    log_buf[0] = b'\0' as c_char;
    bpf_program__set_log_buf(prog, log_buf.as_mut_ptr(), log_buf.len());
    err = struct_ops_arena_attach__load(skel);

    ASSERT_EQ(err, -EOPNOTSUPP, prog_name);
    ASSERT_HAS_SUBSTR(
        log_buf.as_ptr(),
        c"Cannot attach to a target with arena context arguments".as_ptr(),
        c"verifier_log".as_ptr(),
    );

    goto_out_arena_arg_attach_one(skel);
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
unsafe fn goto_out_arena_arg_attach_one(skel: *mut struct_ops_arena_attach) {
    struct_ops_arena_attach__destroy(skel);
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
unsafe fn arena_arg_attach() {
    let skel: *mut struct_ops_arena;
    let target_fd: c_int;

    skel = struct_ops_arena__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *mut c_void,
        c"struct_ops_arena__open_and_load".as_ptr(),
    ) {
        return;
    }

    target_fd = bpf_program__fd((*skel).progs.test_arena_cb);
    arena_arg_attach_one(target_fd, c"fentry_test_arena".as_ptr());
    arena_arg_attach_one(target_fd, c"fexit_test_arena".as_ptr());
    arena_arg_attach_one(target_fd, c"freplace_test_arena".as_ptr());

    struct_ops_arena__destroy(skel);
}

/*
 * Serialized because it attaches the singleton bpf_testmod_ops3, which
 * test_struct_ops_private_stack also attaches; registering it twice fails
 * with -EEXIST.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn serial_test_struct_ops_arena() {
    /*
     * Arena struct_ops arguments need JIT support, currently x86-64 and
     * arm64 only. Elsewhere verification fails with "JIT does not support
     * arena arguments", so the programs cannot even load.
     */
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    {
        if test__start_subtest(c"arena_arg".as_ptr()) {
            arena_arg();
        }
        if test__start_subtest(c"arena_arg_fail".as_ptr()) {
            arena_arg_fail();
        }
        if test__start_subtest(c"arena_arg_attach".as_ptr()) {
            arena_arg_attach();
        }
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        test__skip();
    }
}
