// SPDX-License-Identifier: GPL-2.0-only
/*
 * Network Service Header
 *
 * Copyright (c) 2017 Red Hat, Inc. -- Jiri Benc <jbenc@redhat.com>
 */

// Linux kernel dependencies supplied by the surrounding translated repository.

extern "C" {
    fn nsh_hdr_len(pushed_nh: *const nshhdr) -> usize;
    fn tun_p_from_eth_p(protocol: __be16) -> u8;
    fn tun_p_to_eth_p(proto: u8) -> __be16;
    fn skb_cow_head(skb: *mut sk_buff, length: usize) -> i32;
    fn skb_push(skb: *mut sk_buff, length: usize) -> *mut core::ffi::c_void;
    fn skb_postpush_rcsum(skb: *mut sk_buff, data: *mut nshhdr, length: usize);
    fn skb_reset_mac_header(skb: *mut sk_buff);
    fn skb_reset_network_header(skb: *mut sk_buff);
    fn skb_reset_mac_len(skb: *mut sk_buff);
    fn pskb_may_pull(skb: *mut sk_buff, length: usize) -> bool;
    fn skb_pull_rcsum(skb: *mut sk_buff, length: usize);
    fn skb_mac_header_len(skb: *const sk_buff) -> u32;
    fn nsh_hdr(skb: *mut sk_buff) -> *mut nshhdr;
    fn __skb_pull(skb: *mut sk_buff, length: usize) -> *mut core::ffi::c_void;
    fn skb_mac_gso_segment(skb: *mut sk_buff, features: netdev_features_t) -> *mut sk_buff;
    fn skb_gso_error_unwind(skb: *mut sk_buff, protocol: __be16, nsh_len: u32,
                            mac_offset: u16, mac_len: u16);
    fn __skb_push(skb: *mut sk_buff, length: u32) -> *mut core::ffi::c_void;
    fn skb_set_network_header(skb: *mut sk_buff, offset: u32);
    fn dev_add_offload(offload: *mut packet_offload) -> i32;
    fn dev_remove_offload(offload: *mut packet_offload);
}

type __be16 = u16;
type netdev_features_t = u64;

#[repr(C)]
pub struct sk_buff {
    pub next: *mut sk_buff,
    pub data: *mut u8,
    pub protocol: __be16,
    pub mac_len: u16,
    pub mac_header: u16,
}

#[repr(C)]
pub struct nshhdr {
    pub ver_flags: u8,
    pub length: u8,
    pub mdtype: u8,
    pub np: u8,
}

#[repr(C)]
pub struct packet_offload_callbacks {
    pub gso_segment: Option<unsafe extern "C" fn(*mut sk_buff, netdev_features_t) -> *mut sk_buff>,
}

#[repr(C)]
pub struct packet_offload {
    pub type_: __be16,
    pub priority: i32,
    pub callbacks: packet_offload_callbacks,
}

const TUN_P_ETHERNET: u8 = 1;
const ETH_P_NSH: u16 = 0x894f;
const ETH_P_TEB: u16 = 0x6558;
const ETH_HLEN: u16 = 14;
const NSH_BASE_HDR_LEN: usize = 8;
const NETIF_F_SG: netdev_features_t = 1 <<  scatter_bit();

const fn scatter_bit() -> u32 { 5 }

const EAFNOSUPPORT: i32 = 97;
const ENOMEM: i32 = 12;
const EINVAL: i32 = 22;

#[inline]
unsafe fn htons(value: u16) -> __be16 { value.to_be() }

pub unsafe extern "C" fn nsh_push(skb: *mut sk_buff, pushed_nh: *const nshhdr) -> i32 {
    let mut nh: *mut nshhdr;
    let length = nsh_hdr_len(pushed_nh);
    let next_proto: u8;

    if (*skb).mac_len != 0 {
        next_proto = TUN_P_ETHERNET;
    } else {
        next_proto = tun_p_from_eth_p((*skb).protocol);
        if next_proto == 0 { return -EAFNOSUPPORT; }
    }

    if skb_cow_head(skb, length) < 0 { return -ENOMEM; }
    skb_push(skb, length);
    nh = (*skb).data as *mut nshhdr;
    core::ptr::copy_nonoverlapping(pushed_nh as *const u8, nh as *mut u8, length);
    (*nh).np = next_proto;
    skb_postpush_rcsum(skb, nh, length);
    (*skb).protocol = htons(ETH_P_NSH);
    skb_reset_mac_header(skb);
    skb_reset_network_header(skb);
    skb_reset_mac_len(skb);
    0
}

pub unsafe extern "C" fn nsh_pop(skb: *mut sk_buff) -> i32 {
    if !pskb_may_pull(skb, NSH_BASE_HDR_LEN) { return -ENOMEM; }
    let nh = (*skb).data as *mut nshhdr;
    let length = nsh_hdr_len(nh);
    if length < NSH_BASE_HDR_LEN { return -EINVAL; }
    let inner_proto = tun_p_to_eth_p((*nh).np);
    if !pskb_may_pull(skb, length) { return -ENOMEM; }
    if inner_proto == 0 { return -EAFNOSUPPORT; }
    skb_pull_rcsum(skb, length);
    skb_reset_mac_header(skb);
    skb_reset_network_header(skb);
    skb_reset_mac_len(skb);
    (*skb).protocol = inner_proto;
    0
}

unsafe extern "C" fn nsh_gso_segment(mut skb: *mut sk_buff, mut features: netdev_features_t) -> *mut sk_buff {
    let outer_hlen = skb_mac_header_len(skb);
    let mac_len = (*skb).mac_len;
    let outer_proto = (*skb).protocol;
    let mac_offset = (*skb).mac_header;
    let mut segs: *mut sk_buff = (-EINVAL as isize) as *mut sk_buff;

    skb_reset_network_header(skb);
    if !pskb_may_pull(skb, NSH_BASE_HDR_LEN) { return segs; }
    let nsh_len = nsh_hdr_len(nsh_hdr(skb)) as u32;
    if nsh_len < NSH_BASE_HDR_LEN as u32 || !pskb_may_pull(skb, nsh_len as usize) { return segs; }
    let proto = tun_p_to_eth_p((*nsh_hdr(skb)).np);
    if proto == 0 { return segs; }
    __skb_pull(skb, nsh_len as usize);
    skb_reset_mac_header(skb);
    (*skb).mac_len = if proto == htons(ETH_P_TEB) { ETH_HLEN } else { 0 };
    (*skb).protocol = proto;
    features &= NETIF_F_SG;
    segs = skb_mac_gso_segment(skb, features);
    if segs.is_null() || (segs as isize) < 0 {
        skb_gso_error_unwind(skb, htons(ETH_P_NSH), nsh_len, mac_offset, mac_len);
        return segs;
    }
    let mut segment = segs;
    while !segment.is_null() {
        (*segment).protocol = outer_proto;
        __skb_push(segment, nsh_len + outer_hlen);
        skb_reset_mac_header(segment);
        skb_set_network_header(segment, outer_hlen);
        (*segment).mac_len = mac_len;
        segment = (*segment).next;
    }
    segs
}

static mut nsh_packet_offload: packet_offload = packet_offload {
    type_: htons_const(ETH_P_NSH),
    priority: 15,
    callbacks: packet_offload_callbacks { gso_segment: Some(nsh_gso_segment) },
};

const fn htons_const(value: u16) -> u16 { value.to_be() }

unsafe extern "C" fn nsh_init_module() -> i32 {
    dev_add_offload(&mut nsh_packet_offload);
    0
}

unsafe extern "C" fn nsh_cleanup_module() {
    dev_remove_offload(&mut nsh_packet_offload);
}

// module_init(nsh_init_module);
// module_exit(nsh_cleanup_module);
// MODULE_AUTHOR("Jiri Benc <jbenc@redhat.com>");
// MODULE_DESCRIPTION("NSH protocol");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
