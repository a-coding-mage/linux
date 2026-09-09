/* SPDX-License-Identifier: GPL-2.0 */
/*
 *              INETPEER - A storage for permanent information about peers
 *
 * Authors: Andrey V. Savochkin <saw@msu.ru>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than implemented in this header translation.

/* IPv4 address key for cache lookups */
#[repr(C)]
pub struct ipv4_addr_key {
    pub addr: __be32,
    pub vif: ::core::ffi::c_int,
}

pub const INETPEER_MAXKEYSZ: usize = core::mem::size_of::<in6_addr>() / core::mem::size_of::<u32>();

#[repr(C)]
pub union inetpeer_addr_union {
    pub a4: ipv4_addr_key,
    pub a6: in6_addr,
    pub key: [u32; INETPEER_MAXKEYSZ],
}

#[repr(C)]
pub struct inetpeer_addr {
    pub u: inetpeer_addr_union,
    pub family: __u16,
}

#[repr(C)]
pub union inet_peer_rid_rcu {
    pub rid: atomic_t,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct inet_peer {
    pub rb_node: rb_node,
    pub hash: u64,
    pub daddr: inetpeer_addr,
    pub metrics: [u32; RTAX_MAX as usize],
    pub rate_tokens: u32, /* rate limiting for ICMP */
    pub n_redirects: u32,
    pub rate_last: ::core::ffi::c_ulong,
    /*
     * Once inet_peer is queued for deletion (refcnt == 0), following field
     * is not available: rid
     * We can share memory with rcu_head to help keep inet_peer small.
     */
    pub rid_rcu: inet_peer_rid_rcu,
    /* following fields might be frequently dirtied */
    pub dtime: __u32, /* the time of last use of not referenced entries */
    pub refcnt: refcount_t,
}

#[repr(C)]
pub struct inet_peer_base {
    pub rb_root: rb_root,
    pub lock: seqlock_t,
    pub total: ::core::ffi::c_int,
}

extern "C" {
    pub fn inet_peer_base_init(base: *mut inet_peer_base);
    pub fn inet_initpeers(); // __init
}

pub const INETPEER_METRICS_NEW: u32 = !0u32;

#[inline]
pub unsafe fn inetpeer_set_addr_v4(iaddr: *mut inetpeer_addr, ip: __be32) {
    (*iaddr).u.a4.addr = ip;
    (*iaddr).u.a4.vif = 0;
    (*iaddr).family = AF_INET as __u16;
}

#[inline]
pub unsafe fn inetpeer_get_addr_v4(iaddr: *mut inetpeer_addr) -> __be32 {
    (*iaddr).u.a4.addr
}

#[inline]
pub unsafe fn inetpeer_set_addr_v6(iaddr: *mut inetpeer_addr, in6: *mut in6_addr) {
    (*iaddr).u.a6 = *in6;
    (*iaddr).family = AF_INET6 as __u16;
}

#[inline]
pub unsafe fn inetpeer_get_addr_v6(iaddr: *mut inetpeer_addr) -> *mut in6_addr {
    &mut (*iaddr).u.a6
}

/* can be called with or without local BH being disabled */
extern "C" {
    pub fn inet_getpeer(base: *mut inet_peer_base, daddr: *const inetpeer_addr) -> *mut inet_peer;
}

#[inline]
pub unsafe fn inet_getpeer_v4(base: *mut inet_peer_base, v4daddr: __be32, vif: ::core::ffi::c_int) -> *mut inet_peer {
    let mut daddr = inetpeer_addr { u: inetpeer_addr_union { a4: ipv4_addr_key { addr: v4daddr, vif } }, family: AF_INET as __u16 };
    inet_getpeer(base, &daddr)
}

#[inline]
pub unsafe fn inet_getpeer_v6(base: *mut inet_peer_base, v6daddr: *const in6_addr) -> *mut inet_peer {
    let mut daddr = inetpeer_addr { u: inetpeer_addr_union { a6: *v6daddr }, family: AF_INET6 as __u16 };
    inet_getpeer(base, &daddr)
}

#[inline]
pub unsafe fn inetpeer_addr_cmp(a: *const inetpeer_addr, b: *const inetpeer_addr) -> ::core::ffi::c_int {
    if (*a).family != (*b).family {
        return if (*a).family < (*b).family { -1 } else { 1 };
    }
    let n = if (*a).family == AF_INET as __u16 {
        core::mem::size_of::<ipv4_addr_key>() / core::mem::size_of::<u32>()
    } else {
        core::mem::size_of::<in6_addr>() / core::mem::size_of::<u32>()
    };
    for i in 0..n {
        let ak = (*a).u.key[i];
        let bk = (*b).u.key[i];
        if ak == bk { continue; }
        return if ak < bk { -1 } else { 1 };
    }
    0
}

/* can be called from BH context or outside */
extern "C" {
    pub fn inet_putpeer(p: *mut inet_peer);
    pub fn inet_peer_xrlim_allow(peer: *mut inet_peer, timeout: ::core::ffi::c_int) -> bool;
    pub fn inetpeer_invalidate_tree(base: *mut inet_peer_base);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
