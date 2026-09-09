/* Copyright (c) 2016 Facebook
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 */

// Dependencies supplied by the Linux kernel and BPF build environment.

const DEFAULT_PKTGEN_UDP_PORT: u16 = 9;
const DEBUG: i32 = 0;
const TC_ACT_SHOT: i32 = 2;
const ETH_P_8021Q: u16 = 0x8100;
const ETH_P_8021AD: u16 = 0x88a8;
const ETH_P_IP: u16 = 0x0800;
const ETH_P_IPV6: u16 = 0x86dd;
const IPPROTO_IPIP: u8 = 4;
const IPPROTO_IPV6: u8 = 41;
const IPPROTO_TCP: u8 = 6;
const IPPROTO_UDP: u8 = 17;

#[repr(C)]
struct __sk_buff {
    _prefix: [u8; 76],
    data: u32,
    data_end: u32,
}

#[repr(C)]
struct ethhdr {
    h_dest: [u8; 6],
    h_source: [u8; 6],
    h_proto: u16,
}

#[repr(C)]
struct vlan_hdr {
    h_vlan_TCI: u16,
    h_vlan_encapsulated_proto: u16,
}

#[repr(C)]
struct tcphdr {
    source: u16,
    dest: u16,
    _rest: [u8; 16],
}

#[repr(C)]
struct udphdr {
    source: u16,
    dest: u16,
    _rest: [u8; 4],
}

#[repr(C)]
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
    #[inline]
    unsafe fn ihl(&self) -> u8 { self.ihl_version & 0x0f }
}

#[repr(C)]
struct ipv6hdr {
    _rest: [u8; 6],
    nexthdr: u8,
    _tail: [u8; 33],
}

extern "C" {
    fn htons(value: u16) -> u16;
    fn bpf_trace_printk(fmt: *const u8, fmt_size: u32, ...) -> i64;
}

#[inline]
unsafe fn ip_is_fragment(iph: *const iphdr) -> bool {
    (*iph).frag_off & 0x1fff != 0
}

unsafe fn tcp(data: *mut u8, tp_off: u64, data_end: *mut u8) -> i32 {
    let tcp = data.add(tp_off as usize) as *mut tcphdr;
    if (tcp.add(1) as *mut u8) > data_end { return 0; }
    if (*tcp).dest == htons(80) || (*tcp).source == htons(80) { return TC_ACT_SHOT; }
    0
}

unsafe fn udp(data: *mut u8, tp_off: u64, data_end: *mut u8) -> i32 {
    let udp = data.add(tp_off as usize) as *mut udphdr;
    if (udp.add(1) as *mut u8) > data_end { return 0; }
    if (*udp).dest == htons(DEFAULT_PKTGEN_UDP_PORT) || (*udp).source == htons(DEFAULT_PKTGEN_UDP_PORT) {
        if DEBUG != 0 {
            let fmt = b"udp port 9 indeed\n\0";
            bpf_trace_printk(fmt.as_ptr(), core::mem::size_of_val(fmt) as u32);
        }
        return TC_ACT_SHOT;
    }
    0
}

unsafe fn parse_ipv4(data: *mut u8, nh_off: u64, data_end: *mut u8) -> i32 {
    let mut iph = data.add(nh_off as usize) as *mut iphdr;
    if (iph.add(1) as *mut u8) > data_end { return 0; }
    if ip_is_fragment(iph) { return 0; }
    let mut ihl_len = ((*iph).ihl() as u64) * 4;
    if (*iph).protocol == IPPROTO_IPIP {
        iph = data.add((nh_off + ihl_len) as usize) as *mut iphdr;
        if (iph.add(1) as *mut u8) > data_end { return 0; }
        ihl_len += ((*iph).ihl() as u64) * 4;
    }
    if (*iph).protocol == IPPROTO_TCP { tcp(data, nh_off + ihl_len, data_end) }
    else if (*iph).protocol == IPPROTO_UDP { udp(data, nh_off + ihl_len, data_end) }
    else { 0 }
}

unsafe fn parse_ipv6(data: *mut u8, nh_off: u64, data_end: *mut u8) -> i32 {
    let mut ip6h = data.add(nh_off as usize) as *mut ipv6hdr;
    let mut ihl_len = core::mem::size_of::<ipv6hdr>() as u64;
    let mut nexthdr: u8;
    if (ip6h.add(1) as *mut u8) > data_end { return 0; }
    nexthdr = (*ip6h).nexthdr;
    if nexthdr == IPPROTO_IPIP {
        let iph = data.add((nh_off + ihl_len) as usize) as *mut iphdr;
        if (iph.add(1) as *mut u8) > data_end { return 0; }
        ihl_len += ((*iph).ihl() as u64) * 4;
        nexthdr = (*iph).protocol;
    } else if nexthdr == IPPROTO_IPV6 {
        ip6h = data.add((nh_off + ihl_len) as usize) as *mut ipv6hdr;
        if (ip6h.add(1) as *mut u8) > data_end { return 0; }
        ihl_len += core::mem::size_of::<ipv6hdr>() as u64;
        nexthdr = (*ip6h).nexthdr;
    }
    if nexthdr == IPPROTO_TCP { tcp(data, nh_off + ihl_len, data_end) }
    else if nexthdr == IPPROTO_UDP { udp(data, nh_off + ihl_len, data_end) }
    else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn handle_ingress(skb: *mut __sk_buff) -> i32 {
    let data = (*skb).data as *mut u8;
    let data_end = (*skb).data_end as *mut u8;
    let eth = data as *mut ethhdr;
    let mut nh_off = core::mem::size_of::<ethhdr>() as u64;
    if data.add(nh_off as usize) > data_end { return 0; }
    let mut h_proto = (*eth).h_proto;
    if h_proto == ETH_P_8021Q || h_proto == ETH_P_8021AD {
        let vhdr = data.add(nh_off as usize) as *mut vlan_hdr;
        nh_off += core::mem::size_of::<vlan_hdr>() as u64;
        if data.add(nh_off as usize) > data_end { return 0; }
        h_proto = (*vhdr).h_vlan_encapsulated_proto;
    }
    if h_proto == ETH_P_8021Q || h_proto == ETH_P_8021AD {
        let vhdr = data.add(nh_off as usize) as *mut vlan_hdr;
        nh_off += core::mem::size_of::<vlan_hdr>() as u64;
        if data.add(nh_off as usize) > data_end { return 0; }
        h_proto = (*vhdr).h_vlan_encapsulated_proto;
    }
    if h_proto == htons(ETH_P_IP) { parse_ipv4(data, nh_off, data_end) }
    else if h_proto == htons(ETH_P_IPV6) { parse_ipv6(data, nh_off, data_end) }
    else { 0 }
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
