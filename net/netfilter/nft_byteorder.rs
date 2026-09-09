// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2008-2009 Patrick McHardy <kaber@trash.net>
 *
 * Development of this code funded by Astaro AG (http://www.astaro.com/)
 */

// Kernel dependencies supplied by other translation units.

#[repr(C)]
pub struct nft_byteorder {
    pub sreg: u8,
    pub dreg: u8,
    pub op: nft_byteorder_ops,
    pub size: u8,
}

pub unsafe extern "C" fn nft_byteorder_eval(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    _pkt: *const nft_pktinfo,
) {
    let priv_ = nft_expr_priv(expr);
    let src = (*regs).data.as_ptr().add((*priv_).sreg as usize);
    let dst = (*regs).data.as_mut_ptr().add((*priv_).dreg as usize);

    match (*priv_).size {
        8 => {
            let dst64 = dst as *mut u64;
            let src64: u64;
            match (*priv_).op {
                NFT_BYTEORDER_NTOH => {
                    src64 = nft_reg_load64(src);
                    nft_reg_store64(dst64, be64_to_cpu(src64));
                }
                NFT_BYTEORDER_HTON => {
                    src64 = cpu_to_be64(nft_reg_load64(src));
                    nft_reg_store64(dst64, src64);
                }
                _ => {}
            }
        }
        4 => match (*priv_).op {
            NFT_BYTEORDER_NTOH => *dst = ntohl(*src),
            NFT_BYTEORDER_HTON => *dst = htonl(*src),
            _ => {}
        },
        2 => match (*priv_).op {
            NFT_BYTEORDER_NTOH => nft_reg_store16(dst, ntohs(nft_reg_load_be16(src))),
            NFT_BYTEORDER_HTON => nft_reg_store_be16(dst, htons(nft_reg_load16(src))),
            _ => {}
        },
        _ => {}
    }
}

static nft_byteorder_policy: [nla_policy; NFTA_BYTEORDER_MAX as usize + 1] = [
    /* NFTA_BYTEORDER_SREG = NLA_POLICY_MAX(NLA_BE32, NFT_REG32_MAX) */
    /* NFTA_BYTEORDER_DREG = NLA_POLICY_MAX(NLA_BE32, NFT_REG32_MAX) */
    /* NFTA_BYTEORDER_OP   = NLA_POLICY_MAX(NLA_BE32, 255) */
    /* NFTA_BYTEORDER_LEN  = NLA_POLICY_MAX(NLA_BE32, 255) */
    /* NFTA_BYTEORDER_SIZE = NLA_POLICY_MAX(NLA_BE32, 255) */
    nla_policy::default(); NFTA_BYTEORDER_MAX as usize + 1
];

unsafe extern "C" fn nft_byteorder_init(
    ctx: *const nft_ctx,
    expr: *const nft_expr,
    tb: *const *const nlattr,
) -> c_int {
    let priv_ = nft_expr_priv(expr);
    let mut size: u32 = 0;
    let mut len: u32 = 0;
    let mut err: c_int;

    if (*tb.add(NFTA_BYTEORDER_SREG as usize)).is_null()
        || (*tb.add(NFTA_BYTEORDER_DREG as usize)).is_null()
        || (*tb.add(NFTA_BYTEORDER_LEN as usize)).is_null()
        || (*tb.add(NFTA_BYTEORDER_SIZE as usize)).is_null()
        || (*tb.add(NFTA_BYTEORDER_OP as usize)).is_null()
    { return -EINVAL; }

    (*priv_).op = ntohl(nla_get_be32(*tb.add(NFTA_BYTEORDER_OP as usize))) as nft_byteorder_ops;
    match (*priv_).op {
        NFT_BYTEORDER_NTOH | NFT_BYTEORDER_HTON => {}
        _ => return -EINVAL,
    }
    err = nft_parse_u32_check(*tb.add(NFTA_BYTEORDER_SIZE as usize), U8_MAX, &mut size);
    if err < 0 { return err; }
    (*priv_).size = size as u8;
    match (*priv_).size { 2 | 4 | 8 => {}, _ => return -EINVAL }

    err = nft_parse_u32_check(*tb.add(NFTA_BYTEORDER_LEN as usize), U8_MAX, &mut len);
    if err < 0 { return err; }
    /* no longer support multi-reg conversions */
    if len != size { return -EOPNOTSUPP; }
    err = nft_parse_register_load(ctx, *tb.add(NFTA_BYTEORDER_SREG as usize), &mut (*priv_).sreg, len);
    if err < 0 { return err; }
    err = nft_parse_register_store(ctx, *tb.add(NFTA_BYTEORDER_DREG as usize), &mut (*priv_).dreg, core::ptr::null_mut(), NFT_DATA_VALUE, len);
    if err < 0 { return err; }
    if nft_reg_overlap((*priv_).sreg, (*priv_).dreg, len) { return -EINVAL; }
    0
}

unsafe extern "C" fn nft_byteorder_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> c_int {
    let priv_ = nft_expr_priv(expr);
    if nft_dump_register(skb, NFTA_BYTEORDER_SREG, (*priv_).sreg) != 0 { return -1; }
    if nft_dump_register(skb, NFTA_BYTEORDER_DREG, (*priv_).dreg) != 0 { return -1; }
    if nla_put_be32(skb, NFTA_BYTEORDER_OP, htonl((*priv_).op as u32)) != 0 { return -1; }
    if nla_put_be32(skb, NFTA_BYTEORDER_SIZE, htonl((*priv_).size as u32)) != 0 { return -1; }
    /* compatibility for old userspace which permitted size != len */
    if nla_put_be32(skb, NFTA_BYTEORDER_LEN, htonl((*priv_).size as u32)) != 0 { return -1; }
    0
}

static nft_byteorder_ops: nft_expr_ops = nft_expr_ops {
    type_: &nft_byteorder_type,
    size: NFT_EXPR_SIZE(core::mem::size_of::<nft_byteorder>()),
    eval: nft_byteorder_eval,
    init: nft_byteorder_init,
    dump: nft_byteorder_dump,
};

pub static mut nft_byteorder_type: nft_expr_type = nft_expr_type {
    name: "byteorder",
    ops: &nft_byteorder_ops,
    policy: &nft_byteorder_policy,
    maxattr: NFTA_BYTEORDER_MAX,
    owner: THIS_MODULE,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
