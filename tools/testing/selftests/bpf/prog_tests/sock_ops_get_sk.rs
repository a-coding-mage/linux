// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <test_progs.h>
// #include "cgroup_helpers.h"
// #include "network_helpers.h"
// #include "sock_ops_get_sk.skel.h"

use core::ffi::{c_char, c_int, c_void};

const AF_INET: c_int = 2;
const SOCK_STREAM: c_int = 1;
const BPF_CGROUP_SOCK_OPS: c_int = 15;

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sock_ops_get_sk_bss {
    pub null_seen: c_int,
    pub bug_detected: c_int,
    pub field_null_seen: c_int,
    pub field_bug_detected: c_int,
    pub diff_reg_null_seen: c_int,
    pub diff_reg_bug_detected: c_int,
}

#[repr(C)]
pub struct sock_ops_get_sk_progs {
    pub sock_ops_get_sk_same_reg: *mut bpf_program,
    pub sock_ops_get_field_same_reg: *mut bpf_program,
    pub sock_ops_get_sk_diff_reg: *mut bpf_program,
}

#[repr(C)]
pub struct sock_ops_get_sk {
    pub bss: *mut sock_ops_get_sk_bss,
    pub progs: sock_ops_get_sk_progs,
}

unsafe extern "C" {
    fn bpf_prog_attach(prog_fd: c_int, target_fd: c_int, attach_type: c_int, attach_flags: c_int)
        -> c_int;
    fn bpf_prog_detach(target_fd: c_int, attach_type: c_int) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn start_server(
        family: c_int,
        type_: c_int,
        addr: *const c_void,
        port: c_int,
        timeout_ms: c_int,
    ) -> c_int;
    fn connect_to_fd(server_fd: c_int, timeout_ms: c_int) -> c_int;
    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;

    fn sock_ops_get_sk__open_and_load() -> *mut sock_ops_get_sk;
    fn sock_ops_get_sk__destroy(obj: *mut sock_ops_get_sk);

    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
}

/* See progs/sock_ops_get_sk.c for the bug description. */
unsafe fn run_sock_ops_test(cgroup_fd: c_int, prog_fd: c_int) {
    let server_fd: c_int;
    let client_fd: c_int;
    let err: c_int;

    err = bpf_prog_attach(prog_fd, cgroup_fd, BPF_CGROUP_SOCK_OPS, 0);
    if !ASSERT_OK(err, c"prog_attach".as_ptr()) {
        return;
    }

    server_fd = start_server(AF_INET, SOCK_STREAM, core::ptr::null(), 0, 0);
    if !ASSERT_OK_FD(server_fd, c"start_server".as_ptr()) {
        bpf_prog_detach(cgroup_fd, BPF_CGROUP_SOCK_OPS);
        return;
    }

    /* Trigger TCP handshake which causes TCP_NEW_SYN_RECV state where
     * is_fullsock == 0 and is_locked_tcp_sock == 0.
     */
    client_fd = connect_to_fd(server_fd, 0);
    if !ASSERT_OK_FD(client_fd, c"connect_to_fd".as_ptr()) {
        close(server_fd);
        bpf_prog_detach(cgroup_fd, BPF_CGROUP_SOCK_OPS);
        return;
    }

    close(client_fd);

    close(server_fd);
    bpf_prog_detach(cgroup_fd, BPF_CGROUP_SOCK_OPS);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_ns_sock_ops_get_sk() {
    let skel: *mut sock_ops_get_sk;
    let cgroup_fd: c_int;

    cgroup_fd = test__join_cgroup(c"/sock_ops_get_sk".as_ptr());
    if !ASSERT_OK_FD(cgroup_fd, c"join_cgroup".as_ptr()) {
        return;
    }

    skel = sock_ops_get_sk__open_and_load();
    if !ASSERT_OK_PTR(skel.cast(), c"skel_open_load".as_ptr()) {
        close(cgroup_fd);
        return;
    }

    /* Test SOCK_OPS_GET_SK with same src/dst register */
    if test__start_subtest(c"get_sk".as_ptr()) {
        run_sock_ops_test(
            cgroup_fd,
            bpf_program__fd((*skel).progs.sock_ops_get_sk_same_reg),
        );
        ASSERT_EQ((*(*skel).bss).null_seen, 1, c"null_seen".as_ptr());
        ASSERT_EQ((*(*skel).bss).bug_detected, 0, c"bug_not_detected".as_ptr());
    }

    /* Test SOCK_OPS_GET_FIELD with same src/dst register */
    if test__start_subtest(c"get_field".as_ptr()) {
        run_sock_ops_test(
            cgroup_fd,
            bpf_program__fd((*skel).progs.sock_ops_get_field_same_reg),
        );
        ASSERT_EQ((*(*skel).bss).field_null_seen, 1, c"field_null_seen".as_ptr());
        ASSERT_EQ(
            (*(*skel).bss).field_bug_detected,
            0,
            c"field_bug_not_detected".as_ptr(),
        );
    }

    /* Test SOCK_OPS_GET_SK with different src/dst register */
    if test__start_subtest(c"get_sk_diff_reg".as_ptr()) {
        run_sock_ops_test(
            cgroup_fd,
            bpf_program__fd((*skel).progs.sock_ops_get_sk_diff_reg),
        );
        ASSERT_EQ(
            (*(*skel).bss).diff_reg_null_seen,
            1,
            c"diff_reg_null_seen".as_ptr(),
        );
        ASSERT_EQ(
            (*(*skel).bss).diff_reg_bug_detected,
            0,
            c"diff_reg_bug_not_detected".as_ptr(),
        );
    }

    sock_ops_get_sk__destroy(skel);
    close(cgroup_fd);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
