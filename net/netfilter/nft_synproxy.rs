// SPDX-License-Identifier: GPL-2.0
// C dependencies are supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct nft_synproxy {
    pub info: nf_synproxy_info,
}

static NFT_SYNPROXY_POLICY: [nla_policy; NFTA_SYNPROXY_MAX as usize + 1] = [
    /* [NFTA_SYNPROXY_MSS] = { .type = NLA_U16 } */
    /* [NFTA_SYNPROXY_WSCALE] = NLA_POLICY_MAX(NLA_U8, TCP_MAX_WSCALE) */
    /* [NFTA_SYNPROXY_FLAGS] = NLA_POLICY_MASK(NLA_BE32, NF_SYNPROXY_OPT_MASK) */
];

unsafe fn nft_synproxy_tcp_options(
    opts: *mut synproxy_options,
    tcp: *const tcphdr,
    snet: *mut synproxy_net,
    info: *mut nf_synproxy_info,
) {
    this_cpu_inc((*snet).stats.as_ref().unwrap().syn_received);
    if (*tcp).ece != 0 && (*tcp).cwr != 0 {
        (*opts).options |= NF_SYNPROXY_OPT_ECN;
    }

    (*opts).options &= (*info).options;
    (*opts).mss_encode = (*opts).mss_option;
    (*opts).mss_option = (*info).mss;
    if (*opts).options & NF_SYNPROXY_OPT_TIMESTAMP != 0 {
        synproxy_init_timestamp_cookie(info, opts);
    } else {
        (*opts).options &= !(NF_SYNPROXY_OPT_WSCALE |
            NF_SYNPROXY_OPT_SACK_PERM |
            NF_SYNPROXY_OPT_ECN);
    }
}

unsafe fn nft_synproxy_eval_v4(
    priv_: *const nft_synproxy,
    regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
    tcp: *const tcphdr,
    _tcph: *mut tcphdr,
    opts: *mut synproxy_options,
) {
    let info = core::ptr::read_volatile(&(*priv_).info);
    let net = nft_net(pkt);
    let snet = synproxy_pernet(net);
    let skb = (*pkt).skb;

    if (*tcp).syn != 0 {
        // Initial SYN from client
        nft_synproxy_tcp_options(opts, tcp, snet, &info as *const _ as *mut _);
        synproxy_send_client_synack(net, skb, tcp, opts);
        consume_skb(skb);
        (*regs).verdict.code = NF_STOLEN;
    } else if (*tcp).ack != 0 {
        // ACK from client
        if synproxy_recv_client_ack(net, skb, tcp, opts, ntohl((*tcp).seq)) != 0 {
            consume_skb(skb);
            (*regs).verdict.code = NF_STOLEN;
        } else {
            (*regs).verdict.code = NF_DROP;
        }
    }
}

// Preserved from #if IS_ENABLED(CONFIG_NF_TABLES_IPV6).
#[cfg(feature = "CONFIG_NF_TABLES_IPV6")]
unsafe fn nft_synproxy_eval_v6(
    priv_: *const nft_synproxy,
    regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
    tcp: *const tcphdr,
    _tcph: *mut tcphdr,
    opts: *mut synproxy_options,
) {
    let info = core::ptr::read_volatile(&(*priv_).info);
    let net = nft_net(pkt);
    let snet = synproxy_pernet(net);
    let skb = (*pkt).skb;

    if (*tcp).syn != 0 {
        // Initial SYN from client
        nft_synproxy_tcp_options(opts, tcp, snet, &info as *const _ as *mut _);
        synproxy_send_client_synack_ipv6(net, skb, tcp, opts);
        consume_skb(skb);
        (*regs).verdict.code = NF_STOLEN;
    } else if (*tcp).ack != 0 {
        // ACK from client
        if synproxy_recv_client_ack_ipv6(net, skb, tcp, opts, ntohl((*tcp).seq)) != 0 {
            consume_skb(skb);
            (*regs).verdict.code = NF_STOLEN;
        } else {
            (*regs).verdict.code = NF_DROP;
        }
    }
}

unsafe fn nft_synproxy_do_eval(
    priv_: *const nft_synproxy,
    regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
) {
    let mut opts: synproxy_options = core::mem::zeroed();
    let skb = (*pkt).skb;
    let thoff = nft_thoff(pkt);
    let mut _tcph: tcphdr = core::mem::zeroed();

    if (*pkt).tprot != IPPROTO_TCP {
        (*regs).verdict.code = NFT_BREAK;
        return;
    }
    if nf_ip_checksum(skb, nft_hook(pkt), thoff, IPPROTO_TCP) != 0 {
        (*regs).verdict.code = NF_DROP;
        return;
    }
    let tcp = skb_header_pointer(skb, thoff, core::mem::size_of::<tcphdr>(),
                                 &mut _tcph as *mut _ as *mut core::ffi::c_void)
        as *const tcphdr;
    if tcp.is_null() {
        (*regs).verdict.code = NF_DROP;
        return;
    }
    if synproxy_parse_options(skb, thoff, tcp, &mut opts) == 0 {
        (*regs).verdict.code = NF_DROP;
        return;
    }

    match (*skb).protocol {
        x if x == htons(ETH_P_IP) => {
            nft_synproxy_eval_v4(priv_, regs, pkt, tcp, &mut _tcph, &mut opts);
            return;
        }
        #[cfg(feature = "CONFIG_NF_TABLES_IPV6")]
        x if x == htons(ETH_P_IPV6) => {
            nft_synproxy_eval_v6(priv_, regs, pkt, tcp, &mut _tcph, &mut opts);
            return;
        }
        _ => {}
    }
    (*regs).verdict.code = NFT_BREAK;
}

unsafe fn nft_synproxy_do_init(
    ctx: *const nft_ctx,
    tb: *const *const nlattr,
    priv_: *mut nft_synproxy,
) -> c_int {
    let snet = synproxy_pernet((*ctx).net);
    let mut flags: u32;
    let mut err: c_int;

    if !(*tb.add(NFTA_SYNPROXY_MSS as usize)).is_null() {
        (*priv_).info.mss = ntohs(nla_get_be16(*tb.add(NFTA_SYNPROXY_MSS as usize)));
    }
    if !(*tb.add(NFTA_SYNPROXY_WSCALE as usize)).is_null() {
        (*priv_).info.wscale = nla_get_u8(*tb.add(NFTA_SYNPROXY_WSCALE as usize));
    }
    if !(*tb.add(NFTA_SYNPROXY_FLAGS as usize)).is_null() {
        flags = ntohl(nla_get_be32(*tb.add(NFTA_SYNPROXY_FLAGS as usize)));
        if flags & !NF_SYNPROXY_OPT_MASK != 0 { return -EOPNOTSUPP; }
        (*priv_).info.options = flags;
    }

    err = nf_ct_netns_get((*ctx).net, (*ctx).family);
    if err != 0 { return err; }
    match (*ctx).family {
        NFPROTO_IPV4 => { err = nf_synproxy_ipv4_init(snet, (*ctx).net); if err != 0 { return nf_ct_failure((*ctx).net, (*ctx).family, err); } }
        #[cfg(feature = "CONFIG_NF_TABLES_IPV6")]
        NFPROTO_IPV6 => { err = nf_synproxy_ipv6_init(snet, (*ctx).net); if err != 0 { return nf_ct_failure((*ctx).net, (*ctx).family, err); } }
        NFPROTO_INET => {
            err = nf_synproxy_ipv4_init(snet, (*ctx).net); if err != 0 { return nf_ct_failure((*ctx).net, (*ctx).family, err); }
            err = nf_synproxy_ipv6_init(snet, (*ctx).net); if err != 0 { nf_synproxy_ipv4_fini(snet, (*ctx).net); return nf_ct_failure((*ctx).net, (*ctx).family, err); }
        }
        _ => {}
    }
    0
}

unsafe fn nf_ct_failure(net: *mut net, family: u8, err: c_int) -> c_int {
    nf_ct_netns_put(net, family);
    err
}

unsafe fn nft_synproxy_do_destroy(ctx: *const nft_ctx) {
    let snet = synproxy_pernet((*ctx).net);
    match (*ctx).family {
        NFPROTO_IPV4 => nf_synproxy_ipv4_fini(snet, (*ctx).net),
        #[cfg(feature = "CONFIG_NF_TABLES_IPV6")]
        NFPROTO_IPV6 => nf_synproxy_ipv6_fini(snet, (*ctx).net),
        NFPROTO_INET => { nf_synproxy_ipv4_fini(snet, (*ctx).net); nf_synproxy_ipv6_fini(snet, (*ctx).net); }
        _ => {}
    }
    nf_ct_netns_put((*ctx).net, (*ctx).family);
}

unsafe fn nft_synproxy_do_dump(skb: *mut sk_buff, priv_: *mut nft_synproxy) -> c_int {
    if nla_put_be16(skb, NFTA_SYNPROXY_MSS, htons((*priv_).info.mss)) != 0 ||
       nla_put_u8(skb, NFTA_SYNPROXY_WSCALE, (*priv_).info.wscale) != 0 ||
       nla_put_be32(skb, NFTA_SYNPROXY_FLAGS, htonl((*priv_).info.options)) != 0 { return -1; }
    0
}

unsafe fn nft_synproxy_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    nft_synproxy_do_eval(nft_expr_priv(expr), regs, pkt);
}
unsafe fn nft_synproxy_validate(ctx: *const nft_ctx, _expr: *const nft_expr) -> c_int {
    if (*ctx).family != NFPROTO_IPV4 && (*ctx).family != NFPROTO_IPV6 && (*ctx).family != NFPROTO_INET { return -EOPNOTSUPP; }
    nft_chain_validate_hooks((*ctx).chain, (1 << NF_INET_LOCAL_IN) | (1 << NF_INET_FORWARD))
}
unsafe fn nft_synproxy_init(ctx: *const nft_ctx, _expr: *const nft_expr, tb: *const *const nlattr) -> c_int { nft_synproxy_do_init(ctx, tb, nft_expr_priv(_expr)) }
unsafe fn nft_synproxy_destroy(ctx: *const nft_ctx, _expr: *const nft_expr) { nft_synproxy_do_destroy(ctx); }
unsafe fn nft_synproxy_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> c_int { nft_synproxy_do_dump(skb, nft_expr_priv(expr)) }

// The following registration objects preserve the C callbacks and module registration topology.
static mut NFT_SYNPROXY_TYPE: nft_expr_type = nft_expr_type { ops: &NFT_SYNPROXY_OPS, name: "synproxy", owner: THIS_MODULE, policy: &NFT_SYNPROXY_POLICY, maxattr: NFTA_SYNPROXY_MAX };
static NFT_SYNPROXY_OPS: nft_expr_ops = nft_expr_ops { eval: nft_synproxy_eval, size: NFT_EXPR_SIZE(core::mem::size_of::<nft_synproxy>()), init: nft_synproxy_init, destroy: nft_synproxy_destroy, dump: nft_synproxy_dump, type_: unsafe { &NFT_SYNPROXY_TYPE }, validate: nft_synproxy_validate };

unsafe fn nft_synproxy_obj_init(ctx: *const nft_ctx, tb: *const *const nlattr, obj: *mut nft_object) -> c_int { nft_synproxy_do_init(ctx, tb, nft_obj_data(obj)) }
unsafe fn nft_synproxy_obj_destroy(ctx: *const nft_ctx, _obj: *mut nft_object) { nft_synproxy_do_destroy(ctx); }
unsafe fn nft_synproxy_obj_dump(skb: *mut sk_buff, obj: *mut nft_object, _reset: bool) -> c_int { nft_synproxy_do_dump(skb, nft_obj_data(obj)) }
unsafe fn nft_synproxy_obj_eval(obj: *mut nft_object, regs: *mut nft_regs, pkt: *const nft_pktinfo) { nft_synproxy_do_eval(nft_obj_data(obj), regs, pkt); }
unsafe fn nft_synproxy_obj_update(obj: *mut nft_object, newobj: *mut nft_object) { (*nft_obj_data(obj)).info = core::ptr::read_volatile(&(*nft_obj_data(newobj)).info); }

static NFT_SYNPROXY_OBJ_OPS: nft_object_ops = nft_object_ops {
    type_: unsafe { &NFT_SYNPROXY_OBJ_TYPE }, size: core::mem::size_of::<nft_synproxy>(),
    init: nft_synproxy_obj_init, destroy: nft_synproxy_obj_destroy, dump: nft_synproxy_obj_dump,
    eval: nft_synproxy_obj_eval, update: nft_synproxy_obj_update,
};
static NFT_SYNPROXY_OBJ_TYPE: nft_object_type = nft_object_type {
    type_: NFT_OBJECT_SYNPROXY, ops: &NFT_SYNPROXY_OBJ_OPS, maxattr: NFTA_SYNPROXY_MAX,
    policy: &NFT_SYNPROXY_POLICY, owner: THIS_MODULE,
};

unsafe fn nft_synproxy_module_init() -> c_int {
    let mut err = nft_register_obj(&NFT_SYNPROXY_OBJ_TYPE);
    if err < 0 { return err; }
    err = nft_register_expr(&NFT_SYNPROXY_TYPE);
    if err < 0 { nft_unregister_obj(&NFT_SYNPROXY_OBJ_TYPE); return err; }
    0
}
unsafe fn nft_synproxy_module_exit() { nft_unregister_expr(&NFT_SYNPROXY_TYPE); nft_unregister_obj(&NFT_SYNPROXY_OBJ_TYPE); }

// module_init(nft_synproxy_module_init); module_exit(nft_synproxy_module_exit);
// MODULE_LICENSE("GPL"); MODULE_AUTHOR("Fernando Fernandez <ffmancera@riseup.net>");
// MODULE_ALIAS_NFT_EXPR("synproxy"); MODULE_ALIAS_NFT_OBJ(NFT_OBJECT_SYNPROXY);
// MODULE_DESCRIPTION("nftables SYNPROXY expression support");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
