// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2015 Pablo Neira Ayuso <pablo@netfilter.org>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct nft_fwd_netdev {
    pub sreg_dev: u8,
}

unsafe fn nft_fwd_netdev_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    let priv_ = nft_expr_priv(expr) as *mut nft_fwd_netdev;
    let oif = (*regs).data[(*priv_).sreg_dev as usize];
    let skb = (*pkt).skb;

    (*skb).skb_iif = (*(*skb).dev).ifindex;
    skb_set_redirected(skb, nft_hook(pkt) == NF_NETDEV_INGRESS);
    nf_fwd_netdev_egress(pkt, oif);
    (*regs).verdict.code = NF_STOLEN;
}

static nft_fwd_netdev_policy: [nla_policy; NFTA_FWD_MAX as usize + 1] = [
    /* NFTA_FWD_SREG_DEV */ nla_policy { type_: NLA_U32 },
    /* NFTA_FWD_SREG_ADDR */ nla_policy { type_: NLA_U32 },
    /* NFTA_FWD_NFPROTO */ nla_policy { type_: NLA_BE32, max: 255 },
];

unsafe fn nft_fwd_netdev_init(ctx: *const nft_ctx, expr: *const nft_expr,
                              tb: *const *const nlattr) -> i32 {
    let priv_ = nft_expr_priv(expr) as *mut nft_fwd_netdev;
    if (*tb.add(NFTA_FWD_SREG_DEV as usize)).is_null() { return -EINVAL; }
    nft_parse_register_load(ctx, *tb.add(NFTA_FWD_SREG_DEV as usize),
                            &mut (*priv_).sreg_dev as *mut u8 as *mut _,
                            core::mem::size_of::<i32>())
}

unsafe fn nft_fwd_netdev_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> i32 {
    let priv_ = nft_expr_priv(expr) as *mut nft_fwd_netdev;
    if nft_dump_register(skb, NFTA_FWD_SREG_DEV, (*priv_).sreg_dev) != 0 { return -1; }
    0
}

unsafe fn nft_fwd_netdev_offload(ctx: *mut nft_offload_ctx, flow: *mut nft_flow_rule,
                                 expr: *const nft_expr) -> i32 {
    let priv_ = nft_expr_priv(expr) as *const nft_fwd_netdev;
    let oif = (*ctx).regs[(*priv_).sreg_dev as usize].data.data[0];
    nft_fwd_dup_netdev_offload(ctx, flow, FLOW_ACTION_REDIRECT, oif)
}

unsafe fn nft_fwd_netdev_offload_action(_expr: *const nft_expr) -> bool { true }

#[repr(C)]
pub struct nft_fwd_neigh {
    pub sreg_dev: u8,
    pub sreg_addr: u8,
    pub nfproto: u8,
}

unsafe fn nft_fwd_neigh_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    let priv_ = nft_expr_priv(expr) as *mut nft_fwd_neigh;
    let addr = &mut (*regs).data[(*priv_).sreg_addr as usize] as *mut _ as *mut core::ffi::c_void;
    let oif = (*regs).data[(*priv_).sreg_dev as usize];
    let mut verdict = NF_STOLEN;
    let mut skb = (*pkt).skb;
    let nhoff = skb_network_offset(skb);
    let mut dev: *mut net_device;
    let hh_len: u32;
    let neigh_table: i32;

    match (*priv_).nfproto {
        NFPROTO_IPV4 => {
            if (*skb).protocol != htons(ETH_P_IP) { verdict = NFT_BREAK; goto_out(regs, verdict); return; }
            if skb_ensure_writable(skb, nhoff + core::mem::size_of::<iphdr>()) != 0 { verdict = NF_DROP; goto_out(regs, verdict); return; }
            let iph = ip_hdr(skb);
            if (*iph).ttl <= 1 { verdict = NF_DROP; goto_out(regs, verdict); return; }
            ip_decrease_ttl(iph); neigh_table = NEIGH_ARP_TABLE;
        }
        NFPROTO_IPV6 => {
            if (*skb).protocol != htons(ETH_P_IPV6) { verdict = NFT_BREAK; goto_out(regs, verdict); return; }
            if skb_ensure_writable(skb, nhoff + core::mem::size_of::<ipv6hdr>()) != 0 { verdict = NF_DROP; goto_out(regs, verdict); return; }
            let ip6h = ipv6_hdr(skb);
            if (*ip6h).hop_limit <= 1 { verdict = NF_DROP; goto_out(regs, verdict); return; }
            (*ip6h).hop_limit -= 1; neigh_table = NEIGH_ND_TABLE;
        }
        _ => { verdict = NFT_BREAK; goto_out(regs, verdict); return; }
    }
    dev = dev_get_by_index_rcu(nft_net(pkt), oif);
    if dev.is_null() { verdict = NF_DROP; goto_out(regs, verdict); return; }
    local_bh_disable();
    if nf_dev_xmit_recursion() != 0 { local_bh_enable(); verdict = NF_DROP; goto_out(regs, verdict); return; }
    hh_len = LL_RESERVED_SPACE(dev);
    if skb_headroom(skb) < hh_len && !(*dev).header_ops.is_null() {
        skb = skb_expand_head(skb, hh_len);
        if skb.is_null() { local_bh_enable(); goto_out(regs, verdict); return; }
    }
    (*skb).dev = dev; skb_clear_tstamp(skb);
    nf_dev_xmit_recursion_inc(); neigh_xmit(neigh_table, dev, addr, skb);
    nf_dev_xmit_recursion_dec(); local_bh_enable();
    goto_out(regs, verdict);
}

unsafe fn goto_out(regs: *mut nft_regs, verdict: u32) { (*regs).verdict.code = verdict; }

unsafe fn nft_fwd_neigh_init(ctx: *const nft_ctx, expr: *const nft_expr,
                             tb: *const *const nlattr) -> i32 {
    let priv_ = nft_expr_priv(expr) as *mut nft_fwd_neigh;
    if (*tb.add(NFTA_FWD_SREG_DEV as usize)).is_null() || (*tb.add(NFTA_FWD_SREG_ADDR as usize)).is_null() || (*tb.add(NFTA_FWD_NFPROTO as usize)).is_null() { return -EINVAL; }
    (*priv_).nfproto = ntohl(nla_get_be32(*tb.add(NFTA_FWD_NFPROTO as usize))) as u8;
    let addr_len = match (*priv_).nfproto { NFPROTO_IPV4 => core::mem::size_of::<in_addr>(), NFPROTO_IPV6 => core::mem::size_of::<in6_addr>(), _ => return -EOPNOTSUPP };
    let err = nft_parse_register_load(ctx, *tb.add(NFTA_FWD_SREG_DEV as usize), &mut (*priv_).sreg_dev as *mut u8 as *mut _, core::mem::size_of::<i32>());
    if err < 0 { return err; }
    nft_parse_register_load(ctx, *tb.add(NFTA_FWD_SREG_ADDR as usize), &mut (*priv_).sreg_addr as *mut u8 as *mut _, addr_len)
}

unsafe fn nft_fwd_neigh_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> i32 {
    let p = nft_expr_priv(expr) as *mut nft_fwd_neigh;
    if nft_dump_register(skb, NFTA_FWD_SREG_DEV, (*p).sreg_dev) != 0 || nft_dump_register(skb, NFTA_FWD_SREG_ADDR, (*p).sreg_addr) != 0 || nla_put_be32(skb, NFTA_FWD_NFPROTO, htonl((*p).nfproto as u32)) != 0 { return -1; } 0
}

unsafe fn nft_fwd_validate(ctx: *const nft_ctx, _expr: *const nft_expr) -> i32 { nft_chain_validate_hooks((*ctx).chain, (1 << NF_NETDEV_INGRESS) | (1 << NF_NETDEV_EGRESS)) }

static mut nft_fwd_netdev_type: nft_expr_type = nft_expr_type { family: NFPROTO_NETDEV, name: "fwd", select_ops: nft_fwd_select_ops, policy: nft_fwd_netdev_policy.as_ptr(), maxattr: NFTA_FWD_MAX, owner: THIS_MODULE };

unsafe fn nft_fwd_select_ops(_ctx: *const nft_ctx, tb: *const *const nlattr) -> *const nft_expr_ops {
    if !(*tb.add(NFTA_FWD_SREG_ADDR as usize)).is_null() { return &nft_fwd_neigh_netdev_ops; }
    if !(*tb.add(NFTA_FWD_SREG_DEV as usize)).is_null() { return &nft_fwd_netdev_ops; }
    ERR_PTR(-EOPNOTSUPP)
}

static nft_fwd_neigh_netdev_ops: nft_expr_ops = nft_expr_ops { type_: unsafe { &nft_fwd_netdev_type }, size: NFT_EXPR_SIZE(core::mem::size_of::<nft_fwd_neigh>()), eval: nft_fwd_neigh_eval, init: nft_fwd_neigh_init, dump: nft_fwd_neigh_dump, validate: nft_fwd_validate };
static nft_fwd_netdev_ops: nft_expr_ops = nft_expr_ops { type_: unsafe { &nft_fwd_netdev_type }, size: NFT_EXPR_SIZE(core::mem::size_of::<nft_fwd_netdev>()), eval: nft_fwd_netdev_eval, init: nft_fwd_netdev_init, dump: nft_fwd_netdev_dump, validate: nft_fwd_validate, offload: nft_fwd_netdev_offload, offload_action: nft_fwd_netdev_offload_action };

unsafe fn nft_fwd_netdev_module_init() -> i32 { nft_register_expr(&mut nft_fwd_netdev_type) }
unsafe fn nft_fwd_netdev_module_exit() { nft_unregister_expr(&mut nft_fwd_netdev_type); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
