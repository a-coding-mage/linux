// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * mpls tunnels  An implementation mpls tunnels using the light weight tunnel
 *              infrastructure
 *
 * Authors:     Roopa Prabhu, <roopa@cumulusnetworks.com>
 */

// Linux kernel dependencies supplied by the surrounding translation.

static MPLS_IPTUNNEL_POLICY: [nla_policy; MPLS_IPTUNNEL_MAX + 1] = {
    let mut policy = [nla_policy { len: 0, type_: 0 }; MPLS_IPTUNNEL_MAX + 1];
    policy[MPLS_IPTUNNEL_DST] = nla_policy { len: core::mem::size_of::<u32>() as u16, type_: 0 };
    policy[MPLS_IPTUNNEL_TTL] = nla_policy { len: 0, type_: NLA_U8 };
    policy
};

unsafe fn mpls_encap_size(en: *mut mpls_iptunnel_encap) -> u32 {
    /* The size of the layer 2.5 labels to be added for this route */
    (*en).labels as u32 * core::mem::size_of::<mpls_shim_hdr>() as u32
}

unsafe fn mpls_xmit(skb: *mut sk_buff) -> i32 {
    let mut tun_encap_info: *mut mpls_iptunnel_encap;
    let mut hdr: *mut mpls_shim_hdr;
    let mut out_dev: *mut net_device;
    let mut hh_len: u32;
    let mut new_header_size: u32;
    let mut mtu: u32;
    let dst: *mut dst_entry = skb_dst(skb);
    let mut rt: *mut rtable = core::ptr::null_mut();
    let mut rt6: *mut rt6_info = core::ptr::null_mut();
    let mut out_mdev: *mut mpls_dev;
    let mut net: *mut net;
    let mut err: i32 = 0;
    let mut bos: bool;
    let mut i: i32;
    let ttl: u32;

    /* Find the output device */
    out_dev = (*dst).dev;
    net = dev_net_rcu(out_dev);

    if !mpls_output_possible(out_dev) || (*dst).lwtstate.is_null() || skb_warn_if_lro(skb) != 0 {
        goto_drop(skb, out_dev);
        return -EINVAL;
    }

    skb_forward_csum(skb);
    tun_encap_info = mpls_lwtunnel_encap((*dst).lwtstate);

    /* Obtain the ttl using the LWT and global propagation settings. */
    if (*(*dst).ops).family == AF_INET {
        ttl = if (*tun_encap_info).ttl_propagate == MPLS_TTL_PROP_DISABLED {
            (*tun_encap_info).default_ttl as u32
        } else if (*tun_encap_info).ttl_propagate == MPLS_TTL_PROP_DEFAULT && !(*(*net).mpls).ip_ttl_propagate {
            (*(*net).mpls).default_ttl as u32
        } else { (*ip_hdr(skb)).ttl as u32 };
        rt = dst_rtable(dst);
    } else if (*(*dst).ops).family == AF_INET6 {
        ttl = if (*tun_encap_info).ttl_propagate == MPLS_TTL_PROP_DISABLED {
            (*tun_encap_info).default_ttl as u32
        } else if (*tun_encap_info).ttl_propagate == MPLS_TTL_PROP_DEFAULT && !(*(*net).mpls).ip_ttl_propagate {
            (*(*net).mpls).default_ttl as u32
        } else { (*ipv6_hdr(skb)).hop_limit as u32 };
        rt6 = dst_rt6_info(dst);
    } else {
        goto_drop(skb, out_dev);
        return -EINVAL;
    }

    /* Verify the destination can hold the packet */
    new_header_size = mpls_encap_size(tun_encap_info);
    mtu = mpls_dev_mtu(out_dev);
    if mpls_pkt_too_big(skb, mtu - new_header_size) { goto_drop(skb, out_dev); return -EINVAL; }

    hh_len = if !(*out_dev).header_ops.is_null() { LL_RESERVED_SPACE(out_dev) } else { 0 };
    if skb_cow_head(skb, hh_len + new_header_size) != 0 { goto_drop(skb, out_dev); return -EINVAL; }

    skb_set_inner_protocol(skb, (*skb).protocol);
    skb_reset_inner_network_header(skb);
    skb_push(skb, new_header_size);
    skb_reset_network_header(skb);
    (*skb).dev = out_dev;
    (*skb).protocol = htons(ETH_P_MPLS_UC);

    /* Push the new labels */
    hdr = mpls_hdr(skb);
    bos = true;
    i = (*tun_encap_info).labels as i32 - 1;
    while i >= 0 {
        (*hdr.add(i as usize)) = mpls_entry_encode((*tun_encap_info).label.add(i as usize).read(), ttl, 0, bos);
        bos = false;
        i -= 1;
    }

    mpls_stats_inc_outucastpkts(net, out_dev, skb);
    if !rt.is_null() {
        err = if (*rt).rt_gw_family == AF_INET6 { neigh_xmit(NEIGH_ND_TABLE, out_dev, &mut (*rt).rt_gw6, skb) } else { neigh_xmit(NEIGH_ARP_TABLE, out_dev, &mut (*rt).rt_gw4, skb) };
    } else if !rt6.is_null() {
        if ipv6_addr_v4mapped(&(*rt6).rt6i_gateway) {
            /* 6PE (RFC 4798) */
            err = neigh_xmit(NEIGH_ARP_TABLE, out_dev, &mut (*rt6).rt6i_gateway.s6_addr32[3], skb);
        } else { err = neigh_xmit(NEIGH_ND_TABLE, out_dev, &mut (*rt6).rt6i_gateway, skb); }
    }
    if err != 0 { net_dbg_ratelimited!("%s: packet transmission failed: %d\n", __func__, err); }
    return LWTUNNEL_XMIT_DONE;
}

unsafe fn goto_drop(skb: *mut sk_buff, out_dev: *mut net_device) {
    let out_mdev = if !out_dev.is_null() { mpls_dev_rcu(out_dev) } else { core::ptr::null_mut() };
    if !out_mdev.is_null() { MPLS_INC_STATS(out_mdev, tx_errors); }
    kfree_skb(skb);
}

unsafe fn mpls_build_state(net: *mut net, nla: *mut nlattr, family: u32, cfg: *const core::ffi::c_void, ts: *mut *mut lwtunnel_state, extack: *mut netlink_ext_ack) -> i32 {
    let mut tb = [core::ptr::null_mut(); MPLS_IPTUNNEL_MAX + 1];
    let mut n_labels: u8 = 0;
    let ret = nla_parse_nested_deprecated(tb.as_mut_ptr(), MPLS_IPTUNNEL_MAX, nla, MPLS_IPTUNNEL_POLICY.as_ptr(), extack);
    if ret < 0 { return ret; }
    if tb[MPLS_IPTUNNEL_DST].is_null() { NL_SET_ERR_MSG(extack, "MPLS_IPTUNNEL_DST attribute is missing"); return -EINVAL; }
    if nla_get_labels(tb[MPLS_IPTUNNEL_DST], MAX_NEW_LABELS, &mut n_labels, core::ptr::null_mut(), extack) != 0 { return -EINVAL; }
    let newts = lwtunnel_state_alloc(struct_size::<mpls_iptunnel_encap>(n_labels));
    if newts.is_null() { return -ENOMEM; }
    let info = mpls_lwtunnel_encap(newts);
    let ret = nla_get_labels(tb[MPLS_IPTUNNEL_DST], n_labels, &mut (*info).labels, (*info).label, extack);
    if ret != 0 { kfree(newts); *ts = core::ptr::null_mut(); return ret; }
    (*info).ttl_propagate = MPLS_TTL_PROP_DEFAULT;
    if !tb[MPLS_IPTUNNEL_TTL].is_null() {
        (*info).default_ttl = nla_get_u8(tb[MPLS_IPTUNNEL_TTL]);
        (*info).ttl_propagate = if (*info).default_ttl != 0 { MPLS_TTL_PROP_DISABLED } else { MPLS_TTL_PROP_ENABLED };
    }
    (*newts).type_ = LWTUNNEL_ENCAP_MPLS;
    (*newts).flags |= LWTUNNEL_STATE_XMIT_REDIRECT;
    (*newts).headroom = mpls_encap_size(info);
    *ts = newts;
    0
}

unsafe fn mpls_fill_encap_info(skb: *mut sk_buff, lwtstate: *mut lwtunnel_state) -> i32 {
    let info = mpls_lwtunnel_encap(lwtstate);
    if nla_put_labels(skb, MPLS_IPTUNNEL_DST, (*info).labels, (*info).label) != 0 { return -EMSGSIZE; }
    if (*info).ttl_propagate != MPLS_TTL_PROP_DEFAULT && nla_put_u8(skb, MPLS_IPTUNNEL_TTL, (*info).default_ttl) != 0 { return -EMSGSIZE; }
    0
}

unsafe fn mpls_encap_nlsize(lwtstate: *mut lwtunnel_state) -> i32 {
    let info = mpls_lwtunnel_encap(lwtstate);
    let mut nlsize = nla_total_size((*info).labels as usize * 4);
    if (*info).ttl_propagate != MPLS_TTL_PROP_DEFAULT { nlsize += nla_total_size(1); }
    nlsize
}

unsafe fn mpls_encap_cmp(a: *mut lwtunnel_state, b: *mut lwtunnel_state) -> i32 {
    let ah = mpls_lwtunnel_encap(a); let bh = mpls_lwtunnel_encap(b);
    if (*ah).labels != (*bh).labels || (*ah).ttl_propagate != (*bh).ttl_propagate || (*ah).default_ttl != (*bh).default_ttl { return 1; }
    for l in 0..(*ah).labels as usize { if (*ah).label.add(l).read() != (*bh).label.add(l).read() { return 1; } }
    0
}

static MPLS_IPTUN_OPS: lwtunnel_encap_ops = lwtunnel_encap_ops {
    build_state: Some(mpls_build_state), xmit: Some(mpls_xmit), fill_encap: Some(mpls_fill_encap_info),
    get_encap_size: Some(mpls_encap_nlsize), cmp_encap: Some(mpls_encap_cmp), owner: THIS_MODULE,
};

unsafe fn mpls_iptunnel_init() -> i32 { lwtunnel_encap_add_ops(&MPLS_IPTUN_OPS, LWTUNNEL_ENCAP_MPLS) }
unsafe fn mpls_iptunnel_exit() { lwtunnel_encap_del_ops(&MPLS_IPTUN_OPS, LWTUNNEL_ENCAP_MPLS); }

module_init!(mpls_iptunnel_init);
module_exit!(mpls_iptunnel_exit);
MODULE_ALIAS_RTNL_LWT!(MPLS);
MODULE_SOFTDEP!("post: mpls_gso");
MODULE_DESCRIPTION!("MultiProtocol Label Switching IP Tunnels");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
