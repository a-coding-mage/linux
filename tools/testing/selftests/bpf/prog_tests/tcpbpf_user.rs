// SPDX-License-Identifier: GPL-2.0
// C dependencies: <test_progs.h>, <network_helpers.h>,
// "test_tcpbpf.h", and "test_tcpbpf_kern.skel.h".

use core::ffi::{c_char, c_int, c_void};

const LO_ADDR6: &[u8] = b"::1\0";
const CG_NAME: &[u8] = b"/tcpbpf-user-test\0";

const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SHUT_WR: c_int = 1;

// Constants supplied by test_tcpbpf.h / BPF sock_ops UAPI in the original C.
extern "C" {
    static BPF_SOCK_OPS_TIMEOUT_INIT: c_int;
    static BPF_SOCK_OPS_RWND_INIT: c_int;
    static BPF_SOCK_OPS_TCP_CONNECT_CB: c_int;
    static BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB: c_int;
    static BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB: c_int;
    static BPF_SOCK_OPS_NEEDS_ECN: c_int;
    static BPF_SOCK_OPS_STATE_CB: c_int;
    static BPF_SOCK_OPS_TCP_LISTEN_CB: c_int;
}

#[repr(C)]
pub struct tcpbpf_globals {
    pub event_map: u32,
    pub bytes_received: u32,
    pub bytes_acked: u32,
    pub data_segs_in: u32,
    pub data_segs_out: u32,
    pub bad_cb_test_rv: u32,
    pub good_cb_test_rv: u32,
    pub num_listen: u32,
    pub num_close_events: u32,
    pub tcp_save_syn: u32,
    pub tcp_saved_syn: u32,
    pub window_clamp_client: u32,
    pub window_clamp_server: u32,
}

#[repr(C)]
pub struct test_tcpbpf_kern {
    pub links: test_tcpbpf_kern__links,
    pub progs: test_tcpbpf_kern__progs,
    pub bss: *mut test_tcpbpf_kern__bss,
}

#[repr(C)]
pub struct test_tcpbpf_kern__links {
    pub bpf_testcb: *mut bpf_link,
}

#[repr(C)]
pub struct test_tcpbpf_kern__progs {
    pub bpf_testcb: *mut bpf_program,
}

#[repr(C)]
pub struct test_tcpbpf_kern__bss {
    pub global: tcpbpf_globals,
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

extern "C" {
    fn start_server(
        family: c_int,
        type_: c_int,
        addr: *const c_char,
        port: c_int,
        timeout_ms: c_int,
    ) -> c_int;
    fn connect_to_fd(fd: c_int, timeout_ms: c_int) -> c_int;
    fn accept(fd: c_int, addr: *mut c_void, addrlen: *mut c_void) -> c_int;
    fn send(fd: c_int, buf: *const c_void, len: usize, flags: c_int) -> isize;
    fn recv(fd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> isize;
    fn shutdown(fd: c_int, how: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn test_tcpbpf_kern__open_and_load() -> *mut test_tcpbpf_kern;
    fn test_tcpbpf_kern__destroy(obj: *mut test_tcpbpf_kern);
    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;

    fn ASSERT_EQ(actual: u64, expected: u64, name: *const c_char) -> bool;
    fn ASSERT_NEQ(actual: i64, expected: i64, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: i64, expected: i64, name: *const c_char) -> bool;
    fn ASSERT_OK(err: i64, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
}

unsafe fn verify_result(result: *mut tcpbpf_globals) {
    let expected_events: u32 = ((1u32 << BPF_SOCK_OPS_TIMEOUT_INIT)
        | (1u32 << BPF_SOCK_OPS_RWND_INIT)
        | (1u32 << BPF_SOCK_OPS_TCP_CONNECT_CB)
        | (1u32 << BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB)
        | (1u32 << BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB)
        | (1u32 << BPF_SOCK_OPS_NEEDS_ECN)
        | (1u32 << BPF_SOCK_OPS_STATE_CB)
        | (1u32 << BPF_SOCK_OPS_TCP_LISTEN_CB)) as u32;

    /* check global map */
    ASSERT_EQ(expected_events as u64, (*result).event_map as u64, b"event_map\0".as_ptr() as *const c_char);

    ASSERT_EQ((*result).bytes_received as u64, 501, b"bytes_received\0".as_ptr() as *const c_char);
    ASSERT_EQ((*result).bytes_acked as u64, 1002, b"bytes_acked\0".as_ptr() as *const c_char);
    ASSERT_EQ((*result).data_segs_in as u64, 1, b"data_segs_in\0".as_ptr() as *const c_char);
    ASSERT_EQ((*result).data_segs_out as u64, 1, b"data_segs_out\0".as_ptr() as *const c_char);
    ASSERT_EQ((*result).bad_cb_test_rv as u64, 0x80, b"bad_cb_test_rv\0".as_ptr() as *const c_char);
    ASSERT_EQ((*result).good_cb_test_rv as u64, 0, b"good_cb_test_rv\0".as_ptr() as *const c_char);
    ASSERT_EQ((*result).num_listen as u64, 1, b"num_listen\0".as_ptr() as *const c_char);

    /* 3 comes from one listening socket + both ends of the connection */
    ASSERT_EQ((*result).num_close_events as u64, 3, b"num_close_events\0".as_ptr() as *const c_char);

    /* check setsockopt for SAVE_SYN */
    ASSERT_EQ((*result).tcp_save_syn as u64, 0, b"tcp_save_syn\0".as_ptr() as *const c_char);

    /* check getsockopt for SAVED_SYN */
    ASSERT_EQ((*result).tcp_saved_syn as u64, 1, b"tcp_saved_syn\0".as_ptr() as *const c_char);

    /* check getsockopt for window_clamp */
    ASSERT_EQ((*result).window_clamp_client as u64, 9216, b"window_clamp_client\0".as_ptr() as *const c_char);
    ASSERT_EQ((*result).window_clamp_server as u64, 9216, b"window_clamp_server\0".as_ptr() as *const c_char);
}

unsafe fn run_test(result: *mut tcpbpf_globals) {
    let mut listen_fd: c_int = -1;
    let mut cli_fd: c_int = -1;
    let mut accept_fd: c_int = -1;
    let mut buf = [0 as c_char; 1000];
    let mut err: c_int = -1;
    let mut i: c_int;
    let mut rv: isize;

    listen_fd = start_server(AF_INET6, SOCK_STREAM, LO_ADDR6.as_ptr() as *const c_char, 0, 0);
    if !ASSERT_NEQ(listen_fd as i64, -1, b"start_server\0".as_ptr() as *const c_char) {
        goto_done(result, accept_fd, cli_fd, listen_fd, err);
        return;
    }

    cli_fd = connect_to_fd(listen_fd, 0);
    if !ASSERT_NEQ(cli_fd as i64, -1, b"connect_to_fd(listen_fd)\0".as_ptr() as *const c_char) {
        goto_done(result, accept_fd, cli_fd, listen_fd, err);
        return;
    }

    accept_fd = accept(listen_fd, core::ptr::null_mut(), core::ptr::null_mut());
    if !ASSERT_NEQ(accept_fd as i64, -1, b"accept(listen_fd)\0".as_ptr() as *const c_char) {
        goto_done(result, accept_fd, cli_fd, listen_fd, err);
        return;
    }

    /* Send 1000B of '+'s from cli_fd -> accept_fd */
    i = 0;
    while i < 1000 {
        buf[i as usize] = b'+' as c_char;
        i += 1;
    }

    rv = send(cli_fd, buf.as_ptr() as *const c_void, 1000, 0);
    if !ASSERT_EQ(rv as u64, 1000, b"send(cli_fd)\0".as_ptr() as *const c_char) {
        goto_done(result, accept_fd, cli_fd, listen_fd, err);
        return;
    }

    rv = recv(accept_fd, buf.as_mut_ptr() as *mut c_void, 1000, 0);
    if !ASSERT_EQ(rv as u64, 1000, b"recv(accept_fd)\0".as_ptr() as *const c_char) {
        goto_done(result, accept_fd, cli_fd, listen_fd, err);
        return;
    }

    /* Send 500B of '.'s from accept_fd ->cli_fd */
    i = 0;
    while i < 500 {
        buf[i as usize] = b'.' as c_char;
        i += 1;
    }

    rv = send(accept_fd, buf.as_ptr() as *const c_void, 500, 0);
    if !ASSERT_EQ(rv as u64, 500, b"send(accept_fd)\0".as_ptr() as *const c_char) {
        goto_done(result, accept_fd, cli_fd, listen_fd, err);
        return;
    }

    rv = recv(cli_fd, buf.as_mut_ptr() as *mut c_void, 500, 0);
    if !ASSERT_EQ(rv as u64, 500, b"recv(cli_fd)\0".as_ptr() as *const c_char) {
        goto_done(result, accept_fd, cli_fd, listen_fd, err);
        return;
    }

    /*
     * shutdown accept first to guarantee correct ordering for
     * bytes_received and bytes_acked when we go to verify the results.
     */
    shutdown(accept_fd, SHUT_WR);
    err = recv(cli_fd, buf.as_mut_ptr() as *mut c_void, 1, 0) as c_int;
    if !ASSERT_OK(err as i64, b"recv(cli_fd) for fin\0".as_ptr() as *const c_char) {
        goto_done(result, accept_fd, cli_fd, listen_fd, err);
        return;
    }

    shutdown(cli_fd, SHUT_WR);
    err = recv(accept_fd, buf.as_mut_ptr() as *mut c_void, 1, 0) as c_int;
    ASSERT_OK(err as i64, b"recv(accept_fd) for fin\0".as_ptr() as *const c_char);

    goto_done(result, accept_fd, cli_fd, listen_fd, err);
}

unsafe fn goto_done(
    result: *mut tcpbpf_globals,
    accept_fd: c_int,
    cli_fd: c_int,
    listen_fd: c_int,
    err: c_int,
) {
    if accept_fd != -1 {
        close(accept_fd);
    }
    if cli_fd != -1 {
        close(cli_fd);
    }
    if listen_fd != -1 {
        close(listen_fd);
    }

    if err == 0 {
        verify_result(result);
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_tcpbpf_user() {
    let mut skel: *mut test_tcpbpf_kern;
    let mut cg_fd: c_int = -1;

    skel = test_tcpbpf_kern__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, b"open and load skel\0".as_ptr() as *const c_char) {
        return;
    }

    cg_fd = test__join_cgroup(CG_NAME.as_ptr() as *const c_char);
    if !ASSERT_GE(cg_fd as i64, 0, b"test__join_cgroup(/tcpbpf-user-test)\0".as_ptr() as *const c_char) {
        goto_err(skel, cg_fd);
        return;
    }

    (*skel).links.bpf_testcb = bpf_program__attach_cgroup((*skel).progs.bpf_testcb, cg_fd);
    if !ASSERT_OK_PTR(
        (*skel).links.bpf_testcb as *const c_void,
        b"attach_cgroup(bpf_testcb)\0".as_ptr() as *const c_char,
    ) {
        goto_err(skel, cg_fd);
        return;
    }

    run_test(&mut (*(*skel).bss).global);

    goto_err(skel, cg_fd);
}

unsafe fn goto_err(skel: *mut test_tcpbpf_kern, cg_fd: c_int) {
    if cg_fd != -1 {
        close(cg_fd);
    }
    test_tcpbpf_kern__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
