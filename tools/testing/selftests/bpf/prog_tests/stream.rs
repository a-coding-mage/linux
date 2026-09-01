// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

// Translated from testing/selftests/bpf/prog_tests/stream.c.
// C dependencies removed from executable Rust:
//   <test_progs.h>, <sys/mman.h>, "stream.skel.h", "stream_fail.skel.h"

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem;
use core::ptr;

const BPF_STREAM_STDOUT: u32 = 1;
const BPF_STREAM_STDERR: u32 = 2;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const EFAULT: c_int = 14;

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
pub struct bpf_prog_stream_read_opts {
    pub sz: usize,
}

#[repr(C)]
pub struct stream_progs {
    pub stream_syscall: *mut bpf_program,
    pub stream_arena_read_fault: *mut bpf_program,
    pub stream_arena_write_fault: *mut bpf_program,
    pub stream_arena_load_acquire_fault: *mut bpf_program,
    pub stream_arena_xchg_fault: *mut bpf_program,
    pub stream_arena_cmpxchg_fault: *mut bpf_program,
}

#[repr(C)]
pub struct stream_bss {
    pub fault_addr: c_ulong,
}

#[repr(C)]
pub struct stream {
    pub progs: stream_progs,
    pub bss: *mut stream_bss,
}

unsafe extern "C" {
    fn RUN_TESTS(name: *const c_char);

    fn stream__open_and_load() -> *mut stream;
    fn stream__destroy(obj: *mut stream);

    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_prog_stream_read(
        prog_fd: c_int,
        stream_id: u32,
        buf: *mut c_void,
        size: usize,
        opts: *mut bpf_prog_stream_read_opts,
    ) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_LT(left: c_int, right: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(left: c_int, right: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(left: c_int, right: c_int, name: *const c_char) -> bool;
    fn ASSERT_LE(left: c_int, right: c_int, name: *const c_char) -> bool;
    fn ASSERT_HAS_SUBSTR(buf: *const c_char, needle: *const c_char, name: *const c_char) -> bool;

    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn __errno_location() -> *mut c_int;
}

#[inline]
fn libbpf_opts_bpf_test_run_opts() -> bpf_test_run_opts {
    let mut opts: bpf_test_run_opts = unsafe { mem::zeroed() };
    opts.sz = mem::size_of::<bpf_test_run_opts>();
    opts
}

#[inline]
fn libbpf_opts_bpf_prog_stream_read_opts() -> bpf_prog_stream_read_opts {
    let mut opts: bpf_prog_stream_read_opts = unsafe { mem::zeroed() };
    opts.sz = mem::size_of::<bpf_prog_stream_read_opts>();
    opts
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_stream_failure() {
    unsafe {
        RUN_TESTS(c"stream_fail".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_stream_success() {
    unsafe {
        RUN_TESTS(c"stream".as_ptr());
    }
    return;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_stream_syscall() {
    let mut opts = libbpf_opts_bpf_test_run_opts();
    let mut ropts = libbpf_opts_bpf_prog_stream_read_opts();
    let mut skel: *mut stream;
    let mut ret: c_int;
    let prog_fd: c_int;
    let mut buf = [0 as c_char; 64];

    unsafe {
        skel = stream__open_and_load();
        if !ASSERT_OK_PTR(skel.cast::<c_void>(), c"stream__open_and_load".as_ptr()) {
            return;
        }

        prog_fd = bpf_program__fd((*skel).progs.stream_syscall);
        ret = bpf_prog_test_run_opts(prog_fd, &mut opts);
        ASSERT_OK(ret, c"ret".as_ptr());
        ASSERT_OK(opts.retval as c_int, c"retval".as_ptr());

        ASSERT_LT(
            bpf_prog_stream_read(
                0,
                BPF_STREAM_STDOUT,
                buf.as_mut_ptr().cast::<c_void>(),
                mem::size_of_val(&buf),
                &mut ropts,
            ),
            0,
            c"error".as_ptr(),
        );
        ret = -*__errno_location();
        ASSERT_EQ(ret, -EINVAL, c"bad prog_fd".as_ptr());

        ASSERT_LT(
            bpf_prog_stream_read(
                prog_fd,
                0,
                buf.as_mut_ptr().cast::<c_void>(),
                mem::size_of_val(&buf),
                &mut ropts,
            ),
            0,
            c"error".as_ptr(),
        );
        ret = -*__errno_location();
        ASSERT_EQ(ret, -ENOENT, c"bad stream id".as_ptr());

        ASSERT_LT(
            bpf_prog_stream_read(
                prog_fd,
                BPF_STREAM_STDOUT,
                ptr::null_mut(),
                mem::size_of_val(&buf),
                ptr::null_mut(),
            ),
            0,
            c"error".as_ptr(),
        );
        ret = -*__errno_location();
        ASSERT_EQ(ret, -EFAULT, c"bad stream buf".as_ptr());

        ret = bpf_prog_stream_read(
            prog_fd,
            BPF_STREAM_STDOUT,
            buf.as_mut_ptr().cast::<c_void>(),
            2,
            ptr::null_mut(),
        );
        ASSERT_EQ(ret, 2, c"bytes".as_ptr());
        ret = bpf_prog_stream_read(
            prog_fd,
            BPF_STREAM_STDOUT,
            buf.as_mut_ptr().cast::<c_void>(),
            2,
            ptr::null_mut(),
        );
        ASSERT_EQ(ret, 1, c"bytes".as_ptr());
        ret = bpf_prog_stream_read(
            prog_fd,
            BPF_STREAM_STDOUT,
            buf.as_mut_ptr().cast::<c_void>(),
            1,
            &mut ropts,
        );
        ASSERT_EQ(ret, 0, c"no bytes stdout".as_ptr());
        ret = bpf_prog_stream_read(
            prog_fd,
            BPF_STREAM_STDERR,
            buf.as_mut_ptr().cast::<c_void>(),
            1,
            &mut ropts,
        );
        ASSERT_EQ(ret, 0, c"no bytes stderr".as_ptr());

        stream__destroy(skel);
    }
}

unsafe fn test_address(prog: *mut bpf_program, fault_addr_p: *mut c_ulong) {
    let mut opts = libbpf_opts_bpf_test_run_opts();
    let mut ropts = libbpf_opts_bpf_prog_stream_read_opts();
    let mut ret: c_int;
    let prog_fd: c_int;
    let mut fault_addr = [0 as c_char; 64];
    let mut buf = [0 as c_char; 1024];

    unsafe {
        prog_fd = bpf_program__fd(prog);

        ret = bpf_prog_test_run_opts(prog_fd, &mut opts);
        ASSERT_OK(ret, c"ret".as_ptr());
        ASSERT_OK(opts.retval as c_int, c"retval".as_ptr());

        sprintf(fault_addr.as_mut_ptr(), c"0x%lx".as_ptr(), *fault_addr_p);

        ret = bpf_prog_stream_read(
            prog_fd,
            BPF_STREAM_STDERR,
            buf.as_mut_ptr().cast::<c_void>(),
            mem::size_of_val(&buf),
            &mut ropts,
        );
        ASSERT_GT(ret, 0, c"stream read".as_ptr());
        ASSERT_LE(ret, 1023, c"len for buf".as_ptr());
        buf[ret as usize] = b'\0' as c_char;

        if !ASSERT_HAS_SUBSTR(buf.as_ptr(), fault_addr.as_ptr(), c"fault_addr".as_ptr()) {
            fprintf(stderr, c"Output from stream:\n%s\n".as_ptr(), buf.as_ptr());
            fprintf(stderr, c"Fault Addr: %s\n".as_ptr(), fault_addr.as_ptr());
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_stream_arena_fault_address() {
    let skel: *mut stream;

    // Original C condition:
    // #if !defined(__x86_64__) && !defined(__aarch64__)
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    unsafe {
        printf(
            c"%s:SKIP: arena fault reporting not supported\n".as_ptr(),
            c"test_stream_arena_fault_address".as_ptr(),
        );
        test__skip();
        return;
    }

    unsafe {
        skel = stream__open_and_load();
        if !ASSERT_OK_PTR(skel.cast::<c_void>(), c"stream__open_and_load".as_ptr()) {
            return;
        }

        if test__start_subtest(c"read_fault".as_ptr()) {
            test_address(
                (*skel).progs.stream_arena_read_fault,
                &mut (*(*skel).bss).fault_addr,
            );
        }
        if test__start_subtest(c"write_fault".as_ptr()) {
            test_address(
                (*skel).progs.stream_arena_write_fault,
                &mut (*(*skel).bss).fault_addr,
            );
        }
        if test__start_subtest(c"load_acquire_fault".as_ptr()) {
            test_address(
                (*skel).progs.stream_arena_load_acquire_fault,
                &mut (*(*skel).bss).fault_addr,
            );
        }
        if test__start_subtest(c"xchg_fault".as_ptr()) {
            test_address(
                (*skel).progs.stream_arena_xchg_fault,
                &mut (*(*skel).bss).fault_addr,
            );
        }
        if test__start_subtest(c"cmpxchg_fault".as_ptr()) {
            test_address(
                (*skel).progs.stream_arena_cmpxchg_fault,
                &mut (*(*skel).bss).fault_addr,
            );
        }

        stream__destroy(skel);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
