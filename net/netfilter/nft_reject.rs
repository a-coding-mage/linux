// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2008-2009 Patrick McHardy <kaber@trash.net>
 * Copyright (c) 2013 Eric Leblond <eric@regit.org>
 *
 * Development of this code funded by Astaro AG (http://www.astaro.com/)
 */

// Dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct nla_policy {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nft_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nft_expr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nlattr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nft_reject {
    pub type_: u32,
    pub icmp_code: u8,
}

extern "C" {
    pub static nft_reject_policy: [nla_policy; NFTA_REJECT_MAX as usize + 1];
    fn nft_chain_validate_hooks(chain: *const core::ffi::c_void, hooks: u32) -> i32;
    fn nft_expr_priv(expr: *const nft_expr) -> *mut nft_reject;
    fn nla_get_be32(attr: *const nlattr) -> u32;
    fn nla_get_u8(attr: *const nlattr) -> u8;
    fn ntohl(value: u32) -> u32;
    fn htonl(value: u32) -> u32;
    fn nla_put_be32(skb: *mut sk_buff, attr_type: u16, value: u32) -> i32;
    fn nla_put_u8(skb: *mut sk_buff, attr_type: u16, value: u8) -> i32;
    fn debug_net_warn_on_once(value: i32);
}

pub const NFTA_REJECT_MAX: u32 = 2;
pub const NFTA_REJECT_TYPE: u16 = 1;
pub const NFTA_REJECT_ICMP_CODE: u16 = 2;
pub const NFT_REJECT_ICMP_UNREACH: u32 = 0;
pub const NFT_REJECT_ICMPX_UNREACH: u32 = 1;
pub const NFT_REJECT_TCP_RST: u32 = 2;
pub const NFT_REJECT_ICMPX_MAX: u8 = 3;
pub const NF_INET_LOCAL_IN: u32 = 1;
pub const NF_INET_FORWARD: u32 = 2;
pub const NF_INET_LOCAL_OUT: u32 = 3;
pub const NF_INET_PRE_ROUTING: u32 = 0;
pub const ICMP_NET_UNREACH: u8 = 0;
pub const ICMP_PORT_UNREACH: u8 = 3;
pub const ICMP_HOST_UNREACH: u8 = 1;
pub const ICMP_PKT_FILTERED: u8 = 13;
pub const ICMPV6_NOROUTE: u8 = 0;
pub const ICMPV6_PORT_UNREACH: u8 = 4;
pub const ICMPV6_ADDR_UNREACH: u8 = 3;
pub const ICMPV6_ADM_PROHIBITED: u8 = 1;

pub fn nft_reject_validate(ctx: *const nft_ctx, _expr: *const nft_expr) -> i32 {
    unsafe {
        nft_chain_validate_hooks(ctx as *const core::ffi::c_void, (1 << NF_INET_LOCAL_IN) | (1 << NF_INET_FORWARD) | (1 << NF_INET_LOCAL_OUT) | (1 << NF_INET_PRE_ROUTING))
    }
}

pub unsafe fn nft_reject_init(ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> i32 {
    let priv_ = &mut *nft_expr_priv(expr);
    let _ = ctx;
    if (*tb.add(NFTA_REJECT_TYPE as usize)).is_null() { return -22; }
    priv_.type_ = ntohl(nla_get_be32(*tb.add(NFTA_REJECT_TYPE as usize)));
    match priv_.type_ {
        NFT_REJECT_ICMP_UNREACH | NFT_REJECT_ICMPX_UNREACH => {
            if (*tb.add(NFTA_REJECT_ICMP_CODE as usize)).is_null() { return -22; }
            let icmp_code = nla_get_u8(*tb.add(NFTA_REJECT_ICMP_CODE as usize));
            if priv_.type_ == NFT_REJECT_ICMPX_UNREACH && icmp_code > NFT_REJECT_ICMPX_MAX { return -22; }
            priv_.icmp_code = icmp_code;
        }
        NFT_REJECT_TCP_RST => {}
        _ => return -22,
    }
    0
}

pub unsafe fn nft_reject_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> i32 {
    let priv_ = &*nft_expr_priv(expr);
    if nla_put_be32(skb, NFTA_REJECT_TYPE, htonl(priv_.type_)) != 0 { return -1; }
    match priv_.type_ {
        NFT_REJECT_ICMP_UNREACH | NFT_REJECT_ICMPX_UNREACH => {
            if nla_put_u8(skb, NFTA_REJECT_ICMP_CODE, priv_.icmp_code) != 0 { return -1; }
        }
        _ => {}
    }
    0
}

static mut icmp_code_v4: [u8; NFT_REJECT_ICMPX_MAX as usize + 1] = [
    ICMP_NET_UNREACH, ICMP_PORT_UNREACH, ICMP_HOST_UNREACH, ICMP_PKT_FILTERED,
];
static mut icmp_code_v6: [u8; NFT_REJECT_ICMPX_MAX as usize + 1] = [
    ICMPV6_NOROUTE, ICMPV6_PORT_UNREACH, ICMPV6_ADDR_UNREACH, ICMPV6_ADM_PROHIBITED,
];

pub fn nft_reject_icmp_code(code: u8) -> u8 {
    if code > NFT_REJECT_ICMPX_MAX { unsafe { debug_net_warn_on_once(1); } return ICMP_NET_UNREACH; }
    unsafe { icmp_code_v4[code as usize] }
}

pub fn nft_reject_icmpv6_code(code: u8) -> u8 {
    if code > NFT_REJECT_ICMPX_MAX { unsafe { debug_net_warn_on_once(1); } return ICMPV6_NOROUTE; }
    unsafe { icmp_code_v6[code as usize] }
}

// EXPORT_SYMBOL_GPL(nft_reject_policy);
// EXPORT_SYMBOL_GPL(nft_reject_validate);
// EXPORT_SYMBOL_GPL(nft_reject_init);
// EXPORT_SYMBOL_GPL(nft_reject_dump);
// EXPORT_SYMBOL_GPL(nft_reject_icmp_code);
// EXPORT_SYMBOL_GPL(nft_reject_icmpv6_code);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Patrick McHardy <kaber@trash.net>");
// MODULE_DESCRIPTION("Netfilter x_tables over nftables module");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
