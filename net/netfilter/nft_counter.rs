// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2008-2009 Patrick McHardy <kaber@trash.net>
 *
 * Development of this code funded by Astaro AG (http://www.astaro.com/)
 */

// Kernel dependencies supplied by other translation units.

#[repr(C)]
pub struct nft_counter {
    pub bytes: u64_stats_t,
    pub packets: u64_stats_t,
}

#[repr(C)]
pub struct nft_counter_tot {
    pub bytes: i64,
    pub packets: i64,
}

#[repr(C)]
pub struct nft_counter_percpu_priv {
    pub counter: *mut nft_counter,
}

static mut nft_counter_sync: u64_stats_sync = unsafe { core::mem::zeroed() };
static mut nft_counter_lock: spinlock_t = unsafe { core::mem::zeroed() };

#[inline]
unsafe fn nft_counter_do_eval(
    priv_: *mut nft_counter_percpu_priv,
    _regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
) {
    let mut nft_sync: *mut u64_stats_sync;
    let mut this_cpu: *mut nft_counter;

    local_bh_disable();
    this_cpu = this_cpu_ptr((*priv_).counter);
    nft_sync = this_cpu_ptr(&mut nft_counter_sync);

    u64_stats_update_begin(nft_sync);
    u64_stats_add(&mut (*this_cpu).bytes, (*(*pkt).skb).len);
    u64_stats_inc(&mut (*this_cpu).packets);
    u64_stats_update_end(nft_sync);

    local_bh_enable();
}

#[inline]
unsafe fn nft_counter_obj_eval(
    obj: *mut nft_object,
    regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
) {
    let priv_: *mut nft_counter_percpu_priv = nft_obj_data(obj);
    nft_counter_do_eval(priv_, regs, pkt);
}

unsafe fn nft_counter_do_init(
    tb: *const *const nlattr,
    priv_: *mut nft_counter_percpu_priv,
) -> c_int {
    let cpu_stats: *mut nft_counter = alloc_percpu_gfp::<nft_counter>(GFP_KERNEL_ACCOUNT);
    if cpu_stats.is_null() {
        return -ENOMEM;
    }

    let this_cpu = raw_cpu_ptr(cpu_stats);
    if !(*tb.add(NFTA_COUNTER_PACKETS as usize)).is_null() {
        u64_stats_set(
            &mut (*this_cpu).packets,
            be64_to_cpu(nla_get_be64(*tb.add(NFTA_COUNTER_PACKETS as usize))),
        );
    }
    if !(*tb.add(NFTA_COUNTER_BYTES as usize)).is_null() {
        u64_stats_set(
            &mut (*this_cpu).bytes,
            be64_to_cpu(nla_get_be64(*tb.add(NFTA_COUNTER_BYTES as usize))),
        );
    }

    (*priv_).counter = cpu_stats;
    0
}

unsafe fn nft_counter_obj_init(
    _ctx: *const nft_ctx,
    tb: *const *const nlattr,
    obj: *mut nft_object,
) -> c_int {
    nft_counter_do_init(tb, nft_obj_data(obj))
}

unsafe fn nft_counter_do_destroy(priv_: *mut nft_counter_percpu_priv) {
    free_percpu((*priv_).counter);
}

unsafe fn nft_counter_obj_destroy(_ctx: *const nft_ctx, obj: *mut nft_object) {
    nft_counter_do_destroy(nft_obj_data(obj));
}

unsafe fn nft_counter_reset(priv_: *mut nft_counter_percpu_priv, total: *const nft_counter_tot) {
    local_bh_disable();
    let this_cpu = this_cpu_ptr((*priv_).counter);
    let nft_sync = this_cpu_ptr(&mut nft_counter_sync);
    u64_stats_update_begin(nft_sync);
    u64_stats_sub(&mut (*this_cpu).packets, (*total).packets as u64);
    u64_stats_sub(&mut (*this_cpu).bytes, (*total).bytes as u64);
    u64_stats_update_end(nft_sync);
    local_bh_enable();
}

unsafe fn nft_counter_fetch(priv_: *mut nft_counter_percpu_priv, total: *mut nft_counter_tot) {
    core::ptr::write_bytes(total, 0, 1);
    for_each_possible_cpu!(cpu, {
        let nft_sync = per_cpu_ptr(&mut nft_counter_sync, cpu);
        let this_cpu = per_cpu_ptr((*priv_).counter, cpu);
        let (bytes, packets);
        loop {
            let seq = u64_stats_fetch_begin(nft_sync);
            bytes = u64_stats_read(&(*this_cpu).bytes);
            packets = u64_stats_read(&(*this_cpu).packets);
            if !u64_stats_fetch_retry(nft_sync, seq) { break; }
        }
        (*total).bytes += bytes as i64;
        (*total).packets += packets as i64;
    });
}

unsafe fn nft_counter_fetch_and_reset(priv_: *mut nft_counter_percpu_priv, total: *mut nft_counter_tot) {
    spin_lock(&mut nft_counter_lock);
    nft_counter_fetch(priv_, total);
    nft_counter_reset(priv_, total);
    spin_unlock(&mut nft_counter_lock);
}

unsafe fn nft_counter_do_dump(skb: *mut sk_buff, priv_: *mut nft_counter_percpu_priv, reset: bool) -> c_int {
    let mut total = nft_counter_tot { bytes: 0, packets: 0 };
    if reset { nft_counter_fetch_and_reset(priv_, &mut total); } else { nft_counter_fetch(priv_, &mut total); }
    if nla_put_be64(skb, NFTA_COUNTER_BYTES, cpu_to_be64(total.bytes as u64), NFTA_COUNTER_PAD) != 0
        || nla_put_be64(skb, NFTA_COUNTER_PACKETS, cpu_to_be64(total.packets as u64), NFTA_COUNTER_PAD) != 0 {
        return -1;
    }
    0
}

unsafe fn nft_counter_obj_dump(skb: *mut sk_buff, obj: *mut nft_object, reset: bool) -> c_int {
    nft_counter_do_dump(skb, nft_obj_data(obj), reset)
}

#[repr(C)]
pub struct nla_policy { pub type_: u32 }
static nft_counter_policy: [nla_policy; NFTA_COUNTER_MAX as usize + 1] = [nla_policy { type_: 0 }; NFTA_COUNTER_MAX as usize + 1];

// The following operation/type tables retain the C symbols; their field types
// and callback members are supplied by the kernel nftables declarations.
static nft_counter_obj_ops: nft_object_ops = unsafe { core::mem::zeroed() };

pub static mut nft_counter_obj_type: nft_object_type = unsafe { core::mem::zeroed() };

pub unsafe fn nft_counter_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    nft_counter_do_eval(nft_expr_priv(expr), regs, pkt);
}

unsafe fn nft_counter_dump(skb: *mut sk_buff, expr: *const nft_expr, reset: bool) -> c_int {
    nft_counter_do_dump(skb, nft_expr_priv(expr), reset)
}

unsafe fn nft_counter_init(_ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> c_int {
    nft_counter_do_init(tb, nft_expr_priv(expr))
}

unsafe fn nft_counter_destroy(_ctx: *const nft_ctx, expr: *const nft_expr) {
    nft_counter_do_destroy(nft_expr_priv(expr));
}

unsafe fn nft_counter_clone(dst: *mut nft_expr, src: *const nft_expr, gfp: gfp_t) -> c_int {
    let priv_ = nft_expr_priv(src);
    let priv_clone = nft_expr_priv(dst);
    let mut total = nft_counter_tot { bytes: 0, packets: 0 };
    nft_counter_fetch(priv_, &mut total);
    let cpu_stats = alloc_percpu_gfp::<nft_counter>(gfp);
    if cpu_stats.is_null() { return -ENOMEM; }
    let this_cpu = raw_cpu_ptr(cpu_stats);
    u64_stats_set(&mut (*this_cpu).packets, total.packets as u64);
    u64_stats_set(&mut (*this_cpu).bytes, total.bytes as u64);
    (*priv_clone).counter = cpu_stats;
    0
}

unsafe fn nft_counter_offload(_ctx: *mut nft_offload_ctx, _flow: *mut nft_flow_rule, _expr: *const nft_expr) -> c_int {
    /* No specific offload action is needed, but report success. */
    0
}

unsafe fn nft_counter_offload_stats(expr: *mut nft_expr, stats: *const flow_stats) {
    let priv_ = nft_expr_priv(expr);
    local_bh_disable();
    let this_cpu = this_cpu_ptr((*priv_).counter);
    let nft_sync = this_cpu_ptr(&mut nft_counter_sync);
    u64_stats_update_begin(nft_sync);
    u64_stats_add(&mut (*this_cpu).packets, (*stats).pkts);
    u64_stats_add(&mut (*this_cpu).bytes, (*stats).bytes);
    u64_stats_update_end(nft_sync);
    local_bh_enable();
}

pub unsafe fn nft_counter_init_seqcount() {
    for_each_possible_cpu!(cpu, { u64_stats_init(per_cpu_ptr(&mut nft_counter_sync, cpu)); });
}

pub static mut nft_counter_type: nft_expr_type = unsafe { core::mem::zeroed() };
static nft_counter_ops: nft_expr_ops = unsafe { core::mem::zeroed() };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
