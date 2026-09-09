/* Copyright (c) 2016 Facebook
 *
 * This program is free software: you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 */

// KBUILD_MODNAME = "foo"
// Linux and BPF helper declarations are supplied by the surrounding build.

pub const DEFAULT_PKTGEN_UDP_PORT: u16 = 9;

/* copy of 'struct ethhdr' without __packed */
#[repr(C)]
pub struct eth_hdr {
    pub h_dest: [u8; ETH_ALEN],
    pub h_source: [u8; ETH_ALEN],
    pub h_proto: u16,
}

// SEC("simple")
pub unsafe fn handle_ingress(skb: *mut __sk_buff) -> i32 {
    let data = skb_data(skb) as *mut u8;
    let eth = data as *mut eth_hdr;
    let iph = data.add(core::mem::size_of::<eth_hdr>()) as *mut iphdr;
    let udp = data.add(core::mem::size_of::<eth_hdr>() + core::mem::size_of::<iphdr>())
        as *mut udphdr;
    let data_end = skb_data_end(skb) as *mut u8;

    /* single length check */
    if data.add(
        core::mem::size_of::<eth_hdr>()
            + core::mem::size_of::<iphdr>()
            + core::mem::size_of::<udphdr>(),
    ) > data_end
    {
        return 0;
    }

    if (*eth).h_proto != htons(ETH_P_IP) {
        return 0;
    }
    if (*iph).protocol != IPPROTO_UDP || (*iph).ihl != 5 {
        return 0;
    }
    if ip_is_fragment(iph) {
        return 0;
    }
    if (*udp).dest == htons(DEFAULT_PKTGEN_UDP_PORT) {
        return TC_ACT_SHOT;
    }
    0
}

// SEC("license")
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// External declarations supplied by included Linux/BPF headers and helpers.
extern "C" {
    static ETH_ALEN: usize;
    static ETH_P_IP: u16;
    static IPPROTO_UDP: u8;
    static TC_ACT_SHOT: i32;

    fn htons(value: u16) -> u16;
    fn ip_is_fragment(iph: *const iphdr) -> bool;
    fn skb_data(skb: *const __sk_buff) -> *mut core::ffi::c_void;
    fn skb_data_end(skb: *const __sk_buff) -> *mut core::ffi::c_void;
}

// Types supplied by included Linux headers.
pub struct __sk_buff;
pub struct iphdr {
    pub ihl: u8,
    pub protocol: u8,
}
pub struct udphdr {
    pub dest: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
