// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

/* Translated from C. External kernel/libbpf/test harness declarations are
 * intentionally left as dependencies supplied by surrounding files.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

const NS_TEST: *const c_char = b"fib_lookup_ns\0".as_ptr() as *const c_char;
const IPV6_IFACE_ADDR: *const c_char = b"face::face\0".as_ptr() as *const c_char;
const IPV6_IFACE_ADDR_SEC: *const c_char = b"cafe::cafe\0".as_ptr() as *const c_char;
const IPV6_ADDR_DST: *const c_char = b"face::3\0".as_ptr() as *const c_char;
const IPV6_NUD_FAILED_ADDR: *const c_char = b"face::1\0".as_ptr() as *const c_char;
const IPV6_NUD_STALE_ADDR: *const c_char = b"face::2\0".as_ptr() as *const c_char;
const IPV4_IFACE_ADDR: *const c_char = b"10.0.0.254\0".as_ptr() as *const c_char;
const IPV4_IFACE_ADDR_SEC: *const c_char = b"10.1.0.254\0".as_ptr() as *const c_char;
const IPV4_ADDR_DST: *const c_char = b"10.2.0.254\0".as_ptr() as *const c_char;
const IPV4_NUD_FAILED_ADDR: *const c_char = b"10.0.0.1\0".as_ptr() as *const c_char;
const IPV4_NUD_STALE_ADDR: *const c_char = b"10.0.0.2\0".as_ptr() as *const c_char;
const IPV4_TBID_ADDR: *const c_char = b"172.0.0.254\0".as_ptr() as *const c_char;
const IPV4_TBID_NET: *const c_char = b"172.0.0.0\0".as_ptr() as *const c_char;
const IPV4_TBID_DST: *const c_char = b"172.0.0.2\0".as_ptr() as *const c_char;
const IPV4_TBID_NONEIGH_DST: *const c_char = b"172.0.0.5\0".as_ptr() as *const c_char;
const IPV6_TBID_ADDR: *const c_char = b"fd00::FFFF\0".as_ptr() as *const c_char;
const IPV6_TBID_NET: *const c_char = b"fd00::\0".as_ptr() as *const c_char;
const IPV6_TBID_DST: *const c_char = b"fd00::2\0".as_ptr() as *const c_char;
const MARK_NO_POLICY: u32 = 33;
const MARK: u32 = 42;
const MARK_TABLE: *const c_char = b"200\0".as_ptr() as *const c_char;
const IPV4_REMOTE_DST: *const c_char = b"1.2.3.4\0".as_ptr() as *const c_char;
const IPV4_LOCAL: *const c_char = b"10.4.0.3\0".as_ptr() as *const c_char;
const IPV4_GW1: *const c_char = b"10.4.0.1\0".as_ptr() as *const c_char;
const IPV4_GW2: *const c_char = b"10.4.0.2\0".as_ptr() as *const c_char;
const IPV6_REMOTE_DST: *const c_char = b"be:ef::b0:10\0".as_ptr() as *const c_char;
const IPV6_LOCAL: *const c_char = b"fd01::3\0".as_ptr() as *const c_char;
const IPV6_GW1: *const c_char = b"fd01::1\0".as_ptr() as *const c_char;
const IPV6_GW2: *const c_char = b"fd01::2\0".as_ptr() as *const c_char;
const VLAN_ID: u16 = 100;
const VLAN_IFACE: *const c_char = b"veth1.100\0".as_ptr() as *const c_char;
const VLAN_ID_DOWN: u16 = 102;
const VLAN_IFACE_DOWN: *const c_char = b"veth1.102\0".as_ptr() as *const c_char;
const QINQ_OUTER_IFACE: *const c_char = b"veth1.200\0".as_ptr() as *const c_char;
const QINQ_INNER_IFACE: *const c_char = b"veth1.200.300\0".as_ptr() as *const c_char;
const VLAN_TABLE: *const c_char = b"300\0".as_ptr() as *const c_char;
const IPV4_VLAN_IFACE_ADDR: *const c_char = b"10.5.0.254\0".as_ptr() as *const c_char;
const IPV4_VLAN_EGRESS_DST: *const c_char = b"10.5.0.2\0".as_ptr() as *const c_char;
const IPV4_QINQ_DST: *const c_char = b"10.7.0.2\0".as_ptr() as *const c_char;
const IPV4_VLAN_DST: *const c_char = b"10.6.0.2\0".as_ptr() as *const c_char;
const IPV4_VLAN_GW: *const c_char = b"10.5.0.1\0".as_ptr() as *const c_char;
const IPV6_VLAN_IFACE_ADDR: *const c_char = b"fd02::254\0".as_ptr() as *const c_char;
const IPV6_VLAN_EGRESS_DST: *const c_char = b"fd02::2\0".as_ptr() as *const c_char;
const IPV6_VLAN_DST: *const c_char = b"fd03::2\0".as_ptr() as *const c_char;
const IPV6_VLAN_GW: *const c_char = b"fd02::1\0".as_ptr() as *const c_char;
const VLAN_VID_UNUSED: u16 = 999;
const VRF_IFACE: *const c_char = b"vrf-blue\0".as_ptr() as *const c_char;
const VRF_TABLE: *const c_char = b"1000\0".as_ptr() as *const c_char;
const VRF_VLAN_ID: u16 = 101;
const VRF_VLAN_IFACE: *const c_char = b"veth1.101\0".as_ptr() as *const c_char;
const IPV4_VRF_IFACE_ADDR: *const c_char = b"10.8.0.254\0".as_ptr() as *const c_char;
const IPV4_VRF_GW: *const c_char = b"10.8.0.1\0".as_ptr() as *const c_char;
const IPV4_VRF_DST: *const c_char = b"10.9.0.2\0".as_ptr() as *const c_char;
const TBID_VLAN_ID: u16 = 50;
const TBID_VLAN_IFACE: *const c_char = b"veth2.50\0".as_ptr() as *const c_char;
const IPV4_TBID_VLAN_DST: *const c_char = b"172.2.0.2\0".as_ptr() as *const c_char;
const IPV4_BOND_VLAN_DST: *const c_char = b"10.11.0.2\0".as_ptr() as *const c_char;
const IPV4_VLAN_MTU_DST: *const c_char = b"10.5.9.2\0".as_ptr() as *const c_char;
const QINQ_AD_VLAN_ID: u16 = 200;
const QINQ_INNER_VLAN_ID: u16 = 300;
const BOND_IFACE: *const c_char = b"bond99\0".as_ptr() as *const c_char;
const BOND_PORT: *const c_char = b"veth3\0".as_ptr() as *const c_char;
const BOND_PORT_PEER: *const c_char = b"veth4\0".as_ptr() as *const c_char;
const BOND_VLAN_ID: u16 = 500;
const DMAC: *const c_char = b"11:11:11:11:11:11\0".as_ptr() as *const c_char;
const DMAC_INIT: [u8; 6] = [0x11, 0x11, 0x11, 0x11, 0x11, 0x11];
const DMAC2: *const c_char = b"01:01:01:01:01:01\0".as_ptr() as *const c_char;
const DMAC_INIT2: [u8; 6] = [0x01, 0x01, 0x01, 0x01, 0x01, 0x01];

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const IPPROTO_TCP: u8 = 6;
const INET6_ADDRSTRLEN: usize = 46;
const EINVAL: c_int = 22;
const RT_TABLE_MAIN: u32 = 254;
const ETH_P_8021Q: u16 = 0x8100;
const ETH_P_8021AD: u16 = 0x88A8;
const XDP_FLAGS_DRV_MODE: u32 = 1 << 2;
const BPF_F_TEST_XDP_LIVE_FRAMES: u32 = 1 << 2;

extern "C" {
    static pkt_v6: [u8; 0];
    static pkt_v4: [u8; 0];

    fn write_sysctl(path: *const c_char, val: *const c_char) -> c_int;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn inet_ntop(af: c_int, src: *const c_void, dst: *mut c_char, size: u32) -> *const c_char;
    fn htons(hostshort: u16) -> u16;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;

    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;
    fn ASSERT_EQ<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_NEQ<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_GT<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn PRINT_FAIL(format: *const c_char, ...) -> c_int;
    fn SYS(label: *const c_char, format: *const c_char, ...) -> c_int;
    fn SYS_NOFAIL(format: *const c_char, ...) -> c_int;

    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(token: *mut nstoken);
    fn fib_lookup__open_and_load() -> *mut fib_lookup;
    fn fib_lookup__destroy(obj: *mut fib_lookup);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_xdp_attach(ifindex: c_int, prog_fd: c_int, flags: u32, opts: *const c_void) -> c_int;
    fn bpf_xdp_detach(ifindex: c_int, flags: u32, opts: *const c_void) -> c_int;
}

#[repr(C)]
pub struct bpf_fib_lookup {
    family: u8,
    l4_protocol: u8,
    sport: u16,
    dport: u16,
    tot_len: u16,
    ifindex: u32,
    tos: u8,
    _pad1: u8,
    _pad2: u16,
    ipv6_src: [u8; 16],
    ipv6_dst: [u8; 16],
    h_vlan_proto: u16,
    h_vlan_TCI: u16,
    smac: [u8; 6],
    dmac: [u8; 6],
    mark: u32,
    tbid: u32,
    mtu_result: u16,
}

#[repr(C)]
pub struct __sk_buff {
    ifindex: u32,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    data_in: *const c_void,
    data_size_in: u32,
    ctx_in: *mut c_void,
    ctx_size_in: u32,
    flags: u32,
    repeat: u32,
}

#[repr(C)]
pub struct fib_lookup {
    progs: fib_lookup_progs,
    bss: *mut fib_lookup_bss,
}

#[repr(C)]
pub struct fib_lookup_progs {
    fib_lookup: *mut bpf_program,
    fib_lookup_xdp: *mut bpf_program,
    fib_lookup_redirect: *mut bpf_program,
    xdp_count: *mut bpf_program,
}

#[repr(C)]
pub struct fib_lookup_bss {
    fib_params: bpf_fib_lookup,
    fib_lookup_ret: c_int,
    lookup_flags: c_int,
    redirected: c_int,
    passed: c_int,
    delivered: c_int,
}

pub enum nstoken {}
pub enum bpf_program {}

extern "C" {
    static BPF_FIB_LKUP_RET_SUCCESS: c_int;
    static BPF_FIB_LKUP_RET_NO_NEIGH: c_int;
    static BPF_FIB_LKUP_RET_NOT_FWDED: c_int;
    static BPF_FIB_LKUP_RET_FWD_DISABLED: c_int;
    static BPF_FIB_LKUP_RET_FRAG_NEEDED: c_int;
    static BPF_FIB_LKUP_RET_VLAN_FAILURE: c_int;
    static BPF_FIB_LOOKUP_SKIP_NEIGH: c_int;
    static BPF_FIB_LOOKUP_DIRECT: c_int;
    static BPF_FIB_LOOKUP_TBID: c_int;
    static BPF_FIB_LOOKUP_SRC: c_int;
    static BPF_FIB_LOOKUP_MARK: c_int;
    static BPF_FIB_LOOKUP_VLAN: c_int;
    static BPF_FIB_LOOKUP_OUTPUT: c_int;
    static BPF_FIB_LOOKUP_VLAN_INPUT: c_int;
}

#[repr(C)]
struct fib_lookup_test {
    desc: *const c_char,
    daddr: *const c_char,
    expected_ret: c_int,
    expected_src: *const c_char,
    expected_dst: *const c_char,
    lookup_flags: c_int,
    tbid: u32,
    dmac: [u8; 6],
    mark: u32,
    /*
     * input tag with BPF_FIB_LOOKUP_VLAN_INPUT; expected output tag
     * with BPF_FIB_LOOKUP_VLAN (checked when check_vlan is set)
     */
    vlan_proto: u16,
    vlan_id: u16,
    check_vlan: bool,
    expected_dev: *const c_char, /* expected params->ifindex after lookup */
    iif: *const c_char,          /* override the default veth1 input device */
    tot_len: u16,                /* triggers the in-lookup mtu check when set */
    expected_mtu: u16,           /* expected mtu_result (union with tot_len) */
}

const fn flt(desc: &'static [u8], daddr: *const c_char, expected_ret: c_int) -> fib_lookup_test {
    fib_lookup_test {
        desc: desc.as_ptr() as *const c_char,
        daddr,
        expected_ret,
        expected_src: ptr::null(),
        expected_dst: ptr::null(),
        lookup_flags: 0,
        tbid: 0,
        dmac: [0; 6],
        mark: 0,
        vlan_proto: 0,
        vlan_id: 0,
        check_vlan: false,
        expected_dev: ptr::null(),
        iif: ptr::null(),
        tot_len: 0,
        expected_mtu: 0,
    }
}

macro_rules! t {
    ($desc:expr, $daddr:expr, $ret:expr $(, $field:ident : $value:expr )* $(,)?) => {{
        let mut v = flt($desc, $daddr, $ret);
        $(v.$field = $value;)*
        v
    }};
}

static tests: &[fib_lookup_test] = &[
    t!(b"IPv6 failed neigh\0", IPV6_NUD_FAILED_ADDR, unsafe { BPF_FIB_LKUP_RET_NO_NEIGH }),
    t!(b"IPv6 stale neigh\0", IPV6_NUD_STALE_ADDR, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, dmac: DMAC_INIT),
    t!(b"IPv6 skip neigh\0", IPV6_NUD_FAILED_ADDR, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, lookup_flags: unsafe { BPF_FIB_LOOKUP_SKIP_NEIGH }),
    t!(b"IPv4 failed neigh\0", IPV4_NUD_FAILED_ADDR, unsafe { BPF_FIB_LKUP_RET_NO_NEIGH }),
    t!(b"IPv4 stale neigh\0", IPV4_NUD_STALE_ADDR, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, dmac: DMAC_INIT),
    t!(b"IPv4 skip neigh\0", IPV4_NUD_FAILED_ADDR, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, lookup_flags: unsafe { BPF_FIB_LOOKUP_SKIP_NEIGH }),
    t!(b"IPv4 TBID lookup failure\0", IPV4_TBID_DST, unsafe { BPF_FIB_LKUP_RET_NOT_FWDED }, lookup_flags: unsafe { BPF_FIB_LOOKUP_DIRECT | BPF_FIB_LOOKUP_TBID }, tbid: RT_TABLE_MAIN),
    t!(b"IPv4 TBID lookup success\0", IPV4_TBID_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, lookup_flags: unsafe { BPF_FIB_LOOKUP_DIRECT | BPF_FIB_LOOKUP_TBID }, tbid: 100, dmac: DMAC_INIT2),
    /*
     * An error that returns after the egress device is resolved must
     * report the egress ifindex, not the input. This routes from input
     * veth1 via veth2 (table 100) to a dst with no neighbour, so
     * input != egress, pinning NO_NEIGH to the egress device.
     */
    t!(b"IPv4 NO_NEIGH reports the egress ifindex, not the input\0", IPV4_TBID_NONEIGH_DST, unsafe { BPF_FIB_LKUP_RET_NO_NEIGH }, lookup_flags: unsafe { BPF_FIB_LOOKUP_DIRECT | BPF_FIB_LOOKUP_TBID }, tbid: 100, expected_dev: b"veth2\0".as_ptr() as *const c_char),
    t!(b"IPv6 TBID lookup failure\0", IPV6_TBID_DST, unsafe { BPF_FIB_LKUP_RET_NOT_FWDED }, lookup_flags: unsafe { BPF_FIB_LOOKUP_DIRECT | BPF_FIB_LOOKUP_TBID }, tbid: RT_TABLE_MAIN),
    t!(b"IPv6 TBID lookup success\0", IPV6_TBID_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, lookup_flags: unsafe { BPF_FIB_LOOKUP_DIRECT | BPF_FIB_LOOKUP_TBID }, tbid: 100, dmac: DMAC_INIT2),
    t!(b"IPv4 set src addr from netdev\0", IPV4_NUD_FAILED_ADDR, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, expected_src: IPV4_IFACE_ADDR, lookup_flags: unsafe { BPF_FIB_LOOKUP_SRC | BPF_FIB_LOOKUP_SKIP_NEIGH }),
    t!(b"IPv6 set src addr from netdev\0", IPV6_NUD_FAILED_ADDR, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, expected_src: IPV6_IFACE_ADDR, lookup_flags: unsafe { BPF_FIB_LOOKUP_SRC | BPF_FIB_LOOKUP_SKIP_NEIGH }),
    t!(b"IPv4 set prefsrc addr from route\0", IPV4_ADDR_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, expected_src: IPV4_IFACE_ADDR_SEC, lookup_flags: unsafe { BPF_FIB_LOOKUP_SRC | BPF_FIB_LOOKUP_SKIP_NEIGH }),
    t!(b"IPv6 set prefsrc addr route\0", IPV6_ADDR_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, expected_src: IPV6_IFACE_ADDR_SEC, lookup_flags: unsafe { BPF_FIB_LOOKUP_SRC | BPF_FIB_LOOKUP_SKIP_NEIGH }),
    /* policy routing */
    t!(b"IPv4 policy routing, default\0", IPV4_REMOTE_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, expected_dst: IPV4_GW1, lookup_flags: unsafe { BPF_FIB_LOOKUP_MARK | BPF_FIB_LOOKUP_SKIP_NEIGH }),
    t!(b"IPv4 policy routing, mark doesn't point to a policy\0", IPV4_REMOTE_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, expected_dst: IPV4_GW1, lookup_flags: unsafe { BPF_FIB_LOOKUP_MARK | BPF_FIB_LOOKUP_SKIP_NEIGH }, mark: MARK_NO_POLICY),
    t!(b"IPv4 policy routing, mark points to a policy\0", IPV4_REMOTE_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, expected_dst: IPV4_GW2, lookup_flags: unsafe { BPF_FIB_LOOKUP_MARK | BPF_FIB_LOOKUP_SKIP_NEIGH }, mark: MARK),
    t!(b"IPv4 policy routing, mark points to a policy, but no flag\0", IPV4_REMOTE_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, expected_dst: IPV4_GW1, lookup_flags: unsafe { BPF_FIB_LOOKUP_SKIP_NEIGH }, mark: MARK),
    t!(b"IPv6 policy routing, default\0", IPV6_REMOTE_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, expected_dst: IPV6_GW1, lookup_flags: unsafe { BPF_FIB_LOOKUP_MARK | BPF_FIB_LOOKUP_SKIP_NEIGH }),
    t!(b"IPv6 policy routing, mark doesn't point to a policy\0", IPV6_REMOTE_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, expected_dst: IPV6_GW1, lookup_flags: unsafe { BPF_FIB_LOOKUP_MARK | BPF_FIB_LOOKUP_SKIP_NEIGH }, mark: MARK_NO_POLICY),
    t!(b"IPv6 policy routing, mark points to a policy\0", IPV6_REMOTE_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, expected_dst: IPV6_GW2, lookup_flags: unsafe { BPF_FIB_LOOKUP_MARK | BPF_FIB_LOOKUP_SKIP_NEIGH }, mark: MARK),
    t!(b"IPv6 policy routing, mark points to a policy, but no flag\0", IPV6_REMOTE_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, expected_dst: IPV6_GW1, lookup_flags: unsafe { BPF_FIB_LOOKUP_SKIP_NEIGH }, mark: MARK),
    /* vlan egress resolution */
    /*
     * Invariant the VLAN-egress arms jointly enforce: a
     * BPF_FIB_LOOKUP_VLAN SUCCESS always carries a physical,
     * xmit-capable ifindex; no SUCCESS ever returns a VLAN-device
     * ifindex. Reducible arms pin ifindex == the physical parent; the
     * QinQ and foreign-netns arms pin VLAN_FAILURE with params->ifindex
     * left at the input, so a regression to best-effort (SUCCESS + the
     * VLAN ifindex) fails one.
     */
    t!(b"IPv4 VLAN egress, no flag\0", IPV4_VLAN_EGRESS_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, lookup_flags: unsafe { BPF_FIB_LOOKUP_SKIP_NEIGH }, expected_dev: VLAN_IFACE, check_vlan: true),
    t!(b"IPv4 VLAN egress, single VLAN\0", IPV4_VLAN_EGRESS_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN | BPF_FIB_LOOKUP_SKIP_NEIGH }, expected_dev: b"veth1\0".as_ptr() as *const c_char, check_vlan: true, vlan_proto: ETH_P_8021Q, vlan_id: VLAN_ID),
    /*
     * skb path without tot_len: mtu_result is the VLAN device's mtu
     * (1400), not the parent's (1500)
     */
    t!(b"IPv4 VLAN egress, skb-path mtu is the VLAN device's without the flag\0", IPV4_VLAN_EGRESS_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, lookup_flags: unsafe { BPF_FIB_LOOKUP_SKIP_NEIGH }, expected_dev: VLAN_IFACE, check_vlan: true, expected_mtu: 1400),
    t!(b"IPv4 VLAN egress, flag set but egress is not a VLAN\0", IPV4_NUD_FAILED_ADDR, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN | BPF_FIB_LOOKUP_SKIP_NEIGH }, expected_dev: b"veth1\0".as_ptr() as *const c_char, check_vlan: true),
    t!(b"IPv4 VLAN egress, QinQ not reducible (VLAN_FAILURE)\0", IPV4_QINQ_DST, unsafe { BPF_FIB_LKUP_RET_VLAN_FAILURE }, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN | BPF_FIB_LOOKUP_SKIP_NEIGH }, expected_dev: b"veth1\0".as_ptr() as *const c_char, check_vlan: true),
    t!(b"IPv4 QinQ egress without the flag (escape hatch)\0", IPV4_QINQ_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, lookup_flags: unsafe { BPF_FIB_LOOKUP_SKIP_NEIGH }, expected_dev: QINQ_INNER_IFACE),
    t!(b"IPv6 VLAN egress, single VLAN\0", IPV6_VLAN_EGRESS_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN | BPF_FIB_LOOKUP_SKIP_NEIGH }, expected_dev: b"veth1\0".as_ptr() as *const c_char, check_vlan: true, vlan_proto: ETH_P_8021Q, vlan_id: VLAN_ID),
    t!(b"IPv4 VLAN egress, neighbour on the VLAN device\0", IPV4_VLAN_EGRESS_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN }, expected_dev: b"veth1\0".as_ptr() as *const c_char, check_vlan: true, vlan_proto: ETH_P_8021Q, vlan_id: VLAN_ID, dmac: DMAC_INIT),
    t!(b"IPv4 VLAN egress in OUTPUT mode\0", IPV4_VLAN_EGRESS_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, iif: VLAN_IFACE, lookup_flags: unsafe { BPF_FIB_LOOKUP_OUTPUT | BPF_FIB_LOOKUP_VLAN | BPF_FIB_LOOKUP_SKIP_NEIGH }, expected_dev: b"veth1\0".as_ptr() as *const c_char, check_vlan: true, vlan_proto: ETH_P_8021Q, vlan_id: VLAN_ID),
    t!(b"IPv4 VLAN egress over a bond\0", IPV4_BOND_VLAN_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN | BPF_FIB_LOOKUP_SKIP_NEIGH }, expected_dev: BOND_IFACE, check_vlan: true, vlan_proto: ETH_P_8021Q, vlan_id: BOND_VLAN_ID),
    t!(b"IPv4 VLAN egress via TBID table\0", IPV4_TBID_VLAN_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, lookup_flags: unsafe { BPF_FIB_LOOKUP_DIRECT | BPF_FIB_LOOKUP_TBID | BPF_FIB_LOOKUP_VLAN | BPF_FIB_LOOKUP_SKIP_NEIGH }, tbid: 100, expected_dev: b"veth2\0".as_ptr() as *const c_char, check_vlan: true, vlan_proto: ETH_P_8021Q, vlan_id: TBID_VLAN_ID),
    t!(b"IPv4 VLAN egress, success writes mtu_result with the swap\0", IPV4_VLAN_MTU_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, tot_len: 500, expected_mtu: 1000, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN | BPF_FIB_LOOKUP_SKIP_NEIGH }, expected_dev: b"veth1\0".as_ptr() as *const c_char, check_vlan: true, vlan_proto: ETH_P_8021Q, vlan_id: VLAN_ID),
    t!(b"IPv4 VLAN egress, FRAG_NEEDED reports mtu, swap unwritten\0", IPV4_VLAN_MTU_DST, unsafe { BPF_FIB_LKUP_RET_FRAG_NEEDED }, tot_len: 1400, expected_mtu: 1000, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN | BPF_FIB_LOOKUP_SKIP_NEIGH }, expected_dev: b"veth1\0".as_ptr() as *const c_char, check_vlan: true),
    /* vlan tag as lookup input */
    t!(b"IPv4 VLAN input, no flag\0", IPV4_VLAN_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, expected_dst: IPV4_GW1, lookup_flags: unsafe { BPF_FIB_LOOKUP_SKIP_NEIGH }),
    t!(b"IPv4 VLAN input, tag selects subinterface route\0", IPV4_VLAN_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, expected_dst: IPV4_VLAN_GW, expected_dev: VLAN_IFACE, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN_INPUT | BPF_FIB_LOOKUP_SKIP_NEIGH }, vlan_proto: ETH_P_8021Q, vlan_id: VLAN_ID),
    t!(b"IPv6 VLAN input, tag selects subinterface route\0", IPV6_VLAN_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, expected_dst: IPV6_VLAN_GW, expected_dev: VLAN_IFACE, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN_INPUT | BPF_FIB_LOOKUP_SKIP_NEIGH }, vlan_proto: ETH_P_8021Q, vlan_id: VLAN_ID),
    t!(b"IPv4 VLAN input and egress combined\0", IPV4_VLAN_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, expected_dst: IPV4_VLAN_GW, expected_dev: b"veth1\0".as_ptr() as *const c_char, check_vlan: true, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN_INPUT | BPF_FIB_LOOKUP_VLAN | BPF_FIB_LOOKUP_SKIP_NEIGH }, vlan_proto: ETH_P_8021Q, vlan_id: VLAN_ID),
    t!(b"IPv4 VLAN input, neighbour resolved on the route\0", IPV4_VLAN_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, expected_dst: IPV4_VLAN_GW, expected_dev: VLAN_IFACE, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN_INPUT }, vlan_proto: ETH_P_8021Q, vlan_id: VLAN_ID, dmac: DMAC_INIT2),
    t!(b"IPv4 VLAN input, source address from the subinterface\0", IPV4_VLAN_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, expected_src: IPV4_VLAN_IFACE_ADDR, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN_INPUT | BPF_FIB_LOOKUP_SRC | BPF_FIB_LOOKUP_SKIP_NEIGH }, vlan_proto: ETH_P_8021Q, vlan_id: VLAN_ID),
    /*
     * VRF: the resolved subinterface is enslaved, so the l3mdev rule
     * (full lookup) and l3mdev_fib_table_rcu() (DIRECT) must select
     * the VRF table from the resolved ingress
     */
    t!(b"IPv4 VLAN input, VRF subinterface, no flag\0", IPV4_VRF_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, expected_dst: IPV4_GW1, lookup_flags: unsafe { BPF_FIB_LOOKUP_SKIP_NEIGH }),
    t!(b"IPv4 VLAN input, tag selects VRF table\0", IPV4_VRF_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, expected_dst: IPV4_VRF_GW, expected_dev: VRF_VLAN_IFACE, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN_INPUT | BPF_FIB_LOOKUP_SKIP_NEIGH }, vlan_proto: ETH_P_8021Q, vlan_id: VRF_VLAN_ID),
    t!(b"IPv4 VLAN input, DIRECT uses VRF table from resolved ingress\0", IPV4_VRF_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, expected_dst: IPV4_VRF_GW, expected_dev: VRF_VLAN_IFACE, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN_INPUT | BPF_FIB_LOOKUP_DIRECT | BPF_FIB_LOOKUP_SKIP_NEIGH }, vlan_proto: ETH_P_8021Q, vlan_id: VRF_VLAN_ID),
    /*
     * failure arms also assert params is left untouched: ifindex still
     * names the physical device and the input tag bytes survive
     */
    t!(b"IPv4 VLAN input, invalid proto\0", IPV4_VLAN_DST, -EINVAL, expected_dev: b"veth1\0".as_ptr() as *const c_char, check_vlan: true, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN_INPUT | BPF_FIB_LOOKUP_SKIP_NEIGH }, vlan_proto: 0x1234, vlan_id: VLAN_ID),
    t!(b"IPv4 VLAN input, unmatched VID\0", IPV4_VLAN_DST, unsafe { BPF_FIB_LKUP_RET_NOT_FWDED }, expected_dev: b"veth1\0".as_ptr() as *const c_char, check_vlan: true, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN_INPUT | BPF_FIB_LOOKUP_SKIP_NEIGH }, vlan_proto: ETH_P_8021Q, vlan_id: VLAN_VID_UNUSED),
    t!(b"IPv4 VLAN input, subinterface down\0", IPV4_VLAN_DST, unsafe { BPF_FIB_LKUP_RET_NOT_FWDED }, expected_dev: b"veth1\0".as_ptr() as *const c_char, check_vlan: true, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN_INPUT | BPF_FIB_LOOKUP_SKIP_NEIGH }, vlan_proto: ETH_P_8021Q, vlan_id: VLAN_ID_DOWN),
    /*
     * the resolver runs before the forwarding check, so on devices
     * with forwarding off FWD_DISABLED (not NOT_FWDED) proves the tag
     * resolved to that device and the lookup used it as ingress
     */
    t!(b"IPv4 VLAN input, 802.1ad tag\0", IPV4_VLAN_DST, unsafe { BPF_FIB_LKUP_RET_FWD_DISABLED }, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN_INPUT | BPF_FIB_LOOKUP_SKIP_NEIGH }, vlan_proto: ETH_P_8021AD, vlan_id: QINQ_AD_VLAN_ID),
    t!(b"IPv4 VLAN input, PCP and DEI bits ignored in TCI\0", IPV4_VLAN_DST, unsafe { BPF_FIB_LKUP_RET_SUCCESS }, expected_dst: IPV4_VLAN_GW, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN_INPUT | BPF_FIB_LOOKUP_SKIP_NEIGH }, vlan_proto: ETH_P_8021Q, vlan_id: 0xe000 | VLAN_ID),
    t!(b"IPv4 VLAN input, inner QinQ device from VLAN ifindex\0", IPV4_VLAN_DST, unsafe { BPF_FIB_LKUP_RET_FWD_DISABLED }, iif: QINQ_OUTER_IFACE, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN_INPUT | BPF_FIB_LOOKUP_SKIP_NEIGH }, vlan_proto: ETH_P_8021Q, vlan_id: QINQ_INNER_VLAN_ID),
    /*
     * bonding: the VLANs live on the master, as on receive, where the
     * frame is steered to the master before VLAN processing; a port
     * ifindex does not match (ports carry vid state but no VLAN devs)
     */
    t!(b"IPv4 VLAN input, tag on bond master resolves\0", IPV4_VLAN_DST, unsafe { BPF_FIB_LKUP_RET_FWD_DISABLED }, iif: BOND_IFACE, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN_INPUT | BPF_FIB_LOOKUP_SKIP_NEIGH }, vlan_proto: ETH_P_8021Q, vlan_id: BOND_VLAN_ID),
    t!(b"IPv4 VLAN input, tag on bond port does not match\0", IPV4_VLAN_DST, unsafe { BPF_FIB_LKUP_RET_NOT_FWDED }, iif: BOND_PORT, expected_dev: BOND_PORT, check_vlan: true, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN_INPUT | BPF_FIB_LOOKUP_SKIP_NEIGH }, vlan_proto: ETH_P_8021Q, vlan_id: BOND_VLAN_ID),
    t!(b"IPv6 VLAN input, invalid proto\0", IPV6_VLAN_DST, -EINVAL, expected_dev: b"veth1\0".as_ptr() as *const c_char, check_vlan: true, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN_INPUT | BPF_FIB_LOOKUP_SKIP_NEIGH }, vlan_proto: 0x1234, vlan_id: VLAN_ID),
    t!(b"IPv4 VLAN input, VID 0 priority tag fails closed\0", IPV4_VLAN_DST, unsafe { BPF_FIB_LKUP_RET_NOT_FWDED }, expected_dev: b"veth1\0".as_ptr() as *const c_char, check_vlan: true, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN_INPUT | BPF_FIB_LOOKUP_SKIP_NEIGH }, vlan_proto: ETH_P_8021Q, vlan_id: 0),
    t!(b"IPv6 VLAN input, unmatched VID\0", IPV6_VLAN_DST, unsafe { BPF_FIB_LKUP_RET_NOT_FWDED }, expected_dev: b"veth1\0".as_ptr() as *const c_char, check_vlan: true, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN_INPUT | BPF_FIB_LOOKUP_SKIP_NEIGH }, vlan_proto: ETH_P_8021Q, vlan_id: VLAN_VID_UNUSED),
    t!(b"unknown flag bit rejected\0", IPV4_VLAN_DST, -EINVAL, lookup_flags: unsafe { (1 << 14) | BPF_FIB_LOOKUP_SKIP_NEIGH }),
    t!(b"IPv4 VLAN input rejected with TBID\0", IPV4_VLAN_DST, -EINVAL, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN_INPUT | BPF_FIB_LOOKUP_TBID }, vlan_proto: ETH_P_8021Q, vlan_id: VLAN_ID),
    t!(b"IPv4 VLAN input rejected with OUTPUT\0", IPV4_VLAN_DST, -EINVAL, lookup_flags: unsafe { BPF_FIB_LOOKUP_VLAN_INPUT | BPF_FIB_LOOKUP_OUTPUT }, vlan_proto: ETH_P_8021Q, vlan_id: VLAN_ID),
];

unsafe fn setup_netns() -> c_int {
    let mut err: c_int;
    let fail = b"fail\0".as_ptr() as *const c_char;

    /*
     * a new netns copies the IPv4 conf from init_net, so on a host with
     * forwarding enabled the arms that expect FWD_DISABLED would see the
     * lookup succeed instead; pin it off here and enable it per device
     */
    err = write_sysctl(b"/proc/sys/net/ipv4/conf/all/forwarding\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char);
    if !ASSERT_OK(err, b"write_sysctl(net.ipv4.conf.all.forwarding)\0".as_ptr() as *const c_char) {
        return -1;
    }
    err = write_sysctl(b"/proc/sys/net/ipv4/conf/default/forwarding\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char);
    if !ASSERT_OK(err, b"write_sysctl(net.ipv4.conf.default.forwarding)\0".as_ptr() as *const c_char) {
        return -1;
    }

    SYS(fail, b"ip link add veth1 type veth peer name veth2\0".as_ptr() as *const c_char);
    SYS(fail, b"ip link set dev veth1 up\0".as_ptr() as *const c_char);
    SYS(fail, b"ip link set dev veth2 up\0".as_ptr() as *const c_char);

    err = write_sysctl(b"/proc/sys/net/ipv4/neigh/veth1/gc_stale_time\0".as_ptr() as *const c_char, b"900\0".as_ptr() as *const c_char);
    if !ASSERT_OK(err, b"write_sysctl(net.ipv4.neigh.veth1.gc_stale_time)\0".as_ptr() as *const c_char) { return -1; }
    err = write_sysctl(b"/proc/sys/net/ipv6/neigh/veth1/gc_stale_time\0".as_ptr() as *const c_char, b"900\0".as_ptr() as *const c_char);
    if !ASSERT_OK(err, b"write_sysctl(net.ipv6.neigh.veth1.gc_stale_time)\0".as_ptr() as *const c_char) { return -1; }

    SYS(fail, b"ip addr add %s/64 dev veth1 nodad\0".as_ptr() as *const c_char, IPV6_IFACE_ADDR);
    SYS(fail, b"ip neigh add %s dev veth1 nud failed\0".as_ptr() as *const c_char, IPV6_NUD_FAILED_ADDR);
    SYS(fail, b"ip neigh add %s dev veth1 lladdr %s nud stale\0".as_ptr() as *const c_char, IPV6_NUD_STALE_ADDR, DMAC);
    SYS(fail, b"ip addr add %s/24 dev veth1\0".as_ptr() as *const c_char, IPV4_IFACE_ADDR);
    SYS(fail, b"ip neigh add %s dev veth1 nud failed\0".as_ptr() as *const c_char, IPV4_NUD_FAILED_ADDR);
    SYS(fail, b"ip neigh add %s dev veth1 lladdr %s nud stale\0".as_ptr() as *const c_char, IPV4_NUD_STALE_ADDR, DMAC);

    /* Setup for prefsrc IP addr selection */
    SYS(fail, b"ip addr add %s/24 dev veth1\0".as_ptr() as *const c_char, IPV4_IFACE_ADDR_SEC);
    SYS(fail, b"ip route add %s/32 dev veth1 src %s\0".as_ptr() as *const c_char, IPV4_ADDR_DST, IPV4_IFACE_ADDR_SEC);
    SYS(fail, b"ip addr add %s/64 dev veth1 nodad\0".as_ptr() as *const c_char, IPV6_IFACE_ADDR_SEC);
    SYS(fail, b"ip route add %s/128 dev veth1 src %s\0".as_ptr() as *const c_char, IPV6_ADDR_DST, IPV6_IFACE_ADDR_SEC);

    /* Setup for tbid lookup tests */
    SYS(fail, b"ip addr add %s/24 dev veth2\0".as_ptr() as *const c_char, IPV4_TBID_ADDR);
    SYS(fail, b"ip route del %s/24 dev veth2\0".as_ptr() as *const c_char, IPV4_TBID_NET);
    SYS(fail, b"ip route add table 100 %s/24 dev veth2\0".as_ptr() as *const c_char, IPV4_TBID_NET);
    SYS(fail, b"ip neigh add %s dev veth2 lladdr %s nud stale\0".as_ptr() as *const c_char, IPV4_TBID_DST, DMAC2);
    SYS(fail, b"ip addr add %s/64 dev veth2\0".as_ptr() as *const c_char, IPV6_TBID_ADDR);
    SYS(fail, b"ip -6 route del %s/64 dev veth2\0".as_ptr() as *const c_char, IPV6_TBID_NET);
    SYS(fail, b"ip -6 route add table 100 %s/64 dev veth2\0".as_ptr() as *const c_char, IPV6_TBID_NET);
    SYS(fail, b"ip neigh add %s dev veth2 lladdr %s nud stale\0".as_ptr() as *const c_char, IPV6_TBID_DST, DMAC2);

    err = write_sysctl(b"/proc/sys/net/ipv4/conf/veth1/forwarding\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char);
    if !ASSERT_OK(err, b"write_sysctl(net.ipv4.conf.veth1.forwarding)\0".as_ptr() as *const c_char) { return -1; }
    err = write_sysctl(b"/proc/sys/net/ipv6/conf/veth1/forwarding\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char);
    if !ASSERT_OK(err, b"write_sysctl(net.ipv6.conf.veth1.forwarding)\0".as_ptr() as *const c_char) { return -1; }

    /* Setup for policy routing tests */
    SYS(fail, b"ip addr add %s/24 dev veth1\0".as_ptr() as *const c_char, IPV4_LOCAL);
    SYS(fail, b"ip addr add %s/64 dev veth1 nodad\0".as_ptr() as *const c_char, IPV6_LOCAL);
    SYS(fail, b"ip route add %s/32 via %s\0".as_ptr() as *const c_char, IPV4_REMOTE_DST, IPV4_GW1);
    SYS(fail, b"ip route add %s/32 via %s table %s\0".as_ptr() as *const c_char, IPV4_REMOTE_DST, IPV4_GW2, MARK_TABLE);
    SYS(fail, b"ip -6 route add %s/128 via %s\0".as_ptr() as *const c_char, IPV6_REMOTE_DST, IPV6_GW1);
    SYS(fail, b"ip -6 route add %s/128 via %s table %s\0".as_ptr() as *const c_char, IPV6_REMOTE_DST, IPV6_GW2, MARK_TABLE);
    SYS(fail, b"ip rule add prio 2 fwmark %d lookup %s\0".as_ptr() as *const c_char, MARK, MARK_TABLE);
    SYS(fail, b"ip -6 rule add prio 2 fwmark %d lookup %s\0".as_ptr() as *const c_char, MARK, MARK_TABLE);

    /*
     * Setup for vlan tests: a subinterface for egress resolution and
     * tag-as-input, a QinQ stack, and an iif rule so the input tests
     * observe which device the lookup used as ingress.
     */
    SYS(fail, b"ip link add link veth1 name %s type vlan id %d\0".as_ptr() as *const c_char, VLAN_IFACE, VLAN_ID as c_int);
    SYS(fail, b"ip link set dev %s up\0".as_ptr() as *const c_char, VLAN_IFACE);
    /*
     * lower than the veth1 parent (1500): the skb-path mtu check uses the
     * FIB result (VLAN) device, so mtu_result is this value, which the
     * no-flag arm below pins
     */
    SYS(fail, b"ip link set dev %s mtu 1400\0".as_ptr() as *const c_char, VLAN_IFACE);
    SYS(fail, b"ip addr add %s/24 dev %s\0".as_ptr() as *const c_char, IPV4_VLAN_IFACE_ADDR, VLAN_IFACE);
    SYS(fail, b"ip addr add %s/64 dev %s nodad\0".as_ptr() as *const c_char, IPV6_VLAN_IFACE_ADDR, VLAN_IFACE);

    /*
     * stays down: the input flag must treat its tag the way real
     * ingress treats a frame arriving on a down VLAN device (drop)
     */
    SYS(fail, b"ip link add link veth1 name %s type vlan id %d\0".as_ptr() as *const c_char, VLAN_IFACE_DOWN, VLAN_ID_DOWN as c_int);

    err = write_sysctl(b"/proc/sys/net/ipv4/conf/veth1.100/forwarding\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char);
    if !ASSERT_OK(err, b"write_sysctl(net.ipv4.conf.veth1.100.forwarding)\0".as_ptr() as *const c_char) { return -1; }
    err = write_sysctl(b"/proc/sys/net/ipv6/conf/veth1.100/forwarding\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char);
    if !ASSERT_OK(err, b"write_sysctl(net.ipv6.conf.veth1.100.forwarding)\0".as_ptr() as *const c_char) { return -1; }

    SYS(fail, b"ip link add link veth1 name %s type vlan proto 802.1ad id 200\0".as_ptr() as *const c_char, QINQ_OUTER_IFACE);
    SYS(fail, b"ip link add link %s name %s type vlan id 300\0".as_ptr() as *const c_char, QINQ_OUTER_IFACE, QINQ_INNER_IFACE);
    SYS(fail, b"ip link set dev %s up\0".as_ptr() as *const c_char, QINQ_OUTER_IFACE);
    SYS(fail, b"ip link set dev %s up\0".as_ptr() as *const c_char, QINQ_INNER_IFACE);
    SYS(fail, b"ip route add %s/32 dev %s\0".as_ptr() as *const c_char, IPV4_QINQ_DST, QINQ_INNER_IFACE);
    SYS(fail, b"ip route add %s/32 via %s\0".as_ptr() as *const c_char, IPV4_VLAN_DST, IPV4_GW1);
    SYS(fail, b"ip route add table %s %s/32 via %s\0".as_ptr() as *const c_char, VLAN_TABLE, IPV4_VLAN_DST, IPV4_VLAN_GW);
    SYS(fail, b"ip rule add prio 3 iif %s lookup %s\0".as_ptr() as *const c_char, VLAN_IFACE, VLAN_TABLE);
    SYS(fail, b"ip -6 route add %s/128 via %s\0".as_ptr() as *const c_char, IPV6_VLAN_DST, IPV6_GW1);
    SYS(fail, b"ip -6 route add table %s %s/128 via %s\0".as_ptr() as *const c_char, VLAN_TABLE, IPV6_VLAN_DST, IPV6_VLAN_GW);
    SYS(fail, b"ip -6 rule add prio 3 iif %s lookup %s\0".as_ptr() as *const c_char, VLAN_IFACE, VLAN_TABLE);

    /* a bond with one port and a VLAN on the bond */
    SYS(fail, b"ip link add %s type bond\0".as_ptr() as *const c_char, BOND_IFACE);
    SYS(fail, b"ip link add %s type veth peer name %s\0".as_ptr() as *const c_char, BOND_PORT, BOND_PORT_PEER);
    SYS(fail, b"ip link set %s master %s\0".as_ptr() as *const c_char, BOND_PORT, BOND_IFACE);
    SYS(fail, b"ip link set dev %s up\0".as_ptr() as *const c_char, BOND_IFACE);
    SYS(fail, b"ip link set dev %s up\0".as_ptr() as *const c_char, BOND_PORT);
    SYS(fail, b"ip link add link %s name %s.%d type vlan id %d\0".as_ptr() as *const c_char, BOND_IFACE, BOND_IFACE, BOND_VLAN_ID as c_int, BOND_VLAN_ID as c_int);
    SYS(fail, b"ip link set dev %s.%d up\0".as_ptr() as *const c_char, BOND_IFACE, BOND_VLAN_ID as c_int);
    SYS(fail, b"ip route add %s/32 dev %s.%d\0".as_ptr() as *const c_char, IPV4_BOND_VLAN_DST, BOND_IFACE, BOND_VLAN_ID as c_int);

    /*
     * a VRF with its own dedicated subinterface (the iif rules above
     * must not see it), for the table-selection-by-ingress cases
     */
    SYS(fail, b"ip link add %s type vrf table %s\0".as_ptr() as *const c_char, VRF_IFACE, VRF_TABLE);
    SYS(fail, b"ip link set dev %s up\0".as_ptr() as *const c_char, VRF_IFACE);
    SYS(fail, b"ip link add link veth1 name %s type vlan id %d\0".as_ptr() as *const c_char, VRF_VLAN_IFACE, VRF_VLAN_ID as c_int);
    SYS(fail, b"ip link set %s master %s\0".as_ptr() as *const c_char, VRF_VLAN_IFACE, VRF_IFACE);
    SYS(fail, b"ip link set dev %s up\0".as_ptr() as *const c_char, VRF_VLAN_IFACE);
    SYS(fail, b"ip addr add %s/24 dev %s\0".as_ptr() as *const c_char, IPV4_VRF_IFACE_ADDR, VRF_VLAN_IFACE);
    err = write_sysctl(b"/proc/sys/net/ipv4/conf/veth1.101/forwarding\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char);
    if !ASSERT_OK(err, b"write_sysctl(net.ipv4.conf.veth1.101.forwarding)\0".as_ptr() as *const c_char) { return -1; }
    SYS(fail, b"ip route add %s/32 via %s\0".as_ptr() as *const c_char, IPV4_VRF_DST, IPV4_GW1);
    SYS(fail, b"ip route add table %s %s/32 via %s\0".as_ptr() as *const c_char, VRF_TABLE, IPV4_VRF_DST, IPV4_VRF_GW);

    /* neighbours on the VLAN subinterface for the non-SKIP_NEIGH cases */
    err = write_sysctl(b"/proc/sys/net/ipv4/neigh/veth1.100/gc_stale_time\0".as_ptr() as *const c_char, b"900\0".as_ptr() as *const c_char);
    if !ASSERT_OK(err, b"write_sysctl(net.ipv4.neigh.veth1.100.gc_stale_time)\0".as_ptr() as *const c_char) { return -1; }
    SYS(fail, b"ip neigh add %s dev %s lladdr %s nud stale\0".as_ptr() as *const c_char, IPV4_VLAN_EGRESS_DST, VLAN_IFACE, DMAC);
    SYS(fail, b"ip neigh add %s dev %s lladdr %s nud stale\0".as_ptr() as *const c_char, IPV4_VLAN_GW, VLAN_IFACE, DMAC2);

    /* a VLAN on veth2 with a route in the tbid test table */
    SYS(fail, b"ip link add link veth2 name %s type vlan id %d\0".as_ptr() as *const c_char, TBID_VLAN_IFACE, TBID_VLAN_ID as c_int);
    SYS(fail, b"ip link set dev %s up\0".as_ptr() as *const c_char, TBID_VLAN_IFACE);
    SYS(fail, b"ip route add table 100 %s/32 dev %s\0".as_ptr() as *const c_char, IPV4_TBID_VLAN_DST, TBID_VLAN_IFACE);

    /* a locked-mtu route via the subinterface for the FRAG_NEEDED case */
    SYS(fail, b"ip route add %s/32 dev %s mtu lock 1000\0".as_ptr() as *const c_char, IPV4_VLAN_MTU_DST, VLAN_IFACE);

    0
}

unsafe fn set_lookup_params(params: *mut bpf_fib_lookup, test: *const fib_lookup_test, ifindex: c_int) -> c_int {
    let mut ret: c_int;

    memset(params as *mut c_void, 0, size_of::<bpf_fib_lookup>());

    (*params).l4_protocol = IPPROTO_TCP;
    (*params).ifindex = if !(*test).iif.is_null() { if_nametoindex((*test).iif) } else { ifindex as u32 };
    (*params).tbid = (*test).tbid;
    (*params).mark = (*test).mark;
    (*params).tot_len = (*test).tot_len;

    /* h_vlan_proto/h_vlan_TCI union with tbid */
    if ((*test).lookup_flags & BPF_FIB_LOOKUP_VLAN_INPUT) != 0 {
        (*params).h_vlan_proto = htons((*test).vlan_proto);
        (*params).h_vlan_TCI = htons((*test).vlan_id);
    }

    if inet_pton(AF_INET6, (*test).daddr, (*params).ipv6_dst.as_mut_ptr() as *mut c_void) == 1 {
        (*params).family = AF_INET6 as u8;
        if ((*test).lookup_flags & BPF_FIB_LOOKUP_SRC) == 0 {
            ret = inet_pton(AF_INET6, IPV6_IFACE_ADDR, (*params).ipv6_src.as_mut_ptr() as *mut c_void);
            if !ASSERT_EQ(ret, 1, b"inet_pton(IPV6_IFACE_ADDR)\0".as_ptr() as *const c_char) {
                return -1;
            }
        }
        return 0;
    }

    ret = inet_pton(AF_INET, (*test).daddr, (*params).ipv6_dst.as_mut_ptr() as *mut c_void);
    if !ASSERT_EQ(ret, 1, b"convert IP[46] address\0".as_ptr() as *const c_char) {
        return -1;
    }
    (*params).family = AF_INET as u8;

    if ((*test).lookup_flags & BPF_FIB_LOOKUP_SRC) == 0 {
        ret = inet_pton(AF_INET, IPV4_IFACE_ADDR, (*params).ipv6_src.as_mut_ptr() as *mut c_void);
        if !ASSERT_EQ(ret, 1, b"inet_pton(IPV4_IFACE_ADDR)\0".as_ptr() as *const c_char) {
            return -1;
        }
    }

    0
}

unsafe fn mac_str(b: *mut c_char, mac: *const u8) {
    sprintf(
        b,
        b"%02X:%02X:%02X:%02X:%02X:%02X\0".as_ptr() as *const c_char,
        *mac.add(0) as c_int,
        *mac.add(1) as c_int,
        *mac.add(2) as c_int,
        *mac.add(3) as c_int,
        *mac.add(4) as c_int,
        *mac.add(5) as c_int,
    );
}

unsafe fn assert_ip_address(family: c_int, addr: *mut c_void, expected_str: *const c_char) {
    let mut str_buf = [0 as c_char; INET6_ADDRSTRLEN];
    let mut expected_addr = [0u8; 16];
    let mut addr_len: c_int = 0;
    let ret: c_int;

    match family {
        AF_INET6 => {
            ret = inet_pton(AF_INET6, expected_str, expected_addr.as_mut_ptr() as *mut c_void);
            ASSERT_EQ(ret, 1, b"inet_pton(AF_INET6, expected_str)\0".as_ptr() as *const c_char);
            addr_len = 16;
        }
        AF_INET => {
            ret = inet_pton(AF_INET, expected_str, expected_addr.as_mut_ptr() as *mut c_void);
            ASSERT_EQ(ret, 1, b"inet_pton(AF_INET, expected_str)\0".as_ptr() as *const c_char);
            addr_len = 4;
        }
        _ => {
            PRINT_FAIL(b"invalid address family: %d\0".as_ptr() as *const c_char, family);
        }
    }

    if memcmp(addr, expected_addr.as_ptr() as *const c_void, addr_len as usize) != 0 {
        inet_ntop(family, addr, str_buf.as_mut_ptr(), str_buf.len() as u32);
        PRINT_FAIL(b"expected %s actual %s \0".as_ptr() as *const c_char, expected_str, str_buf.as_ptr());
    }
}

unsafe fn assert_src_ip(params: *mut bpf_fib_lookup, expected: *const c_char) {
    assert_ip_address((*params).family as c_int, (*params).ipv6_src.as_mut_ptr() as *mut c_void, expected);
}

unsafe fn assert_dst_ip(params: *mut bpf_fib_lookup, expected: *const c_char) {
    assert_ip_address((*params).family as c_int, (*params).ipv6_dst.as_mut_ptr() as *mut c_void, expected);
}

#[no_mangle]
pub unsafe extern "C" fn test_fib_lookup() {
    let mut fib_params: *mut bpf_fib_lookup;
    let mut nstoken: *mut nstoken = ptr::null_mut();
    let mut skb: __sk_buff = zeroed();
    let skel: *mut fib_lookup;
    let prog_fd: c_int;
    let xdp_fd: c_int;
    let mut err: c_int;
    let mut ret: c_int;
    let fail = b"fail\0".as_ptr() as *const c_char;

    /* The test does not use the skb->data, so
     * use pkt_v6 for both v6 and v4 test.
     */
    let mut run_opts = bpf_test_run_opts {
        data_in: (&pkt_v6 as *const [u8; 0]) as *const c_void,
        data_size_in: size_of_val(&pkt_v6) as u32,
        ctx_in: (&mut skb as *mut __sk_buff) as *mut c_void,
        ctx_size_in: size_of::<__sk_buff>() as u32,
        flags: 0,
        repeat: 0,
    };
    let mut xdp_opts = bpf_test_run_opts {
        data_in: (&pkt_v6 as *const [u8; 0]) as *const c_void,
        data_size_in: size_of_val(&pkt_v6) as u32,
        ctx_in: ptr::null_mut(),
        ctx_size_in: 0,
        flags: 0,
        repeat: 0,
    };

    skel = fib_lookup__open_and_load();
    if !ASSERT_OK_PTR(skel, b"skel open_and_load\0".as_ptr() as *const c_char) {
        return;
    }
    prog_fd = bpf_program__fd((*skel).progs.fib_lookup);
    xdp_fd = bpf_program__fd((*skel).progs.fib_lookup_xdp);

    SYS(fail, b"ip netns add %s\0".as_ptr() as *const c_char, NS_TEST);

    nstoken = open_netns(NS_TEST);
    if !ASSERT_OK_PTR(nstoken, b"open_netns\0".as_ptr() as *const c_char) {
        goto_fail_fib_lookup(nstoken, skel);
        return;
    }

    if setup_netns() != 0 {
        goto_fail_fib_lookup(nstoken, skel);
        return;
    }

    skb.ifindex = if_nametoindex(b"veth1\0".as_ptr() as *const c_char);
    if !ASSERT_NEQ(skb.ifindex, 0, b"if_nametoindex(veth1)\0".as_ptr() as *const c_char) {
        goto_fail_fib_lookup(nstoken, skel);
        return;
    }

    fib_params = &mut (*(*skel).bss).fib_params;

    for i in 0..tests.len() {
        printf(b"Testing %s \0".as_ptr() as *const c_char, tests[i].desc);

        if set_lookup_params(fib_params, &tests[i], skb.ifindex as c_int) != 0 {
            continue;
        }

        (*(*skel).bss).fib_lookup_ret = -1;
        (*(*skel).bss).lookup_flags = tests[i].lookup_flags;

        err = bpf_prog_test_run_opts(prog_fd, &mut run_opts);
        if !ASSERT_OK(err, b"bpf_prog_test_run_opts\0".as_ptr() as *const c_char) {
            continue;
        }

        /*
         * BPF_FIB_LOOKUP_VLAN is XDP-only; the tc helper rejects it.
         * These cases are exercised on the XDP path below.
         */
        if (tests[i].lookup_flags & BPF_FIB_LOOKUP_VLAN) != 0 {
            ASSERT_EQ((*(*skel).bss).fib_lookup_ret, -EINVAL, b"tc rejects BPF_FIB_LOOKUP_VLAN\0".as_ptr() as *const c_char);
            continue;
        }

        ASSERT_EQ((*(*skel).bss).fib_lookup_ret, tests[i].expected_ret, b"fib_lookup_ret\0".as_ptr() as *const c_char);

        if !tests[i].expected_src.is_null() {
            assert_src_ip(fib_params, tests[i].expected_src);
        }
        if !tests[i].expected_dst.is_null() {
            assert_dst_ip(fib_params, tests[i].expected_dst);
        }
        if !tests[i].expected_dev.is_null() {
            ASSERT_EQ((*fib_params).ifindex, if_nametoindex(tests[i].expected_dev), b"ifindex\0".as_ptr() as *const c_char);
        }
        if tests[i].expected_mtu != 0 {
            ASSERT_EQ((*fib_params).mtu_result, tests[i].expected_mtu, b"mtu_result\0".as_ptr() as *const c_char);
        }
        if tests[i].check_vlan {
            ASSERT_EQ((*fib_params).h_vlan_proto, htons(tests[i].vlan_proto), b"h_vlan_proto\0".as_ptr() as *const c_char);
            ASSERT_EQ((*fib_params).h_vlan_TCI, htons(tests[i].vlan_id), b"h_vlan_TCI\0".as_ptr() as *const c_char);
        }

        ret = memcmp(tests[i].dmac.as_ptr() as *const c_void, (*fib_params).dmac.as_ptr() as *const c_void, size_of::<[u8; 6]>());
        if !ASSERT_EQ(ret, 0, b"dmac not match\0".as_ptr() as *const c_char) {
            let mut expected = [0 as c_char; 18];
            let mut actual = [0 as c_char; 18];
            mac_str(expected.as_mut_ptr(), tests[i].dmac.as_ptr());
            mac_str(actual.as_mut_ptr(), (*fib_params).dmac.as_ptr());
            printf(b"dmac expected %s actual %s \0".as_ptr() as *const c_char, expected.as_ptr(), actual.as_ptr());
        }

        /*
         * ensure tbid is zero'd out after fib lookup. With
         * BPF_FIB_LOOKUP_VLAN the union holds the packed vlan
         * fields instead, so skip the check for those.
         */
        if (tests[i].lookup_flags & BPF_FIB_LOOKUP_DIRECT) != 0 &&
           (tests[i].lookup_flags & BPF_FIB_LOOKUP_VLAN) == 0 {
            if !ASSERT_EQ((*(*skel).bss).fib_params.tbid, 0, b"expected fib_params.tbid to be zero\0".as_ptr() as *const c_char) {
                goto_fail_fib_lookup(nstoken, skel);
                return;
            }
        }
    }

    /*
     * Re-run the cases through bpf_xdp_fib_lookup(). test_run uses the
     * current netns' loopback for ctx->rxq->dev, so dev_net() is NS_TEST
     * and the lookup runs against its FIB. The path-independent results
     * (return code, swapped ifindex, vlan tag, gateway) must match the skb
     * path; the no-tot_len mtu_result is skb-specific and not rechecked.
     */
    for i in 0..tests.len() {
        if set_lookup_params(fib_params, &tests[i], skb.ifindex as c_int) != 0 {
            continue;
        }

        (*(*skel).bss).fib_lookup_ret = -1;
        (*(*skel).bss).lookup_flags = tests[i].lookup_flags;

        err = bpf_prog_test_run_opts(xdp_fd, &mut xdp_opts);
        if !ASSERT_OK(err, b"xdp test_run\0".as_ptr() as *const c_char) {
            continue;
        }

        if !ASSERT_EQ((*(*skel).bss).fib_lookup_ret, tests[i].expected_ret, b"xdp fib_lookup_ret\0".as_ptr() as *const c_char) {
            printf(b"(xdp) %s\n\0".as_ptr() as *const c_char, tests[i].desc);
        }

        if !tests[i].expected_dev.is_null() {
            ASSERT_EQ((*fib_params).ifindex, if_nametoindex(tests[i].expected_dev), b"xdp ifindex\0".as_ptr() as *const c_char);
        }
        if !tests[i].expected_dst.is_null() {
            assert_dst_ip(fib_params, tests[i].expected_dst);
        }
        if tests[i].check_vlan {
            ASSERT_EQ((*fib_params).h_vlan_proto, htons(tests[i].vlan_proto), b"xdp h_vlan_proto\0".as_ptr() as *const c_char);
            ASSERT_EQ((*fib_params).h_vlan_TCI, htons(tests[i].vlan_id), b"xdp h_vlan_TCI\0".as_ptr() as *const c_char);
        }

        ret = memcmp(tests[i].dmac.as_ptr() as *const c_void, (*fib_params).dmac.as_ptr() as *const c_void, size_of::<[u8; 6]>());
        ASSERT_EQ(ret, 0, b"xdp dmac\0".as_ptr() as *const c_char);

        /*
         * mtu_result from a tot_len lookup is the route mtu and is
         * path-independent; the no-tot_len arm reads dev->mtu and is
         * skb-only, so gate on tot_len
         */
        if tests[i].expected_mtu != 0 && tests[i].tot_len != 0 {
            ASSERT_EQ((*fib_params).mtu_result, tests[i].expected_mtu, b"xdp mtu_result\0".as_ptr() as *const c_char);
        }
    }

    goto_fail_fib_lookup(nstoken, skel);
}

unsafe fn goto_fail_fib_lookup(nstoken: *mut nstoken, skel: *mut fib_lookup) {
    if !nstoken.is_null() {
        close_netns(nstoken);
    }
    SYS_NOFAIL(b"ip netns del fib_lookup_ns\0".as_ptr() as *const c_char);
    fib_lookup__destroy(skel);
}

const NS_VLAN_A: *const c_char = b"fib_lookup_vlan_ns_a\0".as_ptr() as *const c_char;
const NS_VLAN_B: *const c_char = b"fib_lookup_vlan_ns_b\0".as_ptr() as *const c_char;
const IPV4_VLAN_NETNS_ADDR: *const c_char = b"10.66.0.1\0".as_ptr() as *const c_char;
const IPV4_VLAN_NETNS_DST: *const c_char = b"10.66.0.2\0".as_ptr() as *const c_char;

/*
 * A VLAN device can be moved to another netns while staying registered
 * on its parent. Neither direction may then cross the boundary: the
 * egress flag must not publish the foreign parent's ifindex, and the
 * input flag must fail closed rather than use a foreign ingress.
 */
#[no_mangle]
pub unsafe extern "C" fn test_fib_lookup_vlan_netns() {
    let fib_params: *mut bpf_fib_lookup;
    let mut nstoken: *mut nstoken = ptr::null_mut();
    let mut skb: __sk_buff = zeroed();
    let mut skel: *mut fib_lookup = ptr::null_mut();
    let prog_fd: c_int;
    let xdp_fd: c_int;
    let mut err: c_int;
    let parent_idx: c_int;
    let vlan_idx: c_int;
    let fail = b"fail\0".as_ptr() as *const c_char;

    let mut run_opts = bpf_test_run_opts {
        data_in: (&pkt_v6 as *const [u8; 0]) as *const c_void,
        data_size_in: size_of_val(&pkt_v6) as u32,
        ctx_in: (&mut skb as *mut __sk_buff) as *mut c_void,
        ctx_size_in: size_of::<__sk_buff>() as u32,
        flags: 0,
        repeat: 0,
    };
    let mut xdp_opts = bpf_test_run_opts {
        data_in: (&pkt_v6 as *const [u8; 0]) as *const c_void,
        data_size_in: size_of_val(&pkt_v6) as u32,
        ctx_in: ptr::null_mut(),
        ctx_size_in: 0,
        flags: 0,
        repeat: 0,
    };

    skel = fib_lookup__open_and_load();
    if !ASSERT_OK_PTR(skel, b"skel open_and_load\0".as_ptr() as *const c_char) { return; }
    prog_fd = bpf_program__fd((*skel).progs.fib_lookup);
    xdp_fd = bpf_program__fd((*skel).progs.fib_lookup_xdp);
    fib_params = &mut (*(*skel).bss).fib_params;

    SYS(fail, b"ip netns add %s\0".as_ptr() as *const c_char, NS_VLAN_A);
    SYS(fail, b"ip netns add %s\0".as_ptr() as *const c_char, NS_VLAN_B);

    nstoken = open_netns(NS_VLAN_A);
    if !ASSERT_OK_PTR(nstoken, b"open_netns(a)\0".as_ptr() as *const c_char) { goto_fail_vlan_netns(nstoken, skel); return; }

    SYS(fail, b"ip link add veth7 type veth peer name veth8\0".as_ptr() as *const c_char);
    SYS(fail, b"ip link set dev veth7 up\0".as_ptr() as *const c_char);
    SYS(fail, b"ip link add link veth7 name veth7.66 type vlan id 66\0".as_ptr() as *const c_char);
    SYS(fail, b"ip link set veth7.66 netns %s\0".as_ptr() as *const c_char, NS_VLAN_B);
    /*
     * up it in B before the input lookup: the move closed it, and a
     * down device fails the resolver on IFF_UP before reaching the
     * netns check this subtest exists to pin
     */
    SYS(fail, b"ip -n %s link set dev veth7.66 up\0".as_ptr() as *const c_char, NS_VLAN_B);

    parent_idx = if_nametoindex(b"veth7\0".as_ptr() as *const c_char) as c_int;
    if !ASSERT_NEQ(parent_idx, 0, b"if_nametoindex(veth7)\0".as_ptr() as *const c_char) { goto_fail_vlan_netns(nstoken, skel); return; }

    /*
     * give this netns a route to the destination: the lookup below runs
     * against this FIB, so without the route a kernel that resolved the
     * moved device anyway would still return NOT_FWDED and the arm would
     * pass for the wrong reason
     */
    SYS(fail, b"ip route add %s/32 dev veth7\0".as_ptr() as *const c_char, IPV4_VLAN_NETNS_DST);

    /*
     * input: the moved device is still in veth7's VLAN group, but it
     * lives in another netns, so the lookup must fail closed
     */
    skb.ifindex = parent_idx as u32;
    memset(fib_params as *mut c_void, 0, size_of::<bpf_fib_lookup>());
    (*fib_params).family = AF_INET as u8;
    (*fib_params).l4_protocol = IPPROTO_TCP;
    (*fib_params).ifindex = parent_idx as u32;
    (*fib_params).h_vlan_proto = htons(ETH_P_8021Q);
    (*fib_params).h_vlan_TCI = htons(66);
    if !ASSERT_EQ(inet_pton(AF_INET, IPV4_VLAN_NETNS_DST, (*fib_params).ipv6_dst.as_mut_ptr() as *mut c_void), 1, b"inet_pton(dst)\0".as_ptr() as *const c_char) {
        goto_fail_vlan_netns(nstoken, skel);
        return;
    }

    (*(*skel).bss).fib_lookup_ret = -1;
    (*(*skel).bss).lookup_flags = BPF_FIB_LOOKUP_VLAN_INPUT | BPF_FIB_LOOKUP_SKIP_NEIGH;
    err = bpf_prog_test_run_opts(prog_fd, &mut run_opts);
    if !ASSERT_OK(err, b"test_run(input)\0".as_ptr() as *const c_char) { goto_fail_vlan_netns(nstoken, skel); return; }
    ASSERT_EQ((*(*skel).bss).fib_lookup_ret, BPF_FIB_LKUP_RET_NOT_FWDED, b"input across netns fails closed\0".as_ptr() as *const c_char);
    ASSERT_EQ((*fib_params).ifindex, parent_idx as u32, b"ifindex untouched\0".as_ptr() as *const c_char);
    ASSERT_EQ((*fib_params).h_vlan_TCI, htons(66), b"tag untouched\0".as_ptr() as *const c_char);

    close_netns(nstoken);
    nstoken = open_netns(NS_VLAN_B);
    if !ASSERT_OK_PTR(nstoken, b"open_netns(b)\0".as_ptr() as *const c_char) { goto_fail_vlan_netns(nstoken, skel); return; }

    /*
     * egress: the fib result is the VLAN device here, but its parent
     * is in the other netns, so the swap must not happen
     */
    SYS(fail, b"ip addr add %s/24 dev veth7.66\0".as_ptr() as *const c_char, IPV4_VLAN_NETNS_ADDR);
    err = write_sysctl(b"/proc/sys/net/ipv4/conf/veth7.66/forwarding\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char);
    if !ASSERT_OK(err, b"write_sysctl(forwarding)\0".as_ptr() as *const c_char) { goto_fail_vlan_netns(nstoken, skel); return; }

    vlan_idx = if_nametoindex(b"veth7.66\0".as_ptr() as *const c_char) as c_int;
    if !ASSERT_NEQ(vlan_idx, 0, b"if_nametoindex(veth7.66)\0".as_ptr() as *const c_char) { goto_fail_vlan_netns(nstoken, skel); return; }

    memset(fib_params as *mut c_void, 0, size_of::<bpf_fib_lookup>());
    (*fib_params).family = AF_INET as u8;
    (*fib_params).l4_protocol = IPPROTO_TCP;
    (*fib_params).ifindex = vlan_idx as u32;
    if !ASSERT_EQ(inet_pton(AF_INET, IPV4_VLAN_NETNS_DST, (*fib_params).ipv6_dst.as_mut_ptr() as *mut c_void), 1, b"inet_pton(dst)\0".as_ptr() as *const c_char) ||
       !ASSERT_EQ(inet_pton(AF_INET, IPV4_VLAN_NETNS_ADDR, (*fib_params).ipv6_src.as_mut_ptr() as *mut c_void), 1, b"inet_pton(src)\0".as_ptr() as *const c_char) {
        goto_fail_vlan_netns(nstoken, skel);
        return;
    }

    (*(*skel).bss).fib_lookup_ret = -1;
    (*(*skel).bss).lookup_flags = BPF_FIB_LOOKUP_VLAN | BPF_FIB_LOOKUP_SKIP_NEIGH;
    err = bpf_prog_test_run_opts(xdp_fd, &mut xdp_opts);
    if !ASSERT_OK(err, b"test_run(egress)\0".as_ptr() as *const c_char) { goto_fail_vlan_netns(nstoken, skel); return; }
    ASSERT_EQ((*(*skel).bss).fib_lookup_ret, BPF_FIB_LKUP_RET_VLAN_FAILURE, b"egress returns VLAN_FAILURE\0".as_ptr() as *const c_char);
    ASSERT_EQ((*fib_params).ifindex, vlan_idx as u32, b"foreign parent not published\0".as_ptr() as *const c_char);
    ASSERT_EQ((*fib_params).h_vlan_TCI, 0, b"vlan fields zero\0".as_ptr() as *const c_char);

    goto_fail_vlan_netns(nstoken, skel);
}

unsafe fn goto_fail_vlan_netns(nstoken: *mut nstoken, skel: *mut fib_lookup) {
    if !nstoken.is_null() {
        close_netns(nstoken);
    }
    SYS_NOFAIL(b"ip netns del fib_lookup_vlan_ns_a\0".as_ptr() as *const c_char);
    SYS_NOFAIL(b"ip netns del fib_lookup_vlan_ns_b\0".as_ptr() as *const c_char);
    fib_lookup__destroy(skel);
}

const REDIRECT_NPKTS: c_int = 1000;
const NS_REDIRECT: *const c_char = b"fib_lookup_redirect_ns\0".as_ptr() as *const c_char;

/*
 * The egress flag exists so an XDP program can redirect to the physical
 * parent. A redirect that lands on a VLAN device is dropped at
 * xdp_do_flush(), because a VLAN device has no ndo_xdp_xmit. Drive real
 * frames with BPF_F_TEST_XDP_LIVE_FRAMES, which runs the native
 * xdp_do_redirect() + xdp_do_flush() path: a reducible VLAN egress
 * resolves to veth1 and is delivered to its peer veth2, while a QinQ
 * egress returns VLAN_FAILURE and is passed to the stack instead of
 * redirected to a device that would silently drop it.
 */
#[no_mangle]
pub unsafe extern "C" fn test_fib_lookup_vlan_redirect() {
    let redirect_fd: c_int;
    let mut err: c_int;
    let veth1_idx: c_int;
    let mut veth2_idx: c_int = -1;
    let fib_params: *mut bpf_fib_lookup;
    let mut nstoken: *mut nstoken = ptr::null_mut();
    let mut skel: *mut fib_lookup = ptr::null_mut();
    let mut xdp_attached = false;
    let fail = b"fail\0".as_ptr() as *const c_char;

    let mut lf_opts = bpf_test_run_opts {
        data_in: (&pkt_v4 as *const [u8; 0]) as *const c_void,
        data_size_in: size_of_val(&pkt_v4) as u32,
        ctx_in: ptr::null_mut(),
        ctx_size_in: 0,
        flags: BPF_F_TEST_XDP_LIVE_FRAMES,
        repeat: REDIRECT_NPKTS as u32,
    };

    skel = fib_lookup__open_and_load();
    if !ASSERT_OK_PTR(skel, b"skel open_and_load\0".as_ptr() as *const c_char) { return; }
    redirect_fd = bpf_program__fd((*skel).progs.fib_lookup_redirect);
    fib_params = &mut (*(*skel).bss).fib_params;

    SYS(fail, b"ip netns add %s\0".as_ptr() as *const c_char, NS_REDIRECT);
    nstoken = open_netns(NS_REDIRECT);
    if !ASSERT_OK_PTR(nstoken, b"open_netns\0".as_ptr() as *const c_char) { goto_fail_redirect(xdp_attached, veth2_idx, nstoken, skel); return; }
    if setup_netns() != 0 { goto_fail_redirect(xdp_attached, veth2_idx, nstoken, skel); return; }

    veth1_idx = if_nametoindex(b"veth1\0".as_ptr() as *const c_char) as c_int;
    veth2_idx = if_nametoindex(b"veth2\0".as_ptr() as *const c_char) as c_int;
    if !ASSERT_NEQ(veth1_idx, 0, b"if_nametoindex(veth1)\0".as_ptr() as *const c_char) ||
       !ASSERT_NEQ(veth2_idx, 0, b"if_nametoindex(veth2)\0".as_ptr() as *const c_char) {
        goto_fail_redirect(xdp_attached, veth2_idx, nstoken, skel);
        return;
    }

    /*
     * A redirect to veth1 is delivered to its peer veth2. veth_xdp_xmit()
     * only accepts the frame if veth2's NAPI is up, which on veth means
     * veth2 carries an XDP program; xdp_count tallies what arrives.
     */
    err = bpf_xdp_attach(veth2_idx, bpf_program__fd((*skel).progs.xdp_count), XDP_FLAGS_DRV_MODE, ptr::null());
    if !ASSERT_OK(err, b"attach xdp_count on veth2\0".as_ptr() as *const c_char) { goto_fail_redirect(xdp_attached, veth2_idx, nstoken, skel); return; }
    xdp_attached = true;

    /* reducible VLAN egress: resolves to the physical parent veth1 */
    memset(fib_params as *mut c_void, 0, size_of::<bpf_fib_lookup>());
    (*fib_params).family = AF_INET as u8;
    (*fib_params).l4_protocol = IPPROTO_TCP;
    (*fib_params).ifindex = veth1_idx as u32;
    if !ASSERT_EQ(inet_pton(AF_INET, IPV4_IFACE_ADDR, (*fib_params).ipv6_src.as_mut_ptr() as *mut c_void), 1, b"inet_pton(src)\0".as_ptr() as *const c_char) ||
       !ASSERT_EQ(inet_pton(AF_INET, IPV4_VLAN_EGRESS_DST, (*fib_params).ipv6_dst.as_mut_ptr() as *mut c_void), 1, b"inet_pton(reducible dst)\0".as_ptr() as *const c_char) {
        goto_fail_redirect(xdp_attached, veth2_idx, nstoken, skel);
        return;
    }
    (*(*skel).bss).lookup_flags = BPF_FIB_LOOKUP_VLAN | BPF_FIB_LOOKUP_SKIP_NEIGH;
    (*(*skel).bss).redirected = 0;
    (*(*skel).bss).passed = 0;
    (*(*skel).bss).delivered = 0;

    err = bpf_prog_test_run_opts(redirect_fd, &mut lf_opts);
    if !ASSERT_OK(err, b"test_run(reducible egress)\0".as_ptr() as *const c_char) { goto_fail_redirect(xdp_attached, veth2_idx, nstoken, skel); return; }
    ASSERT_EQ((*(*skel).bss).redirected, REDIRECT_NPKTS, b"reducible egress redirected\0".as_ptr() as *const c_char);
    ASSERT_EQ((*(*skel).bss).passed, 0, b"reducible egress not passed\0".as_ptr() as *const c_char);
    ASSERT_GT((*(*skel).bss).delivered, 0, b"reducible egress delivered to veth2\0".as_ptr() as *const c_char);

    /*
     * QinQ egress: not reducible, so the lookup returns VLAN_FAILURE and
     * the program passes the frame instead of redirecting to the inner
     * VLAN device. redirected == 0 is the assertion that matters: the
     * program did not redirect to a device that would drop the frame at
     * xdp_do_flush(). veth2's delivered count is not checked here, since
     * a passed frame can still reach veth2 through the stack's forwarding
     * path, which is unrelated to the redirect under test.
     */
    memset(fib_params as *mut c_void, 0, size_of::<bpf_fib_lookup>());
    (*fib_params).family = AF_INET as u8;
    (*fib_params).l4_protocol = IPPROTO_TCP;
    (*fib_params).ifindex = veth1_idx as u32;
    if !ASSERT_EQ(inet_pton(AF_INET, IPV4_IFACE_ADDR, (*fib_params).ipv6_src.as_mut_ptr() as *mut c_void), 1, b"inet_pton(src)\0".as_ptr() as *const c_char) ||
       !ASSERT_EQ(inet_pton(AF_INET, IPV4_QINQ_DST, (*fib_params).ipv6_dst.as_mut_ptr() as *mut c_void), 1, b"inet_pton(qinq dst)\0".as_ptr() as *const c_char) {
        goto_fail_redirect(xdp_attached, veth2_idx, nstoken, skel);
        return;
    }
    (*(*skel).bss).lookup_flags = BPF_FIB_LOOKUP_VLAN | BPF_FIB_LOOKUP_SKIP_NEIGH;
    (*(*skel).bss).redirected = 0;
    (*(*skel).bss).passed = 0;

    err = bpf_prog_test_run_opts(redirect_fd, &mut lf_opts);
    if !ASSERT_OK(err, b"test_run(qinq egress)\0".as_ptr() as *const c_char) { goto_fail_redirect(xdp_attached, veth2_idx, nstoken, skel); return; }
    ASSERT_EQ((*(*skel).bss).passed, REDIRECT_NPKTS, b"qinq egress passed\0".as_ptr() as *const c_char);
    ASSERT_EQ((*(*skel).bss).redirected, 0, b"qinq egress not redirected\0".as_ptr() as *const c_char);

    goto_fail_redirect(xdp_attached, veth2_idx, nstoken, skel);
}

unsafe fn goto_fail_redirect(xdp_attached: bool, veth2_idx: c_int, nstoken: *mut nstoken, skel: *mut fib_lookup) {
    if xdp_attached {
        bpf_xdp_detach(veth2_idx, XDP_FLAGS_DRV_MODE, ptr::null());
    }
    if !nstoken.is_null() {
        close_netns(nstoken);
    }
    SYS_NOFAIL(b"ip netns del fib_lookup_redirect_ns\0".as_ptr() as *const c_char);
    fib_lookup__destroy(skel);
}

unsafe fn size_of_val<T: ?Sized>(_: &T) -> usize {
    0
}
