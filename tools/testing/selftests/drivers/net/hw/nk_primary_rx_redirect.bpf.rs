// SPDX-License-Identifier: GPL-2.0
// C dependencies translated as external declarations:
// <linux/bpf.h>, <linux/pkt_cls.h>, <linux/if_ether.h>, <linux/in.h>,
// <linux/ipv6.h>, <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub type __u32 = u32;
pub type __u16 = u16;
pub type __u8 = u8;

pub const TC_ACT_OK: i32 = 0;
pub const ETH_P_IPV6: __u16 = 0x86DD;
pub const IPPROTO_ICMPV6: __u8 = 58;

#[repr(C)]
pub struct __sk_buff {
    pub data: __u32,
    pub data_end: __u32,
}

#[repr(C, packed)]
pub struct ethhdr {
    pub h_dest: [__u8; 6],
    pub h_source: [__u8; 6],
    pub h_proto: __u16,
}

#[repr(C)]
pub struct ipv6hdr {
    pub _opaque_prefix: [__u8; 6],
    pub nexthdr: __u8,
    pub _opaque_suffix: [__u8; 33],
}

extern "C" {
    pub fn bpf_htons(x: __u16) -> __u16;
    pub fn bpf_redirect_neigh(ifindex: __u32, params: *mut core::ffi::c_void, plen: i32, flags: __u32) -> i32;
}

#[inline]
unsafe fn ctx_ptr(field: __u32) -> *mut core::ffi::c_void {
    field as isize as *mut core::ffi::c_void
}

#[no_mangle]
pub static mut phys_ifindex: __u32 = 0;

// SEC("tc/ingress")
#[no_mangle]
pub unsafe extern "C" fn nk_primary_rx_redirect(skb: *mut __sk_buff) -> i32 {
    let data_end: *mut core::ffi::c_void = ctx_ptr((*skb).data_end);
    let data: *mut core::ffi::c_void = ctx_ptr((*skb).data);
    let mut eth: *mut ethhdr;
    let mut ip6h: *mut ipv6hdr;

    eth = data as *mut ethhdr;
    if eth.add(1) as *mut core::ffi::c_void > data_end {
        return TC_ACT_OK;
    }

    if (*eth).h_proto != bpf_htons(ETH_P_IPV6) {
        return TC_ACT_OK;
    }

    ip6h = (data as *mut __u8).add(core::mem::size_of::<ethhdr>()) as *mut ipv6hdr;
    if ip6h.add(1) as *mut core::ffi::c_void > data_end {
        return TC_ACT_OK;
    }

    if (*ip6h).nexthdr == IPPROTO_ICMPV6 {
        return TC_ACT_OK;
    }

    bpf_redirect_neigh(phys_ifindex, core::ptr::null_mut(), 0, 0)
}

// SEC("license")
#[no_mangle]
pub static __license: [u8; 4] = *b"GPL\0";
