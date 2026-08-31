// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// C dependencies from:
// <errno.h>, <stdlib.h>, <unistd.h>, <sys/mman.h>
// <bpf/libbpf.h>, <bpf/bpf.h>

use core::ffi::{c_char, c_int, c_void};

pub const EINVAL: c_int = 22;
pub const ENOMEM: c_int = 12;
pub const _SC_PAGESIZE: c_int = 30;

unsafe extern "C" {
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn sysconf(name: c_int) -> isize;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn mincore(addr: *mut c_void, length: usize, vec: *mut u8) -> c_int;
    fn __errno_location() -> *mut c_int;
}

pub unsafe fn libarena_run_prog(prog_fd: c_int) -> c_int {
    let mut opts: bpf_test_run_opts = unsafe { core::mem::zeroed() };
    let ret: c_int;

    ret = unsafe { bpf_prog_test_run_opts(prog_fd, &mut opts) };
    if ret != 0 {
        return ret;
    }

    opts.retval
}

pub unsafe fn libarena_is_test_prog(name: *const c_char) -> bool {
    unsafe { strstr(name, c"test_".as_ptr()) == name as *mut c_char }
}

pub unsafe fn libarena_is_asan_test_prog(name: *const c_char) -> bool {
    unsafe { strstr(name, c"asan_test".as_ptr()) == name as *mut c_char }
}

pub unsafe fn libarena_is_parallel_test_prog(name: *const c_char) -> bool {
    unsafe { strstr(name, c"parallel_test".as_ptr()) == name as *mut c_char }
}

pub unsafe fn libarena_run_prog_args(
    prog_fd: c_int,
    args: *mut c_void,
    argsize: usize,
) -> c_int {
    let mut opts: bpf_test_run_opts = unsafe { core::mem::zeroed() };
    let ret: c_int;

    opts.ctx_in = args;
    opts.ctx_size_in = argsize;

    ret = unsafe { bpf_prog_test_run_opts(prog_fd, &mut opts) };

    if ret != 0 { ret } else { opts.retval }
}

pub unsafe fn libarena_get_arena_base(
    arena_get_info_fd: c_int,
    arena_base: *mut *mut c_void,
) -> c_int {
    let mut opts: bpf_test_run_opts = unsafe { core::mem::zeroed() };
    let mut args: arena_get_info_args = unsafe { core::mem::zeroed() };
    let ret: c_int;

    args.arena_base = core::ptr::null_mut();

    opts.ctx_in = &mut args as *mut arena_get_info_args as *mut c_void;
    opts.ctx_size_in = core::mem::size_of::<arena_get_info_args>();

    ret = unsafe { bpf_prog_test_run_opts(arena_get_info_fd, &mut opts) };
    if ret != 0 {
        return ret;
    }
    if opts.retval != 0 {
        return opts.retval;
    }

    unsafe {
        *arena_base = args.arena_base;
    }
    0
}

pub unsafe fn libarena_get_globals_pages(
    arena_get_globals_fd: c_int,
    arena_all_pages: usize,
    globals_pages: *mut u64,
) -> c_int {
    let pgsize: usize = unsafe { sysconf(_SC_PAGESIZE) as usize };
    let mut arena_base: *mut c_void = core::ptr::null_mut();
    let mut i: isize;
    let vec: *mut u8;
    let mut ret: c_int;

    ret = unsafe { libarena_get_arena_base(arena_get_globals_fd, &mut arena_base) };
    if ret != 0 {
        return ret;
    }

    if arena_base.is_null() {
        return -EINVAL;
    }

    vec = unsafe { calloc(arena_all_pages, core::mem::size_of::<u8>()) as *mut u8 };
    if vec.is_null() {
        return -ENOMEM;
    }

    if unsafe { mincore(arena_base, arena_all_pages.wrapping_mul(pgsize), vec) } < 0 {
        ret = unsafe { -*__errno_location() };
        unsafe {
            free(vec as *mut c_void);
        }
        return ret;
    }

    unsafe {
        *globals_pages = 0;
    }
    i = arena_all_pages.wrapping_sub(1) as isize;
    while i >= 0 {
        if unsafe { (*vec.offset(i) & 0x1) == 0 } {
            break;
        }
        unsafe {
            *globals_pages = (*globals_pages).wrapping_add(1);
        }
        i -= 1;
    }

    unsafe {
        free(vec as *mut c_void);
    }
    0
}

pub unsafe fn libarena_asan_init(
    arena_asan_init_fd: c_int,
    asan_init_fd: c_int,
    arena_all_pages: usize,
) -> c_int {
    let mut opts: bpf_test_run_opts = unsafe { core::mem::zeroed() };
    let mut args: asan_init_args;
    let mut globals_pages: u64 = 0;
    let ret: c_int;

    ret = unsafe {
        libarena_get_globals_pages(arena_asan_init_fd, arena_all_pages, &mut globals_pages)
    };
    if ret != 0 {
        return ret;
    }

    args = asan_init_args {
        arena_all_pages,
        arena_globals_pages: globals_pages,
    };

    opts.ctx_in = &mut args as *mut asan_init_args as *mut c_void;
    opts.ctx_size_in = core::mem::size_of::<asan_init_args>();

    ret = unsafe { bpf_prog_test_run_opts(asan_init_fd, &mut opts) };
    if ret != 0 {
        return ret;
    }
    opts.retval
}
