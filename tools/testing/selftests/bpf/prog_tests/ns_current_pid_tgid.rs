// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Carlos Neira cneirabustos@gmail.com */

/* Translated from:
 * - <test_progs.h>
 * - "test_ns_current_pid_tgid.skel.h"
 * - <sys/stat.h>
 * - <sys/types.h>
 * - <unistd.h>
 * - <sys/syscall.h>
 * - <sched.h>
 * - <sys/wait.h>
 * - <sys/mount.h>
 * - <fcntl.h>
 * - "network_helpers.h"
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type pid_t = c_int;
type size_t = usize;
type ssize_t = isize;
type dev_t = u64;
type ino_t = u64;

const STACK_SIZE: usize = 1024 * 1024;
static mut child_stack: [c_char; STACK_SIZE] = [0; STACK_SIZE];

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const CLONE_NEWPID: c_int = 0x20000000;
const SIGCHLD: c_int = 17;
const BPF_ANY: u64 = 0;
const BPF_SK_MSG_VERDICT: c_uint = 7;

#[repr(C)]
struct stat {
    /* Only fields used by this file are represented here; the real layout is
     * provided by the target C ABI dependency.
     */
    st_dev: dev_t,
    st_ino: ino_t,
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct test_ns_current_pid_tgid__bss {
    dev: dev_t,
    ino: ino_t,
    user_pid: pid_t,
    user_tgid: pid_t,
}

#[repr(C)]
struct test_ns_current_pid_tgid__progs {
    tp_handler: *mut bpf_program,
    cgroup_bind4: *mut bpf_program,
    sk_msg: *mut bpf_program,
}

#[repr(C)]
struct test_ns_current_pid_tgid__maps {
    sock_map: *mut bpf_map,
}

#[repr(C)]
struct test_ns_current_pid_tgid__links {
    cgroup_bind4: *mut bpf_link,
}

#[repr(C)]
struct test_ns_current_pid_tgid {
    progs: test_ns_current_pid_tgid__progs,
    maps: test_ns_current_pid_tgid__maps,
    links: test_ns_current_pid_tgid__links,
    bss: *mut test_ns_current_pid_tgid__bss,
}

unsafe extern "C" {
    fn sys_gettid() -> pid_t;
    fn getpid() -> pid_t;
    fn stat(path: *const c_char, buf: *mut stat) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn clone(
        fn_: unsafe extern "C" fn(*mut c_void) -> c_int,
        child_stack: *mut c_void,
        flags: c_int,
        arg: *mut c_void,
    ) -> pid_t;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn send(sockfd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t;

    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_NEQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn test__join_cgroup(path: *const c_char) -> c_int;

    fn test_ns_current_pid_tgid__open() -> *mut test_ns_current_pid_tgid;
    fn test_ns_current_pid_tgid__load(skel: *mut test_ns_current_pid_tgid) -> c_int;
    fn test_ns_current_pid_tgid__attach(skel: *mut test_ns_current_pid_tgid) -> c_int;
    fn test_ns_current_pid_tgid__destroy(skel: *mut test_ns_current_pid_tgid);

    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_prog_attach(prog_fd: c_int, target_fd: c_int, type_: c_uint, flags: c_uint) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;

    fn start_server(
        family: c_int,
        type_: c_int,
        addr: *const c_char,
        port: c_int,
        timeout_ms: c_int,
    ) -> c_int;
    fn connect_to_fd(server_fd: c_int, timeout_ms: c_int) -> c_int;
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn get_pid_tgid(
    pid: *mut pid_t,
    tgid: *mut pid_t,
    bss: *mut test_ns_current_pid_tgid__bss,
) -> c_int {
    let mut st: stat = core::mem::zeroed();
    let err: c_int;

    *pid = sys_gettid();
    *tgid = getpid();

    err = stat(c"/proc/self/ns/pid".as_ptr(), &mut st);
    if !ASSERT_OK(err, c"stat /proc/self/ns/pid".as_ptr()) {
        return err;
    }

    (*bss).dev = st.st_dev;
    (*bss).ino = st.st_ino;
    (*bss).user_pid = 0;
    (*bss).user_tgid = 0;
    0
}

unsafe extern "C" fn test_current_pid_tgid_tp(_args: *mut c_void) -> c_int {
    let bss: *mut test_ns_current_pid_tgid__bss;
    let skel: *mut test_ns_current_pid_tgid;
    let mut ret: c_int = -1;
    let mut err: c_int;
    let mut tgid: pid_t = 0;
    let mut pid: pid_t = 0;

    skel = test_ns_current_pid_tgid__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"test_ns_current_pid_tgid__open".as_ptr()) {
        return ret;
    }

    bpf_program__set_autoload((*skel).progs.tp_handler, true);

    err = test_ns_current_pid_tgid__load(skel);
    if !ASSERT_OK(err, c"test_ns_current_pid_tgid__load".as_ptr()) {
        goto_cleanup_tp(skel);
        return ret;
    }

    bss = (*skel).bss;
    if get_pid_tgid(&mut pid, &mut tgid, bss) != 0 {
        goto_cleanup_tp(skel);
        return ret;
    }

    err = test_ns_current_pid_tgid__attach(skel);
    if !ASSERT_OK(err, c"test_ns_current_pid_tgid__attach".as_ptr()) {
        goto_cleanup_tp(skel);
        return ret;
    }

    /* trigger tracepoint */
    usleep(1);
    if !ASSERT_EQ((*bss).user_pid, pid, c"pid".as_ptr()) {
        goto_cleanup_tp(skel);
        return ret;
    }
    if !ASSERT_EQ((*bss).user_tgid, tgid, c"tgid".as_ptr()) {
        goto_cleanup_tp(skel);
        return ret;
    }
    ret = 0;

    goto_cleanup_tp(skel);
    ret
}

unsafe fn goto_cleanup_tp(skel: *mut test_ns_current_pid_tgid) {
    test_ns_current_pid_tgid__destroy(skel);
}

unsafe extern "C" fn test_current_pid_tgid_cgrp(args: *mut c_void) -> c_int {
    let bss: *mut test_ns_current_pid_tgid__bss;
    let skel: *mut test_ns_current_pid_tgid;
    let mut server_fd: c_int = -1;
    let mut ret: c_int = -1;
    let mut err: c_int;
    let cgroup_fd: c_int = *(args as *mut c_int);
    let mut tgid: pid_t = 0;
    let mut pid: pid_t = 0;

    skel = test_ns_current_pid_tgid__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"test_ns_current_pid_tgid__open".as_ptr()) {
        return ret;
    }

    bpf_program__set_autoload((*skel).progs.cgroup_bind4, true);

    err = test_ns_current_pid_tgid__load(skel);
    if !ASSERT_OK(err, c"test_ns_current_pid_tgid__load".as_ptr()) {
        goto_cleanup_cgrp(server_fd, skel);
        return ret;
    }

    bss = (*skel).bss;
    if get_pid_tgid(&mut pid, &mut tgid, bss) != 0 {
        goto_cleanup_cgrp(server_fd, skel);
        return ret;
    }

    (*skel).links.cgroup_bind4 =
        bpf_program__attach_cgroup((*skel).progs.cgroup_bind4, cgroup_fd);
    if !ASSERT_OK_PTR(
        (*skel).links.cgroup_bind4 as *const c_void,
        c"bpf_program__attach_cgroup".as_ptr(),
    ) {
        goto_cleanup_cgrp(server_fd, skel);
        return ret;
    }

    server_fd = start_server(AF_INET, SOCK_STREAM, ptr::null(), 0, 0);
    if !ASSERT_GE(server_fd, 0, c"start_server".as_ptr()) {
        goto_cleanup_cgrp(server_fd, skel);
        return ret;
    }

    if !ASSERT_EQ((*bss).user_pid, pid, c"pid".as_ptr()) {
        goto_cleanup_cgrp(server_fd, skel);
        return ret;
    }
    if !ASSERT_EQ((*bss).user_tgid, tgid, c"tgid".as_ptr()) {
        goto_cleanup_cgrp(server_fd, skel);
        return ret;
    }
    ret = 0;

    goto_cleanup_cgrp(server_fd, skel);
    ret
}

unsafe fn goto_cleanup_cgrp(server_fd: c_int, skel: *mut test_ns_current_pid_tgid) {
    if server_fd >= 0 {
        close(server_fd);
    }
    test_ns_current_pid_tgid__destroy(skel);
}

unsafe extern "C" fn test_current_pid_tgid_sk_msg(_args: *mut c_void) -> c_int {
    let mut verdict: c_int;
    let mut map: c_int;
    let mut server_fd: c_int = -1;
    let mut client_fd: c_int = -1;
    let bss: *mut test_ns_current_pid_tgid__bss;
    static send_msg: &[u8; 8] = b"message\0";
    let skel: *mut test_ns_current_pid_tgid;
    let mut ret: c_int = -1;
    let mut err: c_int;
    let mut key: c_int = 0;
    let mut tgid: pid_t = 0;
    let mut pid: pid_t = 0;

    skel = test_ns_current_pid_tgid__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"test_ns_current_pid_tgid__open".as_ptr()) {
        return ret;
    }

    bpf_program__set_autoload((*skel).progs.sk_msg, true);

    err = test_ns_current_pid_tgid__load(skel);
    if !ASSERT_OK(err, c"test_ns_current_pid_tgid__load".as_ptr()) {
        goto_cleanup_sk_msg(server_fd, client_fd, skel);
        return ret;
    }

    bss = (*skel).bss;
    if get_pid_tgid(&mut pid, &mut tgid, (*skel).bss) != 0 {
        goto_cleanup_sk_msg(server_fd, client_fd, skel);
        return ret;
    }

    verdict = bpf_program__fd((*skel).progs.sk_msg);
    map = bpf_map__fd((*skel).maps.sock_map);
    err = bpf_prog_attach(verdict, map, BPF_SK_MSG_VERDICT, 0);
    if !ASSERT_OK(err, c"prog_attach".as_ptr()) {
        goto_cleanup_sk_msg(server_fd, client_fd, skel);
        return ret;
    }

    server_fd = start_server(AF_INET6, SOCK_STREAM, c"::1".as_ptr(), 0, 0);
    if !ASSERT_GE(server_fd, 0, c"start_server".as_ptr()) {
        goto_cleanup_sk_msg(server_fd, client_fd, skel);
        return ret;
    }

    client_fd = connect_to_fd(server_fd, 0);
    if !ASSERT_GE(client_fd, 0, c"connect_to_fd".as_ptr()) {
        goto_cleanup_sk_msg(server_fd, client_fd, skel);
        return ret;
    }

    err = bpf_map_update_elem(
        map,
        &key as *const c_int as *const c_void,
        &client_fd as *const c_int as *const c_void,
        BPF_ANY,
    );
    if !ASSERT_OK(err, c"bpf_map_update_elem".as_ptr()) {
        goto_cleanup_sk_msg(server_fd, client_fd, skel);
        return ret;
    }

    err = send(
        client_fd,
        send_msg.as_ptr() as *const c_void,
        core::mem::size_of_val(send_msg),
        0,
    ) as c_int;
    if !ASSERT_EQ(err, core::mem::size_of_val(send_msg) as c_int, c"send(msg)".as_ptr()) {
        goto_cleanup_sk_msg(server_fd, client_fd, skel);
        return ret;
    }

    if !ASSERT_EQ((*bss).user_pid, pid, c"pid".as_ptr()) {
        goto_cleanup_sk_msg(server_fd, client_fd, skel);
        return ret;
    }
    if !ASSERT_EQ((*bss).user_tgid, tgid, c"tgid".as_ptr()) {
        goto_cleanup_sk_msg(server_fd, client_fd, skel);
        return ret;
    }
    ret = 0;

    goto_cleanup_sk_msg(server_fd, client_fd, skel);
    ret
}

unsafe fn goto_cleanup_sk_msg(
    server_fd: c_int,
    client_fd: c_int,
    skel: *mut test_ns_current_pid_tgid,
) {
    if server_fd >= 0 {
        close(server_fd);
    }
    if client_fd >= 0 {
        close(client_fd);
    }
    test_ns_current_pid_tgid__destroy(skel);
}

unsafe fn test_ns_current_pid_tgid_new_ns(
    fn_: unsafe extern "C" fn(*mut c_void) -> c_int,
    arg: *mut c_void,
) {
    let mut wstatus: c_int = 0;
    let cpid: pid_t;

    /* Create a process in a new namespace, this process
     * will be the init process of this new namespace hence will be pid 1.
     */
    cpid = clone(
        fn_,
        child_stack.as_mut_ptr().add(STACK_SIZE) as *mut c_void,
        CLONE_NEWPID | SIGCHLD,
        arg,
    );

    if !ASSERT_NEQ(cpid, -1, c"clone".as_ptr()) {
        return;
    }

    if !ASSERT_NEQ(waitpid(cpid, &mut wstatus, 0), -1, c"waitpid".as_ptr()) {
        return;
    }

    if !ASSERT_OK(WEXITSTATUS(wstatus), c"newns_pidtgid".as_ptr()) {
        return;
    }
}

/* TODO: use a different tracepoint */
#[no_mangle]
pub unsafe extern "C" fn serial_test_current_pid_tgid() {
    if test__start_subtest(c"root_ns_tp".as_ptr()) {
        test_current_pid_tgid_tp(ptr::null_mut());
    }
    if test__start_subtest(c"new_ns_tp".as_ptr()) {
        test_ns_current_pid_tgid_new_ns(test_current_pid_tgid_tp, ptr::null_mut());
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_ns_current_pid_tgid_cgrp() {
    let mut cgroup_fd: c_int = test__join_cgroup(c"/sock_addr".as_ptr());

    if ASSERT_OK_FD(cgroup_fd, c"join_cgroup".as_ptr()) {
        test_ns_current_pid_tgid_new_ns(
            test_current_pid_tgid_cgrp,
            &mut cgroup_fd as *mut c_int as *mut c_void,
        );
        close(cgroup_fd);
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_ns_current_pid_tgid_sk_msg() {
    test_ns_current_pid_tgid_new_ns(test_current_pid_tgid_sk_msg, ptr::null_mut());
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
