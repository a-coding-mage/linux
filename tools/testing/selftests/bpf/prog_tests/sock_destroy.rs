// SPDX-License-Identifier: GPL-2.0
// Translated from C implementation source:
// testing/selftests/bpf/prog_tests/sock_destroy.c
//
// Dependencies originally provided by:
// <test_progs.h>, <bpf/bpf_endian.h>,
// "sock_destroy_prog.skel.h", "sock_destroy_prog_fail.skel.h",
// and "network_helpers.h".

use core::ffi::{c_char, c_int, c_uint, c_void};

const TEST_NS: &[u8] = b"sock_destroy_netns\0";

type __be16 = u16;

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nstoken {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sock_destroy_prog {
    pub progs: sock_destroy_prog_progs,
    pub links: sock_destroy_prog_links,
    pub bss: *mut sock_destroy_prog_bss,
}

#[repr(C)]
pub struct sock_destroy_prog_progs {
    pub iter_tcp6_client: *mut bpf_program,
    pub iter_tcp6_server: *mut bpf_program,
    pub iter_udp6_client: *mut bpf_program,
    pub iter_udp6_server: *mut bpf_program,
    pub sock_connect: *mut bpf_program,
}

#[repr(C)]
pub struct sock_destroy_prog_links {
    pub sock_connect: *mut bpf_link,
}

#[repr(C)]
pub struct sock_destroy_prog_bss {
    pub serv_port: __be16,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn bpf_program__attach_iter(prog: *mut bpf_program, opts: *const c_void) -> *mut bpf_link;
    fn bpf_iter_create(link_fd: c_int) -> c_int;
    fn bpf_link__fd(link: *mut bpf_link) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;

    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn accept(fd: c_int, addr: *mut c_void, len: *mut c_void) -> c_int;
    fn send(fd: c_int, buf: *const c_void, len: usize, flags: c_int) -> isize;

    fn start_server(family: c_int, type_: c_int, addr: *const c_char, port: c_int, timeout_ms: c_int) -> c_int;
    fn connect_to_fd(fd: c_int, timeout_ms: c_int) -> c_int;
    fn get_socket_local_port(fd: c_int) -> c_int;
    fn start_reuseport_server(
        family: c_int,
        type_: c_int,
        addr: *const c_char,
        port: c_int,
        timeout_ms: c_int,
        num_listens: c_uint,
    ) -> *mut c_int;
    fn free_fds(fds: *mut c_int, num_fds: c_uint);

    fn sock_destroy_prog__open_and_load() -> *mut sock_destroy_prog;
    fn sock_destroy_prog__destroy(skel: *mut sock_destroy_prog);

    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(token: *mut nstoken);
    fn test__start_subtest(name: *const c_char) -> bool;
    fn run_tests_sock_destroy_prog_fail();

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: isize, expected: isize, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: isize, expected: isize, name: *const c_char) -> bool;
    fn ASSERT_LT(actual: isize, expected: isize, name: *const c_char) -> bool;
    fn SYS(label: *const c_char, fmt: *const c_char, ...);
    fn SYS_NOFAIL(cmd: *const c_char);
}

const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const ECONNABORTED: c_int = 103;
const ECONNRESET: c_int = 104;

unsafe fn start_iter_sockets(prog: *mut bpf_program) {
    let mut link: *mut bpf_link;
    let mut buf = [0u8; 50];
    let iter_fd: c_int;
    let mut len: isize;

    link = bpf_program__attach_iter(prog, core::ptr::null());
    if !ASSERT_OK_PTR(link as *const c_void, c"attach_iter".as_ptr()) {
        return;
    }

    iter_fd = bpf_iter_create(bpf_link__fd(link));
    if !ASSERT_GE(iter_fd as isize, 0, c"create_iter".as_ptr()) {
        goto_free_link(link);
        return;
    }

    loop {
        len = read(iter_fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf));
        if len <= 0 {
            break;
        }
    }
    ASSERT_GE(len, 0, c"read".as_ptr());

    close(iter_fd);

    goto_free_link(link);
}

unsafe fn goto_free_link(link: *mut bpf_link) {
    bpf_link__destroy(link);
}

unsafe fn test_tcp_client(skel: *mut sock_destroy_prog) {
    let mut serv: c_int = -1;
    let mut clien: c_int = -1;
    let mut accept_serv: c_int = -1;
    let mut n: isize;

    serv = start_server(AF_INET6, SOCK_STREAM, core::ptr::null(), 0, 0);
    if !ASSERT_GE(serv as isize, 0, c"start_server".as_ptr()) {
        goto_cleanup_tcp_client(serv, clien, accept_serv);
        return;
    }

    clien = connect_to_fd(serv, 0);
    if !ASSERT_GE(clien as isize, 0, c"connect_to_fd".as_ptr()) {
        goto_cleanup_tcp_client(serv, clien, accept_serv);
        return;
    }

    accept_serv = accept(serv, core::ptr::null_mut(), core::ptr::null_mut());
    if !ASSERT_GE(accept_serv as isize, 0, c"serv accept".as_ptr()) {
        goto_cleanup_tcp_client(serv, clien, accept_serv);
        return;
    }

    n = send(clien, c"t".as_ptr() as *const c_void, 1, 0);
    if !ASSERT_EQ(n, 1, c"client send".as_ptr()) {
        goto_cleanup_tcp_client(serv, clien, accept_serv);
        return;
    }

    /* Run iterator program that destroys connected client sockets. */
    start_iter_sockets((*skel).progs.iter_tcp6_client);

    n = send(clien, c"t".as_ptr() as *const c_void, 1, 0);
    if !ASSERT_LT(n, 0, c"client_send on destroyed socket".as_ptr()) {
        goto_cleanup_tcp_client(serv, clien, accept_serv);
        return;
    }
    ASSERT_EQ(errno as isize, ECONNABORTED as isize, c"error code on destroyed socket".as_ptr());

    goto_cleanup_tcp_client(serv, clien, accept_serv);
}

unsafe fn goto_cleanup_tcp_client(serv: c_int, clien: c_int, accept_serv: c_int) {
    if clien != -1 {
        close(clien);
    }
    if accept_serv != -1 {
        close(accept_serv);
    }
    if serv != -1 {
        close(serv);
    }
}

unsafe fn test_tcp_server(skel: *mut sock_destroy_prog) {
    let mut serv: c_int = -1;
    let mut clien: c_int = -1;
    let mut accept_serv: c_int = -1;
    let mut n: isize;
    let serv_port: c_int;

    serv = start_server(AF_INET6, SOCK_STREAM, core::ptr::null(), 0, 0);
    if !ASSERT_GE(serv as isize, 0, c"start_server".as_ptr()) {
        goto_cleanup_tcp_server(serv, clien, accept_serv);
        return;
    }
    serv_port = get_socket_local_port(serv);
    if !ASSERT_GE(serv_port as isize, 0, c"get_sock_local_port".as_ptr()) {
        goto_cleanup_tcp_server(serv, clien, accept_serv);
        return;
    }
    (*(*skel).bss).serv_port = serv_port as __be16;

    clien = connect_to_fd(serv, 0);
    if !ASSERT_GE(clien as isize, 0, c"connect_to_fd".as_ptr()) {
        goto_cleanup_tcp_server(serv, clien, accept_serv);
        return;
    }

    accept_serv = accept(serv, core::ptr::null_mut(), core::ptr::null_mut());
    if !ASSERT_GE(accept_serv as isize, 0, c"serv accept".as_ptr()) {
        goto_cleanup_tcp_server(serv, clien, accept_serv);
        return;
    }

    n = send(clien, c"t".as_ptr() as *const c_void, 1, 0);
    if !ASSERT_EQ(n, 1, c"client send".as_ptr()) {
        goto_cleanup_tcp_server(serv, clien, accept_serv);
        return;
    }

    /* Run iterator program that destroys server sockets. */
    start_iter_sockets((*skel).progs.iter_tcp6_server);

    n = send(clien, c"t".as_ptr() as *const c_void, 1, 0);
    if !ASSERT_LT(n, 0, c"client_send on destroyed socket".as_ptr()) {
        goto_cleanup_tcp_server(serv, clien, accept_serv);
        return;
    }
    ASSERT_EQ(errno as isize, ECONNRESET as isize, c"error code on destroyed socket".as_ptr());

    goto_cleanup_tcp_server(serv, clien, accept_serv);
}

unsafe fn goto_cleanup_tcp_server(serv: c_int, clien: c_int, accept_serv: c_int) {
    if clien != -1 {
        close(clien);
    }
    if accept_serv != -1 {
        close(accept_serv);
    }
    if serv != -1 {
        close(serv);
    }
}

unsafe fn test_udp_client(skel: *mut sock_destroy_prog) {
    let mut serv: c_int = -1;
    let mut clien: c_int = -1;
    let mut n: isize = 0;

    serv = start_server(AF_INET6, SOCK_DGRAM, core::ptr::null(), 0, 0);
    if !ASSERT_GE(serv as isize, 0, c"start_server".as_ptr()) {
        goto_cleanup_udp_client(serv, clien);
        return;
    }

    clien = connect_to_fd(serv, 0);
    if !ASSERT_GE(clien as isize, 0, c"connect_to_fd".as_ptr()) {
        goto_cleanup_udp_client(serv, clien);
        return;
    }

    n = send(clien, c"t".as_ptr() as *const c_void, 1, 0);
    if !ASSERT_EQ(n, 1, c"client send".as_ptr()) {
        goto_cleanup_udp_client(serv, clien);
        return;
    }

    /* Run iterator program that destroys sockets. */
    start_iter_sockets((*skel).progs.iter_udp6_client);

    n = send(clien, c"t".as_ptr() as *const c_void, 1, 0);
    if !ASSERT_LT(n, 0, c"client_send on destroyed socket".as_ptr()) {
        goto_cleanup_udp_client(serv, clien);
        return;
    }
    /* UDP sockets have an overriding error code after they are disconnected,
     * so we don't check for ECONNABORTED error code.
     */

    goto_cleanup_udp_client(serv, clien);
}

unsafe fn goto_cleanup_udp_client(serv: c_int, clien: c_int) {
    if clien != -1 {
        close(clien);
    }
    if serv != -1 {
        close(serv);
    }
}

unsafe fn test_udp_server(skel: *mut sock_destroy_prog) {
    let mut listen_fds: *mut c_int = core::ptr::null_mut();
    let mut n: isize;
    let mut i: c_uint;
    let serv_port: c_int;
    let num_listens: c_uint = 5;
    let mut buf = [0u8; 1];

    /* Start reuseport servers. */
    listen_fds = start_reuseport_server(
        AF_INET6,
        SOCK_DGRAM,
        c"::1".as_ptr(),
        0,
        0,
        num_listens,
    );
    if !ASSERT_OK_PTR(listen_fds as *const c_void, c"start_reuseport_server".as_ptr()) {
        goto_cleanup_udp_server(listen_fds, num_listens);
        return;
    }
    serv_port = get_socket_local_port(*listen_fds.add(0));
    if !ASSERT_GE(serv_port as isize, 0, c"get_sock_local_port".as_ptr()) {
        goto_cleanup_udp_server(listen_fds, num_listens);
        return;
    }
    (*(*skel).bss).serv_port = serv_port as __be16;

    /* Run iterator program that destroys server sockets. */
    start_iter_sockets((*skel).progs.iter_udp6_server);

    i = 0;
    while i < num_listens {
        n = read(*listen_fds.add(i as usize), buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf));
        if !ASSERT_EQ(n, -1, c"read".as_ptr())
            || !ASSERT_EQ(errno as isize, ECONNABORTED as isize, c"error code on destroyed socket".as_ptr())
        {
            break;
        }
        i += 1;
    }
    ASSERT_EQ(i as isize, num_listens as isize, c"server socket".as_ptr());

    goto_cleanup_udp_server(listen_fds, num_listens);
}

unsafe fn goto_cleanup_udp_server(listen_fds: *mut c_int, num_listens: c_uint) {
    free_fds(listen_fds, num_listens);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_sock_destroy() {
    let mut skel: *mut sock_destroy_prog;
    let mut nstoken: *mut nstoken = core::ptr::null_mut();
    let mut cgroup_fd: c_int;

    skel = sock_destroy_prog__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
        return;
    }

    cgroup_fd = test__join_cgroup(c"/sock_destroy".as_ptr());
    if !ASSERT_GE(cgroup_fd as isize, 0, c"join_cgroup".as_ptr()) {
        goto_cleanup_sock_destroy(skel, nstoken, cgroup_fd);
        return;
    }

    (*skel).links.sock_connect = bpf_program__attach_cgroup((*skel).progs.sock_connect, cgroup_fd);
    if !ASSERT_OK_PTR((*skel).links.sock_connect as *const c_void, c"prog_attach".as_ptr()) {
        goto_cleanup_sock_destroy(skel, nstoken, cgroup_fd);
        return;
    }

    SYS(c"cleanup".as_ptr(), c"ip netns add %s".as_ptr(), TEST_NS.as_ptr());
    SYS(c"cleanup".as_ptr(), c"ip -net %s link set dev lo up".as_ptr(), TEST_NS.as_ptr());

    nstoken = open_netns(TEST_NS.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(nstoken as *const c_void, c"open_netns".as_ptr()) {
        goto_cleanup_sock_destroy(skel, nstoken, cgroup_fd);
        return;
    }

    if test__start_subtest(c"tcp_client".as_ptr()) {
        test_tcp_client(skel);
    }
    if test__start_subtest(c"tcp_server".as_ptr()) {
        test_tcp_server(skel);
    }
    if test__start_subtest(c"udp_client".as_ptr()) {
        test_udp_client(skel);
    }
    if test__start_subtest(c"udp_server".as_ptr()) {
        test_udp_server(skel);
    }

    run_tests_sock_destroy_prog_fail();

    goto_cleanup_sock_destroy(skel, nstoken, cgroup_fd);
}

unsafe fn goto_cleanup_sock_destroy(skel: *mut sock_destroy_prog, nstoken: *mut nstoken, cgroup_fd: c_int) {
    if !nstoken.is_null() {
        close_netns(nstoken);
    }
    SYS_NOFAIL(c"ip netns del sock_destroy_netns".as_ptr());
    if cgroup_fd >= 0 {
        close(cgroup_fd);
    }
    sock_destroy_prog__destroy(skel);
}
