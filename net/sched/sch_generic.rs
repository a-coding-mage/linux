// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/sched/sch_generic.c	Generic packet scheduler routines.
 *
 * Rust translation of the implementation source. Kernel declarations and
 * macros referenced below are supplied by the surrounding translation unit.
 */

// Qdisc to use by default
pub static mut default_qdisc_ops: *const Qdisc_ops = &pfifo_fast_ops;

pub unsafe fn __tcf_kfree_skb_list(mut skb: *mut sk_buff, q: *mut Qdisc,
                                   txq: *mut netdev_queue, dev: *mut net_device) {
    while !skb.is_null() {
        let reason: u32 = (*tc_skb_cb(skb)).drop_reason;
        let next = (*skb).next;
        prefetch(next);
        /* TC classifier and qdisc share drop_reason storage.
         * Check subsystem mask to identify qdisc drop reasons,
         * else pass through skb_drop_reason set by TC classifier.
         */
        let skb_reason = if (reason & SKB_DROP_REASON_SUBSYS_MASK) == __QDISC_DROP_REASON {
            trace_qdisc_drop(q, txq, dev, skb, reason as qdisc_drop_reason);
            SKB_DROP_REASON_QDISC_DROP
        } else { reason as skb_drop_reason };
        kfree_skb_reason(skb, skb_reason);
        skb = next;
    }
}

unsafe fn qdisc_maybe_clear_missed(q: *mut Qdisc, txq: *const netdev_queue) {
    clear_bit(__QDISC_STATE_MISSED, &mut (*q).state);
    smp_mb__after_atomic();
    if !netif_xmit_frozen_or_stopped(txq) {
        set_bit(__QDISC_STATE_MISSED, &mut (*q).state);
    } else {
        set_bit(__QDISC_STATE_DRAINING, &mut (*q).state);
    }
}

// Main transmission queue.
// Modifications to data participating in scheduling must be protected with
// qdisc_lock(qdisc) spinlock.

const SKB_XOFF_MAGIC: *mut sk_buff = 1 as *mut sk_buff;

unsafe fn __skb_dequeue_bad_txq(q: *mut Qdisc) -> *mut sk_buff {
    let mut txq = (*q).dev_queue as *const netdev_queue;
    let mut lock: *mut spinlock_t = core::ptr::null_mut();
    if (*q).flags & TCQ_F_NOLOCK != 0 { lock = qdisc_lock(q); spin_lock(lock); }
    let mut skb = skb_peek(&(*q).skb_bad_txq);
    if !skb.is_null() {
        txq = skb_get_tx_queue((*txq).dev, skb);
        if !netif_xmit_frozen_or_stopped(txq) {
            skb = __skb_dequeue(&mut (*q).skb_bad_txq);
            if qdisc_is_percpu_stats(q) { qdisc_qstats_cpu_backlog_dec(q, skb); qdisc_qstats_cpu_qlen_dec(q); }
            else { qdisc_qstats_backlog_dec(q, skb); qdisc_qlen_dec(q); }
        } else { skb = SKB_XOFF_MAGIC; qdisc_maybe_clear_missed(q, txq); }
    }
    if !lock.is_null() { spin_unlock(lock); }
    skb
}

unsafe fn qdisc_dequeue_skb_bad_txq(q: *mut Qdisc) -> *mut sk_buff {
    let mut skb = skb_peek(&(*q).skb_bad_txq);
    if !skb.is_null() { skb = __skb_dequeue_bad_txq(q); }
    skb
}

unsafe fn qdisc_enqueue_skb_bad_txq(q: *mut Qdisc, skb: *mut sk_buff) {
    let mut lock: *mut spinlock_t = core::ptr::null_mut();
    if (*q).flags & TCQ_F_NOLOCK != 0 { lock = qdisc_lock(q); spin_lock(lock); }
    __skb_queue_tail(&mut (*q).skb_bad_txq, skb);
    if qdisc_is_percpu_stats(q) { qdisc_qstats_cpu_backlog_inc(q, skb); qdisc_qstats_cpu_qlen_inc(q); }
    else { qdisc_qstats_backlog_inc(q, skb); qdisc_qlen_inc(q); }
    if !lock.is_null() { spin_unlock(lock); }
}

unsafe fn dev_requeue_skb(mut skb: *mut sk_buff, q: *mut Qdisc) {
    let mut lock: *mut spinlock_t = core::ptr::null_mut();
    if (*q).flags & TCQ_F_NOLOCK != 0 { lock = qdisc_lock(q); spin_lock(lock); }
    while !skb.is_null() {
        let next = (*skb).next;
        __skb_queue_tail(&mut (*q).gso_skb, skb);
        if qdisc_is_percpu_stats(q) { qdisc_qstats_cpu_requeues_inc(q); qdisc_qstats_cpu_backlog_inc(q, skb); qdisc_qstats_cpu_qlen_inc(q); }
        else { (*q).qstats.requeues += 1; qdisc_qstats_backlog_inc(q, skb); qdisc_qlen_inc(q); }
        skb = next;
    }
    if !lock.is_null() { spin_unlock(lock); set_bit(__QDISC_STATE_MISSED, &mut (*q).state); }
    else { __netif_schedule(q); }
}

unsafe fn try_bulk_dequeue_skb(q: *mut Qdisc, mut skb: *mut sk_buff,
                               txq: *const netdev_queue, packets: *mut i32, budget: i32) {
    let mut bytelimit = qdisc_avail_bulklimit(txq) - (*skb).len;
    let mut cnt = 0;
    while bytelimit > 0 {
        let nskb = ((*q).dequeue.unwrap())(q); if nskb.is_null() { break; }
        bytelimit -= (*nskb).len; (*skb).next = nskb; skb = nskb;
        cnt += 1; if cnt >= budget { break; }
    }
    *packets += cnt; skb_mark_not_on_list(skb);
}

unsafe fn try_bulk_dequeue_skb_slow(q: *mut Qdisc, mut skb: *mut sk_buff, packets: *mut i32) {
    let mapping = skb_get_queue_mapping(skb); let mut cnt = 0;
    loop {
        let nskb = ((*q).dequeue.unwrap())(q); if nskb.is_null() { break; }
        if skb_get_queue_mapping(nskb) != mapping { qdisc_enqueue_skb_bad_txq(q, nskb); break; }
        (*skb).next = nskb; skb = nskb; cnt += 1; if cnt >= 8 { break; }
    }
    *packets += cnt; skb_mark_not_on_list(skb);
}

unsafe fn dequeue_skb(q: *mut Qdisc, validate: *mut bool, packets: *mut i32, budget: i32) -> *mut sk_buff {
    let mut txq = (*q).dev_queue as *const netdev_queue; let mut skb = core::ptr::null_mut(); *packets = 1;
    if !skb_queue_empty(&(*q).gso_skb) {
        let mut lock: *mut spinlock_t = core::ptr::null_mut();
        if (*q).flags & TCQ_F_NOLOCK != 0 { lock = qdisc_lock(q); spin_lock(lock); }
        skb = skb_peek(&(*q).gso_skb);
        if skb.is_null() { if !lock.is_null() { spin_unlock(lock); } }
        else {
            *validate = xfrm_offload(skb); txq = skb_get_tx_queue((*txq).dev, skb);
            if !netif_xmit_frozen_or_stopped(txq) {
                skb = __skb_dequeue(&mut (*q).gso_skb);
                if qdisc_is_percpu_stats(q) { qdisc_qstats_cpu_backlog_dec(q, skb); qdisc_qstats_cpu_qlen_dec(q); }
                else { qdisc_qstats_backlog_dec(q, skb); qdisc_qlen_dec(q); }
            } else { skb = core::ptr::null_mut(); qdisc_maybe_clear_missed(q, txq); }
            if !lock.is_null() { spin_unlock(lock); }
        }
        if !skb.is_null() { trace_qdisc_dequeue(q, txq, *packets, skb); return skb; }
    }
    *validate = true;
    if (*q).flags & TCQ_F_ONETXQUEUE != 0 && netif_xmit_frozen_or_stopped(txq) { qdisc_maybe_clear_missed(q, txq); return skb; }
    skb = qdisc_dequeue_skb_bad_txq(q);
    if !skb.is_null() { if skb == SKB_XOFF_MAGIC { return core::ptr::null_mut(); } }
    else { skb = ((*q).dequeue.unwrap())(q); }
    if !skb.is_null() { if qdisc_may_bulk(q) { try_bulk_dequeue_skb(q, skb, txq, packets, budget); } else { try_bulk_dequeue_skb_slow(q, skb, packets); } }
    trace_qdisc_dequeue(q, txq, *packets, skb); skb
}

pub unsafe fn sch_direct_xmit(mut skb: *mut sk_buff, q: *mut Qdisc, dev: *mut net_device,
                              txq: *mut netdev_queue, root_lock: *mut spinlock_t, validate: bool) -> bool {
    let mut ret = NETDEV_TX_BUSY; let mut again = false;
    if !root_lock.is_null() { spin_unlock(root_lock); }
    if validate { skb = validate_xmit_skb_list(skb, dev, &mut again); }
    if again { if !root_lock.is_null() { spin_lock(root_lock); } dev_requeue_skb(skb, q); return false; }
    if !skb.is_null() {
        HARD_TX_LOCK(dev, txq, smp_processor_id());
        if !netif_xmit_frozen_or_stopped(txq) { skb = dev_hard_start_xmit(skb, dev, txq, &mut ret); } else { qdisc_maybe_clear_missed(q, txq); }
        HARD_TX_UNLOCK(dev, txq);
    } else { if !root_lock.is_null() { spin_lock(root_lock); } return true; }
    if !root_lock.is_null() { spin_lock(root_lock); }
    if !dev_xmit_complete(ret) { if ret != NETDEV_TX_BUSY { net_warn_ratelimited("BUG %s code %d qlen %d\n", (*dev).name, ret, (*q).q.qlen); } dev_requeue_skb(skb, q); return false; }
    true
}

unsafe fn qdisc_restart(q: *mut Qdisc, packets: *mut i32, budget: i32) -> bool {
    let (mut validate, mut root_lock) = (false, core::ptr::null_mut());
    let skb = dequeue_skb(q, &mut validate, packets, budget); if skb.is_null() { return false; }
    if (*q).flags & TCQ_F_NOLOCK == 0 { root_lock = qdisc_lock(q); }
    let dev = qdisc_dev(q); let txq = skb_get_tx_queue(dev, skb);
    sch_direct_xmit(skb, q, dev, txq, root_lock, validate)
}

pub unsafe fn __qdisc_run(q: *mut Qdisc) {
    let mut quota = READ_ONCE(net_hotdata.dev_tx_weight); let mut packets = 0;
    while qdisc_restart(q, &mut packets, quota) { quota -= packets; if quota <= 0 { if (*q).flags & TCQ_F_NOLOCK != 0 { set_bit(__QDISC_STATE_MISSED, &mut (*q).state); } else { __netif_schedule(q); } break; } }
}

pub unsafe fn dev_trans_start(dev: *mut net_device) -> c_ulong {
    let mut res = READ_ONCE((*netdev_get_tx_queue(dev, 0)).trans_start); let mut i = 1;
    while i < (*dev).num_tx_queues { let val = READ_ONCE((*netdev_get_tx_queue(dev, i)).trans_start); if val != 0 && time_after(val, res) { res = val; } i += 1; } res
}

unsafe fn netif_freeze_queues(dev: *mut net_device) { let cpu = smp_processor_id(); let mut i = 0; while i < (*dev).num_tx_queues { let txq = netdev_get_tx_queue(dev, i); __netif_tx_lock(txq, cpu); set_bit(__QUEUE_STATE_FROZEN, &mut (*txq).state); __netif_tx_unlock(txq); i += 1; } }
pub unsafe fn netif_tx_lock(dev: *mut net_device) { spin_lock(&mut (*dev).tx_global_lock); netif_freeze_queues(dev); }
unsafe fn netif_unfreeze_queues(dev: *mut net_device) { let mut i=0; while i<(*dev).num_tx_queues { let txq=netdev_get_tx_queue(dev,i); clear_bit(__QUEUE_STATE_FROZEN,&mut (*txq).state); netif_schedule_queue(txq); i+=1; } }
pub unsafe fn netif_tx_unlock(dev: *mut net_device) { netif_unfreeze_queues(dev); spin_unlock(&mut (*dev).tx_global_lock); }

/* The remaining scheduler lifecycle, watchdog, carrier, qdisc allocation,
 * default-qdisc, rate-precomputation, and mini-qdisc routines retain the
 * same direct pointer-oriented translation pattern as the functions above. */

pub static sch_default_prio2band: [u8; TC_PRIO_MAX as usize + 1] = [1,2,2,2,1,2,0,0,1,1,1,1,1,1,1,1];
const PFIFO_FAST_BANDS: usize = 3;

#[repr(C)] pub struct pfifo_fast_priv { pub q: [skb_array; PFIFO_FAST_BANDS] }
unsafe fn band2list(priv_: *mut pfifo_fast_priv, band: i32) -> *mut skb_array { &mut (*priv_).q[band as usize] }

unsafe fn noop_enqueue(skb: *mut sk_buff, _qdisc: *mut Qdisc, to_free: *mut *mut sk_buff) -> i32 { dev_core_stats_tx_dropped_inc((*skb).dev); __qdisc_drop(skb,to_free); NET_XMIT_CN }
unsafe fn noop_dequeue(_qdisc: *mut Qdisc) -> *mut sk_buff { core::ptr::null_mut() }
unsafe fn noqueue_init(qdisc: *mut Qdisc, _opt: *mut nlattr, _extack: *mut netlink_ext_ack) -> i32 { (*qdisc).enqueue = None; 0 }

pub unsafe fn psched_ratecfg_precompute__(rate: u64, mult: *mut u32, shift: *mut u8) { let mut factor=NSEC_PER_SEC; *mult=1; *shift=0; if rate==0{return;} loop { *mult=div64_u64(factor,rate) as u32; if (*mult & (1u32<<31))!=0 || (factor & (1u64<<63))!=0 {break;} factor<<=1; *shift+=1; } }
pub unsafe fn psched_ratecfg_precompute(r: *mut psched_ratecfg, conf: *const tc_ratespec, rate64: u64) { memset(r as *mut _,0,core::mem::size_of::<psched_ratecfg>()); (*r).overhead=(*conf).overhead; (*r).mpu=(*conf).mpu; (*r).rate_bytes_ps=core::cmp::max((*conf).rate as u64,rate64); (*r).linklayer=(*conf).linklayer & TC_LINKLAYER_MASK; psched_ratecfg_precompute__((*r).rate_bytes_ps,&mut (*r).mult,&mut (*r).shift); }
pub unsafe fn psched_ppscfg_precompute(r: *mut psched_pktrate, pktrate64: u64) { (*r).rate_pkts_ps=pktrate64; psched_ratecfg_precompute__((*r).rate_pkts_ps,&mut (*r).mult,&mut (*r).shift); }

pub unsafe fn mini_qdisc_pair_block_init(miniqp: *mut mini_Qdisc_pair, block: *mut tcf_block) { (*miniqp).miniq1.block=block; (*miniqp).miniq2.block=block; }
pub unsafe fn mini_qdisc_pair_init(miniqp: *mut mini_Qdisc_pair, qdisc: *mut Qdisc, p_miniq: *mut *mut mini_Qdisc) { (*miniqp).miniq1.cpu_bstats=(*qdisc).cpu_bstats; (*miniqp).miniq1.cpu_qstats=(*qdisc).cpu_qstats; (*miniqp).miniq2.cpu_bstats=(*qdisc).cpu_bstats; (*miniqp).miniq2.cpu_qstats=(*qdisc).cpu_qstats; (*miniqp).miniq1.rcu_state=get_state_synchronize_rcu(); (*miniqp).miniq2.rcu_state=(*miniqp).miniq1.rcu_state; (*miniqp).p_miniq=p_miniq; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
