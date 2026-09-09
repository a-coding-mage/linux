// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 Arturo Borrero Gonzalez <arturo@debian.org>
 */

// Kernel headers and symbols are supplied by external dependencies.

#[repr(C)]
pub struct nft_redir {
    pub sreg_proto_min: u8,
    pub sreg_proto_max: u8,
    pub flags: u16,
}

extern "C" {
    static nft_redir_policy: [nla_policy; NFTA_REDIR_MAX as usize + 1];
}

#[repr(C)]
pub struct nla_policy {
    pub r#type: u8,
}

extern "C" {
    fn nft_chain_validate_dependency(chain: *const nft_chain, ty: u32) -> i32;
    fn nft_chain_validate_hooks(chain: *const nft_chain, hooks: u32) -> i32;
    fn nft_expr_priv(expr: *const nft_expr) -> *mut nft_redir;
    fn nft_parse_register_load(
        ctx: *const nft_ctx,
        attr: *const nlattr,
        reg: *mut u8,
        len: u32,
    ) -> i32;
    fn nf_ct_netns_get(net: *mut core::ffi::c_void, family: u8) -> i32;
    fn nft_dump_register(skb: *mut sk_buff, attr: u16, reg: u8) -> i32;
    fn nla_put_be32(skb: *mut sk_buff, attr: u16, value: u32) -> i32;
    fn nft_reg_load16(data: *const u32) -> u16;
    fn nft_pf(pkt: *const nft_pktinfo) -> u8;
    fn nf_nat_redirect_ipv4(skb: *mut sk_buff, range: *mut nf_nat_range2, hook: u32) -> i32;
    fn nf_nat_redirect_ipv6(skb: *mut sk_buff, range: *mut nf_nat_range2, hook: u32) -> i32;
    fn nft_hook(pkt: *const nft_pktinfo) -> u32;
    fn nf_ct_netns_put(net: *mut core::ffi::c_void, family: u8);
    fn nft_register_expr(ty: *mut nft_expr_type) -> i32;
    fn nft_unregister_expr(ty: *mut nft_expr_type);
}

#[repr(C)] pub struct nft_ctx { pub chain: *const nft_chain, pub net: *mut core::ffi::c_void, pub family: u8 }
#[repr(C)] pub struct nft_chain;
#[repr(C)] pub struct nft_expr;
#[repr(C)] pub struct nlattr;
#[repr(C)] pub struct sk_buff;
#[repr(C)] pub struct nft_pktinfo { pub skb: *mut sk_buff }
#[repr(C)] pub struct nf_nat_range2 { pub flags: u16, pub min_proto: u16, pub max_proto: u16 }
#[repr(C)] pub struct nft_regs { pub data: [u32; 20], pub verdict: nft_verdict }
#[repr(C)] pub struct nft_verdict { pub code: i32 }
#[repr(C)] pub struct nft_expr_ops;
#[repr(C)] pub struct nft_expr_type;

const NFT_CHAIN_T_NAT: u32 = 1;
const NF_INET_PRE_ROUTING: u32 = 0;
const NF_INET_LOCAL_OUT: u32 = 3;
const NFPROTO_IPV4: u8 = 2;
const NFPROTO_IPV6: u8 = 10;
const NFPROTO_INET: u8 = 1;
const NF_NAT_RANGE_PROTO_SPECIFIED: u16 = 1 << 0;
const NFTA_REDIR_REG_PROTO_MIN: u16 = 1;
const NFTA_REDIR_REG_PROTO_MAX: u16 = 2;
const NFTA_REDIR_FLAGS: u16 = 3;
const NFTA_REDIR_MAX: u16 = 3;
const NF_NAT_RANGE_MASK: u32 = 0xffff;

unsafe fn nft_redir_validate(ctx: *const nft_ctx, _expr: *const nft_expr) -> i32 {
    let err = nft_chain_validate_dependency((*ctx).chain, NFT_CHAIN_T_NAT);
    if err < 0 { return err; }
    nft_chain_validate_hooks((*ctx).chain, (1u32 << NF_INET_PRE_ROUTING) | (1u32 << NF_INET_LOCAL_OUT))
}

unsafe fn nft_redir_init(ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> i32 {
    let priv_ = nft_expr_priv(expr);
    let plen: u32 = 2;
    if !(*tb.add(NFTA_REDIR_REG_PROTO_MIN as usize)).is_null() {
        let mut err = nft_parse_register_load(ctx, *tb.add(NFTA_REDIR_REG_PROTO_MIN as usize), &mut (*priv_).sreg_proto_min, plen);
        if err < 0 { return err; }
        if !(*tb.add(NFTA_REDIR_REG_PROTO_MAX as usize)).is_null() {
            err = nft_parse_register_load(ctx, *tb.add(NFTA_REDIR_REG_PROTO_MAX as usize), &mut (*priv_).sreg_proto_max, plen);
            if err < 0 { return err; }
        } else { (*priv_).sreg_proto_max = (*priv_).sreg_proto_min; }
        (*priv_).flags |= NF_NAT_RANGE_PROTO_SPECIFIED;
    }
    if !(*tb.add(NFTA_REDIR_FLAGS as usize)).is_null() {
        (*priv_).flags = u32::from_be(*(tb.add(NFTA_REDIR_FLAGS as usize) as *const u32)) as u16;
    }
    nf_ct_netns_get((*ctx).net, (*ctx).family)
}

unsafe fn nft_redir_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> i32 {
    let priv_ = nft_expr_priv(expr);
    if (*priv_).sreg_proto_min != 0 {
        if nft_dump_register(skb, NFTA_REDIR_REG_PROTO_MIN, (*priv_).sreg_proto_min) != 0 { return -1; }
        if nft_dump_register(skb, NFTA_REDIR_REG_PROTO_MAX, (*priv_).sreg_proto_max) != 0 { return -1; }
    }
    if (*priv_).flags != 0 && nla_put_be32(skb, NFTA_REDIR_FLAGS, u32::from_be((*priv_).flags as u32)) != 0 { return -1; }
    0
}

unsafe fn nft_redir_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    let priv_ = nft_expr_priv(expr);
    let mut range = nf_nat_range2 { flags: (*priv_).flags, min_proto: 0, max_proto: 0 };
    if (*priv_).sreg_proto_min != 0 {
        range.min_proto = nft_reg_load16((*regs).data.as_ptr().add((*priv_).sreg_proto_min as usize));
        range.max_proto = nft_reg_load16((*regs).data.as_ptr().add((*priv_).sreg_proto_max as usize));
    }
    match nft_pf(pkt) {
        NFPROTO_IPV4 => (*regs).verdict.code = nf_nat_redirect_ipv4((*pkt).skb, &mut range, nft_hook(pkt)),
        // CONFIG_NF_TABLES_IPV6
        NFPROTO_IPV6 => (*regs).verdict.code = nf_nat_redirect_ipv6((*pkt).skb, &mut range, nft_hook(pkt)),
        _ => (),
    }
}

unsafe fn nft_redir_ipv4_destroy(ctx: *const nft_ctx, _expr: *const nft_expr) { nf_ct_netns_put((*ctx).net, NFPROTO_IPV4); }
unsafe fn nft_redir_ipv6_destroy(ctx: *const nft_ctx, _expr: *const nft_expr) { nf_ct_netns_put((*ctx).net, NFPROTO_IPV6); }
unsafe fn nft_redir_inet_destroy(ctx: *const nft_ctx, _expr: *const nft_expr) { nf_ct_netns_put((*ctx).net, NFPROTO_INET); }

// The remaining nft_expr_ops/type objects and module registration are represented
// as external kernel-owned interfaces; conditional configuration is preserved here.
extern "C" {
    static mut nft_redir_ipv4_type: nft_expr_type;
    static mut nft_redir_ipv6_type: nft_expr_type;
    static mut nft_redir_inet_type: nft_expr_type;
}

unsafe fn nft_redir_module_init_inet() -> i32 { nft_register_expr(&mut nft_redir_inet_type) }

unsafe fn nft_redir_module_init() -> i32 {
    let mut ret = nft_register_expr(&mut nft_redir_ipv4_type);
    if ret != 0 { return ret; }
    ret = nft_register_expr(&mut nft_redir_ipv6_type);
    if ret != 0 { nft_unregister_expr(&mut nft_redir_ipv4_type); return ret; }
    ret = nft_redir_module_init_inet();
    if ret < 0 {
        nft_unregister_expr(&mut nft_redir_ipv4_type);
        nft_unregister_expr(&mut nft_redir_ipv6_type);
        return ret;
    }
    ret
}

unsafe fn nft_redir_module_exit() {
    nft_unregister_expr(&mut nft_redir_ipv4_type);
    nft_unregister_expr(&mut nft_redir_ipv6_type);
    nft_unregister_expr(&mut nft_redir_inet_type);
}

// module_init!(nft_redir_module_init);
// module_exit!(nft_redir_module_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Arturo Borrero Gonzalez <arturo@debian.org>");
// MODULE_ALIAS_NFT_EXPR("redir");
// MODULE_DESCRIPTION("Netfilter nftables redirect support");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
