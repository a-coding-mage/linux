// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

// Dependencies from the original C includes:
// "vmlinux.h", "bpf_tracing_net.h", <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>

pub const UDP_TEST_PORT: u16 = 7777;

extern "C" {
    #[link_name = "bpf_cast_to_kern_ctx"]
    fn bpf_cast_to_kern_ctx(arg1: *mut core::ffi::c_void) -> *mut core::ffi::c_void;

    fn bpf_skb_load_bytes(
        skb: *mut __sk_buff,
        offset: u32,
        to: *mut core::ffi::c_void,
        len: u32,
    ) -> i32;

    fn bpf_skb_adjust_room(skb: *mut __sk_buff, len_diff: i32, mode: u32, flags: u64) -> i32;
}

#[no_mangle]
pub static mut init_csum_partial: bool = false;
#[no_mangle]
pub static mut final_csum_none: bool = false;
#[no_mangle]
pub static mut broken_csum_start: bool = false;

#[inline]
unsafe fn skb_headlen(skb: *const sk_buff) -> u32 {
    (*skb).len.wrapping_sub((*skb).data_len)
}

#[inline]
unsafe fn skb_headroom(skb: *const sk_buff) -> u32 {
    (*skb).data.wrapping_sub((*skb).head)
}

#[inline]
unsafe fn skb_checksum_start_offset(skb: *const sk_buff) -> i32 {
    (*skb).csum_start.wrapping_sub(skb_headroom(skb)) as i32
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn decap_sanity(skb: *mut __sk_buff) -> i32 {
    let kskb: *mut sk_buff;
    let mut ip6h: ipv6hdr = core::mem::zeroed();
    let mut udph: udphdr = core::mem::zeroed();
    let err: i32;

    if (*skb).protocol != __bpf_constant_htons(ETH_P_IPV6 as u16) {
        return TC_ACT_SHOT;
    }

    if bpf_skb_load_bytes(
        skb,
        ETH_HLEN,
        &mut ip6h as *mut ipv6hdr as *mut core::ffi::c_void,
        core::mem::size_of::<ipv6hdr>() as u32,
    ) != 0
    {
        return TC_ACT_SHOT;
    }

    if ip6h.nexthdr != IPPROTO_UDP {
        return TC_ACT_SHOT;
    }

    if bpf_skb_load_bytes(
        skb,
        ETH_HLEN.wrapping_add(core::mem::size_of::<ipv6hdr>() as u32),
        &mut udph as *mut udphdr as *mut core::ffi::c_void,
        core::mem::size_of::<udphdr>() as u32,
    ) != 0
    {
        return TC_ACT_SHOT;
    }

    if udph.dest != __bpf_constant_htons(UDP_TEST_PORT) {
        return TC_ACT_SHOT;
    }

    kskb = bpf_cast_to_kern_ctx(skb as *mut core::ffi::c_void) as *mut sk_buff;
    init_csum_partial = (*kskb).ip_summed == CHECKSUM_PARTIAL;
    err = bpf_skb_adjust_room(
        skb,
        -((ETH_HLEN
            .wrapping_add(core::mem::size_of::<ipv6hdr>() as u32)
            .wrapping_add(core::mem::size_of::<udphdr>() as u32)) as i32),
        1,
        BPF_F_ADJ_ROOM_FIXED_GSO,
    );
    if err != 0 {
        return TC_ACT_SHOT;
    }
    final_csum_none = (*kskb).ip_summed == CHECKSUM_NONE;
    if (*kskb).ip_summed == CHECKSUM_PARTIAL
        && (skb_checksum_start_offset(kskb) as u32) >= skb_headlen(kskb)
    {
        broken_csum_start = true;
    }

    TC_ACT_SHOT
}

#[no_mangle]
#[link_section = "license"]
pub static __license: [u8; 4] = *b"GPL\0";
