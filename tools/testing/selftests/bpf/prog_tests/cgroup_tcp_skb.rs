// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Facebook */

// Translated from C implementation source. Original includes:
// <test_progs.h>, <linux/in6.h>, <sys/socket.h>, <sched.h>, <unistd.h>,
// "cgroup_helpers.h", "testing_helpers.h", "cgroup_tcp_skb.skel.h",
// "cgroup_tcp_skb.h", "network_helpers.h"

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

const CGROUP_TCP_SKB_PATH: *const c_char = b"/test_cgroup_tcp_skb\0".as_ptr() as *const c_char;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SHUT_WR: c_int = 1;

extern "C" {
    static CLOSED: c_int;
    static TIME_WAIT: c_int;
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cgroup_tcp_skb_bss {
    pub g_sock_state: c_int,
    pub g_unexpected: c_int,
    pub g_sock_port: c_int,
    pub g_packet_count: u32,
}

#[repr(C)]
pub struct cgroup_tcp_skb_progs {
    pub server_egress: *mut bpf_program,
    pub server_ingress: *mut bpf_program,
    pub server_egress_srv: *mut bpf_program,
    pub server_ingress_srv: *mut bpf_program,
    pub client_egress_srv: *mut bpf_program,
    pub client_ingress_srv: *mut bpf_program,
    pub client_egress: *mut bpf_program,
    pub client_ingress: *mut bpf_program,
}

#[repr(C)]
pub struct cgroup_tcp_skb {
    pub bss: *mut cgroup_tcp_skb_bss,
    pub progs: cgroup_tcp_skb_progs,
}

extern "C" {
    fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn perror(s: *const c_char);
    fn join_root_cgroup() -> c_int;
    fn join_cgroup(path: *const c_char) -> c_int;
    fn start_server(
        family: c_int,
        type_: c_int,
        addr: *const c_void,
        port: c_int,
        timeout_ms: c_int,
    ) -> c_int;
    fn get_socket_local_port(fd: c_int) -> c_int;
    fn ntohs(netshort: c_int) -> c_int;
    fn connect_fd_to_fd(client_fd: c_int, listen_fd: c_int, timeout_ms: c_int) -> c_int;
    fn accept(sockfd: c_int, addr: *mut c_void, addrlen: *mut c_void) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn usleep(usec: u32) -> c_int;
    fn shutdown(sockfd: c_int, how: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn cgroup_tcp_skb__open_and_load() -> *mut cgroup_tcp_skb;
    fn setup_cgroup_environment() -> c_int;
    fn create_and_get_cgroup(path: *const c_char) -> c_int;
    fn cleanup_cgroup_environment();
    fn cgroup_tcp_skb__destroy(obj: *mut cgroup_tcp_skb);

    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
}

unsafe fn install_filters(
    cgroup_fd: c_int,
    egress_link: *mut *mut bpf_link,
    ingress_link: *mut *mut bpf_link,
    egress_prog: *mut bpf_program,
    ingress_prog: *mut bpf_program,
    skel: *mut cgroup_tcp_skb,
) -> c_int {
    /* Prepare filters */
    (*(*skel).bss).g_sock_state = 0;
    (*(*skel).bss).g_unexpected = 0;
    *egress_link = bpf_program__attach_cgroup(egress_prog, cgroup_fd);
    if !ASSERT_OK_PTR(egress_link as *const c_void, b"egress_link\0".as_ptr() as *const c_char) {
        return -1;
    }
    *ingress_link = bpf_program__attach_cgroup(ingress_prog, cgroup_fd);
    if !ASSERT_OK_PTR(ingress_link as *const c_void, b"ingress_link\0".as_ptr() as *const c_char) {
        return -1;
    }

    0
}

unsafe fn uninstall_filters(egress_link: *mut *mut bpf_link, ingress_link: *mut *mut bpf_link) {
    bpf_link__destroy(*egress_link);
    *egress_link = ptr::null_mut();
    bpf_link__destroy(*ingress_link);
    *ingress_link = ptr::null_mut();
}

unsafe fn create_client_sock_v6() -> c_int {
    let fd: c_int;

    fd = socket(AF_INET6, SOCK_STREAM, 0);
    if fd < 0 {
        perror(b"socket\0".as_ptr() as *const c_char);
        return -1;
    }

    fd
}

/* Connect to the server in a cgroup from the outside of the cgroup. */
unsafe fn talk_to_cgroup(
    client_fd: *mut c_int,
    listen_fd: *mut c_int,
    service_fd: *mut c_int,
    skel: *mut cgroup_tcp_skb,
) -> c_int {
    let mut err: c_int;
    let mut cp: c_int;
    let mut buf = [0 as c_char; 5];
    let port: c_int;

    /* Create client & server socket */
    err = join_root_cgroup();
    if !ASSERT_OK(err, b"join_root_cgroup\0".as_ptr() as *const c_char) {
        return -1;
    }
    *client_fd = create_client_sock_v6();
    if !ASSERT_GE(*client_fd, 0, b"client_fd\0".as_ptr() as *const c_char) {
        return -1;
    }
    err = join_cgroup(CGROUP_TCP_SKB_PATH);
    if !ASSERT_OK(err, b"join_cgroup\0".as_ptr() as *const c_char) {
        return -1;
    }
    *listen_fd = start_server(AF_INET6, SOCK_STREAM, ptr::null(), 0, 0);
    if !ASSERT_GE(*listen_fd, 0, b"listen_fd\0".as_ptr() as *const c_char) {
        return -1;
    }
    port = get_socket_local_port(*listen_fd);
    if !ASSERT_GE(port, 0, b"get_socket_local_port\0".as_ptr() as *const c_char) {
        return -1;
    }
    (*(*skel).bss).g_sock_port = ntohs(port);

    /* Connect client to server */
    err = connect_fd_to_fd(*client_fd, *listen_fd, 0);
    if !ASSERT_OK(err, b"connect_fd_to_fd\0".as_ptr() as *const c_char) {
        return -1;
    }
    *service_fd = accept(*listen_fd, ptr::null_mut(), ptr::null_mut());
    if !ASSERT_GE(*service_fd, 0, b"service_fd\0".as_ptr() as *const c_char) {
        return -1;
    }
    err = join_root_cgroup();
    if !ASSERT_OK(err, b"join_root_cgroup\0".as_ptr() as *const c_char) {
        return -1;
    }
    cp = write(*client_fd, b"hello".as_ptr() as *const c_void, 5) as c_int;
    if !ASSERT_EQ(cp, 5, b"write\0".as_ptr() as *const c_char) {
        return -1;
    }
    cp = read(*service_fd, buf.as_mut_ptr() as *mut c_void, 5) as c_int;
    if !ASSERT_EQ(cp, 5, b"read\0".as_ptr() as *const c_char) {
        return -1;
    }

    0
}

/* Connect to the server out of a cgroup from inside the cgroup. */
unsafe fn talk_to_outside(
    client_fd: *mut c_int,
    listen_fd: *mut c_int,
    service_fd: *mut c_int,
    skel: *mut cgroup_tcp_skb,
) -> c_int {
    let mut err: c_int;
    let mut cp: c_int;
    let mut buf = [0 as c_char; 5];
    let port: c_int;

    /* Create client & server socket */
    err = join_root_cgroup();
    if !ASSERT_OK(err, b"join_root_cgroup\0".as_ptr() as *const c_char) {
        return -1;
    }
    *listen_fd = start_server(AF_INET6, SOCK_STREAM, ptr::null(), 0, 0);
    if !ASSERT_GE(*listen_fd, 0, b"listen_fd\0".as_ptr() as *const c_char) {
        return -1;
    }
    err = join_cgroup(CGROUP_TCP_SKB_PATH);
    if !ASSERT_OK(err, b"join_cgroup\0".as_ptr() as *const c_char) {
        return -1;
    }
    *client_fd = create_client_sock_v6();
    if !ASSERT_GE(*client_fd, 0, b"client_fd\0".as_ptr() as *const c_char) {
        return -1;
    }
    err = join_root_cgroup();
    if !ASSERT_OK(err, b"join_root_cgroup\0".as_ptr() as *const c_char) {
        return -1;
    }
    port = get_socket_local_port(*listen_fd);
    if !ASSERT_GE(port, 0, b"get_socket_local_port\0".as_ptr() as *const c_char) {
        return -1;
    }
    (*(*skel).bss).g_sock_port = ntohs(port);

    /* Connect client to server */
    err = connect_fd_to_fd(*client_fd, *listen_fd, 0);
    if !ASSERT_OK(err, b"connect_fd_to_fd\0".as_ptr() as *const c_char) {
        return -1;
    }
    *service_fd = accept(*listen_fd, ptr::null_mut(), ptr::null_mut());
    if !ASSERT_GE(*service_fd, 0, b"service_fd\0".as_ptr() as *const c_char) {
        return -1;
    }
    cp = write(*client_fd, b"hello".as_ptr() as *const c_void, 5) as c_int;
    if !ASSERT_EQ(cp, 5, b"write\0".as_ptr() as *const c_char) {
        return -1;
    }
    cp = read(*service_fd, buf.as_mut_ptr() as *mut c_void, 5) as c_int;
    if !ASSERT_EQ(cp, 5, b"read\0".as_ptr() as *const c_char) {
        return -1;
    }

    0
}

unsafe fn close_connection(
    closing_fd: *mut c_int,
    peer_fd: *mut c_int,
    listen_fd: *mut c_int,
    skel: *mut cgroup_tcp_skb,
) -> c_int {
    let mut saved_packet_count: u32 = 0;
    let mut err: c_int;
    let mut i: c_int;

    /* Wait for ACKs to be sent */
    saved_packet_count = (*(*skel).bss).g_packet_count;
    usleep(100000); /* 0.1s */
    i = 0;
    while (*(*skel).bss).g_packet_count != saved_packet_count && i < 10 {
        saved_packet_count = (*(*skel).bss).g_packet_count;
        usleep(100000); /* 0.1s */
        i += 1;
    }
    if !ASSERT_EQ(
        (*(*skel).bss).g_packet_count as c_int,
        saved_packet_count as c_int,
        b"packet_count\0".as_ptr() as *const c_char,
    ) {
        return -1;
    }

    (*(*skel).bss).g_packet_count = 0;
    saved_packet_count = 0;

    /* Half shutdown to make sure the closing socket having a chance to
     * receive a FIN from the peer.
     */
    err = shutdown(*closing_fd, SHUT_WR);
    if !ASSERT_OK(err, b"shutdown closing_fd\0".as_ptr() as *const c_char) {
        return -1;
    }

    /* Wait for FIN and the ACK of the FIN to be observed */
    i = 0;
    while (*(*skel).bss).g_packet_count < saved_packet_count + 2 && i < 10 {
        usleep(100000); /* 0.1s */
        i += 1;
    }
    if !ASSERT_GE(
        (*(*skel).bss).g_packet_count as c_int,
        (saved_packet_count + 2) as c_int,
        b"packet_count\0".as_ptr() as *const c_char,
    ) {
        return -1;
    }

    saved_packet_count = (*(*skel).bss).g_packet_count;

    /* Fully shutdown the connection */
    err = close(*peer_fd);
    if !ASSERT_OK(err, b"close peer_fd\0".as_ptr() as *const c_char) {
        return -1;
    }
    *peer_fd = -1;

    /* Wait for FIN and the ACK of the FIN to be observed */
    i = 0;
    while (*(*skel).bss).g_packet_count < saved_packet_count + 2 && i < 10 {
        usleep(100000); /* 0.1s */
        i += 1;
    }
    if !ASSERT_GE(
        (*(*skel).bss).g_packet_count as c_int,
        (saved_packet_count + 2) as c_int,
        b"packet_count\0".as_ptr() as *const c_char,
    ) {
        return -1;
    }

    err = close(*closing_fd);
    if !ASSERT_OK(err, b"close closing_fd\0".as_ptr() as *const c_char) {
        return -1;
    }
    *closing_fd = -1;

    close(*listen_fd);
    *listen_fd = -1;

    0
}

/* This test case includes four scenarios:
 * 1. Connect to the server from outside the cgroup and close the connection
 *    from outside the cgroup.
 * 2. Connect to the server from outside the cgroup and close the connection
 *    from inside the cgroup.
 * 3. Connect to the server from inside the cgroup and close the connection
 *    from outside the cgroup.
 * 4. Connect to the server from inside the cgroup and close the connection
 *    from inside the cgroup.
 *
 * The test case is to verify that cgroup_skb/{egress,ingress} filters
 * receive expected packets including SYN, SYN/ACK, ACK, FIN, and FIN/ACK.
 */
#[no_mangle]
pub unsafe extern "C" fn test_cgroup_tcp_skb() {
    let mut ingress_link: *mut bpf_link = ptr::null_mut();
    let mut egress_link: *mut bpf_link = ptr::null_mut();
    let mut client_fd: c_int = -1;
    let mut listen_fd: c_int = -1;
    let mut service_fd: c_int = -1;
    let mut cgroup_fd: c_int = -1;
    let mut err: c_int;

    let skel = cgroup_tcp_skb__open_and_load();
    if !ASSERT_OK(skel.is_null() as c_int, b"skel_open_load\0".as_ptr() as *const c_char) {
        return;
    }

    err = setup_cgroup_environment();
    if !ASSERT_OK(err, b"setup_cgroup_environment\0".as_ptr() as *const c_char) {
        goto_cleanup(
            client_fd,
            listen_fd,
            service_fd,
            cgroup_fd,
            egress_link,
            ingress_link,
            skel,
        );
        return;
    }

    cgroup_fd = create_and_get_cgroup(CGROUP_TCP_SKB_PATH);
    if !ASSERT_GE(cgroup_fd, 0, b"cgroup_fd\0".as_ptr() as *const c_char) {
        goto_cleanup(
            client_fd,
            listen_fd,
            service_fd,
            cgroup_fd,
            egress_link,
            ingress_link,
            skel,
        );
        return;
    }

    /* Scenario 1 */
    err = install_filters(
        cgroup_fd,
        &mut egress_link,
        &mut ingress_link,
        (*skel).progs.server_egress,
        (*skel).progs.server_ingress,
        skel,
    );
    if !ASSERT_OK(err, b"install_filters\0".as_ptr() as *const c_char) {
        goto_cleanup(
            client_fd,
            listen_fd,
            service_fd,
            cgroup_fd,
            egress_link,
            ingress_link,
            skel,
        );
        return;
    }

    err = talk_to_cgroup(&mut client_fd, &mut listen_fd, &mut service_fd, skel);
    if !ASSERT_OK(err, b"talk_to_cgroup\0".as_ptr() as *const c_char) {
        goto_cleanup(client_fd, listen_fd, service_fd, cgroup_fd, egress_link, ingress_link, skel);
        return;
    }

    err = close_connection(&mut client_fd, &mut service_fd, &mut listen_fd, skel);
    if !ASSERT_OK(err, b"close_connection\0".as_ptr() as *const c_char) {
        goto_cleanup(client_fd, listen_fd, service_fd, cgroup_fd, egress_link, ingress_link, skel);
        return;
    }

    ASSERT_EQ((*(*skel).bss).g_unexpected, 0, b"g_unexpected\0".as_ptr() as *const c_char);
    ASSERT_EQ((*(*skel).bss).g_sock_state, CLOSED, b"g_sock_state\0".as_ptr() as *const c_char);

    uninstall_filters(&mut egress_link, &mut ingress_link);

    /* Scenario 2 */
    err = install_filters(
        cgroup_fd,
        &mut egress_link,
        &mut ingress_link,
        (*skel).progs.server_egress_srv,
        (*skel).progs.server_ingress_srv,
        skel,
    );

    err = talk_to_cgroup(&mut client_fd, &mut listen_fd, &mut service_fd, skel);
    if !ASSERT_OK(err, b"talk_to_cgroup\0".as_ptr() as *const c_char) {
        goto_cleanup(client_fd, listen_fd, service_fd, cgroup_fd, egress_link, ingress_link, skel);
        return;
    }

    err = close_connection(&mut service_fd, &mut client_fd, &mut listen_fd, skel);
    if !ASSERT_OK(err, b"close_connection\0".as_ptr() as *const c_char) {
        goto_cleanup(client_fd, listen_fd, service_fd, cgroup_fd, egress_link, ingress_link, skel);
        return;
    }

    ASSERT_EQ((*(*skel).bss).g_unexpected, 0, b"g_unexpected\0".as_ptr() as *const c_char);
    ASSERT_EQ((*(*skel).bss).g_sock_state, TIME_WAIT, b"g_sock_state\0".as_ptr() as *const c_char);

    uninstall_filters(&mut egress_link, &mut ingress_link);

    /* Scenario 3 */
    err = install_filters(
        cgroup_fd,
        &mut egress_link,
        &mut ingress_link,
        (*skel).progs.client_egress_srv,
        (*skel).progs.client_ingress_srv,
        skel,
    );

    err = talk_to_outside(&mut client_fd, &mut listen_fd, &mut service_fd, skel);
    if !ASSERT_OK(err, b"talk_to_outside\0".as_ptr() as *const c_char) {
        goto_cleanup(client_fd, listen_fd, service_fd, cgroup_fd, egress_link, ingress_link, skel);
        return;
    }

    err = close_connection(&mut service_fd, &mut client_fd, &mut listen_fd, skel);
    if !ASSERT_OK(err, b"close_connection\0".as_ptr() as *const c_char) {
        goto_cleanup(client_fd, listen_fd, service_fd, cgroup_fd, egress_link, ingress_link, skel);
        return;
    }

    ASSERT_EQ((*(*skel).bss).g_unexpected, 0, b"g_unexpected\0".as_ptr() as *const c_char);
    ASSERT_EQ((*(*skel).bss).g_sock_state, CLOSED, b"g_sock_state\0".as_ptr() as *const c_char);

    uninstall_filters(&mut egress_link, &mut ingress_link);

    /* Scenario 4 */
    err = install_filters(
        cgroup_fd,
        &mut egress_link,
        &mut ingress_link,
        (*skel).progs.client_egress,
        (*skel).progs.client_ingress,
        skel,
    );

    err = talk_to_outside(&mut client_fd, &mut listen_fd, &mut service_fd, skel);
    if !ASSERT_OK(err, b"talk_to_outside\0".as_ptr() as *const c_char) {
        goto_cleanup(client_fd, listen_fd, service_fd, cgroup_fd, egress_link, ingress_link, skel);
        return;
    }

    err = close_connection(&mut client_fd, &mut service_fd, &mut listen_fd, skel);
    if !ASSERT_OK(err, b"close_connection\0".as_ptr() as *const c_char) {
        goto_cleanup(client_fd, listen_fd, service_fd, cgroup_fd, egress_link, ingress_link, skel);
        return;
    }

    ASSERT_EQ((*(*skel).bss).g_unexpected, 0, b"g_unexpected\0".as_ptr() as *const c_char);
    ASSERT_EQ((*(*skel).bss).g_sock_state, TIME_WAIT, b"g_sock_state\0".as_ptr() as *const c_char);

    uninstall_filters(&mut egress_link, &mut ingress_link);

    goto_cleanup(client_fd, listen_fd, service_fd, cgroup_fd, egress_link, ingress_link, skel);
}

unsafe fn goto_cleanup(
    client_fd: c_int,
    listen_fd: c_int,
    service_fd: c_int,
    cgroup_fd: c_int,
    egress_link: *mut bpf_link,
    ingress_link: *mut bpf_link,
    skel: *mut cgroup_tcp_skb,
) {
    close(client_fd);
    close(listen_fd);
    close(service_fd);
    close(cgroup_fd);
    bpf_link__destroy(egress_link);
    bpf_link__destroy(ingress_link);
    cleanup_cgroup_environment();
    cgroup_tcp_skb__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
