// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause

/*
 * Topology:
 * ---------
 *     NS0 namespace         |   NS1 namespace
 *			     |
 *     +--------------+      |   +--------------+
 *     |    veth01    |----------|    veth10    |
 *     | 172.16.1.100 |      |   | 172.16.1.200 |
 *     |     bpf      |      |   +--------------+
 *     +--------------+      |
 *      server(UDP/TCP)      |
 *  +-------------------+    |
 *  |        vrf1       |    |
 *  |  +--------------+ |    |   +--------------+
 *  |  |    veth02    |----------|    veth20    |
 *  |  | 172.16.2.100 | |    |   | 172.16.2.200 |
 *  |  |     bpf      | |    |   +--------------+
 *  |  +--------------+ |    |
 *  |   server(UDP/TCP) |    |
 *  +-------------------+    |
 *
 * Test flow
 * -----------
 *  The tests verifies that socket lookup via TC is VRF aware:
 *  1) Creates two veth pairs between NS0 and NS1:
 *     a) veth01 <-> veth10 outside the VRF
 *     b) veth02 <-> veth20 in the VRF
 *  2) Attaches to veth01 and veth02 a program that calls:
 *     a) bpf_skc_lookup_tcp() with TCP and tcp_skc is true
 *     b) bpf_sk_lookup_tcp() with TCP and tcp_skc is false
 *     c) bpf_sk_lookup_udp() with UDP
 *     The program stores the lookup result in bss->lookup_status.
 *  3) Creates a socket TCP/UDP server in/outside the VRF.
 *  4) The test expects lookup_status to be:
 *     a) 0 from device in VRF to server outside VRF
 *     b) 0 from device outside VRF to server in VRF
 *     c) 1 from device in VRF to server in VRF
 *     d) 1 from device outside VRF to server outside VRF
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

const NS0: &[u8] = b"vrf_socket_lookup_0\0";
const NS1: &[u8] = b"vrf_socket_lookup_1\0";

const IP4_ADDR_VETH01: &[u8] = b"172.16.1.100\0";
const IP4_ADDR_VETH10: &[u8] = b"172.16.1.200\0";
const IP4_ADDR_VETH02: &[u8] = b"172.16.2.100\0";
const IP4_ADDR_VETH20: &[u8] = b"172.16.2.200\0";

const NON_VRF_PORT: c_int = 5000;
const IN_VRF_PORT: c_int = 5001;

const TIMEOUT_MS: c_int = 3000;

const AF_INET: c_int = 2;
const SOCK_STREAM: c_int = 1;
const SOL_SOCKET: c_int = 1;
const SO_BINDTODEVICE: c_int = 25;
const BPF_TC_INGRESS: c_int = 1;

#[repr(C)]
pub struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
pub struct sockaddr_in {
    sin_family: u16,
    sin_port: u16,
    sin_addr: u32,
    sin_zero: [u8; 8],
}

#[repr(C)]
pub struct sockaddr_storage {
    ss_family: u16,
    __ss_padding: [u8; 118],
    __ss_align: u64,
}

#[repr(C)]
pub struct nstoken {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vrf_socket_lookup_bss {
    test_xdp: bool,
    tcp_skc: bool,
    lookup_status: c_int,
}

#[repr(C)]
pub struct vrf_socket_lookup_progs {
    tc_socket_lookup: *mut bpf_program,
    xdp_socket_lookup: *mut bpf_program,
}

#[repr(C)]
pub struct vrf_socket_lookup {
    progs: vrf_socket_lookup_progs,
    bss: *mut vrf_socket_lookup_bss,
}

#[repr(C)]
pub struct bpf_tc_hook {
    sz: usize,
    ifindex: c_int,
    attach_point: c_int,
}

#[repr(C)]
pub struct bpf_tc_opts {
    sz: usize,
    prog_fd: c_int,
    flags: c_uint,
    prog_id: c_uint,
    handle: c_uint,
    priority: c_uint,
}

unsafe extern "C" {
    fn make_sockaddr(
        family: c_int,
        ip: *const c_char,
        port: c_int,
        addr: *mut sockaddr_storage,
        len: *mut c_void,
    ) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn settimeo(fd: c_int, timeout_ms: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn start_server(
        family: c_int,
        type_: c_int,
        addr: *const c_char,
        port: c_int,
        timeout_ms: c_int,
    ) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: u32,
    ) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn bpf_tc_hook_create(hook: *mut bpf_tc_hook) -> c_int;
    fn bpf_tc_hook_destroy(hook: *mut bpf_tc_hook) -> c_int;
    fn bpf_tc_attach(hook: *mut bpf_tc_hook, opts: *mut bpf_tc_opts) -> c_int;
    fn bpf_xdp_attach(ifindex: c_int, prog_fd: c_int, flags: c_uint, opts: *const c_void) -> c_int;
    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(token: *mut nstoken);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn connect(sockfd: c_int, addr: *const sockaddr, addrlen: u32) -> c_int;
    fn sendto(
        sockfd: c_int,
        buf: *const c_void,
        len: usize,
        flags: c_int,
        dest_addr: *const sockaddr,
        addrlen: u32,
    ) -> isize;
    fn vrf_socket_lookup__open_and_load() -> *mut vrf_socket_lookup;
    fn vrf_socket_lookup__destroy(obj: *mut vrf_socket_lookup);
    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(value: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_NEQ(value: c_uint, expected: c_uint, name: *const c_char) -> bool;
    fn ASSERT_EQ(value: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;
    fn SYS(cmd: *const c_char) -> c_int;
    fn SYS_NOFAIL(cmd: *const c_char);
}

unsafe fn make_socket(
    sotype: c_int,
    ip: *const c_char,
    port: c_int,
    addr: *mut sockaddr_storage,
) -> c_int {
    let mut err: c_int;
    let fd: c_int;

    err = make_sockaddr(AF_INET, ip, port, addr, ptr::null_mut());
    if !ASSERT_OK(err, c"make_address".as_ptr()) {
        return -1;
    }

    fd = socket(AF_INET, sotype, 0);
    if !ASSERT_GE(fd, 0, c"socket".as_ptr()) {
        return -1;
    }

    if !ASSERT_OK(settimeo(fd, TIMEOUT_MS), c"settimeo".as_ptr()) {
        close(fd);
        return -1;
    }

    fd
}

unsafe fn make_server(
    sotype: c_int,
    ip: *const c_char,
    port: c_int,
    ifname: *const c_char,
) -> c_int {
    let mut err: c_int;
    let fd: c_int;

    fd = start_server(AF_INET, sotype, ip, port, TIMEOUT_MS);
    if !ASSERT_GE(fd, 0, c"start_server".as_ptr()) {
        return -1;
    }

    if !ifname.is_null() {
        err = setsockopt(
            fd,
            SOL_SOCKET,
            SO_BINDTODEVICE,
            ifname as *const c_void,
            (strlen(ifname) + 1) as u32,
        );
        if !ASSERT_OK(err, c"setsockopt(SO_BINDTODEVICE)".as_ptr()) {
            close(fd);
            return -1;
        }
    }

    fd
}

unsafe fn attach_progs(ifname: *mut c_char, tc_prog_fd: c_int, xdp_prog_fd: c_int) -> c_int {
    let mut hook = bpf_tc_hook {
        sz: mem::size_of::<bpf_tc_hook>(),
        ifindex: 0,
        attach_point: BPF_TC_INGRESS,
    };
    let mut opts = bpf_tc_opts {
        sz: mem::size_of::<bpf_tc_opts>(),
        prog_fd: tc_prog_fd,
        flags: 0,
        prog_id: 0,
        handle: 1,
        priority: 1,
    };
    let mut ret: c_int;
    let ifindex: c_uint;

    ifindex = if_nametoindex(ifname);
    if !ASSERT_NEQ(ifindex, 0, c"if_nametoindex".as_ptr()) {
        return -1;
    }
    hook.ifindex = ifindex as c_int;

    ret = bpf_tc_hook_create(&mut hook);
    if !ASSERT_OK(ret, c"bpf_tc_hook_create".as_ptr()) {
        return ret;
    }

    ret = bpf_tc_attach(&mut hook, &mut opts);
    if !ASSERT_OK(ret, c"bpf_tc_attach".as_ptr()) {
        bpf_tc_hook_destroy(&mut hook);
        return ret;
    }
    ret = bpf_xdp_attach(ifindex as c_int, xdp_prog_fd, 0, ptr::null());
    if !ASSERT_OK(ret, c"bpf_xdp_attach".as_ptr()) {
        bpf_tc_hook_destroy(&mut hook);
        return ret;
    }

    0
}

unsafe fn cleanup() {
    SYS_NOFAIL(c"test -f /var/run/netns/vrf_socket_lookup_0 && ip netns delete vrf_socket_lookup_0".as_ptr());
    SYS_NOFAIL(c"test -f /var/run/netns/vrf_socket_lookup_1 && ip netns delete vrf_socket_lookup_1".as_ptr());
}

unsafe fn setup(skel: *mut vrf_socket_lookup) -> c_int {
    let tc_prog_fd: c_int;
    let xdp_prog_fd: c_int;
    let mut ret: c_int = 0;
    let mut nstoken: *mut nstoken = ptr::null_mut();

    if SYS(c"ip netns add vrf_socket_lookup_0".as_ptr()) != 0 {
        ret = -1;
        goto_close(nstoken);
        return ret;
    }
    if SYS(c"ip netns add vrf_socket_lookup_1".as_ptr()) != 0 {
        ret = -1;
        goto_close(nstoken);
        return ret;
    }

    /* NS0 <-> NS1 [veth01 <-> veth10] */
    if SYS(c"ip link add veth01 netns vrf_socket_lookup_0 type veth peer name veth10 netns vrf_socket_lookup_1".as_ptr()) != 0 {
        ret = -1;
        goto_close(nstoken);
        return ret;
    }
    if SYS(c"ip -net vrf_socket_lookup_0 addr add 172.16.1.100/24 dev veth01".as_ptr()) != 0 {
        ret = -1;
        goto_close(nstoken);
        return ret;
    }
    if SYS(c"ip -net vrf_socket_lookup_0 link set dev veth01 up".as_ptr()) != 0 {
        ret = -1;
        goto_close(nstoken);
        return ret;
    }
    if SYS(c"ip -net vrf_socket_lookup_1 addr add 172.16.1.200/24 dev veth10".as_ptr()) != 0 {
        ret = -1;
        goto_close(nstoken);
        return ret;
    }
    if SYS(c"ip -net vrf_socket_lookup_1 link set dev veth10 up".as_ptr()) != 0 {
        ret = -1;
        goto_close(nstoken);
        return ret;
    }

    /* NS0 <-> NS1 [veth02 <-> veth20] */
    if SYS(c"ip link add veth02 netns vrf_socket_lookup_0 type veth peer name veth20 netns vrf_socket_lookup_1".as_ptr()) != 0 {
        ret = -1;
        goto_close(nstoken);
        return ret;
    }
    if SYS(c"ip -net vrf_socket_lookup_0 addr add 172.16.2.100/24 dev veth02".as_ptr()) != 0 {
        ret = -1;
        goto_close(nstoken);
        return ret;
    }
    if SYS(c"ip -net vrf_socket_lookup_0 link set dev veth02 up".as_ptr()) != 0 {
        ret = -1;
        goto_close(nstoken);
        return ret;
    }
    if SYS(c"ip -net vrf_socket_lookup_1 addr add 172.16.2.200/24 dev veth20".as_ptr()) != 0 {
        ret = -1;
        goto_close(nstoken);
        return ret;
    }
    if SYS(c"ip -net vrf_socket_lookup_1 link set dev veth20 up".as_ptr()) != 0 {
        ret = -1;
        goto_close(nstoken);
        return ret;
    }

    /* veth02 -> vrf1  */
    if SYS(c"ip -net vrf_socket_lookup_0 link add vrf1 type vrf table 11".as_ptr()) != 0 {
        ret = -1;
        goto_close(nstoken);
        return ret;
    }
    if SYS(c"ip -net vrf_socket_lookup_0 route add vrf vrf1 unreachable default metric 4278198272".as_ptr()) != 0 {
        ret = -1;
        goto_close(nstoken);
        return ret;
    }
    if SYS(c"ip -net vrf_socket_lookup_0 link set vrf1 alias vrf".as_ptr()) != 0 {
        ret = -1;
        goto_close(nstoken);
        return ret;
    }
    if SYS(c"ip -net vrf_socket_lookup_0 link set vrf1 up".as_ptr()) != 0 {
        ret = -1;
        goto_close(nstoken);
        return ret;
    }
    if SYS(c"ip -net vrf_socket_lookup_0 link set veth02 master vrf1".as_ptr()) != 0 {
        ret = -1;
        goto_close(nstoken);
        return ret;
    }

    /* Attach TC and XDP progs to veth devices in NS0 */
    nstoken = open_netns(NS0.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(nstoken, c"setns vrf_socket_lookup_0".as_ptr()) {
        ret = -1;
        goto_close(nstoken);
        return ret;
    }
    tc_prog_fd = bpf_program__fd((*skel).progs.tc_socket_lookup);
    if !ASSERT_GE(tc_prog_fd, 0, c"bpf_program__tc_fd".as_ptr()) {
        ret = -1;
        goto_close(nstoken);
        return ret;
    }
    xdp_prog_fd = bpf_program__fd((*skel).progs.xdp_socket_lookup);
    if !ASSERT_GE(xdp_prog_fd, 0, c"bpf_program__xdp_fd".as_ptr()) {
        ret = -1;
        goto_close(nstoken);
        return ret;
    }

    if attach_progs(c"veth01".as_ptr() as *mut c_char, tc_prog_fd, xdp_prog_fd) != 0 {
        ret = -1;
        goto_close(nstoken);
        return ret;
    }

    if attach_progs(c"veth02".as_ptr() as *mut c_char, tc_prog_fd, xdp_prog_fd) != 0 {
        ret = -1;
        goto_close(nstoken);
        return ret;
    }

    goto_close(nstoken);
    ret
}

unsafe fn goto_close(nstoken: *mut nstoken) {
    if !nstoken.is_null() {
        close_netns(nstoken);
    }
}

unsafe fn test_lookup(
    skel: *mut vrf_socket_lookup,
    sotype: c_int,
    ip: *const c_char,
    port: c_int,
    test_xdp: bool,
    tcp_skc: bool,
    lookup_status_exp: c_int,
) -> c_int {
    static MSG: &[u8] = b"Hello Server\0";
    let mut addr: sockaddr_storage = mem::zeroed();
    let fd: c_int;
    let mut ret: c_int = 0;

    fd = make_socket(sotype, ip, port, &mut addr);
    if fd < 0 {
        return -1;
    }

    (*(*skel).bss).test_xdp = test_xdp;
    (*(*skel).bss).tcp_skc = tcp_skc;
    (*(*skel).bss).lookup_status = -1;

    if sotype == SOCK_STREAM {
        connect(
            fd,
            &addr as *const sockaddr_storage as *const sockaddr,
            mem::size_of::<sockaddr_in>() as u32,
        );
    } else {
        sendto(
            fd,
            MSG.as_ptr() as *const c_void,
            MSG.len(),
            0,
            &addr as *const sockaddr_storage as *const sockaddr,
            mem::size_of::<sockaddr_in>() as u32,
        );
    }

    if !ASSERT_EQ((*(*skel).bss).lookup_status, lookup_status_exp, c"lookup_status".as_ptr()) {
        ret = -1;
    }

    close(fd);
    ret
}

unsafe fn _test_vrf_socket_lookup(
    skel: *mut vrf_socket_lookup,
    sotype: c_int,
    test_xdp: bool,
    tcp_skc: bool,
) {
    let mut in_vrf_server: c_int = -1;
    let mut non_vrf_server: c_int = -1;
    let mut nstoken: *mut nstoken = ptr::null_mut();

    nstoken = open_netns(NS0.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(nstoken, c"setns vrf_socket_lookup_0".as_ptr()) {
        goto_done(non_vrf_server, in_vrf_server, nstoken);
        return;
    }

    /* Open sockets in and outside VRF */
    non_vrf_server = make_server(sotype, c"0.0.0.0".as_ptr(), NON_VRF_PORT, ptr::null());
    if !ASSERT_GE(non_vrf_server, 0, c"make_server__outside_vrf_fd".as_ptr()) {
        goto_done(non_vrf_server, in_vrf_server, nstoken);
        return;
    }

    in_vrf_server = make_server(sotype, c"0.0.0.0".as_ptr(), IN_VRF_PORT, c"veth02".as_ptr());
    if !ASSERT_GE(in_vrf_server, 0, c"make_server__in_vrf_fd".as_ptr()) {
        goto_done(non_vrf_server, in_vrf_server, nstoken);
        return;
    }

    /* Perform test from NS1 */
    close_netns(nstoken);
    nstoken = open_netns(NS1.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(nstoken, c"setns vrf_socket_lookup_1".as_ptr()) {
        goto_done(non_vrf_server, in_vrf_server, nstoken);
        return;
    }

    if !ASSERT_OK(
        test_lookup(
            skel,
            sotype,
            IP4_ADDR_VETH02.as_ptr() as *const c_char,
            NON_VRF_PORT,
            test_xdp,
            tcp_skc,
            0,
        ),
        c"in_to_out".as_ptr(),
    ) {
        goto_done(non_vrf_server, in_vrf_server, nstoken);
        return;
    }
    if !ASSERT_OK(
        test_lookup(
            skel,
            sotype,
            IP4_ADDR_VETH02.as_ptr() as *const c_char,
            IN_VRF_PORT,
            test_xdp,
            tcp_skc,
            1,
        ),
        c"in_to_in".as_ptr(),
    ) {
        goto_done(non_vrf_server, in_vrf_server, nstoken);
        return;
    }
    if !ASSERT_OK(
        test_lookup(
            skel,
            sotype,
            IP4_ADDR_VETH01.as_ptr() as *const c_char,
            NON_VRF_PORT,
            test_xdp,
            tcp_skc,
            1,
        ),
        c"out_to_out".as_ptr(),
    ) {
        goto_done(non_vrf_server, in_vrf_server, nstoken);
        return;
    }
    if !ASSERT_OK(
        test_lookup(
            skel,
            sotype,
            IP4_ADDR_VETH01.as_ptr() as *const c_char,
            IN_VRF_PORT,
            test_xdp,
            tcp_skc,
            0,
        ),
        c"out_to_in".as_ptr(),
    ) {
        goto_done(non_vrf_server, in_vrf_server, nstoken);
        return;
    }

    goto_done(non_vrf_server, in_vrf_server, nstoken);
}

unsafe fn goto_done(non_vrf_server: c_int, in_vrf_server: c_int, nstoken: *mut nstoken) {
    if non_vrf_server >= 0 {
        close(non_vrf_server);
    }
    if in_vrf_server >= 0 {
        close(in_vrf_server);
    }
    if !nstoken.is_null() {
        close_netns(nstoken);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_vrf_socket_lookup() {
    let skel: *mut vrf_socket_lookup;

    cleanup();

    skel = vrf_socket_lookup__open_and_load();
    if !ASSERT_OK_PTR(skel, c"vrf_socket_lookup__open_and_load".as_ptr()) {
        return;
    }

    if !ASSERT_OK(setup(skel), c"setup".as_ptr()) {
        vrf_socket_lookup__destroy(skel);
        cleanup();
        return;
    }

    if test__start_subtest(c"tc_socket_lookup_tcp".as_ptr()) {
        _test_vrf_socket_lookup(skel, SOCK_STREAM, false, false);
    }
    if test__start_subtest(c"tc_socket_lookup_tcp_skc".as_ptr()) {
        _test_vrf_socket_lookup(skel, SOCK_STREAM, false, false);
    }
    if test__start_subtest(c"tc_socket_lookup_udp".as_ptr()) {
        _test_vrf_socket_lookup(skel, SOCK_STREAM, false, false);
    }
    if test__start_subtest(c"xdp_socket_lookup_tcp".as_ptr()) {
        _test_vrf_socket_lookup(skel, SOCK_STREAM, true, false);
    }
    if test__start_subtest(c"xdp_socket_lookup_tcp_skc".as_ptr()) {
        _test_vrf_socket_lookup(skel, SOCK_STREAM, true, false);
    }
    if test__start_subtest(c"xdp_socket_lookup_udp".as_ptr()) {
        _test_vrf_socket_lookup(skel, SOCK_STREAM, true, false);
    }

    vrf_socket_lookup__destroy(skel);
    cleanup();
}
