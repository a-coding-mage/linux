// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include "vmlinux.h"
// #include "bpf_tracing_net.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_endian.h>

const UDP_TEST_PORT: u16 = 7777;

extern "C" {
    #[link_name = "bpf_cast_to_kern_ctx"]
    fn bpf_cast_to_kern_ctx(arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_skb_load_bytes(
        skb: *mut __sk_buff,
        offset: u32,
        to: *mut core::ffi::c_void,
        len: u32,
    ) -> i32;
    fn bpf_skb_adjust_room(
        skb: *mut __sk_buff,
        len_diff: s32,
        mode: u32,
        flags: u64,
    ) -> i32;
    fn __bpf_constant_htons(x: u16) -> u16;
}

type s32 = i32;

extern "C" {
    static ETH_P_IP: u16;
    static ETH_HLEN: u32;
    static IPPROTO_UDP: u8;
    static TC_ACT_OK: i32;
    static TC_ACT_SHOT: i32;
    static BPF_ADJ_ROOM_MAC: u32;
    static BPF_F_ADJ_ROOM_FIXED_GSO: u64;
    static BPF_F_ADJ_ROOM_ENCAP_L3_IPV4: u64;
}

#[repr(C)]
pub struct __sk_buff {
    pub protocol: u16,
}

#[repr(C)]
pub struct sk_buff {
    pub _skb_refdst: usize,
}

#[repr(C)]
pub struct iphdr {
    pub protocol: u8,
}

#[repr(C)]
pub struct udphdr {
    pub dest: u16,
}

pub static mut had_dst: bool = false;
pub static mut dst_cleared: bool = false;

// SEC("tc/egress")
#[no_mangle]
pub unsafe extern "C" fn dst_clear(skb: *mut __sk_buff) -> i32 {
    let mut kskb: *mut sk_buff;
    let mut iph: iphdr = core::mem::zeroed();
    let mut udph: udphdr = core::mem::zeroed();
    let err: i32;

    if (*skb).protocol != __bpf_constant_htons(ETH_P_IP) {
        return TC_ACT_OK;
    }

    if bpf_skb_load_bytes(
        skb,
        ETH_HLEN,
        &mut iph as *mut iphdr as *mut core::ffi::c_void,
        core::mem::size_of::<iphdr>() as u32,
    ) != 0
    {
        return TC_ACT_OK;
    }

    if iph.protocol != IPPROTO_UDP {
        return TC_ACT_OK;
    }

    if bpf_skb_load_bytes(
        skb,
        ETH_HLEN + core::mem::size_of::<iphdr>() as u32,
        &mut udph as *mut udphdr as *mut core::ffi::c_void,
        core::mem::size_of::<udphdr>() as u32,
    ) != 0
    {
        return TC_ACT_OK;
    }

    if udph.dest != __bpf_constant_htons(UDP_TEST_PORT) {
        return TC_ACT_OK;
    }

    kskb = bpf_cast_to_kern_ctx(skb as *mut core::ffi::c_void) as *mut sk_buff;
    had_dst = (*kskb)._skb_refdst != 0;

    /* Same-protocol encap (IPIP): protocol stays IPv4, but the dst
     * from the original routing is no longer valid for the outer hdr.
     */
    err = bpf_skb_adjust_room(
        skb,
        core::mem::size_of::<iphdr>() as s32,
        BPF_ADJ_ROOM_MAC,
        BPF_F_ADJ_ROOM_FIXED_GSO | BPF_F_ADJ_ROOM_ENCAP_L3_IPV4,
    );
    if err != 0 {
        return TC_ACT_SHOT;
    }

    dst_cleared = (*kskb)._skb_refdst == 0;

    return TC_ACT_SHOT;
}

// char __license[] SEC("license") = "GPL";
#[no_mangle]
pub static __license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
