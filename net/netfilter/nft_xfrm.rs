// SPDX-License-Identifier: GPL-2.0-only
//
// Generic part shared by ipv4 and ipv6 backends.
//
// C header dependencies are supplied by the surrounding kernel translation.

#[repr(C)]
pub struct NlaPolicy {
    pub kind: u32,
    pub maximum: u32,
}

// enum nft_xfrm_keys and the kernel constants/types below are supplied by
// the corresponding translated kernel headers.

#[repr(C)]
pub struct NftXfrm {
    pub key: u8,
    pub dreg: u8,
    pub dir: u8,
    pub spnum: u8,
    pub len: u8,
}

static mut NFT_XFRM_POLICY: [NlaPolicy; NFTA_XFRM_MAX as usize + 1] = [
    NlaPolicy { kind: NLA_BE32, maximum: 255 },
    NlaPolicy { kind: NLA_U8, maximum: XFRM_POLICY_OUT as u32 },
    NlaPolicy { kind: NLA_BE32, maximum: (XFRM_MAX_DEPTH - 1) as u32 },
    NlaPolicy { kind: NLA_BE32, maximum: NFT_REG32_MAX as u32 },
];

unsafe fn nft_xfrm_get_init(
    ctx: *const NftCtx,
    expr: *const NftExpr,
    tb: *const *const Nlattr,
) -> i32 {
    let priv_ = nft_expr_priv(expr) as *mut NftXfrm;
    let mut len: u32 = 0;
    let mut spnum: u32 = 0;
    let dir: u8;

    if (*tb.add(NFTA_XFRM_KEY as usize)).is_null()
        || (*tb.add(NFTA_XFRM_DIR as usize)).is_null()
        || (*tb.add(NFTA_XFRM_DREG as usize)).is_null()
    {
        return -EINVAL;
    }

    match (*ctx).family {
        NFPROTO_IPV4 | NFPROTO_IPV6 | NFPROTO_INET => {}
        _ => return -EOPNOTSUPP,
    }

    (*priv_).key = ntohl(nla_get_be32(*tb.add(NFTA_XFRM_KEY as usize))) as u8;
    match (*priv_).key as u32 {
        NFT_XFRM_KEY_REQID | NFT_XFRM_KEY_SPI => len = core::mem::size_of::<u32>() as u32,
        NFT_XFRM_KEY_DADDR_IP4 | NFT_XFRM_KEY_SADDR_IP4 => {
            len = core::mem::size_of::<InAddr>() as u32
        }
        NFT_XFRM_KEY_DADDR_IP6 | NFT_XFRM_KEY_SADDR_IP6 => {
            len = core::mem::size_of::<In6Addr>() as u32
        }
        _ => return -EINVAL,
    }

    dir = nla_get_u8(*tb.add(NFTA_XFRM_DIR as usize));
    match dir as u32 {
        XFRM_POLICY_IN | XFRM_POLICY_OUT => (*priv_).dir = dir,
        _ => return -EINVAL,
    }

    if !(*tb.add(NFTA_XFRM_SPNUM as usize)).is_null() {
        spnum = ntohl(nla_get_be32(*tb.add(NFTA_XFRM_SPNUM as usize)));
    }
    if spnum >= XFRM_MAX_DEPTH as u32 {
        return -ERANGE;
    }
    (*priv_).spnum = spnum as u8;
    (*priv_).len = len as u8;
    nft_parse_register_store(
        ctx,
        *tb.add(NFTA_XFRM_DREG as usize),
        &mut (*priv_).dreg,
        core::ptr::null_mut(),
        NFT_DATA_VALUE,
        len,
    )
}

unsafe fn xfrm_state_addr_ok(k: u32, family: u8, mode: u8) -> bool {
    match k {
        NFT_XFRM_KEY_DADDR_IP4 | NFT_XFRM_KEY_SADDR_IP4 if family != NFPROTO_IPV4 => return false,
        NFT_XFRM_KEY_DADDR_IP6 | NFT_XFRM_KEY_SADDR_IP6 if family != NFPROTO_IPV6 => return false,
        NFT_XFRM_KEY_DADDR_IP4 | NFT_XFRM_KEY_SADDR_IP4
        | NFT_XFRM_KEY_DADDR_IP6 | NFT_XFRM_KEY_SADDR_IP6 => {}
        _ => return true,
    }
    mode == XFRM_MODE_BEET || mode == XFRM_MODE_TUNNEL || mode == XFRM_MODE_IPTFS
}

unsafe fn nft_xfrm_state_get_key(priv_: *const NftXfrm, regs: *mut NftRegs, state: *const XfrmState) {
    let dest = (*regs).data.as_mut_ptr().add((*priv_).dreg as usize);
    if !xfrm_state_addr_ok((*priv_).key as u32, (*state).props.family, (*state).props.mode) {
        (*regs).verdict.code = NFT_BREAK;
        return;
    }
    match (*priv_).key as u32 {
        NFT_XFRM_KEY_UNSPEC | __NFT_XFRM_KEY_MAX => {
            DEBUG_NET_WARN_ON_ONCE(1);
        }
        NFT_XFRM_KEY_DADDR_IP4 => { *dest = (*state).id.daddr.a4; return; }
        NFT_XFRM_KEY_DADDR_IP6 => { memcpy(dest, &(*state).id.daddr.in6 as *const _ as *const u8, core::mem::size_of::<In6Addr>()); return; }
        NFT_XFRM_KEY_SADDR_IP4 => { *dest = (*state).props.saddr.a4; return; }
        NFT_XFRM_KEY_SADDR_IP6 => { memcpy(dest, &(*state).props.saddr.in6 as *const _ as *const u8, core::mem::size_of::<In6Addr>()); return; }
        NFT_XFRM_KEY_REQID => { *dest = (*state).props.reqid; return; }
        NFT_XFRM_KEY_SPI => { *dest = (*state).id.spi; return; }
        _ => {}
    }
    (*regs).verdict.code = NFT_BREAK;
}

unsafe fn nft_xfrm_get_eval_in(priv_: *const NftXfrm, regs: *mut NftRegs, pkt: *const NftPktinfo) {
    let sp = skb_sec_path((*pkt).skb);
    if sp.is_null() || (*sp).len <= (*priv_).spnum as u32 { (*regs).verdict.code = NFT_BREAK; return; }
    nft_xfrm_state_get_key(priv_, regs, *(*sp).xvec.add((*priv_).spnum as usize));
}

unsafe fn nft_xfrm_get_eval_out(priv_: *const NftXfrm, regs: *mut NftRegs, pkt: *const NftPktinfo) {
    if !skb_valid_dst((*pkt).skb) { (*regs).verdict.code = NFT_BREAK; return; }
    let mut dst = skb_dst((*pkt).skb);
    let mut i = 0u32;
    while !dst.is_null() && !(*dst).xfrm.is_null() {
        if i >= (*priv_).spnum as u32 { nft_xfrm_state_get_key(priv_, regs, (*dst).xfrm); return; }
        dst = (*(dst as *const XfrmDst)).child; i += 1;
    }
    (*regs).verdict.code = NFT_BREAK;
}

unsafe fn nft_xfrm_get_eval(expr: *const NftExpr, regs: *mut NftRegs, pkt: *const NftPktinfo) {
    let priv_ = nft_expr_priv(expr) as *const NftXfrm;
    match (*priv_).dir as u32 { XFRM_POLICY_IN => nft_xfrm_get_eval_in(priv_, regs, pkt), XFRM_POLICY_OUT => nft_xfrm_get_eval_out(priv_, regs, pkt), _ => { DEBUG_NET_WARN_ON_ONCE(1); (*regs).verdict.code = NFT_BREAK; } }
}

unsafe fn nft_xfrm_get_dump(skb: *mut SkBuff, expr: *const NftExpr, _reset: bool) -> i32 {
    let p = nft_expr_priv(expr) as *const NftXfrm;
    if nft_dump_register(skb, NFTA_XFRM_DREG, (*p).dreg) != 0 || nla_put_be32(skb, NFTA_XFRM_KEY, htonl((*p).key as u32)) != 0 || nla_put_u8(skb, NFTA_XFRM_DIR, (*p).dir) != 0 || nla_put_be32(skb, NFTA_XFRM_SPNUM, htonl((*p).spnum as u32)) != 0 { return -1; }
    0
}

unsafe fn nft_xfrm_validate(ctx: *const NftCtx, expr: *const NftExpr) -> i32 {
    let priv_ = nft_expr_priv(expr) as *const NftXfrm;
    if (*ctx).family != NFPROTO_IPV4 && (*ctx).family != NFPROTO_IPV6 && (*ctx).family != NFPROTO_INET { return -EOPNOTSUPP; }
    let hooks = match (*priv_).dir as u32 {
        XFRM_POLICY_IN => (1 << NF_INET_FORWARD) | (1 << NF_INET_LOCAL_IN) | (1 << NF_INET_PRE_ROUTING),
        XFRM_POLICY_OUT => (1 << NF_INET_FORWARD) | (1 << NF_INET_LOCAL_OUT) | (1 << NF_INET_POST_ROUTING),
        _ => { DEBUG_NET_WARN_ON_ONCE(1); return -EINVAL; }
    };
    nft_chain_validate_hooks((*ctx).chain, hooks)
}

// Equivalent declarations for nft_xfrm_get_ops and nft_xfrm_type.  Their
// structure layouts and module registration are supplied by the kernel ABI.
extern "C" {
    static mut nft_xfrm_type: NftExprType;
    fn nft_register_expr(ty: *mut NftExprType) -> i32;
    fn nft_unregister_expr(ty: *mut NftExprType);
}

unsafe fn nft_xfrm_module_init() -> i32 { nft_register_expr(&raw mut nft_xfrm_type) }
unsafe fn nft_xfrm_module_exit() { nft_unregister_expr(&raw mut nft_xfrm_type); }

// module_init(nft_xfrm_module_init);
// module_exit(nft_xfrm_module_exit);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("nf_tables: xfrm/IPSec matching");
// MODULE_AUTHOR("Florian Westphal <fw@strlen.de>");
// MODULE_AUTHOR("Máté Eckl <ecklm94@gmail.com>");
// MODULE_ALIAS_NFT_EXPR("xfrm");

// Remaining kernel declarations, callback structure definitions, and module
// registration macros are provided by the translated kernel environment.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
