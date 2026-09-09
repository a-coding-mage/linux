// SPDX-License-Identifier: GPL-2.0-only

// Kernel dependencies supplied by other translation units.

unsafe fn ndisc_check_ip6hdr(skb: *mut sk_buff) -> c_int {
    let ip6h: *const ipv6hdr;
    let offset: c_uint;
    let len: c_uint;

    offset = skb_network_offset(skb) + core::mem::size_of::<ipv6hdr>() as c_uint;
    if !pskb_may_pull(skb, offset) {
        return -EINVAL;
    }

    ip6h = ipv6_hdr(skb);

    if (*ip6h).version != 6 {
        return -EINVAL;
    }

    if (*ip6h).nexthdr != IPPROTO_ICMPV6 {
        return -ENOMSG;
    }

    /* RFC 4861 7.1.1 / 7.1.2: must not have been forwarded by a router */
    if (*ip6h).hop_limit != 255 {
        return -EINVAL;
    }

    len = offset + ntohs((*ip6h).payload_len) as c_uint;
    if (*skb).len < len || len <= offset {
        return -EINVAL;
    }

    skb_set_transport_header(skb, offset);

    0
}

unsafe fn ndisc_validate_checksum(skb: *mut sk_buff) -> __sum16 {
    skb_checksum_validate(skb, IPPROTO_ICMPV6, ip6_compute_pseudo)
}

unsafe fn ndisc_check_icmpv6(skb: *mut sk_buff) -> c_int {
    let len: c_uint = skb_transport_offset(skb)
        + core::mem::size_of::<icmp6hdr>() as c_uint;
    let transport_len: c_uint = ipv6_transport_len(skb);
    let skb_chk: *mut sk_buff;
    let hdr: *mut icmp6hdr;

    if !pskb_may_pull(skb, len) {
        return -EINVAL;
    }

    /* RFC 4861 7.1.1 / 7.1.2: the ICMPv6 checksum must be valid */
    skb_chk = skb_checksum_trimmed(skb, transport_len, ndisc_validate_checksum);
    if skb_chk.is_null() {
        return -EINVAL;
    }

    if skb_chk != skb {
        kfree_skb(skb_chk);
    }

    /* RFC 4861 7.1.1 / 7.1.2: Code must be 0 */
    hdr = skb_transport_header(skb) as *mut icmp6hdr;
    if (*hdr).icmp6_code != 0 {
        return -EINVAL;
    }

    0
}

unsafe fn ndisc_check_options(
    skb: *mut sk_buff,
    mut opts_len: c_uint,
    reject_slla: bool,
) -> c_int {
    let mut offset: c_uint = skb_transport_offset(skb)
        + core::mem::size_of::<nd_msg>() as c_uint;
    let mut opt: *mut nd_opt_hdr;
    let mut _opt: nd_opt_hdr = core::mem::zeroed();

    while opts_len > 0 {
        if opts_len < core::mem::size_of::<nd_opt_hdr>() as c_uint {
            return -EINVAL;
        }

        opt = skb_header_pointer(
            skb,
            offset,
            core::mem::size_of::<nd_opt_hdr>() as c_uint,
            &mut _opt as *mut nd_opt_hdr as *mut c_void,
        ) as *mut nd_opt_hdr;
        if opt.is_null() {
            return -EINVAL;
        }

        /* RFC 4861 7.1.1 / 7.1.2: all option lengths must be > 0 */
        if (*opt).nd_opt_len == 0 {
            return -EINVAL;
        }

        /* RFC 4861 7.1.1: DAD NS must not contain a source link-layer
         * address option
         */
        if reject_slla && (*opt).nd_opt_type == ND_OPT_SOURCE_LL_ADDR {
            return -EINVAL;
        }

        if ((*opt).nd_opt_len as c_uint) * 8 > opts_len {
            return -EINVAL;
        }

        offset += (*opt).nd_opt_len as c_uint * 8;
        opts_len -= (*opt).nd_opt_len as c_uint * 8;
    }

    0
}

unsafe fn ndisc_check_nd_msg(skb: *mut sk_buff) -> c_int {
    let len: c_uint = skb_transport_offset(skb)
        + core::mem::size_of::<nd_msg>() as c_uint;
    let transport_len: c_uint = ipv6_transport_len(skb);
    let mut reject_slla: bool = false;
    let msg: *const nd_msg;

    if !pskb_may_pull(skb, len) {
        return -EINVAL;
    }

    /* RFC 4861 7.1.1 / 7.1.2: ICMP length is at least sizeof(nd_msg) */
    if transport_len < core::mem::size_of::<nd_msg>() as c_uint {
        return -EINVAL;
    }

    msg = skb_transport_header(skb) as *const nd_msg;

    /* RFC 4861 7.1.1 / 7.1.2: Target Address must not be a
     * multicast address
     */
    if ipv6_addr_is_multicast(&(*msg).target) {
        return -EINVAL;
    }

    match (*msg).icmph.icmp6_type {
        NDISC_NEIGHBOUR_SOLICITATION => {
            if ipv6_addr_any(&(*ipv6_hdr(skb)).saddr) {
                /* RFC 4861 7.1.1: DAD NS destination must be a
                 * solicited-node multicast address
                 */
                if !ipv6_addr_is_solict_mult(&(*ipv6_hdr(skb)).daddr) {
                    return -EINVAL;
                }
                /* RFC 4861 7.1.1: DAD NS must not contain a source
                 * link-layer address option
                 */
                reject_slla = true;
            }
        }
        NDISC_NEIGHBOUR_ADVERTISEMENT => {
            /* RFC 4861 7.1.2: Solicited flag must be 0 for
             * multicast destinations
             */
            if ipv6_addr_is_multicast(&(*ipv6_hdr(skb)).daddr)
                && (*msg).icmph.icmp6_solicited != 0
            {
                return -EINVAL;
            }
        }
        _ => return -ENODATA,
    }

    ndisc_check_options(skb, transport_len - core::mem::size_of::<nd_msg>() as c_uint, reject_slla)
}

/**
 * ndisc_check_ns_na - validate an NS/NA packet and set its transport header
 * @skb: the skb to validate
 *
 * Validates an IPv6 packet for compliance with RFC 4861 sections 7.1.1
 * (Neighbor Solicitation) and 7.1.2 (Neighbor Advertisement). If valid,
 * sets the skb transport header.
 *
 * Caller needs to set the skb network header.
 *
 * Return:
 * * 0        - valid NS/NA; the skb transport header has been set.
 * * -EINVAL  - a broken packet was detected, i.e. it violates some
 *              internet standard.
 * * -ENOMSG  - IP header validation succeeded but it is not an ICMPv6
 *              packet.
 * * -ENODATA - IP+ICMPv6 header validation succeeded but it is not a
 *              Neighbor Solicitation or Neighbor Advertisement.
 */
pub unsafe fn ndisc_check_ns_na(skb: *mut sk_buff) -> c_int {
    let mut ret: c_int;

    ret = ndisc_check_ip6hdr(skb);
    if ret < 0 {
        return ret;
    }

    ret = ndisc_check_icmpv6(skb);
    if ret < 0 {
        return ret;
    }

    ndisc_check_nd_msg(skb)
}

// EXPORT_SYMBOL_GPL(ndisc_check_ns_na);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
