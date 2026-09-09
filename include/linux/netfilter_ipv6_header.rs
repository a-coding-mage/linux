/* IPv6-specific defines for netfilter.
 * (C)1998 Rusty Russell -- This code is GPL.
 * (C)1999 David Jeffery
 *   this header was blatantly ripped from netfilter_ipv4.h
 *   it's amazing what adding a bunch of 6s can do =8^)
 */

// Dependencies supplied by other headers are intentionally not implemented here.

/* Check for an extension */
#[inline]
pub fn nf_ip6_ext_hdr(nexthdr: u8) -> i32 {
    (nexthdr == IPPROTO_HOPOPTS
        || nexthdr == IPPROTO_ROUTING
        || nexthdr == IPPROTO_FRAGMENT
        || nexthdr == IPPROTO_ESP
        || nexthdr == IPPROTO_AH
        || nexthdr == IPPROTO_NONE
        || nexthdr == IPPROTO_DSTOPTS) as i32
}

/* Extra routing may needed on local out, as the QUEUE target never returns
 * control to the table.
 */
#[repr(C)]
pub struct ip6_rt_info {
    pub daddr: in6_addr,
    pub saddr: in6_addr,
    pub mark: u32,
}

pub enum nf_queue_entry {}
pub enum nf_bridge_frag_data {}

// CONFIG_NETFILTER declarations and inline definitions.
#[inline]
pub unsafe fn nf_ipv6_chk_addr(
    net: *mut net,
    addr: *const in6_addr,
    dev: *const net_device,
    strict: i32,
) -> i32 {
    // IS_ENABLED(CONFIG_IPV6): call ipv6_chk_addr when enabled; otherwise return 1.
    ipv6_chk_addr(net, addr, dev, strict)
}

extern "C" {
    pub fn __nf_ip6_route(
        net: *mut net,
        dst: *mut *mut dst_entry,
        fl: *mut flowi,
        strict: bool,
    ) -> i32;

    pub fn ipv6_chk_addr(
        net: *mut net,
        addr: *const in6_addr,
        dev: *const net_device,
        strict: i32,
    ) -> i32;

    pub fn br_ip6_fragment(
        net: *mut net,
        sk: *mut sock,
        skb: *mut sk_buff,
        data: *mut nf_bridge_frag_data,
        output: Option<unsafe extern "C" fn(
            *mut net,
            *mut sock,
            *const nf_bridge_frag_data,
            *mut sk_buff,
        ) -> i32>,
    ) -> i32;

    pub fn ip6_route_me_harder(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> i32;

    pub fn __cookie_v6_init_sequence(
        iph: *const ipv6hdr,
        th: *const tcphdr,
        mssp: *mut u16,
    ) -> u32;
    pub fn __cookie_v6_check(iph: *const ipv6hdr, th: *const tcphdr) -> i32;

    pub fn nf_ip6_checksum(
        skb: *mut sk_buff,
        hook: u32,
        dataoff: u32,
        protocol: u8,
    ) -> __sum16;
    pub fn nf_ip6_check_hbh_len(skb: *mut sk_buff, plen: *mut u32) -> i32;
}

#[inline]
pub unsafe fn nf_ip6_route(
    net: *mut net,
    dst: *mut *mut dst_entry,
    fl: *mut flowi,
    strict: bool,
) -> i32 {
    // IS_ENABLED(CONFIG_IPV6): call __nf_ip6_route when enabled; otherwise return -EHOSTUNREACH.
    __nf_ip6_route(net, dst, fl, strict)
}

#[inline]
pub unsafe fn nf_br_ip6_fragment(
    net: *mut net,
    sk: *mut sock,
    skb: *mut sk_buff,
    data: *mut nf_bridge_frag_data,
    output: Option<unsafe extern "C" fn(
        *mut net,
        *mut sock,
        *const nf_bridge_frag_data,
        *mut sk_buff,
    ) -> i32>,
) -> i32 {
    // IS_ENABLED(CONFIG_IPV6): call br_ip6_fragment when enabled; otherwise return 1.
    br_ip6_fragment(net, sk, skb, data, output)
}

#[inline]
pub unsafe fn nf_ip6_route_me_harder(
    net: *mut net,
    sk: *mut sock,
    skb: *mut sk_buff,
) -> i32 {
    // IS_ENABLED(CONFIG_IPV6): call ip6_route_me_harder when enabled; otherwise return -EHOSTUNREACH.
    ip6_route_me_harder(net, sk, skb)
}

#[inline]
pub unsafe fn nf_ipv6_cookie_init_sequence(
    iph: *const ipv6hdr,
    th: *const tcphdr,
    mssp: *mut u16,
) -> u32 {
    // IS_ENABLED(CONFIG_IPV6) && IS_ENABLED(CONFIG_SYN_COOKIES): call helper when enabled; otherwise return 0.
    __cookie_v6_init_sequence(iph, th, mssp)
}

#[inline]
pub unsafe fn nf_cookie_v6_check(iph: *const ipv6hdr, th: *const tcphdr) -> i32 {
    // IS_ENABLED(CONFIG_IPV6) && IS_ENABLED(CONFIG_SYN_COOKIES): call helper when enabled; otherwise return 0.
    __cookie_v6_check(iph, th)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
