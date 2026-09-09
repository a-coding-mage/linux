// SPDX-License-Identifier: GPL-2.0-only
/*
 * Authors:
 * (C) 2020 Alexander Aring <alex.aring@gmail.com>
 */

// External Linux kernel declarations and build-time constants are supplied by
// the surrounding translation unit.

#[repr(C)]
pub struct rpl_iptunnel_encap {
    pub srh: [ipv6_rpl_sr_hdr; 0],
}

#[repr(C)]
pub struct rpl_lwt {
    pub cache: dst_cache,
    pub tuninfo: rpl_iptunnel_encap,
}

#[inline]
unsafe fn rpl_lwt_lwtunnel(lwt: *mut lwtunnel_state) -> *mut rpl_lwt {
    (*lwt).data as *mut rpl_lwt
}

#[inline]
unsafe fn rpl_encap_lwtunnel(lwt: *mut lwtunnel_state) -> *mut rpl_iptunnel_encap {
    &mut (*rpl_lwt_lwtunnel(lwt)).tuninfo
}

static rpl_iptunnel_policy: [nla_policy; (RPL_IPTUNNEL_MAX + 1) as usize] = {
    let mut policy = [nla_policy { type_: 0 }; (RPL_IPTUNNEL_MAX + 1) as usize];
    policy[RPL_IPTUNNEL_SRH as usize] = nla_policy { type_: NLA_BINARY };
    policy
};

unsafe fn rpl_validate_srh(
    net: *mut net,
    srh: *mut ipv6_rpl_sr_hdr,
    seglen: usize,
) -> bool {
    let err: i32;

    if (((*srh).hdrlen as usize) << 3) != seglen {
        return false;
    }

    /* check at least one segment and seglen fit with segments_left */
    if (*srh).segments_left == 0
        || ((*srh).segments_left as usize * core::mem::size_of::<in6_addr>()) != seglen
    {
        return false;
    }

    if (*srh).cmpri != 0 || (*srh).cmpre != 0 {
        return false;
    }

    err = ipv6_chk_rpl_srh_loop(net, (*srh).rpl_segaddr.as_mut_ptr(), (*srh).segments_left);
    if err != 0 {
        return false;
    }

    if (ipv6_addr_type(
        &(*srh).rpl_segaddr[(*srh).segments_left as usize - 1],
    ) & IPV6_ADDR_MULTICAST) != 0
    {
        return false;
    }

    true
}

unsafe fn rpl_build_state(
    net: *mut net,
    nla: *mut nlattr,
    family: c_uint,
    cfg: *const c_void,
    ts: *mut *mut lwtunnel_state,
    extack: *mut netlink_ext_ack,
) -> i32 {
    let mut tb: [*mut nlattr; (RPL_IPTUNNEL_MAX + 1) as usize] = [core::ptr::null_mut(); (RPL_IPTUNNEL_MAX + 1) as usize];
    let newts: *mut lwtunnel_state;
    let srh: *mut ipv6_rpl_sr_hdr;
    let rlwt: *mut rpl_lwt;
    let err: i32;
    let srh_len: i32;

    if family != AF_INET6 {
        return -EINVAL;
    }

    err = nla_parse_nested(
        tb.as_mut_ptr(), RPL_IPTUNNEL_MAX, nla, rpl_iptunnel_policy.as_ptr(), extack,
    );
    if err < 0 {
        return err;
    }

    if tb[RPL_IPTUNNEL_SRH as usize].is_null() {
        return -EINVAL;
    }

    srh = nla_data(tb[RPL_IPTUNNEL_SRH as usize]) as *mut ipv6_rpl_sr_hdr;
    srh_len = nla_len(tb[RPL_IPTUNNEL_SRH as usize]);

    if srh_len < core::mem::size_of::<ipv6_rpl_sr_hdr>() as i32 {
        return -EINVAL;
    }

    /* verify that SRH is consistent */
    if !rpl_validate_srh(net, srh, (srh_len as usize) - core::mem::size_of::<ipv6_rpl_sr_hdr>()) {
        return -EINVAL;
    }

    newts = lwtunnel_state_alloc(
        srh_len as usize + core::mem::size_of::<rpl_lwt>(),
    );
    if newts.is_null() {
        return -ENOMEM;
    }

    rlwt = rpl_lwt_lwtunnel(newts);

    err = dst_cache_init(&mut (*rlwt).cache, GFP_ATOMIC);
    if err != 0 {
        kfree(newts as *mut c_void);
        return err;
    }

    memcpy(
        &mut (*rlwt).tuninfo.srh as *mut _ as *mut c_void,
        srh as *const c_void,
        srh_len as usize,
    );

    (*newts).type_ = LWTUNNEL_ENCAP_RPL;
    (*newts).flags |= LWTUNNEL_STATE_INPUT_REDIRECT;
    (*newts).flags |= LWTUNNEL_STATE_OUTPUT_REDIRECT;
    *ts = newts;
    0
}

unsafe fn rpl_destroy_state(lwt: *mut lwtunnel_state) {
    dst_cache_destroy(&mut (*rpl_lwt_lwtunnel(lwt)).cache);
}

unsafe fn rpl_do_srh_inline(
    skb: *mut sk_buff,
    rlwt: *const rpl_lwt,
    srh: *const ipv6_rpl_sr_hdr,
    cache_dst: *mut dst_entry,
) -> i32 {
    let mut oldhdr: ipv6hdr = core::mem::zeroed();
    let mut hdr: *mut ipv6hdr;
    let mut buf: *mut u8;
    let mut hdrlen: usize;
    let err: i32;
    let isrh: *mut ipv6_rpl_sr_hdr;
    let csrh: *mut ipv6_rpl_sr_hdr;

    memcpy(&mut oldhdr as *mut _ as *mut c_void, ipv6_hdr(skb) as *const c_void, core::mem::size_of::<ipv6hdr>());
    buf = kcalloc(
        struct_size(srh, segments.addr, (*srh).segments_left),
        2,
        GFP_ATOMIC,
    ) as *mut u8;
    if buf.is_null() { return -ENOMEM; }

    isrh = buf as *mut ipv6_rpl_sr_hdr;
    csrh = buf.add((((*srh).hdrlen as usize + 1) << 3)) as *mut ipv6_rpl_sr_hdr;
    memcpy(isrh as *mut c_void, srh as *const c_void, core::mem::size_of::<ipv6_rpl_sr_hdr>());
    memcpy((*isrh).rpl_segaddr.as_mut_ptr() as *mut c_void, (*srh).rpl_segaddr.as_ptr().add(1) as *const c_void, ((*srh).segments_left as usize - 1) * 16);
    (*isrh).rpl_segaddr[(*srh).segments_left as usize - 1] = oldhdr.daddr;
    ipv6_rpl_srh_compress(csrh, isrh, &(*srh).rpl_segaddr[0], (*isrh).segments_left - 1);
    hdrlen = ((*csrh).hdrlen as usize + 1) << 3;
    err = skb_cow_head(skb, hdrlen + dst_dev_overhead(cache_dst, skb));
    if unlikely(err != 0) { kfree(buf as *mut c_void); return err; }
    skb_pull(skb, core::mem::size_of::<ipv6hdr>());
    skb_postpull_rcsum(skb, skb_network_header(skb), core::mem::size_of::<ipv6hdr>());
    skb_push(skb, core::mem::size_of::<ipv6hdr>() + hdrlen);
    skb_reset_network_header(skb);
    skb_mac_header_rebuild(skb);
    hdr = ipv6_hdr(skb);
    memmove(hdr as *mut c_void, &oldhdr as *const _ as *const c_void, core::mem::size_of::<ipv6hdr>());
    isrh = (hdr as *mut u8).add(core::mem::size_of::<ipv6hdr>()) as *mut ipv6_rpl_sr_hdr;
    memcpy(isrh as *mut c_void, csrh as *const c_void, hdrlen);
    (*isrh).nexthdr = (*hdr).nexthdr;
    (*hdr).nexthdr = NEXTHDR_ROUTING;
    (*hdr).daddr = (*srh).rpl_segaddr[0];
    (*ipv6_hdr(skb)).payload_len = htons(((*skb).len - core::mem::size_of::<ipv6hdr>()) as u16);
    skb_set_transport_header(skb, core::mem::size_of::<ipv6hdr>());
    skb_postpush_rcsum(skb, hdr, core::mem::size_of::<ipv6hdr>() + hdrlen);
    kfree(buf as *mut c_void);
    0
}

unsafe fn rpl_do_srh(skb: *mut sk_buff, rlwt: *const rpl_lwt, cache_dst: *mut dst_entry) -> i32 {
    let dst = skb_dst(skb);
    if (*skb).protocol != htons(ETH_P_IPV6) { return -EINVAL; }
    let tinfo = rpl_encap_lwtunnel((*dst).lwtstate);
    rpl_do_srh_inline(skb, rlwt, (*tinfo).srh.as_ptr(), cache_dst)
}

unsafe fn rpl_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> i32 {
    let orig_dst = skb_dst(skb);
    let mut dst: *mut dst_entry = core::ptr::null_mut();
    let rlwt = rpl_lwt_lwtunnel((*orig_dst).lwtstate);
    local_bh_disable(); dst = dst_cache_get(&mut (*rlwt).cache); local_bh_enable();
    let mut err = rpl_do_srh(skb, rlwt, dst);
    if unlikely(err != 0) { dst_release(dst); kfree_skb(skb); return err; }
    if dst.is_null() {
        let hdr = ipv6_hdr(skb); let mut fl6: flowi6 = core::mem::zeroed();
        fl6.daddr = (*hdr).daddr; fl6.saddr = (*hdr).saddr; fl6.flowlabel = ip6_flowinfo(hdr); fl6.flowi6_mark = (*skb).mark; fl6.flowi6_proto = (*hdr).nexthdr;
        dst = ip6_route_output(net, core::ptr::null_mut(), &mut fl6);
        if (*dst).error != 0 { err = (*dst).error; dst_release(dst); kfree_skb(skb); return err; }
        if (*orig_dst).lwtstate != (*dst).lwtstate { local_bh_disable(); dst_cache_set_ip6(&mut (*rlwt).cache, dst, &fl6.saddr); local_bh_enable(); }
        err = skb_cow_head(skb, LL_RESERVED_SPACE(dst_dev(dst))); if unlikely(err != 0) { dst_release(dst); kfree_skb(skb); return err; }
    }
    skb_dst_drop(skb); skb_dst_set(skb, dst); dst_output(net, sk, skb)
}

unsafe fn rpl_input(skb: *mut sk_buff) -> i32 {
    let orig_dst = skb_dst(skb); let mut dst: *mut dst_entry = core::ptr::null_mut();
    let lwtst = (*orig_dst).lwtstate; let rlwt = rpl_lwt_lwtunnel(lwtst);
    local_bh_disable(); dst = dst_cache_get(&mut (*rlwt).cache); local_bh_enable();
    let mut err = rpl_do_srh(skb, rlwt, dst);
    if unlikely(err != 0) { dst_release(dst); kfree_skb(skb); return err; }
    if dst.is_null() {
        ip6_route_input(skb); skb_dst_force(skb); dst = skb_dst(skb);
        if dst.is_null() { err = -ENETUNREACH; kfree_skb(skb); return err; }
        if (*dst).error == 0 && lwtst != (*dst).lwtstate { local_bh_disable(); dst_cache_set_ip6(&mut (*rlwt).cache, dst, &(*ipv6_hdr(skb)).saddr); local_bh_enable(); }
        err = skb_cow_head(skb, LL_RESERVED_SPACE(dst_dev(dst))); if unlikely(err != 0) { kfree_skb(skb); return err; }
    } else { skb_dst_drop(skb); skb_dst_set(skb, dst); }
    dst_input(skb)
}

unsafe fn nla_put_rpl_srh(skb: *mut sk_buff, attrtype: i32, tuninfo: *mut rpl_iptunnel_encap) -> i32 {
    let len = RPL_IPTUNNEL_SRH_SIZE((*tuninfo).srh.as_ptr());
    let nla = nla_reserve(skb, attrtype, len); if nla.is_null() { return -EMSGSIZE; }
    memcpy(nla_data(nla), (*tuninfo).srh.as_ptr() as *const c_void, len); 0
}

unsafe fn rpl_fill_encap_info(skb: *mut sk_buff, lwtstate: *mut lwtunnel_state) -> i32 {
    let tuninfo = rpl_encap_lwtunnel(lwtstate); if nla_put_rpl_srh(skb, RPL_IPTUNNEL_SRH, tuninfo) != 0 { return -EMSGSIZE; } 0
}

unsafe fn rpl_encap_nlsize(lwtstate: *mut lwtunnel_state) -> i32 {
    nla_total_size(RPL_IPTUNNEL_SRH_SIZE((*rpl_encap_lwtunnel(lwtstate)).srh.as_ptr()))
}

unsafe fn rpl_encap_cmp(a: *mut lwtunnel_state, b: *mut lwtunnel_state) -> i32 {
    let ah = rpl_encap_lwtunnel(a); let bh = rpl_encap_lwtunnel(b); let len = RPL_IPTUNNEL_SRH_SIZE((*ah).srh.as_ptr());
    if len != RPL_IPTUNNEL_SRH_SIZE((*bh).srh.as_ptr()) { return 1; }
    memcmp(ah as *const c_void, bh as *const c_void, len)
}

static rpl_ops: lwtunnel_encap_ops = lwtunnel_encap_ops {
    build_state: Some(rpl_build_state), destroy_state: Some(rpl_destroy_state), output: Some(rpl_output), input: Some(rpl_input), fill_encap: Some(rpl_fill_encap_info), get_encap_size: Some(rpl_encap_nlsize), cmp_encap: Some(rpl_encap_cmp), owner: THIS_MODULE,
};

pub unsafe fn rpl_init() -> i32 {
    let err = lwtunnel_encap_add_ops(&rpl_ops, LWTUNNEL_ENCAP_RPL);
    if err != 0 { return err; }
    pr_info!("RPL Segment Routing with IPv6\n");
    0
}

pub unsafe fn rpl_exit() {
    lwtunnel_encap_del_ops(&rpl_ops, LWTUNNEL_ENCAP_RPL);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
