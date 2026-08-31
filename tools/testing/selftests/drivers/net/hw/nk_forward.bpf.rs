// SPDX-License-Identifier: GPL-2.0
// C dependencies translated from:
// <linux/bpf.h>, <linux/pkt_cls.h>, <linux/if_ether.h>, <linux/ipv6.h>,
// <linux/in6.h>, <bpf/bpf_endian.h>, <bpf/bpf_helpers.h>

pub const TC_ACT_OK: i32 = 0;
pub const ETH_P_IPV6: u16 = 0x86DD;

#[repr(C)]
pub struct __sk_buff {
    pub data: u32,
    pub data_end: u32,
    pub protocol: u32,
}

#[repr(C)]
pub struct in6_addr {
    pub s6_addr32: [u32; 4],
}

#[repr(C)]
pub struct ipv6hdr {
    pub daddr: in6_addr,
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [u8; 6],
    pub h_source: [u8; 6],
    pub h_proto: u16,
}

extern "C" {
    fn bpf_htons(x: u16) -> u16;
    fn bpf_redirect_peer(ifindex: u32, flags: u64) -> i32;
}

#[no_mangle]
pub static mut netkit_ifindex: u32 = 0;

#[no_mangle]
pub static mut ipv6_prefix: [u8; 16] = [0; 16];

#[inline(always)]
unsafe fn ctx_ptr(field: u32) -> *mut core::ffi::c_void {
    field as i64 as *mut core::ffi::c_void
}

#[inline(always)]
unsafe fn v6_p64_equal(a: &in6_addr, b: &in6_addr) -> bool {
    a.s6_addr32[0] == b.s6_addr32[0] && a.s6_addr32[1] == b.s6_addr32[1]
}

#[no_mangle]
#[link_section = "tc/ingress"]
pub unsafe extern "C" fn tc_redirect_peer(skb: *mut __sk_buff) -> i32 {
    let data_end = ctx_ptr((*skb).data_end);
    let data = ctx_ptr((*skb).data);
    let peer_addr: *mut in6_addr;
    let ip6h: *mut ipv6hdr;
    let eth: *mut ethhdr;

    peer_addr = ipv6_prefix.as_mut_ptr() as *mut in6_addr;

    if (*skb).protocol != bpf_htons(ETH_P_IPV6) as u32 {
        return TC_ACT_OK;
    }

    eth = data as *mut ethhdr;
    if eth.add(1) as *mut core::ffi::c_void > data_end {
        return TC_ACT_OK;
    }

    ip6h = (data as *mut u8).add(core::mem::size_of::<ethhdr>()) as *mut ipv6hdr;
    if ip6h.add(1) as *mut core::ffi::c_void > data_end {
        return TC_ACT_OK;
    }

    if !v6_p64_equal(&(*ip6h).daddr, &*peer_addr) {
        return TC_ACT_OK;
    }

    bpf_redirect_peer(netkit_ifindex, 0)
}

#[no_mangle]
#[link_section = "license"]
pub static __license: [u8; 4] = *b"GPL\0";
