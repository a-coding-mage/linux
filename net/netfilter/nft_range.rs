// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2016 Pablo Neira Ayuso <pablo@netfilter.org>
 */

// Kernel headers and symbols used by this translation are supplied by other
// translated units.

#[repr(C)]
pub struct nft_range_expr {
    pub data_from: nft_data,
    pub data_to: nft_data,
    pub sreg: u8,
    pub len: u8,
    pub op: nft_range_ops,
}

pub unsafe extern "C" fn nft_range_eval(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    _pkt: *const nft_pktinfo,
) {
    let priv_: *const nft_range_expr = nft_expr_priv(expr) as *const nft_range_expr;
    let d1: i32;
    let d2: i32;

    d1 = memcmp(
        unsafe { (*regs).data.as_ptr().add((*priv_).sreg as usize) } as *const _,
        unsafe { &(*priv_).data_from } as *const _ as *const _,
        unsafe { (*priv_).len as usize },
    );
    d2 = memcmp(
        unsafe { (*regs).data.as_ptr().add((*priv_).sreg as usize) } as *const _,
        unsafe { &(*priv_).data_to } as *const _ as *const _,
        unsafe { (*priv_).len as usize },
    );
    match unsafe { (*priv_).op } {
        NFT_RANGE_EQ => {
            if d1 < 0 || d2 > 0 {
                unsafe { (*regs).verdict.code = NFT_BREAK; }
            }
        }
        NFT_RANGE_NEQ => {
            if d1 >= 0 && d2 <= 0 {
                unsafe { (*regs).verdict.code = NFT_BREAK; }
            }
        }
        _ => {}
    }
}

#[repr(C)]
pub struct nla_policy {
    pub _private: [u8; 0],
}

static mut NFT_RANGE_POLICY: [nla_policy; (NFTA_RANGE_MAX as usize) + 1] =
    [nla_policy { _private: [] }; (NFTA_RANGE_MAX as usize) + 1];

pub unsafe extern "C" fn nft_range_init(
    ctx: *const nft_ctx,
    expr: *const nft_expr,
    tb: *const *const nlattr,
) -> i32 {
    let priv_: *mut nft_range_expr = nft_expr_priv(expr) as *mut nft_range_expr;
    let mut desc_from = nft_data_desc { type_: NFT_DATA_VALUE, size: core::mem::size_of::<nft_data>(), len: 0 };
    let mut desc_to = nft_data_desc { type_: NFT_DATA_VALUE, size: core::mem::size_of::<nft_data>(), len: 0 };
    let mut err: i32;
    let mut op: u32 = 0;

    if (*tb.add(NFTA_RANGE_SREG as usize)).is_null()
        || (*tb.add(NFTA_RANGE_OP as usize)).is_null()
        || (*tb.add(NFTA_RANGE_FROM_DATA as usize)).is_null()
        || (*tb.add(NFTA_RANGE_TO_DATA as usize)).is_null()
    { return -EINVAL; }

    err = nft_data_init(core::ptr::null(), &mut (*priv_).data_from, &mut desc_from, *tb.add(NFTA_RANGE_FROM_DATA as usize));
    if err < 0 { return err; }
    err = nft_data_init(core::ptr::null(), &mut (*priv_).data_to, &mut desc_to, *tb.add(NFTA_RANGE_TO_DATA as usize));
    if err < 0 { nft_data_release(&mut (*priv_).data_from, desc_from.type_); return err; }
    if desc_from.len != desc_to.len { err = -EINVAL; } else {
        err = nft_parse_register_load(ctx, *tb.add(NFTA_RANGE_SREG as usize), &mut (*priv_).sreg, desc_from.len);
        if err >= 0 { err = nft_parse_u32_check(*tb.add(NFTA_RANGE_OP as usize), U8_MAX, &mut op); }
        if err >= 0 && op != NFT_RANGE_EQ && op != NFT_RANGE_NEQ { err = -EINVAL; }
    }
    if err < 0 { nft_data_release(&mut (*priv_).data_to, desc_to.type_); nft_data_release(&mut (*priv_).data_from, desc_from.type_); return err; }
    (*priv_).op = op as nft_range_ops; (*priv_).len = desc_from.len; 0
}

pub unsafe extern "C" fn nft_range_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> i32 {
    let priv_: *const nft_range_expr = nft_expr_priv(expr) as *const nft_range_expr;
    if nft_dump_register(skb, NFTA_RANGE_SREG, (*priv_).sreg) != 0 { return -1; }
    if nla_put_be32(skb, NFTA_RANGE_OP, htonl((*priv_).op as u32)) != 0 { return -1; }
    if nft_data_dump(skb, NFTA_RANGE_FROM_DATA, &(*priv_).data_from, NFT_DATA_VALUE, (*priv_).len) < 0
        || nft_data_dump(skb, NFTA_RANGE_TO_DATA, &(*priv_).data_to, NFT_DATA_VALUE, (*priv_).len) < 0 { return -1; }
    0
}

// External declarations for symbols provided by the kernel translation units.
extern "C" {
    static mut nft_range_type: nft_expr_type;
    fn nft_expr_priv(expr: *const nft_expr) -> *mut core::ffi::c_void;
    fn memcmp(a: *const core::ffi::c_void, b: *const core::ffi::c_void, n: usize) -> i32;
    fn nft_data_init(ctx: *const core::ffi::c_void, data: *mut nft_data, desc: *mut nft_data_desc, attr: *const nlattr) -> i32;
    fn nft_data_release(data: *mut nft_data, type_: u32);
    fn nft_parse_register_load(ctx: *const nft_ctx, attr: *const nlattr, reg: *mut u8, len: u8) -> i32;
    fn nft_parse_u32_check(attr: *const nlattr, max: u32, value: *mut u32) -> i32;
    fn nft_dump_register(skb: *mut sk_buff, attr: u32, reg: u8) -> i32;
    fn nla_put_be32(skb: *mut sk_buff, attr: u32, value: u32) -> i32;
    fn nft_data_dump(skb: *mut sk_buff, attr: u32, data: *const nft_data, type_: u32, len: u8) -> i32;
    fn htonl(value: u32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
