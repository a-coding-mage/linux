// SPDX-License-Identifier: GPL-2.0

/**
 * Test XDP bonding support
 *
 * Sets up two bonded veth pairs between two fresh namespaces
 * and verifies that XDP_TX program loaded on a bond device
 * are correctly loaded onto the slave devices and XDP_TX'd
 * packets are balanced using bonding.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, size_of_val, zeroed};
use core::ptr::{null, null_mut};

const CLONE_NEWNET: c_int = 0x40000000;
const O_RDONLY: c_int = 0;
const O_CLOEXEC: c_int = 0o2000000;
const PATH_MAX: usize = 4096;
const AF_PACKET: c_int = 17;
const SOCK_RAW: c_int = 3;
const IPPROTO_RAW: c_int = 255;
const IPPROTO_UDP: c_int = 17;
const ETH_P_IP: u16 = 0x0800;
const ETH_HLEN: usize = 14;
const ETH_ALEN: usize = 6;
const XDP_FLAGS_SKB_MODE: c_uint = 1 << 1;
const XDP_FLAGS_DRV_MODE: c_uint = 1 << 2;
const BPF_F_TEST_XDP_LIVE_FRAMES: u32 = 1 << 0;
const NETDEV_XDP_ACT_BASIC: u64 = 1 << 0;
const NETDEV_XDP_ACT_REDIRECT: u64 = 1 << 1;
const NETDEV_XDP_ACT_NDO_XMIT: u64 = 1 << 2;
const NETDEV_XDP_ACT_RX_SG: u64 = 1 << 3;
const NETDEV_XDP_ACT_NDO_XMIT_SG: u64 = 1 << 4;
const LIBBPF_WARN: libbpf_print_level = 1;

const BOND1_MAC: [u8; ETH_ALEN] = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
const BOND1_MAC_STR: &[u8] = b"00:11:22:33:44:55\0";
const BOND2_MAC: [u8; ETH_ALEN] = [0x00, 0x22, 0x33, 0x44, 0x55, 0x66];
const BOND2_MAC_STR: &[u8] = b"00:22:33:44:55:66\0";
const NPACKETS: c_int = 100;

const MAX_BPF_LINKS: usize = 8;

const BOND_MODE_ROUNDROBIN: c_int = 0;
const BOND_MODE_ACTIVEBACKUP: c_int = 1;
const BOND_MODE_XOR: c_int = 2;
const BOND_MODE_BROADCAST: c_int = 3;
const BOND_MODE_8023AD: c_int = 4;
const BOND_MODE_TLB: c_int = 5;
const BOND_MODE_ALB: c_int = 6;

const BOND_XMIT_POLICY_LAYER2: c_int = 0;
const BOND_XMIT_POLICY_LAYER34: c_int = 1;
const BOND_XMIT_POLICY_LAYER23: c_int = 2;
const BOND_XMIT_POLICY_ENCAP23: c_int = 3;
const BOND_XMIT_POLICY_ENCAP34: c_int = 4;

type va_list = *mut c_void;
type libbpf_print_level = c_int;
type libbpf_print_fn_t = Option<
    unsafe extern "C" fn(level: libbpf_print_level, format: *const c_char, args: va_list) -> c_int,
>;

#[repr(C)]
struct FILE {
    _private: [u8; 0],
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
struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
struct nstoken {
    _private: [u8; 0],
}

#[repr(C)]
struct xdp_dummy_progs {
    xdp_dummy_prog: *mut bpf_program,
}

#[repr(C)]
struct xdp_dummy {
    progs: xdp_dummy_progs,
}

#[repr(C)]
struct xdp_tx_progs {
    xdp_tx: *mut bpf_program,
}

#[repr(C)]
struct xdp_tx {
    progs: xdp_tx_progs,
}

#[repr(C)]
struct xdp_redirect_multi_kern_progs {
    xdp_redirect_map_multi_prog: *mut bpf_program,
}

#[repr(C)]
struct xdp_redirect_multi_kern_maps {
    map_all: *mut bpf_map,
}

#[repr(C)]
struct xdp_redirect_multi_kern {
    progs: xdp_redirect_multi_kern_progs,
    maps: xdp_redirect_multi_kern_maps,
}

#[repr(C)]
struct skeletons {
    xdp_dummy: *mut xdp_dummy,
    xdp_tx: *mut xdp_tx,
    xdp_redirect_multi_kern: *mut xdp_redirect_multi_kern,

    nlinks: c_int,
    links: [*mut bpf_link; MAX_BPF_LINKS],
}

#[repr(C)]
struct ethhdr {
    h_dest: [u8; ETH_ALEN],
    h_source: [u8; ETH_ALEN],
    h_proto: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct iphdr {
    ihl_version: u8,
    tos: u8,
    tot_len: u16,
    id: u16,
    frag_off: u16,
    ttl: u8,
    protocol: u8,
    check: u16,
    saddr: u32,
    daddr: u32,
}

impl iphdr {
    fn set_ihl(&mut self, ihl: u8) {
        self.ihl_version = (self.ihl_version & 0xf0) | (ihl & 0x0f);
    }

    fn set_version(&mut self, version: u8) {
        self.ihl_version = (self.ihl_version & 0x0f) | ((version & 0x0f) << 4);
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct udphdr {
    source: u16,
    dest: u16,
    len: u16,
    check: u16,
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_ll {
    sll_family: u16,
    sll_protocol: u16,
    sll_ifindex: c_int,
    sll_hatype: u16,
    sll_pkttype: u8,
    sll_halen: u8,
    sll_addr: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct xdp_md {
    data: u32,
    data_end: u32,
    data_meta: u32,
    ingress_ifindex: u32,
    rx_queue_index: u32,
    egress_ifindex: u32,
}

#[repr(C)]
struct bpf_test_run_opts {
    sz: usize,
    data_in: *mut c_void,
    data_size_in: u32,
    ctx_in: *mut c_void,
    ctx_size_in: u32,
    flags: u32,
    repeat: u32,
    batch_size: u32,
}

#[repr(C)]
struct bpf_xdp_query_opts {
    sz: usize,
    feature_flags: u64,
}

#[repr(C)]
struct bond_test_case {
    name: *mut c_char,
    mode: c_int,
    xmit_policy: c_int,
}

unsafe extern "C" {
    fn setns(fd: c_int, nstype: c_int) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fclose(stream: *mut FILE) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn sendto(
        sockfd: c_int,
        buf: *const c_void,
        len: usize,
        flags: c_int,
        dest_addr: *const sockaddr,
        addrlen: u32,
    ) -> isize;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn system(command: *const c_char) -> c_int;
    fn abs(j: c_int) -> c_int;
    fn vprintf(format: *const c_char, ap: va_list) -> c_int;

    fn bpf_program__attach_xdp(prog: *mut bpf_program, ifindex: c_int) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64)
        -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_xdp_attach(ifindex: c_int, prog_fd: c_int, flags: c_uint, opts: *const c_void)
        -> c_int;
    fn bpf_xdp_detach(ifindex: c_int, flags: c_uint, opts: *const c_void) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_xdp_query(ifindex: c_int, flags: c_uint, opts: *mut bpf_xdp_query_opts) -> c_int;

    fn xdp_dummy__open_and_load() -> *mut xdp_dummy;
    fn xdp_dummy__destroy(obj: *mut xdp_dummy);
    fn xdp_tx__open_and_load() -> *mut xdp_tx;
    fn xdp_tx__destroy(obj: *mut xdp_tx);
    fn xdp_redirect_multi_kern__open_and_load() -> *mut xdp_redirect_multi_kern;
    fn xdp_redirect_multi_kern__destroy(obj: *mut xdp_redirect_multi_kern);

    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(token: *mut nstoken);
    fn libbpf_set_print(fn_: libbpf_print_fn_t) -> libbpf_print_fn_t;
    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_ERR_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_GT(a: c_int, b: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(a: c_int, b: c_int, name: *const c_char) -> bool;
    fn ASSERT_LE(a: c_int, b: c_int, name: *const c_char) -> bool;
    fn ASSERT_LT(a: c_int, b: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(a: c_int, b: c_int, name: *const c_char) -> bool;
    fn ASSERT_NEQ(a: c_int, b: c_int, name: *const c_char) -> bool;
    fn PRINT_FAIL(format: *const c_char, ...);
}

static mut root_netns_fd: c_int = -1;

static mut mode_names: [*const c_char; 7] = [
    b"balance-rr\0".as_ptr() as *const c_char,
    b"active-backup\0".as_ptr() as *const c_char,
    b"balance-xor\0".as_ptr() as *const c_char,
    b"broadcast\0".as_ptr() as *const c_char,
    b"802.3ad\0".as_ptr() as *const c_char,
    b"balance-tlb\0".as_ptr() as *const c_char,
    b"balance-alb\0".as_ptr() as *const c_char,
];

static mut xmit_policy_names: [*const c_char; 5] = [
    b"layer2\0".as_ptr() as *const c_char,
    b"layer3+4\0".as_ptr() as *const c_char,
    b"layer2+3\0".as_ptr() as *const c_char,
    b"encap2+3\0".as_ptr() as *const c_char,
    b"encap3+4\0".as_ptr() as *const c_char,
];

const BOND_ONE_NO_ATTACH: c_int = 0;
const BOND_BOTH_AND_ATTACH: c_int = 1;

fn htons(x: u16) -> u16 {
    x.to_be()
}

unsafe fn restore_root_netns() {
    ASSERT_OK(
        setns(root_netns_fd, CLONE_NEWNET),
        b"restore_root_netns\0".as_ptr() as *const c_char,
    );
}

unsafe fn setns_by_name(name: *mut c_char) -> c_int {
    let mut nsfd: c_int;
    let err: c_int;
    let mut nspath = [0 as c_char; PATH_MAX];

    snprintf(
        nspath.as_mut_ptr(),
        nspath.len(),
        b"%s/%s\0".as_ptr() as *const c_char,
        b"/var/run/netns\0".as_ptr() as *const c_char,
        name,
    );
    nsfd = open(nspath.as_ptr(), O_RDONLY | O_CLOEXEC);
    if nsfd < 0 {
        return -1;
    }

    err = setns(nsfd, CLONE_NEWNET);
    close(nsfd);
    err
}

unsafe fn get_rx_packets(iface: *const c_char) -> c_int {
    let mut line = [0 as c_char; 512];
    let iface_len = strlen(iface);

    let f = fopen(
        b"/proc/net/dev\0".as_ptr() as *const c_char,
        b"r\0".as_ptr() as *const c_char,
    );
    if f.is_null() {
        return -1;
    }

    while !fgets(line.as_mut_ptr(), line.len() as c_int, f).is_null() {
        let mut p = line.as_mut_ptr();

        while *p == b' ' as c_char {
            p = p.add(1); /* skip whitespace */
        }
        if strncmp(p, iface, iface_len) == 0 {
            p = p.add(iface_len);
            if {
                let ch = *p;
                p = p.add(1);
                ch != b':' as c_char
            } {
                continue;
            }
            while *p == b' ' as c_char {
                p = p.add(1); /* skip whitespace */
            }
            while *p != 0 && *p != b' ' as c_char {
                p = p.add(1); /* skip rx bytes */
            }
            while *p == b' ' as c_char {
                p = p.add(1); /* skip whitespace */
            }
            fclose(f);
            return atoi(p);
        }
    }
    fclose(f);
    -1
}

unsafe fn xdp_attach(
    skeletons: *mut skeletons,
    prog: *mut bpf_program,
    iface: *mut c_char,
) -> c_int {
    let link: *mut bpf_link;
    let ifindex: c_int;

    ifindex = if_nametoindex(iface) as c_int;
    if !ASSERT_GT(ifindex, 0, b"get ifindex\0".as_ptr() as *const c_char) {
        return -1;
    }

    if !ASSERT_LE(
        (*skeletons).nlinks + 1,
        MAX_BPF_LINKS as c_int,
        b"too many XDP programs attached\0".as_ptr() as *const c_char,
    ) {
        return -1;
    }

    link = bpf_program__attach_xdp(prog, ifindex);
    if !ASSERT_OK_PTR(link as *mut c_void, b"attach xdp program\0".as_ptr() as *const c_char) {
        return -1;
    }

    (*skeletons).links[(*skeletons).nlinks as usize] = link;
    (*skeletons).nlinks += 1;
    0
}

unsafe fn bonding_setup(
    skeletons: *mut skeletons,
    mode: c_int,
    xmit_policy: c_int,
    bond_both_attach: c_int,
) -> c_int {
    let mut cmd = [0 as c_char; 512];

    if system(b"ip netns add ns_dst\0".as_ptr() as *const c_char) != 0 {
        return -1;
    }
    if system(b"ip link add veth1_1 type veth peer name veth2_1 netns ns_dst\0".as_ptr() as *const c_char) != 0 {
        return -1;
    }
    if system(b"ip link add veth1_2 type veth peer name veth2_2 netns ns_dst\0".as_ptr() as *const c_char) != 0 {
        return -1;
    }

    snprintf(
        cmd.as_mut_ptr(),
        cmd.len(),
        b"ip link add bond1 type bond mode %s xmit_hash_policy %s\0".as_ptr() as *const c_char,
        mode_names[mode as usize],
        xmit_policy_names[xmit_policy as usize],
    );
    if system(cmd.as_ptr()) != 0 {
        return -1;
    }
    if system(b"ip link set bond1 up address 00:11:22:33:44:55 addrgenmode none\0".as_ptr() as *const c_char) != 0 {
        return -1;
    }
    snprintf(
        cmd.as_mut_ptr(),
        cmd.len(),
        b"ip -netns ns_dst link add bond2 type bond mode %s xmit_hash_policy %s\0".as_ptr() as *const c_char,
        mode_names[mode as usize],
        xmit_policy_names[xmit_policy as usize],
    );
    if system(cmd.as_ptr()) != 0 {
        return -1;
    }
    if system(b"ip -netns ns_dst link set bond2 up address 00:22:33:44:55:66 addrgenmode none\0".as_ptr() as *const c_char) != 0 {
        return -1;
    }

    if system(b"ip link set veth1_1 master bond1\0".as_ptr() as *const c_char) != 0 {
        return -1;
    }
    if bond_both_attach == BOND_BOTH_AND_ATTACH {
        if system(b"ip link set veth1_2 master bond1\0".as_ptr() as *const c_char) != 0 {
            return -1;
        }
    } else {
        if system(b"ip link set veth1_2 up addrgenmode none\0".as_ptr() as *const c_char) != 0 {
            return -1;
        }

        if xdp_attach(
            skeletons,
            (*(*skeletons).xdp_dummy).progs.xdp_dummy_prog,
            b"veth1_2\0".as_ptr() as *mut c_char,
        ) != 0
        {
            return -1;
        }
    }

    if system(b"ip -netns ns_dst link set veth2_1 master bond2\0".as_ptr() as *const c_char) != 0 {
        return -1;
    }

    if bond_both_attach == BOND_BOTH_AND_ATTACH {
        if system(b"ip -netns ns_dst link set veth2_2 master bond2\0".as_ptr() as *const c_char) != 0 {
            return -1;
        }
    } else if system(b"ip -netns ns_dst link set veth2_2 up addrgenmode none\0".as_ptr() as *const c_char) != 0 {
        return -1;
    }

    /* Load a dummy program on sending side as with veth peer needs to have a
     * XDP program loaded as well.
     */
    if xdp_attach(
        skeletons,
        (*(*skeletons).xdp_dummy).progs.xdp_dummy_prog,
        b"bond1\0".as_ptr() as *mut c_char,
    ) != 0
    {
        return -1;
    }

    if bond_both_attach == BOND_BOTH_AND_ATTACH {
        if !ASSERT_OK(
            setns_by_name(b"ns_dst\0".as_ptr() as *mut c_char),
            b"set netns to ns_dst\0".as_ptr() as *const c_char,
        ) {
            return -1;
        }

        if xdp_attach(
            skeletons,
            (*(*skeletons).xdp_tx).progs.xdp_tx,
            b"bond2\0".as_ptr() as *mut c_char,
        ) != 0
        {
            return -1;
        }

        restore_root_netns();
    }

    0
}

unsafe fn link_cleanup(skeletons: *mut skeletons) {
    while (*skeletons).nlinks != 0 {
        (*skeletons).nlinks -= 1;
        bpf_link__destroy((*skeletons).links[(*skeletons).nlinks as usize]);
    }
}

unsafe fn bonding_cleanup(skeletons: *mut skeletons) {
    restore_root_netns();
    link_cleanup(skeletons);
    ASSERT_OK(system(b"ip link delete bond1\0".as_ptr() as *const c_char), b"delete bond1\0".as_ptr() as *const c_char);
    ASSERT_OK(system(b"ip link delete veth1_1\0".as_ptr() as *const c_char), b"delete veth1_1\0".as_ptr() as *const c_char);
    ASSERT_OK(system(b"ip link delete veth1_2\0".as_ptr() as *const c_char), b"delete veth1_2\0".as_ptr() as *const c_char);
    ASSERT_OK(system(b"ip netns delete ns_dst\0".as_ptr() as *const c_char), b"delete ns_dst\0".as_ptr() as *const c_char);
}

unsafe fn send_udp_packets(vary_dst_ip: c_int) -> c_int {
    let eh = ethhdr {
        h_source: BOND1_MAC,
        h_dest: BOND2_MAC,
        h_proto: htons(ETH_P_IP),
    };
    let mut iph: iphdr = zeroed();
    let mut uh: udphdr = zeroed();
    let mut buf = [0u8; 128];
    let mut s: c_int = -1;

    s = socket(AF_PACKET, SOCK_RAW, IPPROTO_RAW);
    if !ASSERT_GE(s, 0, b"socket\0".as_ptr() as *const c_char) {
        if s >= 0 {
            close(s);
        }
        return -1;
    }

    let ifindex = if_nametoindex(b"bond1\0".as_ptr() as *const c_char) as c_int;
    if !ASSERT_GT(ifindex, 0, b"get bond1 ifindex\0".as_ptr() as *const c_char) {
        if s >= 0 {
            close(s);
        }
        return -1;
    }

    iph.set_ihl(5);
    iph.set_version(4);
    iph.tos = 16;
    iph.id = 1;
    iph.ttl = 64;
    iph.protocol = IPPROTO_UDP as u8;
    iph.saddr = 1;
    iph.daddr = 2;
    iph.tot_len = htons((size_of::<[u8; 128]>() - ETH_HLEN) as u16);
    iph.check = 0;

    for _i in 1..=NPACKETS {
        let mut saddr_ll: sockaddr_ll = zeroed();
        saddr_ll.sll_ifindex = ifindex;
        saddr_ll.sll_halen = ETH_ALEN as u8;
        saddr_ll.sll_addr[..ETH_ALEN].copy_from_slice(&BOND2_MAC);

        /* vary the UDP destination port for even distribution with roundrobin/xor modes */
        uh.dest = uh.dest.wrapping_add(1);

        if vary_dst_ip != 0 {
            iph.daddr = iph.daddr.wrapping_add(1);
        }

        /* construct a packet */
        memcpy(buf.as_mut_ptr() as *mut c_void, &eh as *const _ as *const c_void, size_of::<ethhdr>());
        memcpy(buf.as_mut_ptr().add(size_of::<ethhdr>()) as *mut c_void, &iph as *const _ as *const c_void, size_of::<iphdr>());
        memcpy(buf.as_mut_ptr().add(size_of::<ethhdr>() + size_of::<iphdr>()) as *mut c_void, &uh as *const _ as *const c_void, size_of::<udphdr>());

        let n = sendto(
            s,
            buf.as_ptr() as *const c_void,
            buf.len(),
            0,
            &saddr_ll as *const _ as *const sockaddr,
            size_of::<sockaddr_ll>() as u32,
        );
        if !ASSERT_EQ(n as c_int, buf.len() as c_int, b"sendto\0".as_ptr() as *const c_char) {
            if s >= 0 {
                close(s);
            }
            return -1;
        }
    }

    0
}

unsafe fn test_xdp_bonding_with_mode(skeletons: *mut skeletons, mode: c_int, xmit_policy: c_int) {
    if bonding_setup(skeletons, mode, xmit_policy, BOND_BOTH_AND_ATTACH) != 0 {
        bonding_cleanup(skeletons);
        return;
    }

    if send_udp_packets((xmit_policy != BOND_XMIT_POLICY_LAYER34) as c_int) != 0 {
        bonding_cleanup(skeletons);
        return;
    }

    let bond1_rx = get_rx_packets(b"bond1\0".as_ptr() as *const c_char);
    ASSERT_EQ(bond1_rx, NPACKETS, b"expected more received packets\0".as_ptr() as *const c_char);

    match mode {
        BOND_MODE_ROUNDROBIN | BOND_MODE_XOR => {
            let veth1_rx = get_rx_packets(b"veth1_1\0".as_ptr() as *const c_char);
            let veth2_rx = get_rx_packets(b"veth1_2\0".as_ptr() as *const c_char);
            let diff = abs(veth1_rx - veth2_rx);

            ASSERT_GE(veth1_rx + veth2_rx, NPACKETS, b"expected more packets\0".as_ptr() as *const c_char);

            match xmit_policy {
                BOND_XMIT_POLICY_LAYER2 => {
                    ASSERT_GE(diff, NPACKETS, b"expected packets on only one of the interfaces\0".as_ptr() as *const c_char);
                }
                BOND_XMIT_POLICY_LAYER23 | BOND_XMIT_POLICY_LAYER34 => {
                    ASSERT_LT(diff, NPACKETS / 2, b"expected even distribution of packets\0".as_ptr() as *const c_char);
                }
                _ => {
                    PRINT_FAIL(b"Unimplemented xmit_policy=%d\n\0".as_ptr() as *const c_char, xmit_policy);
                }
            }
        }
        BOND_MODE_ACTIVEBACKUP => {
            let veth1_rx = get_rx_packets(b"veth1_1\0".as_ptr() as *const c_char);
            let veth2_rx = get_rx_packets(b"veth1_2\0".as_ptr() as *const c_char);
            let diff = abs(veth1_rx - veth2_rx);

            ASSERT_GE(diff, NPACKETS, b"expected packets on only one of the interfaces\0".as_ptr() as *const c_char);
        }
        _ => {
            PRINT_FAIL(b"Unimplemented xmit_policy=%d\n\0".as_ptr() as *const c_char, xmit_policy);
        }
    }

    bonding_cleanup(skeletons);
}

/* Test the broadcast redirection using xdp_redirect_map_multi_prog and adding
 * all the interfaces to it and checking that broadcasting won't send the packet
 * to neither the ingress bond device (bond2) or its slave (veth2_1).
 */
unsafe fn test_xdp_bonding_redirect_multi(skeletons: *mut skeletons) {
    let ifaces: [*const c_char; 3] = [
        b"bond2\0".as_ptr() as *const c_char,
        b"veth2_1\0".as_ptr() as *const c_char,
        b"veth2_2\0".as_ptr() as *const c_char,
    ];

    if bonding_setup(skeletons, BOND_MODE_ROUNDROBIN, BOND_XMIT_POLICY_LAYER23, BOND_ONE_NO_ATTACH) != 0 {
        restore_root_netns();
        bonding_cleanup(skeletons);
        return;
    }

    if !ASSERT_OK(setns_by_name(b"ns_dst\0".as_ptr() as *mut c_char), b"could not set netns to ns_dst\0".as_ptr() as *const c_char) {
        restore_root_netns();
        bonding_cleanup(skeletons);
        return;
    }

    /* populate the devmap with the relevant interfaces */
    for i in 0..ifaces.len() {
        let ifindex = if_nametoindex(ifaces[i]) as c_int;
        let map_fd = bpf_map__fd((*(*skeletons).xdp_redirect_multi_kern).maps.map_all);

        if !ASSERT_GT(ifindex, 0, b"could not get interface index\0".as_ptr() as *const c_char) {
            restore_root_netns();
            bonding_cleanup(skeletons);
            return;
        }

        let err = bpf_map_update_elem(
            map_fd,
            &ifindex as *const _ as *const c_void,
            &ifindex as *const _ as *const c_void,
            0,
        );
        if !ASSERT_OK(err, b"add interface to map_all\0".as_ptr() as *const c_char) {
            restore_root_netns();
            bonding_cleanup(skeletons);
            return;
        }
    }

    if xdp_attach(
        skeletons,
        (*(*skeletons).xdp_redirect_multi_kern).progs.xdp_redirect_map_multi_prog,
        b"bond2\0".as_ptr() as *mut c_char,
    ) != 0
    {
        restore_root_netns();
        bonding_cleanup(skeletons);
        return;
    }

    restore_root_netns();

    if send_udp_packets(BOND_MODE_ROUNDROBIN) != 0 {
        restore_root_netns();
        bonding_cleanup(skeletons);
        return;
    }

    let veth1_1_rx = get_rx_packets(b"veth1_1\0".as_ptr() as *const c_char);
    let veth1_2_rx = get_rx_packets(b"veth1_2\0".as_ptr() as *const c_char);

    ASSERT_EQ(veth1_1_rx, 0, b"expected no packets on veth1_1\0".as_ptr() as *const c_char);
    ASSERT_GE(veth1_2_rx, NPACKETS, b"expected packets on veth1_2\0".as_ptr() as *const c_char);

    restore_root_netns();
    bonding_cleanup(skeletons);
}

/* Test that XDP programs cannot be attached to both the bond master and slaves simultaneously */
unsafe fn test_xdp_bonding_attach(skeletons: *mut skeletons) {
    let mut link: *mut bpf_link = null_mut();
    let mut link2: *mut bpf_link = null_mut();

    if !ASSERT_OK(system(b"ip link add veth type veth\0".as_ptr() as *const c_char), b"add veth\0".as_ptr() as *const c_char) {
        goto_attach_out(link, link2);
        return;
    }
    if !ASSERT_OK(system(b"ip link add bond type bond\0".as_ptr() as *const c_char), b"add bond\0".as_ptr() as *const c_char) {
        goto_attach_out(link, link2);
        return;
    }

    let veth = if_nametoindex(b"veth\0".as_ptr() as *const c_char) as c_int;
    if !ASSERT_GE(veth, 0, b"if_nametoindex veth\0".as_ptr() as *const c_char) {
        goto_attach_out(link, link2);
        return;
    }
    let bond = if_nametoindex(b"bond\0".as_ptr() as *const c_char) as c_int;
    if !ASSERT_GE(bond, 0, b"if_nametoindex bond\0".as_ptr() as *const c_char) {
        goto_attach_out(link, link2);
        return;
    }

    /* enslaving with a XDP program loaded is allowed */
    link = bpf_program__attach_xdp((*(*skeletons).xdp_dummy).progs.xdp_dummy_prog, veth);
    if !ASSERT_OK_PTR(link as *mut c_void, b"attach program to veth\0".as_ptr() as *const c_char) {
        goto_attach_out(link, link2);
        return;
    }

    let mut err = system(b"ip link set veth master bond\0".as_ptr() as *const c_char);
    if !ASSERT_OK(err, b"set veth master\0".as_ptr() as *const c_char) {
        goto_attach_out(link, link2);
        return;
    }

    bpf_link__destroy(link);
    link = null_mut();

    /* attaching to slave when master has no program is allowed */
    link = bpf_program__attach_xdp((*(*skeletons).xdp_dummy).progs.xdp_dummy_prog, veth);
    if !ASSERT_OK_PTR(link as *mut c_void, b"attach program to slave when enslaved\0".as_ptr() as *const c_char) {
        goto_attach_out(link, link2);
        return;
    }

    /* attaching to master not allowed when slave has program loaded */
    link2 = bpf_program__attach_xdp((*(*skeletons).xdp_dummy).progs.xdp_dummy_prog, bond);
    if !ASSERT_ERR_PTR(link2 as *mut c_void, b"attach program to master when slave has program\0".as_ptr() as *const c_char) {
        goto_attach_out(link, link2);
        return;
    }

    bpf_link__destroy(link);
    link = null_mut();

    /* attaching XDP program to master allowed when slave has no program */
    link = bpf_program__attach_xdp((*(*skeletons).xdp_dummy).progs.xdp_dummy_prog, bond);
    if !ASSERT_OK_PTR(link as *mut c_void, b"attach program to master\0".as_ptr() as *const c_char) {
        goto_attach_out(link, link2);
        return;
    }

    /* attaching to slave not allowed when master has program loaded */
    link2 = bpf_program__attach_xdp((*(*skeletons).xdp_dummy).progs.xdp_dummy_prog, veth);
    if !ASSERT_ERR_PTR(link2 as *mut c_void, b"attach program to slave when master has program\0".as_ptr() as *const c_char) {
        goto_attach_out(link, link2);
        return;
    }

    bpf_link__destroy(link);
    link = null_mut();

    /* test program unwinding with a non-XDP slave */
    if !ASSERT_OK(system(b"ip link add vxlan type vxlan id 1 remote 1.2.3.4 dstport 0 dev lo\0".as_ptr() as *const c_char), b"add vxlan\0".as_ptr() as *const c_char) {
        goto_attach_out(link, link2);
        return;
    }

    err = system(b"ip link set vxlan master bond\0".as_ptr() as *const c_char);
    if !ASSERT_OK(err, b"set vxlan master\0".as_ptr() as *const c_char) {
        goto_attach_out(link, link2);
        return;
    }

    /* attaching not allowed when one slave does not support XDP */
    link = bpf_program__attach_xdp((*(*skeletons).xdp_dummy).progs.xdp_dummy_prog, bond);
    if !ASSERT_ERR_PTR(link as *mut c_void, b"attach program to master when slave does not support XDP\0".as_ptr() as *const c_char) {
        goto_attach_out(link, link2);
        return;
    }

    goto_attach_out(link, link2);
}

unsafe fn goto_attach_out(link: *mut bpf_link, link2: *mut bpf_link) {
    bpf_link__destroy(link);
    bpf_link__destroy(link2);

    system(b"ip link del veth\0".as_ptr() as *const c_char);
    system(b"ip link del bond\0".as_ptr() as *const c_char);
    system(b"ip link del vxlan\0".as_ptr() as *const c_char);
}

/* Test with nested bonding devices to catch issue with negative jump label count */
unsafe fn test_xdp_bonding_nested(skeletons: *mut skeletons) {
    let mut link: *mut bpf_link = null_mut();

    if !ASSERT_OK(system(b"ip link add bond type bond\0".as_ptr() as *const c_char), b"add bond\0".as_ptr() as *const c_char) {
        goto_nested_out(link);
        return;
    }

    let bond = if_nametoindex(b"bond\0".as_ptr() as *const c_char) as c_int;
    if !ASSERT_GE(bond, 0, b"if_nametoindex bond\0".as_ptr() as *const c_char) {
        goto_nested_out(link);
        return;
    }

    if !ASSERT_OK(system(b"ip link add bond_nest1 type bond\0".as_ptr() as *const c_char), b"add bond_nest1\0".as_ptr() as *const c_char) {
        goto_nested_out(link);
        return;
    }

    let mut err = system(b"ip link set bond_nest1 master bond\0".as_ptr() as *const c_char);
    if !ASSERT_OK(err, b"set bond_nest1 master\0".as_ptr() as *const c_char) {
        goto_nested_out(link);
        return;
    }

    if !ASSERT_OK(system(b"ip link add bond_nest2 type bond\0".as_ptr() as *const c_char), b"add bond_nest1\0".as_ptr() as *const c_char) {
        goto_nested_out(link);
        return;
    }

    err = system(b"ip link set bond_nest2 master bond_nest1\0".as_ptr() as *const c_char);
    if !ASSERT_OK(err, b"set bond_nest2 master\0".as_ptr() as *const c_char) {
        goto_nested_out(link);
        return;
    }

    link = bpf_program__attach_xdp((*(*skeletons).xdp_dummy).progs.xdp_dummy_prog, bond);
    ASSERT_OK_PTR(link as *mut c_void, b"attach program to master\0".as_ptr() as *const c_char);

    goto_nested_out(link);
}

unsafe fn goto_nested_out(link: *mut bpf_link) {
    bpf_link__destroy(link);
    system(b"ip link del bond\0".as_ptr() as *const c_char);
    system(b"ip link del bond_nest1\0".as_ptr() as *const c_char);
    system(b"ip link del bond_nest2\0".as_ptr() as *const c_char);
}

/*
 * Test that XDP redirect via xdp_master_redirect() does not crash when
 * the bond master device is not up. When bond is in round-robin mode but
 * never opened, rr_tx_counter is NULL.
 */
unsafe fn test_xdp_bonding_redirect_no_up(skeletons: *mut skeletons) {
    let mut nstoken: *mut nstoken = null_mut();
    let xdp_pass_fd: c_int;
    let veth1_ifindex: c_int;
    let mut err: c_int;
    let mut pkt = [0 as c_char; ETH_HLEN + 1];
    let mut ctx_in: xdp_md = zeroed();

    let mut opts = bpf_test_run_opts {
        sz: size_of::<bpf_test_run_opts>(),
        data_in: pkt.as_mut_ptr() as *mut c_void,
        data_size_in: size_of_val(&pkt) as u32,
        ctx_in: &mut ctx_in as *mut _ as *mut c_void,
        ctx_size_in: size_of::<xdp_md>() as u32,
        flags: BPF_F_TEST_XDP_LIVE_FRAMES,
        repeat: 1,
        batch_size: 1,
    };

    /* We can't use bonding_setup() because bond will be active */
    if system(b"ip netns add ns_rr_no_up\0".as_ptr() as *const c_char) != 0 {
        goto_redirect_no_up_out(skeletons, nstoken);
        return;
    }
    nstoken = open_netns(b"ns_rr_no_up\0".as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(nstoken as *mut c_void, b"open ns_rr_no_up\0".as_ptr() as *const c_char) {
        goto_redirect_no_up_out(skeletons, nstoken);
        return;
    }

    /* bond0: active-backup, UP with slave veth0.
     * Attaching native XDP to bond0 enables bpf_master_redirect_enabled_key
     * globally.
     */
    if system(b"ip link add bond0 type bond mode active-backup\0".as_ptr() as *const c_char) != 0 {
        goto_redirect_no_up_out(skeletons, nstoken);
        return;
    }
    if system(b"ip link add veth0 type veth peer name veth0p\0".as_ptr() as *const c_char) != 0 {
        goto_redirect_no_up_out(skeletons, nstoken);
        return;
    }
    if system(b"ip link set veth0 master bond0\0".as_ptr() as *const c_char) != 0 {
        goto_redirect_no_up_out(skeletons, nstoken);
        return;
    }
    if system(b"ip link set bond0 up\0".as_ptr() as *const c_char) != 0 {
        goto_redirect_no_up_out(skeletons, nstoken);
        return;
    }
    if system(b"ip link set veth0p up\0".as_ptr() as *const c_char) != 0 {
        goto_redirect_no_up_out(skeletons, nstoken);
        return;
    }

    /* bond1: round-robin, never UP -> rr_tx_counter stays NULL */
    if system(b"ip link add bond1 type bond mode balance-rr\0".as_ptr() as *const c_char) != 0 {
        goto_redirect_no_up_out(skeletons, nstoken);
        return;
    }
    if system(b"ip link add veth1 type veth peer name veth1p\0".as_ptr() as *const c_char) != 0 {
        goto_redirect_no_up_out(skeletons, nstoken);
        return;
    }
    if system(b"ip link set veth1 master bond1\0".as_ptr() as *const c_char) != 0 {
        goto_redirect_no_up_out(skeletons, nstoken);
        return;
    }

    veth1_ifindex = if_nametoindex(b"veth1\0".as_ptr() as *const c_char) as c_int;
    if !ASSERT_GT(veth1_ifindex, 0, b"veth1_ifindex\0".as_ptr() as *const c_char) {
        goto_redirect_no_up_out(skeletons, nstoken);
        return;
    }

    /* Attach native XDP to bond0 -> enables global redirect key */
    if xdp_attach(skeletons, (*(*skeletons).xdp_tx).progs.xdp_tx, b"bond0\0".as_ptr() as *mut c_char) != 0 {
        goto_redirect_no_up_out(skeletons, nstoken);
        return;
    }

    /* Attach generic XDP (XDP_TX) to veth1.
     * When packets arrive at veth1 via netif_receive_skb, do_xdp_generic()
     * runs this program. XDP_TX + bond slave triggers xdp_master_redirect().
     */
    err = bpf_xdp_attach(
        veth1_ifindex,
        bpf_program__fd((*(*skeletons).xdp_tx).progs.xdp_tx),
        XDP_FLAGS_SKB_MODE,
        null(),
    );
    if !ASSERT_OK(err, b"attach generic XDP to veth1\0".as_ptr() as *const c_char) {
        goto_redirect_no_up_out(skeletons, nstoken);
        return;
    }

    /* Run BPF_PROG_TEST_RUN with XDP_PASS live frames on veth1.
     * XDP_PASS frames become SKBs with skb->dev = veth1, entering
     * netif_receive_skb -> do_xdp_generic -> xdp_master_redirect.
     * Without the fix, bond_rr_gen_slave_id() dereferences NULL
     * rr_tx_counter and crashes.
     */
    xdp_pass_fd = bpf_program__fd((*(*skeletons).xdp_dummy).progs.xdp_dummy_prog);

    memset(pkt.as_mut_ptr() as *mut c_void, 0, pkt.len());
    ctx_in.data_end = pkt.len() as u32;
    ctx_in.ingress_ifindex = veth1_ifindex as u32;

    err = bpf_prog_test_run_opts(xdp_pass_fd, &mut opts);
    ASSERT_OK(err, b"xdp_pass test_run should not crash\0".as_ptr() as *const c_char);

    goto_redirect_no_up_out(skeletons, nstoken);
}

unsafe fn goto_redirect_no_up_out(skeletons: *mut skeletons, token: *mut nstoken) {
    link_cleanup(skeletons);
    close_netns(token);
    system(b"ip netns del ns_rr_no_up\0".as_ptr() as *const c_char);
}

unsafe fn test_xdp_bonding_features(skeletons: *mut skeletons) {
    let mut query_opts = bpf_xdp_query_opts {
        sz: size_of::<bpf_xdp_query_opts>(),
        feature_flags: 0,
    };
    let bond_idx: c_int;
    let veth1_idx: c_int;
    let mut err: c_int;
    let mut link: *mut bpf_link = null_mut();

    if !ASSERT_OK(system(b"ip link add bond type bond\0".as_ptr() as *const c_char), b"add bond\0".as_ptr() as *const c_char) {
        goto_features_out(link);
        return;
    }

    bond_idx = if_nametoindex(b"bond\0".as_ptr() as *const c_char) as c_int;
    if !ASSERT_GE(bond_idx, 0, b"if_nametoindex bond\0".as_ptr() as *const c_char) {
        goto_features_out(link);
        return;
    }

    /* query default xdp-feature for bond device */
    err = bpf_xdp_query(bond_idx, XDP_FLAGS_DRV_MODE, &mut query_opts);
    if !ASSERT_OK(err, b"bond bpf_xdp_query\0".as_ptr() as *const c_char) {
        goto_features_out(link);
        return;
    }

    if !ASSERT_EQ(query_opts.feature_flags as c_int, 0, b"bond query_opts.feature_flags\0".as_ptr() as *const c_char) {
        goto_features_out(link);
        return;
    }

    if !ASSERT_OK(system(b"ip link add veth0 type veth peer name veth1\0".as_ptr() as *const c_char), b"add veth{0,1} pair\0".as_ptr() as *const c_char) {
        goto_features_out(link);
        return;
    }

    if !ASSERT_OK(system(b"ip link add veth2 type veth peer name veth3\0".as_ptr() as *const c_char), b"add veth{2,3} pair\0".as_ptr() as *const c_char) {
        goto_features_out(link);
        return;
    }

    if !ASSERT_OK(system(b"ip link set veth0 master bond\0".as_ptr() as *const c_char), b"add veth0 to master bond\0".as_ptr() as *const c_char) {
        goto_features_out(link);
        return;
    }

    /* xdp-feature for bond device should be obtained from the single slave
     * device (veth0)
     */
    err = bpf_xdp_query(bond_idx, XDP_FLAGS_DRV_MODE, &mut query_opts);
    if !ASSERT_OK(err, b"bond bpf_xdp_query\0".as_ptr() as *const c_char) {
        goto_features_out(link);
        return;
    }

    if !ASSERT_EQ(
        query_opts.feature_flags as c_int,
        (NETDEV_XDP_ACT_BASIC | NETDEV_XDP_ACT_REDIRECT | NETDEV_XDP_ACT_RX_SG) as c_int,
        b"bond query_opts.feature_flags\0".as_ptr() as *const c_char,
    ) {
        goto_features_out(link);
        return;
    }

    veth1_idx = if_nametoindex(b"veth1\0".as_ptr() as *const c_char) as c_int;
    if !ASSERT_GE(veth1_idx, 0, b"if_nametoindex veth1\0".as_ptr() as *const c_char) {
        goto_features_out(link);
        return;
    }

    link = bpf_program__attach_xdp((*(*skeletons).xdp_dummy).progs.xdp_dummy_prog, veth1_idx);
    if !ASSERT_OK_PTR(link as *mut c_void, b"attach program to veth1\0".as_ptr() as *const c_char) {
        goto_features_out(link);
        return;
    }

    /* xdp-feature for veth0 are changed */
    err = bpf_xdp_query(bond_idx, XDP_FLAGS_DRV_MODE, &mut query_opts);
    if !ASSERT_OK(err, b"bond bpf_xdp_query\0".as_ptr() as *const c_char) {
        goto_features_out(link);
        return;
    }

    if !ASSERT_EQ(
        query_opts.feature_flags as c_int,
        (NETDEV_XDP_ACT_BASIC | NETDEV_XDP_ACT_REDIRECT | NETDEV_XDP_ACT_RX_SG |
         NETDEV_XDP_ACT_NDO_XMIT | NETDEV_XDP_ACT_NDO_XMIT_SG) as c_int,
        b"bond query_opts.feature_flags\0".as_ptr() as *const c_char,
    ) {
        goto_features_out(link);
        return;
    }

    if !ASSERT_OK(system(b"ip link set veth2 master bond\0".as_ptr() as *const c_char), b"add veth2 to master bond\0".as_ptr() as *const c_char) {
        goto_features_out(link);
        return;
    }

    err = bpf_xdp_query(bond_idx, XDP_FLAGS_DRV_MODE, &mut query_opts);
    if !ASSERT_OK(err, b"bond bpf_xdp_query\0".as_ptr() as *const c_char) {
        goto_features_out(link);
        return;
    }

    /* xdp-feature for bond device should be set to the most restrict
     * value obtained from attached slave devices (veth0 and veth2)
     */
    if !ASSERT_EQ(
        query_opts.feature_flags as c_int,
        (NETDEV_XDP_ACT_BASIC | NETDEV_XDP_ACT_REDIRECT | NETDEV_XDP_ACT_RX_SG) as c_int,
        b"bond query_opts.feature_flags\0".as_ptr() as *const c_char,
    ) {
        goto_features_out(link);
        return;
    }

    if !ASSERT_OK(system(b"ip link set veth2 nomaster\0".as_ptr() as *const c_char), b"del veth2 to master bond\0".as_ptr() as *const c_char) {
        goto_features_out(link);
        return;
    }

    err = bpf_xdp_query(bond_idx, XDP_FLAGS_DRV_MODE, &mut query_opts);
    if !ASSERT_OK(err, b"bond bpf_xdp_query\0".as_ptr() as *const c_char) {
        goto_features_out(link);
        return;
    }

    if !ASSERT_EQ(
        query_opts.feature_flags as c_int,
        (NETDEV_XDP_ACT_BASIC | NETDEV_XDP_ACT_REDIRECT | NETDEV_XDP_ACT_RX_SG |
         NETDEV_XDP_ACT_NDO_XMIT | NETDEV_XDP_ACT_NDO_XMIT_SG) as c_int,
        b"bond query_opts.feature_flags\0".as_ptr() as *const c_char,
    ) {
        goto_features_out(link);
        return;
    }

    if !ASSERT_OK(system(b"ip link set veth0 nomaster\0".as_ptr() as *const c_char), b"del veth0 to master bond\0".as_ptr() as *const c_char) {
        goto_features_out(link);
        return;
    }

    err = bpf_xdp_query(bond_idx, XDP_FLAGS_DRV_MODE, &mut query_opts);
    if !ASSERT_OK(err, b"bond bpf_xdp_query\0".as_ptr() as *const c_char) {
        goto_features_out(link);
        return;
    }

    ASSERT_EQ(query_opts.feature_flags as c_int, 0, b"bond query_opts.feature_flags\0".as_ptr() as *const c_char);
    goto_features_out(link);
}

unsafe fn goto_features_out(link: *mut bpf_link) {
    bpf_link__destroy(link);
    system(b"ip link del veth0\0".as_ptr() as *const c_char);
    system(b"ip link del veth2\0".as_ptr() as *const c_char);
    system(b"ip link del bond\0".as_ptr() as *const c_char);
}

/*
 * Test that changing xmit_hash_policy to vlan+srcmac is rejected when a
 * native XDP program is loaded on a bond in 802.3ad or balance-xor mode.
 * These modes support XDP only when xmit_hash_policy != vlan+srcmac; freely
 * changing the policy creates an inconsistency that triggers a WARNING in
 * dev_xdp_uninstall() during device teardown.
 */
unsafe fn test_xdp_bonding_xmit_policy_compat(skeletons: *mut skeletons) {
    let mut nstoken: *mut nstoken = null_mut();
    let mut bond_ifindex: c_int = -1;
    let xdp_fd: c_int;
    let mut err: c_int;

    if system(b"ip netns add ns_xmit_policy\0".as_ptr() as *const c_char) != 0 {
        goto_xmit_policy_out(nstoken, bond_ifindex);
        return;
    }
    nstoken = open_netns(b"ns_xmit_policy\0".as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(nstoken as *mut c_void, b"open ns_xmit_policy\0".as_ptr() as *const c_char) {
        goto_xmit_policy_out(nstoken, bond_ifindex);
        return;
    }

    /* 802.3ad with layer2+3 policy: native XDP is supported */
    if system(b"ip link add bond0 type bond mode 802.3ad xmit_hash_policy layer2+3\0".as_ptr() as *const c_char) != 0 {
        goto_xmit_policy_out(nstoken, bond_ifindex);
        return;
    }
    if system(b"ip link add veth0 type veth peer name veth0p\0".as_ptr() as *const c_char) != 0 {
        goto_xmit_policy_out(nstoken, bond_ifindex);
        return;
    }
    if system(b"ip link set veth0 master bond0\0".as_ptr() as *const c_char) != 0 {
        goto_xmit_policy_out(nstoken, bond_ifindex);
        return;
    }
    if system(b"ip link set bond0 up\0".as_ptr() as *const c_char) != 0 {
        goto_xmit_policy_out(nstoken, bond_ifindex);
        return;
    }

    bond_ifindex = if_nametoindex(b"bond0\0".as_ptr() as *const c_char) as c_int;
    if !ASSERT_GT(bond_ifindex, 0, b"bond0 ifindex\0".as_ptr() as *const c_char) {
        goto_xmit_policy_out(nstoken, bond_ifindex);
        return;
    }

    xdp_fd = bpf_program__fd((*(*skeletons).xdp_dummy).progs.xdp_dummy_prog);
    if !ASSERT_GE(xdp_fd, 0, b"xdp_dummy fd\0".as_ptr() as *const c_char) {
        goto_xmit_policy_out(nstoken, bond_ifindex);
        return;
    }

    err = bpf_xdp_attach(bond_ifindex, xdp_fd, XDP_FLAGS_DRV_MODE, null());
    if !ASSERT_OK(err, b"attach XDP to bond0\0".as_ptr() as *const c_char) {
        goto_xmit_policy_out(nstoken, bond_ifindex);
        return;
    }

    /* With XDP loaded, switching to vlan+srcmac must be rejected */
    err = system(b"ip link set bond0 type bond xmit_hash_policy vlan+srcmac 2>/dev/null\0".as_ptr() as *const c_char);
    ASSERT_NEQ(err, 0, b"vlan+srcmac change with XDP loaded should fail\0".as_ptr() as *const c_char);

    /* Detach XDP first, then the same change must succeed */
    ASSERT_OK(
        bpf_xdp_detach(bond_ifindex, XDP_FLAGS_DRV_MODE, null()),
        b"detach XDP from bond0\0".as_ptr() as *const c_char,
    );

    bond_ifindex = -1;
    err = system(b"ip link set bond0 type bond xmit_hash_policy vlan+srcmac 2>/dev/null\0".as_ptr() as *const c_char);
    ASSERT_OK(err, b"vlan+srcmac change without XDP should succeed\0".as_ptr() as *const c_char);

    goto_xmit_policy_out(nstoken, bond_ifindex);
}

unsafe fn goto_xmit_policy_out(token: *mut nstoken, bond_ifindex: c_int) {
    if bond_ifindex > 0 {
        bpf_xdp_detach(bond_ifindex, XDP_FLAGS_DRV_MODE, null());
    }
    close_netns(token);
    system(b"ip netns del ns_xmit_policy\0".as_ptr() as *const c_char);
}

unsafe extern "C" fn libbpf_debug_print(
    level: libbpf_print_level,
    format: *const c_char,
    args: va_list,
) -> c_int {
    if level != LIBBPF_WARN {
        vprintf(format, args);
    }
    0
}

static mut bond_test_cases: [bond_test_case; 5] = [
    bond_test_case {
        name: b"xdp_bonding_roundrobin\0".as_ptr() as *mut c_char,
        mode: BOND_MODE_ROUNDROBIN,
        xmit_policy: BOND_XMIT_POLICY_LAYER23,
    },
    bond_test_case {
        name: b"xdp_bonding_activebackup\0".as_ptr() as *mut c_char,
        mode: BOND_MODE_ACTIVEBACKUP,
        xmit_policy: BOND_XMIT_POLICY_LAYER23,
    },
    bond_test_case {
        name: b"xdp_bonding_xor_layer2\0".as_ptr() as *mut c_char,
        mode: BOND_MODE_XOR,
        xmit_policy: BOND_XMIT_POLICY_LAYER2,
    },
    bond_test_case {
        name: b"xdp_bonding_xor_layer23\0".as_ptr() as *mut c_char,
        mode: BOND_MODE_XOR,
        xmit_policy: BOND_XMIT_POLICY_LAYER23,
    },
    bond_test_case {
        name: b"xdp_bonding_xor_layer34\0".as_ptr() as *mut c_char,
        mode: BOND_MODE_XOR,
        xmit_policy: BOND_XMIT_POLICY_LAYER34,
    },
];

#[no_mangle]
pub unsafe extern "C" fn serial_test_xdp_bonding() {
    let old_print_fn: libbpf_print_fn_t;
    let mut skeletons: skeletons = zeroed();

    old_print_fn = libbpf_set_print(Some(libbpf_debug_print));

    root_netns_fd = open(b"/proc/self/ns/net\0".as_ptr() as *const c_char, O_RDONLY);
    if !ASSERT_GE(root_netns_fd, 0, b"open /proc/self/ns/net\0".as_ptr() as *const c_char) {
        goto_serial_out(&mut skeletons, old_print_fn);
        return;
    }

    skeletons.xdp_dummy = xdp_dummy__open_and_load();
    if !ASSERT_OK_PTR(skeletons.xdp_dummy as *mut c_void, b"xdp_dummy__open_and_load\0".as_ptr() as *const c_char) {
        goto_serial_out(&mut skeletons, old_print_fn);
        return;
    }

    skeletons.xdp_tx = xdp_tx__open_and_load();
    if !ASSERT_OK_PTR(skeletons.xdp_tx as *mut c_void, b"xdp_tx__open_and_load\0".as_ptr() as *const c_char) {
        goto_serial_out(&mut skeletons, old_print_fn);
        return;
    }

    skeletons.xdp_redirect_multi_kern = xdp_redirect_multi_kern__open_and_load();
    if !ASSERT_OK_PTR(
        skeletons.xdp_redirect_multi_kern as *mut c_void,
        b"xdp_redirect_multi_kern__open_and_load\0".as_ptr() as *const c_char,
    ) {
        goto_serial_out(&mut skeletons, old_print_fn);
        return;
    }

    if test__start_subtest(b"xdp_bonding_attach\0".as_ptr() as *const c_char) {
        test_xdp_bonding_attach(&mut skeletons);
    }

    if test__start_subtest(b"xdp_bonding_nested\0".as_ptr() as *const c_char) {
        test_xdp_bonding_nested(&mut skeletons);
    }

    if test__start_subtest(b"xdp_bonding_features\0".as_ptr() as *const c_char) {
        test_xdp_bonding_features(&mut skeletons);
    }

    for i in 0..bond_test_cases.len() {
        let test_case = &mut bond_test_cases[i];

        if test__start_subtest(test_case.name) {
            test_xdp_bonding_with_mode(&mut skeletons, test_case.mode, test_case.xmit_policy);
        }
    }

    if test__start_subtest(b"xdp_bonding_xmit_policy_compat\0".as_ptr() as *const c_char) {
        test_xdp_bonding_xmit_policy_compat(&mut skeletons);
    }

    if test__start_subtest(b"xdp_bonding_redirect_multi\0".as_ptr() as *const c_char) {
        test_xdp_bonding_redirect_multi(&mut skeletons);
    }

    if test__start_subtest(b"xdp_bonding_redirect_no_up\0".as_ptr() as *const c_char) {
        test_xdp_bonding_redirect_no_up(&mut skeletons);
    }

    goto_serial_out(&mut skeletons, old_print_fn);
}

unsafe fn goto_serial_out(skeletons: *mut skeletons, old_print_fn: libbpf_print_fn_t) {
    xdp_dummy__destroy((*skeletons).xdp_dummy);
    xdp_tx__destroy((*skeletons).xdp_tx);
    xdp_redirect_multi_kern__destroy((*skeletons).xdp_redirect_multi_kern);

    libbpf_set_print(old_print_fn);
    if root_netns_fd >= 0 {
        close(root_netns_fd);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
