// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 Pablo Neira Ayuso <pablo@netfilter.org>
 */

// Kernel headers and symbols referenced below are supplied by the surrounding
// translation unit and are intentionally not reimplemented here.

unsafe fn nft_reject_br_push_etherhdr(
    oldskb: *mut sk_buff,
    nskb: *mut sk_buff,
) {
    let eth: *mut ethhdr;

    eth = skb_push(nskb, ETH_HLEN);
    skb_reset_mac_header(nskb);
    ether_addr_copy((*eth).h_source.as_mut_ptr(), eth_hdr(oldskb).as_ref().unwrap().h_dest.as_ptr());
    ether_addr_copy((*eth).h_dest.as_mut_ptr(), eth_hdr(oldskb).as_ref().unwrap().h_source.as_ptr());
    (*eth).h_proto = eth_hdr(oldskb).as_ref().unwrap().h_proto;
    skb_pull(nskb, ETH_HLEN);

    if skb_vlan_tag_present(oldskb) {
        let vid: u16 = skb_vlan_tag_get(oldskb);

        __vlan_hwaccel_put_tag(nskb, (*oldskb).vlan_proto, vid);
    }
}

/* We cannot use oldskb->dev, it can be either bridge device (NF_BRIDGE INPUT)
 * or the bridge port (NF_BRIDGE PREROUTING).
 */
unsafe fn nft_reject_br_send_v4_tcp_reset(
    net: *mut net,
    oldskb: *mut sk_buff,
    dev: *const net_device,
    hook: i32,
) {
    let nskb: *mut sk_buff;

    nskb = nf_reject_skb_v4_tcp_reset(net, oldskb, core::ptr::null_mut(), hook);
    if nskb.is_null() {
        return;
    }

    nft_reject_br_push_etherhdr(oldskb, nskb);

    br_forward(br_port_get_rcu(dev), nskb, false, true);
}

unsafe fn nft_reject_br_send_v4_unreach(
    net: *mut net,
    oldskb: *mut sk_buff,
    dev: *const net_device,
    hook: i32,
    code: u8,
) {
    let nskb: *mut sk_buff;

    nskb = nf_reject_skb_v4_unreach(net, oldskb, core::ptr::null_mut(), hook, code);
    if nskb.is_null() {
        return;
    }

    nft_reject_br_push_etherhdr(oldskb, nskb);

    br_forward(br_port_get_rcu(dev), nskb, false, true);
}

unsafe fn nft_reject_br_send_v6_tcp_reset(
    net: *mut net,
    oldskb: *mut sk_buff,
    dev: *const net_device,
    hook: i32,
) {
    let nskb: *mut sk_buff;

    nskb = nf_reject_skb_v6_tcp_reset(net, oldskb, core::ptr::null_mut(), hook);
    if nskb.is_null() {
        return;
    }

    nft_reject_br_push_etherhdr(oldskb, nskb);

    br_forward(br_port_get_rcu(dev), nskb, false, true);
}

unsafe fn nft_reject_br_send_v6_unreach(
    net: *mut net,
    oldskb: *mut sk_buff,
    dev: *const net_device,
    hook: i32,
    code: u8,
) {
    let nskb: *mut sk_buff;

    nskb = nf_reject_skb_v6_unreach(net, oldskb, core::ptr::null_mut(), hook, code);
    if nskb.is_null() {
        return;
    }

    nft_reject_br_push_etherhdr(oldskb, nskb);

    br_forward(br_port_get_rcu(dev), nskb, false, true);
}

unsafe fn nft_reject_bridge_eval(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
) {
    let priv_: *mut nft_reject = nft_expr_priv(expr);
    let dest: *const u8 = (*eth_hdr((*pkt).skb)).h_dest.as_ptr();

    if is_broadcast_ether_addr(dest) || is_multicast_ether_addr(dest) {
        (*regs).verdict.code = NF_DROP;
        return;
    }

    match (*eth_hdr((*pkt).skb)).h_proto {
        x if x == htons(ETH_P_IP) => match (*priv_).type_ {
            NFT_REJECT_ICMP_UNREACH => nft_reject_br_send_v4_unreach(nft_net(pkt), (*pkt).skb, nft_in(pkt), nft_hook(pkt), (*priv_).icmp_code),
            NFT_REJECT_TCP_RST => nft_reject_br_send_v4_tcp_reset(nft_net(pkt), (*pkt).skb, nft_in(pkt), nft_hook(pkt)),
            NFT_REJECT_ICMPX_UNREACH => nft_reject_br_send_v4_unreach(nft_net(pkt), (*pkt).skb, nft_in(pkt), nft_hook(pkt), nft_reject_icmp_code((*priv_).icmp_code)),
            _ => (),
        },
        x if x == htons(ETH_P_IPV6) => match (*priv_).type_ {
            NFT_REJECT_ICMP_UNREACH => nft_reject_br_send_v6_unreach(nft_net(pkt), (*pkt).skb, nft_in(pkt), nft_hook(pkt), (*priv_).icmp_code),
            NFT_REJECT_TCP_RST => nft_reject_br_send_v6_tcp_reset(nft_net(pkt), (*pkt).skb, nft_in(pkt), nft_hook(pkt)),
            NFT_REJECT_ICMPX_UNREACH => nft_reject_br_send_v6_unreach(nft_net(pkt), (*pkt).skb, nft_in(pkt), nft_hook(pkt), nft_reject_icmpv6_code((*priv_).icmp_code)),
            _ => (),
        },
        _ => {
            /* No explicit way to reject this protocol, drop it. */
        },
    }
    (*regs).verdict.code = NF_DROP;
}

unsafe fn nft_reject_bridge_validate(ctx: *const nft_ctx, _expr: *const nft_expr) -> i32 {
    nft_chain_validate_hooks((*ctx).chain, (1 << NF_BR_PRE_ROUTING) | (1 << NF_BR_LOCAL_IN))
}

static mut nft_reject_bridge_type: nft_expr_type = nft_expr_type {
    family: NFPROTO_BRIDGE,
    name: "reject",
    ops: &nft_reject_bridge_ops,
    policy: nft_reject_policy,
    maxattr: NFTA_REJECT_MAX,
    owner: THIS_MODULE,
};

static nft_reject_bridge_ops: nft_expr_ops = nft_expr_ops {
    type_: unsafe { &nft_reject_bridge_type },
    size: NFT_EXPR_SIZE(core::mem::size_of::<nft_reject>()),
    eval: nft_reject_bridge_eval,
    init: nft_reject_init,
    dump: nft_reject_dump,
    validate: nft_reject_bridge_validate,
};

unsafe fn nft_reject_bridge_module_init() -> i32 {
    nft_register_expr(&mut nft_reject_bridge_type)
}

unsafe fn nft_reject_bridge_module_exit() {
    nft_unregister_expr(&mut nft_reject_bridge_type);
}

// module_init(nft_reject_bridge_module_init);
// module_exit(nft_reject_bridge_module_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Pablo Neira Ayuso <pablo@netfilter.org>");
// MODULE_ALIAS_NFT_AF_EXPR(AF_BRIDGE, "reject");
// MODULE_DESCRIPTION("Reject packets from bridge via nftables");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
