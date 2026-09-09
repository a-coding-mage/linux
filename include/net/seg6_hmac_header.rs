/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  SR-IPv6 implementation
 *
 *  Author:
 *  David Lebrun <david.lebrun@uclouvain.be>
 */

// Dependencies supplied by the corresponding kernel headers are intentionally
// referenced here rather than redefined in this translation unit.

pub const SEG6_HMAC_RING_SIZE: usize = 256;

#[repr(C)]
pub union seg6_hmac_info_key {
    pub sha1: hmac_sha1_key,
    pub sha256: hmac_sha256_key,
}

#[repr(C)]
pub struct seg6_hmac_info {
    pub node: rhash_head,
    pub rcu: rcu_head,

    pub hmackeyid: u32,
    /* The raw key, kept only so it can be returned back to userspace */
    pub secret: [core::ffi::c_char; SEG6_HMAC_SECRET_LEN],
    pub slen: u8,
    pub alg_id: u8,
    /* The prepared key, which the calculations actually use */
    pub key: seg6_hmac_info_key,
}

extern "C" {
    pub fn seg6_hmac_compute(
        hinfo: *mut seg6_hmac_info,
        hdr: *mut ipv6_sr_hdr,
        saddr: *mut in6_addr,
        output: *mut u8,
    ) -> i32;
    pub fn seg6_hmac_info_lookup(net: *mut net, key: u32) -> *mut seg6_hmac_info;
    pub fn seg6_hmac_info_add(
        net: *mut net,
        key: u32,
        hinfo: *mut seg6_hmac_info,
    ) -> i32;
    pub fn seg6_hmac_info_del(net: *mut net, key: u32) -> i32;
    pub fn seg6_push_hmac(
        net: *mut net,
        saddr: *mut in6_addr,
        srh: *mut ipv6_sr_hdr,
    ) -> i32;
    pub fn seg6_hmac_validate_skb(skb: *mut sk_buff) -> bool;

    #[cfg(CONFIG_IPV6_SEG6_HMAC)]
    pub fn seg6_hmac_net_init(net: *mut net) -> i32;
    #[cfg(CONFIG_IPV6_SEG6_HMAC)]
    pub fn seg6_hmac_net_exit(net: *mut net);
}

#[cfg(not(CONFIG_IPV6_SEG6_HMAC))]
#[inline]
pub unsafe fn seg6_hmac_net_init(_net: *mut net) -> i32 {
    0
}

#[cfg(not(CONFIG_IPV6_SEG6_HMAC))]
#[inline]
pub unsafe fn seg6_hmac_net_exit(_net: *mut net) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
