// SPDX-License-Identifier: GPL-2.0
// Linux kernel dependencies are supplied by the surrounding translation unit.

const TCP_METRIC_MAX_KERNEL: usize = TCP_METRIC_MAX - 2;

#[repr(C)]
pub struct tcp_fastopen_metrics {
    pub mss: u16,
    pub syn_loss: u16,
    pub try_exp: u16,
    pub last_syn_loss: ::core::ffi::c_ulong,
    pub cookie: tcp_fastopen_cookie,
}

#[repr(C)]
pub struct tcp_metrics_block {
    pub tcpm_next: *mut tcp_metrics_block,
    pub tcpm_net: *mut net,
    pub tcpm_saddr: inetpeer_addr,
    pub tcpm_daddr: inetpeer_addr,
    pub tcpm_stamp: ::core::ffi::c_ulong,
    pub tcpm_lock: u32,
    pub tcpm_vals: [u32; TCP_METRIC_MAX_KERNEL + 1],
    pub tcpm_fastopen: tcp_fastopen_metrics,
    pub rcu_head: rcu_head,
}

#[inline]
unsafe fn tm_net(tm: *const tcp_metrics_block) -> *mut net {
    // Paired with the WRITE_ONCE() in tcpm_new().
    READ_ONCE((*tm).tcpm_net)
}

unsafe fn tcp_metric_locked(tm: *mut tcp_metrics_block, idx: tcp_metric_index) -> bool {
    // Paired with WRITE_ONCE() in tcpm_suck_dst().
    (READ_ONCE((*tm).tcpm_lock) & (1u32 << idx as u32)) != 0
}

unsafe fn tcp_metric_get(tm: *const tcp_metrics_block, idx: tcp_metric_index) -> u32 {
    // Paired with WRITE_ONCE() in tcp_metric_set().
    READ_ONCE((*tm).tcpm_vals[idx as usize])
}

unsafe fn tcp_metric_set(tm: *mut tcp_metrics_block, idx: tcp_metric_index, val: u32) {
    // Paired with READ_ONCE() in tcp_metric_get().
    WRITE_ONCE((*tm).tcpm_vals[idx as usize], val);
}

unsafe fn addr_same(a: *const inetpeer_addr, b: *const inetpeer_addr) -> bool {
    (*a).family == (*b).family && inetpeer_addr_cmp(a, b) == 0
}

#[repr(C)]
pub struct tcpm_hash_bucket {
    pub chain: *mut tcp_metrics_block,
}

static mut tcp_metrics_hash: *mut tcpm_hash_bucket = core::ptr::null_mut();
static mut tcp_metrics_hash_log: u32 = 0;
static mut tcp_metrics_lock: spinlock_t = DEFINE_SPINLOCK!();
static mut fastopen_seqlock: seqlock_t = DEFINE_SEQLOCK!();

unsafe fn tcpm_suck_dst(tm: *mut tcp_metrics_block, dst: *const dst_entry, fastopen_clear: bool) {
    let mut msval: u32;
    let mut val: u32 = 0;
    WRITE_ONCE((*tm).tcpm_stamp, jiffies);
    if dst_metric_locked(dst, RTAX_RTT) { val |= 1 << TCP_METRIC_RTT as u32; }
    if dst_metric_locked(dst, RTAX_RTTVAR) { val |= 1 << TCP_METRIC_RTTVAR as u32; }
    if dst_metric_locked(dst, RTAX_SSTHRESH) { val |= 1 << TCP_METRIC_SSTHRESH as u32; }
    if dst_metric_locked(dst, RTAX_CWND) { val |= 1 << TCP_METRIC_CWND as u32; }
    if dst_metric_locked(dst, RTAX_REORDERING) { val |= 1 << TCP_METRIC_REORDERING as u32; }
    WRITE_ONCE((*tm).tcpm_lock, val);
    msval = dst_metric_raw(dst, RTAX_RTT); tcp_metric_set(tm, TCP_METRIC_RTT, msval * USEC_PER_MSEC);
    msval = dst_metric_raw(dst, RTAX_RTTVAR); tcp_metric_set(tm, TCP_METRIC_RTTVAR, msval * USEC_PER_MSEC);
    tcp_metric_set(tm, TCP_METRIC_SSTHRESH, dst_metric_raw(dst, RTAX_SSTHRESH));
    tcp_metric_set(tm, TCP_METRIC_CWND, dst_metric_raw(dst, RTAX_CWND));
    tcp_metric_set(tm, TCP_METRIC_REORDERING, dst_metric_raw(dst, RTAX_REORDERING));
    if fastopen_clear {
        write_seqlock(&mut fastopen_seqlock);
        (*tm).tcpm_fastopen.mss = 0; (*tm).tcpm_fastopen.syn_loss = 0; (*tm).tcpm_fastopen.try_exp = 0;
        (*tm).tcpm_fastopen.cookie.exp = false; (*tm).tcpm_fastopen.cookie.len = 0;
        write_sequnlock(&mut fastopen_seqlock);
    }
}

const TCP_METRICS_TIMEOUT: ::core::ffi::c_ulong = 60 * 60 * HZ;
unsafe fn tcpm_check_stamp(tm: *mut tcp_metrics_block, dst: *const dst_entry) {
    if tm.is_null() { return; }
    let limit = READ_ONCE((*tm).tcpm_stamp) + TCP_METRICS_TIMEOUT;
    if time_after(jiffies, limit) { tcpm_suck_dst(tm, dst, false); }
}

const TCP_METRICS_RECLAIM_DEPTH: i32 = 5;
const TCP_METRICS_RECLAIM_PTR: *mut tcp_metrics_block = 1usize as *mut tcp_metrics_block;

unsafe fn __tcp_get_metrics(saddr: *const inetpeer_addr, daddr: *const inetpeer_addr,
                            netp: *mut net, hash: u32) -> *mut tcp_metrics_block {
    let mut tm = rcu_dereference((*tcp_metrics_hash.add(hash as usize)).chain);
    let mut depth = 0;
    while !tm.is_null() {
        if addr_same(&(*tm).tcpm_saddr, saddr) && addr_same(&(*tm).tcpm_daddr, daddr) && net_eq(tm_net(tm), netp) { break; }
        depth += 1; tm = rcu_dereference((*tm).tcpm_next);
    }
    if !tm.is_null() { tm } else if depth > TCP_METRICS_RECLAIM_DEPTH { TCP_METRICS_RECLAIM_PTR } else { core::ptr::null_mut() }
}

unsafe fn tcpm_new(dst: *mut dst_entry, saddr: *mut inetpeer_addr, daddr: *mut inetpeer_addr, hash: u32) -> *mut tcp_metrics_block {
    let mut tm: *mut tcp_metrics_block;
    let mut reclaim = false;
    spin_lock_bh(&mut tcp_metrics_lock);
    let netp = dst_dev_net_rcu(dst);
    tm = __tcp_get_metrics(saddr, daddr, netp, hash);
    if tm == TCP_METRICS_RECLAIM_PTR { reclaim = true; tm = core::ptr::null_mut(); }
    if !tm.is_null() { tcpm_check_stamp(tm, dst); spin_unlock_bh(&mut tcp_metrics_lock); return tm; }
    if reclaim {
        let mut oldest = (*tcp_metrics_hash.add(hash as usize)).chain;
        tm = (*oldest).tcpm_next;
        while !tm.is_null() { if time_before(READ_ONCE((*tm).tcpm_stamp), READ_ONCE((*oldest).tcpm_stamp)) { oldest = tm; } tm = (*tm).tcpm_next; }
        tm = oldest;
    } else {
        tm = kzalloc_obj::<tcp_metrics_block>();
        if tm.is_null() { spin_unlock_bh(&mut tcp_metrics_lock); return tm; }
    }
    WRITE_ONCE((*tm).tcpm_net, netp); (*tm).tcpm_saddr = *saddr; (*tm).tcpm_daddr = *daddr;
    tcpm_suck_dst(tm, dst, reclaim);
    if !reclaim { (*tm).tcpm_next = (*tcp_metrics_hash.add(hash as usize)).chain; rcu_assign_pointer(&mut (*tcp_metrics_hash.add(hash as usize)).chain, tm); }
    spin_unlock_bh(&mut tcp_metrics_lock); tm
}

unsafe fn __tcp_get_metrics_req(req: *mut request_sock, dst: *mut dst_entry) -> *mut tcp_metrics_block {
    let mut saddr: inetpeer_addr = core::mem::zeroed(); let mut daddr: inetpeer_addr = core::mem::zeroed();
    let family = (*(*req).rsk_ops).family; saddr.family = family; daddr.family = family;
    let hash = match family { AF_INET => { inetpeer_set_addr_v4(&mut saddr, (*inet_rsk(req)).ir_loc_addr); inetpeer_set_addr_v4(&mut daddr, (*inet_rsk(req)).ir_rmt_addr); ipv4_addr_hash((*inet_rsk(req)).ir_rmt_addr) }, AF_INET6 => { inetpeer_set_addr_v6(&mut saddr, &(*inet_rsk(req)).ir_v6_loc_addr); inetpeer_set_addr_v6(&mut daddr, &(*inet_rsk(req)).ir_v6_rmt_addr); ipv6_addr_hash(&(*inet_rsk(req)).ir_v6_rmt_addr) }, _ => return core::ptr::null_mut() };
    let netp = dst_dev_net_rcu(dst); let hash = hash_32(hash ^ net_hash_mix(netp), tcp_metrics_hash_log);
    let mut tm = rcu_dereference((*tcp_metrics_hash.add(hash as usize)).chain);
    while !tm.is_null() { if addr_same(&(*tm).tcpm_saddr, &saddr) && addr_same(&(*tm).tcpm_daddr, &daddr) && net_eq(tm_net(tm), netp) { break; } tm = rcu_dereference((*tm).tcpm_next); }
    tcpm_check_stamp(tm, dst); tm
}

unsafe fn tcp_get_metrics(sk: *mut sock, dst: *mut dst_entry, create: bool) -> *mut tcp_metrics_block {
    let mut saddr: inetpeer_addr = core::mem::zeroed(); let mut daddr: inetpeer_addr = core::mem::zeroed();
    let hash = if (*sk).sk_family == AF_INET { inetpeer_set_addr_v4(&mut saddr, (*inet_sk(sk)).inet_saddr); inetpeer_set_addr_v4(&mut daddr, (*inet_sk(sk)).inet_daddr); ipv4_addr_hash((*inet_sk(sk)).inet_daddr) } else { return core::ptr::null_mut() };
    let netp = dst_dev_net_rcu(dst); let hash = hash_32(hash ^ net_hash_mix(netp), tcp_metrics_hash_log);
    let mut tm = __tcp_get_metrics(&saddr, &daddr, netp, hash);
    if tm == TCP_METRICS_RECLAIM_PTR { tm = core::ptr::null_mut(); }
    if tm.is_null() && create { tm = tcpm_new(dst, &mut saddr, &mut daddr, hash); } else { tcpm_check_stamp(tm, dst); } tm
}

pub unsafe fn tcp_peer_is_proven(req: *mut request_sock, dst: *mut dst_entry) -> bool {
    if dst.is_null() { return false; } let tm = __tcp_get_metrics_req(req, dst); !tm.is_null() && tcp_metric_get(tm, TCP_METRIC_RTT) != 0
}

pub unsafe fn tcp_update_metrics(sk: *mut sock) {
    let dst = __sk_dst_get(sk); if dst.is_null() { return; }
    let tp = tcp_sk(sk); let tm = tcp_get_metrics(sk, dst, true); if tm.is_null() { return; }
    if !tcp_metric_locked(tm, TCP_METRIC_RTT) { tcp_metric_set(tm, TCP_METRIC_RTT, (*tp).srtt_us); }
    if !tcp_metric_locked(tm, TCP_METRIC_RTTVAR) { tcp_metric_set(tm, TCP_METRIC_RTTVAR, (*tp).mdev_us); }
    WRITE_ONCE((*tm).tcpm_stamp, jiffies);
}

pub unsafe fn tcp_init_metrics(sk: *mut sock) {
    let tp = tcp_sk(sk); (*tp).snd_ssthresh = TCP_INFINITE_SSTHRESH;
    let dst = __sk_dst_get(sk); if dst.is_null() { return; }
    let tm = tcp_get_metrics(sk, dst, false); if tm.is_null() { return; }
    if tcp_metric_locked(tm, TCP_METRIC_CWND) { (*tp).snd_cwnd_clamp = tcp_metric_get(tm, TCP_METRIC_CWND); }
    let val = tcp_metric_get(tm, TCP_METRIC_SSTHRESH); if val != 0 { (*tp).snd_ssthresh = val.min((*tp).snd_cwnd_clamp); }
    let val = tcp_metric_get(tm, TCP_METRIC_REORDERING); if val != 0 { (*tp).reordering = val; }
    let crtt = tcp_metric_get(tm, TCP_METRIC_RTT); if crtt > (*tp).srtt_us { (*inet_csk(sk)).icsk_rto = crtt / (8 * USEC_PER_SEC / HZ); }
}

extern "C" {
    static mut tcp_metrics_nl_family: genl_family;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
