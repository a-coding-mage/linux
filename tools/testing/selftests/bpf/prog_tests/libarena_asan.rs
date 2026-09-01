// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

/*
 * C dependencies removed from executable Rust:
 * <test_progs.h>, <unistd.h>, <libarena/common.h>, <libarena/asan.h>,
 * <libarena/buddy.h>, <libarena/userspace.h>, and
 * "libarena/libarena_asan.skel.h".
 */

use core::ffi::{c_char, c_int, c_long, c_void};

const ARENA_RESERVE_PAGES_DFL: u32 = 256;
const _SC_PAGESIZE: c_int = 30;

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct arena_alloc_reserve_args {
    pub nr_pages: u32,
}

#[repr(C)]
pub struct libarena_asan_progs {
    pub arena_buddy_destroy: *mut bpf_program,
    pub arena_buddy_reset: *mut bpf_program,
    pub arena_alloc_reserve: *mut bpf_program,
    pub arena_get_info: *mut bpf_program,
    pub asan_init: *mut bpf_program,
}

#[repr(C)]
pub struct libarena_asan {
    pub obj: *mut bpf_object,
    pub progs: libarena_asan_progs,
}

unsafe extern "C" {
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn sysconf(name: c_int) -> c_long;

    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut libarena_asan, name: *const c_char) -> bool;
    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;

    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_program__name(prog: *mut bpf_program) -> *const c_char;
    fn bpf_object__next_program(obj: *mut bpf_object, prog: *mut bpf_program) -> *mut bpf_program;

    fn libarena_run_prog(fd: c_int) -> c_int;
    fn libarena_run_prog_args(fd: c_int, args: *mut c_void, args_sz: usize) -> c_int;
    fn libarena_asan_init(arena_get_info_fd: c_int, asan_init_fd: c_int, nr_pages: u64) -> c_int;
    fn libarena_is_asan_test_prog(name: *const c_char) -> bool;
    fn verify_test_stderr(obj: *mut bpf_object, prog: *mut bpf_program);

    fn libarena_asan__open_and_load() -> *mut libarena_asan;
    fn libarena_asan__attach(skel: *mut libarena_asan) -> c_int;
    fn libarena_asan__destroy(skel: *mut libarena_asan);
}

unsafe fn c_str(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

/*
 * Translated body of code guarded in C by:
 * #ifdef HAS_BPF_ARENA_ASAN
 */
unsafe fn run_libarena_asan_test(
    skel: *mut libarena_asan,
    prog: *mut bpf_program,
    name: *const c_char,
) {
    let mut ret: c_int;

    if !strstr(name, c_str(b"test_buddy\0")).is_null() {
        /* Buddy tests initialize the allocator directly. */
        ret = libarena_run_prog(bpf_program__fd((*skel).progs.arena_buddy_destroy));
        if !ASSERT_OK(ret, c_str(b"arena_buddy_destroy\0")) {
            return;
        }
    } else {
        ret = libarena_run_prog(bpf_program__fd((*skel).progs.arena_buddy_reset));
        if !ASSERT_OK(ret, c_str(b"arena_buddy_reset\0")) {
            return;
        }
    }

    ret = libarena_run_prog(bpf_program__fd(prog));
    ASSERT_OK(ret, name);

    verify_test_stderr((*skel).obj, prog);
}

unsafe fn run_test() {
    let mut args: arena_alloc_reserve_args = core::mem::zeroed();
    let skel: *mut libarena_asan;
    let mut prog: *mut bpf_program;
    let mut ret: c_int;

    skel = libarena_asan__open_and_load();
    if !ASSERT_OK_PTR(skel, c_str(b"open_and_load\0")) {
        return;
    }

    ret = libarena_asan__attach(skel);
    if !ASSERT_OK(ret, c_str(b"attach\0")) {
        libarena_asan__destroy(skel);
        return;
    }

    args.nr_pages = ARENA_RESERVE_PAGES_DFL;

    ret = libarena_run_prog_args(
        bpf_program__fd((*skel).progs.arena_alloc_reserve),
        &mut args as *mut arena_alloc_reserve_args as *mut c_void,
        core::mem::size_of_val(&args),
    );
    if !ASSERT_OK(ret, c_str(b"arena_alloc_reserve\0")) {
        libarena_asan__destroy(skel);
        return;
    }

    ret = libarena_asan_init(
        bpf_program__fd((*skel).progs.arena_get_info),
        bpf_program__fd((*skel).progs.asan_init),
        ((1u64 << 32) / sysconf(_SC_PAGESIZE) as u64) as u64,
    );
    if !ASSERT_OK(ret, c_str(b"libarena_asan_init\0")) {
        libarena_asan__destroy(skel);
        return;
    }

    /*
     * Rust equivalent of:
     * bpf_object__for_each_program(prog, skel->obj)
     */
    prog = bpf_object__next_program((*skel).obj, core::ptr::null_mut());
    while !prog.is_null() {
        let name: *const c_char = bpf_program__name(prog);

        if !libarena_is_asan_test_prog(name) {
            prog = bpf_object__next_program((*skel).obj, prog);
            continue;
        }

        if !test__start_subtest(name) {
            prog = bpf_object__next_program((*skel).obj, prog);
            continue;
        }

        run_libarena_asan_test(skel, prog, name);
        prog = bpf_object__next_program((*skel).obj, prog);
    }

    libarena_asan__destroy(skel);
}

/*
 * Run the test depending on whether LLVM can compile arena ASAN
 * programs.
 */
#[no_mangle]
pub unsafe extern "C" fn serial_test_libarena_asan() {
    /*
     * C conditional preserved:
     * #ifdef HAS_BPF_ARENA_ASAN
     *     run_test();
     * #else
     *     test__skip();
     * #endif
     */
    #[cfg(HAS_BPF_ARENA_ASAN)]
    {
        run_test();
    }

    #[cfg(not(HAS_BPF_ARENA_ASAN))]
    {
        test__skip();
    }

    return;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
