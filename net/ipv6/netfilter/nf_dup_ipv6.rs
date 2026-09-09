// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (C) 2007 by Sebastian Claßen <sebastian.classen@freenet.ag>
 * (C) 2007-2010 by Jan Engelhardt <jengelh@medozas.de>
 *
 * Extracted from xt_TEE.c
 */
// Dependencies supplied by the surrounding kernel translation.

unsafe fn nf_dup_ipv6_route(
    net: *mut net,
    skb: *mut sk_buff,
    gw: *const in6_addr,
    oif: i32,
) -> bool {
    let iph: *const ipv6hdr = ipv6_hdr(skb);
    let mut dst: *mut dst_entry;
    let mut fl6: flowi6 = core::mem::zeroed();

    if oif != -1 {
        fl6.flowi6_oif = oif;
    }

    fl6.daddr = *gw;
    fl6.flowlabel = (((((*iph).flow_lbl[0] & 0xF) << 16)
        | ((*iph).flow_lbl[1] << 8)
        | (*iph).flow_lbl[2]) as u32);
    fl6.flowi6_flags = FLOWI_FLAG_KNOWN_NH;
    dst = ip6_route_output(net, core::ptr::null_mut(), &mut fl6);
    if (*dst).error != 0 {
        dst_release(dst);
        return false;
    }
    skb_dst_drop(skb);
    skb_dst_set(skb, dst);
    (*skb).dev = dst_dev(dst);
    (*skb).protocol = htons(ETH_P_IPV6);

    true
}

pub unsafe fn nf_dup_ipv6(
    net: *mut net,
    skb: *mut sk_buff,
    hooknum: u32,
    gw: *const in6_addr,
    oif: i32,
) {
    local_bh_disable();
    if (*current).in_nf_duplicate {
        local_bh_enable();
        return;
    }
    let skb = pskb_copy(skb, GFP_ATOMIC);
    if skb.is_null() {
        local_bh_enable();
        return;
    }

    // Preserved from IS_ENABLED(CONFIG_NF_CONNTRACK); enable when conntrack is configured.
    #[cfg(feature = "CONFIG_NF_CONNTRACK")]
    {
        nf_reset_ct(skb);
        nf_ct_set(skb, core::ptr::null_mut(), IP_CT_UNTRACKED);
    }
    if hooknum == NF_INET_PRE_ROUTING || hooknum == NF_INET_LOCAL_IN {
        let iph: *mut ipv6hdr = ipv6_hdr(skb);
        (*iph).hop_limit = (*iph).hop_limit.wrapping_sub(1);
    }
    if nf_dup_ipv6_route(net, skb, gw, oif) {
        (*current).in_nf_duplicate = true;
        ip6_local_out(net, (*skb).sk, skb);
        (*current).in_nf_duplicate = false;
    } else {
        kfree_skb(skb);
    }
    local_bh_enable();
}

// EXPORT_SYMBOL_GPL(nf_dup_ipv6);
// MODULE_AUTHOR("Sebastian Claßen <sebastian.classen@freenet.ag>");
// MODULE_AUTHOR("Jan Engelhardt <jengelh@medozas.de>");
// MODULE_DESCRIPTION("nf_dup_ipv6: IPv6 packet duplication");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
