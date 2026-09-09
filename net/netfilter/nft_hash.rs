// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2016 Laura Garcia <nevola@gmail.com>
 */

// Kernel dependencies supplied by the surrounding translation unit/build.

#[repr(C)]
pub struct nft_jhash {
    pub sreg: u8,
    pub dreg: u8,
    pub len: u8,
    pub autogen_seed: u8,
    pub modulus: u32,
    pub seed: u32,
    pub offset: u32,
}

unsafe fn nft_jhash_eval(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    _pkt: *const nft_pktinfo,
) {
    let priv_: *mut nft_jhash = nft_expr_priv(expr);
    let data = unsafe { (*regs).data.as_ptr().add((*priv_).sreg as usize) as *const _ };
    let h: u32 = reciprocal_scale(
        jhash(data, (*priv_).len, (*priv_).seed),
        (*priv_).modulus,
    );

    unsafe {
        (*regs).data[(*priv_).dreg as usize] = h.wrapping_add((*priv_).offset);
    }
}

#[repr(C)]
pub struct nft_symhash {
    pub dreg: u8,
    pub modulus: u32,
    pub offset: u32,
}

unsafe fn nft_symhash_eval(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
) {
    let priv_: *mut nft_symhash = nft_expr_priv(expr);
    let skb = unsafe { (*pkt).skb };
    let h: u32 = reciprocal_scale(
        __skb_get_hash_symmetric_net(nft_net(pkt), skb),
        (*priv_).modulus,
    );

    unsafe {
        (*regs).data[(*priv_).dreg as usize] = h.wrapping_add((*priv_).offset);
    }
}

static nft_hash_policy: [nla_policy; NFTA_HASH_MAX as usize + 1] = [
    /* NFTA_HASH_SREG = NLA_POLICY_MAX(NLA_BE32, NFT_REG32_MAX) */
    /* NFTA_HASH_DREG = NLA_POLICY_MAX(NLA_BE32, NFT_REG32_MAX) */
    /* NFTA_HASH_LEN = NLA_POLICY_MAX(NLA_BE32, 255) */
    /* NFTA_HASH_MODULUS = { .type = NLA_U32 } */
    /* NFTA_HASH_SEED = { .type = NLA_U32 } */
    /* NFTA_HASH_OFFSET = { .type = NLA_U32 } */
    /* NFTA_HASH_TYPE = { .type = NLA_U32 } */
];

unsafe fn nft_jhash_init(
    ctx: *const nft_ctx,
    expr: *const nft_expr,
    tb: *const *const nlattr,
) -> i32 {
    let priv_: *mut nft_jhash = nft_expr_priv(expr);
    let mut len: u32 = 0;
    let mut err: i32;

    if (*tb.add(NFTA_HASH_SREG as usize)).is_null()
        || (*tb.add(NFTA_HASH_DREG as usize)).is_null()
        || (*tb.add(NFTA_HASH_LEN as usize)).is_null()
        || (*tb.add(NFTA_HASH_MODULUS as usize)).is_null()
    {
        return -EINVAL;
    }

    if !(*tb.add(NFTA_HASH_OFFSET as usize)).is_null() {
        (*priv_).offset = ntohl(nla_get_be32(*tb.add(NFTA_HASH_OFFSET as usize)));
    }

    err = nft_parse_u32_check(*tb.add(NFTA_HASH_LEN as usize), U8_MAX, &mut len);
    if err < 0 { return err; }
    if len == 0 { return -ERANGE; }
    (*priv_).len = len as u8;

    err = nft_parse_register_load(ctx, *tb.add(NFTA_HASH_SREG as usize), &mut (*priv_).sreg, len);
    if err < 0 { return err; }
    (*priv_).modulus = ntohl(nla_get_be32(*tb.add(NFTA_HASH_MODULUS as usize)));
    if (*priv_).modulus < 1 { return -ERANGE; }
    if (*priv_).offset.wrapping_add((*priv_).modulus).wrapping_sub(1) < (*priv_).offset {
        return -EOVERFLOW;
    }
    if !(*tb.add(NFTA_HASH_SEED as usize)).is_null() {
        (*priv_).seed = ntohl(nla_get_be32(*tb.add(NFTA_HASH_SEED as usize)));
    } else {
        (*priv_).autogen_seed = 1;
        get_random_bytes(&mut (*priv_).seed as *mut u32 as *mut _, core::mem::size_of::<u32>());
    }
    nft_parse_register_store(ctx, *tb.add(NFTA_HASH_DREG as usize), &mut (*priv_).dreg, core::ptr::null_mut(), NFT_DATA_VALUE, core::mem::size_of::<u32>())
}

unsafe fn nft_symhash_init(ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> i32 {
    let priv_: *mut nft_symhash = nft_expr_priv(expr);
    if (*tb.add(NFTA_HASH_DREG as usize)).is_null() || (*tb.add(NFTA_HASH_MODULUS as usize)).is_null() { return -EINVAL; }
    if !(*tb.add(NFTA_HASH_OFFSET as usize)).is_null() { (*priv_).offset = ntohl(nla_get_be32(*tb.add(NFTA_HASH_OFFSET as usize))); }
    (*priv_).modulus = ntohl(nla_get_be32(*tb.add(NFTA_HASH_MODULUS as usize)));
    if (*priv_).modulus < 1 { return -ERANGE; }
    if (*priv_).offset.wrapping_add((*priv_).modulus).wrapping_sub(1) < (*priv_).offset { return -EOVERFLOW; }
    nft_parse_register_store(ctx, *tb.add(NFTA_HASH_DREG as usize), &mut (*priv_).dreg, core::ptr::null_mut(), NFT_DATA_VALUE, core::mem::size_of::<u32>())
}

unsafe fn nft_jhash_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> i32 {
    let priv_: *const nft_jhash = nft_expr_priv(expr);
    if nft_dump_register(skb, NFTA_HASH_SREG, (*priv_).sreg) != 0 { return -1; }
    if nft_dump_register(skb, NFTA_HASH_DREG, (*priv_).dreg) != 0 { return -1; }
    if nla_put_be32(skb, NFTA_HASH_LEN, htonl((*priv_).len as u32)) != 0 { return -1; }
    if nla_put_be32(skb, NFTA_HASH_MODULUS, htonl((*priv_).modulus)) != 0 { return -1; }
    if (*priv_).autogen_seed == 0 && nla_put_be32(skb, NFTA_HASH_SEED, htonl((*priv_).seed)) != 0 { return -1; }
    if (*priv_).offset != 0 && nla_put_be32(skb, NFTA_HASH_OFFSET, htonl((*priv_).offset)) != 0 { return -1; }
    if nla_put_be32(skb, NFTA_HASH_TYPE, htonl(NFT_HASH_JENKINS)) != 0 { return -1; }
    0
}

unsafe fn nft_symhash_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> i32 {
    let priv_: *const nft_symhash = nft_expr_priv(expr);
    if nft_dump_register(skb, NFTA_HASH_DREG, (*priv_).dreg) != 0 { return -1; }
    if nla_put_be32(skb, NFTA_HASH_MODULUS, htonl((*priv_).modulus)) != 0 { return -1; }
    if (*priv_).offset != 0 && nla_put_be32(skb, NFTA_HASH_OFFSET, htonl((*priv_).offset)) != 0 { return -1; }
    if nla_put_be32(skb, NFTA_HASH_TYPE, htonl(NFT_HASH_SYM)) != 0 { return -1; }
    0
}

unsafe fn nft_hash_select_ops(_ctx: *const nft_ctx, tb: *const *const nlattr) -> *const nft_expr_ops {
    if (*tb.add(NFTA_HASH_TYPE as usize)).is_null() { return &nft_jhash_ops; }
    match ntohl(nla_get_be32(*tb.add(NFTA_HASH_TYPE as usize))) {
        NFT_HASH_SYM => &nft_symhash_ops,
        NFT_HASH_JENKINS => &nft_jhash_ops,
        _ => ERR_PTR(-EOPNOTSUPP),
    }
}
static mut nft_hash_type: nft_expr_type = nft_expr_type::default();

unsafe fn nft_hash_module_init() -> i32 { nft_register_expr(&mut nft_hash_type) }
unsafe fn nft_hash_module_exit() { nft_unregister_expr(&mut nft_hash_type); }

static nft_jhash_ops: nft_expr_ops = nft_expr_ops::default();
static nft_symhash_ops: nft_expr_ops = nft_expr_ops::default();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
