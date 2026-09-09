// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (C) 2007 by Sebastian Claßen <sebastian.classen@freenet.ag>
 * (C) 2007-2010 by Jan Engelhardt <jengelh@medozas.de>
 *
 * Extracted from xt_TEE.c
 */
// C dependencies: linux/ip.h, linux/module.h, linux/percpu.h,
// linux/route.h, linux/skbuff.h, linux/netfilter.h, net/checksum.h,
// net/flow.h, net/icmp.h, net/ip.h, net/route.h,
// net/netfilter/ipv4/nf_dup_ipv4.h.
// CONFIG_NF_CONNTRACK conditionally supplies net/netfilter/nf_conntrack.h.

unsafe fn nf_dup_ipv4_route(
    net: *mut net,
    skb: *mut sk_buff,
    gw: *const in_addr,
    oif: i32,
) -> bool {
    let iph: *const iphdr = ip_hdr(skb);
    let mut rt: *mut rtable;
    let mut fl4: flowi4;

    memset(
        &mut fl4 as *mut flowi4 as *mut core::ffi::c_void,
        0,
        core::mem::size_of::<flowi4>(),
    );
    if oif != -1 {
        fl4.flowi4_oif = oif;
    }

    fl4.daddr = (*gw).s_addr;
    fl4.flowi4_dscp = ip4h_dscp(iph);
    fl4.flowi4_scope = RT_SCOPE_UNIVERSE;
    fl4.flowi4_flags = FLOWI_FLAG_KNOWN_NH;
    rt = ip_route_output_key(net, &mut fl4);
    if IS_ERR(rt) {
        return false;
    }

    skb_dst_drop(skb);
    skb_dst_set(skb, &mut (*rt).dst);
    (*skb).dev = (*rt).dst.dev;
    (*skb).protocol = htons(ETH_P_IP);

    true
}

pub unsafe fn nf_dup_ipv4(
    net: *mut net,
    mut skb: *mut sk_buff,
    hooknum: u32,
    gw: *const in_addr,
    oif: i32,
) {
    let mut iph: *mut iphdr;

    local_bh_disable();
    if (*current).in_nf_duplicate {
        local_bh_enable();
        return;
    }
    /*
     * Copy the skb, and route the copy. Will later return %XT_CONTINUE for
     * the original skb, which should continue on its way as if nothing has
     * happened. The copy should be independently delivered to the gateway.
     */
    skb = pskb_copy(skb, GFP_ATOMIC);
    if skb.is_null() {
        local_bh_enable();
        return;
    }

    // When CONFIG_NF_CONNTRACK is enabled:
    /* Avoid counting cloned packets towards the original connection. */
    nf_reset_ct(skb);
    nf_ct_set(skb, core::ptr::null_mut(), IP_CT_UNTRACKED);
    // #endif
    /*
     * If we are in PREROUTING/INPUT, decrease the TTL to mitigate potential
     * loops between two hosts.
     *
     * Set %IP_DF so that the original source is notified of a potentially
     * decreased MTU on the clone route. IPv6 does this too.
     *
     * IP header checksum will be recalculated at ip_local_out.
     */
    iph = ip_hdr(skb);
    (*iph).frag_off |= htons(IP_DF);
    if hooknum == NF_INET_PRE_ROUTING || hooknum == NF_INET_LOCAL_IN {
        (*iph).ttl = (*iph).ttl.wrapping_sub(1);
    }

    if nf_dup_ipv4_route(net, skb, gw, oif) {
        (*current).in_nf_duplicate = true;
        ip_local_out(net, (*skb).sk, skb);
        (*current).in_nf_duplicate = false;
    } else {
        kfree_skb(skb);
    }

    local_bh_enable();
}

// EXPORT_SYMBOL_GPL(nf_dup_ipv4);
// MODULE_AUTHOR("Sebastian Claßen <sebastian.classen@freenet.ag>");
// MODULE_AUTHOR("Jan Engelhardt <jengelh@medozas.de>");
// MODULE_DESCRIPTION("nf_dup_ipv4: Duplicate IPv4 packet");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
