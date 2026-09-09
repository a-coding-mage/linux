// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * IPv4 specific functions of netfilter core
 *
 * Rusty Russell (C) 2000
 * Patrick McHardy (C) 2006-2012
 */

// Dependencies are supplied by the surrounding kernel translation.

/* route_me_harder function, used by iptable_nat, iptable_mangle + ip_queue */
pub unsafe fn ip_route_me_harder(
    mut net: *mut net,
    mut sk: *mut sock,
    skb: *mut sk_buff,
    mut addr_type: c_uint,
) -> c_int {
    let dev: *mut net_device = skb_dst_dev(skb);
    let iph: *const iphdr = ip_hdr(skb);
    let mut rt: *mut rtable;
    let mut fl4: flowi4 = core::mem::zeroed();
    let saddr: __be32 = (*iph).saddr;
    let mut flags: __u8;
    let mut flkeys: flow_keys = core::mem::zeroed();
    let hh_len: c_uint;

    sk = sk_to_full_sk(sk);
    flags = if !sk.is_null() { inet_sk_flowi_flags(sk) } else { 0 };

    if addr_type == RTN_UNSPEC {
        addr_type = inet_addr_type_dev_table(net, dev, saddr);
    }
    if addr_type == RTN_LOCAL || addr_type == RTN_UNICAST {
        flags |= FLOWI_FLAG_ANYSRC;
    } else {
        // C assigns zero to saddr here; preserve the original value through a
        // mutable local because Rust bindings may define __be32 distinctly.
    }

    /* some non-standard hacks like ipt_REJECT.c:send_reset() can cause
     * packets with foreign saddr to appear on the NF_INET_LOCAL_OUT hook.
     */
    fl4.daddr = (*iph).daddr;
    fl4.saddr = if addr_type == RTN_LOCAL || addr_type == RTN_UNICAST {
        saddr
    } else {
        0
    };
    fl4.flowi4_dscp = ip4h_dscp(iph);
    fl4.flowi4_oif = if !sk.is_null() { (*sk).sk_bound_dev_if } else { 0 };
    fl4.flowi4_l3mdev = l3mdev_master_ifindex(dev);
    fl4.flowi4_mark = (*skb).mark;
    fl4.flowi4_flags = flags;
    fib4_rules_early_flow_dissect(net, skb, &mut fl4, &mut flkeys);
    rt = ip_route_output_key(net, &mut fl4);
    if IS_ERR(rt) {
        return PTR_ERR(rt);
    }

    /* Drop old route. */
    skb_dst_drop(skb);
    skb_dst_set(skb, &mut (*rt).dst);

    if (*skb_dst(skb)).error != 0 {
        return (*skb_dst(skb)).error;
    }

    // CONFIG_XFRM conditional code is retained below; enable it in builds
    // corresponding to the original kernel configuration.
    #[cfg(CONFIG_XFRM)]
    {
        if ((*IPCB(skb)).flags & IPSKB_XFRM_TRANSFORMED) == 0
            && xfrm_decode_session(net, skb, flowi4_to_flowi(&mut fl4), AF_INET) == 0
        {
            let mut dst: *mut dst_entry = skb_dst(skb);
            /* ignore return value from skb_dstref_steal, xfrm_lookup takes
             * care of dropping the refcnt if needed.
             */
            skb_dstref_steal(skb);
            dst = xfrm_lookup(net, dst, flowi4_to_flowi(&mut fl4), sk, 0);
            if IS_ERR(dst) {
                return PTR_ERR(dst);
            }
            skb_dst_set(skb, dst);
        }
    }

    /* Change in oif may mean change in hh_len. */
    hh_len = (*skb_dst_dev(skb)).hard_header_len;
    if skb_headroom(skb) < hh_len
        && pskb_expand_head(skb, HH_DATA_ALIGN(hh_len - skb_headroom(skb)), 0, GFP_ATOMIC) != 0
    {
        return -ENOMEM;
    }

    0
}

// EXPORT_SYMBOL(ip_route_me_harder);

pub unsafe fn nf_ip_route(
    net: *mut net,
    dst: *mut *mut dst_entry,
    fl: *mut flowi,
    _strict: bool,
) -> c_int {
    let rt: *mut rtable = ip_route_output_key(net, &mut (*fl).u.ip4);
    if IS_ERR(rt) {
        return PTR_ERR(rt);
    }
    *dst = &mut (*rt).dst;
    0
}

// EXPORT_SYMBOL_GPL(nf_ip_route);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
