// SPDX-License-Identifier: GPL-2.0
//
// Dependencies from the original C includes:
// <test_progs.h>, <net/if.h>, <linux/netfilter.h>, <network_helpers.h>,
// "ip_check_defrag.skel.h", and "ip_check_defrag_frags.h".

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

/*
 * This selftest spins up a client and an echo server, each in their own
 * network namespace. The client will send a fragmented message to the server.
 * The prog attached to the server will shoot down any fragments. Thus, if
 * the server is able to correctly echo back the message to the client, we will
 * have verified that netfilter is reassembling packets for us.
 *
 * Topology:
 * =========
 *           NS0         |         NS1
 *                       |
 *         client        |       server
 *       ----------      |     ----------
 *       |  veth0  | --------- |  veth1  |
 *       ----------    peer    ----------
 *                       |
 *                       |       with bpf
 */

const NS0: &[u8] = b"defrag_ns0\0";
const NS1: &[u8] = b"defrag_ns1\0";
const VETH0: &str = "veth0";
const VETH1: &str = "veth1";
const VETH0_ADDR: &[u8] = b"172.16.1.100\0";
const VETH0_ADDR6: &[u8] = b"fc00::100\0";
/* The following constants must stay in sync with `generate_udp_fragments.py` */
const VETH1_ADDR: &[u8] = b"172.16.1.200\0";
const VETH1_ADDR6: &[u8] = b"fc00::200\0";
const CLIENT_PORT: c_int = 48878;
const SERVER_PORT: c_int = 48879;
const MAGIC_MESSAGE: &[u8] = b"THIS IS THE ORIGINAL MESSAGE, PLEASE REASSEMBLE ME\0";

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_DGRAM: c_int = 2;
const SOCK_RAW: c_int = 3;
const IPPROTO_RAW: c_int = 255;
const NFPROTO_IPV4: c_uint = 2;
const NFPROTO_IPV6: c_uint = 10;
const BPF_F_NETFILTER_IP_DEFRAG: c_uint = 1;

type SocklenT = c_uint;

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct in_addr {
    s_addr: u32,
}

#[repr(C)]
struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
struct sockaddr_in {
    sin_family: u16,
    sin_port: u16,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

#[repr(C)]
struct sockaddr_in6 {
    sin6_family: u16,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: in6_addr,
    sin6_scope_id: u32,
}

#[repr(C)]
struct sockaddr_storage {
    ss_family: u16,
    __ss_padding: [u8; 118],
    __ss_align: u64,
}

#[repr(C)]
struct network_helper_opts {
    timeout_ms: c_int,
    proto: c_int,
}

#[repr(C)]
struct bpf_netfilter_opts {
    sz: usize,
    pf: c_uint,
    priority: c_int,
    flags: c_uint,
}

#[repr(C)]
struct ip_check_defrag {
    progs: ip_check_defrag_progs,
    links: ip_check_defrag_links,
    bss: *mut ip_check_defrag_bss,
}

#[repr(C)]
struct ip_check_defrag_progs {
    defrag: *mut bpf_program,
}

#[repr(C)]
struct ip_check_defrag_links {
    defrag: *mut bpf_link,
}

#[repr(C)]
struct ip_check_defrag_bss {
    shootdowns: c_int,
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct nstoken {
    _private: [u8; 0],
}

unsafe extern "C" {
    static frag_0: [u8; 0];
    static frag_1: [u8; 0];
    static frag_2: [u8; 0];
    static frag6_0: [u8; 0];
    static frag6_1: [u8; 0];
    static frag6_2: [u8; 0];

    fn system(command: *const c_char) -> c_int;
    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(token: *mut nstoken);
    fn bpf_program__attach_netfilter(
        prog: *mut bpf_program,
        opts: *const bpf_netfilter_opts,
    ) -> *mut bpf_link;
    fn make_sockaddr(
        family: c_int,
        addr: *const c_char,
        port: c_int,
        saddr: *mut sockaddr_storage,
        saddr_len: *mut SocklenT,
    ) -> c_int;
    fn sendto(
        sockfd: c_int,
        buf: *const c_void,
        len: usize,
        flags: c_int,
        dest_addr: *const sockaddr,
        addrlen: SocklenT,
    ) -> isize;
    fn recvfrom(
        sockfd: c_int,
        buf: *mut c_void,
        len: usize,
        flags: c_int,
        src_addr: *mut sockaddr,
        addrlen: *mut SocklenT,
    ) -> isize;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: SocklenT) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn start_server(
        family: c_int,
        type_: c_int,
        addr: *const c_char,
        port: c_int,
        timeout_ms: c_int,
    ) -> c_int;
    fn client_socket(
        family: c_int,
        type_: c_int,
        opts: *const network_helper_opts,
    ) -> c_int;
    fn ip_check_defrag__open_and_load() -> *mut ip_check_defrag;
    fn ip_check_defrag__destroy(skel: *mut ip_check_defrag);
    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: isize, expected: isize, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: isize, expected: isize, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
}

unsafe fn sys(command: &'static [u8]) -> c_int {
    let err = system(command.as_ptr() as *const c_char);
    if err != 0 {
        -1
    } else {
        0
    }
}

unsafe fn sys_nofail(command: &'static [u8]) -> c_int {
    system(command.as_ptr() as *const c_char)
}

unsafe fn setup_topology(ipv6: bool) -> c_int {
    let mut up: bool;
    let mut i: c_int;

    if sys(b"ip netns add defrag_ns0\0") != 0 {
        return -1;
    }
    if sys(b"ip netns add defrag_ns1\0") != 0 {
        return -1;
    }
    if sys(b"ip link add veth0 netns defrag_ns0 type veth peer name veth1 netns defrag_ns1\0") != 0 {
        return -1;
    }
    if ipv6 {
        if sys(b"ip -6 -net defrag_ns0 addr add fc00::100/64 dev veth0 nodad\0") != 0 {
            return -1;
        }
        if sys(b"ip -6 -net defrag_ns1 addr add fc00::200/64 dev veth1 nodad\0") != 0 {
            return -1;
        }
    } else {
        if sys(b"ip -net defrag_ns0 addr add 172.16.1.100/24 dev veth0\0") != 0 {
            return -1;
        }
        if sys(b"ip -net defrag_ns1 addr add 172.16.1.200/24 dev veth1\0") != 0 {
            return -1;
        }
    }
    if sys(b"ip -net defrag_ns0 link set dev veth0 up\0") != 0 {
        return -1;
    }
    if sys(b"ip -net defrag_ns1 link set dev veth1 up\0") != 0 {
        return -1;
    }

    /* Wait for up to 5s for links to come up */
    i = 0;
    while i < 5 {
        if ipv6 {
            up = sys_nofail(b"ip netns exec defrag_ns0 ping -6 -c 1 -W 1 fc00::200\0") == 0;
        } else {
            up = sys_nofail(b"ip netns exec defrag_ns0 ping -c 1 -W 1 172.16.1.200\0") == 0;
        }

        if up {
            break;
        }
        i += 1;
    }

    0
}

unsafe fn cleanup_topology() {
    sys_nofail(b"test -f /var/run/netns/defrag_ns0 && ip netns delete defrag_ns0\0");
    sys_nofail(b"test -f /var/run/netns/defrag_ns1 && ip netns delete defrag_ns1\0");
}

unsafe fn attach(skel: *mut ip_check_defrag, ipv6: bool) -> c_int {
    let opts = bpf_netfilter_opts {
        sz: size_of::<bpf_netfilter_opts>(),
        pf: if ipv6 { NFPROTO_IPV6 } else { NFPROTO_IPV4 },
        priority: 42,
        flags: BPF_F_NETFILTER_IP_DEFRAG,
    };
    let mut nstoken: *mut nstoken;
    let mut err: c_int = -1;

    nstoken = open_netns(NS1.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(nstoken as *const c_void, b"setns\0".as_ptr() as *const c_char) {
        goto_out_attach(nstoken);
        return err;
    }

    (*skel).links.defrag = bpf_program__attach_netfilter((*skel).progs.defrag, &opts);
    if !ASSERT_OK_PTR(
        (*skel).links.defrag as *const c_void,
        b"program attach\0".as_ptr() as *const c_char,
    ) {
        goto_out_attach(nstoken);
        return err;
    }

    err = 0;
    goto_out_attach(nstoken);
    err
}

unsafe fn goto_out_attach(nstoken: *mut nstoken) {
    close_netns(nstoken);
}

unsafe fn send_frags(client: c_int) -> c_int {
    let mut saddr: sockaddr_storage = zeroed();
    let saddr_p: *mut sockaddr;
    let mut saddr_len: SocklenT = 0;
    let mut err: isize;

    saddr_p = &mut saddr as *mut sockaddr_storage as *mut sockaddr;
    let make_err = make_sockaddr(
        AF_INET,
        VETH1_ADDR.as_ptr() as *const c_char,
        SERVER_PORT,
        &mut saddr,
        &mut saddr_len,
    );
    if !ASSERT_OK(make_err, b"make_sockaddr\0".as_ptr() as *const c_char) {
        return -1;
    }

    err = sendto(
        client,
        frag_0.as_ptr() as *const c_void,
        size_of_val_extern(&frag_0),
        0,
        saddr_p,
        saddr_len,
    );
    if !ASSERT_GE(err, 0, b"sendto frag_0\0".as_ptr() as *const c_char) {
        return -1;
    }

    err = sendto(
        client,
        frag_1.as_ptr() as *const c_void,
        size_of_val_extern(&frag_1),
        0,
        saddr_p,
        saddr_len,
    );
    if !ASSERT_GE(err, 0, b"sendto frag_1\0".as_ptr() as *const c_char) {
        return -1;
    }

    err = sendto(
        client,
        frag_2.as_ptr() as *const c_void,
        size_of_val_extern(&frag_2),
        0,
        saddr_p,
        saddr_len,
    );
    if !ASSERT_GE(err, 0, b"sendto frag_2\0".as_ptr() as *const c_char) {
        return -1;
    }

    0
}

unsafe fn send_frags6(client: c_int) -> c_int {
    let mut saddr: sockaddr_storage = zeroed();
    let saddr_p: *mut sockaddr;
    let mut saddr_len: SocklenT = 0;
    let mut err: isize;

    saddr_p = &mut saddr as *mut sockaddr_storage as *mut sockaddr;
    /* Port needs to be set to 0 for raw ipv6 socket for some reason */
    let make_err = make_sockaddr(
        AF_INET6,
        VETH1_ADDR6.as_ptr() as *const c_char,
        0,
        &mut saddr,
        &mut saddr_len,
    );
    if !ASSERT_OK(make_err, b"make_sockaddr\0".as_ptr() as *const c_char) {
        return -1;
    }

    err = sendto(
        client,
        frag6_0.as_ptr() as *const c_void,
        size_of_val_extern(&frag6_0),
        0,
        saddr_p,
        saddr_len,
    );
    if !ASSERT_GE(err, 0, b"sendto frag6_0\0".as_ptr() as *const c_char) {
        return -1;
    }

    err = sendto(
        client,
        frag6_1.as_ptr() as *const c_void,
        size_of_val_extern(&frag6_1),
        0,
        saddr_p,
        saddr_len,
    );
    if !ASSERT_GE(err, 0, b"sendto frag6_1\0".as_ptr() as *const c_char) {
        return -1;
    }

    err = sendto(
        client,
        frag6_2.as_ptr() as *const c_void,
        size_of_val_extern(&frag6_2),
        0,
        saddr_p,
        saddr_len,
    );
    if !ASSERT_GE(err, 0, b"sendto frag6_2\0".as_ptr() as *const c_char) {
        return -1;
    }

    0
}

unsafe fn size_of_val_extern<T: ?Sized>(_val: &T) -> usize {
    // File-local Rust cannot know the array lengths from ip_check_defrag_frags.h.
    // This stands in for C's sizeof(frag_N) until those extern objects are bound
    // with their concrete array types.
    size_of::<T>()
}

#[no_mangle]
pub unsafe extern "C" fn test_bpf_ip_check_defrag_ok(ipv6: bool) {
    let family: c_int = if ipv6 { AF_INET6 } else { AF_INET };
    let rx_opts = network_helper_opts {
        timeout_ms: 1000,
        proto: 0,
    };
    let tx_ops = network_helper_opts {
        timeout_ms: 1000,
        proto: IPPROTO_RAW,
    };
    let mut caddr: sockaddr_storage = zeroed();
    let mut skel: *mut ip_check_defrag;
    let mut nstoken: *mut nstoken;
    let mut client_tx_fd: c_int = -1;
    let mut client_rx_fd: c_int = -1;
    let mut caddr_len: SocklenT;
    let mut srv_fd: c_int = -1;
    let mut buf = [0 as c_char; 1024];
    let mut len: isize;
    let mut err: c_int = 0;

    skel = ip_check_defrag__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, b"skel_open\0".as_ptr() as *const c_char) {
        return;
    }

    if !ASSERT_OK(setup_topology(ipv6), b"setup_topology\0".as_ptr() as *const c_char) {
        goto_out_test(skel, client_rx_fd, client_tx_fd, srv_fd);
        return;
    }

    if !ASSERT_OK(attach(skel, ipv6), b"attach\0".as_ptr() as *const c_char) {
        goto_out_test(skel, client_rx_fd, client_tx_fd, srv_fd);
        return;
    }

    /* Start server in ns1 */
    nstoken = open_netns(NS1.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(nstoken as *const c_void, b"setns ns1\0".as_ptr() as *const c_char) {
        goto_out_test(skel, client_rx_fd, client_tx_fd, srv_fd);
        return;
    }
    srv_fd = start_server(family, SOCK_DGRAM, ptr::null(), SERVER_PORT, 0);
    close_netns(nstoken);
    if !ASSERT_GE(srv_fd as isize, 0, b"start_server\0".as_ptr() as *const c_char) {
        goto_out_test(skel, client_rx_fd, client_tx_fd, srv_fd);
        return;
    }

    /* Open tx raw socket in ns0 */
    nstoken = open_netns(NS0.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(nstoken as *const c_void, b"setns ns0\0".as_ptr() as *const c_char) {
        goto_out_test(skel, client_rx_fd, client_tx_fd, srv_fd);
        return;
    }
    client_tx_fd = client_socket(family, SOCK_RAW, &tx_ops);
    close_netns(nstoken);
    if !ASSERT_GE(client_tx_fd as isize, 0, b"client_socket\0".as_ptr() as *const c_char) {
        goto_out_test(skel, client_rx_fd, client_tx_fd, srv_fd);
        return;
    }

    /* Open rx socket in ns0 */
    nstoken = open_netns(NS0.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(nstoken as *const c_void, b"setns ns0\0".as_ptr() as *const c_char) {
        goto_out_test(skel, client_rx_fd, client_tx_fd, srv_fd);
        return;
    }
    client_rx_fd = client_socket(family, SOCK_DGRAM, &rx_opts);
    close_netns(nstoken);
    if !ASSERT_GE(client_rx_fd as isize, 0, b"client_socket\0".as_ptr() as *const c_char) {
        goto_out_test(skel, client_rx_fd, client_tx_fd, srv_fd);
        return;
    }

    /* Bind rx socket to a premeditated port */
    ptr::write_bytes(&mut caddr as *mut sockaddr_storage, 0, 1);
    nstoken = open_netns(NS0.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(nstoken as *const c_void, b"setns ns0\0".as_ptr() as *const c_char) {
        goto_out_test(skel, client_rx_fd, client_tx_fd, srv_fd);
        return;
    }
    if ipv6 {
        let c = &mut caddr as *mut sockaddr_storage as *mut sockaddr_in6;

        (*c).sin6_family = AF_INET6 as u16;
        inet_pton(
            AF_INET6,
            VETH0_ADDR6.as_ptr() as *const c_char,
            &mut (*c).sin6_addr as *mut in6_addr as *mut c_void,
        );
        (*c).sin6_port = htons(CLIENT_PORT as u16);
        err = bind(
            client_rx_fd,
            c as *const sockaddr,
            size_of::<sockaddr_in6>() as SocklenT,
        );
    } else {
        let c = &mut caddr as *mut sockaddr_storage as *mut sockaddr_in;

        (*c).sin_family = AF_INET as u16;
        inet_pton(
            AF_INET,
            VETH0_ADDR.as_ptr() as *const c_char,
            &mut (*c).sin_addr as *mut in_addr as *mut c_void,
        );
        (*c).sin_port = htons(CLIENT_PORT as u16);
        err = bind(
            client_rx_fd,
            c as *const sockaddr,
            size_of::<sockaddr_in>() as SocklenT,
        );
    }
    close_netns(nstoken);
    if !ASSERT_OK(err, b"bind\0".as_ptr() as *const c_char) {
        goto_out_test(skel, client_rx_fd, client_tx_fd, srv_fd);
        return;
    }

    /* Send message in fragments */
    if ipv6 {
        if !ASSERT_OK(send_frags6(client_tx_fd), b"send_frags6\0".as_ptr() as *const c_char) {
            goto_out_test(skel, client_rx_fd, client_tx_fd, srv_fd);
            return;
        }
    } else if !ASSERT_OK(send_frags(client_tx_fd), b"send_frags\0".as_ptr() as *const c_char) {
        goto_out_test(skel, client_rx_fd, client_tx_fd, srv_fd);
        return;
    }

    if !ASSERT_EQ(
        (*(*skel).bss).shootdowns as isize,
        0,
        b"shootdowns\0".as_ptr() as *const c_char,
    ) {
        goto_out_test(skel, client_rx_fd, client_tx_fd, srv_fd);
        return;
    }

    /* Receive reassembled msg on server and echo back to client */
    caddr_len = size_of::<sockaddr_storage>() as SocklenT;
    len = recvfrom(
        srv_fd,
        buf.as_mut_ptr() as *mut c_void,
        size_of::<[c_char; 1024]>(),
        0,
        &mut caddr as *mut sockaddr_storage as *mut sockaddr,
        &mut caddr_len,
    );
    if !ASSERT_GE(len, 0, b"server recvfrom\0".as_ptr() as *const c_char) {
        goto_out_test(skel, client_rx_fd, client_tx_fd, srv_fd);
        return;
    }
    len = sendto(
        srv_fd,
        buf.as_ptr() as *const c_void,
        len as usize,
        0,
        &caddr as *const sockaddr_storage as *const sockaddr,
        caddr_len,
    );
    if !ASSERT_GE(len, 0, b"server sendto\0".as_ptr() as *const c_char) {
        goto_out_test(skel, client_rx_fd, client_tx_fd, srv_fd);
        return;
    }

    /* Expect reassembed message to be echoed back */
    len = recvfrom(
        client_rx_fd,
        buf.as_mut_ptr() as *mut c_void,
        size_of::<[c_char; 1024]>(),
        0,
        ptr::null_mut(),
        ptr::null_mut(),
    );
    if !ASSERT_EQ(
        len,
        (MAGIC_MESSAGE.len() - 1) as isize,
        b"client short read\0".as_ptr() as *const c_char,
    ) {
        goto_out_test(skel, client_rx_fd, client_tx_fd, srv_fd);
        return;
    }

    goto_out_test(skel, client_rx_fd, client_tx_fd, srv_fd);
}

unsafe fn goto_out_test(
    skel: *mut ip_check_defrag,
    client_rx_fd: c_int,
    client_tx_fd: c_int,
    srv_fd: c_int,
) {
    if client_rx_fd != -1 {
        close(client_rx_fd);
    }
    if client_tx_fd != -1 {
        close(client_tx_fd);
    }
    if srv_fd != -1 {
        close(srv_fd);
    }
    cleanup_topology();
    ip_check_defrag__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_bpf_ip_check_defrag() {
    if test__start_subtest(b"v4\0".as_ptr() as *const c_char) {
        test_bpf_ip_check_defrag_ok(false);
    }
    if test__start_subtest(b"v6\0".as_ptr() as *const c_char) {
        test_bpf_ip_check_defrag_ok(true);
    }
}
