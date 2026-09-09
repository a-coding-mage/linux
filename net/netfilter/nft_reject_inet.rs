// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 Patrick McHardy <kaber@trash.net>
 */

// Linux kernel and Netfilter declarations are supplied by the surrounding
// translation unit.

unsafe fn nft_reject_inet_eval(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
) {
    let priv_: *mut nft_reject = nft_expr_priv(expr);

    match nft_pf(pkt) {
        NFPROTO_IPV4 => match (*priv_).type_ {
            NFT_REJECT_ICMP_UNREACH => {
                nf_send_unreach((*pkt).skb, (*priv_).icmp_code, nft_hook(pkt));
            }
            NFT_REJECT_TCP_RST => {
                nf_send_reset(nft_net(pkt), nft_sk(pkt), (*pkt).skb, nft_hook(pkt));
            }
            NFT_REJECT_ICMPX_UNREACH => {
                nf_send_unreach(
                    (*pkt).skb,
                    nft_reject_icmp_code((*priv_).icmp_code),
                    nft_hook(pkt),
                );
            }
            _ => {}
        },
        NFPROTO_IPV6 => match (*priv_).type_ {
            NFT_REJECT_ICMP_UNREACH => {
                nf_send_unreach6(
                    nft_net(pkt),
                    (*pkt).skb,
                    (*priv_).icmp_code,
                    nft_hook(pkt),
                );
            }
            NFT_REJECT_TCP_RST => {
                nf_send_reset6(nft_net(pkt), nft_sk(pkt), (*pkt).skb, nft_hook(pkt));
            }
            NFT_REJECT_ICMPX_UNREACH => {
                nf_send_unreach6(
                    nft_net(pkt),
                    (*pkt).skb,
                    nft_reject_icmpv6_code((*priv_).icmp_code),
                    nft_hook(pkt),
                );
            }
            _ => {}
        },
        _ => {}
    }

    (*regs).verdict.code = NF_DROP;
}

unsafe fn nft_reject_inet_validate(
    ctx: *const nft_ctx,
    _expr: *const nft_expr,
) -> i32 {
    nft_chain_validate_hooks(
        (*ctx).chain,
        (1 << NF_INET_LOCAL_IN)
            | (1 << NF_INET_FORWARD)
            | (1 << NF_INET_LOCAL_OUT)
            | (1 << NF_INET_PRE_ROUTING)
            | (1 << NF_INET_INGRESS),
    )
}

static mut NFT_REJECT_INET_OPS: nft_expr_ops = nft_expr_ops {
    type_: &mut NFT_REJECT_INET_TYPE,
    size: NFT_EXPR_SIZE(core::mem::size_of::<nft_reject>()),
    eval: Some(nft_reject_inet_eval),
    init: Some(nft_reject_init),
    dump: Some(nft_reject_dump),
    validate: Some(nft_reject_inet_validate),
};

static mut NFT_REJECT_INET_TYPE: nft_expr_type = nft_expr_type {
    family: NFPROTO_INET,
    name: "reject",
    ops: &mut NFT_REJECT_INET_OPS,
    policy: nft_reject_policy,
    maxattr: NFTA_REJECT_MAX,
    owner: THIS_MODULE,
};

unsafe fn nft_reject_inet_module_init() -> i32 {
    nft_register_expr(&mut NFT_REJECT_INET_TYPE)
}

unsafe fn nft_reject_inet_module_exit() {
    nft_unregister_expr(&mut NFT_REJECT_INET_TYPE);
}

// module_init(nft_reject_inet_module_init);
// module_exit(nft_reject_inet_module_exit);

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Patrick McHardy <kaber@trash.net>");
// MODULE_ALIAS_NFT_AF_EXPR(1, "reject");
// MODULE_DESCRIPTION("Netfilter nftables reject inet support");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
