// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2017 Facebook
 */
// C includes translated as external Rust dependencies/import expectations:
// <stddef.h>, <string.h>, <linux/bpf.h>, <linux/if_ether.h>,
// <linux/if_packet.h>, <linux/ip.h>, <linux/ipv6.h>, <linux/in.h>,
// <linux/tcp.h>, <linux/pkt_cls.h>, <bpf/bpf_helpers.h>,
// <bpf/bpf_endian.h>, and "bpf_misc.h".

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;

const ETH_P_IP: __u16 = 0x0800;
const ETH_P_IPV6: __u16 = 0x86DD;
const TC_ACT_UNSPEC: i32 = -1;
const TC_ACT_OK: i32 = 0;
const TC_ACT_SHOT: i32 = 2;

#[repr(C)]
pub struct __sk_buff {
    pub len: __u32,
    _pad0: [u8; 12],
    pub ifindex: __u32,
    _pad1: [u8; 56],
    pub data: __u32,
    pub data_end: __u32,
}

#[repr(C, packed)]
pub struct ethhdr {
    pub h_dest: [u8; 6],
    pub h_source: [u8; 6],
    pub h_proto: __u16,
}

#[repr(C)]
pub struct iphdr {
    pub ihl_version: __u8,
    pub tos: __u8,
    pub tot_len: __u16,
    pub id: __u16,
    pub frag_off: __u16,
    pub ttl: __u8,
    pub protocol: __u8,
    pub check: __u16,
    pub saddr: __u32,
    pub daddr: __u32,
}

impl iphdr {
    unsafe fn ihl(&self) -> __u8 {
        self.ihl_version & 0x0f
    }
}

#[repr(C)]
pub struct ipv6hdr {
    pub priority_version: __u8,
    pub flow_lbl: [__u8; 3],
    pub payload_len: __u16,
    pub nexthdr: __u8,
    pub hop_limit: __u8,
    pub saddr: [__u8; 16],
    pub daddr: [__u8; 16],
}

#[repr(C)]
pub struct tcphdr {
    pub source: __u16,
    pub dest: __u16,
    pub seq: __u32,
    pub ack_seq: __u32,
    pub doff_res1: __u16,
    pub window: __u16,
    pub check: __u16,
    pub urg_ptr: __u16,
}

extern "C" {
    fn __sink(arg: __u8);
}

#[inline(always)]
fn bpf_htons(x: __u16) -> __u16 {
    x.to_be()
}

#[inline(always)]
fn barrier() {
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

/* llvm will optimize both subprograms into exactly the same BPF assembly
 *
 * Disassembly of section .text:
 *
 * 0000000000000000 test_pkt_access_subprog1:
 * ; 	return skb->len * 2;
 *        0:	61 10 00 00 00 00 00 00	r0 = *(u32 *)(r1 + 0)
 *        1:	64 00 00 00 01 00 00 00	w0 <<= 1
 *        2:	95 00 00 00 00 00 00 00	exit
 *
 * 0000000000000018 test_pkt_access_subprog2:
 * ; 	return skb->len * val;
 *        3:	61 10 00 00 00 00 00 00	r0 = *(u32 *)(r1 + 0)
 *        4:	64 00 00 00 01 00 00 00	w0 <<= 1
 *        5:	95 00 00 00 00 00 00 00	exit
 *
 * Which makes it an interesting test for BTF-enabled verifier.
 */
#[inline(never)]
unsafe fn test_pkt_access_subprog1(skb: *mut __sk_buff) -> i32 {
    core::ptr::addr_of!((*skb).len).read_volatile().wrapping_mul(2) as i32
}

#[inline(never)]
unsafe fn test_pkt_access_subprog2(val: i32, skb: *mut __sk_buff) -> i32 {
    (core::ptr::addr_of!((*skb).len).read_volatile() as i32).wrapping_mul(val)
}

const MAX_STACK: usize = 512 - 2 * 32;

#[inline(never)]
pub unsafe extern "C" fn get_skb_len(skb: *mut __sk_buff) -> i32 {
    let buf: [__u8; MAX_STACK] = [0; MAX_STACK];

    __sink(core::ptr::addr_of!(buf[MAX_STACK - 1]).read_volatile());

    (*skb).len as i32
}

#[inline(never)]
pub extern "C" fn get_constant(val: i64) -> i32 {
    (val - 122) as i32
}

#[inline(never)]
pub unsafe extern "C" fn test_pkt_access_subprog3(val: i32, skb: *mut __sk_buff) -> i32 {
    get_skb_len(skb).wrapping_mul(get_skb_ifindex(val, skb, get_constant(123)))
}

#[inline(never)]
pub unsafe extern "C" fn get_skb_ifindex(val: i32, skb: *mut __sk_buff, var: i32) -> i32 {
    let buf: [__u8; MAX_STACK] = [0; MAX_STACK];

    __sink(core::ptr::addr_of!(buf[MAX_STACK - 1]).read_volatile());

    ((*skb).ifindex as i32).wrapping_mul(val).wrapping_mul(var)
}

#[inline(never)]
pub unsafe extern "C" fn test_pkt_write_access_subprog(skb: *mut __sk_buff, off: __u32) -> i32 {
    let data = (*skb).data as usize as *mut core::ffi::c_void;
    let data_end = (*skb).data_end as usize as *mut core::ffi::c_void;
    let mut tcp: *mut tcphdr = core::ptr::null_mut();

    if (off as usize) > core::mem::size_of::<ethhdr>() + core::mem::size_of::<ipv6hdr>() {
        return -1;
    }

    tcp = (data as *mut u8).add(off as usize) as *mut tcphdr;
    if tcp.add(1) as *mut core::ffi::c_void > data_end {
        return -1;
    }
    /* make modification to the packet data */
    (*tcp).check = (*tcp).check.wrapping_add(1);
    0
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn test_pkt_access(skb: *mut __sk_buff) -> i32 {
    let data_end = (*skb).data_end as usize as *mut core::ffi::c_void;
    let data = (*skb).data as usize as *mut core::ffi::c_void;
    let eth = data as *mut ethhdr;
    let mut tcp: *mut tcphdr = core::ptr::null_mut();
    let mut proto: __u8 = 255;
    let mut ihl_len: __u64;

    if eth.add(1) as *mut core::ffi::c_void > data_end {
        return TC_ACT_SHOT;
    }

    if (*eth).h_proto == bpf_htons(ETH_P_IP) {
        let iph = eth.add(1) as *mut iphdr;

        if iph.add(1) as *mut core::ffi::c_void > data_end {
            return TC_ACT_SHOT;
        }
        ihl_len = ((*iph).ihl() as __u64).wrapping_mul(4);
        proto = (*iph).protocol;
        tcp = (iph as *mut u8).add(ihl_len as usize) as *mut tcphdr;
    } else if (*eth).h_proto == bpf_htons(ETH_P_IPV6) {
        let ip6h = eth.add(1) as *mut ipv6hdr;

        if ip6h.add(1) as *mut core::ffi::c_void > data_end {
            return TC_ACT_SHOT;
        }
        ihl_len = core::mem::size_of::<ipv6hdr>() as __u64;
        proto = (*ip6h).nexthdr;
        tcp = (ip6h as *mut u8).add(ihl_len as usize) as *mut tcphdr;
    }

    if test_pkt_access_subprog1(skb) != ((*skb).len as i32).wrapping_mul(2) {
        return TC_ACT_SHOT;
    }
    if test_pkt_access_subprog2(2, skb) != ((*skb).len as i32).wrapping_mul(2) {
        return TC_ACT_SHOT;
    }
    if test_pkt_access_subprog3(3, skb)
        != ((*skb).len as i32)
            .wrapping_mul(3)
            .wrapping_mul((*skb).ifindex as i32)
    {
        return TC_ACT_SHOT;
    }
    if !tcp.is_null() {
        if test_pkt_write_access_subprog(skb, (tcp as *mut u8).offset_from(data as *mut u8) as __u32) != 0 {
            return TC_ACT_SHOT;
        }
        if (tcp as *mut u8).add(20) as *mut core::ffi::c_void > data_end || proto != 6 {
            return TC_ACT_SHOT;
        }
        barrier(); /* to force ordering of checks */
        if (tcp as *mut u8).add(18) as *mut core::ffi::c_void > data_end {
            return TC_ACT_SHOT;
        }
        if (*tcp).urg_ptr == 123 {
            return TC_ACT_OK;
        }
    }

    TC_ACT_UNSPEC
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
