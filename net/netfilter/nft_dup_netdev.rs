// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2015 Pablo Neira Ayuso <pablo@netfilter.org>
 */

// Linux kernel, netfilter, nf_tables, nf_tables_offload, and nf_dup_netdev
// headers are supplied by the surrounding translation environment.

#[repr(C)]
pub struct nft_dup_netdev {
    pub sreg_dev: u8,
}

unsafe fn nft_dup_netdev_eval(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
) {
    let priv_: *mut nft_dup_netdev = nft_expr_priv(expr);
    let oif: i32 = (*regs).data[(*priv_).sreg_dev as usize] as i32;

    nf_dup_netdev_egress(pkt, oif);
}

static mut nft_dup_netdev_policy: [nla_policy; NFTA_DUP_MAX as usize + 1] = {
    let mut policy = [nla_policy { type_: 0 }; NFTA_DUP_MAX as usize + 1];
    policy[NFTA_DUP_SREG_DEV as usize] = nla_policy { type_: NLA_U32 };
    policy
};

unsafe fn nft_dup_netdev_init(
    ctx: *const nft_ctx,
    expr: *const nft_expr,
    tb: *const *const nlattr,
) -> i32 {
    let priv_: *mut nft_dup_netdev = nft_expr_priv(expr);

    if (*tb.add(NFTA_DUP_SREG_DEV as usize)).is_null() {
        return -EINVAL;
    }

    nft_parse_register_load(
        ctx,
        *tb.add(NFTA_DUP_SREG_DEV as usize),
        &mut (*priv_).sreg_dev as *mut u8,
        core::mem::size_of::<i32>(),
    )
}

unsafe fn nft_dup_netdev_dump(
    skb: *mut sk_buff,
    expr: *const nft_expr,
    _reset: bool,
) -> i32 {
    let priv_: *mut nft_dup_netdev = nft_expr_priv(expr);

    if nft_dump_register(skb, NFTA_DUP_SREG_DEV, (*priv_).sreg_dev) != 0 {
        return -1;
    }

    0
}

unsafe fn nft_dup_netdev_offload(
    ctx: *mut nft_offload_ctx,
    flow: *mut nft_flow_rule,
    expr: *const nft_expr,
) -> i32 {
    let priv_: *const nft_dup_netdev = nft_expr_priv(expr);
    let oif: i32 = (*ctx).regs[(*priv_).sreg_dev as usize].data.data[0] as i32;

    nft_fwd_dup_netdev_offload(ctx, flow, FLOW_ACTION_MIRRED, oif)
}

unsafe fn nft_dup_netdev_offload_action(_expr: *const nft_expr) -> bool {
    true
}

static mut nft_dup_netdev_type: nft_expr_type = nft_expr_type {
    family: NFPROTO_NETDEV,
    name: "dup" as *const str,
    ops: core::ptr::null(),
    policy: core::ptr::null(),
    maxattr: 0,
    owner: core::ptr::null_mut(),
};

static nft_dup_netdev_ops: nft_expr_ops = nft_expr_ops {
    type_: unsafe { &raw mut nft_dup_netdev_type },
    size: NFT_EXPR_SIZE(core::mem::size_of::<nft_dup_netdev>()),
    eval: Some(nft_dup_netdev_eval),
    init: Some(nft_dup_netdev_init),
    dump: Some(nft_dup_netdev_dump),
    offload: Some(nft_dup_netdev_offload),
    offload_action: Some(nft_dup_netdev_offload_action),
};

unsafe fn nft_dup_netdev_module_init() -> i32 {
    nft_register_expr(&raw mut nft_dup_netdev_type)
}

unsafe fn nft_dup_netdev_module_exit() {
    nft_unregister_expr(&raw mut nft_dup_netdev_type);
}

// module_init(nft_dup_netdev_module_init);
// module_exit(nft_dup_netdev_module_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Pablo Neira Ayuso <pablo@netfilter.org>");
// MODULE_ALIAS_NFT_AF_EXPR(5, "dup");
// MODULE_DESCRIPTION("nftables netdev packet duplication support");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
