// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/core/gen_stats.c
 *
 * Authors:  Thomas Graf <tgraf@suug.ch>
 *           Jamal Hadi Salim
 *           Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
 *
 * See Documentation/networking/gen_stats.rst
 */

#[inline]
unsafe fn gnet_stats_copy(d: *mut gnet_dump, type_: c_int, buf: *const c_void, size: c_int, padattr: c_int) -> c_int {
    if nla_put_64bit((*d).skb, type_, size, buf, padattr) != 0 {
        if !(*d).lock.is_null() {
            spin_unlock_bh((*d).lock);
        }
        kfree((*d).xstats);
        (*d).xstats = core::ptr::null_mut();
        (*d).xstats_len = 0;
        return -1;
    }
    0
}

pub unsafe fn gnet_stats_start_copy_compat(skb: *mut sk_buff, type_: c_int, tc_stats_type: c_int,
                                           xstats_type: c_int, lock: *mut spinlock_t,
                                           d: *mut gnet_dump, padattr: c_int) -> c_int {
    core::ptr::write_bytes(d, 0, 1);

    if type_ != 0 {
        (*d).tail = skb_tail_pointer(skb) as *mut nlattr;
    }
    (*d).skb = skb;
    (*d).compat_tc_stats = tc_stats_type;
    (*d).compat_xstats = xstats_type;
    (*d).padattr = padattr;
    if !lock.is_null() {
        (*d).lock = lock;
        spin_lock_bh(lock);
    }
    if !(*d).tail.is_null() {
        let ret = gnet_stats_copy(d, type_, core::ptr::null(), 0, padattr);
        if ret == 0 && (*d).tail.as_ref().unwrap().nla_type == padattr as _ {
            (*d).tail = ((*d).tail as *mut u8).add(nla_align((*d).tail.as_ref().unwrap().nla_len as usize)) as *mut nlattr;
        }
        return ret;
    }
    0
}

pub unsafe fn gnet_stats_start_copy(skb: *mut sk_buff, type_: c_int, lock: *mut spinlock_t,
                                    d: *mut gnet_dump, padattr: c_int) -> c_int {
    gnet_stats_start_copy_compat(skb, type_, 0, 0, lock, d, padattr)
}

pub unsafe fn gnet_stats_basic_sync_init(b: *mut gnet_stats_basic_sync) {
    u64_stats_set(&mut (*b).bytes, 0);
    u64_stats_set(&mut (*b).packets, 0);
    u64_stats_init(&mut (*b).syncp);
}

unsafe fn gnet_stats_add_basic_cpu(bstats: *mut gnet_stats_basic_sync, cpu: *const gnet_stats_basic_sync) {
    let mut t_bytes: u64 = 0;
    let mut t_packets: u64 = 0;
    for_each_possible_cpu!(i {
        let bcpu = per_cpu_ptr(cpu, i);
        let mut start: c_uint;
        let (bytes, packets);
        loop {
            start = u64_stats_fetch_begin(&(*bcpu).syncp);
            bytes = u64_stats_read(&(*bcpu).bytes);
            packets = u64_stats_read(&(*bcpu).packets);
            if !u64_stats_fetch_retry(&(*bcpu).syncp, start) { break; }
        }
        t_bytes += bytes;
        t_packets += packets;
    });
    _bstats_update(bstats, t_bytes, t_packets);
}

pub unsafe fn gnet_stats_add_basic(bstats: *mut gnet_stats_basic_sync, cpu: *const gnet_stats_basic_sync,
                                   b: *const gnet_stats_basic_sync, running: bool) {
    warn_on_once!((!cpu.is_null() || running) && in_hardirq());
    if !cpu.is_null() { gnet_stats_add_basic_cpu(bstats, cpu); return; }
    let mut start: c_uint = 0;
    let (bytes, packets);
    loop {
        if running { start = u64_stats_fetch_begin(&(*b).syncp); }
        bytes = u64_stats_read(&(*b).bytes);
        packets = u64_stats_read(&(*b).packets);
        if !(running && u64_stats_fetch_retry(&(*b).syncp, start)) { break; }
    }
    _bstats_update(bstats, bytes, packets);
}

unsafe fn gnet_stats_read_basic(ret_bytes: *mut u64, ret_packets: *mut u64,
                                cpu: *const gnet_stats_basic_sync, b: *const gnet_stats_basic_sync,
                                running: bool) {
    let mut start: c_uint = 0;
    if !cpu.is_null() {
        let (mut t_bytes, mut t_packets) = (0u64, 0u64);
        for_each_possible_cpu!(i {
            let bcpu = per_cpu_ptr(cpu, i);
            let (bytes, packets);
            loop {
                start = u64_stats_fetch_begin(&(*bcpu).syncp);
                bytes = u64_stats_read(&(*bcpu).bytes);
                packets = u64_stats_read(&(*bcpu).packets);
                if !u64_stats_fetch_retry(&(*bcpu).syncp, start) { break; }
            }
            t_bytes += bytes; t_packets += packets;
        });
        *ret_bytes = t_bytes; *ret_packets = t_packets; return;
    }
    loop {
        if running { start = u64_stats_fetch_begin(&(*b).syncp); }
        *ret_bytes = u64_stats_read(&(*b).bytes);
        *ret_packets = u64_stats_read(&(*b).packets);
        if !(running && u64_stats_fetch_retry(&(*b).syncp, start)) { break; }
    }
}

unsafe fn ___gnet_stats_copy_basic(d: *mut gnet_dump, cpu: *const gnet_stats_basic_sync,
                                   b: *const gnet_stats_basic_sync, type_: c_int, running: bool) -> c_int {
    let (mut bstats_bytes, mut bstats_packets) = (0u64, 0u64);
    gnet_stats_read_basic(&mut bstats_bytes, &mut bstats_packets, cpu, b, running);
    if (*d).compat_tc_stats != 0 && type_ == TCA_STATS_BASIC { (*d).tc_stats.bytes = bstats_bytes; (*d).tc_stats.packets = bstats_packets; }
    if !(*d).tail.is_null() {
        let mut sb: gnet_stats_basic = core::mem::zeroed();
        sb.bytes = bstats_bytes; sb.packets = bstats_packets;
        let res = gnet_stats_copy(d, type_, &sb, core::mem::size_of::<gnet_stats_basic>() as _, TCA_STATS_PAD);
        if res < 0 || sb.packets == bstats_packets { return res; }
        return gnet_stats_copy(d, TCA_STATS_PKT64, &bstats_packets, core::mem::size_of::<u64>() as _, TCA_STATS_PAD);
    }
    0
}

pub unsafe fn gnet_stats_copy_basic(d: *mut gnet_dump, cpu: *const gnet_stats_basic_sync, b: *const gnet_stats_basic_sync, running: bool) -> c_int { ___gnet_stats_copy_basic(d, cpu, b, TCA_STATS_BASIC, running) }
pub unsafe fn gnet_stats_copy_basic_hw(d: *mut gnet_dump, cpu: *const gnet_stats_basic_sync, b: *const gnet_stats_basic_sync, running: bool) -> c_int { ___gnet_stats_copy_basic(d, cpu, b, TCA_STATS_BASIC_HW, running) }

pub unsafe fn gnet_stats_copy_rate_est(d: *mut gnet_dump, rate_est: *mut *mut net_rate_estimator) -> c_int {
    let mut sample: gnet_stats_rate_est64 = core::mem::zeroed();
    let mut est: gnet_stats_rate_est = core::mem::zeroed();
    if !gen_estimator_read(rate_est, &mut sample) { return 0; }
    est.bps = core::cmp::min(u32::MAX as u64, sample.bps) as _;
    est.pps = sample.pps;
    if (*d).compat_tc_stats != 0 { (*d).tc_stats.bps = est.bps; (*d).tc_stats.pps = est.pps; }
    if !(*d).tail.is_null() {
        let res = gnet_stats_copy(d, TCA_STATS_RATE_EST, &est, core::mem::size_of::<gnet_stats_rate_est>() as _, TCA_STATS_PAD);
        if res < 0 || est.bps as u64 == sample.bps { return res; }
        return gnet_stats_copy(d, TCA_STATS_RATE_EST64, &sample, core::mem::size_of::<gnet_stats_rate_est64>() as _, TCA_STATS_PAD);
    }
    0
}

unsafe fn gnet_stats_add_queue_cpu(qstats: *mut gnet_stats_queue, q: *const gnet_stats_queue) {
    for_each_possible_cpu!(i {
        let qcpu = per_cpu_ptr(q, i);
        (*qstats).qlen += read_once!((*qcpu).qlen); (*qstats).backlog += read_once!((*qcpu).backlog);
        (*qstats).drops += read_once!((*qcpu).drops); (*qstats).requeues += read_once!((*qcpu).requeues);
        (*qstats).overlimits += read_once!((*qcpu).overlimits);
    });
}

pub unsafe fn gnet_stats_add_queue(qstats: *mut gnet_stats_queue, cpu: *const gnet_stats_queue, q: *const gnet_stats_queue) {
    if !cpu.is_null() { gnet_stats_add_queue_cpu(qstats, cpu); } else {
        (*qstats).qlen += read_once!((*q).qlen); (*qstats).backlog += read_once!((*q).backlog);
        (*qstats).drops += read_once!((*q).drops); (*qstats).requeues += read_once!((*q).requeues);
        (*qstats).overlimits += read_once!((*q).overlimits);
    }
}

pub unsafe fn gnet_stats_copy_queue(d: *mut gnet_dump, cpu_q: *const gnet_stats_queue, q: *const gnet_stats_queue, qlen: u32) -> c_int {
    let mut qstats: gnet_stats_queue = core::mem::zeroed();
    gnet_stats_add_queue(&mut qstats, cpu_q, q); qstats.qlen = qlen;
    if (*d).compat_tc_stats != 0 { (*d).tc_stats.drops = qstats.drops; (*d).tc_stats.qlen = qstats.qlen; (*d).tc_stats.backlog = qstats.backlog; (*d).tc_stats.overlimits = qstats.overlimits; }
    if !(*d).tail.is_null() { return gnet_stats_copy(d, TCA_STATS_QUEUE, &qstats, core::mem::size_of::<gnet_stats_queue>() as _, TCA_STATS_PAD); }
    0
}

pub unsafe fn gnet_stats_copy_app(d: *mut gnet_dump, st: *const c_void, len: c_int) -> c_int {
    if (*d).compat_xstats != 0 { (*d).xstats = kmemdup(st, len, GFP_ATOMIC); if (*d).xstats.is_null() { if !(*d).lock.is_null() { spin_unlock_bh((*d).lock); } (*d).xstats_len = 0; return -1; } (*d).xstats_len = len; }
    if !(*d).tail.is_null() { return gnet_stats_copy(d, TCA_STATS_APP, st, len, TCA_STATS_PAD); }
    0
}

pub unsafe fn gnet_stats_finish_copy(d: *mut gnet_dump) -> c_int {
    if !(*d).tail.is_null() { (*d).tail.as_mut().unwrap().nla_len = (skb_tail_pointer((*d).skb) as usize - (*d).tail as usize) as _; }
    if (*d).compat_tc_stats != 0 && gnet_stats_copy(d, (*d).compat_tc_stats, &(*d).tc_stats, core::mem::size_of_val(&(*d).tc_stats) as _, (*d).padattr) < 0 { return -1; }
    if (*d).compat_xstats != 0 && !(*d).xstats.is_null() && gnet_stats_copy(d, (*d).compat_xstats, (*d).xstats, (*d).xstats_len, (*d).padattr) < 0 { return -1; }
    if !(*d).lock.is_null() { spin_unlock_bh((*d).lock); }
    kfree((*d).xstats); (*d).xstats = core::ptr::null_mut(); (*d).xstats_len = 0; 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
