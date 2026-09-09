/* Copyright (c) 2016 Facebook
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 */

// KBUILD_MODNAME is "foo".
// Dependencies supplied by the Linux/BPF headers are intentionally external.

const DEFAULT_PKTGEN_UDP_PORT: u16 = 9;
const IP_MF: u16 = 0x2000;
const IP_OFFSET: u16 = 0x1FFF;

unsafe extern "C" {
    fn load_half(ctx: *mut __sk_buff, offset: u64) -> u16;
    fn load_byte(ctx: *mut __sk_buff, offset: u64) -> u8;
}

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

// These types and constants are provided by the Linux headers:
// iphdr, ethhdr, udphdr, ETH_HLEN, ETH_P_IP, IPPROTO_UDP, and TC_ACT_SHOT.

#[inline]
unsafe fn ip_is_fragment(ctx: *mut __sk_buff, nhoff: u64) -> i32 {
    (load_half(
        ctx,
        nhoff + core::mem::offset_of!(iphdr, frag_off) as u64,
    ) & (IP_MF | IP_OFFSET)) as i32
}

#[link_section = "ldabs"]
pub unsafe fn handle_ingress(skb: *mut __sk_buff) -> i32 {
    let troff: u64 = ETH_HLEN as u64 + core::mem::size_of::<iphdr>() as u64;

    if load_half(
        skb,
        core::mem::offset_of!(ethhdr, h_proto) as u64,
    ) != ETH_P_IP {
        return 0;
    }
    if load_byte(
        skb,
        ETH_HLEN as u64 + core::mem::offset_of!(iphdr, protocol) as u64,
    ) != IPPROTO_UDP
        || load_byte(skb, ETH_HLEN as u64) != 0x45
    {
        return 0;
    }
    if ip_is_fragment(skb, ETH_HLEN as u64) != 0 {
        return 0;
    }
    if load_half(
        skb,
        troff + core::mem::offset_of!(udphdr, dest) as u64,
    ) == DEFAULT_PKTGEN_UDP_PORT
    {
        return TC_ACT_SHOT;
    }
    0
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
