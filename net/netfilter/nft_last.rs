// SPDX-License-Identifier: GPL-2.0-only
// Translated from nft_last.c. Kernel headers and external symbols are supplied
// by the surrounding build environment.

#[repr(C)]
pub struct nft_last {
    pub jiffies: ::core::ffi::c_ulong,
    pub set: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct nft_last_priv {
    pub last: *mut nft_last,
}

// const struct nla_policy nft_last_policy[NFTA_LAST_MAX + 1]
static nft_last_policy: [nla_policy; NFTA_LAST_MAX as usize + 1] = [
    // [NFTA_LAST_SET] = { .type = NLA_U32 },
    // [NFTA_LAST_MSECS] = { .type = NLA_U64 },
    nla_policy { ..unsafe { ::core::mem::zeroed() } };
    NFTA_LAST_MAX as usize + 1
];

unsafe fn nft_last_init(
    ctx: *const nft_ctx,
    expr: *const nft_expr,
    tb: *const *const nlattr,
) -> ::core::ffi::c_int {
    let priv_: *mut nft_last_priv = nft_expr_priv(expr);
    let mut last: *mut nft_last;
    let mut last_jiffies: u64 = 0;
    let mut err: ::core::ffi::c_int;

    last = kzalloc_obj::<nft_last>(GFP_KERNEL_ACCOUNT);
    if last.is_null() {
        return -ENOMEM;
    }

    if !(*tb.add(NFTA_LAST_SET as usize)).is_null() {
        (*last).set = ntohl(nla_get_be32(*tb.add(NFTA_LAST_SET as usize)));
    }

    if (*last).set != 0 && !(*tb.add(NFTA_LAST_MSECS as usize)).is_null() {
        err = nf_msecs_to_jiffies64(*tb.add(NFTA_LAST_MSECS as usize), &mut last_jiffies);
        if err < 0 {
            kfree(last);
            return err;
        }

        (*last).jiffies = jiffies - last_jiffies as ::core::ffi::c_ulong;
    }
    (*priv_).last = last;

    0
}

unsafe fn nft_last_eval(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
) {
    let priv_: *mut nft_last_priv = nft_expr_priv(expr);
    let last: *mut nft_last = (*priv_).last;

    if READ_ONCE(&(*last).jiffies) != jiffies {
        WRITE_ONCE(&mut (*last).jiffies, jiffies);
    }
    if READ_ONCE(&(*last).set) == 0 {
        WRITE_ONCE(&mut (*last).set, 1);
    }
}

unsafe fn nft_last_dump(
    skb: *mut sk_buff,
    expr: *const nft_expr,
    reset: bool,
) -> ::core::ffi::c_int {
    let priv_: *mut nft_last_priv = nft_expr_priv(expr);
    let last: *mut nft_last = (*priv_).last;
    let last_jiffies: ::core::ffi::c_ulong = READ_ONCE(&(*last).jiffies);
    let mut last_set: u32 = READ_ONCE(&(*last).set);
    let mut msecs: __be64;

    if time_before(jiffies, last_jiffies) {
        WRITE_ONCE(&mut (*last).set, 0);
        last_set = 0;
    }

    if last_set != 0 {
        msecs = nf_jiffies64_to_msecs(jiffies - last_jiffies);
    } else {
        msecs = 0;
    }

    if nla_put_be32(skb, NFTA_LAST_SET, htonl(last_set)) != 0
        || nla_put_be64(skb, NFTA_LAST_MSECS, msecs, NFTA_LAST_PAD) != 0
    {
        return -1;
    }

    0
}

unsafe fn nft_last_destroy(ctx: *const nft_ctx, expr: *const nft_expr) {
    let priv_: *mut nft_last_priv = nft_expr_priv(expr);
    kfree((*priv_).last);
}

unsafe fn nft_last_clone(
    dst: *mut nft_expr,
    src: *const nft_expr,
    gfp: gfp_t,
) -> ::core::ffi::c_int {
    let priv_dst: *mut nft_last_priv = nft_expr_priv(dst);
    let priv_src: *mut nft_last_priv = nft_expr_priv(src);

    (*priv_dst).last = kzalloc_obj::<nft_last>(gfp);
    if (*priv_dst).last.is_null() {
        return -ENOMEM;
    }

    (*(*priv_dst).last).set = (*(*priv_src).last).set;
    (*(*priv_dst).last).jiffies = (*(*priv_src).last).jiffies;

    0
}

static nft_last_ops: nft_expr_ops = nft_expr_ops {
    type_: &nft_last_type,
    size: NFT_EXPR_SIZE(::core::mem::size_of::<nft_last_priv>()),
    eval: Some(nft_last_eval),
    init: Some(nft_last_init),
    destroy: Some(nft_last_destroy),
    clone: Some(nft_last_clone),
    dump: Some(nft_last_dump),
};

#[no_mangle]
pub static mut nft_last_type: nft_expr_type = nft_expr_type {
    name: b"last\0".as_ptr() as *const _,
    ops: &nft_last_ops,
    policy: &nft_last_policy,
    maxattr: NFTA_LAST_MAX,
    flags: NFT_EXPR_STATEFUL,
    owner: THIS_MODULE,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
