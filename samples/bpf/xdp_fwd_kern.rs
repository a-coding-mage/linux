// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2017-18 David Ahern <dsahern@gmail.com>
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 */

// C includes and build-time BPF declarations are supplied by the surrounding
// environment.

const IPV6_FLOWINFO_MASK: u32 = 0x0fff_ffffu32.to_be();

#[repr(C)]
pub struct xdp_md {
    pub data: u32,
    pub data_end: u32,
    pub data_meta: u32,
    pub ingress_ifindex: u32,
    pub rx_queue_index: u32,
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
pub struct ipv6hdr {
    pub version_priority_flowlabel: u32,
    pub payload_len: u16,
    pub nexthdr: u8,
    pub hop_limit: u8,
    pub saddr: [u8; 16],
    pub daddr: [u8; 16],
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [u8; 6],
    pub h_source: [u8; 6],
    pub h_proto: u16,
}

#[repr(C)]
pub struct bpf_fib_lookup {
    pub family: u8,
    pub l4_protocol: u8,
    pub sport: u16,
    pub dport: u16,
    pub tot_len: u16,
    pub tos: u8,
    pub flowinfo: u32,
    pub ipv4_src: u32,
    pub ipv4_dst: u32,
    pub ipv6_src: [u8; 16],
    pub ipv6_dst: [u8; 16],
    pub ifindex: u32,
    pub dmac: [u8; 6],
    pub smac: [u8; 6],
}

#[repr(C)]
pub struct xdp_tx_ports_t {
    _private: [u8; 0],
}

extern "C" {
    static mut xdp_tx_ports: xdp_tx_ports_t;
    fn bpf_fib_lookup(ctx: *mut xdp_md, params: *mut bpf_fib_lookup, plen: u32, flags: u32) -> i32;
    fn bpf_map_lookup_elem(map: *mut xdp_tx_ports_t, key: *const u32) -> *mut core::ffi::c_void;
    fn bpf_redirect_map(map: *mut xdp_tx_ports_t, key: u32, flags: u64) -> i32;
}

const XDP_DROP: i32 = 1;
const XDP_PASS: i32 = 2;
const BPF_FIB_LKUP_RET_SUCCESS: i32 = 0;
const BPF_FIB_LOOKUP_DIRECT: u32 = 1;
const AF_INET: u8 = 2;
const AF_INET6: u8 = 10;
const ETH_P_IP: u16 = 0x0800;
const ETH_P_IPV6: u16 = 0x86dd;

#[inline(always)]
unsafe fn ip_decrease_ttl(iph: *mut iphdr) -> i32 {
    let mut check = (*iph).check as u32;
    check = check.wrapping_add((0x0100u16.to_be()) as u32);
    (*iph).check = check.wrapping_add((check >= 0xffff) as u32) as u16;
    (*iph).ttl = (*iph).ttl.wrapping_sub(1);
    (*iph).ttl as i32
}

#[inline(always)]
unsafe fn xdp_fwd_flags(ctx: *mut xdp_md, flags: u32) -> i32 {
    let data_end = (*ctx).data_end as usize as *mut u8;
    let data = (*ctx).data as usize as *mut u8;
    let mut fib_params: bpf_fib_lookup = core::mem::zeroed();
    let eth = data as *mut ethhdr;
    let mut iph: *mut iphdr = core::ptr::null_mut();
    let mut ip6h: *mut ipv6hdr = core::ptr::null_mut();
    let h_proto = (*eth).h_proto;
    let nh_off = core::mem::size_of::<ethhdr>();

    if data.add(nh_off) > data_end { return XDP_DROP; }

    if h_proto == ETH_P_IP.to_be() {
        iph = data.add(nh_off) as *mut iphdr;
        if (iph.add(1) as *mut u8) > data_end { return XDP_DROP; }
        if (*iph).ttl <= 1 { return XDP_PASS; }
        fib_params.family = AF_INET;
        fib_params.tos = (*iph).tos;
        fib_params.l4_protocol = (*iph).protocol;
        fib_params.tot_len = u16::from_be((*iph).tot_len);
        fib_params.ipv4_src = (*iph).saddr;
        fib_params.ipv4_dst = (*iph).daddr;
    } else if h_proto == ETH_P_IPV6.to_be() {
        ip6h = data.add(nh_off) as *mut ipv6hdr;
        if (ip6h.add(1) as *mut u8) > data_end { return XDP_DROP; }
        if (*ip6h).hop_limit <= 1 { return XDP_PASS; }
        fib_params.family = AF_INET6;
        fib_params.flowinfo = (*ip6h).version_priority_flowlabel & IPV6_FLOWINFO_MASK;
        fib_params.l4_protocol = (*ip6h).nexthdr;
        fib_params.tot_len = u16::from_be((*ip6h).payload_len);
        fib_params.ipv6_src = (*ip6h).saddr;
        fib_params.ipv6_dst = (*ip6h).daddr;
    } else { return XDP_PASS; }

    fib_params.ifindex = (*ctx).ingress_ifindex;
    let rc = bpf_fib_lookup(ctx, &mut fib_params, core::mem::size_of::<bpf_fib_lookup>() as u32, flags);
    if rc == BPF_FIB_LKUP_RET_SUCCESS {
        if bpf_map_lookup_elem(&mut xdp_tx_ports, &fib_params.ifindex).is_null() { return XDP_PASS; }
        if h_proto == ETH_P_IP.to_be() { ip_decrease_ttl(iph); }
        else if h_proto == ETH_P_IPV6.to_be() { (*ip6h).hop_limit = (*ip6h).hop_limit.wrapping_sub(1); }
        core::ptr::copy_nonoverlapping(fib_params.dmac.as_ptr(), (*eth).h_dest.as_mut_ptr(), 6);
        core::ptr::copy_nonoverlapping(fib_params.smac.as_ptr(), (*eth).h_source.as_mut_ptr(), 6);
        return bpf_redirect_map(&mut xdp_tx_ports, fib_params.ifindex, 0);
    }
    XDP_PASS
}

#[no_mangle]
pub unsafe extern "C" fn xdp_fwd_prog(ctx: *mut xdp_md) -> i32 { xdp_fwd_flags(ctx, 0) }

#[no_mangle]
pub unsafe extern "C" fn xdp_fwd_direct_prog(ctx: *mut xdp_md) -> i32 {
    xdp_fwd_flags(ctx, BPF_FIB_LOOKUP_DIRECT)
}

#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
