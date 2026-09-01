// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook

// C dependencies translated as external declarations:
// <test_progs.h>
// "network_helpers.h"
// "cgroup_skb_sk_lookup_kern.skel.h"

use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

type __u16 = u16;
type __u32 = u32;
type socklen_t = u32;

const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const EINPROGRESS: c_int = 115;

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
struct sockaddr_in6 {
    sin6_family: u16,
    sin6_port: __u16,
    sin6_flowinfo: __u32,
    sin6_addr: in6_addr,
    sin6_scope_id: __u32,
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct cgroup_skb_sk_lookup_kern__progs {
    ingress_lookup: *mut bpf_program,
}

#[repr(C)]
struct cgroup_skb_sk_lookup_kern__bss {
    g_serv_port: __u16,
}

#[repr(C)]
struct cgroup_skb_sk_lookup_kern {
    progs: cgroup_skb_sk_lookup_kern__progs,
    bss: *mut cgroup_skb_sk_lookup_kern__bss,
}

extern "C" {
    static mut errno: c_int;

    fn start_server(
        family: c_int,
        type_: c_int,
        addr: *const c_void,
        port: c_int,
        timeout_ms: c_int,
    ) -> c_int;
    fn connect_fd_to_fd(client_fd: c_int, server_fd: c_int, timeout_ms: c_int) -> c_int;
    fn connect_to_fd(server_fd: c_int, timeout_ms: c_int) -> c_int;
    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn cgroup_skb_sk_lookup_kern__open_and_load() -> *mut cgroup_skb_sk_lookup_kern;
    fn cgroup_skb_sk_lookup_kern__destroy(obj: *mut cgroup_skb_sk_lookup_kern);
    fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);

    fn getsockname(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn CHECK(condition: bool, tag: *const c_char, format: *const c_char, ...) -> bool;
    fn CHECK_FAIL(condition: bool) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
}

unsafe fn run_lookup_test(g_serv_port: *mut __u16, out_sk: c_int) {
    let mut serv_sk: c_int = -1;
    let mut in_sk: c_int = -1;
    let mut serv_in_sk: c_int = -1;
    let mut err: c_int;
    let mut addr: sockaddr_in6 = mem::zeroed();
    let mut addr_len: socklen_t = mem::size_of::<sockaddr_in6>() as socklen_t;
    let _duration: __u32 = 0;

    serv_sk = start_server(AF_INET6, SOCK_STREAM, ptr::null(), 0, 0);
    if CHECK(
        serv_sk < 0,
        b"start_server\0".as_ptr() as *const c_char,
        b"failed to start server\n\0".as_ptr() as *const c_char,
    ) {
        return;
    }

    err = getsockname(
        serv_sk,
        &mut addr as *mut sockaddr_in6 as *mut sockaddr,
        &mut addr_len,
    );
    if CHECK(
        err != 0,
        b"getsockname\0".as_ptr() as *const c_char,
        b"errno %d\n\0".as_ptr() as *const c_char,
        errno,
    ) {
        close(serv_in_sk);
        close(in_sk);
        close(serv_sk);
        return;
    }

    *g_serv_port = addr.sin6_port;

    /* Client outside of test cgroup should fail to connect by timeout. */
    err = connect_fd_to_fd(out_sk, serv_sk, 1000);
    if CHECK(
        err == 0 || errno != EINPROGRESS,
        b"connect_fd_to_fd\0".as_ptr() as *const c_char,
        b"unexpected result err %d errno %d\n\0".as_ptr() as *const c_char,
        err,
        errno,
    ) {
        close(serv_in_sk);
        close(in_sk);
        close(serv_sk);
        return;
    }

    /* Client inside test cgroup should connect just fine. */
    in_sk = connect_to_fd(serv_sk, 0);
    if CHECK(
        in_sk < 0,
        b"connect_to_fd\0".as_ptr() as *const c_char,
        b"errno %d\n\0".as_ptr() as *const c_char,
        errno,
    ) {
        close(serv_in_sk);
        close(in_sk);
        close(serv_sk);
        return;
    }

    serv_in_sk = accept(serv_sk, ptr::null_mut(), ptr::null_mut());
    if CHECK(
        serv_in_sk < 0,
        b"accept\0".as_ptr() as *const c_char,
        b"errno %d\n\0".as_ptr() as *const c_char,
        errno,
    ) {
        close(serv_in_sk);
        close(in_sk);
        close(serv_sk);
        return;
    }

    close(serv_in_sk);
    close(in_sk);
    close(serv_sk);
}

unsafe fn run_cgroup_bpf_test(cg_path: *const c_char, out_sk: c_int) {
    let mut skel: *mut cgroup_skb_sk_lookup_kern;
    let link: *mut bpf_link;
    let _duration: __u32 = 0;
    let mut cgfd: c_int = -1;

    skel = cgroup_skb_sk_lookup_kern__open_and_load();
    if CHECK(
        skel.is_null(),
        b"skel_open_load\0".as_ptr() as *const c_char,
        b"open_load failed\n\0".as_ptr() as *const c_char,
    ) {
        return;
    }

    cgfd = test__join_cgroup(cg_path);
    if CHECK(
        cgfd < 0,
        b"cgroup_join\0".as_ptr() as *const c_char,
        b"cgroup setup failed\n\0".as_ptr() as *const c_char,
    ) {
        close(cgfd);
        cgroup_skb_sk_lookup_kern__destroy(skel);
        return;
    }

    link = bpf_program__attach_cgroup((*skel).progs.ingress_lookup, cgfd);
    if !ASSERT_OK_PTR(link as *const c_void, b"cgroup_attach\0".as_ptr() as *const c_char) {
        close(cgfd);
        cgroup_skb_sk_lookup_kern__destroy(skel);
        return;
    }

    run_lookup_test(&mut (*(*skel).bss).g_serv_port, out_sk);

    bpf_link__destroy(link);

    close(cgfd);
    cgroup_skb_sk_lookup_kern__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_cgroup_skb_sk_lookup() {
    let cg_path: *const c_char = b"/foo\0".as_ptr() as *const c_char;
    let out_sk: c_int;

    /* Create a socket before joining testing cgroup so that its cgroup id
     * differs from that of testing cgroup. Moving selftests process to
     * testing cgroup won't change cgroup id of an already created socket.
     */
    out_sk = socket(AF_INET6, SOCK_STREAM, 0);
    if CHECK_FAIL(out_sk < 0) {
        return;
    }

    run_cgroup_bpf_test(cg_path, out_sk);

    close(out_sk);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
