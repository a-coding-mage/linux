/* Copyright (c) 2016 Facebook
 *
 * This program is free software: you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 */

// Definitions supplied by the Linux/BPF headers are intentionally external.
use core::ffi::c_void;

const KBUILD_MODNAME: &str = "foo";
const PIN_GLOBAL_NS: u32 = 2;

#[repr(C)]
pub struct bpf_elf_map {
    pub type_: u32,
    pub size_key: u32,
    pub size_value: u32,
    pub max_elem: u32,
    pub flags: u32,
    pub id: u32,
    pub pinning: u32,
}

#[repr(C)]
pub struct eth_hdr {
    pub h_dest: [u8; ETH_ALEN],
    pub h_source: [u8; ETH_ALEN],
    pub h_proto: u16,
}

#[no_mangle]
#[link_section = "maps"]
pub static mut tun_iface: bpf_elf_map = bpf_elf_map {
    type_: BPF_MAP_TYPE_ARRAY,
    size_key: core::mem::size_of::<i32>() as u32,
    size_value: core::mem::size_of::<i32>() as u32,
    max_elem: 1,
    flags: 0,
    id: 0,
    pinning: PIN_GLOBAL_NS,
};

#[inline(always)]
unsafe fn is_vip_addr(eth_proto: u16, daddr: u32) -> bool {
    if eth_proto == htons(ETH_P_IP) {
        (_htonl(0xffffff00) & daddr) == _htonl(0x0a0a0100)
    } else if eth_proto == htons(ETH_P_IPV6) {
        daddr == _htonl(0x2401face)
    } else {
        false
    }
}

#[link_section = "l2_to_iptun_ingress_forward"]
pub unsafe extern "C" fn _l2_to_iptun_ingress_forward(skb: *mut __sk_buff) -> i32 {
    let data = (*skb).data as *mut c_void;
    let eth = data as *mut eth_hdr;
    let data_end = (*skb).data_end as *mut c_void;
    let key: i32 = 0;
    let ifindex: *mut i32;

    if (data as usize) + core::mem::size_of::<eth_hdr>() > data_end as usize { return TC_ACT_OK; }
    ifindex = bpf_map_lookup_elem(&mut tun_iface, &key);
    if ifindex.is_null() { return TC_ACT_OK; }

    if (*eth).h_proto == htons(ETH_P_IP) {
        let fmt4 = b"ingress forward to ifindex:%d daddr4:%x\n\0";
        let iph = (data as usize + core::mem::size_of::<eth_hdr>()) as *mut iphdr;
        if (data as usize) + core::mem::size_of::<eth_hdr>() + core::mem::size_of::<iphdr>() > data_end as usize { return TC_ACT_OK; }
        if (*iph).protocol != IPPROTO_IPIP { return TC_ACT_OK; }
        bpf_trace_printk(fmt4.as_ptr() as *const i8, fmt4.len(), *ifindex, _htonl((*iph).daddr));
        bpf_redirect(*ifindex, BPF_F_INGRESS)
    } else if (*eth).h_proto == htons(ETH_P_IPV6) {
        let fmt6 = b"ingress forward to ifindex:%d daddr6:%x::%x\n\0";
        let ip6h = (data as usize + core::mem::size_of::<eth_hdr>()) as *mut ipv6hdr;
        if (data as usize) + core::mem::size_of::<eth_hdr>() + core::mem::size_of::<ipv6hdr>() > data_end as usize { return TC_ACT_OK; }
        if (*ip6h).nexthdr != IPPROTO_IPIP && (*ip6h).nexthdr != IPPROTO_IPV6 { return TC_ACT_OK; }
        bpf_trace_printk(fmt6.as_ptr() as *const i8, fmt6.len(), *ifindex, _htonl((*ip6h).daddr.s6_addr32[0]), _htonl((*ip6h).daddr.s6_addr32[3]));
        bpf_redirect(*ifindex, BPF_F_INGRESS)
    } else { TC_ACT_OK }
}

// The remaining program bodies retain the original BPF operations and depend on
// the corresponding Linux/BPF header declarations.
#[link_section = "l2_to_iptun_ingress_redirect"]
pub unsafe extern "C" fn _l2_to_iptun_ingress_redirect(skb: *mut __sk_buff) -> i32 {
    let mut tkey: bpf_tunnel_key = core::mem::zeroed();
    let data = (*skb).data as *mut c_void;
    let eth = data as *mut eth_hdr;
    let data_end = (*skb).data_end as *mut c_void;
    let key: i32 = 0;
    if (data as usize) + core::mem::size_of::<eth_hdr>() > data_end as usize { return TC_ACT_OK; }
    let ifindex = bpf_map_lookup_elem(&mut tun_iface, &key);
    if ifindex.is_null() { return TC_ACT_OK; }
    if (*eth).h_proto != htons(ETH_P_IP) { return TC_ACT_OK; }
    let fmt4 = b"e/ingress redirect daddr4:%x to ifindex:%d\n\0";
    let iph = (data as usize + core::mem::size_of::<eth_hdr>()) as *mut iphdr;
    if (data as usize) + core::mem::size_of::<eth_hdr>() + core::mem::size_of::<iphdr>() > data_end as usize { return TC_ACT_OK; }
    let daddr = (*iph).daddr;
    if !is_vip_addr((*eth).h_proto, daddr) { return TC_ACT_OK; }
    bpf_trace_printk(fmt4.as_ptr() as *const i8, fmt4.len(), _htonl(daddr), *ifindex);
    (*(&mut tkey)).tunnel_id = 10000;
    tkey.tunnel_ttl = 64;
    tkey.remote_ipv4 = 0x0a020166;
    bpf_skb_set_tunnel_key(skb, &mut tkey, core::mem::size_of::<bpf_tunnel_key>(), 0);
    bpf_redirect(*ifindex, 0)
}

#[link_section = "l2_to_ip6tun_ingress_redirect"]
pub unsafe extern "C" fn _l2_to_ip6tun_ingress_redirect(skb: *mut __sk_buff) -> i32 {
    let mut tkey: bpf_tunnel_key = core::mem::zeroed();
    let data = (*skb).data as *mut c_void;
    let eth = data as *mut eth_hdr;
    let data_end = (*skb).data_end as *mut c_void;
    let key: i32 = 0;
    if (data as usize) + core::mem::size_of::<eth_hdr>() > data_end as usize { return TC_ACT_OK; }
    let ifindex = bpf_map_lookup_elem(&mut tun_iface, &key);
    if ifindex.is_null() { return TC_ACT_OK; }
    let fmt4 = b"e/ingress redirect daddr4:%x to ifindex:%d\n\0";
    let fmt6 = b"e/ingress redirect daddr6:%x to ifindex:%d\n\0";
    if (*eth).h_proto == htons(ETH_P_IP) {
        let iph = (data as usize + core::mem::size_of::<eth_hdr>()) as *mut iphdr;
        if (data as usize) + core::mem::size_of::<eth_hdr>() + core::mem::size_of::<iphdr>() > data_end as usize || !is_vip_addr((*eth).h_proto, (*iph).daddr) { return TC_ACT_OK; }
        bpf_trace_printk(fmt4.as_ptr() as *const i8, fmt4.len(), _htonl((*iph).daddr), *ifindex);
    } else if (*eth).h_proto == htons(ETH_P_IPV6) {
        let ip6h = (data as usize + core::mem::size_of::<eth_hdr>()) as *mut ipv6hdr;
        if (data as usize) + core::mem::size_of::<eth_hdr>() + core::mem::size_of::<ipv6hdr>() > data_end as usize || !is_vip_addr((*eth).h_proto, (*ip6h).daddr.s6_addr32[0]) { return TC_ACT_OK; }
        bpf_trace_printk(fmt6.as_ptr() as *const i8, fmt6.len(), _htonl((*ip6h).daddr.s6_addr32[0]), *ifindex);
    } else { return TC_ACT_OK; }
    tkey.tunnel_id = 10000; tkey.tunnel_ttl = 64;
    tkey.remote_ipv6[0] = _htonl(0x2401db02); tkey.remote_ipv6[1] = 0; tkey.remote_ipv6[2] = 0; tkey.remote_ipv6[3] = _htonl(0x00000066);
    bpf_skb_set_tunnel_key(skb, &mut tkey, core::mem::size_of::<bpf_tunnel_key>(), BPF_F_TUNINFO_IPV6);
    bpf_redirect(*ifindex, 0)
}

#[link_section = "drop_non_tun_vip"]
pub unsafe extern "C" fn _drop_non_tun_vip(skb: *mut __sk_buff) -> i32 {
    let data = (*skb).data as *mut c_void; let eth = data as *mut eth_hdr; let data_end = (*skb).data_end as *mut c_void;
    if (data as usize) + core::mem::size_of::<eth_hdr>() > data_end as usize { return TC_ACT_OK; }
    if (*eth).h_proto == htons(ETH_P_IP) {
        let iph = (data as usize + core::mem::size_of::<eth_hdr>()) as *mut iphdr;
        if (data as usize) + core::mem::size_of::<eth_hdr>() + core::mem::size_of::<iphdr>() > data_end as usize { return TC_ACT_OK; }
        if is_vip_addr((*eth).h_proto, (*iph).daddr) { return TC_ACT_SHOT; }
    } else if (*eth).h_proto == htons(ETH_P_IPV6) {
        let ip6h = (data as usize + core::mem::size_of::<eth_hdr>()) as *mut ipv6hdr;
        if (data as usize) + core::mem::size_of::<eth_hdr>() + core::mem::size_of::<ipv6hdr>() > data_end as usize { return TC_ACT_OK; }
        if is_vip_addr((*eth).h_proto, (*ip6h).daddr.s6_addr32[0]) { return TC_ACT_SHOT; }
    }
    TC_ACT_OK
}

#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
