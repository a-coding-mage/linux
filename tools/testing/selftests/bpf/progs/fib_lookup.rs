// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

/* Translated from C includes:
 * <linux/types.h>, <linux/bpf.h>, <linux/pkt_cls.h>, <linux/if_ether.h>,
 * <linux/ip.h>, <linux/in.h>, <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __be16 = u16;

pub const TC_ACT_SHOT: i32 = 2;
pub const XDP_DROP: i32 = 1;
pub const XDP_PASS: i32 = 2;
pub const BPF_FIB_LKUP_RET_SUCCESS: i64 = 0;
pub const ETH_P_IP: u16 = 0x0800;
pub const IPPROTO_TCP: u8 = 6;

#[repr(C)]
#[derive(Copy, Clone)]
pub union bpf_fib_lookup_addrs {
    pub ipv4: bpf_fib_lookup_ipv4,
    pub ipv6: bpf_fib_lookup_ipv6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_fib_lookup_ipv4 {
    pub ipv4_src: __u32,
    pub ipv4_dst: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_fib_lookup_ipv6 {
    pub ipv6_src: [__u32; 4],
    pub ipv6_dst: [__u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_fib_lookup {
    pub family: __u8,
    pub l4_protocol: __u8,
    pub sport: __be16,
    pub dport: __be16,
    pub tot_len: __u16,
    pub ifindex: __u32,
    pub addrs: bpf_fib_lookup_addrs,
    pub smac: [__u8; 6],
    pub dmac: [__u8; 6],
    pub h_vlan_proto: __be16,
    pub h_vlan_TCI: __be16,
}

#[repr(C)]
pub struct __sk_buff {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct xdp_md {
    pub data: __u32,
    pub data_end: __u32,
    pub data_meta: __u32,
    pub ingress_ifindex: __u32,
    pub rx_queue_index: __u32,
    pub egress_ifindex: __u32,
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [u8; 6],
    pub h_source: [u8; 6],
    pub h_proto: __be16,
}

#[repr(C)]
pub struct iphdr {
    pub ihl_version: __u8,
    pub tos: __u8,
    pub tot_len: __be16,
    pub id: __be16,
    pub frag_off: __be16,
    pub ttl: __u8,
    pub protocol: __u8,
    pub check: __be16,
    pub saddr: __u32,
    pub daddr: __u32,
}

unsafe extern "C" {
    pub fn bpf_fib_lookup(
        ctx: *mut core::ffi::c_void,
        params: *mut bpf_fib_lookup,
        plen: i32,
        flags: __u32,
    ) -> i64;
    pub fn bpf_redirect(ifindex: __u32, flags: __u64) -> i32;
}

pub type __u64 = u64;

#[inline]
pub const fn bpf_htons(x: u16) -> u16 {
    x.to_be()
}

#[no_mangle]
pub static mut fib_params: bpf_fib_lookup = bpf_fib_lookup {
    family: 0,
    l4_protocol: 0,
    sport: 0,
    dport: 0,
    tot_len: 0,
    ifindex: 0,
    addrs: bpf_fib_lookup_addrs {
        ipv6: bpf_fib_lookup_ipv6 {
            ipv6_src: [0; 4],
            ipv6_dst: [0; 4],
        },
    },
    smac: [0; 6],
    dmac: [0; 6],
    h_vlan_proto: 0,
    h_vlan_TCI: 0,
};
#[no_mangle]
pub static mut fib_lookup_ret: i32 = 0;
#[no_mangle]
pub static mut lookup_flags: i32 = 0;

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn fib_lookup(skb: *mut __sk_buff) -> i32 {
    fib_lookup_ret = bpf_fib_lookup(
        skb as *mut core::ffi::c_void,
        core::ptr::addr_of_mut!(fib_params),
        core::mem::size_of::<bpf_fib_lookup>() as i32,
        lookup_flags as __u32,
    ) as i32;

    TC_ACT_SHOT
}

#[no_mangle]
#[link_section = "xdp"]
pub unsafe extern "C" fn fib_lookup_xdp(ctx: *mut xdp_md) -> i32 {
    fib_lookup_ret = bpf_fib_lookup(
        ctx as *mut core::ffi::c_void,
        core::ptr::addr_of_mut!(fib_params),
        core::mem::size_of::<bpf_fib_lookup>() as i32,
        lookup_flags as __u32,
    ) as i32;

    XDP_DROP
}

#[no_mangle]
pub static mut redirected: i32 = 0;
#[no_mangle]
pub static mut passed: i32 = 0;
#[no_mangle]
pub static mut delivered: i32 = 0;

#[no_mangle]
#[link_section = "xdp"]
pub unsafe extern "C" fn fib_lookup_redirect(ctx: *mut xdp_md) -> i32 {
    let mut params: bpf_fib_lookup = fib_params;
    let ret: i64;

    ret = bpf_fib_lookup(
        ctx as *mut core::ffi::c_void,
        &mut params,
        core::mem::size_of_val(&params) as i32,
        lookup_flags as __u32,
    );
    if ret == BPF_FIB_LKUP_RET_SUCCESS {
        redirected += 1;
        return bpf_redirect(params.ifindex, 0);
    }

    passed += 1;
    XDP_PASS
}

#[no_mangle]
#[link_section = "xdp"]
pub unsafe extern "C" fn xdp_count(ctx: *mut xdp_md) -> i32 {
    let data = (*ctx).data as usize as *mut core::ffi::c_void;
    let data_end = (*ctx).data_end as usize as *mut core::ffi::c_void;
    let eth = data as *mut ethhdr;
    let mut iph: *mut iphdr;

    /*
     * count only the test's TCP frames: the netns has live
     * link-local traffic (DAD, MLD) that would satisfy a bare
     * counter
     */
    if (eth.add(1) as *mut core::ffi::c_void) > data_end
        || (*eth).h_proto != bpf_htons(ETH_P_IP)
    {
        return XDP_DROP;
    }
    iph = eth.add(1) as *mut core::ffi::c_void as *mut iphdr;
    if (iph.add(1) as *mut core::ffi::c_void) > data_end || (*iph).protocol != IPPROTO_TCP {
        return XDP_DROP;
    }

    delivered += 1;
    XDP_DROP
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
