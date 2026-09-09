/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * INET		An implementation of the TCP/IP protocol suite for the LINUX
 * operating system. INET is implemented using the BSD Socket interface.
 *
 * Rust translation of inet6_hashtables.h.
 */

/* The declarations below are available when CONFIG_IPV6 is enabled. */

use core::ffi::{c_int, c_ushort};

/* Dependencies supplied by the surrounding kernel translation. */
extern "C" {
    pub fn inet6_init_ehash_secret();

    pub fn __inet6_lookup_established(
        net: *const net,
        saddr: *const in6_addr,
        sport: __be16,
        daddr: *const in6_addr,
        hnum: u16,
        dif: c_int,
        sdif: c_int,
    ) -> *mut sock;

    pub fn inet6_ehashfn(
        net: *const net,
        laddr: *const in6_addr,
        lport: u16,
        faddr: *const in6_addr,
        fport: __be16,
    ) -> u32;

    pub fn udp6_ehashfn(
        net: *const net,
        laddr: *const in6_addr,
        lport: u16,
        faddr: *const in6_addr,
        fport: __be16,
    ) -> u32;

    pub fn inet6_lookup_reuseport(
        net: *const net,
        sk: *mut sock,
        skb: *mut sk_buff,
        doff: c_int,
        saddr: *const in6_addr,
        sport: __be16,
        daddr: *const in6_addr,
        hnum: c_ushort,
        ehashfn: Option<inet6_ehashfn_t>,
    ) -> *mut sock;

    pub fn inet6_lookup_listener(
        net: *const net,
        skb: *mut sk_buff,
        doff: c_int,
        saddr: *const in6_addr,
        sport: __be16,
        daddr: *const in6_addr,
        hnum: c_ushort,
        dif: c_int,
        sdif: c_int,
    ) -> *mut sock;

    pub fn inet6_lookup_run_sk_lookup(
        net: *const net,
        protocol: c_int,
        skb: *mut sk_buff,
        doff: c_int,
        saddr: *const in6_addr,
        sport: __be16,
        daddr: *const in6_addr,
        hnum: u16,
        dif: c_int,
        ehashfn: Option<inet6_ehashfn_t>,
    ) -> *mut sock;

    pub fn inet6_lookup(
        net: *const net,
        skb: *mut sk_buff,
        doff: c_int,
        saddr: *const in6_addr,
        sport: __be16,
        daddr: *const in6_addr,
        dport: __be16,
        dif: c_int,
    ) -> *mut sock;
}

pub type __be16 = u16;
pub type __portpair = u32;

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}
#[repr(C)]
pub struct in6_addr {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ipv6hdr {
    pub saddr: in6_addr,
    pub daddr: in6_addr,
}
#[repr(C)]
pub struct sock {
    pub sk_protocol: u8,
    pub sk_state: u8,
    pub sk_family: u16,
    pub sk_portpair: __portpair,
    pub sk_v6_daddr: in6_addr,
    pub sk_v6_rcv_saddr: in6_addr,
    pub sk_bound_dev_if: c_int,
}

pub type inet6_ehashfn_t = unsafe extern "C" fn(
    net: *const net,
    laddr: *const in6_addr,
    lport: u16,
    faddr: *const in6_addr,
    fport: __be16,
) -> u32;

pub unsafe fn __inet6_lookup(
    net: *const net,
    skb: *mut sk_buff,
    doff: c_int,
    saddr: *const in6_addr,
    sport: __be16,
    daddr: *const in6_addr,
    hnum: u16,
    dif: c_int,
    sdif: c_int,
    refcounted: *mut bool,
) -> *mut sock {
    let sk = __inet6_lookup_established(net, saddr, sport, daddr, hnum, dif, sdif);
    *refcounted = true;
    if !sk.is_null() {
        return sk;
    }
    *refcounted = false;
    inet6_lookup_listener(net, skb, doff, saddr, sport, daddr, hnum as c_ushort, dif, sdif)
}

pub unsafe fn inet6_steal_sock(
    net: *mut net,
    skb: *mut sk_buff,
    doff: c_int,
    saddr: *const in6_addr,
    sport: __be16,
    daddr: *const in6_addr,
    dport: __be16,
    refcounted: *mut bool,
    ehashfn: Option<inet6_ehashfn_t>,
) -> *mut sock {
    /* skb_steal_sock, sk_fullsock, WARN_ON_ONCE, and related symbols are external dependencies. */
    let mut prefetched = false;
    let sk = skb_steal_sock(skb, refcounted, &mut prefetched);
    if sk.is_null() {
        return core::ptr::null_mut();
    }
    if !prefetched || !sk_fullsock(sk) {
        return sk;
    }
    if (*sk).sk_protocol == IPPROTO_TCP {
        if (*sk).sk_state != TCP_LISTEN {
            return sk;
        }
    } else if (*sk).sk_protocol == IPPROTO_UDP {
        if (*sk).sk_state != TCP_CLOSE {
            return sk;
        }
    } else {
        return sk;
    }
    let reuse_sk = inet6_lookup_reuseport(net, sk, skb, doff, saddr, sport, daddr,
                                          ntohs(dport), ehashfn);
    if reuse_sk.is_null() {
        return sk;
    }
    /* The selected reuseport socket is never refcounted. */
    WARN_ON_ONCE(*refcounted);
    reuse_sk
}

pub unsafe fn __inet6_lookup_skb(
    skb: *mut sk_buff,
    doff: c_int,
    sport: __be16,
    dport: __be16,
    iif: c_int,
    sdif: c_int,
    refcounted: *mut bool,
) -> *mut sock {
    let net = skb_dst_dev_net_rcu(skb);
    let ip6h = ipv6_hdr(skb);
    let sk = inet6_steal_sock(net, skb, doff, &(*ip6h).saddr, sport, &(*ip6h).daddr,
                              dport, refcounted, Some(inet6_ehashfn));
    if is_err(sk) {
        return core::ptr::null_mut();
    }
    if !sk.is_null() {
        return sk;
    }
    __inet6_lookup(net, skb, doff, &(*ip6h).saddr, sport, &(*ip6h).daddr,
                   ntohs(dport), iif, sdif, refcounted)
}

pub unsafe fn inet6_match(
    net: *const net,
    sk: *const sock,
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    ports: __portpair,
    dif: c_int,
    sdif: c_int,
) -> bool {
    if !net_eq(sock_net(sk), net)
        || (*sk).sk_family != AF_INET6
        || read_once(&(*sk).sk_portpair) != ports
        || !ipv6_addr_equal(&(*sk).sk_v6_daddr, saddr)
        || !ipv6_addr_equal(&(*sk).sk_v6_rcv_saddr, daddr)
    {
        return false;
    }
    inet_sk_bound_dev_eq(net, read_once(&(*sk).sk_bound_dev_if), dif, sdif)
}

/* External symbols and constants referenced above are provided by other headers. */
extern "C" {
    fn skb_steal_sock(skb: *mut sk_buff, refcounted: *mut bool, prefetched: *mut bool) -> *mut sock;
    fn sk_fullsock(sk: *mut sock) -> bool;
    fn skb_dst_dev_net_rcu(skb: *mut sk_buff) -> *mut net;
    fn ipv6_hdr(skb: *mut sk_buff) -> *mut ipv6hdr;
    fn is_err(sk: *mut sock) -> bool;
    fn ntohs(value: __be16) -> u16;
    fn net_eq(a: *const net, b: *const net) -> bool;
    fn sock_net(sk: *const sock) -> *const net;
    fn ipv6_addr_equal(a: *const in6_addr, b: *const in6_addr) -> bool;
    fn inet_sk_bound_dev_eq(net: *const net, bound_dev_if: c_int, dif: c_int, sdif: c_int) -> bool;
    fn WARN_ON_ONCE(condition: bool) -> bool;
}

unsafe fn read_once<T: Copy>(ptr: *const T) -> T {
    core::ptr::read_volatile(ptr)
}

pub const IPPROTO_TCP: u8 = 6;
pub const IPPROTO_UDP: u8 = 17;
pub const TCP_LISTEN: u8 = 10;
pub const TCP_CLOSE: u8 = 7;
pub const AF_INET6: u16 = 10;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
