// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2016 Laura Garcia <nevola@gmail.com>
 */

// Linux kernel and netfilter declarations supplied by the surrounding tree.

#[repr(C)]
struct NftNgInc {
    dreg: u8,
    modulus: u32,
    counter: *mut AtomicT,
    offset: u32,
}

unsafe fn nft_ng_inc_gen(priv_: *mut NftNgInc) -> u32 {
    let mut oval: u32;
    let nval: u32;
    loop {
        oval = atomic_read((*priv_).counter);
        nval = if oval.wrapping_add(1) < (*priv_).modulus {
            oval.wrapping_add(1)
        } else {
            0
        };
        if atomic_cmpxchg((*priv_).counter, oval, nval) == oval {
            break;
        }
    }
    nval.wrapping_add((*priv_).offset)
}

unsafe extern "C" fn nft_ng_inc_eval(
    expr: *const NftExpr,
    regs: *mut NftRegs,
    _pkt: *const NftPktinfo,
) {
    let priv_: *mut NftNgInc = nft_expr_priv(expr);
    (*regs).data[(*priv_).dreg as usize] = nft_ng_inc_gen(priv_);
}

static NFT_NG_POLICY: [NlaPolicy; NFTA_NG_MAX as usize + 1] = [
    // NFTA_NG_DREG: NLA_POLICY_MAX(NLA_BE32, NFT_REG32_MAX)
    NlaPolicy::default(),
    NlaPolicy { type_: NlaU32 },
    NlaPolicy { type_: NlaU32 },
    NlaPolicy { type_: NlaU32 },
];

unsafe extern "C" fn nft_ng_inc_init(
    ctx: *const NftCtx,
    expr: *const NftExpr,
    tb: *const *const Nlattr,
) -> i32 {
    let priv_: *mut NftNgInc = nft_expr_priv(expr);
    let mut err: i32;

    if !(*tb.add(NFTA_NG_OFFSET as usize)).is_null() {
        (*priv_).offset = ntohl(nla_get_be32(*tb.add(NFTA_NG_OFFSET as usize)));
    }
    (*priv_).modulus = ntohl(nla_get_be32(*tb.add(NFTA_NG_MODULUS as usize)));
    if (*priv_).modulus == 0 {
        return -ERANGE;
    }
    if (*priv_).offset.wrapping_add((*priv_).modulus).wrapping_sub(1) < (*priv_).offset {
        return -EOVERFLOW;
    }

    (*priv_).counter = kmalloc_obj::<AtomicT>(GFP_KERNEL_ACCOUNT);
    if (*priv_).counter.is_null() {
        return -ENOMEM;
    }
    atomic_set((*priv_).counter, (*priv_).modulus - 1);

    err = nft_parse_register_store(
        ctx,
        *tb.add(NFTA_NG_DREG as usize),
        &mut (*priv_).dreg,
        core::ptr::null_mut(),
        NFT_DATA_VALUE,
        core::mem::size_of::<u32>(),
    );
    if err < 0 {
        kfree((*priv_).counter);
    }
    err
}

unsafe fn nft_ng_dump(
    skb: *mut SkBuff,
    dreg: NftRegisters,
    modulus: u32,
    type_: NftNgTypes,
    offset: u32,
) -> i32 {
    if nft_dump_register(skb, NFTA_NG_DREG, dreg) != 0
        || nla_put_be32(skb, NFTA_NG_MODULUS, htonl(modulus)) != 0
        || nla_put_be32(skb, NFTA_NG_TYPE, htonl(type_ as u32)) != 0
        || nla_put_be32(skb, NFTA_NG_OFFSET, htonl(offset)) != 0
    {
        return -1;
    }
    0
}

unsafe extern "C" fn nft_ng_inc_dump(skb: *mut SkBuff, expr: *const NftExpr, _reset: bool) -> i32 {
    let priv_: *const NftNgInc = nft_expr_priv(expr);
    nft_ng_dump(skb, (*priv_).dreg as NftRegisters, (*priv_).modulus, NftNgTypes::Incremental, (*priv_).offset)
}

unsafe extern "C" fn nft_ng_inc_destroy(_ctx: *const NftCtx, expr: *const NftExpr) {
    let priv_: *const NftNgInc = nft_expr_priv(expr);
    kfree((*priv_).counter);
}

#[repr(C)]
struct NftNgRandom {
    dreg: u8,
    modulus: u32,
    offset: u32,
}

unsafe fn nft_ng_random_gen(priv_: *const NftNgRandom) -> u32 {
    reciprocal_scale(get_random_u32(), (*priv_).modulus).wrapping_add((*priv_).offset)
}

unsafe extern "C" fn nft_ng_random_eval(expr: *const NftExpr, regs: *mut NftRegs, _pkt: *const NftPktinfo) {
    let priv_: *const NftNgRandom = nft_expr_priv(expr);
    (*regs).data[(*priv_).dreg as usize] = nft_ng_random_gen(priv_);
}

unsafe extern "C" fn nft_ng_random_init(ctx: *const NftCtx, expr: *const NftExpr, tb: *const *const Nlattr) -> i32 {
    let priv_: *mut NftNgRandom = nft_expr_priv(expr);
    if !(*tb.add(NFTA_NG_OFFSET as usize)).is_null() {
        (*priv_).offset = ntohl(nla_get_be32(*tb.add(NFTA_NG_OFFSET as usize)));
    }
    (*priv_).modulus = ntohl(nla_get_be32(*tb.add(NFTA_NG_MODULUS as usize)));
    if (*priv_).modulus == 0 { return -ERANGE; }
    if (*priv_).offset.wrapping_add((*priv_).modulus).wrapping_sub(1) < (*priv_).offset { return -EOVERFLOW; }
    nft_parse_register_store(ctx, *tb.add(NFTA_NG_DREG as usize), &mut (*priv_).dreg, core::ptr::null_mut(), NFT_DATA_VALUE, core::mem::size_of::<u32>())
}

unsafe extern "C" fn nft_ng_random_dump(skb: *mut SkBuff, expr: *const NftExpr, _reset: bool) -> i32 {
    let priv_: *const NftNgRandom = nft_expr_priv(expr);
    nft_ng_dump(skb, (*priv_).dreg as NftRegisters, (*priv_).modulus, NftNgTypes::Random, (*priv_).offset)
}

static mut NFT_NG_TYPE: NftExprType = NftExprType::new("numgen", nft_ng_select_ops, &NFT_NG_POLICY, NFTA_NG_MAX, THIS_MODULE);

unsafe extern "C" fn nft_ng_select_ops(_ctx: *const NftCtx, tb: *const *const Nlattr) -> *const NftExprOps {
    if (*tb.add(NFTA_NG_DREG as usize)).is_null() || (*tb.add(NFTA_NG_MODULUS as usize)).is_null() || (*tb.add(NFTA_NG_TYPE as usize)).is_null() { return err_ptr(-EINVAL); }
    match ntohl(nla_get_be32(*tb.add(NFTA_NG_TYPE as usize))) {
        NFT_NG_INCREMENTAL => &NFT_NG_INC_OPS,
        NFT_NG_RANDOM => &NFT_NG_RANDOM_OPS,
        _ => err_ptr(-EINVAL),
    }
}

static NFT_NG_INC_OPS: NftExprOps = NftExprOps::inc(&NFT_NG_TYPE, core::mem::size_of::<NftNgInc>(), nft_ng_inc_eval, nft_ng_inc_init, nft_ng_inc_destroy, nft_ng_inc_dump);
static NFT_NG_RANDOM_OPS: NftExprOps = NftExprOps::random(&NFT_NG_TYPE, core::mem::size_of::<NftNgRandom>(), nft_ng_random_eval, nft_ng_random_init, nft_ng_random_dump);

unsafe extern "C" fn nft_ng_module_init() -> i32 { nft_register_expr(&mut NFT_NG_TYPE) }
unsafe extern "C" fn nft_ng_module_exit() { nft_unregister_expr(&mut NFT_NG_TYPE); }

// module_init(nft_ng_module_init); module_exit(nft_ng_module_exit);
// MODULE_LICENSE("GPL"); MODULE_AUTHOR("Laura Garcia <nevola@gmail.com>");
// MODULE_ALIAS_NFT_EXPR("numgen"); MODULE_DESCRIPTION("nftables number generator module");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
