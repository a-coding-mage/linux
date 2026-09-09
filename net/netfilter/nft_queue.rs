// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013 Eric Leblond <eric@regit.org>
 *
 * Development of this code partly funded by OISF
 * (http://www.openinfosecfoundation.org/)
 */

// Linux kernel dependencies are supplied by the surrounding translation.

static mut jhash_initval: u32 = 0;

#[repr(C)]
struct nft_queue {
    sreg_qnum: u8,
    queuenum: u16,
    queues_total: u16,
    flags: u16,
}

unsafe extern "C" fn nft_queue_eval(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
) {
    let priv_: *mut nft_queue = nft_expr_priv(expr);
    let mut queue: u32 = (*priv_).queuenum as u32;
    let ret: u32;

    if (*priv_).queues_total > 1 {
        if (*priv_).flags & NFT_QUEUE_FLAG_CPU_FANOUT != 0 {
            let cpu: i32 = raw_smp_processor_id();
            queue = (*priv_).queuenum as u32
                + (cpu as u32) % (*priv_).queues_total as u32;
        } else {
            queue = nfqueue_hash(
                (*pkt).skb,
                queue,
                (*priv_).queues_total,
                nft_pf(pkt),
                jhash_initval,
            );
        }
    }

    ret = NF_QUEUE_NR(queue)
        | if (*priv_).flags & NFT_QUEUE_FLAG_BYPASS != 0 {
            NF_VERDICT_FLAG_QUEUE_BYPASS
        } else {
            0
        };

    (*regs).verdict.code = ret;
}

unsafe extern "C" fn nft_queue_sreg_eval(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    _pkt: *const nft_pktinfo,
) {
    let priv_: *mut nft_queue = nft_expr_priv(expr);
    let queue: u32 = (*regs).data[(*priv_).sreg_qnum as usize];
    let ret = NF_QUEUE_NR(queue)
        | if (*priv_).flags & NFT_QUEUE_FLAG_BYPASS != 0 {
            NF_VERDICT_FLAG_QUEUE_BYPASS
        } else {
            0
        };

    (*regs).verdict.code = ret;
}

unsafe extern "C" fn nft_queue_validate(
    ctx: *const nft_ctx,
    _expr: *const nft_expr,
) -> i32 {
    let supported_hooks: u32 = (1 << NF_INET_PRE_ROUTING)
        | (1 << NF_INET_LOCAL_IN)
        | (1 << NF_INET_FORWARD)
        | (1 << NF_INET_LOCAL_OUT)
        | (1 << NF_INET_POST_ROUTING);

    match (*ctx).family {
        NFPROTO_IPV4 | NFPROTO_IPV6 | NFPROTO_INET | NFPROTO_BRIDGE => {}
        NFPROTO_NETDEV => return -EOPNOTSUPP,
        _ => return -EOPNOTSUPP,
    }

    nft_chain_validate_hooks((*ctx).chain, supported_hooks)
}

static nft_queue_policy: [nla_policy; NFTA_QUEUE_MAX as usize + 1] = [
    nla_policy { type_: NLA_U16 },
    nla_policy { type_: NLA_U16 },
    NLA_POLICY_MASK(NLA_BE16, NFT_QUEUE_FLAG_MASK),
    nla_policy { type_: NLA_U32 },
];

unsafe extern "C" fn nft_queue_init(
    _ctx: *const nft_ctx,
    expr: *const nft_expr,
    tb: *const *const nlattr,
) -> i32 {
    let priv_: *mut nft_queue = nft_expr_priv(expr);
    let maxid: u32;

    (*priv_).queuenum = ntohs(nla_get_be16(*tb.add(NFTA_QUEUE_NUM as usize)));
    if !(*tb.add(NFTA_QUEUE_TOTAL as usize)).is_null() {
        (*priv_).queues_total = ntohs(nla_get_be16(*tb.add(NFTA_QUEUE_TOTAL as usize)));
    } else {
        (*priv_).queues_total = 1;
    }
    if (*priv_).queues_total == 0 { return -EINVAL; }

    maxid = (*priv_).queues_total as u32 - 1 + (*priv_).queuenum as u32;
    if maxid > U16_MAX as u32 { return -ERANGE; }

    if !(*tb.add(NFTA_QUEUE_FLAGS as usize)).is_null() {
        (*priv_).flags = ntohs(nla_get_be16(*tb.add(NFTA_QUEUE_FLAGS as usize)));
        if (*priv_).flags & !NFT_QUEUE_FLAG_MASK != 0 { return -EINVAL; }
    }
    0
}

unsafe extern "C" fn nft_queue_sreg_init(
    ctx: *const nft_ctx,
    expr: *const nft_expr,
    tb: *const *const nlattr,
) -> i32 {
    let priv_: *mut nft_queue = nft_expr_priv(expr);
    let err = nft_parse_register_load(
        ctx,
        *tb.add(NFTA_QUEUE_SREG_QNUM as usize),
        &mut (*priv_).sreg_qnum as *mut u8,
        core::mem::size_of::<u32>(),
    );
    if err < 0 { return err; }
    if !(*tb.add(NFTA_QUEUE_FLAGS as usize)).is_null() {
        (*priv_).flags = ntohs(nla_get_be16(*tb.add(NFTA_QUEUE_FLAGS as usize)));
        if (*priv_).flags & !NFT_QUEUE_FLAG_MASK != 0 { return -EINVAL; }
        if (*priv_).flags & NFT_QUEUE_FLAG_CPU_FANOUT != 0 { return -EOPNOTSUPP; }
    }
    0
}

unsafe extern "C" fn nft_queue_dump(
    skb: *mut sk_buff, expr: *const nft_expr, _reset: bool,
) -> i32 {
    let priv_: *const nft_queue = nft_expr_priv(expr);
    if nla_put_be16(skb, NFTA_QUEUE_NUM, htons((*priv_).queuenum)) != 0
        || nla_put_be16(skb, NFTA_QUEUE_TOTAL, htons((*priv_).queues_total)) != 0
        || nla_put_be16(skb, NFTA_QUEUE_FLAGS, htons((*priv_).flags)) != 0 {
        return -1;
    }
    0
}

unsafe extern "C" fn nft_queue_sreg_dump(
    skb: *mut sk_buff, expr: *const nft_expr, _reset: bool,
) -> i32 {
    let priv_: *const nft_queue = nft_expr_priv(expr);
    if nft_dump_register(skb, NFTA_QUEUE_SREG_QNUM, (*priv_).sreg_qnum) != 0
        || nla_put_be16(skb, NFTA_QUEUE_FLAGS, htons((*priv_).flags)) != 0 {
        return -1;
    }
    0
}

static mut nft_queue_type: nft_expr_type = nft_expr_type {
    name: b"queue\0".as_ptr(),
    select_ops: nft_queue_select_ops,
    policy: nft_queue_policy.as_ptr(),
    maxattr: NFTA_QUEUE_MAX,
    owner: THIS_MODULE,
};

unsafe extern "C" fn nft_queue_select_ops(
    _ctx: *const nft_ctx, tb: *const *const nlattr,
) -> *const nft_expr_ops {
    if !(*tb.add(NFTA_QUEUE_NUM as usize)).is_null()
        && !(*tb.add(NFTA_QUEUE_SREG_QNUM as usize)).is_null() { return ERR_PTR(-EINVAL); }
    init_hashrandom(&mut jhash_initval);
    if !(*tb.add(NFTA_QUEUE_NUM as usize)).is_null() { return &nft_queue_ops; }
    if !(*tb.add(NFTA_QUEUE_SREG_QNUM as usize)).is_null() { return &nft_queue_sreg_ops; }
    ERR_PTR(-EINVAL)
}

static nft_queue_ops: nft_expr_ops = nft_expr_ops {
    type_: unsafe { &nft_queue_type }, size: NFT_EXPR_SIZE(core::mem::size_of::<nft_queue>()),
    eval: nft_queue_eval, init: nft_queue_init, dump: nft_queue_dump, validate: nft_queue_validate,
};
static nft_queue_sreg_ops: nft_expr_ops = nft_expr_ops {
    type_: unsafe { &nft_queue_type }, size: NFT_EXPR_SIZE(core::mem::size_of::<nft_queue>()),
    eval: nft_queue_sreg_eval, init: nft_queue_sreg_init, dump: nft_queue_sreg_dump, validate: nft_queue_validate,
};

unsafe extern "C" fn nft_queue_module_init() -> i32 { nft_register_expr(&nft_queue_type) }
unsafe extern "C" fn nft_queue_module_exit() { nft_unregister_expr(&nft_queue_type); }

// module_init(nft_queue_module_init);
// module_exit(nft_queue_module_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Eric Leblond <eric@regit.org>");
// MODULE_ALIAS_NFT_EXPR("queue");
// MODULE_DESCRIPTION("Netfilter nftables queue module");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
