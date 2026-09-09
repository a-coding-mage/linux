// SPDX-License-Identifier: GPL-2.0
// C dependencies supplied by the surrounding kernel translation unit.

unsafe fn nft_meta_get_bridge(dev: *const net_device) -> *const net_device {
    if !dev.is_null() && netif_is_bridge_port(dev) {
        return netdev_master_upper_dev_get_rcu(dev as *mut net_device);
    }

    core::ptr::null()
}

unsafe extern "C" fn nft_meta_bridge_get_eval(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
) {
    let priv_ = nft_expr_priv(expr);
    let in_ = nft_in(pkt);
    let out = nft_out(pkt);
    let dest: *mut u32 = (*regs).data.as_mut_ptr().add((*priv_).dreg as usize);
    let br_dev: *const net_device;

    match (*priv_).key {
        NFT_META_BRI_IIFNAME => {
            br_dev = nft_meta_get_bridge(in_);
        }
        NFT_META_BRI_OIFNAME => {
            br_dev = nft_meta_get_bridge(out);
        }
        NFT_META_BRI_IIFPVID => {
            let mut p_pvid: u16 = 0;
            br_dev = nft_meta_get_bridge(in_);
            if br_dev.is_null() || !br_vlan_enabled(br_dev) {
                (*regs).verdict.code = NFT_BREAK;
                return;
            }
            if br_vlan_get_pvid_rcu(in_, &mut p_pvid) != 0 {
                (*regs).verdict.code = NFT_BREAK;
                return;
            }
            nft_reg_store16(dest, p_pvid);
            return;
        }
        NFT_META_BRI_IIFVPROTO => {
            let mut p_proto: u16 = 0;
            br_dev = nft_meta_get_bridge(in_);
            if br_dev.is_null() || !br_vlan_enabled(br_dev) {
                (*regs).verdict.code = NFT_BREAK;
                return;
            }
            br_vlan_get_proto(br_dev, &mut p_proto);
            nft_reg_store_be16(dest, htons(p_proto));
            return;
        }
        NFT_META_BRI_IIFHWADDR => {
            br_dev = nft_meta_get_bridge(in_);
            if br_dev.is_null() {
                (*regs).verdict.code = NFT_BREAK;
                return;
            }
            // ETH_ALEN (6) is shorter than the destination register span (8)
            *dest.add(1) = 0;
            core::ptr::copy_nonoverlapping(
                (*br_dev).dev_addr.as_ptr(),
                dest as *mut u8,
                ETH_ALEN as usize,
            );
            return;
        }
        _ => {
            nft_meta_get_eval(expr, regs, pkt);
            return;
        }
    }

    strscpy_pad(
        dest as *mut i8,
        if !br_dev.is_null() { (*br_dev).name.as_ptr() } else { b"\0".as_ptr() },
        IFNAMSIZ as usize,
    );
}

unsafe extern "C" fn nft_meta_bridge_get_init(
    ctx: *const nft_ctx,
    expr: *const nft_expr,
    tb: *const *const nlattr,
) -> i32 {
    let priv_ = nft_expr_priv(expr);
    let len: usize;

    (*priv_).key = ntohl(nla_get_be32(*tb.add(NFTA_META_KEY as usize)));
    match (*priv_).key {
        NFT_META_BRI_IIFNAME | NFT_META_BRI_OIFNAME => len = IFNAMSIZ as usize,
        NFT_META_BRI_IIFPVID | NFT_META_BRI_IIFVPROTO => len = core::mem::size_of::<u16>(),
        NFT_META_BRI_IIFHWADDR => len = ETH_ALEN as usize,
        _ => return nft_meta_get_init(ctx, expr, tb),
    }

    (*priv_).len = len as u32;
    nft_parse_register_store(
        ctx,
        *tb.add(NFTA_META_DREG as usize),
        &mut (*priv_).dreg,
        core::ptr::null_mut(),
        NFT_DATA_VALUE,
        len,
    )
}

unsafe extern "C" fn nft_meta_bridge_get_validate(
    ctx: *const nft_ctx,
    expr: *const nft_expr,
) -> i32 {
    let priv_ = nft_expr_priv(expr);
    let hooks: u32;
    match (*priv_).key {
        NFT_META_BRI_IIFHWADDR => hooks = 1u32 << NF_BR_PRE_ROUTING,
        _ => return nft_meta_get_validate(ctx, expr),
    }
    nft_chain_validate_hooks((*ctx).chain, hooks)
}

static mut nft_meta_bridge_type: nft_expr_type = nft_expr_type {
    family: NFPROTO_BRIDGE,
    name: b"meta\0".as_ptr() as *const i8,
    select_ops: Some(nft_meta_bridge_select_ops),
    policy: nft_meta_policy,
    maxattr: NFTA_META_MAX,
    owner: THIS_MODULE,
};

static nft_meta_bridge_get_ops: nft_expr_ops = nft_expr_ops {
    type_: unsafe { &mut nft_meta_bridge_type },
    size: NFT_EXPR_SIZE(core::mem::size_of::<nft_meta>()),
    eval: Some(nft_meta_bridge_get_eval),
    init: Some(nft_meta_bridge_get_init),
    validate: Some(nft_meta_bridge_get_validate),
    dump: Some(nft_meta_get_dump),
};

unsafe extern "C" fn nft_meta_bridge_set_eval(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
) {
    let meta = nft_expr_priv(expr);
    let sreg: *mut u32 = (*regs).data.as_mut_ptr().add((*meta).sreg as usize);
    let skb = (*pkt).skb;
    let value8: u8;

    match (*meta).key {
        NFT_META_BRI_BROUTE => {
            value8 = nft_reg_load8(sreg);
            (*BR_INPUT_SKB_CB(skb)).br_netfilter_broute = (value8 != 0) as u8;
        }
        _ => nft_meta_set_eval(expr, regs, pkt),
    }
}

unsafe extern "C" fn nft_meta_bridge_set_init(
    ctx: *const nft_ctx,
    expr: *const nft_expr,
    tb: *const *const nlattr,
) -> i32 {
    let priv_ = nft_expr_priv(expr);
    let len: usize;
    let err: i32;

    (*priv_).key = ntohl(nla_get_be32(*tb.add(NFTA_META_KEY as usize)));
    match (*priv_).key {
        NFT_META_BRI_BROUTE => len = core::mem::size_of::<u8>(),
        _ => return nft_meta_set_init(ctx, expr, tb),
    }

    (*priv_).len = len as u32;
    err = nft_parse_register_load(
        ctx,
        *tb.add(NFTA_META_SREG as usize),
        &mut (*priv_).sreg,
        len,
    );
    if err < 0 { return err; }
    0
}

unsafe extern "C" fn nft_meta_bridge_set_validate(
    ctx: *const nft_ctx,
    expr: *const nft_expr,
) -> i32 {
    let priv_ = nft_expr_priv(expr);
    let hooks: u32;
    match (*priv_).key {
        NFT_META_BRI_BROUTE => hooks = 1u32 << NF_BR_PRE_ROUTING,
        _ => return nft_meta_set_validate(ctx, expr),
    }
    nft_chain_validate_hooks((*ctx).chain, hooks)
}

static nft_meta_bridge_set_ops: nft_expr_ops = nft_expr_ops {
    type_: unsafe { &mut nft_meta_bridge_type },
    size: NFT_EXPR_SIZE(core::mem::size_of::<nft_meta>()),
    eval: Some(nft_meta_bridge_set_eval),
    init: Some(nft_meta_bridge_set_init),
    destroy: Some(nft_meta_set_destroy),
    dump: Some(nft_meta_set_dump),
    validate: Some(nft_meta_bridge_set_validate),
};

unsafe extern "C" fn nft_meta_bridge_select_ops(
    _ctx: *const nft_ctx,
    tb: *const *const nlattr,
) -> *const nft_expr_ops {
    if (*tb.add(NFTA_META_KEY as usize)).is_null() { return ERR_PTR(-EINVAL); }
    if !(*tb.add(NFTA_META_DREG as usize)).is_null() && !(*tb.add(NFTA_META_SREG as usize)).is_null() {
        return ERR_PTR(-EINVAL);
    }
    if !(*tb.add(NFTA_META_DREG as usize)).is_null() { return &nft_meta_bridge_get_ops; }
    if !(*tb.add(NFTA_META_SREG as usize)).is_null() { return &nft_meta_bridge_set_ops; }
    ERR_PTR(-EINVAL)
}

unsafe extern "C" fn nft_meta_bridge_module_init() -> i32 {
    nft_register_expr(&mut nft_meta_bridge_type)
}

unsafe extern "C" fn nft_meta_bridge_module_exit() {
    nft_unregister_expr(&mut nft_meta_bridge_type);
}

// module_init(nft_meta_bridge_module_init);
// module_exit(nft_meta_bridge_module_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("wenxu <wenxu@ucloud.cn>");
// MODULE_ALIAS_NFT_AF_EXPR(AF_BRIDGE, "meta");
// MODULE_DESCRIPTION("Support for bridge dedicated meta key");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
