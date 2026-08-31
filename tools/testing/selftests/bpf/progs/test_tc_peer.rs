// SPDX-License-Identifier: GPL-2.0

// C includes translated as external dependencies:
// <stdint.h>, <stdbool.h>, <linux/bpf.h>, <linux/stddef.h>,
// <linux/pkt_cls.h>, <linux/if_ether.h>, <linux/ip.h>, <bpf/bpf_helpers.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;

const TC_ACT_SHOT: i32 = 2;
const BPF_F_EGRESS: u64 = 1 << 1;
const ETH_ALEN: u32 = 6;
const ETH_HLEN: u32 = 14;

#[repr(C)]
pub struct __sk_buff {
    pub len: __u32,
    pub pkt_type: __u32,
    pub mark: __u32,
    pub queue_mapping: __u32,
    pub protocol: __u32,
}

extern "C" {
    fn bpf_redirect_peer(ifindex: __u32, flags: u64) -> i32;
    fn bpf_redirect(ifindex: __u32, flags: u64) -> i32;
    fn bpf_skb_change_head(skb: *mut __sk_buff, len: __u32, flags: u64) -> i32;
    fn bpf_skb_store_bytes(
        skb: *mut __sk_buff,
        offset: __u32,
        from: *const core::ffi::c_void,
        len: __u32,
        flags: u64,
    ) -> i32;
}

// Original C declarations are volatile const __u32 globals.
#[no_mangle]
pub static IFINDEX_SRC: __u32 = 0;
#[no_mangle]
pub static IFINDEX_DST: __u32 = 0;

static src_mac: [__u8; 6] = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
static dst_mac: [__u8; 6] = [0x00, 0x22, 0x33, 0x44, 0x55, 0x66];

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn tc_chk(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    TC_ACT_SHOT
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn tc_dst(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    bpf_redirect_peer(core::ptr::read_volatile(&IFINDEX_SRC), 0)
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn tc_src(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    bpf_redirect_peer(core::ptr::read_volatile(&IFINDEX_DST), 0)
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn tc_dst_ing(skb: *mut __sk_buff) -> i32 {
    if (*skb).mark == 0 {
        (*skb).mark = 0x1;
        return bpf_redirect_peer(core::ptr::read_volatile(&IFINDEX_SRC), BPF_F_EGRESS);
    }

    bpf_redirect(core::ptr::read_volatile(&IFINDEX_DST), 0)
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn tc_src_ing(skb: *mut __sk_buff) -> i32 {
    if (*skb).mark == 0 {
        (*skb).mark = 0x1;
        return bpf_redirect_peer(core::ptr::read_volatile(&IFINDEX_DST), BPF_F_EGRESS);
    }

    bpf_redirect(core::ptr::read_volatile(&IFINDEX_SRC), 0)
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn tc_dst_l3(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    bpf_redirect(core::ptr::read_volatile(&IFINDEX_SRC), 0)
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn tc_src_l3(skb: *mut __sk_buff) -> i32 {
    let proto: __u16 = (*skb).protocol as __u16;

    if bpf_skb_change_head(skb, ETH_HLEN, 0) != 0 {
        return TC_ACT_SHOT;
    }

    if bpf_skb_store_bytes(
        skb,
        0,
        (&src_mac as *const [__u8; 6]).cast::<core::ffi::c_void>(),
        ETH_ALEN,
        0,
    ) != 0
    {
        return TC_ACT_SHOT;
    }

    if bpf_skb_store_bytes(
        skb,
        ETH_ALEN,
        (&dst_mac as *const [__u8; 6]).cast::<core::ffi::c_void>(),
        ETH_ALEN,
        0,
    ) != 0
    {
        return TC_ACT_SHOT;
    }

    if bpf_skb_store_bytes(
        skb,
        ETH_ALEN + ETH_ALEN,
        (&proto as *const __u16).cast::<core::ffi::c_void>(),
        core::mem::size_of::<__u16>() as __u32,
        0,
    ) != 0
    {
        return TC_ACT_SHOT;
    }

    bpf_redirect_peer(core::ptr::read_volatile(&IFINDEX_DST), 0)
}

#[no_mangle]
#[link_section = "license"]
pub static __license: [u8; 4] = *b"GPL\0";
