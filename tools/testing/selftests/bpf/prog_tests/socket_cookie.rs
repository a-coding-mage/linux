// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Google LLC.
// Copyright (c) 2018 Facebook

// C dependencies: <test_progs.h>, "socket_cookie_prog.skel.h",
// "network_helpers.h".

use core::ffi::{c_char, c_int, c_void};

type __u64 = u64;
type __u32 = u32;
type socklen_t = u32;

const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;

static mut duration: c_int = 0;

#[repr(C)]
struct socket_cookie {
    cookie_key: __u64,
    cookie_value: __u32,
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [u8; 14],
}

#[repr(C)]
struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
struct sockaddr_in6 {
    sin6_family: u16,
    sin6_port: u16,
    sin6_flowinfo: __u32,
    sin6_addr: in6_addr,
    sin6_scope_id: __u32,
}

#[repr(C)]
struct socket_cookie_prog {
    progs: socket_cookie_prog_progs,
    maps: socket_cookie_prog_maps,
    links: socket_cookie_prog_links,
}

#[repr(C)]
struct socket_cookie_prog_progs {
    set_cookie: *mut bpf_program,
    update_cookie_sockops: *mut bpf_program,
    update_cookie_tracing: *mut bpf_program,
}

#[repr(C)]
struct socket_cookie_prog_maps {
    socket_cookies: *mut bpf_map,
}

#[repr(C)]
struct socket_cookie_prog_links {
    set_cookie: *mut bpf_link,
    update_cookie_sockops: *mut bpf_link,
    update_cookie_tracing: *mut bpf_link,
}

enum bpf_program {}
enum bpf_map {}
enum bpf_link {}

unsafe extern "C" {
    static mut errno: c_int;

    fn socket_cookie_prog__open_and_load() -> *mut socket_cookie_prog;
    fn socket_cookie_prog__destroy(skel: *mut socket_cookie_prog);

    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;

    fn start_server(
        family: c_int,
        type_: c_int,
        addr: *const c_char,
        port: c_int,
        timeout_ms: c_int,
    ) -> c_int;
    fn connect_to_fd(server_fd: c_int, timeout_ms: c_int) -> c_int;
    fn getsockname(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn ntohs(netshort: u16) -> u16;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: __u32, expected: __u32, name: *const c_char) -> bool;
    fn CHECK(
        condition: bool,
        tag: *const c_char,
        format: *const c_char,
        ...
    ) -> bool;
}

pub unsafe fn test_socket_cookie() {
    let mut server_fd: c_int = 0;
    let mut client_fd: c_int = 0;
    let mut cgroup_fd: c_int = 0;
    let mut err: c_int = 0;
    let mut addr_len: socklen_t = core::mem::size_of::<sockaddr_in6>() as socklen_t;
    let mut skel: *mut socket_cookie_prog;
    let mut cookie_expected_value: __u32;
    let mut addr: sockaddr_in6 = core::mem::zeroed();
    let mut val: socket_cookie = core::mem::zeroed();

    skel = socket_cookie_prog__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, b"skel_open\0".as_ptr() as *const c_char) {
        return;
    }

    cgroup_fd = test__join_cgroup(b"/socket_cookie\0".as_ptr() as *const c_char);
    if CHECK(
        cgroup_fd < 0,
        b"join_cgroup\0".as_ptr() as *const c_char,
        b"cgroup creation failed\n\0".as_ptr() as *const c_char,
    ) {
        goto_out(skel);
        return;
    }

    (*skel).links.set_cookie =
        bpf_program__attach_cgroup((*skel).progs.set_cookie, cgroup_fd);
    if !ASSERT_OK_PTR(
        (*skel).links.set_cookie as *const c_void,
        b"prog_attach\0".as_ptr() as *const c_char,
    ) {
        goto_close_cgroup_fd(cgroup_fd, skel);
        return;
    }

    (*skel).links.update_cookie_sockops =
        bpf_program__attach_cgroup((*skel).progs.update_cookie_sockops, cgroup_fd);
    if !ASSERT_OK_PTR(
        (*skel).links.update_cookie_sockops as *const c_void,
        b"prog_attach\0".as_ptr() as *const c_char,
    ) {
        goto_close_cgroup_fd(cgroup_fd, skel);
        return;
    }

    (*skel).links.update_cookie_tracing =
        bpf_program__attach((*skel).progs.update_cookie_tracing);
    if !ASSERT_OK_PTR(
        (*skel).links.update_cookie_tracing as *const c_void,
        b"prog_attach\0".as_ptr() as *const c_char,
    ) {
        goto_close_cgroup_fd(cgroup_fd, skel);
        return;
    }

    server_fd = start_server(
        AF_INET6,
        SOCK_STREAM,
        b"::1\0".as_ptr() as *const c_char,
        0,
        0,
    );
    if CHECK(
        server_fd < 0,
        b"start_server\0".as_ptr() as *const c_char,
        b"errno %d\n\0".as_ptr() as *const c_char,
        errno,
    ) {
        goto_close_cgroup_fd(cgroup_fd, skel);
        return;
    }

    client_fd = connect_to_fd(server_fd, 0);
    if CHECK(
        client_fd < 0,
        b"connect_to_fd\0".as_ptr() as *const c_char,
        b"errno %d\n\0".as_ptr() as *const c_char,
        errno,
    ) {
        goto_close_server_fd(server_fd, cgroup_fd, skel);
        return;
    }

    err = bpf_map_lookup_elem(
        bpf_map__fd((*skel).maps.socket_cookies),
        &client_fd as *const c_int as *const c_void,
        &mut val as *mut socket_cookie as *mut c_void,
    );
    if !ASSERT_OK(err, b"map_lookup(socket_cookies)\0".as_ptr() as *const c_char) {
        goto_close_client_fd(client_fd, server_fd, cgroup_fd, skel);
        return;
    }

    err = getsockname(
        client_fd,
        &mut addr as *mut sockaddr_in6 as *mut sockaddr,
        &mut addr_len as *mut socklen_t,
    );
    if !ASSERT_OK(err, b"getsockname\0".as_ptr() as *const c_char) {
        goto_close_client_fd(client_fd, server_fd, cgroup_fd, skel);
        return;
    }

    cookie_expected_value = ((ntohs(addr.sin6_port) as __u32) << 8) | 0xFF;
    ASSERT_EQ(
        val.cookie_value,
        cookie_expected_value,
        b"cookie_value\0".as_ptr() as *const c_char,
    );

    goto_close_client_fd(client_fd, server_fd, cgroup_fd, skel);
}

unsafe fn goto_close_client_fd(
    client_fd: c_int,
    server_fd: c_int,
    cgroup_fd: c_int,
    skel: *mut socket_cookie_prog,
) {
    close(client_fd);
    goto_close_server_fd(server_fd, cgroup_fd, skel);
}

unsafe fn goto_close_server_fd(server_fd: c_int, cgroup_fd: c_int, skel: *mut socket_cookie_prog) {
    close(server_fd);
    goto_close_cgroup_fd(cgroup_fd, skel);
}

unsafe fn goto_close_cgroup_fd(cgroup_fd: c_int, skel: *mut socket_cookie_prog) {
    close(cgroup_fd);
    goto_out(skel);
}

unsafe fn goto_out(skel: *mut socket_cookie_prog) {
    socket_cookie_prog__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
