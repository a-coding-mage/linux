// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2015 Pablo Neira Ayuso <pablo@netfilter.org>
 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/kernel.h, linux/init.h, linux/module.h, linux/netlink.h,
// linux/netfilter.h, linux/netfilter/nf_tables.h,
// net/netfilter/nf_tables.h, and net/netfilter/ipv6/nf_dup_ipv6.h.

#[repr(C)]
pub struct nft_dup_ipv6 {
    pub sreg_addr: u8,
    pub sreg_dev: u8,
}

unsafe fn nft_dup_ipv6_eval(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
) {
    let priv_: *mut nft_dup_ipv6 = nft_expr_priv(expr);
    let gw: *mut in6_addr = unsafe {
        (&mut (*regs).data[(*priv_).sreg_addr as usize]) as *mut _ as *mut in6_addr
    };
    let oif: i32 = if (*priv_).sreg_dev != 0 {
        (*regs).data[(*priv_).sreg_dev as usize] as i32
    } else {
        -1
    };

    nf_dup_ipv6(nft_net(pkt), (*pkt).skb, nft_hook(pkt), gw, oif);
}

unsafe fn nft_dup_ipv6_init(
    ctx: *const nft_ctx,
    expr: *const nft_expr,
    tb: *const *const nlattr,
) -> i32 {
    let priv_: *mut nft_dup_ipv6 = nft_expr_priv(expr);
    let mut err: i32;

    if (*tb.add(NFTA_DUP_SREG_ADDR as usize)).is_null() {
        return -EINVAL;
    }

    err = nft_parse_register_load(
        ctx,
        *tb.add(NFTA_DUP_SREG_ADDR as usize),
        &mut (*priv_).sreg_addr as *mut u8,
        core::mem::size_of::<in6_addr>(),
    );
    if err < 0 {
        return err;
    }

    if !(*tb.add(NFTA_DUP_SREG_DEV as usize)).is_null() {
        err = nft_parse_register_load(
            ctx,
            *tb.add(NFTA_DUP_SREG_DEV as usize),
            &mut (*priv_).sreg_dev as *mut u8,
            core::mem::size_of::<i32>(),
        );
    }

    err
}

unsafe fn nft_dup_ipv6_dump(
    skb: *mut sk_buff,
    expr: *const nft_expr,
    _reset: bool,
) -> i32 {
    let priv_: *mut nft_dup_ipv6 = nft_expr_priv(expr);

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

static mut nft_dup_ipv6_type: nft_expr_type = nft_expr_type {
    family: NFPROTO_IPV6,
    name: "dup",
    ops: &nft_dup_ipv6_ops,
    policy: nft_dup_ipv6_policy,
    maxattr: NFTA_DUP_MAX,
    owner: THIS_MODULE,
};

static nft_dup_ipv6_ops: nft_expr_ops = nft_expr_ops {
    type_: unsafe { &nft_dup_ipv6_type },
    size: NFT_EXPR_SIZE(core::mem::size_of::<nft_dup_ipv6>()),
    eval: nft_dup_ipv6_eval,
    init: nft_dup_ipv6_init,
    dump: nft_dup_ipv6_dump,
};

static nft_dup_ipv6_policy: [nla_policy; NFTA_DUP_MAX as usize + 1] = {
    let mut policy = [nla_policy { type_: 0 }; NFTA_DUP_MAX as usize + 1];
    policy[NFTA_DUP_SREG_ADDR as usize] = nla_policy { type_: NLA_U32 };
    policy[NFTA_DUP_SREG_DEV as usize] = nla_policy { type_: NLA_U32 };
    policy
};

unsafe fn nft_dup_ipv6_module_init() -> i32 {
    nft_register_expr(&nft_dup_ipv6_type)
}

unsafe fn nft_dup_ipv6_module_exit() {
    nft_unregister_expr(&nft_dup_ipv6_type);
}

// module_init(nft_dup_ipv6_module_init);
// module_exit(nft_dup_ipv6_module_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Pablo Neira Ayuso <pablo@netfilter.org>");
// MODULE_ALIAS_NFT_AF_EXPR(AF_INET6, "dup");
// MODULE_DESCRIPTION("IPv6 nftables packet duplication support");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
