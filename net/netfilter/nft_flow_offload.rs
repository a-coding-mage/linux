// SPDX-License-Identifier: GPL-2.0-only
// External Linux kernel declarations and build-time configuration are supplied
// by the surrounding translation unit.

#[repr(C)]
struct nft_flow_offload {
    flowtable: *mut nft_flowtable,
}

unsafe fn nft_flow_offload_skip(skb: *mut sk_buff, family: libc::c_int) -> bool {
    if !skb_sec_path(skb).is_null() {
        return true;
    }

    if family == NFPROTO_IPV4 {
        let opt: *const ip_options = &(*IPCB(skb)).opt;
        if (*opt).optlen != 0 {
            return true;
        }
    }

    false
}

unsafe fn flow_offload_ct_tcp(ct: *mut nf_conn) {
    /* conntrack will not see all packets, disable tcp window validation. */
    spin_lock_bh(&mut (*ct).lock);
    (*ct).proto.tcp.seen[0].flags |= IP_CT_TCP_FLAG_BE_LIBERAL;
    (*ct).proto.tcp.seen[1].flags |= IP_CT_TCP_FLAG_BE_LIBERAL;
    spin_unlock_bh(&mut (*ct).lock);
}

unsafe fn nft_flow_offload_eval(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
) {
    let priv_: *mut nft_flow_offload = nft_expr_priv(expr);
    let flowtable: *mut nf_flowtable = &mut (*(*priv_).flowtable).data;
    let mut _tcph: tcphdr = core::mem::zeroed();
    let mut tcph: *mut tcphdr = core::ptr::null_mut();
    let mut route: nf_flow_route = core::mem::zeroed();
    let mut ctinfo: ip_conntrack_info = core::mem::zeroed();
    let mut flow: *mut flow_offload;
    let mut dir: ip_conntrack_dir;
    let mut ct: *mut nf_conn;
    let mut ret: libc::c_int;

    if nft_flow_offload_skip((*pkt).skb, nft_pf(pkt)) {
        (*regs).verdict.code = NFT_BREAK;
        return;
    }

    ct = nf_ct_get((*pkt).skb, &mut ctinfo);
    if ct.is_null() {
        (*regs).verdict.code = NFT_BREAK;
        return;
    }

    match (*ct).tuplehash[IP_CT_DIR_ORIGINAL as usize].tuple.dst.protonum {
        IPPROTO_TCP => {
            tcph = skb_header_pointer((*pkt).skb, nft_thoff(pkt), core::mem::size_of::<tcphdr>(), &mut _tcph) as *mut tcphdr;
            if tcph.is_null() || (*tcph).fin != 0 || (*tcph).rst != 0 || !nf_conntrack_tcp_established(ct) {
                (*regs).verdict.code = NFT_BREAK;
                return;
            }
        }
        IPPROTO_UDP => {}
        #[cfg(CONFIG_NF_CT_PROTO_GRE)]
        IPPROTO_GRE => {
            let tuple: *mut nf_conntrack_tuple = &mut (*ct).tuplehash[IP_CT_DIR_ORIGINAL as usize].tuple;
            if (*ct).status & IPS_NAT_MASK != 0 || (*tuple).src.u.gre.key != 0 || (*tuple).dst.u.gre.key != 0 {
                (*regs).verdict.code = NFT_BREAK;
                return;
            }
            /* No support for GRE v1 */
        }
        _ => {
            (*regs).verdict.code = NFT_BREAK;
            return;
        }
    }

    if nf_ct_ext_exist(ct, NF_CT_EXT_HELPER) || (*ct).status & (IPS_SEQ_ADJUST | IPS_NAT_CLASH) != 0 || !nf_ct_is_confirmed(ct) {
        (*regs).verdict.code = NFT_BREAK;
        return;
    }
    if test_and_set_bit(IPS_OFFLOAD_BIT, &mut (*ct).status) != 0 {
        (*regs).verdict.code = NFT_BREAK;
        return;
    }

    dir = CTINFO2DIR(ctinfo);
    if nft_flow_route(pkt, ct, &mut route, dir, (*priv_).flowtable) < 0 {
        (*ct).status &= !(1 << IPS_OFFLOAD_BIT);
        (*regs).verdict.code = NFT_BREAK;
        return;
    }
    flow = flow_offload_alloc(ct);
    if flow.is_null() {
        dst_release(route.tuple[dir as usize].dst);
        dst_release(route.tuple[(!dir) as usize].dst);
        (*ct).status &= !(1 << IPS_OFFLOAD_BIT);
        (*regs).verdict.code = NFT_BREAK;
        return;
    }

    flow_offload_route_init(flow, &mut route);
    if !tcph.is_null() {
        flow_offload_ct_tcp(ct);
    }
    __set_bit(NF_FLOW_HW_BIDIRECTIONAL, &mut (*flow).flags);
    ret = flow_offload_add(flowtable, flow);
    if ret < 0 {
        flow_offload_free(flow);
        dst_release(route.tuple[dir as usize].dst);
        dst_release(route.tuple[(!dir) as usize].dst);
        (*ct).status &= !(1 << IPS_OFFLOAD_BIT);
        (*regs).verdict.code = NFT_BREAK;
    }
}

unsafe fn nft_flow_offload_validate(ctx: *const nft_ctx, _expr: *const nft_expr) -> libc::c_int {
    let hook_mask: libc::c_uint = 1 << NF_INET_FORWARD;
    if (*ctx).family != NFPROTO_IPV4 && (*ctx).family != NFPROTO_IPV6 && (*ctx).family != NFPROTO_INET {
        return -EOPNOTSUPP;
    }
    nft_chain_validate_hooks((*ctx).chain, hook_mask)
}

static nft_flow_offload_policy: [nla_policy; NFTA_FLOW_MAX as usize + 1] = {
    let mut p = [nla_policy { type_: 0, len: 0 }; NFTA_FLOW_MAX as usize + 1];
    p[NFTA_FLOW_TABLE_NAME as usize] = nla_policy { type_: NLA_STRING, len: NFT_NAME_MAXLEN - 1 };
    p
};

unsafe fn nft_flow_offload_init(ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> libc::c_int {
    let priv_: *mut nft_flow_offload = nft_expr_priv(expr);
    let genmask: u8 = nft_genmask_next((*ctx).net);
    if (*tb.add(NFTA_FLOW_TABLE_NAME as usize)).is_null() { return -EINVAL; }
    let flowtable = nft_flowtable_lookup((*ctx).net, (*ctx).table, *tb.add(NFTA_FLOW_TABLE_NAME as usize), genmask);
    if IS_ERR(flowtable) { return PTR_ERR(flowtable); }
    if !nft_use_inc(&mut (*flowtable).use_) { return -EMFILE; }
    (*priv_).flowtable = flowtable;
    nf_ct_netns_get((*ctx).net, (*ctx).family)
}

unsafe fn nft_flow_offload_deactivate(ctx: *const nft_ctx, expr: *const nft_expr, phase: nft_trans_phase) {
    let priv_: *mut nft_flow_offload = nft_expr_priv(expr);
    nf_tables_deactivate_flowtable(ctx, (*priv_).flowtable, phase);
}

unsafe fn nft_flow_offload_activate(_ctx: *const nft_ctx, expr: *const nft_expr) {
    let priv_: *mut nft_flow_offload = nft_expr_priv(expr);
    nft_use_inc_restore(&mut (*(*priv_).flowtable).use_);
}

unsafe fn nft_flow_offload_destroy(ctx: *const nft_ctx, _expr: *const nft_expr) {
    nf_ct_netns_put((*ctx).net, (*ctx).family);
}

unsafe fn nft_flow_offload_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> libc::c_int {
    let priv_: *mut nft_flow_offload = nft_expr_priv(expr);
    if nla_put_string(skb, NFTA_FLOW_TABLE_NAME, (*priv_).flowtable.as_ref().unwrap().name.as_ptr()) != 0 { return -1; }
    0
}

static mut nft_flow_offload_type: nft_expr_type = core::mem::zeroed();
static nft_flow_offload_ops: nft_expr_ops = nft_expr_ops {
    type_: unsafe { &mut nft_flow_offload_type }, size: NFT_EXPR_SIZE(core::mem::size_of::<nft_flow_offload>()), eval: Some(nft_flow_offload_eval), init: Some(nft_flow_offload_init), activate: Some(nft_flow_offload_activate), deactivate: Some(nft_flow_offload_deactivate), destroy: Some(nft_flow_offload_destroy), validate: Some(nft_flow_offload_validate), dump: Some(nft_flow_offload_dump),
};

static mut nft_flow_offload_type: nft_expr_type = nft_expr_type {
    name: b"flow_offload\0".as_ptr() as *const _, ops: unsafe { &nft_flow_offload_ops }, policy: nft_flow_offload_policy.as_ptr(), maxattr: NFTA_FLOW_MAX, owner: THIS_MODULE,
};

unsafe fn flow_offload_netdev_event(_this: *mut notifier_block, event: libc::c_ulong, ptr: *mut libc::c_void) -> libc::c_int {
    let dev = netdev_notifier_info_to_dev(ptr);
    if event != NETDEV_DOWN { return NOTIFY_DONE; }
    nf_flow_table_cleanup(dev);
    NOTIFY_DONE
}

static mut flow_offload_netdev_notifier: notifier_block = notifier_block { notifier_call: Some(flow_offload_netdev_event) };

unsafe fn nft_flow_offload_module_init() -> libc::c_int {
    let mut err = register_netdevice_notifier(&mut flow_offload_netdev_notifier);
    if err != 0 { return err; }
    err = nft_register_expr(&mut nft_flow_offload_type);
    if err < 0 {
        unregister_netdevice_notifier(&mut flow_offload_netdev_notifier);
        return err;
    }
    0
}

unsafe fn nft_flow_offload_module_exit() {
    nft_unregister_expr(&mut nft_flow_offload_type);
    unregister_netdevice_notifier(&mut flow_offload_netdev_notifier);
}

// module_init(nft_flow_offload_module_init);
// module_exit(nft_flow_offload_module_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Pablo Neira Ayuso <pablo@netfilter.org>");
// MODULE_ALIAS_NFT_EXPR("flow_offload");
// MODULE_DESCRIPTION("nftables hardware flow offload module");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
