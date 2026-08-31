// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

// C dependencies translated as external declarations:
// cgroup_helpers.h, test_progs.h, sched.h, sys/wait.h
// cgrp_kfunc_failure.skel.h, cgrp_kfunc_success.skel.h

use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

const CLONE_NEWCGROUP: c_int = 0x02000000;

#[repr(C)]
pub struct bpf_object {
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
pub struct cgrp_kfunc_success_bss {
    pub pid: c_int,
    pub err: c_int,
    pub invocations: c_int,
}

#[repr(C)]
pub struct cgrp_kfunc_success_progs {
    pub test_cgrp_from_id_ns: *mut bpf_program,
}

#[repr(C)]
pub struct cgrp_kfunc_success {
    pub obj: *mut bpf_object,
    pub progs: cgrp_kfunc_success_progs,
    pub bss: *mut cgrp_kfunc_success_bss,
}

// File-local stand-in for LIBBPF_OPTS(bpf_test_run_opts, opts).
// The complete layout is supplied by libbpf headers in the original build.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub retval: u32,
}

impl Default for bpf_test_run_opts {
    fn default() -> Self {
        Self {
            sz: mem::size_of::<bpf_test_run_opts>(),
            retval: 0,
        }
    }
}

unsafe extern "C" {
    fn getpid() -> c_int;
    fn close(fd: c_int) -> c_int;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn fork() -> c_int;
    fn exit(status: c_int) -> !;
    fn unshare(flags: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn waitpid(pid: c_int, stat_loc: *mut c_int, options: c_int) -> c_int;

    fn create_and_get_cgroup(path: *const c_char) -> c_int;
    fn remove_cgroup(path: *const c_char);
    fn cgroup_setup_and_join(path: *const c_char) -> c_int;
    fn remove_cgroup_pid(path: *const c_char, pid: c_int);
    fn setup_cgroup_environment() -> c_int;
    fn cleanup_cgroup_environment();

    fn test__start_subtest(name: *const c_char) -> bool;

    fn cgrp_kfunc_success__open() -> *mut cgrp_kfunc_success;
    fn cgrp_kfunc_success__load(skel: *mut cgrp_kfunc_success) -> c_int;
    fn cgrp_kfunc_success__destroy(skel: *mut cgrp_kfunc_success);

    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: isize, expected: isize, name: *const c_char) -> bool;

    fn RUN_TESTS_cgrp_kfunc_failure();
}

unsafe fn open_load_cgrp_kfunc_skel() -> *mut cgrp_kfunc_success {
    let skel: *mut cgrp_kfunc_success;
    let err: c_int;

    skel = cgrp_kfunc_success__open();
    if !ASSERT_OK_PTR(skel as *const c_void, b"skel_open\0".as_ptr() as *const c_char) {
        return ptr::null_mut();
    }

    (*(*skel).bss).pid = getpid();

    err = cgrp_kfunc_success__load(skel);
    if !ASSERT_OK(err, b"skel_load\0".as_ptr() as *const c_char) {
        cgrp_kfunc_success__destroy(skel);
        return ptr::null_mut();
    }

    skel
}

unsafe fn mkdir_rm_test_dir() -> c_int {
    let fd: c_int;
    let cgrp_path: *const c_char = b"cgrp_kfunc\0".as_ptr() as *const c_char;

    fd = create_and_get_cgroup(cgrp_path);
    if !ASSERT_GT(fd, 0, b"mkdir_cgrp_fd\0".as_ptr() as *const c_char) {
        return -1;
    }

    close(fd);
    remove_cgroup(cgrp_path);

    0
}

unsafe fn run_success_test(prog_name: *const c_char) {
    let skel: *mut cgrp_kfunc_success;
    let prog: *mut bpf_program;
    let mut link: *mut bpf_link = ptr::null_mut();

    skel = open_load_cgrp_kfunc_skel();
    if !ASSERT_OK_PTR(skel as *const c_void, b"open_load_skel\0".as_ptr() as *const c_char) {
        return;
    }

    if !ASSERT_OK((*(*skel).bss).err, b"pre_mkdir_err\0".as_ptr() as *const c_char) {
        bpf_link__destroy(link);
        cgrp_kfunc_success__destroy(skel);
        return;
    }

    prog = bpf_object__find_program_by_name((*skel).obj, prog_name);
    if !ASSERT_OK_PTR(
        prog as *const c_void,
        b"bpf_object__find_program_by_name\0".as_ptr() as *const c_char,
    ) {
        bpf_link__destroy(link);
        cgrp_kfunc_success__destroy(skel);
        return;
    }

    link = bpf_program__attach(prog);
    if !ASSERT_OK_PTR(link as *const c_void, b"attached_link\0".as_ptr() as *const c_char) {
        bpf_link__destroy(link);
        cgrp_kfunc_success__destroy(skel);
        return;
    }

    ASSERT_EQ(
        (*(*skel).bss).invocations as isize,
        0,
        b"pre_rmdir_count\0".as_ptr() as *const c_char,
    );
    if !ASSERT_OK(mkdir_rm_test_dir(), b"cgrp_mkdir\0".as_ptr() as *const c_char) {
        bpf_link__destroy(link);
        cgrp_kfunc_success__destroy(skel);
        return;
    }

    ASSERT_EQ(
        (*(*skel).bss).invocations as isize,
        1,
        b"post_rmdir_count\0".as_ptr() as *const c_char,
    );
    ASSERT_OK((*(*skel).bss).err, b"post_rmdir_err\0".as_ptr() as *const c_char);

    bpf_link__destroy(link);
    cgrp_kfunc_success__destroy(skel);
}

static SUCCESS_TESTS: [*const c_char; 6] = [
    b"test_cgrp_acquire_release_argument\0".as_ptr() as *const c_char,
    b"test_cgrp_acquire_leave_in_map\0".as_ptr() as *const c_char,
    b"test_cgrp_xchg_release\0".as_ptr() as *const c_char,
    b"test_cgrp_get_release\0".as_ptr() as *const c_char,
    b"test_cgrp_get_ancestors\0".as_ptr() as *const c_char,
    b"test_cgrp_from_id\0".as_ptr() as *const c_char,
];

unsafe fn test_cgrp_from_id_ns() {
    let mut opts = bpf_test_run_opts::default();
    let skel: *mut cgrp_kfunc_success;
    let prog: *mut bpf_program;
    let pid: c_int;
    let mut pipe_fd: [c_int; 2] = [0; 2];

    skel = open_load_cgrp_kfunc_skel();
    if !ASSERT_OK_PTR(skel as *const c_void, b"open_load_skel\0".as_ptr() as *const c_char) {
        return;
    }

    if !ASSERT_OK((*(*skel).bss).err, b"pre_mkdir_err\0".as_ptr() as *const c_char) {
        cgrp_kfunc_success__destroy(skel);
        return;
    }

    prog = (*skel).progs.test_cgrp_from_id_ns;

    if !ASSERT_OK(pipe(pipe_fd.as_mut_ptr()), b"pipe\0".as_ptr() as *const c_char) {
        cgrp_kfunc_success__destroy(skel);
        return;
    }

    pid = fork();
    if !ASSERT_GE(pid, 0, b"fork result\0".as_ptr() as *const c_char) {
        close(pipe_fd[0]);
        close(pipe_fd[1]);
        cgrp_kfunc_success__destroy(skel);
        return;
    }

    if pid == 0 {
        let mut ret: c_int = 0;

        close(pipe_fd[0]);

        if !ASSERT_GE(
            cgroup_setup_and_join(b"cgrp_from_id_ns\0".as_ptr() as *const c_char),
            0,
            b"join cgroup\0".as_ptr() as *const c_char,
        ) {
            exit(1);
        }

        if !ASSERT_OK(unshare(CLONE_NEWCGROUP), b"unshare cgns\0".as_ptr() as *const c_char) {
            exit(1);
        }

        ret = bpf_prog_test_run_opts(bpf_program__fd(prog), &mut opts);
        if !ASSERT_OK(ret, b"test run ret\0".as_ptr() as *const c_char) {
            exit(1);
        }

        if !ASSERT_OK(opts.retval as c_int, b"test run retval\0".as_ptr() as *const c_char) {
            exit(1);
        }

        if !ASSERT_EQ(
            write(
                pipe_fd[1],
                &ret as *const c_int as *const c_void,
                mem::size_of_val(&ret),
            ),
            mem::size_of_val(&ret) as isize,
            b"write pipe\0".as_ptr() as *const c_char,
        ) {
            exit(1);
        }

        exit(0);
    } else {
        let mut res: c_int = 0;

        close(pipe_fd[1]);

        ASSERT_EQ(
            read(
                pipe_fd[0],
                &mut res as *mut c_int as *mut c_void,
                mem::size_of_val(&res),
            ),
            mem::size_of_val(&res) as isize,
            b"read res\0".as_ptr() as *const c_char,
        );
        ASSERT_EQ(
            waitpid(pid, ptr::null_mut(), 0) as isize,
            pid as isize,
            b"wait on child\0".as_ptr() as *const c_char,
        );

        remove_cgroup_pid(b"cgrp_from_id_ns\0".as_ptr() as *const c_char, pid);

        ASSERT_OK(res, b"result from run\0".as_ptr() as *const c_char);
    }

    close(pipe_fd[0]);
    cgrp_kfunc_success__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_cgrp_kfunc() {
    let mut i: usize;
    let err: c_int;

    err = setup_cgroup_environment();
    if !ASSERT_OK(err, b"cgrp_env_setup\0".as_ptr() as *const c_char) {
        cleanup_cgroup_environment();
        return;
    }

    i = 0;
    while i < SUCCESS_TESTS.len() {
        if !test__start_subtest(SUCCESS_TESTS[i]) {
            i += 1;
            continue;
        }

        run_success_test(SUCCESS_TESTS[i]);
        i += 1;
    }

    if test__start_subtest(b"test_cgrp_from_id_ns\0".as_ptr() as *const c_char) {
        test_cgrp_from_id_ns();
    }

    RUN_TESTS_cgrp_kfunc_failure();

    cleanup_cgroup_environment();
}
