// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/bpf/prog_tests/flow_dissector.c.
// C dependencies: test_progs.h, network_helpers.h, linux/if_tun.h,
// sys/uio.h, and bpf_flow.skel.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(static_mut_refs)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __be16 = u16;
type __be32 = u32;
type size_t = usize;

const TEST_NS: &[u8] = b"flow_dissector_ns\0";
const FLOW_CONTINUE_SADDR: __u32 = 0x7f00007f; /* 127.0.0.127 */
const TEST_NAME_MAX_LEN: usize = 64;

const IP_MF: __u16 = 0x2000;
const VLAN_HLEN: usize = 4;

unsafe extern "C" {
    static mut errno: c_int;

    static ETH_P_IP: c_int;
    static ETH_P_IPV6: c_int;
    static ETH_P_8021Q: c_int;
    static ETH_P_8021AD: c_int;
    static ETH_HLEN: c_int;
    static IPPROTO_TCP: c_int;
    static IPPROTO_FRAGMENT: c_int;
    static IPPROTO_IPIP: c_int;
    static IPPROTO_GRE: c_int;
    static MAGIC_BYTES: c_int;
    static BPF_OK: c_int;
    static BPF_FLOW_DISSECTOR_CONTINUE: c_int;
    static BPF_FLOW_DISSECTOR: c_int;
    static BPF_FLOW_DISSECTOR_F_PARSE_1ST_FRAG: __u32;
    static BPF_FLOW_DISSECTOR_F_STOP_AT_FLOW_LABEL: __u32;
    static BPF_FLOW_DISSECTOR_F_STOP_AT_ENCAP: __u32;
    static BPF_ANY: u64;
    static EEXIST: c_int;
    static O_RDWR: c_int;
    static O_RDONLY: c_int;
    static TUNSETIFF: c_uint;
    static IFF_TAP: c_short;
    static IFF_NO_PI: c_short;
    static IFF_NAPI: c_short;
    static IFF_NAPI_FRAGS: c_short;
    static IFF_UP: c_short;
    static PF_INET: c_int;
    static SOCK_DGRAM: c_int;
    static SIOCGIFFLAGS: c_uint;
    static SIOCSIFFLAGS: c_uint;

    fn __bpf_constant_htons(x: c_int) -> __be16;
    fn __bpf_constant_htonl(x: __u32) -> __be32;
    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(a: c_int, b: c_int, name: *const c_char) -> bool;
    fn ASSERT_MEMEQ(a: *const c_void, b: *const c_void, len: size_t, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;

    fn bpf_flow__open_and_load() -> *mut bpf_flow;
    fn bpf_flow__destroy(skel: *mut bpf_flow);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_attach(prog_fd: c_int, target_fd: c_int, typ: c_int, flags: c_uint) -> c_int;
    fn bpf_prog_detach2(prog_fd: c_int, target_fd: c_int, typ: c_int) -> c_int;
    fn make_netns(name: *const c_char) -> c_int;
    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(ns: *mut nstoken);
    fn remove_netns(name: *const c_char);
    fn netns_new(name: *const c_char, open: bool) -> *mut netns_obj;
    fn netns_free(ns: *mut netns_obj);
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map__max_entries(map: *mut bpf_map) -> c_int;
    fn bpf_object__find_program_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_program;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    fn bpf_program__attach_netns(prog: *mut bpf_program, net_fd: c_int) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn open(path: *const c_char, flags: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_uint, ...) -> c_int;
    fn writev(fd: c_int, iov: *const iovec, iovcnt: c_int) -> isize;
    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
}

type c_short = i16;

#[repr(C)]
struct ethhdr {
    h_proto: __be16,
}

#[repr(C)]
struct iphdr {
    ihl: __u8,
    protocol: __u8,
    tot_len: __be16,
    frag_off: __be16,
    saddr: __be32,
}

#[repr(C)]
struct ipv6hdr {
    payload_len: __be16,
    nexthdr: __u8,
    flow_lbl: [__u8; 3],
}

#[repr(C)]
struct tcphdr {
    source: __be16,
    dest: __be16,
    doff: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct bpf_flow_keys {
    nhoff: __u16,
    thoff: __u16,
    addr_proto: __u16,
    ip_proto: __u8,
    n_proto: __be16,
    flags: __u32,
    is_frag: bool,
    is_first_frag: bool,
    is_encap: bool,
    sport: __be16,
    dport: __be16,
    flow_label: __be32,
}

#[repr(C, packed)]
struct ipv4_pkt {
    eth: ethhdr,
    iph: iphdr,
    tcp: tcphdr,
}

#[repr(C, packed)]
struct ipip_pkt {
    eth: ethhdr,
    iph: iphdr,
    iph_inner: iphdr,
    tcp: tcphdr,
}

#[repr(C, packed)]
struct svlan_ipv4_pkt {
    eth: ethhdr,
    vlan_tci: __u16,
    vlan_proto: __u16,
    iph: iphdr,
    tcp: tcphdr,
}

#[repr(C, packed)]
struct ipv6_pkt {
    eth: ethhdr,
    iph: ipv6hdr,
    tcp: tcphdr,
}

#[repr(C)]
struct frag_hdr {
    nexthdr: __u8,
    reserved: __u8,
    frag_off: __be16,
    identification: __be32,
}

#[repr(C, packed)]
struct ipv6_frag_pkt {
    eth: ethhdr,
    iph: ipv6hdr,
    ipf: frag_hdr,
    tcp: tcphdr,
}

#[repr(C, packed)]
struct dvlan_ipv6_pkt {
    eth: ethhdr,
    vlan_tci: __u16,
    vlan_proto: __u16,
    vlan_tci2: __u16,
    vlan_proto2: __u16,
    iph: ipv6hdr,
    tcp: tcphdr,
}

#[repr(C)]
struct gre_base_hdr {
    flags: __be16,
    protocol: __be16,
}

#[repr(C, packed)]
struct gre_minimal_pkt {
    eth: ethhdr,
    iph: iphdr,
    gre_hdr: gre_base_hdr,
    iph_inner: iphdr,
    tcp: tcphdr,
}

#[repr(C)]
union test_pkt {
    ipv4: core::mem::ManuallyDrop<ipv4_pkt>,
    svlan_ipv4: core::mem::ManuallyDrop<svlan_ipv4_pkt>,
    ipip: core::mem::ManuallyDrop<ipip_pkt>,
    ipv6: core::mem::ManuallyDrop<ipv6_pkt>,
    ipv6_frag: core::mem::ManuallyDrop<ipv6_frag_pkt>,
    dvlan_ipv6: core::mem::ManuallyDrop<dvlan_ipv6_pkt>,
    gre_minimal: core::mem::ManuallyDrop<gre_minimal_pkt>,
}

#[repr(C)]
struct test {
    name: *const c_char,
    pkt: test_pkt,
    keys: bpf_flow_keys,
    flags: __u32,
    retval: __u32,
}

#[repr(C)]
struct bpf_flow {
    obj: *mut bpf_object,
    progs: bpf_flow_progs,
    maps: bpf_flow_maps,
}

#[repr(C)]
struct bpf_flow_progs {
    _dissect: *mut bpf_program,
}

#[repr(C)]
struct bpf_flow_maps {
    jmp_table: *mut bpf_map,
    last_dissection: *mut bpf_map,
}

#[repr(C)]
struct bpf_object;
#[repr(C)]
struct bpf_program;
#[repr(C)]
struct bpf_map;
#[repr(C)]
struct bpf_link;
#[repr(C)]
struct nstoken;
#[repr(C)]
struct netns_obj;

#[repr(C)]
struct ifreq {
    ifr_name: [c_char; 16],
    ifr_flags: c_short,
}

#[repr(C)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: size_t,
}

#[repr(C)]
struct bpf_test_run_opts {
    sz: size_t,
    data_in: *mut c_void,
    data_size_in: __u32,
    data_out: *mut c_void,
    data_size_out: __u32,
    ctx_in: *mut c_void,
    ctx_size_in: __u32,
    retval: __u32,
}

unsafe fn tests() -> [test; 17] {
    [
        test {
            name: c"ipv4".as_ptr(),
            pkt: test_pkt { ipv4: core::mem::ManuallyDrop::new(ipv4_pkt {
                eth: ethhdr { h_proto: __bpf_constant_htons(ETH_P_IP) },
                iph: iphdr { ihl: 5, protocol: IPPROTO_TCP as __u8, tot_len: __bpf_constant_htons(MAGIC_BYTES), frag_off: 0, saddr: 0 },
                tcp: tcphdr { doff: 5, source: 80, dest: 8080 },
            }) },
            keys: bpf_flow_keys { nhoff: ETH_HLEN as __u16, thoff: (ETH_HLEN as usize + size_of::<iphdr>()) as __u16, addr_proto: ETH_P_IP as __u16, ip_proto: IPPROTO_TCP as __u8, n_proto: __bpf_constant_htons(ETH_P_IP), sport: 80, dport: 8080, ..zeroed() },
            flags: 0,
            retval: BPF_OK as __u32,
        },
        test {
            name: c"ipv6".as_ptr(),
            pkt: test_pkt { ipv6: core::mem::ManuallyDrop::new(ipv6_pkt {
                eth: ethhdr { h_proto: __bpf_constant_htons(ETH_P_IPV6) },
                iph: ipv6hdr { nexthdr: IPPROTO_TCP as __u8, payload_len: __bpf_constant_htons(MAGIC_BYTES), flow_lbl: [0; 3] },
                tcp: tcphdr { doff: 5, source: 80, dest: 8080 },
            }) },
            keys: bpf_flow_keys { nhoff: ETH_HLEN as __u16, thoff: (ETH_HLEN as usize + size_of::<ipv6hdr>()) as __u16, addr_proto: ETH_P_IPV6 as __u16, ip_proto: IPPROTO_TCP as __u8, n_proto: __bpf_constant_htons(ETH_P_IPV6), sport: 80, dport: 8080, ..zeroed() },
            flags: 0,
            retval: BPF_OK as __u32,
        },
        test {
            name: c"802.1q-ipv4".as_ptr(),
            pkt: test_pkt { svlan_ipv4: core::mem::ManuallyDrop::new(svlan_ipv4_pkt {
                eth: ethhdr { h_proto: __bpf_constant_htons(ETH_P_8021Q) },
                vlan_tci: 0,
                vlan_proto: __bpf_constant_htons(ETH_P_IP),
                iph: iphdr { ihl: 5, protocol: IPPROTO_TCP as __u8, tot_len: __bpf_constant_htons(MAGIC_BYTES), frag_off: 0, saddr: 0 },
                tcp: tcphdr { doff: 5, source: 80, dest: 8080 },
            }) },
            keys: bpf_flow_keys { nhoff: (ETH_HLEN as usize + VLAN_HLEN) as __u16, thoff: (ETH_HLEN as usize + VLAN_HLEN + size_of::<iphdr>()) as __u16, addr_proto: ETH_P_IP as __u16, ip_proto: IPPROTO_TCP as __u8, n_proto: __bpf_constant_htons(ETH_P_IP), sport: 80, dport: 8080, ..zeroed() },
            flags: 0,
            retval: BPF_OK as __u32,
        },
        test {
            name: c"802.1ad-ipv6".as_ptr(),
            pkt: test_pkt { dvlan_ipv6: core::mem::ManuallyDrop::new(dvlan_ipv6_pkt {
                eth: ethhdr { h_proto: __bpf_constant_htons(ETH_P_8021AD) },
                vlan_tci: 0,
                vlan_proto: __bpf_constant_htons(ETH_P_8021Q),
                vlan_tci2: 0,
                vlan_proto2: __bpf_constant_htons(ETH_P_IPV6),
                iph: ipv6hdr { nexthdr: IPPROTO_TCP as __u8, payload_len: __bpf_constant_htons(MAGIC_BYTES), flow_lbl: [0; 3] },
                tcp: tcphdr { doff: 5, source: 80, dest: 8080 },
            }) },
            keys: bpf_flow_keys { nhoff: (ETH_HLEN as usize + VLAN_HLEN * 2) as __u16, thoff: (ETH_HLEN as usize + VLAN_HLEN * 2 + size_of::<ipv6hdr>()) as __u16, addr_proto: ETH_P_IPV6 as __u16, ip_proto: IPPROTO_TCP as __u8, n_proto: __bpf_constant_htons(ETH_P_IPV6), sport: 80, dport: 8080, ..zeroed() },
            flags: 0,
            retval: BPF_OK as __u32,
        },
        test {
            name: c"ipv4-frag".as_ptr(),
            pkt: test_pkt { ipv4: core::mem::ManuallyDrop::new(ipv4_pkt {
                eth: ethhdr { h_proto: __bpf_constant_htons(ETH_P_IP) },
                iph: iphdr { ihl: 5, protocol: IPPROTO_TCP as __u8, tot_len: __bpf_constant_htons(MAGIC_BYTES), frag_off: __bpf_constant_htons(IP_MF as c_int), saddr: 0 },
                tcp: tcphdr { doff: 5, source: 80, dest: 8080 },
            }) },
            keys: bpf_flow_keys { flags: BPF_FLOW_DISSECTOR_F_PARSE_1ST_FRAG, nhoff: ETH_HLEN as __u16, thoff: (ETH_HLEN as usize + size_of::<iphdr>()) as __u16, addr_proto: ETH_P_IP as __u16, ip_proto: IPPROTO_TCP as __u8, n_proto: __bpf_constant_htons(ETH_P_IP), is_frag: true, is_first_frag: true, sport: 80, dport: 8080, ..zeroed() },
            flags: BPF_FLOW_DISSECTOR_F_PARSE_1ST_FRAG,
            retval: BPF_OK as __u32,
        },
        test {
            name: c"ipv4-no-frag".as_ptr(),
            pkt: test_pkt { ipv4: core::mem::ManuallyDrop::new(ipv4_pkt {
                eth: ethhdr { h_proto: __bpf_constant_htons(ETH_P_IP) },
                iph: iphdr { ihl: 5, protocol: IPPROTO_TCP as __u8, tot_len: __bpf_constant_htons(MAGIC_BYTES), frag_off: __bpf_constant_htons(IP_MF as c_int), saddr: 0 },
                tcp: tcphdr { doff: 5, source: 80, dest: 8080 },
            }) },
            keys: bpf_flow_keys { nhoff: ETH_HLEN as __u16, thoff: (ETH_HLEN as usize + size_of::<iphdr>()) as __u16, addr_proto: ETH_P_IP as __u16, ip_proto: IPPROTO_TCP as __u8, n_proto: __bpf_constant_htons(ETH_P_IP), is_frag: true, is_first_frag: true, ..zeroed() },
            flags: 0,
            retval: BPF_OK as __u32,
        },
        test {
            name: c"ipv6-frag".as_ptr(),
            pkt: test_pkt { ipv6_frag: core::mem::ManuallyDrop::new(ipv6_frag_pkt {
                eth: ethhdr { h_proto: __bpf_constant_htons(ETH_P_IPV6) },
                iph: ipv6hdr { nexthdr: IPPROTO_FRAGMENT as __u8, payload_len: __bpf_constant_htons(MAGIC_BYTES), flow_lbl: [0; 3] },
                ipf: frag_hdr { nexthdr: IPPROTO_TCP as __u8, reserved: 0, frag_off: 0, identification: 0 },
                tcp: tcphdr { doff: 5, source: 80, dest: 8080 },
            }) },
            keys: bpf_flow_keys { flags: BPF_FLOW_DISSECTOR_F_PARSE_1ST_FRAG, nhoff: ETH_HLEN as __u16, thoff: (ETH_HLEN as usize + size_of::<ipv6hdr>() + size_of::<frag_hdr>()) as __u16, addr_proto: ETH_P_IPV6 as __u16, ip_proto: IPPROTO_TCP as __u8, n_proto: __bpf_constant_htons(ETH_P_IPV6), is_frag: true, is_first_frag: true, sport: 80, dport: 8080, ..zeroed() },
            flags: BPF_FLOW_DISSECTOR_F_PARSE_1ST_FRAG,
            retval: BPF_OK as __u32,
        },
        test {
            name: c"ipv6-no-frag".as_ptr(),
            pkt: test_pkt { ipv6_frag: core::mem::ManuallyDrop::new(ipv6_frag_pkt {
                eth: ethhdr { h_proto: __bpf_constant_htons(ETH_P_IPV6) },
                iph: ipv6hdr { nexthdr: IPPROTO_FRAGMENT as __u8, payload_len: __bpf_constant_htons(MAGIC_BYTES), flow_lbl: [0; 3] },
                ipf: frag_hdr { nexthdr: IPPROTO_TCP as __u8, reserved: 0, frag_off: 0, identification: 0 },
                tcp: tcphdr { doff: 5, source: 80, dest: 8080 },
            }) },
            keys: bpf_flow_keys { nhoff: ETH_HLEN as __u16, thoff: (ETH_HLEN as usize + size_of::<ipv6hdr>() + size_of::<frag_hdr>()) as __u16, addr_proto: ETH_P_IPV6 as __u16, ip_proto: IPPROTO_TCP as __u8, n_proto: __bpf_constant_htons(ETH_P_IPV6), is_frag: true, is_first_frag: true, ..zeroed() },
            flags: 0,
            retval: BPF_OK as __u32,
        },
        test {
            name: c"ipv6-flow-label".as_ptr(),
            pkt: test_pkt { ipv6: core::mem::ManuallyDrop::new(ipv6_pkt {
                eth: ethhdr { h_proto: __bpf_constant_htons(ETH_P_IPV6) },
                iph: ipv6hdr { nexthdr: IPPROTO_TCP as __u8, payload_len: __bpf_constant_htons(MAGIC_BYTES), flow_lbl: [0xb, 0xee, 0xef] },
                tcp: tcphdr { doff: 5, source: 80, dest: 8080 },
            }) },
            keys: bpf_flow_keys { nhoff: ETH_HLEN as __u16, thoff: (ETH_HLEN as usize + size_of::<ipv6hdr>()) as __u16, addr_proto: ETH_P_IPV6 as __u16, ip_proto: IPPROTO_TCP as __u8, n_proto: __bpf_constant_htons(ETH_P_IPV6), sport: 80, dport: 8080, flow_label: __bpf_constant_htonl(0xbeeef), ..zeroed() },
            flags: 0,
            retval: BPF_OK as __u32,
        },
        test {
            name: c"ipv6-no-flow-label".as_ptr(),
            pkt: test_pkt { ipv6: core::mem::ManuallyDrop::new(ipv6_pkt {
                eth: ethhdr { h_proto: __bpf_constant_htons(ETH_P_IPV6) },
                iph: ipv6hdr { nexthdr: IPPROTO_TCP as __u8, payload_len: __bpf_constant_htons(MAGIC_BYTES), flow_lbl: [0xb, 0xee, 0xef] },
                tcp: tcphdr { doff: 5, source: 80, dest: 8080 },
            }) },
            keys: bpf_flow_keys { flags: BPF_FLOW_DISSECTOR_F_STOP_AT_FLOW_LABEL, nhoff: ETH_HLEN as __u16, thoff: (ETH_HLEN as usize + size_of::<ipv6hdr>()) as __u16, addr_proto: ETH_P_IPV6 as __u16, ip_proto: IPPROTO_TCP as __u8, n_proto: __bpf_constant_htons(ETH_P_IPV6), flow_label: __bpf_constant_htonl(0xbeeef), ..zeroed() },
            flags: BPF_FLOW_DISSECTOR_F_STOP_AT_FLOW_LABEL,
            retval: BPF_OK as __u32,
        },
        test {
            name: c"ipv6-empty-flow-label".as_ptr(),
            pkt: test_pkt { ipv6: core::mem::ManuallyDrop::new(ipv6_pkt {
                eth: ethhdr { h_proto: __bpf_constant_htons(ETH_P_IPV6) },
                iph: ipv6hdr { nexthdr: IPPROTO_TCP as __u8, payload_len: __bpf_constant_htons(MAGIC_BYTES), flow_lbl: [0, 0, 0] },
                tcp: tcphdr { doff: 5, source: 80, dest: 8080 },
            }) },
            keys: bpf_flow_keys { flags: BPF_FLOW_DISSECTOR_F_STOP_AT_FLOW_LABEL, nhoff: ETH_HLEN as __u16, thoff: (ETH_HLEN as usize + size_of::<ipv6hdr>()) as __u16, addr_proto: ETH_P_IPV6 as __u16, ip_proto: IPPROTO_TCP as __u8, n_proto: __bpf_constant_htons(ETH_P_IPV6), sport: 80, dport: 8080, ..zeroed() },
            flags: BPF_FLOW_DISSECTOR_F_STOP_AT_FLOW_LABEL,
            retval: BPF_OK as __u32,
        },
        test {
            name: c"ipip-encap".as_ptr(),
            pkt: test_pkt { ipip: core::mem::ManuallyDrop::new(ipip_pkt {
                eth: ethhdr { h_proto: __bpf_constant_htons(ETH_P_IP) },
                iph: iphdr { ihl: 5, protocol: IPPROTO_IPIP as __u8, tot_len: __bpf_constant_htons(MAGIC_BYTES), frag_off: 0, saddr: 0 },
                iph_inner: iphdr { ihl: 5, protocol: IPPROTO_TCP as __u8, tot_len: __bpf_constant_htons(MAGIC_BYTES - size_of::<iphdr>() as c_int), frag_off: 0, saddr: 0 },
                tcp: tcphdr { doff: 5, source: 80, dest: 8080 },
            }) },
            keys: bpf_flow_keys { nhoff: ETH_HLEN as __u16, thoff: (ETH_HLEN as usize + size_of::<iphdr>() + size_of::<iphdr>()) as __u16, addr_proto: ETH_P_IP as __u16, ip_proto: IPPROTO_TCP as __u8, n_proto: __bpf_constant_htons(ETH_P_IP), is_encap: true, sport: 80, dport: 8080, ..zeroed() },
            flags: 0,
            retval: BPF_OK as __u32,
        },
        test {
            name: c"ipip-no-encap".as_ptr(),
            pkt: test_pkt { ipip: core::mem::ManuallyDrop::new(ipip_pkt {
                eth: ethhdr { h_proto: __bpf_constant_htons(ETH_P_IP) },
                iph: iphdr { ihl: 5, protocol: IPPROTO_IPIP as __u8, tot_len: __bpf_constant_htons(MAGIC_BYTES), frag_off: 0, saddr: 0 },
                iph_inner: iphdr { ihl: 5, protocol: IPPROTO_TCP as __u8, tot_len: __bpf_constant_htons(MAGIC_BYTES - size_of::<iphdr>() as c_int), frag_off: 0, saddr: 0 },
                tcp: tcphdr { doff: 5, source: 80, dest: 8080 },
            }) },
            keys: bpf_flow_keys { flags: BPF_FLOW_DISSECTOR_F_STOP_AT_ENCAP, nhoff: ETH_HLEN as __u16, thoff: (ETH_HLEN as usize + size_of::<iphdr>()) as __u16, addr_proto: ETH_P_IP as __u16, ip_proto: IPPROTO_IPIP as __u8, n_proto: __bpf_constant_htons(ETH_P_IP), is_encap: true, ..zeroed() },
            flags: BPF_FLOW_DISSECTOR_F_STOP_AT_ENCAP,
            retval: BPF_OK as __u32,
        },
        test {
            name: c"ipip-encap-dissector-continue".as_ptr(),
            pkt: test_pkt { ipip: core::mem::ManuallyDrop::new(ipip_pkt {
                eth: ethhdr { h_proto: __bpf_constant_htons(ETH_P_IP) },
                iph: iphdr { ihl: 5, protocol: IPPROTO_IPIP as __u8, tot_len: __bpf_constant_htons(MAGIC_BYTES), frag_off: 0, saddr: __bpf_constant_htonl(FLOW_CONTINUE_SADDR) },
                iph_inner: iphdr { ihl: 5, protocol: IPPROTO_TCP as __u8, tot_len: __bpf_constant_htons(MAGIC_BYTES - size_of::<iphdr>() as c_int), frag_off: 0, saddr: 0 },
                tcp: tcphdr { doff: 5, source: 99, dest: 9090 },
            }) },
            keys: zeroed(),
            flags: 0,
            retval: BPF_FLOW_DISSECTOR_CONTINUE as __u32,
        },
        test {
            name: c"ip-gre".as_ptr(),
            pkt: test_pkt { gre_minimal: core::mem::ManuallyDrop::new(gre_minimal_pkt {
                eth: ethhdr { h_proto: __bpf_constant_htons(ETH_P_IP) },
                iph: iphdr { ihl: 5, protocol: IPPROTO_GRE as __u8, tot_len: __bpf_constant_htons(MAGIC_BYTES), frag_off: 0, saddr: 0 },
                gre_hdr: gre_base_hdr { flags: 0, protocol: __bpf_constant_htons(ETH_P_IP) },
                iph_inner: iphdr { ihl: 5, protocol: IPPROTO_TCP as __u8, tot_len: __bpf_constant_htons(MAGIC_BYTES - size_of::<iphdr>() as c_int), frag_off: 0, saddr: 0 },
                tcp: tcphdr { doff: 5, source: 80, dest: 8080 },
            }) },
            keys: bpf_flow_keys { nhoff: ETH_HLEN as __u16, thoff: (ETH_HLEN as usize + size_of::<iphdr>() * 2 + size_of::<gre_base_hdr>()) as __u16, addr_proto: ETH_P_IP as __u16, ip_proto: IPPROTO_TCP as __u8, n_proto: __bpf_constant_htons(ETH_P_IP), is_encap: true, sport: 80, dport: 8080, ..zeroed() },
            flags: 0,
            retval: BPF_OK as __u32,
        },
        test {
            name: c"ip-gre-no-encap".as_ptr(),
            pkt: test_pkt { ipip: core::mem::ManuallyDrop::new(ipip_pkt {
                eth: ethhdr { h_proto: __bpf_constant_htons(ETH_P_IP) },
                iph: iphdr { ihl: 5, protocol: IPPROTO_GRE as __u8, tot_len: __bpf_constant_htons(MAGIC_BYTES), frag_off: 0, saddr: 0 },
                iph_inner: iphdr { ihl: 5, protocol: IPPROTO_TCP as __u8, tot_len: __bpf_constant_htons(MAGIC_BYTES - size_of::<iphdr>() as c_int), frag_off: 0, saddr: 0 },
                tcp: tcphdr { doff: 5, source: 80, dest: 8080 },
            }) },
            keys: bpf_flow_keys { flags: BPF_FLOW_DISSECTOR_F_STOP_AT_ENCAP, nhoff: ETH_HLEN as __u16, thoff: (ETH_HLEN as usize + size_of::<iphdr>() + size_of::<gre_base_hdr>()) as __u16, addr_proto: ETH_P_IP as __u16, ip_proto: IPPROTO_GRE as __u8, n_proto: __bpf_constant_htons(ETH_P_IP), is_encap: true, ..zeroed() },
            flags: BPF_FLOW_DISSECTOR_F_STOP_AT_ENCAP,
            retval: BPF_OK as __u32,
        },
    ]
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn serial_test_flow_dissector_namespace() {
    let mut err: c_int;
    let prog_fd: c_int;

    let skel = bpf_flow__open_and_load();
    if !ASSERT_OK_PTR(skel as *mut c_void, c"open/load skeleton".as_ptr()) {
        return;
    }

    prog_fd = bpf_program__fd((*skel).progs._dissect);
    if !ASSERT_OK_FD(prog_fd, c"get dissector fd".as_ptr()) {
        bpf_flow__destroy(skel);
        return;
    }

    /* We must be able to attach a flow dissector to root namespace */
    err = bpf_prog_attach(prog_fd, 0, BPF_FLOW_DISSECTOR, 0);
    if !ASSERT_OK(err, c"attach on root namespace ok".as_ptr()) {
        bpf_flow__destroy(skel);
        return;
    }

    err = make_netns(TEST_NS.as_ptr() as *const c_char);
    if !ASSERT_OK(err, c"create non-root net namespace".as_ptr()) {
        bpf_flow__destroy(skel);
        return;
    }

    /* We must not be able to additionally attach a flow dissector to a
     * non-root net namespace
     */
    let mut ns = open_netns(TEST_NS.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(ns as *mut c_void, c"enter non-root net namespace".as_ptr()) {
        remove_netns(TEST_NS.as_ptr() as *const c_char);
        bpf_flow__destroy(skel);
        return;
    }
    err = bpf_prog_attach(prog_fd, 0, BPF_FLOW_DISSECTOR, 0);
    if !ASSERT_ERR(err, c"refuse new flow dissector in non-root net namespace".as_ptr()) {
        bpf_prog_detach2(prog_fd, 0, BPF_FLOW_DISSECTOR);
    } else {
        ASSERT_EQ(errno, EEXIST, c"refused because of already attached prog".as_ptr());
    }
    close_netns(ns);

    /* If no flow dissector is attached to the root namespace, we must
     * be able to attach one to a non-root net namespace
     */
    bpf_prog_detach2(prog_fd, 0, BPF_FLOW_DISSECTOR);
    ns = open_netns(TEST_NS.as_ptr() as *const c_char);
    ASSERT_OK_PTR(ns as *mut c_void, c"enter non-root net namespace".as_ptr());
    err = bpf_prog_attach(prog_fd, 0, BPF_FLOW_DISSECTOR, 0);
    close_netns(ns);
    ASSERT_OK(err, c"accept new flow dissector in non-root net namespace".as_ptr());

    /* If a flow dissector is attached to non-root net namespace, attaching
     * a flow dissector to root namespace must fail
     */
    err = bpf_prog_attach(prog_fd, 0, BPF_FLOW_DISSECTOR, 0);
    if !ASSERT_ERR(err, c"refuse new flow dissector on root namespace".as_ptr()) {
        bpf_prog_detach2(prog_fd, 0, BPF_FLOW_DISSECTOR);
    } else {
        ASSERT_EQ(errno, EEXIST, c"refused because of already attached prog".as_ptr());
    }

    ns = open_netns(TEST_NS.as_ptr() as *const c_char);
    bpf_prog_detach2(prog_fd, 0, BPF_FLOW_DISSECTOR);
    close_netns(ns);
    remove_netns(TEST_NS.as_ptr() as *const c_char);
    bpf_flow__destroy(skel);
}

unsafe fn create_tap(ifname: *const c_char) -> c_int {
    let mut ifr: ifreq = zeroed();
    ifr.ifr_flags = IFF_TAP | IFF_NO_PI | IFF_NAPI | IFF_NAPI_FRAGS;
    strscpy(ifr.ifr_name.as_mut_ptr(), ifname);

    let fd = open(c"/dev/net/tun".as_ptr(), O_RDWR);
    if fd < 0 {
        return -1;
    }

    let ret = ioctl(fd, TUNSETIFF, &mut ifr);
    if ret != 0 {
        return -1;
    }

    fd
}

unsafe fn tx_tap(fd: c_int, pkt: *mut c_void, len: size_t) -> c_int {
    let iov = [iovec { iov_len: len, iov_base: pkt }];
    writev(fd, iov.as_ptr(), iov.len() as c_int) as c_int
}

unsafe fn ifup(ifname: *const c_char) -> c_int {
    let mut ifr: ifreq = zeroed();
    strscpy(ifr.ifr_name.as_mut_ptr(), ifname);

    let sk = socket(PF_INET, SOCK_DGRAM, 0);
    if sk < 0 {
        return -1;
    }

    let mut ret = ioctl(sk, SIOCGIFFLAGS, &mut ifr);
    if ret != 0 {
        close(sk);
        return -1;
    }

    ifr.ifr_flags |= IFF_UP;
    ret = ioctl(sk, SIOCSIFFLAGS, &mut ifr);
    if ret != 0 {
        close(sk);
        return -1;
    }

    close(sk);
    0
}

unsafe fn init_prog_array(obj: *mut bpf_object, prog_array: *mut bpf_map) -> c_int {
    let map_fd = bpf_map__fd(prog_array);
    if map_fd < 0 {
        return -1;
    }

    let mut i = 0;
    while i < bpf_map__max_entries(prog_array) {
        let mut prog_name = [0 as c_char; 32];
        snprintf(prog_name.as_mut_ptr(), prog_name.len(), c"flow_dissector_%d".as_ptr(), i);

        let prog = bpf_object__find_program_by_name(obj, prog_name.as_ptr());
        if prog.is_null() {
            return -1;
        }

        let prog_fd = bpf_program__fd(prog);
        if prog_fd < 0 {
            return -1;
        }

        let err = bpf_map_update_elem(
            map_fd,
            &i as *const _ as *const c_void,
            &prog_fd as *const _ as *const c_void,
            BPF_ANY,
        );
        if err != 0 {
            return -1;
        }
        i += 1;
    }
    0
}

unsafe fn run_tests_skb_less(tap_fd: c_int, keys: *mut bpf_map, test_suffix: *mut c_char) {
    let mut test_name = [0 as c_char; TEST_NAME_MAX_LEN];
    let keys_fd = bpf_map__fd(keys);
    if !ASSERT_OK_FD(keys_fd, c"bpf_map__fd".as_ptr()) {
        return;
    }

    let tests = tests();
    let mut i = 0usize;
    while i < tests.len() {
        /* Keep in sync with 'flags' from eth_get_headlen. */
        let eth_get_headlen_flags: __u32 = BPF_FLOW_DISSECTOR_F_PARSE_1ST_FRAG;
        let mut flow_keys: bpf_flow_keys = zeroed();
        let key: __u32 = ((tests[i].keys.sport as __u32) << 16) | tests[i].keys.dport as __u32;
        snprintf(test_name.as_mut_ptr(), TEST_NAME_MAX_LEN, c"%s-%s".as_ptr(), tests[i].name, test_suffix);
        if !test__start_subtest(test_name.as_ptr()) {
            i += 1;
            continue;
        }

        /* For skb-less case we can't pass input flags; run
         * only the tests that have a matching set of flags.
         */
        if tests[i].flags != eth_get_headlen_flags {
            i += 1;
            continue;
        }

        let err = tx_tap(tap_fd, &tests[i].pkt as *const _ as *mut c_void, size_of::<test_pkt>());
        if !ASSERT_EQ(err, size_of::<test_pkt>() as c_int, c"tx_tap".as_ptr()) {
            i += 1;
            continue;
        }

        /* check the stored flow_keys only if BPF_OK expected */
        if tests[i].retval != BPF_OK as __u32 {
            i += 1;
            continue;
        }

        let mut err = bpf_map_lookup_elem(
            keys_fd,
            &key as *const _ as *const c_void,
            &mut flow_keys as *mut _ as *mut c_void,
        );
        if !ASSERT_OK(err, c"bpf_map_lookup_elem".as_ptr()) {
            i += 1;
            continue;
        }

        ASSERT_MEMEQ(
            &flow_keys as *const _ as *const c_void,
            &tests[i].keys as *const _ as *const c_void,
            size_of::<bpf_flow_keys>(),
            c"returned flow keys".as_ptr(),
        );

        err = bpf_map_delete_elem(keys_fd, &key as *const _ as *const c_void);
        ASSERT_OK(err, c"bpf_map_delete_elem".as_ptr());
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_flow_dissector_skb_less_direct_attach() {
    let ns = netns_new(c"flow_dissector_skb_less_indirect_attach_ns".as_ptr(), true);
    if !ASSERT_OK_PTR(ns as *mut c_void, c"create and open netns".as_ptr()) {
        return;
    }

    let skel = bpf_flow__open_and_load();
    if !ASSERT_OK_PTR(skel as *mut c_void, c"open/load skeleton".as_ptr()) {
        netns_free(ns);
        return;
    }

    let mut err = init_prog_array((*skel).obj, (*skel).maps.jmp_table);
    if !ASSERT_OK(err, c"init_prog_array".as_ptr()) {
        bpf_flow__destroy(skel);
        netns_free(ns);
        return;
    }

    let prog_fd = bpf_program__fd((*skel).progs._dissect);
    if !ASSERT_OK_FD(prog_fd, c"bpf_program__fd".as_ptr()) {
        bpf_flow__destroy(skel);
        netns_free(ns);
        return;
    }

    err = bpf_prog_attach(prog_fd, 0, BPF_FLOW_DISSECTOR, 0);
    if !ASSERT_OK(err, c"bpf_prog_attach".as_ptr()) {
        bpf_flow__destroy(skel);
        netns_free(ns);
        return;
    }

    let tap_fd = create_tap(c"tap0".as_ptr());
    if !ASSERT_OK_FD(tap_fd, c"create_tap".as_ptr()) {
        bpf_flow__destroy(skel);
        netns_free(ns);
        return;
    }
    err = ifup(c"tap0".as_ptr());
    if !ASSERT_OK(err, c"ifup".as_ptr()) {
        close(tap_fd);
        bpf_flow__destroy(skel);
        netns_free(ns);
        return;
    }

    run_tests_skb_less(tap_fd, (*skel).maps.last_dissection, c"non-skb-direct-attach".as_ptr() as *mut c_char);

    err = bpf_prog_detach2(prog_fd, 0, BPF_FLOW_DISSECTOR);
    ASSERT_OK(err, c"bpf_prog_detach2".as_ptr());

    close(tap_fd);
    bpf_flow__destroy(skel);
    netns_free(ns);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_flow_dissector_skb_less_indirect_attach() {
    let ns = netns_new(c"flow_dissector_skb_less_indirect_attach_ns".as_ptr(), true);
    if !ASSERT_OK_PTR(ns as *mut c_void, c"create and open netns".as_ptr()) {
        return;
    }

    let skel = bpf_flow__open_and_load();
    if !ASSERT_OK_PTR(skel as *mut c_void, c"open/load skeleton".as_ptr()) {
        netns_free(ns);
        return;
    }

    let net_fd = open(c"/proc/self/ns/net".as_ptr(), O_RDONLY);
    if !ASSERT_OK_FD(net_fd, c"open(/proc/self/ns/net".as_ptr()) {
        bpf_flow__destroy(skel);
        netns_free(ns);
        return;
    }

    let mut err = init_prog_array((*skel).obj, (*skel).maps.jmp_table);
    if !ASSERT_OK(err, c"init_prog_array".as_ptr()) {
        bpf_flow__destroy(skel);
        netns_free(ns);
        return;
    }

    let tap_fd = create_tap(c"tap0".as_ptr());
    if !ASSERT_OK_FD(tap_fd, c"create_tap".as_ptr()) {
        close(net_fd);
        bpf_flow__destroy(skel);
        netns_free(ns);
        return;
    }
    err = ifup(c"tap0".as_ptr());
    if !ASSERT_OK(err, c"ifup".as_ptr()) {
        close(tap_fd);
        close(net_fd);
        bpf_flow__destroy(skel);
        netns_free(ns);
        return;
    }

    let link = bpf_program__attach_netns((*skel).progs._dissect, net_fd);
    if !ASSERT_OK_PTR(link as *mut c_void, c"attach_netns".as_ptr()) {
        close(tap_fd);
        close(net_fd);
        bpf_flow__destroy(skel);
        netns_free(ns);
        return;
    }

    run_tests_skb_less(tap_fd, (*skel).maps.last_dissection, c"non-skb-indirect-attach".as_ptr() as *mut c_char);

    err = bpf_link__destroy(link);
    ASSERT_OK(err, c"bpf_link__destroy".as_ptr());

    close(tap_fd);
    close(net_fd);
    bpf_flow__destroy(skel);
    netns_free(ns);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_flow_dissector_skb() {
    let mut test_name = [0 as c_char; TEST_NAME_MAX_LEN];

    let skel = bpf_flow__open_and_load();
    if !ASSERT_OK_PTR(skel as *mut c_void, c"open/load skeleton".as_ptr()) {
        return;
    }

    let mut err = init_prog_array((*skel).obj, (*skel).maps.jmp_table);
    if !ASSERT_OK(err, c"init_prog_array".as_ptr()) {
        bpf_flow__destroy(skel);
        return;
    }

    let prog_fd = bpf_program__fd((*skel).progs._dissect);
    if !ASSERT_OK_FD(prog_fd, c"bpf_program__fd".as_ptr()) {
        bpf_flow__destroy(skel);
        return;
    }

    let tests = tests();
    let mut i = 0usize;
    while i < tests.len() {
        let mut flow_keys: bpf_flow_keys = zeroed();
        let mut topts = bpf_test_run_opts {
            sz: size_of::<bpf_test_run_opts>(),
            data_in: &tests[i].pkt as *const _ as *mut c_void,
            data_size_in: size_of::<test_pkt>() as __u32,
            data_out: &mut flow_keys as *mut _ as *mut c_void,
            data_size_out: size_of::<bpf_flow_keys>() as __u32,
            ctx_in: core::ptr::null_mut(),
            ctx_size_in: 0,
            retval: 0,
        };
        static mut ctx: bpf_flow_keys = bpf_flow_keys {
            nhoff: 0,
            thoff: 0,
            addr_proto: 0,
            ip_proto: 0,
            n_proto: 0,
            flags: 0,
            is_frag: false,
            is_first_frag: false,
            is_encap: false,
            sport: 0,
            dport: 0,
            flow_label: 0,
        };

        snprintf(test_name.as_mut_ptr(), TEST_NAME_MAX_LEN, c"%s-skb".as_ptr(), tests[i].name);
        if !test__start_subtest(test_name.as_ptr()) {
            i += 1;
            continue;
        }

        if tests[i].flags != 0 {
            topts.ctx_in = &raw mut ctx as *mut c_void;
            topts.ctx_size_in = size_of::<bpf_flow_keys>() as __u32;
            ctx.flags = tests[i].flags;
        }

        err = bpf_prog_test_run_opts(prog_fd, &mut topts);
        ASSERT_OK(err, c"test_run".as_ptr());
        ASSERT_EQ(topts.retval as c_int, tests[i].retval as c_int, c"test_run retval".as_ptr());

        /* check the resulting flow_keys only if BPF_OK returned */
        if topts.retval != BPF_OK as __u32 {
            i += 1;
            continue;
        }
        ASSERT_EQ(topts.data_size_out as c_int, size_of::<bpf_flow_keys>() as c_int, c"test_run data_size_out".as_ptr());
        ASSERT_MEMEQ(
            &flow_keys as *const _ as *const c_void,
            &tests[i].keys as *const _ as *const c_void,
            size_of::<bpf_flow_keys>(),
            c"returned flow keys".as_ptr(),
        );
        i += 1;
    }

    bpf_flow__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
