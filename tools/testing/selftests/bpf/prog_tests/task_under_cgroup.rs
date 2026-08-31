// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Bytedance */

// C dependencies translated from:
// <sys/syscall.h>, <test_progs.h>, <cgroup_helpers.h>,
// "test_task_under_cgroup.skel.h"

use core::ffi::{c_char, c_int, c_void};

const FOO: &[u8] = b"/foo\0";

type pid_t = c_int;

#[repr(C)]
pub struct test_task_under_cgroup_rodata {
    pub local_pid: pid_t,
    pub cgid: u64,
}

#[repr(C)]
pub struct test_task_under_cgroup_bss {
    pub remote_pid: pid_t,
}

#[repr(C)]
pub struct test_task_under_cgroup_progs {
    pub lsm_run: *mut c_void,
    pub tp_btf_run: *mut c_void,
}

#[repr(C)]
pub struct test_task_under_cgroup_links {
    pub lsm_run: *mut c_void,
    pub tp_btf_run: *mut c_void,
}

#[repr(C)]
pub struct test_task_under_cgroup {
    pub rodata: *mut test_task_under_cgroup_rodata,
    pub bss: *mut test_task_under_cgroup_bss,
    pub progs: test_task_under_cgroup_progs,
    pub links: test_task_under_cgroup_links,
}

unsafe extern "C" {
    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn get_cgroup_id(path: *const c_char) -> u64;

    fn test_task_under_cgroup__open() -> *mut test_task_under_cgroup;
    fn test_task_under_cgroup__load(skel: *mut test_task_under_cgroup) -> c_int;
    fn test_task_under_cgroup__detach(skel: *mut test_task_under_cgroup);
    fn test_task_under_cgroup__destroy(skel: *mut test_task_under_cgroup);

    fn bpf_program__attach_lsm(prog: *mut c_void) -> *mut c_void;
    fn bpf_program__attach_trace(prog: *mut c_void) -> *mut c_void;

    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_NEQ(a: pid_t, b: pid_t, name: *const c_char);

    fn getpid() -> pid_t;
    fn fork() -> pid_t;
    fn exit(status: c_int) -> !;
    fn wait(status: *mut c_int) -> pid_t;
    fn close(fd: c_int) -> c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_task_under_cgroup() {
    let mut skel: *mut test_task_under_cgroup;
    let mut ret: c_int;
    let foo: c_int;
    let pid: pid_t;

    foo = unsafe { test__join_cgroup(FOO.as_ptr() as *const c_char) };
    if !unsafe { ASSERT_OK((foo < 0) as c_int, c"cgroup_join_foo".as_ptr()) } {
        return;
    }

    skel = unsafe { test_task_under_cgroup__open() };
    if !unsafe { ASSERT_OK_PTR(skel as *const c_void, c"test_task_under_cgroup__open".as_ptr()) } {
        goto_cleanup(skel, foo);
        return;
    }

    unsafe {
        (*(*skel).rodata).local_pid = getpid();
        (*(*skel).bss).remote_pid = getpid();
        (*(*skel).rodata).cgid = get_cgroup_id(FOO.as_ptr() as *const c_char);
    }

    ret = unsafe { test_task_under_cgroup__load(skel) };
    if !unsafe { ASSERT_OK(ret, c"test_task_under_cgroup__load".as_ptr()) } {
        goto_cleanup(skel, foo);
        return;
    }

    /* First, attach the LSM program, and then it will be triggered when the
     * TP_BTF program is attached.
     */
    unsafe {
        (*skel).links.lsm_run = bpf_program__attach_lsm((*skel).progs.lsm_run);
    }
    if !unsafe { ASSERT_OK_PTR((*skel).links.lsm_run as *const c_void, c"attach_lsm".as_ptr()) } {
        goto_cleanup(skel, foo);
        return;
    }

    unsafe {
        (*skel).links.tp_btf_run = bpf_program__attach_trace((*skel).progs.tp_btf_run);
    }
    if !unsafe { ASSERT_OK_PTR((*skel).links.tp_btf_run as *const c_void, c"attach_tp_btf".as_ptr()) } {
        goto_cleanup(skel, foo);
        return;
    }

    pid = unsafe { fork() };
    if pid == 0 {
        unsafe { exit(0) };
    }

    ret = (pid == -1) as c_int;
    if unsafe { ASSERT_OK(ret, c"fork process".as_ptr()) } {
        unsafe {
            wait(core::ptr::null_mut());
        }
    }

    unsafe {
        test_task_under_cgroup__detach(skel);

        ASSERT_NEQ(
            (*(*skel).bss).remote_pid,
            (*(*skel).rodata).local_pid,
            c"test task_under_cgroup".as_ptr(),
        );
    }

    goto_cleanup(skel, foo);
}

unsafe fn goto_cleanup(skel: *mut test_task_under_cgroup, foo: c_int) {
    unsafe {
        test_task_under_cgroup__destroy(skel);
        close(foo);
    }
}
