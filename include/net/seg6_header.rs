/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  SR-IPv6 implementation
 *
 *  Author:
 *  David Lebrun <david.lebrun@uclouvain.be>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than redefined.

#[inline]
pub unsafe fn update_csum_diff4(skb: *mut sk_buff, from: __be32, to: __be32) {
    let diff: [__be32; 2] = [!from, to];

    (*skb).csum = !csum_partial(
        diff.as_ptr() as *const i8,
        core::mem::size_of_val(&diff),
        !(*skb).csum,
    );
}

#[inline]
pub unsafe fn update_csum_diff16(
    skb: *mut sk_buff,
    from: *mut __be32,
    to: *mut __be32,
) {
    let diff: [__be32; 8] = [
        !*from.add(0),
        !*from.add(1),
        !*from.add(2),
        !*from.add(3),
        *to.add(0),
        *to.add(1),
        *to.add(2),
        *to.add(3),
    ];

    (*skb).csum = !csum_partial(
        diff.as_ptr() as *const i8,
        core::mem::size_of_val(&diff),
        !(*skb).csum,
    );
}

#[repr(C)]
pub struct seg6_pernet_data {
    pub lock: mutex,
    pub tun_src: *mut in6_addr,
    // CONFIG_IPV6_SEG6_HMAC: struct rhashtable hmac_infos;
    #[cfg(CONFIG_IPV6_SEG6_HMAC)]
    pub hmac_infos: rhashtable,
}

#[inline]
pub unsafe fn seg6_pernet(net: *mut net) -> *mut seg6_pernet_data {
    // CONFIG_IPV6 is a build-time condition preserved from the C header.
    #[cfg(CONFIG_IPV6)]
    {
        return (*net).ipv6.seg6_data;
    }
    #[cfg(not(CONFIG_IPV6))]
    {
        let _ = net;
        core::ptr::null_mut()
    }
}

extern "C" {
    pub fn seg6_init() -> i32;
    pub fn seg6_exit();

    // CONFIG_IPV6_SEG6_LWTUNNEL is a build-time condition preserved below.
    #[cfg(CONFIG_IPV6_SEG6_LWTUNNEL)]
    pub fn seg6_iptunnel_init() -> i32;
    #[cfg(CONFIG_IPV6_SEG6_LWTUNNEL)]
    pub fn seg6_iptunnel_exit();
    #[cfg(CONFIG_IPV6_SEG6_LWTUNNEL)]
    pub fn seg6_local_init() -> i32;
    #[cfg(CONFIG_IPV6_SEG6_LWTUNNEL)]
    pub fn seg6_local_exit();
}

#[cfg(not(CONFIG_IPV6_SEG6_LWTUNNEL))]
#[inline]
pub fn seg6_iptunnel_init() -> i32 { 0 }
#[cfg(not(CONFIG_IPV6_SEG6_LWTUNNEL))]
#[inline]
pub fn seg6_iptunnel_exit() {}
#[cfg(not(CONFIG_IPV6_SEG6_LWTUNNEL))]
#[inline]
pub fn seg6_local_init() -> i32 { 0 }
#[cfg(not(CONFIG_IPV6_SEG6_LWTUNNEL))]
#[inline]
pub fn seg6_local_exit() {}

extern "C" {
    pub fn seg6_validate_srh(srh: *mut ipv6_sr_hdr, len: i32, reduced: bool) -> bool;
    pub fn seg6_get_srh(skb: *mut sk_buff, flags: i32) -> *mut ipv6_sr_hdr;
    pub fn seg6_icmp_srh(skb: *mut sk_buff, opt: *mut inet6_skb_parm);
    pub fn seg6_do_srh_encap(skb: *mut sk_buff, osrh: *mut ipv6_sr_hdr, proto: i32) -> i32;
    pub fn seg6_do_srh_inline(skb: *mut sk_buff, osrh: *mut ipv6_sr_hdr) -> i32;
    pub fn seg6_lookup_nexthop(skb: *mut sk_buff, nhaddr: *mut in6_addr, tbl_id: u32) -> i32;
}

/* If the packet which invoked an ICMP error contains an SRH return
 * the true destination address from within the SRH, otherwise use the
 * destination address in the IP header.
 */
#[inline]
pub unsafe fn seg6_get_daddr(
    skb: *mut sk_buff,
    opt: *mut inet6_skb_parm,
) -> *const in6_addr {
    if (*opt).flags & IP6SKB_SEG6 != 0 {
        let srh = ((*skb).data.add((*opt).srhoff as usize)) as *mut ipv6_sr_hdr;
        return &(*srh).segments[0] as *const in6_addr;
    }

    core::ptr::null()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
