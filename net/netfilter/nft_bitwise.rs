// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2008-2009 Patrick McHardy <kaber@trash.net>
 *
 * Development of this code funded by Astaro AG (http://www.astaro.com/)
 */

// Kernel and netfilter declarations are supplied by the surrounding crate.

#[repr(C)]
pub struct nft_bitwise {
    pub sreg: u8,
    pub sreg2: u8,
    pub dreg: u8,
    pub op: nft_bitwise_ops,
    pub len: u8,
    pub mask: nft_data,
    pub xor: nft_data,
    pub data: nft_data,
}

unsafe fn nft_bitwise_eval_mask_xor(dst: *mut u32, src: *const u32, priv_: *const nft_bitwise) {
    let mut i = 0usize;
    while i < (((*priv_).len as usize) + core::mem::size_of::<u32>() - 1) / core::mem::size_of::<u32>() {
        *dst.add(i) = (*src.add(i) & (*priv_).mask.data[i]) ^ (*priv_).xor.data[i];
        i += 1;
    }
}

unsafe fn nft_bitwise_eval_lshift(dst: *mut u32, src: *const u32, priv_: *const nft_bitwise) {
    let shift = (*priv_).data.data[0];
    let mut i = (((*priv_).len as usize) + core::mem::size_of::<u32>() - 1) / core::mem::size_of::<u32>();
    let mut carry: u32 = 0;
    while i > 0 {
        i -= 1;
        let tmp_src = *src.add(i);
        *dst.add(i) = tmp_src.wrapping_shl(shift) | carry;
        carry = tmp_src.wrapping_shr(32u32.wrapping_sub(shift));
    }
}

unsafe fn nft_bitwise_eval_rshift(dst: *mut u32, src: *const u32, priv_: *const nft_bitwise) {
    let shift = (*priv_).data.data[0];
    let n = (((*priv_).len as usize) + core::mem::size_of::<u32>() - 1) / core::mem::size_of::<u32>();
    let mut carry: u32 = 0;
    let mut i = 0usize;
    while i < n {
        let tmp_src = *src.add(i);
        *dst.add(i) = carry | tmp_src.wrapping_shr(shift);
        carry = tmp_src.wrapping_shl(32u32.wrapping_sub(shift));
        i += 1;
    }
}

unsafe fn nft_bitwise_eval_and(dst: *mut u32, src: *const u32, src2: *const u32, priv_: *const nft_bitwise) {
    let n = (((*priv_).len as usize) + 3) / 4;
    for i in 0..n { *dst.add(i) = *src.add(i) & *src2.add(i); }
}

unsafe fn nft_bitwise_eval_or(dst: *mut u32, src: *const u32, src2: *const u32, priv_: *const nft_bitwise) {
    let n = (((*priv_).len as usize) + 3) / 4;
    for i in 0..n { *dst.add(i) = *src.add(i) | *src2.add(i); }
}

unsafe fn nft_bitwise_eval_xor(dst: *mut u32, src: *const u32, src2: *const u32, priv_: *const nft_bitwise) {
    let n = (((*priv_).len as usize) + 3) / 4;
    for i in 0..n { *dst.add(i) = *src.add(i) ^ *src2.add(i); }
}

pub unsafe extern "C" fn nft_bitwise_eval(expr: *const nft_expr, regs: *mut nft_regs, _pkt: *const nft_pktinfo) {
    let priv_ = nft_expr_priv(expr) as *const nft_bitwise;
    let src = (*regs).data.as_ptr().add((*priv_).sreg as usize);
    let dst = (*regs).data.as_mut_ptr().add((*priv_).dreg as usize);
    if (*priv_).op == NFT_BITWISE_MASK_XOR { nft_bitwise_eval_mask_xor(dst, src, priv_); return; }
    if (*priv_).op == NFT_BITWISE_LSHIFT { nft_bitwise_eval_lshift(dst, src, priv_); return; }
    if (*priv_).op == NFT_BITWISE_RSHIFT { nft_bitwise_eval_rshift(dst, src, priv_); return; }
    let src2 = if (*priv_).sreg2 != 0 { (*regs).data.as_ptr().add((*priv_).sreg2 as usize) } else { (*priv_).data.data.as_ptr() };
    if (*priv_).op == NFT_BITWISE_AND { nft_bitwise_eval_and(dst, src, src2, priv_); return; }
    if (*priv_).op == NFT_BITWISE_OR { nft_bitwise_eval_or(dst, src, src2, priv_); return; }
    if (*priv_).op == NFT_BITWISE_XOR { nft_bitwise_eval_xor(dst, src, src2, priv_); }
}

unsafe fn nft_bitwise_init_mask_xor(priv_: *mut nft_bitwise, tb: *const *const nlattr) -> c_int {
    let mask = nft_data_desc { type_: NFT_DATA_VALUE, size: core::mem::size_of::<nft_data>(), len: (*priv_).len };
    let xor = mask;
    if !(*tb.add(NFTA_BITWISE_DATA as usize)).is_null() || !(*tb.add(NFTA_BITWISE_SREG2 as usize)).is_null() { return -EINVAL; }
    if (*tb.add(NFTA_BITWISE_MASK as usize)).is_null() || (*tb.add(NFTA_BITWISE_XOR as usize)).is_null() { return -EINVAL; }
    let mut err = nft_data_init(core::ptr::null(), &mut (*priv_).mask, &mask, *tb.add(NFTA_BITWISE_MASK as usize));
    if err < 0 { return err; }
    err = nft_data_init(core::ptr::null(), &mut (*priv_).xor, &xor, *tb.add(NFTA_BITWISE_XOR as usize));
    if err < 0 { nft_data_release(&mut (*priv_).mask, mask.type_); }
    err
}

unsafe fn nft_bitwise_init_shift(priv_: *mut nft_bitwise, tb: *const *const nlattr) -> c_int {
    let desc = nft_data_desc { type_: NFT_DATA_VALUE, size: core::mem::size_of::<nft_data>(), len: 4 };
    if !(*tb.add(NFTA_BITWISE_MASK as usize)).is_null() || !(*tb.add(NFTA_BITWISE_XOR as usize)).is_null() || !(*tb.add(NFTA_BITWISE_SREG2 as usize)).is_null() || (*tb.add(NFTA_BITWISE_DATA as usize)).is_null() { return -EINVAL; }
    let err = nft_data_init(core::ptr::null(), &mut (*priv_).data, &desc, *tb.add(NFTA_BITWISE_DATA as usize));
    if err < 0 { return err; }
    if (*priv_).data.data[0] == 0 || (*priv_).data.data[0] >= 32 { nft_data_release(&mut (*priv_).data, desc.type_); return -EINVAL; }
    0
}

unsafe fn nft_bitwise_init_bool(ctx: *const nft_ctx, priv_: *mut nft_bitwise, tb: *const *const nlattr) -> c_int {
    if !(*tb.add(NFTA_BITWISE_MASK as usize)).is_null() || !(*tb.add(NFTA_BITWISE_XOR as usize)).is_null() { return -EINVAL; }
    let data = !(*tb.add(NFTA_BITWISE_DATA as usize)).is_null();
    let reg = !(*tb.add(NFTA_BITWISE_SREG2 as usize)).is_null();
    if data == reg { return -EINVAL; }
    if data {
        let desc = nft_data_desc { type_: NFT_DATA_VALUE, size: core::mem::size_of::<nft_data>(), len: (*priv_).len };
        return nft_data_init(core::ptr::null(), &mut (*priv_).data, &desc, *tb.add(NFTA_BITWISE_DATA as usize));
    }
    let err = nft_parse_register_load(ctx, *tb.add(NFTA_BITWISE_SREG2 as usize), &mut (*priv_).sreg2, (*priv_).len);
    if err < 0 { return err; }
    if nft_reg_overlap((*priv_).sreg2, (*priv_).dreg, (*priv_).len) { return -EINVAL; }
    0
}

pub unsafe extern "C" fn nft_bitwise_init(ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> c_int {
    let priv_ = nft_expr_priv(expr) as *mut nft_bitwise;
    let mut len = 0u32;
    let mut err = nft_parse_u32_check(*tb.add(NFTA_BITWISE_LEN as usize), 255, &mut len);
    if err < 0 { return err; }
    (*priv_).len = len as u8;
    err = nft_parse_register_load(ctx, *tb.add(NFTA_BITWISE_SREG as usize), &mut (*priv_).sreg, (*priv_).len); if err < 0 { return err; }
    err = nft_parse_register_store(ctx, *tb.add(NFTA_BITWISE_DREG as usize), &mut (*priv_).dreg, core::ptr::null_mut(), NFT_DATA_VALUE, (*priv_).len); if err < 0 { return err; }
    if nft_reg_overlap((*priv_).sreg, (*priv_).dreg, (*priv_).len) { return -EINVAL; }
    (*priv_).op = if !(*tb.add(NFTA_BITWISE_OP as usize)).is_null() { ntohl(nla_get_be32(*tb.add(NFTA_BITWISE_OP as usize))) as nft_bitwise_ops } else { NFT_BITWISE_MASK_XOR };
    match (*priv_).op { NFT_BITWISE_MASK_XOR => nft_bitwise_init_mask_xor(priv_, tb), NFT_BITWISE_LSHIFT | NFT_BITWISE_RSHIFT => nft_bitwise_init_shift(priv_, tb), NFT_BITWISE_AND | NFT_BITWISE_OR | NFT_BITWISE_XOR => nft_bitwise_init_bool(ctx, priv_, tb), _ => -EOPNOTSUPP }
}

unsafe fn nft_bitwise_extract_u32_data(tb: *const nlattr, out: *mut u32) -> c_int {
    let mut data = core::mem::zeroed::<nft_data>();
    let desc = nft_data_desc { type_: NFT_DATA_VALUE, size: core::mem::size_of::<nft_data>(), len: 4 };
    let err = nft_data_init(core::ptr::null(), &mut data, &desc, tb); if err < 0 { return err; } *out = data.data[0]; 0
}

// Declarations for the remaining C callbacks and operation table.
extern "C" {
    static nft_bitwise_type: nft_expr_type;
}

unsafe fn nft_bitwise_dump_mask_xor(skb: *mut sk_buff, priv_: *const nft_bitwise) -> c_int {
    if nft_data_dump(skb, NFTA_BITWISE_MASK, &(*priv_).mask, NFT_DATA_VALUE, (*priv_).len) < 0 { return -1; }
    if nft_data_dump(skb, NFTA_BITWISE_XOR, &(*priv_).xor, NFT_DATA_VALUE, (*priv_).len) < 0 { return -1; } 0
}
unsafe fn nft_bitwise_dump_shift(skb: *mut sk_buff, priv_: *const nft_bitwise) -> c_int {
    if nft_data_dump(skb, NFTA_BITWISE_DATA, &(*priv_).data, NFT_DATA_VALUE, 4) < 0 { -1 } else { 0 }
}
unsafe fn nft_bitwise_dump_bool(skb: *mut sk_buff, priv_: *const nft_bitwise) -> c_int {
    if (*priv_).sreg2 != 0 { if nft_dump_register(skb, NFTA_BITWISE_SREG2, (*priv_).sreg2) != 0 { return -1; } }
    else if nft_data_dump(skb, NFTA_BITWISE_DATA, &(*priv_).data, NFT_DATA_VALUE, 4) < 0 { return -1; } 0
}
pub unsafe extern "C" fn nft_bitwise_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> c_int {
    let p = nft_expr_priv(expr) as *const nft_bitwise;
    if nft_dump_register(skb, NFTA_BITWISE_SREG, (*p).sreg) != 0 || nft_dump_register(skb, NFTA_BITWISE_DREG, (*p).dreg) != 0 || nla_put_be32(skb, NFTA_BITWISE_LEN, htonl((*p).len as u32)) != 0 || nla_put_be32(skb, NFTA_BITWISE_OP, htonl((*p).op as u32)) != 0 { return -1; }
    match (*p).op { NFT_BITWISE_MASK_XOR => nft_bitwise_dump_mask_xor(skb,p), NFT_BITWISE_LSHIFT | NFT_BITWISE_RSHIFT => nft_bitwise_dump_shift(skb,p), _ => nft_bitwise_dump_bool(skb,p) }
}

static mut zero: nft_data = nft_data { data: [0; 4] };
unsafe fn nft_bitwise_offload(ctx: *mut nft_offload_ctx, _flow: *mut nft_flow_rule, expr: *const nft_expr) -> c_int {
    let p = nft_expr_priv(expr) as *const nft_bitwise; let reg = &mut (*ctx).regs[(*p).dreg as usize];
    if (*p).op != NFT_BITWISE_MASK_XOR || memcmp(&(*p).xor, &zero, core::mem::size_of::<nft_data>()) != 0 || (*p).sreg != (*p).dreg || (*p).len != (*reg).len { return -EOPNOTSUPP; }
    memcpy(&mut (*reg).mask, &(*p).mask, core::mem::size_of::<nft_data>()); 0
}

unsafe fn nft_bitwise_fast_init(ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> c_int {
    let p = nft_expr_priv(expr) as *mut nft_bitwise_fast_expr;
    let mut e = nft_parse_register_load(ctx, *tb.add(NFTA_BITWISE_SREG as usize), &mut (*p).sreg, 4); if e < 0 { return e; }
    e = nft_parse_register_store(ctx, *tb.add(NFTA_BITWISE_DREG as usize), &mut (*p).dreg, core::ptr::null_mut(), NFT_DATA_VALUE, 4); if e < 0 { return e; }
    if !(*tb.add(NFTA_BITWISE_DATA as usize)).is_null() || !(*tb.add(NFTA_BITWISE_SREG2 as usize)).is_null() || (*tb.add(NFTA_BITWISE_MASK as usize)).is_null() || (*tb.add(NFTA_BITWISE_XOR as usize)).is_null() { return -EINVAL; }
    e = nft_bitwise_extract_u32_data(*tb.add(NFTA_BITWISE_MASK as usize), &mut (*p).mask); if e < 0 { return e; } nft_bitwise_extract_u32_data(*tb.add(NFTA_BITWISE_XOR as usize), &mut (*p).xor)
}

static nft_bitwise_policy: [nla_policy; NFTA_BITWISE_MAX as usize + 1] = [nla_policy { type_: 0 }; NFTA_BITWISE_MAX as usize + 1];

unsafe fn nft_bitwise_fast_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> c_int {
    let p = nft_expr_priv(expr) as *const nft_bitwise_fast_expr; let mut data = core::mem::zeroed::<nft_data>();
    if nft_dump_register(skb,NFTA_BITWISE_SREG,(*p).sreg)!=0 || nft_dump_register(skb,NFTA_BITWISE_DREG,(*p).dreg)!=0 || nla_put_be32(skb,NFTA_BITWISE_LEN,htonl(4))!=0 || nla_put_be32(skb,NFTA_BITWISE_OP,htonl(NFT_BITWISE_MASK_XOR as u32))!=0 { return -1; }
    data.data[0]=(*p).mask; if nft_data_dump(skb,NFTA_BITWISE_MASK,&data,NFT_DATA_VALUE,4)<0{return -1;} data.data[0]=(*p).xor; if nft_data_dump(skb,NFTA_BITWISE_XOR,&data,NFT_DATA_VALUE,4)<0{return -1;} 0
}
unsafe fn nft_bitwise_fast_offload(ctx:*mut nft_offload_ctx,_flow:*mut nft_flow_rule,expr:*const nft_expr)->c_int { let p=nft_expr_priv(expr) as *const nft_bitwise_fast_expr; let r=&mut (*ctx).regs[(*p).dreg as usize]; if (*p).xor!=0||(*p).sreg!=(*p).dreg||(*r).len!=4{return -EOPNOTSUPP;} (*r).mask.data[0]=(*p).mask; 0 }

static nft_bitwise_ops: nft_expr_ops = nft_expr_ops { type_: &nft_bitwise_type, size: 0, eval: Some(nft_bitwise_eval), init: Some(nft_bitwise_init), dump: Some(nft_bitwise_dump), offload: Some(nft_bitwise_offload) };
pub static nft_bitwise_fast_ops: nft_expr_ops = nft_expr_ops { type_: &nft_bitwise_type, size: 0, eval: None, init: Some(nft_bitwise_fast_init), dump: Some(nft_bitwise_fast_dump), offload: Some(nft_bitwise_fast_offload) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
