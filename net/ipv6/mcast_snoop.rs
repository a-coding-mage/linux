// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2010: YOSHIFUJI Hideaki <yoshfuji@linux-ipv6.org>
 * Copyright (C) 2015: Linus Lüssing <linus.luessing@c0d3.blue>
 *
 * Based on the MLD support added to br_multicast.c by YOSHIFUJI Hideaki.
 */

// Linux kernel dependencies supplied by other translation units.

unsafe fn ipv6_mc_check_ip6hdr(skb: *mut sk_buff) -> c_int {
    let mut ip6h: *const ipv6hdr;
    let mut len: c_uint;
    let mut offset: c_uint = skb_network_offset(skb) + core::mem::size_of::<ipv6hdr>() as c_uint;

    if !pskb_may_pull(skb, offset) {
        return -EINVAL;
    }

    ip6h = ipv6_hdr(skb);

    if (*ip6h).version != 6 {
        return -EINVAL;
    }

    len = offset + u16::from_be((*ip6h).payload_len) as c_uint;
    if (*skb).len < len || len <= offset {
        return -EINVAL;
    }

    skb_set_transport_header(skb, offset);

    0
}

unsafe fn ipv6_mc_check_exthdrs(skb: *mut sk_buff) -> c_int {
    let ip6h: *const ipv6hdr = ipv6_hdr(skb);
    let mut offset: c_int;
    let mut nexthdr: u8;
    let mut frag_off: __be16 = 0;

    if (*ip6h).nexthdr != IPPROTO_HOPOPTS {
        return -ENOMSG;
    }

    nexthdr = (*ip6h).nexthdr;
    offset = (skb_network_offset(skb) + core::mem::size_of::<ipv6hdr>() as c_uint) as c_int;
    offset = ipv6_skip_exthdr(skb, offset, &mut nexthdr, &mut frag_off);

    if offset < 0 {
        return -EINVAL;
    }

    if nexthdr != IPPROTO_ICMPV6 {
        return -ENOMSG;
    }

    skb_set_transport_header(skb, offset as c_uint);

    0
}

unsafe fn ipv6_mc_check_mld_reportv2(skb: *mut sk_buff) -> c_int {
    let mut len: c_uint = skb_transport_offset(skb);
    len += core::mem::size_of::<mld2_report>() as c_uint;
    if ipv6_mc_may_pull(skb, len) { 0 } else { -EINVAL }
}

unsafe fn ipv6_mc_check_mld_query(skb: *mut sk_buff) -> c_int {
    let transport_len: c_uint = ipv6_transport_len(skb);
    let mld: *mut mld_msg;
    let mut len: c_uint;

    // RFC2710+RFC3810 (MLDv1+MLDv2) require link-local source addresses
    if (ipv6_addr_type(&(*ipv6_hdr(skb)).saddr) & IPV6_ADDR_LINKLOCAL) == 0 {
        return -EINVAL;
    }

    // MLDv1?
    if transport_len != core::mem::size_of::<mld_msg>() as c_uint {
        // or MLDv2?
        if transport_len < core::mem::size_of::<mld2_query>() as c_uint {
            return -EINVAL;
        }

        len = skb_transport_offset(skb) + core::mem::size_of::<mld2_query>() as c_uint;
        if !ipv6_mc_may_pull(skb, len) {
            return -EINVAL;
        }
    }

    mld = skb_transport_header(skb) as *mut mld_msg;

    // RFC2710+RFC3810 (MLDv1+MLDv2) require the multicast link layer
    // all-nodes destination address (ff02::1) for general queries
    if ipv6_addr_any(&(*mld).mld_mca) &&
        !ipv6_addr_is_ll_all_nodes(&(*ipv6_hdr(skb)).daddr) {
        return -EINVAL;
    }

    0
}

unsafe fn ipv6_mc_check_mld_msg(skb: *mut sk_buff) -> c_int {
    let len = skb_transport_offset(skb) + core::mem::size_of::<mld_msg>() as c_uint;
    let mld: *mut mld_msg;

    if !ipv6_mc_may_pull(skb, len) {
        return -ENODATA;
    }

    mld = skb_transport_header(skb) as *mut mld_msg;

    match (*mld).mld_type {
        ICMPV6_MGM_REDUCTION | ICMPV6_MGM_REPORT => 0,
        ICMPV6_MLD2_REPORT => ipv6_mc_check_mld_reportv2(skb),
        ICMPV6_MGM_QUERY => ipv6_mc_check_mld_query(skb),
        _ => -ENODATA,
    }
}

#[inline]
unsafe fn ipv6_mc_validate_checksum(skb: *mut sk_buff) -> __sum16 {
    skb_checksum_validate(skb, IPPROTO_ICMPV6, ip6_compute_pseudo)
}

unsafe fn ipv6_mc_check_icmpv6(skb: *mut sk_buff) -> c_int {
    let len = skb_transport_offset(skb) + core::mem::size_of::<icmp6hdr>() as c_uint;
    let transport_len = ipv6_transport_len(skb);
    let skb_chk: *mut sk_buff;

    if !ipv6_mc_may_pull(skb, len) {
        return -EINVAL;
    }

    skb_chk = skb_checksum_trimmed(skb, transport_len, ipv6_mc_validate_checksum);
    if skb_chk.is_null() {
        return -EINVAL;
    }

    if skb_chk != skb {
        kfree_skb(skb_chk);
    }

    0
}

/**
 * ipv6_mc_check_mld - checks whether this is a sane MLD packet
 * @skb: the skb to validate
 *
 * Checks whether an IPv6 packet is a valid MLD packet. If so sets
 * skb transport header accordingly and returns zero.
 *
 * -EINVAL: A broken packet was detected, i.e. it violates some internet
 *  standard
 * -ENOMSG: IP header validation succeeded but it is not an ICMPv6 packet
 *  with a hop-by-hop option.
 * -ENODATA: IP+ICMPv6 header with hop-by-hop option validation succeeded
 *  but it is not an MLD packet.
 * -ENOMEM: A memory allocation failure happened.
 *
 * Caller needs to set the skb network header and free any returned skb if it
 * differs from the provided skb.
 */
#[no_mangle]
pub unsafe extern "C" fn ipv6_mc_check_mld(skb: *mut sk_buff) -> c_int {
    let mut ret: c_int;

    ret = ipv6_mc_check_ip6hdr(skb);
    if ret < 0 {
        return ret;
    }

    ret = ipv6_mc_check_exthdrs(skb);
    if ret < 0 {
        return ret;
    }

    ret = ipv6_mc_check_icmpv6(skb);
    if ret < 0 {
        return ret;
    }

    ipv6_mc_check_mld_msg(skb)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
