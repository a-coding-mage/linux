// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2015 Pablo Neira Ayuso <pablo@netfilter.org>
 */

// Dependencies supplied by the surrounding kernel translation are referenced
// here but intentionally not implemented in this isolated translation.

#[repr(C)]
pub struct nft_dup_ipv4 {
    pub sreg_addr: u8,
    pub sreg_dev: u8,
}

unsafe fn nft_dup_ipv4_eval(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
) {
    let priv_ = nft_expr_priv(expr) as *mut nft_dup_ipv4;
    let gw = in_addr {
        s_addr: (*regs).data[(*priv_).sreg_addr as usize] as __be32,
    };
    let oif: i32 = if (*priv_).sreg_dev != 0 {
        (*regs).data[(*priv_).sreg_dev as usize] as i32
    } else {
        -1
    };

    nf_dup_ipv4(nft_net(pkt), (*pkt).skb, nft_hook(pkt), &gw, oif);
}

unsafe fn nft_dup_ipv4_init(
    ctx: *const nft_ctx,
    expr: *const nft_expr,
    tb: *const *const nlattr,
) -> i32 {
    let priv_ = nft_expr_priv(expr) as *mut nft_dup_ipv4;
    let mut err: i32;

    if (*tb.add(NFTA_DUP_SREG_ADDR as usize)).is_null() {
        return -EINVAL;
    }

    err = nft_parse_register_load(
        ctx,
        *tb.add(NFTA_DUP_SREG_ADDR as usize),
        &mut (*priv_).sreg_addr,
        core::mem::size_of::<in_addr>(),
    );
    if err < 0 {
        return err;
    }

    if !(*tb.add(NFTA_DUP_SREG_DEV as usize)).is_null() {
        err = nft_parse_register_load(
            ctx,
            *tb.add(NFTA_DUP_SREG_DEV as usize),
            &mut (*priv_).sreg_dev,
            core::mem::size_of::<i32>(),
        );
    }

    err
}

unsafe fn nft_dup_ipv4_dump(
    skb: *mut sk_buff,
    expr: *const nft_expr,
    _reset: bool,
) -> i32 {
    let priv_ = nft_expr_priv(expr) as *mut nft_dup_ipv4;

    if nft_dump_register(skb, NFTA_DUP_SREG_ADDR, (*priv_).sreg_addr) != 0 {
        return -1;
    }
    if (*priv_).sreg_dev != 0
        && nft_dump_register(skb, NFTA_DUP_SREG_DEV, (*priv_).sreg_dev) != 0
    {
        return -1;
    }

    0
}

static mut nft_dup_ipv4_type: nft_expr_type = nft_expr_type {
    family: NFPROTO_IPV4,
    name: "dup",
    ops: &nft_dup_ipv4_ops,
    policy: nft_dup_ipv4_policy,
    maxattr: NFTA_DUP_MAX,
    owner: THIS_MODULE,
};

static nft_dup_ipv4_ops: nft_expr_ops = nft_expr_ops {
    type_: &nft_dup_ipv4_type,
    size: NFT_EXPR_SIZE(core::mem::size_of::<nft_dup_ipv4>()),
    eval: Some(nft_dup_ipv4_eval),
    init: Some(nft_dup_ipv4_init),
    dump: Some(nft_dup_ipv4_dump),
};

static nft_dup_ipv4_policy: [nla_policy; NFTA_DUP_MAX as usize + 1] = {
    let mut policy = [nla_policy { type_: 0 }; NFTA_DUP_MAX as usize + 1];
    policy[NFTA_DUP_SREG_ADDR as usize] = nla_policy { type_: NLA_U32 };
    policy[NFTA_DUP_SREG_DEV as usize] = nla_policy { type_: NLA_U32 };
    policy
};

unsafe fn nft_dup_ipv4_module_init() -> i32 {
    nft_register_expr(&mut nft_dup_ipv4_type)
}

unsafe fn nft_dup_ipv4_module_exit() {
    nft_unregister_expr(&mut nft_dup_ipv4_type);
}

// module_init(nft_dup_ipv4_module_init);
// module_exit(nft_dup_ipv4_module_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Pablo Neira Ayuso <pablo@netfilter.org>");
// MODULE_ALIAS_NFT_AF_EXPR(AF_INET, "dup");
// MODULE_DESCRIPTION("IPv4 nftables packet duplication support");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
