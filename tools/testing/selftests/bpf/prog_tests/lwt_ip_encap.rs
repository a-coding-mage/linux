// SPDX-License-Identifier: GPL-2.0-only
//
// Dependencies from the original C includes:
// <netinet/in.h>, "network_helpers.h", "test_progs.h",
// and "test_lwt_ip_encap.skel.h".

use core::ffi::{c_char, c_int, c_void};

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;

const BPF_FILE: &[u8] = b"test_lwt_ip_encap.bpf.o\0";

const NETNS_NAME_SIZE: usize = 32;
const NETNS_BASE: &str = "ns-lwt-ip-encap";

const IP4_ADDR_1: &[u8] = b"172.16.1.100\0";
const IP4_ADDR_2: &[u8] = b"172.16.2.100\0";
const IP4_ADDR_3: &[u8] = b"172.16.3.100\0";
const IP4_ADDR_4: &[u8] = b"172.16.4.100\0";
const IP4_ADDR_5: &[u8] = b"172.16.5.100\0";
const IP4_ADDR_6: &[u8] = b"172.16.6.100\0";
const IP4_ADDR_7: &[u8] = b"172.16.7.100\0";
const IP4_ADDR_8: &[u8] = b"172.16.8.100\0";
const IP4_ADDR_GRE: &[u8] = b"172.16.16.100\0";

const IP4_ADDR_SRC: &[u8] = IP4_ADDR_1;
const IP4_ADDR_DST: &[u8] = IP4_ADDR_4;

const IP6_ADDR_1: &[u8] = b"fb01::1\0";
const IP6_ADDR_2: &[u8] = b"fb02::1\0";
const IP6_ADDR_3: &[u8] = b"fb03::1\0";
const IP6_ADDR_4: &[u8] = b"fb04::1\0";
const IP6_ADDR_5: &[u8] = b"fb05::1\0";
const IP6_ADDR_6: &[u8] = b"fb06::1\0";
const IP6_ADDR_7: &[u8] = b"fb07::1\0";
const IP6_ADDR_8: &[u8] = b"fb08::1\0";
const IP6_ADDR_GRE: &[u8] = b"fb10::1\0";

const IP4_ADDR_VXLAN: &[u8] = b"172.16.17.100\0";
const IP6_ADDR_VXLAN: &[u8] = b"fb11::1\0";

const IP6_ADDR_SRC: &[u8] = IP6_ADDR_1;
const IP6_ADDR_DST: &[u8] = IP6_ADDR_4;

const GSO_SIZE: usize = 5000;
const GSO_TCP_PORT: c_int = 9000;

const EGRESS: bool = true;
const INGRESS: bool = false;
const IPV4_ENCAP: bool = true;
const IPV6_ENCAP: bool = false;

#[repr(C)]
struct nstoken {
    _private: [u8; 0],
}

#[repr(C)]
struct iphdr {
    _private: [u8; 0],
}

#[repr(C)]
struct ipv6hdr {
    _private: [u8; 0],
}

#[repr(C)]
struct test_lwt_ip_encap {
    progs: test_lwt_ip_encap_progs,
    rodata: *mut test_lwt_ip_encap_rodata,
    bss: *mut test_lwt_ip_encap_bss,
}

#[repr(C)]
struct test_lwt_ip_encap_progs {
    bpf_lwt_encap_gre: *mut c_void,
    bpf_lwt_encap_gre6: *mut c_void,
    bpf_lwt_encap_vxlan: *mut c_void,
    bpf_lwt_encap_vxlan6: *mut c_void,
    fexit_lwt_push_ip_encap: *mut c_void,
}

#[repr(C)]
struct test_lwt_ip_encap_rodata {
    tgt_ip_version: c_int,
}

#[repr(C)]
struct test_lwt_ip_encap_bss {
    fexit_triggered: bool,
    transport_hdr: c_int,
    network_hdr: c_int,
}

unsafe extern "C" {
    fn append_tid(name: *mut c_char, name_sz: usize) -> c_int;
    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(token: *mut nstoken);
    fn ping_command(family: c_int) -> *const c_char;
    fn start_server_str(
        family: c_int,
        socktype: c_int,
        addr: *const c_char,
        port: c_int,
        opts: *mut c_void,
    ) -> c_int;
    fn connect_to_addr_str(
        family: c_int,
        socktype: c_int,
        addr: *const c_char,
        port: c_int,
        opts: *mut c_void,
    ) -> c_int;
    fn accept(fd: c_int, addr: *mut c_void, len: *mut c_void) -> c_int;
    fn send(fd: c_int, buf: *const c_void, len: usize, flags: c_int) -> isize;
    fn read(fd: c_int, buf: *mut c_void, len: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn bpf_program__set_autoload(prog: *mut c_void, autoload: bool);
    fn test_lwt_ip_encap__open() -> *mut test_lwt_ip_encap;
    fn test_lwt_ip_encap__load(skel: *mut test_lwt_ip_encap) -> c_int;
    fn test_lwt_ip_encap__attach(skel: *mut test_lwt_ip_encap) -> c_int;
    fn test_lwt_ip_encap__destroy(skel: *mut test_lwt_ip_encap);
}

fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

fn init_ns_name(suffix: &[u8]) -> [c_char; NETNS_NAME_SIZE] {
    let mut name = [0 as c_char; NETNS_NAME_SIZE];
    let base = NETNS_BASE.as_bytes();
    let mut i = 0;
    while i < base.len() {
        name[i] = base[i] as c_char;
        i += 1;
    }
    let mut j = 0;
    while j < suffix.len() {
        name[i + j] = suffix[j] as c_char;
        j += 1;
    }
    name
}

/* Setup/topology:
 *
 *    NS1             NS2             NS3
 *   veth1 <---> veth2   veth3 <---> veth4 (the top route)
 *   veth5 <---> veth6   veth7 <---> veth8 (the bottom route)
 *
 *   Each vethN gets IP[4|6]_ADDR_N address.
 *
 *   IP*_ADDR_SRC = IP*_ADDR_1
 *   IP*_ADDR_DST = IP*_ADDR_4
 *
 *   All tests test pings from IP*_ADDR__SRC to IP*_ADDR_DST.
 *
 *   By default, routes are configured to allow packets to go
 *   IP*_ADDR_1 <=> IP*_ADDR_2 <=> IP*_ADDR_3 <=> IP*_ADDR_4 (the top route).
 *
 *   A GRE device is installed in NS3 with IP*_ADDR_GRE, and
 *   NS1/NS2 are configured to route packets to IP*_ADDR_GRE via IP*_ADDR_8
 *   (the bottom route).
 *
 * Tests:
 *
 *   1. Routes NS2->IP*_ADDR_DST are brought down, so the only way a ping
 *      from IP*_ADDR_SRC to IP*_ADDR_DST can work is via IP*_ADDR_GRE.
 *
 *   2a. In an egress test, a bpf LWT_XMIT program is installed on veth1
 *       that encaps the packets with an IP/GRE header to route to IP*_ADDR_GRE.
 *
 *       ping: SRC->[encap at veth1:egress]->GRE:decap->DST
 *       ping replies go DST->SRC directly
 *
 *   2b. In an ingress test, a bpf LWT_IN program is installed on veth2
 *       that encaps the packets with an IP/GRE header to route to IP*_ADDR_GRE.
 *
 *       ping: SRC->[encap at veth2:ingress]->GRE:decap->DST
 *       ping replies go DST->SRC directly
 */

unsafe fn create_ns(name: *mut c_char, name_sz: usize) -> c_int {
    if name.is_null() {
        return -1;
    }

    if !ASSERT_OK!(append_tid(name, name_sz), c"append TID".as_ptr()) {
        return -1;
    }

    if SYS!(c"ip netns add %s".as_ptr(), name) != 0 {
        return -1;
    }

    /* rp_filter gets confused by what these tests are doing, so disable it */
    if SYS!(c"ip netns exec %s sysctl -wq net.ipv4.conf.all.rp_filter=0".as_ptr(), name) != 0 {
        return -1;
    }
    if SYS!(c"ip netns exec %s sysctl -wq net.ipv4.conf.default.rp_filter=0".as_ptr(), name) != 0 {
        return -1;
    }
    /* Disable IPv6 DAD because it sometimes takes too long and fails tests */
    if SYS!(c"ip netns exec %s sysctl -wq net.ipv6.conf.all.accept_dad=0".as_ptr(), name) != 0 {
        return -1;
    }
    if SYS!(c"ip netns exec %s sysctl -wq net.ipv6.conf.default.accept_dad=0".as_ptr(), name) != 0 {
        return -1;
    }

    0
}

unsafe fn set_top_addr(ns1: *const c_char, ns2: *const c_char, ns3: *const c_char) -> c_int {
    if SYS!(c"ip -n %s    a add %s/24  dev veth1".as_ptr(), ns1, cstr(IP4_ADDR_1)) != 0 { return 1; }
    if SYS!(c"ip -n %s    a add %s/24  dev veth2".as_ptr(), ns2, cstr(IP4_ADDR_2)) != 0 { return 1; }
    if SYS!(c"ip -n %s    a add %s/24  dev veth3".as_ptr(), ns2, cstr(IP4_ADDR_3)) != 0 { return 1; }
    if SYS!(c"ip -n %s    a add %s/24  dev veth4".as_ptr(), ns3, cstr(IP4_ADDR_4)) != 0 { return 1; }
    if SYS!(c"ip -n %s -6 a add %s/128 dev veth1".as_ptr(), ns1, cstr(IP6_ADDR_1)) != 0 { return 1; }
    if SYS!(c"ip -n %s -6 a add %s/128 dev veth2".as_ptr(), ns2, cstr(IP6_ADDR_2)) != 0 { return 1; }
    if SYS!(c"ip -n %s -6 a add %s/128 dev veth3".as_ptr(), ns2, cstr(IP6_ADDR_3)) != 0 { return 1; }
    if SYS!(c"ip -n %s -6 a add %s/128 dev veth4".as_ptr(), ns3, cstr(IP6_ADDR_4)) != 0 { return 1; }

    if SYS!(c"ip -n %s link set dev veth1 up".as_ptr(), ns1) != 0 { return 1; }
    if SYS!(c"ip -n %s link set dev veth2 up".as_ptr(), ns2) != 0 { return 1; }
    if SYS!(c"ip -n %s link set dev veth3 up".as_ptr(), ns2) != 0 { return 1; }
    if SYS!(c"ip -n %s link set dev veth4 up".as_ptr(), ns3) != 0 { return 1; }

    0
}

unsafe fn set_bottom_addr(ns1: *const c_char, ns2: *const c_char, ns3: *const c_char) -> c_int {
    if SYS!(c"ip -n %s    a add %s/24  dev veth5".as_ptr(), ns1, cstr(IP4_ADDR_5)) != 0 { return 1; }
    if SYS!(c"ip -n %s    a add %s/24  dev veth6".as_ptr(), ns2, cstr(IP4_ADDR_6)) != 0 { return 1; }
    if SYS!(c"ip -n %s    a add %s/24  dev veth7".as_ptr(), ns2, cstr(IP4_ADDR_7)) != 0 { return 1; }
    if SYS!(c"ip -n %s    a add %s/24  dev veth8".as_ptr(), ns3, cstr(IP4_ADDR_8)) != 0 { return 1; }
    if SYS!(c"ip -n %s -6 a add %s/128 dev veth5".as_ptr(), ns1, cstr(IP6_ADDR_5)) != 0 { return 1; }
    if SYS!(c"ip -n %s -6 a add %s/128 dev veth6".as_ptr(), ns2, cstr(IP6_ADDR_6)) != 0 { return 1; }
    if SYS!(c"ip -n %s -6 a add %s/128 dev veth7".as_ptr(), ns2, cstr(IP6_ADDR_7)) != 0 { return 1; }
    if SYS!(c"ip -n %s -6 a add %s/128 dev veth8".as_ptr(), ns3, cstr(IP6_ADDR_8)) != 0 { return 1; }

    if SYS!(c"ip -n %s link set dev veth5 up".as_ptr(), ns1) != 0 { return 1; }
    if SYS!(c"ip -n %s link set dev veth6 up".as_ptr(), ns2) != 0 { return 1; }
    if SYS!(c"ip -n %s link set dev veth7 up".as_ptr(), ns2) != 0 { return 1; }
    if SYS!(c"ip -n %s link set dev veth8 up".as_ptr(), ns3) != 0 { return 1; }

    0
}

unsafe fn configure_vrf(ns1: *const c_char, ns2: *const c_char) -> c_int {
    if ns1.is_null() || ns2.is_null() {
        return -1;
    }

    if SYS!(c"ip -n %s link add red type vrf table 1001".as_ptr(), ns1) != 0 { return -1; }
    if SYS!(c"ip -n %s link set red up".as_ptr(), ns1) != 0 { return -1; }
    if SYS!(c"ip -n %s route add table 1001 unreachable default metric 8192".as_ptr(), ns1) != 0 { return -1; }
    if SYS!(c"ip -n %s -6 route add table 1001 unreachable default metric 8192".as_ptr(), ns1) != 0 { return -1; }
    if SYS!(c"ip -n %s link set veth1 vrf red".as_ptr(), ns1) != 0 { return -1; }
    if SYS!(c"ip -n %s link set veth5 vrf red".as_ptr(), ns1) != 0 { return -1; }

    if SYS!(c"ip -n %s link add red type vrf table 1001".as_ptr(), ns2) != 0 { return -1; }
    if SYS!(c"ip -n %s link set red up".as_ptr(), ns2) != 0 { return -1; }
    if SYS!(c"ip -n %s route add table 1001 unreachable default metric 8192".as_ptr(), ns2) != 0 { return -1; }
    if SYS!(c"ip -n %s -6 route add table 1001 unreachable default metric 8192".as_ptr(), ns2) != 0 { return -1; }
    if SYS!(c"ip -n %s link set veth2 vrf red".as_ptr(), ns2) != 0 { return -1; }
    if SYS!(c"ip -n %s link set veth3 vrf red".as_ptr(), ns2) != 0 { return -1; }
    if SYS!(c"ip -n %s link set veth6 vrf red".as_ptr(), ns2) != 0 { return -1; }
    if SYS!(c"ip -n %s link set veth7 vrf red".as_ptr(), ns2) != 0 { return -1; }

    0
}

unsafe fn configure_ns1(ns1: *const c_char, vrf: *const c_char) -> c_int {
    let mut nstoken: *mut nstoken = core::ptr::null_mut();

    if ns1.is_null() || vrf.is_null() {
        return -1;
    }

    nstoken = open_netns(ns1);
    if !ASSERT_OK_PTR!(nstoken, c"open ns1".as_ptr()) {
        return -1;
    }

    /* Top route */
    if SYS!(c"ip    route add %s/32  dev veth1 %s".as_ptr(), cstr(IP4_ADDR_2), vrf) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip    route add default dev veth1 via %s %s".as_ptr(), cstr(IP4_ADDR_2), vrf) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip -6 route add %s/128 dev veth1 %s".as_ptr(), cstr(IP6_ADDR_2), vrf) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip -6 route add default dev veth1 via %s %s".as_ptr(), cstr(IP6_ADDR_2), vrf) != 0 { close_netns(nstoken); return -1; }
    /* Bottom route */
    if SYS!(c"ip    route add %s/32  dev veth5 %s".as_ptr(), cstr(IP4_ADDR_6), vrf) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip    route add %s/32  dev veth5 via  %s %s".as_ptr(), cstr(IP4_ADDR_7), cstr(IP4_ADDR_6), vrf) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip    route add %s/32  dev veth5 via  %s %s".as_ptr(), cstr(IP4_ADDR_8), cstr(IP4_ADDR_6), vrf) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip -6 route add %s/128 dev veth5 %s".as_ptr(), cstr(IP6_ADDR_6), vrf) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip -6 route add %s/128 dev veth5 via  %s %s".as_ptr(), cstr(IP6_ADDR_7), cstr(IP6_ADDR_6), vrf) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip -6 route add %s/128 dev veth5 via  %s %s".as_ptr(), cstr(IP6_ADDR_8), cstr(IP6_ADDR_6), vrf) != 0 { close_netns(nstoken); return -1; }

    close_netns(nstoken);
    0
}

unsafe fn configure_ns2(ns2: *const c_char, vrf: *const c_char) -> c_int {
    let mut nstoken: *mut nstoken = core::ptr::null_mut();

    if ns2.is_null() || vrf.is_null() {
        return -1;
    }

    nstoken = open_netns(ns2);
    if !ASSERT_OK_PTR!(nstoken, c"open ns2".as_ptr()) {
        return -1;
    }

    if SYS!(c"ip netns exec %s sysctl -wq net.ipv4.ip_forward=1".as_ptr(), ns2) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip netns exec %s sysctl -wq net.ipv6.conf.all.forwarding=1".as_ptr(), ns2) != 0 { close_netns(nstoken); return -1; }

    /* Top route */
    if SYS!(c"ip    route add %s/32  dev veth2 %s".as_ptr(), cstr(IP4_ADDR_1), vrf) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip    route add %s/32  dev veth3 %s".as_ptr(), cstr(IP4_ADDR_4), vrf) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip -6 route add %s/128 dev veth2 %s".as_ptr(), cstr(IP6_ADDR_1), vrf) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip -6 route add %s/128 dev veth3 %s".as_ptr(), cstr(IP6_ADDR_4), vrf) != 0 { close_netns(nstoken); return -1; }
    /* Bottom route */
    if SYS!(c"ip    route add %s/32  dev veth6 %s".as_ptr(), cstr(IP4_ADDR_5), vrf) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip    route add %s/32  dev veth7 %s".as_ptr(), cstr(IP4_ADDR_8), vrf) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip -6 route add %s/128 dev veth6 %s".as_ptr(), cstr(IP6_ADDR_5), vrf) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip -6 route add %s/128 dev veth7 %s".as_ptr(), cstr(IP6_ADDR_8), vrf) != 0 { close_netns(nstoken); return -1; }

    close_netns(nstoken);
    0
}

unsafe fn configure_ns3(ns3: *const c_char) -> c_int {
    let mut nstoken: *mut nstoken = core::ptr::null_mut();

    if ns3.is_null() {
        return -1;
    }

    nstoken = open_netns(ns3);
    if !ASSERT_OK_PTR!(nstoken, c"open ns3".as_ptr()) {
        return -1;
    }

    /* Top route */
    if SYS!(c"ip    route add %s/32  dev veth4".as_ptr(), cstr(IP4_ADDR_3)) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip    route add %s/32  dev veth4 via  %s".as_ptr(), cstr(IP4_ADDR_1), cstr(IP4_ADDR_3)) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip    route add %s/32  dev veth4 via  %s".as_ptr(), cstr(IP4_ADDR_2), cstr(IP4_ADDR_3)) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip -6 route add %s/128 dev veth4".as_ptr(), cstr(IP6_ADDR_3)) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip -6 route add %s/128 dev veth4 via  %s".as_ptr(), cstr(IP6_ADDR_1), cstr(IP6_ADDR_3)) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip -6 route add %s/128 dev veth4 via  %s".as_ptr(), cstr(IP6_ADDR_2), cstr(IP6_ADDR_3)) != 0 { close_netns(nstoken); return -1; }
    /* Bottom route */
    if SYS!(c"ip    route add %s/32  dev veth8".as_ptr(), cstr(IP4_ADDR_7)) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip    route add %s/32  dev veth8 via  %s".as_ptr(), cstr(IP4_ADDR_5), cstr(IP4_ADDR_7)) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip    route add %s/32  dev veth8 via  %s".as_ptr(), cstr(IP4_ADDR_6), cstr(IP4_ADDR_7)) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip -6 route add %s/128 dev veth8".as_ptr(), cstr(IP6_ADDR_7)) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip -6 route add %s/128 dev veth8 via  %s".as_ptr(), cstr(IP6_ADDR_5), cstr(IP6_ADDR_7)) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip -6 route add %s/128 dev veth8 via  %s".as_ptr(), cstr(IP6_ADDR_6), cstr(IP6_ADDR_7)) != 0 { close_netns(nstoken); return -1; }

    /* Configure IPv4 GRE device */
    if SYS!(c"ip tunnel add gre_dev mode gre remote %s local %s ttl 255".as_ptr(), cstr(IP4_ADDR_1), cstr(IP4_ADDR_GRE)) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip link set gre_dev up".as_ptr()) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip a add %s dev gre_dev".as_ptr(), cstr(IP4_ADDR_GRE)) != 0 { close_netns(nstoken); return -1; }

    /* Configure IPv6 GRE device */
    if SYS!(c"ip tunnel add gre6_dev mode ip6gre remote %s local %s ttl 255".as_ptr(), cstr(IP6_ADDR_1), cstr(IP6_ADDR_GRE)) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip link set gre6_dev up".as_ptr()) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip a add %s dev gre6_dev".as_ptr(), cstr(IP6_ADDR_GRE)) != 0 { close_netns(nstoken); return -1; }

    close_netns(nstoken);
    0
}

unsafe fn setup_network(ns1: *mut c_char, ns2: *mut c_char, ns3: *mut c_char, vrf: *const c_char) -> c_int {
    if ns1.is_null() || ns2.is_null() || ns3.is_null() || vrf.is_null() {
        return -1;
    }

    if SYS!(c"ip -n %s link add veth1 type veth peer name veth2 netns %s".as_ptr(), ns1, ns2) != 0 { return -1; }
    if SYS!(c"ip -n %s link add veth3 type veth peer name veth4 netns %s".as_ptr(), ns2, ns3) != 0 { return -1; }
    if SYS!(c"ip -n %s link add veth5 type veth peer name veth6 netns %s".as_ptr(), ns1, ns2) != 0 { return -1; }
    if SYS!(c"ip -n %s link add veth7 type veth peer name veth8 netns %s".as_ptr(), ns2, ns3) != 0 { return -1; }

    if *vrf != 0 {
        if !ASSERT_OK!(configure_vrf(ns1, ns2), c"configure vrf".as_ptr()) {
            return -1;
        }
    }
    if !ASSERT_OK!(set_top_addr(ns1, ns2, ns3), c"set top addresses".as_ptr()) {
        return -1;
    }

    if !ASSERT_OK!(set_bottom_addr(ns1, ns2, ns3), c"set bottom addresses".as_ptr()) {
        return -1;
    }

    if !ASSERT_OK!(configure_ns1(ns1, vrf), c"configure ns1 routes".as_ptr()) {
        return -1;
    }

    if !ASSERT_OK!(configure_ns2(ns2, vrf), c"configure ns2 routes".as_ptr()) {
        return -1;
    }

    if !ASSERT_OK!(configure_ns3(ns3), c"configure ns3 routes".as_ptr()) {
        return -1;
    }

    /* Link bottom route to the GRE tunnels */
    if SYS!(c"ip -n %s route add %s/32 dev veth5 via %s %s".as_ptr(), ns1, cstr(IP4_ADDR_GRE), cstr(IP4_ADDR_6), vrf) != 0 { return -1; }
    if SYS!(c"ip -n %s route add %s/32 dev veth7 via %s %s".as_ptr(), ns2, cstr(IP4_ADDR_GRE), cstr(IP4_ADDR_8), vrf) != 0 { return -1; }
    if SYS!(c"ip -n %s -6 route add %s/128 dev veth5 via %s %s".as_ptr(), ns1, cstr(IP6_ADDR_GRE), cstr(IP6_ADDR_6), vrf) != 0 { return -1; }
    if SYS!(c"ip -n %s -6 route add %s/128 dev veth7 via %s %s".as_ptr(), ns2, cstr(IP6_ADDR_GRE), cstr(IP6_ADDR_8), vrf) != 0 { return -1; }

    0
}

unsafe fn remove_routes_to_gredev(ns1: *const c_char, ns2: *const c_char, vrf: *const c_char) -> c_int {
    if SYS!(c"ip -n %s route del %s dev veth5 %s".as_ptr(), ns1, cstr(IP4_ADDR_GRE), vrf) != 0 { return -1; }
    if SYS!(c"ip -n %s route del %s dev veth7 %s".as_ptr(), ns2, cstr(IP4_ADDR_GRE), vrf) != 0 { return -1; }
    if SYS!(c"ip -n %s -6 route del %s/128 dev veth5 %s".as_ptr(), ns1, cstr(IP6_ADDR_GRE), vrf) != 0 { return -1; }
    if SYS!(c"ip -n %s -6 route del %s/128 dev veth7 %s".as_ptr(), ns2, cstr(IP6_ADDR_GRE), vrf) != 0 { return -1; }

    0
}

unsafe fn add_unreachable_routes_to_gredev(ns1: *const c_char, ns2: *const c_char, vrf: *const c_char) -> c_int {
    if SYS!(c"ip -n %s route add unreachable %s/32 %s".as_ptr(), ns1, cstr(IP4_ADDR_GRE), vrf) != 0 { return -1; }
    if SYS!(c"ip -n %s route add unreachable %s/32 %s".as_ptr(), ns2, cstr(IP4_ADDR_GRE), vrf) != 0 { return -1; }
    if SYS!(c"ip -n %s -6 route add unreachable %s/128 %s".as_ptr(), ns1, cstr(IP6_ADDR_GRE), vrf) != 0 { return -1; }
    if SYS!(c"ip -n %s -6 route add unreachable %s/128 %s".as_ptr(), ns2, cstr(IP6_ADDR_GRE), vrf) != 0 { return -1; }

    0
}

/* This tests the fix from commit ea0371f78799 ("net: fix GSO in bpf_lwt_push_ip_encap") */
unsafe fn test_gso_fix(ns1: *const c_char, ns3: *const c_char, family: c_int) -> c_int {
    let ip_addr = if family == AF_INET { cstr(IP4_ADDR_DST) } else { cstr(IP6_ADDR_DST) };
    let mut gso_packet = [0 as c_char; GSO_SIZE];
    let mut nstoken: *mut nstoken = core::ptr::null_mut();
    let (mut sfd, mut cfd, mut afd): (c_int, c_int, c_int);
    let mut bytes: isize;
    let mut ret: c_int = -1;

    if ns1.is_null() || ns3.is_null() {
        return ret;
    }

    nstoken = open_netns(ns3);
    if !ASSERT_OK_PTR!(nstoken, c"open ns3".as_ptr()) {
        return ret;
    }

    sfd = start_server_str(family, SOCK_STREAM, ip_addr, GSO_TCP_PORT, core::ptr::null_mut());
    if !ASSERT_OK_FD!(sfd, c"start server".as_ptr()) {
        close_netns(nstoken);
        return ret;
    }

    close_netns(nstoken);

    nstoken = open_netns(ns1);
    if !ASSERT_OK_PTR!(nstoken, c"open ns1".as_ptr()) {
        close(sfd);
        close_netns(nstoken);
        return ret;
    }

    cfd = connect_to_addr_str(family, SOCK_STREAM, ip_addr, GSO_TCP_PORT, core::ptr::null_mut());
    if !ASSERT_OK_FD!(cfd, c"connect to server".as_ptr()) {
        close(sfd);
        close_netns(nstoken);
        return ret;
    }

    close_netns(nstoken);
    nstoken = core::ptr::null_mut();

    afd = accept(sfd, core::ptr::null_mut(), core::ptr::null_mut());
    if !ASSERT_OK_FD!(afd, c"accept".as_ptr()) {
        close(cfd);
        close(sfd);
        close_netns(nstoken);
        return ret;
    }

    /* Send a packet larger than MTU */
    bytes = send(cfd, gso_packet.as_ptr() as *const c_void, GSO_SIZE, 0);
    if !ASSERT_EQ!(bytes, GSO_SIZE as isize, c"send packet".as_ptr()) {
        close(afd);
        close(cfd);
        close(sfd);
        close_netns(nstoken);
        return ret;
    }

    /* Verify we received all expected bytes */
    bytes = read(afd, gso_packet.as_mut_ptr() as *mut c_void, GSO_SIZE);
    if !ASSERT_EQ!(bytes, GSO_SIZE as isize, c"receive packet".as_ptr()) {
        close(afd);
        close(cfd);
        close(sfd);
        close_netns(nstoken);
        return ret;
    }

    ret = 0;

    close(afd);
    close(cfd);
    close(sfd);
    close_netns(nstoken);

    ret
}

unsafe fn check_ping_ok(ns1: *const c_char) -> c_int {
    if SYS!(c"ip netns exec %s ping -c 1 -W1 -I veth1 %s > /dev/null".as_ptr(), ns1, cstr(IP4_ADDR_DST)) != 0 {
        return -1;
    }
    if SYS!(c"ip netns exec %s %s -c 1 -W1 -I veth1 %s > /dev/null".as_ptr(), ns1, ping_command(AF_INET6), cstr(IP6_ADDR_DST)) != 0 {
        return -1;
    }
    0
}

unsafe fn check_ping_fails(ns1: *const c_char) -> c_int {
    let mut ret: c_int;

    ret = SYS_NOFAIL!(c"ip netns exec %s ping -c 1 -W1 -I veth1 %s".as_ptr(), ns1, cstr(IP4_ADDR_DST));
    if ret == 0 {
        return -1;
    }

    ret = SYS_NOFAIL!(c"ip netns exec %s %s -c 1 -W1 -I veth1 %s".as_ptr(), ns1, ping_command(AF_INET6), cstr(IP6_ADDR_DST));
    if ret == 0 {
        return -1;
    }

    0
}

unsafe fn lwt_ip_encap(ipv4_encap: bool, egress: bool, vrf: *const c_char) {
    let mut ns1 = init_ns_name(b"-1-");
    let mut ns2 = init_ns_name(b"-2-");
    let mut ns3 = init_ns_name(b"-3-");
    let sec = if ipv4_encap { c"encap_gre".as_ptr() } else { c"encap_gre6".as_ptr() };

    if vrf.is_null() {
        return;
    }

    if !ASSERT_OK!(create_ns(ns1.as_mut_ptr(), NETNS_NAME_SIZE), c"create ns1".as_ptr()) {
        goto_lwt_ip_encap_out(&mut ns1, &mut ns2, &mut ns3);
        return;
    }
    if !ASSERT_OK!(create_ns(ns2.as_mut_ptr(), NETNS_NAME_SIZE), c"create ns2".as_ptr()) {
        goto_lwt_ip_encap_out(&mut ns1, &mut ns2, &mut ns3);
        return;
    }
    if !ASSERT_OK!(create_ns(ns3.as_mut_ptr(), NETNS_NAME_SIZE), c"create ns3".as_ptr()) {
        goto_lwt_ip_encap_out(&mut ns1, &mut ns2, &mut ns3);
        return;
    }

    if !ASSERT_OK!(setup_network(ns1.as_mut_ptr(), ns2.as_mut_ptr(), ns3.as_mut_ptr(), vrf), c"setup network".as_ptr()) {
        goto_lwt_ip_encap_out(&mut ns1, &mut ns2, &mut ns3);
        return;
    }

    /* By default, pings work */
    if !ASSERT_OK!(check_ping_ok(ns1.as_ptr()), c"ping OK".as_ptr()) {
        goto_lwt_ip_encap_out(&mut ns1, &mut ns2, &mut ns3);
        return;
    }

    /* Remove NS2->DST routes, ping fails */
    if SYS!(c"ip -n %s    route del %s/32  dev veth3 %s".as_ptr(), ns2.as_ptr(), cstr(IP4_ADDR_DST), vrf) != 0 {
        goto_lwt_ip_encap_out(&mut ns1, &mut ns2, &mut ns3);
        return;
    }
    if SYS!(c"ip -n %s -6 route del %s/128 dev veth3 %s".as_ptr(), ns2.as_ptr(), cstr(IP6_ADDR_DST), vrf) != 0 {
        goto_lwt_ip_encap_out(&mut ns1, &mut ns2, &mut ns3);
        return;
    }
    if !ASSERT_OK!(check_ping_fails(ns1.as_ptr()), c"ping expected fail".as_ptr()) {
        goto_lwt_ip_encap_out(&mut ns1, &mut ns2, &mut ns3);
        return;
    }

    /* Install replacement routes (LWT/eBPF), pings succeed */
    if egress {
        if SYS!(c"ip -n %s route add %s encap bpf xmit obj %s sec %s dev veth1 %s".as_ptr(), ns1.as_ptr(), cstr(IP4_ADDR_DST), cstr(BPF_FILE), sec, vrf) != 0 {
            goto_lwt_ip_encap_out(&mut ns1, &mut ns2, &mut ns3);
            return;
        }
        if SYS!(c"ip -n %s -6 route add %s encap bpf xmit obj %s sec %s dev veth1 %s".as_ptr(), ns1.as_ptr(), cstr(IP6_ADDR_DST), cstr(BPF_FILE), sec, vrf) != 0 {
            goto_lwt_ip_encap_out(&mut ns1, &mut ns2, &mut ns3);
            return;
        }
    } else {
        if SYS!(c"ip -n %s route add %s encap bpf in obj %s sec %s dev veth2 %s".as_ptr(), ns2.as_ptr(), cstr(IP4_ADDR_DST), cstr(BPF_FILE), sec, vrf) != 0 {
            goto_lwt_ip_encap_out(&mut ns1, &mut ns2, &mut ns3);
            return;
        }
        if SYS!(c"ip -n %s -6 route add %s encap bpf in obj %s sec %s dev veth2 %s".as_ptr(), ns2.as_ptr(), cstr(IP6_ADDR_DST), cstr(BPF_FILE), sec, vrf) != 0 {
            goto_lwt_ip_encap_out(&mut ns1, &mut ns2, &mut ns3);
            return;
        }
    }

    if !ASSERT_OK!(check_ping_ok(ns1.as_ptr()), c"ping OK".as_ptr()) {
        goto_lwt_ip_encap_out(&mut ns1, &mut ns2, &mut ns3);
        return;
    }

    /* Skip GSO tests with VRF: VRF routing needs properly assigned
     * source IP/device, which is easy to do with ping but hard with TCP.
     */
    if egress && *vrf == 0 {
        if !ASSERT_OK!(test_gso_fix(ns1.as_ptr(), ns3.as_ptr(), AF_INET), c"test GSO".as_ptr()) {
            goto_lwt_ip_encap_out(&mut ns1, &mut ns2, &mut ns3);
            return;
        }
    }

    /* Negative test: remove routes to GRE devices: ping fails */
    if !ASSERT_OK!(remove_routes_to_gredev(ns1.as_ptr(), ns2.as_ptr(), vrf), c"remove routes to gredev".as_ptr()) {
        goto_lwt_ip_encap_out(&mut ns1, &mut ns2, &mut ns3);
        return;
    }
    if !ASSERT_OK!(check_ping_fails(ns1.as_ptr()), c"ping expected fail".as_ptr()) {
        goto_lwt_ip_encap_out(&mut ns1, &mut ns2, &mut ns3);
        return;
    }

    /* Another negative test */
    if !ASSERT_OK!(add_unreachable_routes_to_gredev(ns1.as_ptr(), ns2.as_ptr(), vrf), c"add unreachable routes".as_ptr()) {
        goto_lwt_ip_encap_out(&mut ns1, &mut ns2, &mut ns3);
        return;
    }
    ASSERT_OK!(check_ping_fails(ns1.as_ptr()), c"ping expected fail".as_ptr());

    goto_lwt_ip_encap_out(&mut ns1, &mut ns2, &mut ns3);
}

unsafe fn goto_lwt_ip_encap_out(ns1: &mut [c_char; NETNS_NAME_SIZE], ns2: &mut [c_char; NETNS_NAME_SIZE], ns3: &mut [c_char; NETNS_NAME_SIZE]) {
    SYS_NOFAIL!(c"ip netns del %s".as_ptr(), ns1.as_ptr());
    SYS_NOFAIL!(c"ip netns del %s".as_ptr(), ns2.as_ptr());
    SYS_NOFAIL!(c"ip netns del %s".as_ptr(), ns3.as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn test_lwt_ip_encap_vrf_ipv6() {
    if test__start_subtest(c"egress".as_ptr()) {
        lwt_ip_encap(IPV6_ENCAP, EGRESS, c"vrf red".as_ptr());
    }

    if test__start_subtest(c"ingress".as_ptr()) {
        lwt_ip_encap(IPV6_ENCAP, INGRESS, c"vrf red".as_ptr());
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_lwt_ip_encap_vrf_ipv4() {
    if test__start_subtest(c"egress".as_ptr()) {
        lwt_ip_encap(IPV4_ENCAP, EGRESS, c"vrf red".as_ptr());
    }

    if test__start_subtest(c"ingress".as_ptr()) {
        lwt_ip_encap(IPV4_ENCAP, INGRESS, c"vrf red".as_ptr());
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_lwt_ip_encap_ipv6() {
    if test__start_subtest(c"egress".as_ptr()) {
        lwt_ip_encap(IPV6_ENCAP, EGRESS, c"".as_ptr());
    }

    if test__start_subtest(c"ingress".as_ptr()) {
        lwt_ip_encap(IPV6_ENCAP, INGRESS, c"".as_ptr());
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_lwt_ip_encap_ipv4() {
    if test__start_subtest(c"egress".as_ptr()) {
        lwt_ip_encap(IPV4_ENCAP, EGRESS, c"".as_ptr());
    }

    if test__start_subtest(c"ingress".as_ptr()) {
        lwt_ip_encap(IPV4_ENCAP, INGRESS, c"".as_ptr());
    }
}

/*
 * VxLAN Setup/topology:
 *
 * NS1 (IP*_ADDR_1)                NS2                  NS3 (IP*_ADDR_4)
 *       [ping src]
 *           |                          top route
 *         veth1 (LWT encap)  <<-- veth2        veth3  <<-- veth4 (ping dst)
 *           |                                                ^
 *       (bottom route)                                       | (inner pkt)
 *           v                        bottom route            |
 *         veth5              -->> veth6        veth7  -->> veth8 (vxlan decap)
 *                                                          (IP*_ADDR_VXLAN)
 *
 * Add the VxLAN endpoint addresses to NS3's veth8, create standard
 * VxLAN decap devices bound to those addresses, and install routes so
 * NS1/NS2 can reach the endpoints via the bottom route.  NS2 here is to
 * make sure the LWT-encap VxLAN packets are routed to NS3 correctly.
 */
unsafe fn setup_vxlan_routes(ns3: *const c_char, ns1: *const c_char, ns2: *const c_char) -> c_int {
    let nstoken: *mut nstoken;

    nstoken = open_netns(ns3);
    if !ASSERT_OK_PTR!(nstoken, c"open ns3 for vxlan".as_ptr()) {
        return -1;
    }

    if SYS!(c"ip    a add %s/32  dev veth8".as_ptr(), cstr(IP4_ADDR_VXLAN)) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip -6 a add %s/128 dev veth8".as_ptr(), cstr(IP6_ADDR_VXLAN)) != 0 { close_netns(nstoken); return -1; }
    /*
     * Standard VxLAN devices to decap the encapsulated packets.  The inner
     * Ethernet frame uses a broadcast dst MAC so the IP stack accepts it
     * without ARP or FDB configuration.
     */
    if SYS!(c"ip link add vxlan4 type vxlan id 1 dstport 4789 local %s dev veth8 nolearning noudpcsum".as_ptr(), cstr(IP4_ADDR_VXLAN)) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip link set vxlan4 up".as_ptr()) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip link add vxlan6 type vxlan id 1 dstport 4789 local %s dev veth8 nolearning udp6zerocsumrx".as_ptr(), cstr(IP6_ADDR_VXLAN)) != 0 { close_netns(nstoken); return -1; }
    if SYS!(c"ip link set vxlan6 up".as_ptr()) != 0 { close_netns(nstoken); return -1; }
    close_netns(nstoken);

    if SYS!(c"ip -n %s    route add %s/32  dev veth5 via %s".as_ptr(), ns1, cstr(IP4_ADDR_VXLAN), cstr(IP4_ADDR_6)) != 0 { return -1; }
    if SYS!(c"ip -n %s    route add %s/32  dev veth7 via %s".as_ptr(), ns2, cstr(IP4_ADDR_VXLAN), cstr(IP4_ADDR_8)) != 0 { return -1; }
    if SYS!(c"ip -n %s -6 route add %s/128 dev veth5 via %s".as_ptr(), ns1, cstr(IP6_ADDR_VXLAN), cstr(IP6_ADDR_6)) != 0 { return -1; }
    if SYS!(c"ip -n %s -6 route add %s/128 dev veth7 via %s".as_ptr(), ns2, cstr(IP6_ADDR_VXLAN), cstr(IP6_ADDR_8)) != 0 { return -1; }
    0
}

unsafe fn lwt_ip_encap_vxlan(ipv4_encap: bool) {
    let mut ns1 = init_ns_name(b"-1-");
    let mut ns2 = init_ns_name(b"-2-");
    let mut ns3 = init_ns_name(b"-3-");
    let sec = if ipv4_encap { c"encap_vxlan".as_ptr() } else { c"encap_vxlan6".as_ptr() };
    let expected_offset: c_int = if ipv4_encap {
        core::mem::size_of::<iphdr>() as c_int
    } else {
        core::mem::size_of::<ipv6hdr>() as c_int
    };
    let mut skel: *mut test_lwt_ip_encap = core::ptr::null_mut();
    let mut thdr_offset: c_int;
    let mut err: c_int;

    if !ASSERT_OK!(create_ns(ns1.as_mut_ptr(), NETNS_NAME_SIZE), c"create ns1".as_ptr()) {
        goto_lwt_ip_encap_vxlan_out(skel, &mut ns1, &mut ns2, &mut ns3);
        return;
    }
    if !ASSERT_OK!(create_ns(ns2.as_mut_ptr(), NETNS_NAME_SIZE), c"create ns2".as_ptr()) {
        goto_lwt_ip_encap_vxlan_out(skel, &mut ns1, &mut ns2, &mut ns3);
        return;
    }
    if !ASSERT_OK!(create_ns(ns3.as_mut_ptr(), NETNS_NAME_SIZE), c"create ns3".as_ptr()) {
        goto_lwt_ip_encap_vxlan_out(skel, &mut ns1, &mut ns2, &mut ns3);
        return;
    }

    if !ASSERT_OK!(setup_network(ns1.as_mut_ptr(), ns2.as_mut_ptr(), ns3.as_mut_ptr(), c"".as_ptr()), c"setup network".as_ptr()) {
        goto_lwt_ip_encap_vxlan_out(skel, &mut ns1, &mut ns2, &mut ns3);
        return;
    }

    if !ASSERT_OK!(setup_vxlan_routes(ns3.as_ptr(), ns1.as_ptr(), ns2.as_ptr()), c"setup vxlan routes".as_ptr()) {
        goto_lwt_ip_encap_vxlan_out(skel, &mut ns1, &mut ns2, &mut ns3);
        return;
    }

    skel = test_lwt_ip_encap__open();
    if !ASSERT_OK_PTR!(skel, c"test_lwt_ip_encap__open".as_ptr()) {
        goto_lwt_ip_encap_vxlan_out(skel, &mut ns1, &mut ns2, &mut ns3);
        return;
    }

    bpf_program__set_autoload((*skel).progs.bpf_lwt_encap_gre, false);
    bpf_program__set_autoload((*skel).progs.bpf_lwt_encap_gre6, false);
    bpf_program__set_autoload((*skel).progs.bpf_lwt_encap_vxlan, false);
    bpf_program__set_autoload((*skel).progs.bpf_lwt_encap_vxlan6, false);
    bpf_program__set_autoload((*skel).progs.fexit_lwt_push_ip_encap, true);
    (*(*skel).rodata).tgt_ip_version = if ipv4_encap { 4 } else { 6 };

    err = test_lwt_ip_encap__load(skel);
    if !ASSERT_OK!(err, c"test_lwt_ip_encap__load".as_ptr()) {
        goto_lwt_ip_encap_vxlan_out(skel, &mut ns1, &mut ns2, &mut ns3);
        return;
    }

    err = test_lwt_ip_encap__attach(skel);
    if !ASSERT_OK!(err, c"test_lwt_ip_encap__attach".as_ptr()) {
        goto_lwt_ip_encap_vxlan_out(skel, &mut ns1, &mut ns2, &mut ns3);
        return;
    }

    /* Remove the direct NS2->DST route so packets must go via LWT encap. */
    if SYS!(c"ip -n %s    route del %s/32  dev veth3".as_ptr(), ns2.as_ptr(), cstr(IP4_ADDR_DST)) != 0 {
        goto_lwt_ip_encap_vxlan_out(skel, &mut ns1, &mut ns2, &mut ns3);
        return;
    }
    if SYS!(c"ip -n %s -6 route del %s/128 dev veth3".as_ptr(), ns2.as_ptr(), cstr(IP6_ADDR_DST)) != 0 {
        goto_lwt_ip_encap_vxlan_out(skel, &mut ns1, &mut ns2, &mut ns3);
        return;
    }

    if ipv4_encap {
        if SYS!(c"ip -n %s route add %s encap bpf xmit obj %s sec %s dev veth1".as_ptr(), ns1.as_ptr(), cstr(IP4_ADDR_DST), cstr(BPF_FILE), sec) != 0 {
            goto_lwt_ip_encap_vxlan_out(skel, &mut ns1, &mut ns2, &mut ns3);
            return;
        }
    } else if SYS!(c"ip -n %s -6 route add %s encap bpf xmit obj %s sec %s dev veth1".as_ptr(), ns1.as_ptr(), cstr(IP6_ADDR_DST), cstr(BPF_FILE), sec) != 0 {
        goto_lwt_ip_encap_vxlan_out(skel, &mut ns1, &mut ns2, &mut ns3);
        return;
    }

    (*(*skel).bss).fexit_triggered = false;

    if ipv4_encap {
        if SYS!(c"ip netns exec %s ping -c 1 -W1 %s".as_ptr(), ns1.as_ptr(), cstr(IP4_ADDR_DST)) != 0 {
            goto_lwt_ip_encap_vxlan_out(skel, &mut ns1, &mut ns2, &mut ns3);
            return;
        }
    } else if SYS!(c"ip netns exec %s %s -c 1 -W1 %s".as_ptr(), ns1.as_ptr(), ping_command(AF_INET6), cstr(IP6_ADDR_DST)) != 0 {
        goto_lwt_ip_encap_vxlan_out(skel, &mut ns1, &mut ns2, &mut ns3);
        return;
    }

    if !ASSERT_TRUE!((*(*skel).bss).fexit_triggered, c"fexit_triggered".as_ptr()) {
        goto_lwt_ip_encap_vxlan_out(skel, &mut ns1, &mut ns2, &mut ns3);
        return;
    }

    thdr_offset = (*(*skel).bss).transport_hdr as c_int - (*(*skel).bss).network_hdr as c_int;
    ASSERT_EQ!(thdr_offset, expected_offset, c"transport_hdr offset".as_ptr());

    goto_lwt_ip_encap_vxlan_out(skel, &mut ns1, &mut ns2, &mut ns3);
}

unsafe fn goto_lwt_ip_encap_vxlan_out(
    skel: *mut test_lwt_ip_encap,
    ns1: &mut [c_char; NETNS_NAME_SIZE],
    ns2: &mut [c_char; NETNS_NAME_SIZE],
    ns3: &mut [c_char; NETNS_NAME_SIZE],
) {
    test_lwt_ip_encap__destroy(skel);
    SYS_NOFAIL!(c"ip netns del %s".as_ptr(), ns1.as_ptr());
    SYS_NOFAIL!(c"ip netns del %s".as_ptr(), ns2.as_ptr());
    SYS_NOFAIL!(c"ip netns del %s".as_ptr(), ns3.as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn test_lwt_ip_encap_vxlan_ipv4() {
    lwt_ip_encap_vxlan(IPV4_ENCAP);
}

#[no_mangle]
pub unsafe extern "C" fn test_lwt_ip_encap_vxlan_ipv6() {
    lwt_ip_encap_vxlan(IPV6_ENCAP);
}
