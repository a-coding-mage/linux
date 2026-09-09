// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of nft_compat.c; kernel dependencies are supplied externally. */

const NFT_MATCH_LARGE_THRESH: usize = 192;

#[repr(C)]
pub struct nft_xt_match_priv { pub info: *mut core::ffi::c_void }

unsafe fn nft_compat_chain_validate_dependency(ctx: *const nft_ctx, tablename: *const core::ffi::c_char) -> i32 {
    let mut ty = NFT_CHAIN_T_DEFAULT;
    let chain = (*ctx).chain;
    if tablename.is_null() || !nft_is_base_chain(chain) { return 0; }
    let basechain = nft_base_chain(chain);
    if strcmp(tablename, b"nat\0".as_ptr() as _) == 0 {
        if (*ctx).family != NFPROTO_BRIDGE { ty = NFT_CHAIN_T_NAT; }
        if (*(*basechain).type_).type_ != ty { return -EINVAL; }
    }
    0
}

#[repr(C)] pub union nft_entry { pub e4: ipt_entry, pub e6: ip6t_entry, pub ebt: ebt_entry, pub arp: arpt_entry }

unsafe fn nft_compat_set_par(par: *mut xt_action_param, pkt: *const nft_pktinfo, xt: *const core::ffi::c_void, info: *const core::ffi::c_void) {
    (*par).state = (*pkt).state; (*par).thoff = nft_thoff(pkt); (*par).fragoff = (*pkt).fragoff;
    (*par).target = xt; (*par).targinfo = info; (*par).hotdrop = false;
}

unsafe fn nft_target_eval_xt(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    let info = nft_expr_priv(expr); let target = (*(*expr).ops).data as *mut xt_target; let skb = (*pkt).skb;
    let mut xt: xt_action_param = core::mem::zeroed(); nft_compat_set_par(&mut xt, pkt, target as _, info);
    let mut ret = ((*target).target.unwrap())(skb, &mut xt); if xt.hotdrop { ret = NF_DROP; }
    (*regs).verdict.code = if ret == XT_CONTINUE { NFT_CONTINUE } else { ret };
}

unsafe fn nft_target_eval_bridge(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    let info = nft_expr_priv(expr); let target = (*(*expr).ops).data as *mut xt_target; let skb = (*pkt).skb;
    let mut xt: xt_action_param = core::mem::zeroed(); nft_compat_set_par(&mut xt, pkt, target as _, info);
    let mut ret = ((*target).target.unwrap())(skb, &mut xt); if xt.hotdrop { ret = NF_DROP; }
    (*regs).verdict.code = match ret { EBT_ACCEPT => NF_ACCEPT, EBT_DROP => NF_DROP, EBT_CONTINUE => NFT_CONTINUE, EBT_RETURN => NFT_RETURN, x => x };
}

static nft_target_policy: [nla_policy; NFTA_TARGET_MAX + 1] = [nla_policy::default(); NFTA_TARGET_MAX + 1];

unsafe fn nft_target_set_tgchk_param(par: *mut xt_tgchk_param, ctx: *const nft_ctx, target: *mut xt_target, info: *mut core::ffi::c_void, entry: *mut nft_entry, proto: u16, inv: bool) {
    (*par).net = (*ctx).net; (*par).table = (*(*ctx).table).name;
    match (*ctx).family { AF_INET => { (*entry).e4.ip.proto = proto; (*entry).e4.ip.invflags = if inv { IPT_INV_PROTO } else { 0 }; }, AF_INET6 => { if proto != 0 { (*entry).e6.ipv6.flags |= IP6T_F_PROTO; } (*entry).e6.ipv6.proto = proto; (*entry).e6.ipv6.invflags = if inv { IP6T_INV_PROTO } else { 0 }; }, NFPROTO_BRIDGE => { (*entry).ebt.ethproto = proto as _; (*entry).ebt.invflags = if inv { EBT_IPROTO } else { 0 }; }, NFPROTO_ARP => {}, _ => {} }
    (*par).entryinfo = entry as _; (*par).target = target; (*par).targinfo = info;
    (*par).hook_mask = if nft_is_base_chain((*ctx).chain) { 1u32 << (*(*nft_base_chain((*ctx).chain)).ops).hooknum } else { 0 };
    (*par).family = (*ctx).family; (*par).nft_compat = true;
}

unsafe fn target_compat_from_user(t: *mut xt_target, input: *const core::ffi::c_void, output: *mut u8) { let n = (*t).targetsize; memcpy(output as _, input as _, n); let pad = XT_ALIGN(n) - n; if pad > 0 { memset(output.add(n) as _, 0, pad); } }

static nft_rule_compat_policy: [nla_policy; NFTA_RULE_COMPAT_MAX + 1] = [nla_policy::default(); NFTA_RULE_COMPAT_MAX + 1];

unsafe fn nft_parse_compat(attr: *const nlattr, proto: *mut u16, inv: *mut bool) -> i32 {
    let mut tb: [*mut nlattr; NFTA_RULE_COMPAT_MAX + 1] = [core::ptr::null_mut(); NFTA_RULE_COMPAT_MAX + 1];
    let err = nla_parse_nested_deprecated(tb.as_mut_ptr(), NFTA_RULE_COMPAT_MAX, attr, nft_rule_compat_policy.as_ptr(), core::ptr::null_mut()); if err < 0 { return err; }
    if tb[NFTA_RULE_COMPAT_PROTO].is_null() || tb[NFTA_RULE_COMPAT_FLAGS].is_null() { return -EINVAL; }
    let flags = ntohl(nla_get_be32(tb[NFTA_RULE_COMPAT_FLAGS])); if flags & NFT_RULE_COMPAT_F_UNUSED != 0 || flags & !NFT_RULE_COMPAT_F_MASK != 0 { return -EINVAL; } if flags & NFT_RULE_COMPAT_F_INV != 0 { *inv = true; }
    let p = ntohl(nla_get_be32(tb[NFTA_RULE_COMPAT_PROTO])); if p > u16::MAX as u32 { return -EINVAL; } *proto = p as u16; 0
}

unsafe fn nft_compat_wait_for_destructors(net: *mut net) { nf_tables_trans_destroy_flush_work(net); }

// Remaining operations retain the C ABI and kernel callback layout.
extern "C" {
    fn nft_target_init(ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> i32;
    fn nft_target_destroy(ctx: *const nft_ctx, expr: *const nft_expr);
    fn nft_target_dump(skb: *mut sk_buff, expr: *const nft_expr, reset: bool) -> i32;
    fn nft_target_validate(ctx: *const nft_ctx, expr: *const nft_expr) -> i32;
    fn nft_match_init(ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> i32;
    fn nft_match_destroy(ctx: *const nft_ctx, expr: *const nft_expr);
    fn nft_match_dump(skb: *mut sk_buff, expr: *const nft_expr, reset: bool) -> i32;
    fn nft_match_validate(ctx: *const nft_ctx, expr: *const nft_expr) -> i32;
}

// The following declarations mirror the remaining file-local callback entry
// points and registration objects; their kernel-defined types are external.
extern "C" {
    fn __nft_match_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo, info: *mut core::ffi::c_void);
    fn nft_match_large_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo);
    fn nft_match_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo);
    fn nft_match_large_init(ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> i32;
    fn nft_match_large_destroy(ctx: *const nft_ctx, expr: *const nft_expr);
    fn nft_match_large_dump(skb: *mut sk_buff, expr: *const nft_expr, reset: bool) -> i32;
    fn nfnl_compat_fill_info(skb: *mut sk_buff, portid: u32, seq: u32, ty: u32, event: i32, family: u16, name: *const core::ffi::c_char, rev: i32, target: i32) -> i32;
    fn nfnl_compat_get_rcu(skb: *mut sk_buff, info: *const nfnl_info, tb: *const *const nlattr) -> i32;
    fn nft_match_select_ops(ctx: *const nft_ctx, tb: *const *const nlattr) -> *const nft_expr_ops;
    fn nft_match_release_ops(ops: *const nft_expr_ops);
    fn nft_target_select_ops(ctx: *const nft_ctx, tb: *const *const nlattr) -> *const nft_expr_ops;
    fn nft_target_release_ops(ops: *const nft_expr_ops);
    fn nft_compat_module_init() -> i32;
    fn nft_compat_module_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
