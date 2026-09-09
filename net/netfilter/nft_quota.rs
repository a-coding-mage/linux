// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2016 Pablo Neira Ayuso <pablo@netfilter.org>
 */

// Kernel headers and symbols referenced below are supplied by the surrounding build.

#[repr(C)]
pub struct nft_quota {
    pub quota: atomic64_t,
    pub flags: libc::c_ulong,
    pub consumed: *mut atomic64_t,
}

#[inline]
unsafe fn nft_overquota(priv_: *mut nft_quota, skb: *const sk_buff, report: *mut bool) -> bool {
    let consumed = atomic64_add_return((*skb).len as u64, (*priv_).consumed);
    let quota = atomic64_read(&(*priv_).quota);
    if !report.is_null() { *report = consumed >= quota; }
    consumed > quota
}

#[inline]
unsafe fn nft_quota_invert(priv_: *mut nft_quota) -> bool { ((*priv_).flags & NFT_QUOTA_F_INV as libc::c_ulong) != 0 }

#[inline]
unsafe fn nft_quota_do_eval(priv_: *mut nft_quota, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    if nft_overquota(priv_, (*pkt).skb, core::ptr::null_mut()) ^ nft_quota_invert(priv_) { (*regs).verdict.code = NFT_BREAK; }
}

static mut nft_quota_policy: [nla_policy; NFTA_QUOTA_MAX as usize + 1] = [nla_policy { type_: 0 }; NFTA_QUOTA_MAX as usize + 1];

const NFT_QUOTA_DEPLETED_BIT: libc::c_ulong = 1;

unsafe fn nft_quota_obj_eval(obj: *mut nft_object, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    let priv_ = nft_obj_data(obj);
    let mut report = false;
    let overquota = nft_overquota(priv_, (*pkt).skb, &mut report);
    if overquota ^ nft_quota_invert(priv_) { (*regs).verdict.code = NFT_BREAK; }
    if report && test_and_set_bit(NFT_QUOTA_DEPLETED_BIT, &mut (*priv_).flags) == 0 {
        nft_obj_notify(nft_net(pkt), (*obj).key.table, obj, 0, 0, NFT_MSG_NEWOBJ, 0, nft_pf(pkt), 0, GFP_ATOMIC);
    }
}

unsafe fn nft_quota_do_init(tb: *const *const nlattr, priv_: *mut nft_quota) -> i32 {
    let mut flags: libc::c_ulong = 0;
    let mut quota: u64;
    let mut consumed: u64 = 0;
    if (*tb.add(NFTA_QUOTA_BYTES as usize)).is_null() { return -EINVAL; }
    quota = be64_to_cpu(nla_get_be64(*tb.add(NFTA_QUOTA_BYTES as usize)));
    if quota > S64_MAX as u64 { return -EOVERFLOW; }
    if !(*tb.add(NFTA_QUOTA_CONSUMED as usize)).is_null() {
        consumed = be64_to_cpu(nla_get_be64(*tb.add(NFTA_QUOTA_CONSUMED as usize)));
        if consumed > quota { return -EINVAL; }
    }
    if !(*tb.add(NFTA_QUOTA_FLAGS as usize)).is_null() {
        flags = ntohl(nla_get_be32(*tb.add(NFTA_QUOTA_FLAGS as usize))) as libc::c_ulong;
        if flags & !(NFT_QUOTA_F_INV as libc::c_ulong) != 0 { return -EINVAL; }
        if flags & NFT_QUOTA_F_DEPLETED as libc::c_ulong != 0 { return -EOPNOTSUPP; }
    }
    (*priv_).consumed = kmalloc_obj::<atomic64_t>(GFP_KERNEL_ACCOUNT);
    if (*priv_).consumed.is_null() { return -ENOMEM; }
    atomic64_set(&mut (*priv_).quota, quota);
    (*priv_).flags = flags;
    atomic64_set((*priv_).consumed, consumed);
    0
}

unsafe fn nft_quota_do_destroy(_ctx: *const nft_ctx, priv_: *mut nft_quota) { kfree((*priv_).consumed); }

unsafe fn nft_quota_obj_init(_ctx: *const nft_ctx, tb: *const *const nlattr, obj: *mut nft_object) -> i32 { nft_quota_do_init(tb, nft_obj_data(obj)) }

unsafe fn nft_quota_obj_update(obj: *mut nft_object, newobj: *mut nft_object) {
    let newpriv = nft_obj_data(newobj); let priv_ = nft_obj_data(obj);
    atomic64_set(&mut (*priv_).quota, atomic64_read(&(*newpriv).quota)); (*priv_).flags = (*newpriv).flags;
}

unsafe fn nft_quota_do_dump(skb: *mut sk_buff, priv_: *mut nft_quota, reset: bool) -> i32 {
    let mut flags = (*priv_).flags as u32;
    let consumed = if reset { let v = atomic64_xchg((*priv_).consumed, 0); clear_bit(NFT_QUOTA_DEPLETED_BIT, &mut (*priv_).flags); v } else { atomic64_read((*priv_).consumed) };
    let quota = atomic64_read(&(*priv_).quota);
    let consumed_cap = if consumed >= quota { flags |= NFT_QUOTA_F_DEPLETED; quota } else { consumed };
    if nla_put_be64(skb, NFTA_QUOTA_BYTES, cpu_to_be64(quota), NFTA_QUOTA_PAD) != 0 || nla_put_be64(skb, NFTA_QUOTA_CONSUMED, cpu_to_be64(consumed_cap), NFTA_QUOTA_PAD) != 0 || nla_put_be32(skb, NFTA_QUOTA_FLAGS, htonl(flags),) != 0 { return -1; }
    0
}

unsafe fn nft_quota_obj_dump(skb: *mut sk_buff, obj: *mut nft_object, reset: bool) -> i32 { nft_quota_do_dump(skb, nft_obj_data(obj), reset) }
unsafe fn nft_quota_obj_destroy(ctx: *const nft_ctx, obj: *mut nft_object) { nft_quota_do_destroy(ctx, nft_obj_data(obj)); }

static mut nft_quota_obj_type: nft_object_type = nft_object_type { type_: NFT_OBJECT_QUOTA, ops: &nft_quota_obj_ops, maxattr: NFTA_QUOTA_MAX, policy: unsafe { &nft_quota_policy }, owner: THIS_MODULE };
static nft_quota_obj_ops: nft_object_ops = nft_object_ops { type_: unsafe { &nft_quota_obj_type }, size: core::mem::size_of::<nft_quota>(), init: Some(nft_quota_obj_init), destroy: Some(nft_quota_obj_destroy), eval: Some(nft_quota_obj_eval), dump: Some(nft_quota_obj_dump), update: Some(nft_quota_obj_update) };

unsafe fn nft_quota_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) { nft_quota_do_eval(nft_expr_priv(expr), regs, pkt); }
unsafe fn nft_quota_init(_ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> i32 { nft_quota_do_init(tb, nft_expr_priv(expr)) }
unsafe fn nft_quota_dump(skb: *mut sk_buff, expr: *const nft_expr, reset: bool) -> i32 { nft_quota_do_dump(skb, nft_expr_priv(expr), reset) }
unsafe fn nft_quota_destroy(ctx: *const nft_ctx, expr: *const nft_expr) { nft_quota_do_destroy(ctx, nft_expr_priv(expr)); }
unsafe fn nft_quota_clone(dst: *mut nft_expr, src: *const nft_expr, gfp: gfp_t) -> i32 {
    let d = nft_expr_priv(dst); let s = nft_expr_priv(src); (*d).quota = (*s).quota; (*d).flags = (*s).flags;
    (*d).consumed = kmalloc_obj_with_gfp::<atomic64_t>(gfp); if (*d).consumed.is_null() { return -ENOMEM; } *(*d).consumed = *(*s).consumed; 0
}

static mut nft_quota_type: nft_expr_type = nft_expr_type { name: "quota", ops: &nft_quota_ops, policy: unsafe { &nft_quota_policy }, maxattr: NFTA_QUOTA_MAX, flags: NFT_EXPR_STATEFUL, owner: THIS_MODULE };
static nft_quota_ops: nft_expr_ops = nft_expr_ops { type_: unsafe { &nft_quota_type }, size: NFT_EXPR_SIZE(core::mem::size_of::<nft_quota>()), eval: Some(nft_quota_eval), init: Some(nft_quota_init), destroy: Some(nft_quota_destroy), clone_: Some(nft_quota_clone), dump: Some(nft_quota_dump) };

unsafe fn nft_quota_module_init() -> i32 {
    let mut err = nft_register_obj(&mut nft_quota_obj_type); if err < 0 { return err; }
    err = nft_register_expr(&mut nft_quota_type); if err < 0 { nft_unregister_obj(&mut nft_quota_obj_type); return err; } 0
}
unsafe fn nft_quota_module_exit() { nft_unregister_expr(&mut nft_quota_type); nft_unregister_obj(&mut nft_quota_obj_type); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
