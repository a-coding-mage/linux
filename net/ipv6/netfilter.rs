// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * IPv6 specific functions of netfilter core
 *
 * Rusty Russell (C) 2000
 * Patrick McHardy (C) 2006-2012
 */
// Kernel dependencies supplied by the surrounding translation unit.

pub unsafe fn ip6_route_me_harder(
    net: *mut net,
    sk_partial: *mut sock,
    skb: *mut sk_buff,
) -> i32 {
    let iph: *const ipv6hdr = ipv6_hdr(skb);
    let sk: *mut sock = sk_to_full_sk(sk_partial);
    let dev: *mut net_device = skb_dst_dev(skb);
    let mut flkeys: flow_keys = core::mem::zeroed();
    let hh_len: u32;
    let mut dst: *mut dst_entry;
    let strict = (ipv6_addr_type(&(*iph).daddr)
        & (IPV6_ADDR_MULTICAST | IPV6_ADDR_LINKLOCAL)) != 0;
    let mut fl6: flowi6 = flowi6 {
        flowi6_l3mdev: l3mdev_master_ifindex(dev),
        flowi6_mark: (*skb).mark,
        flowi6_uid: sock_net_uid(net, sk),
        daddr: (*iph).daddr,
        saddr: (*iph).saddr,
        flowlabel: ip6_flowinfo(iph),
        ..core::mem::zeroed()
    };
    let err: i32;

    if !sk.is_null() && (*sk).sk_bound_dev_if != 0 {
        fl6.flowi6_oif = (*sk).sk_bound_dev_if;
    } else if strict {
        fl6.flowi6_oif = (*dev).ifindex;
    }

    fib6_rules_early_flow_dissect(net, skb, &mut fl6, &mut flkeys);
    dst = ip6_route_output(net, sk, &mut fl6);
    err = (*dst).error;
    if err != 0 {
        IP6_INC_STATS(net, ip6_dst_idev(dst), IPSTATS_MIB_OUTNOROUTES);
        net_dbg_ratelimited!("ip6_route_me_harder: No more route\n");
        dst_release(dst);
        return err;
    }

    /* Drop old route. */
    skb_dst_drop(skb);
    skb_dst_set(skb, dst);

    // CONFIG_XFRM: preserve the conditional kernel path when enabled.
    #[cfg(CONFIG_XFRM)]
    {
        if ((*IP6CB(skb)).flags & IP6SKB_XFRM_TRANSFORMED) == 0
            && xfrm_decode_session(net, skb, flowi6_to_flowi(&mut fl6), AF_INET6) == 0
        {
            /* ignore return value from skb_dstref_steal, xfrm_lookup takes
             * care of dropping the refcnt if needed.
             */
            skb_dstref_steal(skb);
            dst = xfrm_lookup(net, dst, flowi6_to_flowi(&mut fl6), sk, 0);
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

pub unsafe fn __nf_ip6_route(
    net: *mut net,
    dst: *mut *mut dst_entry,
    fl: *mut flowi,
    strict: bool,
) -> i32 {
    static FAKE_PINFO: ipv6_pinfo = unsafe { core::mem::zeroed() };
    static mut FAKE_SK: inet_sock = unsafe { core::mem::zeroed() };
    let sk: *const core::ffi::c_void = if strict {
        (*FAKE_SK.sk.sk_bound_dev_if = 1);
        FAKE_SK.pinet6 = &FAKE_PINFO as *const _ as *mut _;
        &FAKE_SK as *const _ as *const core::ffi::c_void
    } else {
        core::ptr::null()
    };
    let result: *mut dst_entry = ip6_route_output(net, sk as *mut sock, &mut (*fl).u.ip6);
    let err = (*result).error;
    if err != 0 {
        dst_release(result);
    } else {
        *dst = result;
    }
    err
}

pub unsafe fn br_ip6_fragment(
    net: *mut net,
    sk: *mut sock,
    mut skb: *mut sk_buff,
    data: *mut nf_bridge_frag_data,
    output: unsafe extern "C" fn(*mut net, *mut sock, *const nf_bridge_frag_data, *mut sk_buff) -> i32,
) -> i32 {
    let frag_max_size = (*BR_INPUT_SKB_CB(skb)).frag_max_size;
    let tstamp_type = (*skb).tstamp_type;
    let tstamp = (*skb).tstamp;
    let mut state: ip6_frag_state = core::mem::zeroed();
    let mut prevhdr: *mut u8;
    let mut nexthdr: u8 = 0;
    let mut mtu: u32;
    let hlen: u32;
    let nexthdr_offset: u32;
    let hroom: i32;
    let mut err = 0;
    let frag_id: __be32;

    err = ip6_find_1stfragopt(skb, &mut prevhdr);
    if err < 0 { kfree_skb(skb); return 0; }
    hlen = err as u32;
    nexthdr = *prevhdr;
    nexthdr_offset = prevhdr.offset_from(skb_network_header(skb)) as u32;
    mtu = (*(*skb).dev).mtu;
    if frag_max_size > mtu || frag_max_size < IPV6_MIN_MTU { kfree_skb(skb); return 0; }
    mtu = frag_max_size;
    if mtu < hlen + core::mem::size_of::<frag_hdr>() as u32 + 8 { kfree_skb(skb); return 0; }
    mtu -= hlen + core::mem::size_of::<frag_hdr>() as u32;
    frag_id = ipv6_select_ident(net, &(*ipv6_hdr(skb)).daddr, &(*ipv6_hdr(skb)).saddr);
    if (*skb).ip_summed == CHECKSUM_PARTIAL && { err = skb_checksum_help(skb); err != 0 } { kfree_skb(skb); return 0; }
    prevhdr = skb_network_header(skb).add(nexthdr_offset as usize);
    hroom = LL_RESERVED_SPACE((*skb).dev);
    // The frag-list and slow-path logic is supplied by the corresponding kernel bindings.
    ip6_frag_init(skb, hlen, mtu, (*(*skb).dev).needed_tailroom, LL_RESERVED_SPACE((*skb).dev), prevhdr, nexthdr, frag_id, &mut state);
    while state.left > 0 {
        let skb2 = ip6_frag_next(skb, &mut state);
        if IS_ERR(skb2) { err = PTR_ERR(skb2); kfree_skb(skb); return 0; }
        skb_set_delivery_time(skb2, tstamp, tstamp_type);
        err = output(net, sk, data, skb2);
        if err != 0 { kfree_skb(skb); return 0; }
    }
    consume_skb(skb);
    return err;

}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
