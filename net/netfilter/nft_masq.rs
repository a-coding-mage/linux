// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 Arturo Borrero Gonzalez <arturo@debian.org>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
struct nft_masq {
    flags: u32,
    sreg_proto_min: u8,
    sreg_proto_max: u8,
}

static mut nft_masq_policy: [nla_policy; NFTA_MASQ_MAX as usize + 1] = [nla_policy { type_: 0 }; NFTA_MASQ_MAX as usize + 1];

// The expression operation/type layouts and constants are provided by the kernel headers.
extern "C" {
    static mut nft_masq_ipv4_type: nft_expr_type;
    static mut nft_masq_ipv6_type: nft_expr_type;
    static mut nft_masq_inet_type: nft_expr_type;
}

unsafe fn nft_masq_validate(ctx: *const nft_ctx, _expr: *const nft_expr) -> i32 {
    let err = nft_chain_validate_dependency((*ctx).chain, NFT_CHAIN_T_NAT);
    if err < 0 { return err; }
    nft_chain_validate_hooks((*ctx).chain, 1 << NF_INET_POST_ROUTING)
}

unsafe fn nft_masq_init(ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> i32 {
    let plen = core::mem::size_of::<__be16>();
    let priv_ = nft_expr_priv(expr) as *mut nft_masq;
    let mut err: i32;
    if !(*tb.add(NFTA_MASQ_FLAGS as usize)).is_null() {
        (*priv_).flags = ntohl(nla_get_be32(*tb.add(NFTA_MASQ_FLAGS as usize)));
    }
    if !(*tb.add(NFTA_MASQ_REG_PROTO_MIN as usize)).is_null() {
        err = nft_parse_register_load(ctx, *tb.add(NFTA_MASQ_REG_PROTO_MIN as usize), &mut (*priv_).sreg_proto_min, plen);
        if err < 0 { return err; }
        if !(*tb.add(NFTA_MASQ_REG_PROTO_MAX as usize)).is_null() {
            err = nft_parse_register_load(ctx, *tb.add(NFTA_MASQ_REG_PROTO_MAX as usize), &mut (*priv_).sreg_proto_max, plen);
            if err < 0 { return err; }
        } else { (*priv_).sreg_proto_max = (*priv_).sreg_proto_min; }
    }
    nf_ct_netns_get((*ctx).net, (*ctx).family)
}

unsafe fn nft_masq_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> i32 {
    let priv_ = nft_expr_priv(expr) as *const nft_masq;
    if (*priv_).flags != 0 && nla_put_be32(skb, NFTA_MASQ_FLAGS, htonl((*priv_).flags)) != 0 { return -1; }
    if (*priv_).sreg_proto_min != 0 &&
        (nft_dump_register(skb, NFTA_MASQ_REG_PROTO_MIN, (*priv_).sreg_proto_min) != 0 ||
         nft_dump_register(skb, NFTA_MASQ_REG_PROTO_MAX, (*priv_).sreg_proto_max) != 0) { return -1; }
    0
}

unsafe fn nft_masq_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    let priv_ = nft_expr_priv(expr) as *const nft_masq;
    let mut range: nf_nat_range2 = core::mem::zeroed();
    (*(&mut range as *mut _)).flags = (*priv_).flags;
    if (*priv_).sreg_proto_min != 0 {
        range.min_proto.all = nft_reg_load16((*regs).data.as_ptr().add((*priv_).sreg_proto_min as usize)) as __be16;
        range.max_proto.all = nft_reg_load16((*regs).data.as_ptr().add((*priv_).sreg_proto_max as usize)) as __be16;
    }
    match nft_pf(pkt) {
        NFPROTO_IPV4 => (*regs).verdict.code = nf_nat_masquerade_ipv4((*pkt).skb, nft_hook(pkt), &range, nft_out(pkt)),
        NFPROTO_IPV6 => (*regs).verdict.code = nf_nat_masquerade_ipv6((*pkt).skb, &range, nft_out(pkt)),
        _ => DEBUG_NET_WARN_ON_ONCE(1),
    }
}

unsafe fn nft_masq_ipv4_destroy(ctx: *const nft_ctx, _expr: *const nft_expr) { nf_ct_netns_put((*ctx).net, NFPROTO_IPV4); }
unsafe fn nft_masq_ipv6_destroy(ctx: *const nft_ctx, _expr: *const nft_expr) { nf_ct_netns_put((*ctx).net, NFPROTO_IPV6); }
unsafe fn nft_masq_inet_destroy(ctx: *const nft_ctx, _expr: *const nft_expr) { nf_ct_netns_put((*ctx).net, NFPROTO_INET); }

// CONFIG_NF_TABLES_IPV6 selects the real IPv6 registration; otherwise these are inline no-ops.
unsafe fn nft_masq_module_init_ipv6() -> i32 { nft_register_expr(&mut nft_masq_ipv6_type) }
unsafe fn nft_masq_module_exit_ipv6() { nft_unregister_expr(&mut nft_masq_ipv6_type); }
// CONFIG_NF_TABLES_INET selects the real inet registration; otherwise these are inline no-ops.
unsafe fn nft_masq_module_init_inet() -> i32 { nft_register_expr(&mut nft_masq_inet_type) }
unsafe fn nft_masq_module_exit_inet() { nft_unregister_expr(&mut nft_masq_inet_type); }

unsafe fn nft_masq_module_init() -> i32 {
    let mut ret = nft_masq_module_init_ipv6();
    if ret < 0 { return ret; }
    ret = nft_masq_module_init_inet();
    if ret < 0 { nft_masq_module_exit_ipv6(); return ret; }
    ret = nft_register_expr(&mut nft_masq_ipv4_type);
    if ret < 0 { nft_masq_module_exit_inet(); nft_masq_module_exit_ipv6(); return ret; }
    ret = nf_nat_masquerade_inet_register_notifiers();
    if ret < 0 { nft_masq_module_exit_ipv6(); nft_masq_module_exit_inet(); nft_unregister_expr(&mut nft_masq_ipv4_type); return ret; }
    ret
}

unsafe fn nft_masq_module_exit() {
    nft_masq_module_exit_ipv6();
    nft_masq_module_exit_inet();
    nft_unregister_expr(&mut nft_masq_ipv4_type);
    nf_nat_masquerade_inet_unregister_notifiers();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
