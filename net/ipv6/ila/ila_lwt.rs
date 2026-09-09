// SPDX-License-Identifier: GPL-2.0
// External Linux kernel dependencies are supplied by the surrounding translation.

#[repr(C)]
struct ila_lwt {
    p: ila_params,
    dst_cache: dst_cache,
    connected: u32,
    lwt_output: u32,
}

#[inline]
unsafe fn ila_lwt_lwtunnel(lwt: *mut lwtunnel_state) -> *mut ila_lwt {
    (*lwt).data as *mut ila_lwt
}

#[inline]
unsafe fn ila_params_lwtunnel(lwt: *mut lwtunnel_state) -> *mut ila_params {
    &mut (*ila_lwt_lwtunnel(lwt)).p
}

unsafe fn ila_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    let orig_dst = skb_dst(skb);
    let rt = dst_rt6_info(orig_dst);
    let ilwt = ila_lwt_lwtunnel((*orig_dst).lwtstate);
    let mut dst: *mut dst_entry;
    let mut err: c_int = -EINVAL;

    if (*skb).protocol != htons(ETH_P_IPV6) {
        goto_drop!(drop);
    }

    if (*ilwt).lwt_output != 0 {
        ila_update_ipv6_locator(
            skb,
            ila_params_lwtunnel((*orig_dst).lwtstate),
            true,
        );
    }

    if ((*rt).rt6i_flags & (RTF_GATEWAY | RTF_CACHE)) != 0 {
        return ((*(*orig_dst).lwtstate).orig_output)(net, sk, skb);
    }

    local_bh_disable();
    dst = dst_cache_get(&mut (*ilwt).dst_cache);
    local_bh_enable();
    if dst.is_null() {
        let ip6h = ipv6_hdr(skb);
        let mut fl6: flowi6 = core::mem::zeroed();

        fl6.flowi6_oif = (*dst_dev(orig_dst)).ifindex;
        fl6.flowi6_iif = LOOPBACK_IFINDEX;
        fl6.daddr = *rt6_nexthop(dst_rt6_info(orig_dst), &(*ip6h).daddr);

        dst = ip6_route_output(net, core::ptr::null_mut(), &mut fl6);
        if (*dst).error != 0 {
            err = -EHOSTUNREACH;
            dst_release(dst);
            goto_drop!(drop);
        }

        dst = xfrm_lookup(net, dst, flowi6_to_flowi(&mut fl6), core::ptr::null_mut(), 0);
        if IS_ERR(dst) {
            err = PTR_ERR(dst);
            goto_drop!(drop);
        }

        if (*ilwt).connected != 0 && (*orig_dst).lwtstate != (*dst).lwtstate {
            local_bh_disable();
            dst_cache_set_ip6(&mut (*ilwt).dst_cache, dst, &fl6.saddr);
            local_bh_enable();
        }
    }

    skb_dst_drop(skb);
    skb_dst_set(skb, dst);
    return dst_output(net, sk, skb);

drop:
    kfree_skb(skb);
    err
}

unsafe fn ila_input(skb: *mut sk_buff) -> c_int {
    let dst = skb_dst(skb);
    let ilwt = ila_lwt_lwtunnel((*dst).lwtstate);

    if (*skb).protocol != htons(ETH_P_IPV6) {
        kfree_skb(skb);
        return -EINVAL;
    }

    if (*ilwt).lwt_output == 0 {
        ila_update_ipv6_locator(skb, ila_params_lwtunnel((*dst).lwtstate), false);
    }

    ((*(*dst).lwtstate).orig_input)(skb)
}

static ila_nl_policy: [nla_policy; ILA_ATTR_MAX + 1] = [
    [ILA_ATTR_LOCATOR] = nla_policy { type_: NLA_U64 },
    [ILA_ATTR_CSUM_MODE] = nla_policy { type_: NLA_U8 },
    [ILA_ATTR_IDENT_TYPE] = nla_policy { type_: NLA_U8 },
    [ILA_ATTR_HOOK_TYPE] = nla_policy { type_: NLA_U8 },
];

unsafe fn ila_build_state(
    net: *mut net, nla: *mut nlattr, family: c_uint, cfg: *const c_void,
    ts: *mut *mut lwtunnel_state, extack: *mut netlink_ext_ack,
) -> c_int {
    let cfg6 = cfg as *const fib6_config;
    let mut ident_type: u8 = ILA_ATYPE_USE_FORMAT;
    let mut hook_type: u8 = ILA_HOOK_ROUTE_OUTPUT;
    let mut csum_mode: u8 = ILA_CSUM_NO_ACTION;
    let mut lwt_output = true;
    let mut eff_ident_type: u8;
    let mut ret: c_int;

    if family != AF_INET6 { return -EINVAL; }

    let mut tb: [*mut nlattr; ILA_ATTR_MAX + 1] = [core::ptr::null_mut(); ILA_ATTR_MAX + 1];
    ret = nla_parse_nested_deprecated(tb.as_mut_ptr(), ILA_ATTR_MAX, nla, ila_nl_policy.as_ptr(), extack);
    if ret < 0 { return ret; }
    if tb[ILA_ATTR_LOCATOR].is_null() { return -EINVAL; }

    let iaddr = &mut *((&(*cfg6).fc_dst) as *const _ as *mut ila_addr);
    if !tb[ILA_ATTR_IDENT_TYPE].is_null() { ident_type = nla_get_u8(tb[ILA_ATTR_IDENT_TYPE]); }
    if ident_type == ILA_ATYPE_USE_FORMAT {
        if (*cfg6).fc_dst_len < 8 * core::mem::size_of::<ila_locator>() as u8 + 3 { return -EINVAL; }
        eff_ident_type = iaddr.ident.type_;
    } else { eff_ident_type = ident_type; }

    match eff_ident_type {
        ILA_ATYPE_LUID => (),
        ILA_ATYPE_IID | ILA_ATYPE_VIRT_V4 | ILA_ATYPE_VIRT_UNI_V6 |
        ILA_ATYPE_VIRT_MULTI_V6 | ILA_ATYPE_NONLOCAL_ADDR => return -EINVAL,
        _ => return -EINVAL,
    }

    if !tb[ILA_ATTR_HOOK_TYPE].is_null() { hook_type = nla_get_u8(tb[ILA_ATTR_HOOK_TYPE]); }
    match hook_type { ILA_HOOK_ROUTE_OUTPUT => lwt_output = true, ILA_HOOK_ROUTE_INPUT => lwt_output = false, _ => return -EINVAL }
    if !tb[ILA_ATTR_CSUM_MODE].is_null() { csum_mode = nla_get_u8(tb[ILA_ATTR_CSUM_MODE]); }
    if csum_mode == ILA_CSUM_NEUTRAL_MAP && ila_csum_neutral_set(iaddr.ident) { return -EINVAL; }

    let newts = lwtunnel_state_alloc(core::mem::size_of::<ila_lwt>());
    if newts.is_null() { return -ENOMEM; }
    let ilwt = ila_lwt_lwtunnel(newts);
    ret = dst_cache_init(&mut (*ilwt).dst_cache, GFP_ATOMIC);
    if ret != 0 { kfree(newts as *mut c_void); return ret; }
    (*ilwt).lwt_output = lwt_output as u32;
    let p = ila_params_lwtunnel(newts);
    (*p).csum_mode = csum_mode;
    (*p).ident_type = ident_type;
    (*p).locator.v64 = nla_get_u64(tb[ILA_ATTR_LOCATOR]) as __be64;
    (*p).locator_match = iaddr.loc;
    ila_init_saved_csum(p);
    (*newts).type_ = LWTUNNEL_ENCAP_ILA;
    (*newts).flags |= LWTUNNEL_STATE_OUTPUT_REDIRECT | LWTUNNEL_STATE_INPUT_REDIRECT;
    if (*cfg6).fc_dst_len == 8 * core::mem::size_of::<in6_addr>() as u8 { (*ilwt).connected = 1; }
    *ts = newts;
    0
}

unsafe fn ila_destroy_state(lwt: *mut lwtunnel_state) { dst_cache_destroy(&mut (*ila_lwt_lwtunnel(lwt)).dst_cache); }

unsafe fn ila_fill_encap_info(skb: *mut sk_buff, lwtstate: *mut lwtunnel_state) -> c_int {
    let p = ila_params_lwtunnel(lwtstate);
    let ilwt = ila_lwt_lwtunnel(lwtstate);
    if nla_put_u64_64bit(skb, ILA_ATTR_LOCATOR, (*p).locator.v64 as u64, ILA_ATTR_PAD) != 0 { return -EMSGSIZE; }
    if nla_put_u8(skb, ILA_ATTR_CSUM_MODE, (*p).csum_mode as u8) != 0 { return -EMSGSIZE; }
    if nla_put_u8(skb, ILA_ATTR_IDENT_TYPE, (*p).ident_type as u8) != 0 { return -EMSGSIZE; }
    if nla_put_u8(skb, ILA_ATTR_HOOK_TYPE, if (*ilwt).lwt_output != 0 { ILA_HOOK_ROUTE_OUTPUT } else { ILA_HOOK_ROUTE_INPUT }) != 0 { return -EMSGSIZE; }
    0
}

unsafe fn ila_encap_nlsize(_lwtstate: *mut lwtunnel_state) -> c_int {
    nla_total_size_64bit(core::mem::size_of::<u64>()) + nla_total_size(core::mem::size_of::<u8>()) * 3
}

unsafe fn ila_encap_cmp(a: *mut lwtunnel_state, b: *mut lwtunnel_state) -> c_int {
    (ila_params_lwtunnel(a).as_ref().unwrap().locator.v64 != ila_params_lwtunnel(b).as_ref().unwrap().locator.v64) as c_int
}

static ila_encap_ops: lwtunnel_encap_ops = lwtunnel_encap_ops {
    build_state: Some(ila_build_state), destroy: Some(ila_destroy_state), output: Some(ila_output),
    input: Some(ila_input), fill_encap: Some(ila_fill_encap_info), get_encap_size: Some(ila_encap_nlsize),
    cmp_encap: Some(ila_encap_cmp), owner: THIS_MODULE,
};

unsafe fn ila_lwt_init() -> c_int { lwtunnel_encap_add_ops(&ila_encap_ops, LWTUNNEL_ENCAP_ILA) }
unsafe fn ila_lwt_fini() { lwtunnel_encap_del_ops(&ila_encap_ops, LWTUNNEL_ENCAP_ILA); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
