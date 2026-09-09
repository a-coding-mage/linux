// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2008-2009 Patrick McHardy <kaber@trash.net>
 *
 * Development of this code funded by Astaro AG (http://www.astaro.com/)
 */

// Kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct nft_cmp_expr {
    pub data: nft_data,
    pub sreg: u8,
    pub len: u8,
    pub op: nft_cmp_ops,
}

pub unsafe extern "C" fn nft_cmp_eval(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    _pkt: *const nft_pktinfo,
) {
    let priv_ = nft_expr_priv(expr) as *const nft_cmp_expr;
    let d: i32 = libc::memcmp(
        (&(*regs).data[(*priv_).sreg as usize]) as *const _ as *const libc::c_void,
        (&(*priv_).data) as *const _ as *const libc::c_void,
        (*priv_).len as usize,
    ) as i32;

    match (*priv_).op {
        NFT_CMP_EQ => {
            if d != 0 {
                (*regs).verdict.code = NFT_BREAK;
            }
        }
        NFT_CMP_NEQ => {
            if d == 0 {
                (*regs).verdict.code = NFT_BREAK;
            }
        }
        NFT_CMP_LT => {
            if d == 0 || d > 0 {
                (*regs).verdict.code = NFT_BREAK;
            }
        }
        NFT_CMP_LTE => {
            if d > 0 {
                (*regs).verdict.code = NFT_BREAK;
            }
        }
        NFT_CMP_GT => {
            if d == 0 || d < 0 {
                (*regs).verdict.code = NFT_BREAK;
            }
        }
        NFT_CMP_GTE => {
            if d < 0 {
                (*regs).verdict.code = NFT_BREAK;
            }
        }
        _ => {}
    }
}

static nft_cmp_policy: [nla_policy; NFTA_CMP_MAX + 1] = [
    /* NFTA_CMP_SREG = NLA_POLICY_MAX(NLA_BE32, NFT_REG32_MAX) */
    /* NFTA_CMP_OP = { .type = NLA_U32 } */
    /* NFTA_CMP_DATA = { .type = NLA_NESTED } */
];

unsafe extern "C" fn nft_cmp_init(
    ctx: *const nft_ctx,
    expr: *const nft_expr,
    tb: *const *const nlattr,
) -> i32 {
    let priv_ = nft_expr_priv(expr) as *mut nft_cmp_expr;
    let mut desc = nft_data_desc { type_: NFT_DATA_VALUE, size: core::mem::size_of::<nft_data>() };
    let mut err = nft_data_init(core::ptr::null_mut(), &mut (*priv_).data, &mut desc, *tb.add(NFTA_CMP_DATA));
    if err < 0 { return err; }
    err = nft_parse_register_load(ctx, *tb.add(NFTA_CMP_SREG), &mut (*priv_).sreg, desc.len);
    if err < 0 { return err; }
    (*priv_).op = ntohl(nla_get_be32(*tb.add(NFTA_CMP_OP)));
    (*priv_).len = desc.len;
    0
}

unsafe extern "C" fn nft_cmp_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> i32 {
    let priv_ = nft_expr_priv(expr) as *const nft_cmp_expr;
    if nft_dump_register(skb, NFTA_CMP_SREG, (*priv_).sreg) != 0 { return -1; }
    if nla_put_be32(skb, NFTA_CMP_OP, htonl((*priv_).op)) != 0 { return -1; }
    if nft_data_dump(skb, NFTA_CMP_DATA, &(*priv_).data, NFT_DATA_VALUE, (*priv_).len) < 0 { return -1; }
    0
}

#[repr(C)]
pub union nft_cmp_offload_data { pub val16: u16, pub val32: u32, pub val64: u64 }

unsafe fn nft_payload_n2h(data: *mut nft_cmp_offload_data, val: *const u8, len: u32) {
    match len {
        2 => (*data).val16 = ntohs(*(val as *const u16)),
        4 => (*data).val32 = ntohl(*(val as *const u32)),
        8 => (*data).val64 = be64_to_cpu(*(val as *const u64)),
        _ => WARN_ON_ONCE(1),
    }
}

unsafe fn __nft_cmp_offload(ctx: *mut nft_offload_ctx, flow: *mut nft_flow_rule, priv_: *const nft_cmp_expr) -> i32 {
    let reg = &mut (*ctx).regs[(*priv_).sreg as usize];
    let mut data_u = nft_cmp_offload_data { val64: 0 };
    let mut datamask_u = nft_cmp_offload_data { val64: 0 };
    let (data, datamask): (*mut u8, *mut u8);
    if (*priv_).op != NFT_CMP_EQ || (*priv_).len > reg.len { return -EOPNOTSUPP; }
    if reg.flags & NFT_OFFLOAD_F_NETWORK2HOST != 0 {
        nft_payload_n2h(&mut data_u, (&(*priv_).data) as *const _ as *const u8, reg.len as u32);
        nft_payload_n2h(&mut datamask_u, (&reg.mask) as *const _ as *const u8, reg.len as u32);
        data = &mut data_u as *mut _ as *mut u8;
        datamask = &mut datamask_u as *mut _ as *mut u8;
    } else {
        data = &(*priv_).data as *const _ as *mut u8;
        datamask = &reg.mask as *const _ as *mut u8;
    }
    libc::memcpy(((&mut (*flow).match_.key as *mut _) as *mut u8).add(reg.offset as usize), data as *const _, reg.len as usize);
    libc::memcpy(((&mut (*flow).match_.mask as *mut _) as *mut u8).add(reg.offset as usize), datamask as *const _, reg.len as usize);
    (*flow).match_.dissector.used_keys |= 1u64 << reg.key;
    (*flow).match_.dissector.offset[reg.key as usize] = reg.base_offset;
    if reg.key == FLOW_DISSECTOR_KEY_META && reg.offset == core::mem::offset_of!(nft_flow_key, meta.ingress_iftype) && nft_reg_load16((*priv_).data.data.as_ptr()) != ARPHRD_ETHER { return -EOPNOTSUPP; }
    nft_offload_update_dependency(ctx, &(*priv_).data, reg.len as u32);
    0
}

unsafe extern "C" fn nft_cmp_offload(ctx: *mut nft_offload_ctx, flow: *mut nft_flow_rule, expr: *const nft_expr) -> i32 {
    __nft_cmp_offload(ctx, flow, nft_expr_priv(expr) as *const nft_cmp_expr)
}

// The remaining expression-operation tables and fast-path helpers mirror the C definitions.
// External kernel types and functions are intentionally left unresolved.

unsafe fn nft_cmp_fast_mask(len: u32) -> u32 {
    cpu_to_le32(!0u32 >> (core::mem::size_of::<u32>() as u32 * BITS_PER_BYTE - len))
}

unsafe extern "C" fn nft_cmp_fast_init(ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> i32 {
    let priv_ = nft_expr_priv(expr) as *mut nft_cmp_fast_expr;
    let mut data = core::mem::zeroed::<nft_data>();
    let mut desc = nft_data_desc { type_: NFT_DATA_VALUE, size: core::mem::size_of::<nft_data>() };
    let mut err = nft_data_init(core::ptr::null_mut(), &mut data, &mut desc, *tb.add(NFTA_CMP_DATA));
    if err < 0 { return err; }
    err = nft_parse_register_load(ctx, *tb.add(NFTA_CMP_SREG), &mut (*priv_).sreg, desc.len);
    if err < 0 { return err; }
    desc.len *= BITS_PER_BYTE;
    (*priv_).mask = nft_cmp_fast_mask(desc.len as u32);
    (*priv_).data = data.data[0] & (*priv_).mask;
    (*priv_).len = desc.len;
    (*priv_).inv = ntohl(nla_get_be32(*tb.add(NFTA_CMP_OP))) != NFT_CMP_EQ;
    0
}

unsafe extern "C" fn nft_cmp_fast_offload(ctx: *mut nft_offload_ctx, flow: *mut nft_flow_rule, expr: *const nft_expr) -> i32 {
    let priv_ = nft_expr_priv(expr) as *const nft_cmp_fast_expr;
    let mut cmp: nft_cmp_expr = core::mem::zeroed();
    cmp.data.data[0] = (*priv_).data;
    cmp.sreg = (*priv_).sreg;
    cmp.len = (*priv_).len / BITS_PER_BYTE as u8;
    cmp.op = if (*priv_).inv { NFT_CMP_NEQ } else { NFT_CMP_EQ };
    __nft_cmp_offload(ctx, flow, &cmp)
}

unsafe extern "C" fn nft_cmp_fast_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> i32 {
    let priv_ = nft_expr_priv(expr) as *const nft_cmp_fast_expr;
    let op = if (*priv_).inv { NFT_CMP_NEQ } else { NFT_CMP_EQ };
    let mut data = core::mem::zeroed::<nft_data>();
    if nft_dump_register(skb, NFTA_CMP_SREG, (*priv_).sreg) != 0 { return -1; }
    if nla_put_be32(skb, NFTA_CMP_OP, htonl(op)) != 0 { return -1; }
    data.data[0] = (*priv_).data;
    if nft_data_dump(skb, NFTA_CMP_DATA, &data, NFT_DATA_VALUE, (*priv_).len / BITS_PER_BYTE as u8) < 0 { return -1; }
    0
}

pub static nft_cmp_fast_ops: nft_expr_ops = nft_expr_ops { type_: &nft_cmp_type, size: NFT_EXPR_SIZE(core::mem::size_of::<nft_cmp_fast_expr>()), eval: None, init: Some(nft_cmp_fast_init), dump: Some(nft_cmp_fast_dump), offload: Some(nft_cmp_fast_offload) };

unsafe fn nft_cmp_mask(bitlen: u32) -> u32 { cpu_to_le32(!0u32 >> (core::mem::size_of::<u32>() as u32 * BITS_PER_BYTE - bitlen)) }

unsafe fn nft_cmp16_fast_mask(data: *mut nft_data, mut bitlen: u32) {
    let len = bitlen / BITS_PER_BYTE;
    let words = len / core::mem::size_of::<u32>() as u32;
    let mut i = 0;
    while i < words { (*data).data[i as usize] = 0xffffffff; bitlen -= core::mem::size_of::<u32>() as u32 * BITS_PER_BYTE; i += 1; }
    if len % core::mem::size_of::<u32>() as u32 != 0 { (*data).data[i as usize] = nft_cmp_mask(bitlen); i += 1; }
    while i < 4 { (*data).data[i as usize] = 0; i += 1; }
}

unsafe extern "C" fn nft_cmp16_fast_init(ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> i32 {
    let priv_ = nft_expr_priv(expr) as *mut nft_cmp16_fast_expr;
    let mut desc = nft_data_desc { type_: NFT_DATA_VALUE, size: core::mem::size_of::<nft_data>() };
    let mut err = nft_data_init(core::ptr::null_mut(), &mut (*priv_).data, &mut desc, *tb.add(NFTA_CMP_DATA));
    if err < 0 { return err; }
    err = nft_parse_register_load(ctx, *tb.add(NFTA_CMP_SREG), &mut (*priv_).sreg, desc.len);
    if err < 0 { return err; }
    nft_cmp16_fast_mask(&mut (*priv_).mask, desc.len as u32 * BITS_PER_BYTE);
    (*priv_).inv = ntohl(nla_get_be32(*tb.add(NFTA_CMP_OP))) != NFT_CMP_EQ;
    (*priv_).len = desc.len;
    0
}

unsafe extern "C" fn nft_cmp16_fast_offload(ctx: *mut nft_offload_ctx, flow: *mut nft_flow_rule, expr: *const nft_expr) -> i32 {
    let p = nft_expr_priv(expr) as *const nft_cmp16_fast_expr;
    let cmp = nft_cmp_expr { data: (*p).data, sreg: (*p).sreg, len: (*p).len, op: if (*p).inv { NFT_CMP_NEQ } else { NFT_CMP_EQ } };
    __nft_cmp_offload(ctx, flow, &cmp)
}

unsafe extern "C" fn nft_cmp16_fast_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> i32 {
    let p = nft_expr_priv(expr) as *const nft_cmp16_fast_expr;
    let op = if (*p).inv { NFT_CMP_NEQ } else { NFT_CMP_EQ };
    if nft_dump_register(skb, NFTA_CMP_SREG, (*p).sreg) != 0 || nla_put_be32(skb, NFTA_CMP_OP, htonl(op)) != 0 { return -1; }
    if nft_data_dump(skb, NFTA_CMP_DATA, &(*p).data, NFT_DATA_VALUE, (*p).len) < 0 { return -1; }
    0
}

pub static nft_cmp16_fast_ops: nft_expr_ops = nft_expr_ops { type_: &nft_cmp_type, size: NFT_EXPR_SIZE(core::mem::size_of::<nft_cmp16_fast_expr>()), eval: None, init: Some(nft_cmp16_fast_init), dump: Some(nft_cmp16_fast_dump), offload: Some(nft_cmp16_fast_offload) };

unsafe extern "C" fn nft_cmp_select_ops(ctx: *const nft_ctx, tb: *const *const nlattr) -> *const nft_expr_ops {
    if (*tb.add(NFTA_CMP_SREG)).is_null() || (*tb.add(NFTA_CMP_OP)).is_null() || (*tb.add(NFTA_CMP_DATA)).is_null() { return ERR_PTR(-EINVAL); }
    let op = ntohl(nla_get_be32(*tb.add(NFTA_CMP_OP)));
    match op { NFT_CMP_EQ | NFT_CMP_NEQ | NFT_CMP_LT | NFT_CMP_LTE | NFT_CMP_GT | NFT_CMP_GTE => {}, _ => return ERR_PTR(-EINVAL) }
    let mut data = core::mem::zeroed::<nft_data>();
    let mut desc = nft_data_desc { type_: NFT_DATA_VALUE, size: core::mem::size_of::<nft_data>() };
    let err = nft_data_init(core::ptr::null_mut(), &mut data, &mut desc, *tb.add(NFTA_CMP_DATA));
    if err < 0 { return ERR_PTR(err); }
    let sreg = ntohl(nla_get_be32(*tb.add(NFTA_CMP_SREG))) as u8;
    if op == NFT_CMP_EQ || op == NFT_CMP_NEQ {
        if desc.len <= core::mem::size_of::<u32>() as u8 { return &nft_cmp_fast_ops; }
        if desc.len <= core::mem::size_of::<nft_data>() as u8 && ((sreg >= NFT_REG_1 && sreg <= NFT_REG_4) || (sreg >= NFT_REG32_00 && sreg <= NFT_REG32_12 && sreg % 2 == 0)) { return &nft_cmp16_fast_ops; }
    }
    &nft_cmp_ops
}

pub static nft_cmp_ops: nft_expr_ops = nft_expr_ops { type_: &nft_cmp_type, size: NFT_EXPR_SIZE(core::mem::size_of::<nft_cmp_expr>()), eval: Some(nft_cmp_eval), init: Some(nft_cmp_init), dump: Some(nft_cmp_dump), offload: Some(nft_cmp_offload) };

#[no_mangle]
pub static mut nft_cmp_type: nft_expr_type = nft_expr_type { name: "cmp", select_ops: Some(nft_cmp_select_ops), policy: &nft_cmp_policy, maxattr: NFTA_CMP_MAX, owner: THIS_MODULE };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
