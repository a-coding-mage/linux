/* Copyright (c) 2016 Facebook
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 *
 * This program shows how to use bpf_xdp_adjust_head() by
 * encapsulating the incoming packet in an IPv4/v6 header
 * and then XDP_TX it out.
 */

// KBUILD_MODNAME "foo"; dependencies supplied by the kernel/BPF build.
// The C headers and xdp_tx_iptunnel_common.h are external dependencies.

#[repr(C)]
pub struct xdp_md {
    pub data: u32,
    pub data_end: u32,
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [u8; 6],
    pub h_source: [u8; 6],
    pub h_proto: u16,
}

#[repr(C)]
pub struct tcphdr { pub dest: u16 }
#[repr(C)]
pub struct udphdr { pub dest: u16 }

#[repr(C)]
pub struct iphdr {
    pub ihl: u8, pub version: u8, pub tos: u8, pub tot_len: u16,
    pub id: u16, pub frag_off: u16, pub ttl: u8, pub protocol: u8,
    pub check: u16, pub saddr: u32, pub daddr: u32,
}

#[repr(C)]
pub struct in6_addr { pub s6_addr32: [u32; 4] }
#[repr(C)]
pub struct ipv6hdr {
    pub version: u8, pub priority: u8, pub flow_lbl: [u8; 3],
    pub payload_len: u16, pub nexthdr: u8, pub hop_limit: u8,
    pub saddr: in6_addr, pub daddr: in6_addr,
}

#[repr(C)]
pub union iptnl_addr { pub v4: u32, pub v6: [u32; 4] }
#[repr(C)]
pub struct iptnl_info {
    pub family: u16, pub daddr: iptnl_addr, pub saddr: iptnl_addr,
    pub dmac: [u8; 6],
}
#[repr(C)]
pub struct vip {
    pub protocol: u8, pub family: u16, pub daddr: iptnl_addr, pub dport: i32,
}

extern "C" {
    static mut rxcnt: core::ffi::c_void;
    static mut vip2tnl: core::ffi::c_void;
    fn bpf_map_lookup_elem(map: *const core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_xdp_adjust_head(xdp: *mut xdp_md, delta: i32) -> i64;
}

const XDP_DROP: i32 = 1;
const XDP_PASS: i32 = 2;
const XDP_TX: i32 = 3;
const AF_INET: u16 = 2;
const AF_INET6: u16 = 10;
const IPPROTO_TCP: u8 = 6;
const IPPROTO_UDP: u8 = 17;
const IPPROTO_IPIP: u8 = 4;
const IPPROTO_IPV6: u8 = 41;
const ETH_P_IP: u16 = 0x0800;
const ETH_P_IPV6: u16 = 0x86dd;

#[inline(always)]
unsafe fn count_tx(protocol: u32) {
    let rxcnt_count = bpf_map_lookup_elem(&rxcnt, &protocol as *const _ as *const _)
        as *mut u64;
    if !rxcnt_count.is_null() { *rxcnt_count += 1; }
}

#[inline(always)]
unsafe fn get_dport(trans_data: *mut core::ffi::c_void, data_end: *mut core::ffi::c_void, protocol: u8) -> i32 {
    match protocol {
        IPPROTO_TCP => {
            let th = trans_data as *mut tcphdr;
            if th.add(1) > data_end as *mut tcphdr { return -1; }
            (*th).dest as i32
        }
        IPPROTO_UDP => {
            let uh = trans_data as *mut udphdr;
            if uh.add(1) > data_end as *mut udphdr { return -1; }
            (*uh).dest as i32
        }
        _ => 0,
    }
}

#[inline(always)]
unsafe fn set_ethhdr(new_eth: *mut ethhdr, old_eth: *const ethhdr, tnl: *const iptnl_info, h_proto: u16) {
    (*new_eth).h_source = (*old_eth).h_dest;
    (*new_eth).h_dest = (*tnl).dmac;
    (*new_eth).h_proto = h_proto;
}

#[inline(always)]
unsafe fn handle_ipv4(xdp: *mut xdp_md) -> i32 {
    let mut data = (*xdp).data as usize as *mut u8;
    let mut data_end = (*xdp).data_end as usize as *mut u8;
    let mut iph = data.add(core::mem::size_of::<ethhdr>()) as *mut iphdr;
    if iph.add(1) as *mut u8 > data_end { return XDP_DROP; }
    let dport = get_dport(iph.add(1) as *mut _, data_end as *mut _, (*iph).protocol);
    if dport == -1 { return XDP_DROP; }
    let mut vip = core::mem::zeroed::<vip>();
    vip.protocol = (*iph).protocol; vip.family = AF_INET;
    vip.daddr.v4 = (*iph).daddr; vip.dport = dport;
    let payload_len = u16::from_be((*iph).tot_len);
    let tnl = bpf_map_lookup_elem(&vip2tnl, &vip as *const _ as *const _) as *mut iptnl_info;
    if tnl.is_null() || (*tnl).family != AF_INET { return XDP_PASS; }
    if bpf_xdp_adjust_head(xdp, -(core::mem::size_of::<iphdr>() as i32)) != 0 { return XDP_DROP; }
    data = (*xdp).data as usize as *mut u8; data_end = (*xdp).data_end as usize as *mut u8;
    let new_eth = data as *mut ethhdr; iph = data.add(core::mem::size_of::<ethhdr>()) as *mut iphdr;
    let old_eth = data.add(core::mem::size_of::<ethhdr>() + core::mem::size_of::<iphdr>()) as *mut ethhdr;
    if new_eth.add(1) as *mut u8 > data_end || old_eth.add(1) as *mut u8 > data_end || iph.add(1) as *mut u8 > data_end { return XDP_DROP; }
    set_ethhdr(new_eth, old_eth, tnl, ETH_P_IP.to_be());
    (*iph).version=4; (*iph).ihl=(core::mem::size_of::<iphdr>() >> 2) as u8; (*iph).frag_off=0; (*iph).protocol=IPPROTO_IPIP; (*iph).check=0; (*iph).tos=0; (*iph).tot_len=(payload_len + core::mem::size_of::<iphdr>() as u16).to_be(); (*iph).daddr=(*tnl).daddr.v4; (*iph).saddr=(*tnl).saddr.v4; (*iph).ttl=8;
    let words = iph as *mut u16; let mut csum=0u32; for i in 0..(core::mem::size_of::<iphdr>() >> 1) { csum += *words.add(i) as u32; } (*iph).check = !((csum & 0xffff) + (csum >> 16)) as u16;
    count_tx(vip.protocol as u32); XDP_TX
}

#[inline(always)]
unsafe fn handle_ipv6(xdp: *mut xdp_md) -> i32 {
    let data=(*xdp).data as usize as *mut u8; let data_end=(*xdp).data_end as usize as *mut u8;
    let ip6h=data.add(core::mem::size_of::<ethhdr>()) as *mut ipv6hdr;
    if ip6h.add(1) as *mut u8 > data_end { return XDP_DROP; }
    let dport=get_dport(ip6h.add(1) as *mut _, data_end as *mut _, (*ip6h).nexthdr); if dport == -1 { return XDP_DROP; }
    let mut vip=core::mem::zeroed::<vip>(); vip.protocol=(*ip6h).nexthdr; vip.family=AF_INET6; vip.daddr.v6=(*ip6h).daddr.s6_addr32; vip.dport=dport;
    let payload_len=(*ip6h).payload_len; let tnl=bpf_map_lookup_elem(&vip2tnl,&vip as *const _ as *const _) as *mut iptnl_info; if tnl.is_null() || (*tnl).family != AF_INET6 { return XDP_PASS; }
    if bpf_xdp_adjust_head(xdp,-(core::mem::size_of::<ipv6hdr>() as i32)) != 0 { return XDP_DROP; }
    let data=(*xdp).data as usize as *mut u8; let data_end=(*xdp).data_end as usize as *mut u8; let new_eth=data as *mut ethhdr; let ip6h=data.add(core::mem::size_of::<ethhdr>()) as *mut ipv6hdr; let old_eth=data.add(core::mem::size_of::<ethhdr>()+core::mem::size_of::<ipv6hdr>()) as *mut ethhdr;
    if new_eth.add(1) as *mut u8 > data_end || old_eth.add(1) as *mut u8 > data_end || ip6h.add(1) as *mut u8 > data_end { return XDP_DROP; }
    set_ethhdr(new_eth,old_eth,tnl,ETH_P_IPV6.to_be()); (*ip6h).version=6; (*ip6h).priority=0; (*ip6h).flow_lbl=[0;3]; (*ip6h).payload_len=(u16::from_be(payload_len)+core::mem::size_of::<ipv6hdr>() as u16).to_be(); (*ip6h).nexthdr=IPPROTO_IPV6; (*ip6h).hop_limit=8; (*ip6h).saddr.s6_addr32=(*tnl).saddr.v6; (*ip6h).daddr.s6_addr32=(*tnl).daddr.v6; count_tx(vip.protocol as u32); XDP_TX
}

#[no_mangle]
pub unsafe extern "C" fn _xdp_tx_iptunnel(xdp: *mut xdp_md) -> i32 {
    let data=(*xdp).data as usize as *mut ethhdr; let data_end=(*xdp).data_end as usize as *mut u8; if data.add(1) as *mut u8 > data_end { return XDP_DROP; }
    match (*data).h_proto { p if p == ETH_P_IP.to_be() => handle_ipv4(xdp), p if p == ETH_P_IPV6.to_be() => handle_ipv6(xdp), _ => XDP_PASS }
}

#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
