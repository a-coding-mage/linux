// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2008-2009 Patrick McHardy <kaber@trash.net>
 *
 * Development of this code funded by Astaro AG (http://www.astaro.com/)
 */

// Kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct nft_limit {
    pub lock: spinlock_t,
    pub last: u64,
    pub tokens: u64,
}

#[repr(C)]
pub struct nft_limit_priv {
    pub limit: *mut nft_limit,
    pub tokens_max: u64,
    pub rate: u64,
    pub nsecs: u64,
    pub burst: u32,
    pub invert: bool,
}

#[inline]
unsafe fn nft_limit_eval(priv_: *mut nft_limit_priv, cost: u64) -> bool {
    let mut now: u64;
    let mut tokens: u64;
    let delta: i64;

    spin_lock_bh(&mut (*(*priv_).limit).lock);
    now = ktime_get_ns();
    tokens = (*(*priv_).limit).tokens.wrapping_add(now.wrapping_sub((*(*priv_).limit).last));
    if tokens > (*priv_).tokens_max {
        tokens = (*priv_).tokens_max;
    }

    (*(*priv_).limit).last = now;
    delta = (tokens as i64).wrapping_sub(cost as i64);
    if delta >= 0 {
        (*(*priv_).limit).tokens = delta as u64;
        spin_unlock_bh(&mut (*(*priv_).limit).lock);
        return (*priv_).invert;
    }
    (*(*priv_).limit).tokens = tokens;
    spin_unlock_bh(&mut (*(*priv_).limit).lock);
    !(*priv_).invert
}

// Use same default as in iptables.
const NFT_LIMIT_PKT_BURST_DEFAULT: u32 = 5;

unsafe fn nft_limit_init(priv_: *mut nft_limit_priv, tb: *const *const nlattr, pkts: bool) -> i32 {
    let mut unit: u64;
    let mut tokens: u64;
    let mut rate_with_burst: u64;
    let mut invert = false;

    if (*tb.add(NFTA_LIMIT_RATE as usize)).is_null() || (*tb.add(NFTA_LIMIT_UNIT as usize)).is_null() {
        return -EINVAL;
    }

    (*priv_).rate = be64_to_cpu(nla_get_be64(*tb.add(NFTA_LIMIT_RATE as usize)));
    if (*priv_).rate == 0 { return -EINVAL; }

    unit = be64_to_cpu(nla_get_be64(*tb.add(NFTA_LIMIT_UNIT as usize)));
    if check_mul_overflow(unit, NSEC_PER_SEC, &mut (*priv_).nsecs) { return -EOVERFLOW; }

    if !(*tb.add(NFTA_LIMIT_BURST as usize)).is_null() {
        (*priv_).burst = ntohl(nla_get_be32(*tb.add(NFTA_LIMIT_BURST as usize)));
    }
    if pkts && (*priv_).burst == 0 { (*priv_).burst = NFT_LIMIT_PKT_BURST_DEFAULT; }
    if check_add_overflow((*priv_).rate, (*priv_).burst as u64, &mut rate_with_burst) { return -EOVERFLOW; }

    if pkts {
        let tmp = div64_u64((*priv_).nsecs, (*priv_).rate);
        if check_mul_overflow(tmp, (*priv_).burst as u64, &mut tokens) { return -EOVERFLOW; }
    } else {
        let mut tmp = 0u64;
        // The token bucket size limits the number of tokens can be accumulated.
        // tokens_max specifies the bucket size. tokens_max = unit * (rate + burst) / rate.
        if check_mul_overflow((*priv_).nsecs, rate_with_burst, &mut tmp) { return -EOVERFLOW; }
        tokens = div64_u64(tmp, (*priv_).rate);
    }

    if !(*tb.add(NFTA_LIMIT_FLAGS as usize)).is_null() {
        let flags = ntohl(nla_get_be32(*tb.add(NFTA_LIMIT_FLAGS as usize)));
        if flags & !NFT_LIMIT_F_INV != 0 { return -EOPNOTSUPP; }
        if flags & NFT_LIMIT_F_INV != 0 { invert = true; }
    }

    (*priv_).limit = kmalloc_obj::<nft_limit>(GFP_KERNEL_ACCOUNT);
    if (*priv_).limit.is_null() { return -ENOMEM; }
    (*(*priv_).limit).tokens = tokens;
    (*priv_).tokens_max = (*priv_).limit.as_ref().unwrap().tokens;
    (*priv_).invert = invert;
    (*(*priv_).limit).last = ktime_get_ns();
    spin_lock_init(&mut (*(*priv_).limit).lock);
    0
}

unsafe fn nft_limit_dump(skb: *mut sk_buff, priv_: *const nft_limit_priv, type_: nft_limit_type) -> i32 {
    let flags = if (*priv_).invert { NFT_LIMIT_F_INV } else { 0 };
    let secs = div_u64((*priv_).nsecs, NSEC_PER_SEC);
    if nla_put_be64(skb, NFTA_LIMIT_RATE, cpu_to_be64((*priv_).rate), NFTA_LIMIT_PAD) != 0
        || nla_put_be64(skb, NFTA_LIMIT_UNIT, cpu_to_be64(secs), NFTA_LIMIT_PAD) != 0
        || nla_put_be32(skb, NFTA_LIMIT_BURST, htonl((*priv_).burst)) != 0
        || nla_put_be32(skb, NFTA_LIMIT_TYPE, htonl(type_ as u32)) != 0
        || nla_put_be32(skb, NFTA_LIMIT_FLAGS, htonl(flags)) != 0 { return -1; }
    0
}

unsafe fn nft_limit_destroy(_ctx: *const nft_ctx, priv_: *const nft_limit_priv) {
    kfree((*priv_).limit as *mut core::ffi::c_void);
}

unsafe fn nft_limit_clone(dst: *mut nft_limit_priv, src: *const nft_limit_priv, gfp: gfp_t) -> i32 {
    (*dst).tokens_max = (*src).tokens_max;
    (*dst).rate = (*src).rate;
    (*dst).nsecs = (*src).nsecs;
    (*dst).burst = (*src).burst;
    (*dst).invert = (*src).invert;
    (*dst).limit = kmalloc_obj::<nft_limit>(gfp);
    if (*dst).limit.is_null() { return -ENOMEM; }
    spin_lock_init(&mut (*(*dst).limit).lock);
    (*(*dst).limit).tokens = (*src).tokens_max;
    (*(*dst).limit).last = ktime_get_ns();
    0
}

#[repr(C)]
pub struct nft_limit_priv_pkts { pub limit: nft_limit_priv, pub cost: u64 }

// [NFTA_LIMIT_MAX + 1] policy entries: RATE/UNIT are NLA_U64, BURST/TYPE are
// NLA_U32, and FLAGS is a masked big-endian u32 accepting NFT_LIMIT_F_INV.
extern "C" {
    static nft_limit_policy: [nla_policy; NFTA_LIMIT_MAX as usize + 1];
    static nft_limit_pkts_ops: nft_expr_ops;
    static nft_limit_bytes_ops: nft_expr_ops;
    static nft_limit_obj_pkts_ops: nft_object_ops;
    static nft_limit_obj_bytes_ops: nft_object_ops;
}

unsafe fn nft_limit_pkts_eval(expr: *const nft_expr, regs: *mut nft_regs, _pkt: *const nft_pktinfo) {
    let priv_ = nft_expr_priv(expr) as *mut nft_limit_priv_pkts;
    if nft_limit_eval(&mut (*priv_).limit, (*priv_).cost) { (*regs).verdict.code = NFT_BREAK; }
}

unsafe fn nft_limit_pkts_init(_ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> i32 {
    let priv_ = nft_expr_priv(expr) as *mut nft_limit_priv_pkts;
    let err = nft_limit_init(&mut (*priv_).limit, tb, true);
    if err < 0 { return err; }
    (*priv_).cost = div64_u64((*priv_).limit.nsecs, (*priv_).limit.rate);
    0
}
unsafe fn nft_limit_pkts_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> i32 {
    let priv_ = nft_expr_priv(expr) as *const nft_limit_priv_pkts;
    nft_limit_dump(skb, &(*priv_).limit, NFT_LIMIT_PKTS)
}
unsafe fn nft_limit_pkts_destroy(ctx: *const nft_ctx, expr: *const nft_expr) {
    nft_limit_destroy(ctx, &(*(nft_expr_priv(expr) as *const nft_limit_priv_pkts)).limit);
}
unsafe fn nft_limit_pkts_clone(dst: *mut nft_expr, src: *const nft_expr, gfp: gfp_t) -> i32 {
    let d = nft_expr_priv(dst) as *mut nft_limit_priv_pkts;
    let s = nft_expr_priv(src) as *const nft_limit_priv_pkts;
    (*d).cost = (*s).cost;
    nft_limit_clone(&mut (*d).limit, &(*s).limit, gfp)
}

unsafe fn nft_limit_bytes_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    let priv_ = nft_expr_priv(expr) as *mut nft_limit_priv;
    let cost = div64_u64((*priv_).nsecs.wrapping_mul((*(*pkt).skb).len as u64), (*priv_).rate);
    if nft_limit_eval(priv_, cost) { (*regs).verdict.code = NFT_BREAK; }
}
unsafe fn nft_limit_bytes_init(_ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> i32 {
    nft_limit_init(nft_expr_priv(expr) as *mut nft_limit_priv, tb, false)
}
unsafe fn nft_limit_bytes_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> i32 {
    nft_limit_dump(skb, nft_expr_priv(expr) as *const nft_limit_priv, NFT_LIMIT_PKT_BYTES)
}
unsafe fn nft_limit_bytes_destroy(ctx: *const nft_ctx, expr: *const nft_expr) {
    nft_limit_destroy(ctx, nft_expr_priv(expr) as *const nft_limit_priv);
}
unsafe fn nft_limit_bytes_clone(dst: *mut nft_expr, src: *const nft_expr, gfp: gfp_t) -> i32 {
    nft_limit_clone(nft_expr_priv(dst) as *mut nft_limit_priv, nft_expr_priv(src) as *const nft_limit_priv, gfp)
}

unsafe fn nft_limit_select_ops(_ctx: *const nft_ctx, tb: *const *const nlattr) -> *const nft_expr_ops {
    if (*tb.add(NFTA_LIMIT_TYPE as usize)).is_null() { return &nft_limit_pkts_ops; }
    match ntohl(nla_get_be32(*tb.add(NFTA_LIMIT_TYPE as usize))) {
        NFT_LIMIT_PKTS => &nft_limit_pkts_ops,
        NFT_LIMIT_PKT_BYTES => &nft_limit_bytes_ops,
        _ => ERR_PTR(-EOPNOTSUPP),
    }
}

unsafe fn nft_limit_obj_pkts_eval(obj: *mut nft_object, regs: *mut nft_regs, _pkt: *const nft_pktinfo) {
    let priv_ = nft_obj_data(obj) as *mut nft_limit_priv_pkts;
    if nft_limit_eval(&mut (*priv_).limit, (*priv_).cost) { (*regs).verdict.code = NFT_BREAK; }
}
unsafe fn nft_limit_obj_pkts_init(_ctx: *const nft_ctx, tb: *const *const nlattr, obj: *mut nft_object) -> i32 {
    let priv_ = nft_obj_data(obj) as *mut nft_limit_priv_pkts;
    let err = nft_limit_init(&mut (*priv_).limit, tb, true);
    if err < 0 { return err; }
    (*priv_).cost = div64_u64((*priv_).limit.nsecs, (*priv_).limit.rate); 0
}
unsafe fn nft_limit_obj_pkts_dump(skb: *mut sk_buff, obj: *mut nft_object, _reset: bool) -> i32 {
    nft_limit_dump(skb, &(*(nft_obj_data(obj) as *const nft_limit_priv_pkts)).limit, NFT_LIMIT_PKTS)
}
unsafe fn nft_limit_obj_pkts_destroy(ctx: *const nft_ctx, obj: *mut nft_object) { nft_limit_destroy(ctx, &(*(nft_obj_data(obj) as *const nft_limit_priv_pkts)).limit); }

unsafe fn nft_limit_obj_bytes_eval(obj: *mut nft_object, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    let priv_ = nft_obj_data(obj) as *mut nft_limit_priv;
    let cost = div64_u64((*priv_).nsecs.wrapping_mul((*(*pkt).skb).len as u64), (*priv_).rate);
    if nft_limit_eval(priv_, cost) { (*regs).verdict.code = NFT_BREAK; }
}
unsafe fn nft_limit_obj_bytes_init(_ctx: *const nft_ctx, tb: *const *const nlattr, obj: *mut nft_object) -> i32 { nft_limit_init(nft_obj_data(obj) as *mut nft_limit_priv, tb, false) }
unsafe fn nft_limit_obj_bytes_dump(skb: *mut sk_buff, obj: *mut nft_object, _reset: bool) -> i32 { nft_limit_dump(skb, nft_obj_data(obj) as *const nft_limit_priv, NFT_LIMIT_PKT_BYTES) }
unsafe fn nft_limit_obj_bytes_destroy(ctx: *const nft_ctx, obj: *mut nft_object) { nft_limit_destroy(ctx, nft_obj_data(obj) as *const nft_limit_priv); }

unsafe fn nft_limit_obj_select_ops(_ctx: *const nft_ctx, tb: *const *const nlattr) -> *const nft_object_ops {
    if (*tb.add(NFTA_LIMIT_TYPE as usize)).is_null() { return &nft_limit_obj_pkts_ops; }
    match ntohl(nla_get_be32(*tb.add(NFTA_LIMIT_TYPE as usize))) {
        NFT_LIMIT_PKTS => &nft_limit_obj_pkts_ops,
        NFT_LIMIT_PKT_BYTES => &nft_limit_obj_bytes_ops,
        _ => ERR_PTR(-EOPNOTSUPP),
    }
}

static mut nft_limit_type: nft_expr_type = nft_expr_type { name: "limit", select_ops: nft_limit_select_ops, policy: nft_limit_policy, maxattr: NFTA_LIMIT_MAX, flags: NFT_EXPR_STATEFUL, owner: THIS_MODULE };
static mut nft_limit_obj_type: nft_object_type = nft_object_type { select_ops: nft_limit_obj_select_ops, type_: NFT_OBJECT_LIMIT, maxattr: NFTA_LIMIT_MAX, policy: nft_limit_policy, owner: THIS_MODULE };

unsafe fn nft_limit_module_init() -> i32 {
    let mut err = nft_register_obj(&mut nft_limit_obj_type);
    if err < 0 { return err; }
    err = nft_register_expr(&mut nft_limit_type);
    if err < 0 { nft_unregister_obj(&mut nft_limit_obj_type); return err; }
    0
}
unsafe fn nft_limit_module_exit() { nft_unregister_expr(&mut nft_limit_type); nft_unregister_obj(&mut nft_limit_obj_type); }

// module_init(nft_limit_module_init);
// module_exit(nft_limit_module_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Patrick McHardy <kaber@trash.net>");
// MODULE_ALIAS_NFT_EXPR("limit");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
