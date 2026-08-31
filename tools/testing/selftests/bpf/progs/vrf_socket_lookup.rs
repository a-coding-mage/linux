// SPDX-License-Identifier: GPL-2.0
// Translated from C source using Linux/BPF helper and protocol definitions.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::mem::size_of;

pub const BPF_F_CURRENT_NETNS: u64 = -1i64 as u64;
pub const CUR_NS: u64 = BPF_F_CURRENT_NETNS;

pub const ETH_P_IP: u16 = 0x0800;
pub const IPPROTO_TCP: u8 = 6;
pub const IPPROTO_UDP: u8 = 17;
pub const TC_ACT_UNSPEC: i32 = -1;
pub const XDP_PASS: i32 = 2;

#[inline]
pub const fn bpf_htons(x: u16) -> u16 {
    x.to_be()
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [u8; 6],
    pub h_source: [u8; 6],
    pub h_proto: u16,
}

#[repr(C)]
pub struct iphdr {
    pub ihl_version: u8,
    pub tos: u8,
    pub tot_len: u16,
    pub id: u16,
    pub frag_off: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub check: u16,
    pub saddr: u32,
    pub daddr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_sock_tuple_ipv4 {
    pub saddr: u32,
    pub daddr: u32,
    pub sport: u16,
    pub dport: u16,
}

#[repr(C)]
pub union bpf_sock_tuple {
    pub ipv4: bpf_sock_tuple_ipv4,
}

#[repr(C)]
pub struct bpf_sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __sk_buff {
    pub len: u32,
    pub pkt_type: u32,
    pub mark: u32,
    pub queue_mapping: u32,
    pub protocol: u32,
    pub vlan_present: u32,
    pub vlan_tci: u32,
    pub vlan_proto: u32,
    pub priority: u32,
    pub ingress_ifindex: u32,
    pub ifindex: u32,
    pub tc_index: u32,
    pub cb: [u32; 5],
    pub hash: u32,
    pub tc_classid: u32,
    pub data: u32,
    pub data_end: u32,
}

#[repr(C)]
pub struct xdp_md {
    pub data: u32,
    pub data_end: u32,
    pub data_meta: u32,
    pub ingress_ifindex: u32,
    pub rx_queue_index: u32,
}

unsafe extern "C" {
    pub fn bpf_skc_lookup_tcp(
        ctx: *mut core::ffi::c_void,
        tuple: *mut bpf_sock_tuple,
        tuple_size: i32,
        netns: u64,
        flags: u64,
    ) -> *mut bpf_sock;
    pub fn bpf_sk_lookup_tcp(
        ctx: *mut core::ffi::c_void,
        tuple: *mut bpf_sock_tuple,
        tuple_size: i32,
        netns: u64,
        flags: u64,
    ) -> *mut bpf_sock;
    pub fn bpf_sk_lookup_udp(
        ctx: *mut core::ffi::c_void,
        tuple: *mut bpf_sock_tuple,
        tuple_size: i32,
        netns: u64,
        flags: u64,
    ) -> *mut bpf_sock;
    pub fn bpf_sk_release(sock: *mut bpf_sock) -> i64;
}

#[unsafe(no_mangle)]
pub static mut lookup_status: i32 = 0;
#[unsafe(no_mangle)]
pub static mut test_xdp: bool = false;
#[unsafe(no_mangle)]
pub static mut tcp_skc: bool = false;

unsafe fn socket_lookup(
    ctx: *mut core::ffi::c_void,
    data_end: *mut core::ffi::c_void,
    data: *mut core::ffi::c_void,
) {
    let eth: *mut ethhdr = data as *mut ethhdr;
    let tp: *mut bpf_sock_tuple;
    let mut sk: *mut bpf_sock;
    let iph: *mut iphdr;
    let tplen: i32;

    if eth.add(1) as *mut core::ffi::c_void > data_end {
        return;
    }

    if (*eth).h_proto != bpf_htons(ETH_P_IP) {
        return;
    }

    iph = eth.add(1) as *mut iphdr;
    if iph.add(1) as *mut core::ffi::c_void > data_end {
        return;
    }

    tp = core::ptr::addr_of_mut!((*iph).saddr) as *mut bpf_sock_tuple;
    tplen = size_of::<bpf_sock_tuple_ipv4>() as i32;
    if (tp as *mut u8).add(tplen as usize) as *mut core::ffi::c_void > data_end {
        return;
    }

    match (*iph).protocol {
        IPPROTO_TCP => {
            if tcp_skc {
                sk = bpf_skc_lookup_tcp(ctx, tp, tplen, CUR_NS, 0);
            } else {
                sk = bpf_sk_lookup_tcp(ctx, tp, tplen, CUR_NS, 0);
            }
        }
        IPPROTO_UDP => {
            sk = bpf_sk_lookup_udp(ctx, tp, tplen, CUR_NS, 0);
        }
        _ => {
            return;
        }
    }

    lookup_status = 0;

    if !sk.is_null() {
        bpf_sk_release(sk);
        lookup_status = 1;
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "tc")]
pub unsafe extern "C" fn tc_socket_lookup(skb: *mut __sk_buff) -> i32 {
    let data_end: *mut core::ffi::c_void = (*skb).data_end as usize as *mut core::ffi::c_void;
    let data: *mut core::ffi::c_void = (*skb).data as usize as *mut core::ffi::c_void;

    if test_xdp {
        return TC_ACT_UNSPEC;
    }

    socket_lookup(skb as *mut core::ffi::c_void, data_end, data);
    TC_ACT_UNSPEC
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "xdp")]
pub unsafe extern "C" fn xdp_socket_lookup(xdp: *mut xdp_md) -> i32 {
    let data_end: *mut core::ffi::c_void = (*xdp).data_end as usize as *mut core::ffi::c_void;
    let data: *mut core::ffi::c_void = (*xdp).data as usize as *mut core::ffi::c_void;

    if !test_xdp {
        return XDP_PASS;
    }

    socket_lookup(xdp as *mut core::ffi::c_void, data_end, data);
    XDP_PASS
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static _license: [u8; 4] = *b"GPL\0";
