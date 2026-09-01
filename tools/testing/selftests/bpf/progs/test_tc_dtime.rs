// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2022 Meta

// C dependencies translated as external Rust dependencies:
// <stddef.h>, <stdint.h>, <stdbool.h>, <linux/bpf.h>, <linux/stddef.h>,
// <linux/pkt_cls.h>, <linux/if_ether.h>, <linux/in.h>, <linux/ip.h>,
// <linux/ipv6.h>, <linux/tcp.h>, <linux/udp.h>, <bpf/bpf_helpers.h>,
// <bpf/bpf_endian.h>

/* veth_src --- veth_src_fwd --- veth_det_fwd --- veth_dst
 *           |                                 |
 *  ns_src   |              ns_fwd             |   ns_dst
 *
 * ns_src and ns_dst: ENDHOST namespace
 *            ns_fwd: Fowarding namespace
 */

const IP4_SRC: __u32 = __bpf_htonl(0xac100164); /* 172.16.1.100 */
const IP4_DST: __u32 = __bpf_htonl(0xac100264); /* 172.16.2.100 */

const IP6_SRC: [__u8; 16] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xde, 0xad, 0xbe, 0xef,
    0xca, 0xfe,
];
const IP6_DST: [__u8; 16] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xde, 0xad, 0xbe, 0xef,
    0xca, 0xfe,
];

extern "C" {
    static IFINDEX_SRC: __u32;
    static IFINDEX_DST: __u32;
}

const EGRESS_ENDHOST_MAGIC: __u64 = 0x0b9fbeef;
const INGRESS_FWDNS_MAGIC: __u64 = 0x1b9fbeef;
const EGRESS_FWDNS_MAGIC: __u64 = 0x2b9fbeef;

const INGRESS_FWDNS_P100: usize = 0;
const INGRESS_FWDNS_P101: usize = 1;
const EGRESS_FWDNS_P100: usize = 2;
const EGRESS_FWDNS_P101: usize = 3;
const INGRESS_ENDHOST: usize = 4;
const EGRESS_ENDHOST: usize = 5;
const SET_DTIME: usize = 6;
const __MAX_CNT: usize = 7;

const TCP_IP6_CLEAR_DTIME: usize = 0;
const TCP_IP4: usize = 1;
const TCP_IP6: usize = 2;
const UDP_IP4: usize = 3;
const UDP_IP6: usize = 4;
const TCP_IP4_RT_FWD: usize = 5;
const TCP_IP6_RT_FWD: usize = 6;
const UDP_IP4_RT_FWD: usize = 7;
const UDP_IP6_RT_FWD: usize = 8;
const UKN_TEST: usize = 9;
const __NR_TESTS: usize = 10;

const SRC_NS: i32 = 1;
const DST_NS: i32 = 2;

#[no_mangle]
static mut dtimes: [[__u32; __MAX_CNT]; __NR_TESTS] = [[0; __MAX_CNT]; __NR_TESTS];
#[no_mangle]
static mut errs: [[__u32; __MAX_CNT]; __NR_TESTS] = [[0; __MAX_CNT]; __NR_TESTS];
#[no_mangle]
static mut test: __u32 = 0;

unsafe fn inc_dtimes(idx: __u32) {
    if test < __NR_TESTS as __u32 {
        dtimes[test as usize][idx as usize] = dtimes[test as usize][idx as usize].wrapping_add(1);
    } else {
        dtimes[UKN_TEST][idx as usize] = dtimes[UKN_TEST][idx as usize].wrapping_add(1);
    }
}

unsafe fn inc_errs(idx: __u32) {
    if test < __NR_TESTS as __u32 {
        errs[test as usize][idx as usize] = errs[test as usize][idx as usize].wrapping_add(1);
    } else {
        errs[UKN_TEST][idx as usize] = errs[UKN_TEST][idx as usize].wrapping_add(1);
    }
}

fn skb_proto(type_: i32) -> i32 {
    type_ & 0xff
}

fn skb_ns(type_: i32) -> i32 {
    (type_ >> 8) & 0xff
}

unsafe fn fwdns_clear_dtime() -> bool {
    test == TCP_IP6_CLEAR_DTIME as __u32
}

unsafe fn bpf_fwd() -> bool {
    test < TCP_IP4_RT_FWD as __u32
}

unsafe fn get_proto() -> __u8 {
    match test as usize {
        UDP_IP4 | UDP_IP6 | UDP_IP4_RT_FWD | UDP_IP6_RT_FWD => IPPROTO_UDP as __u8,
        _ => IPPROTO_TCP as __u8,
    }
}

unsafe fn v6_equal(a: in6_addr, b: in6_addr) -> bool {
    a.s6_addr32[0] == b.s6_addr32[0]
        && a.s6_addr32[1] == b.s6_addr32[1]
        && a.s6_addr32[2] == b.s6_addr32[2]
        && a.s6_addr32[3] == b.s6_addr32[3]
}

/* -1: parse error: TC_ACT_SHOT
 *  0: not testing traffic: TC_ACT_OK
 * >0: first byte is the inet_proto, second byte has the netns
 *     of the sender
 */
unsafe fn skb_get_type(skb: *mut __sk_buff) -> i32 {
    let dst_ns_port: __u16 = __bpf_htons((50000u32).wrapping_add(test) as __u16);
    let data_end = (*skb).data_end as usize as *mut core::ffi::c_void;
    let data = (*skb).data as usize as *mut core::ffi::c_void;
    let mut inet_proto: __u8 = 0;
    let mut ns: __u8 = 0;
    let ip6h: *mut ipv6hdr;
    let mut sport: __u16;
    let mut dport: __u16;
    let iph: *mut iphdr;
    let th: *mut tcphdr;
    let uh: *mut udphdr;
    let trans: *mut core::ffi::c_void;

    match (*skb).protocol {
        x if x == __bpf_htons(ETH_P_IP as __u16) => {
            iph = (data as *mut __u8).add(core::mem::size_of::<ethhdr>()) as *mut iphdr;
            if iph.add(1) as *mut core::ffi::c_void > data_end {
                return -1;
            }
            if (*iph).saddr == IP4_SRC {
                ns = SRC_NS as __u8;
            } else if (*iph).saddr == IP4_DST {
                ns = DST_NS as __u8;
            }
            inet_proto = (*iph).protocol;
            trans = iph.add(1) as *mut core::ffi::c_void;
        }
        x if x == __bpf_htons(ETH_P_IPV6 as __u16) => {
            ip6h = (data as *mut __u8).add(core::mem::size_of::<ethhdr>()) as *mut ipv6hdr;
            if ip6h.add(1) as *mut core::ffi::c_void > data_end {
                return -1;
            }
            if v6_equal((*ip6h).saddr, in6_addr { s6_addr: IP6_SRC }) {
                ns = SRC_NS as __u8;
            } else if v6_equal((*ip6h).saddr, in6_addr { s6_addr: IP6_DST }) {
                ns = DST_NS as __u8;
            }
            inet_proto = (*ip6h).nexthdr;
            trans = ip6h.add(1) as *mut core::ffi::c_void;
        }
        _ => return 0,
    }

    /* skb is not from src_ns or dst_ns.
     * skb is not the testing IPPROTO.
     */
    if ns == 0 || inet_proto != get_proto() {
        return 0;
    }

    match inet_proto as i32 {
        IPPROTO_TCP => {
            th = trans as *mut tcphdr;
            if th.add(1) as *mut core::ffi::c_void > data_end {
                return -1;
            }
            sport = (*th).source;
            dport = (*th).dest;
        }
        IPPROTO_UDP => {
            uh = trans as *mut udphdr;
            if uh.add(1) as *mut core::ffi::c_void > data_end {
                return -1;
            }
            sport = (*uh).source;
            dport = (*uh).dest;
        }
        _ => return 0,
    }

    /* The skb is the testing traffic */
    if (ns as i32 == SRC_NS && dport == dst_ns_port) || (ns as i32 == DST_NS && sport == dst_ns_port)
    {
        return ((ns as i32) << 8) | inet_proto as i32;
    }

    0
}

/* format: direction@iface@netns
 * egress@veth_(src|dst)@ns_(src|dst)
 */
#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn egress_host(skb: *mut __sk_buff) -> i32 {
    let skb_type: i32;

    skb_type = skb_get_type(skb);
    if skb_type == -1 {
        return TC_ACT_SHOT;
    }
    if skb_type == 0 {
        return TC_ACT_OK;
    }

    if skb_proto(skb_type) == IPPROTO_TCP {
        if (*skb).tstamp_type == BPF_SKB_CLOCK_MONOTONIC && (*skb).tstamp != 0 {
            inc_dtimes(EGRESS_ENDHOST as __u32);
        } else {
            inc_errs(EGRESS_ENDHOST as __u32);
        }
    } else if skb_proto(skb_type) == IPPROTO_UDP {
        if (*skb).tstamp_type == BPF_SKB_CLOCK_TAI && (*skb).tstamp != 0 {
            inc_dtimes(EGRESS_ENDHOST as __u32);
        } else {
            inc_errs(EGRESS_ENDHOST as __u32);
        }
    } else if (*skb).tstamp_type == BPF_SKB_CLOCK_REALTIME && (*skb).tstamp != 0 {
        inc_errs(EGRESS_ENDHOST as __u32);
    }

    (*skb).tstamp = EGRESS_ENDHOST_MAGIC;

    TC_ACT_OK
}

/* ingress@veth_(src|dst)@ns_(src|dst) */
#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn ingress_host(skb: *mut __sk_buff) -> i32 {
    let skb_type: i32;

    skb_type = skb_get_type(skb);
    if skb_type == -1 {
        return TC_ACT_SHOT;
    }
    if skb_type == 0 {
        return TC_ACT_OK;
    }

    if (*skb).tstamp_type == BPF_SKB_CLOCK_MONOTONIC && (*skb).tstamp == EGRESS_FWDNS_MAGIC {
        inc_dtimes(INGRESS_ENDHOST as __u32);
    } else {
        inc_errs(INGRESS_ENDHOST as __u32);
    }

    TC_ACT_OK
}

/* ingress@veth_(src|dst)_fwd@ns_fwd priority 100 */
#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn ingress_fwdns_prio100(skb: *mut __sk_buff) -> i32 {
    let skb_type: i32;

    skb_type = skb_get_type(skb);
    if skb_type == -1 {
        return TC_ACT_SHOT;
    }
    if skb_type == 0 {
        return TC_ACT_OK;
    }

    /* delivery_time is only available to the ingress
     * if the tc-bpf checks the skb->tstamp_type.
     */
    if (*skb).tstamp == EGRESS_ENDHOST_MAGIC {
        inc_errs(INGRESS_FWDNS_P100 as __u32);
    }

    if fwdns_clear_dtime() {
        (*skb).tstamp = 0;
    }

    TC_ACT_UNSPEC
}

/* egress@veth_(src|dst)_fwd@ns_fwd priority 100 */
#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn egress_fwdns_prio100(skb: *mut __sk_buff) -> i32 {
    let skb_type: i32;

    skb_type = skb_get_type(skb);
    if skb_type == -1 {
        return TC_ACT_SHOT;
    }
    if skb_type == 0 {
        return TC_ACT_OK;
    }

    /* delivery_time is always available to egress even
     * the tc-bpf did not use the tstamp_type.
     */
    if (*skb).tstamp == INGRESS_FWDNS_MAGIC {
        inc_dtimes(EGRESS_FWDNS_P100 as __u32);
    } else {
        inc_errs(EGRESS_FWDNS_P100 as __u32);
    }

    if fwdns_clear_dtime() {
        (*skb).tstamp = 0;
    }

    TC_ACT_UNSPEC
}

/* ingress@veth_(src|dst)_fwd@ns_fwd priority 101 */
#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn ingress_fwdns_prio101(skb: *mut __sk_buff) -> i32 {
    let skb_type: i32;

    skb_type = skb_get_type(skb);
    if skb_type == -1 || skb_type == 0 {
        /* Should have handled in prio100 */
        return TC_ACT_SHOT;
    }

    if (*skb).tstamp_type != 0 {
        if fwdns_clear_dtime()
            || ((*skb).tstamp_type != BPF_SKB_CLOCK_MONOTONIC
                && (*skb).tstamp_type != BPF_SKB_CLOCK_TAI)
            || (*skb).tstamp != EGRESS_ENDHOST_MAGIC
        {
            inc_errs(INGRESS_FWDNS_P101 as __u32);
        } else {
            inc_dtimes(INGRESS_FWDNS_P101 as __u32);
        }
    } else if !fwdns_clear_dtime() {
        inc_errs(INGRESS_FWDNS_P101 as __u32);
    }

    if (*skb).tstamp_type == BPF_SKB_CLOCK_MONOTONIC {
        (*skb).tstamp = INGRESS_FWDNS_MAGIC;
    } else if bpf_skb_set_tstamp(skb, INGRESS_FWDNS_MAGIC, BPF_SKB_CLOCK_MONOTONIC) != 0 {
        inc_errs(SET_DTIME as __u32);
    }

    if skb_ns(skb_type) == SRC_NS {
        if bpf_fwd() {
            bpf_redirect_neigh(IFINDEX_DST, core::ptr::null_mut(), 0, 0)
        } else {
            TC_ACT_OK
        }
    } else if bpf_fwd() {
        bpf_redirect_neigh(IFINDEX_SRC, core::ptr::null_mut(), 0, 0)
    } else {
        TC_ACT_OK
    }
}

/* egress@veth_(src|dst)_fwd@ns_fwd priority 101 */
#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn egress_fwdns_prio101(skb: *mut __sk_buff) -> i32 {
    let skb_type: i32;

    skb_type = skb_get_type(skb);
    if skb_type == -1 || skb_type == 0 {
        /* Should have handled in prio100 */
        return TC_ACT_SHOT;
    }

    if (*skb).tstamp_type != 0 {
        if fwdns_clear_dtime()
            || (*skb).tstamp_type != BPF_SKB_CLOCK_MONOTONIC
            || (*skb).tstamp != INGRESS_FWDNS_MAGIC
        {
            inc_errs(EGRESS_FWDNS_P101 as __u32);
        } else {
            inc_dtimes(EGRESS_FWDNS_P101 as __u32);
        }
    } else if !fwdns_clear_dtime() {
        inc_errs(EGRESS_FWDNS_P101 as __u32);
    }

    if (*skb).tstamp_type == BPF_SKB_CLOCK_MONOTONIC {
        (*skb).tstamp = EGRESS_FWDNS_MAGIC;
    } else if bpf_skb_set_tstamp(skb, EGRESS_FWDNS_MAGIC, BPF_SKB_CLOCK_MONOTONIC) != 0 {
        inc_errs(SET_DTIME as __u32);
    }

    TC_ACT_OK
}

#[no_mangle]
#[link_section = "license"]
pub static __license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
