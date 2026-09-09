/* SPDX-License-Identifier: GPL-2.0 */
// Translated from net/pkt_sched.h. C header dependencies are supplied externally.

pub const DEFAULT_TX_QUEUE_LEN: i32 = 1000;
pub const STAB_SIZE_LOG_MAX: i32 = 30;
pub const QDISC_PKT_LEN_MAX: i32 = 1 << 20; // 1 MiB

#[repr(C)]
pub struct qdisc_walker {
    pub stop: ::core::ffi::c_int,
    pub skip: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
    pub fn_: Option<unsafe extern "C" fn(*mut Qdisc, ::core::ffi::c_ulong, *mut qdisc_walker) -> ::core::ffi::c_int>,
}

// qdisc_priv(q): const Qdisc yields a const pointer to privdata; mutable Qdisc yields a mutable pointer.

/*
   Timer resolution MUST BE < 10% of min_schedulable_packet_size/bandwidth

   Normal IP packet size ~ 512byte, hence:

   0.5Kbyte/1Mbyte/sec = 0.5msec, so that we need 50usec timer for
   10Mbit ethernet.

   10msec resolution -> <50Kbit/sec.

   The result: [34]86 is not good choice for QoS router :-(

   The things are not so bad, because we may use artificial
   clock evaluated by integration of network data flow
   in the most critical places.
 */
pub type psched_time_t = u64;

pub const PSCHED_SHIFT: u32 = 6;
#[inline]
pub const fn PSCHED_TICKS2NS(x: i64) -> i64 { x << PSCHED_SHIFT }
#[inline]
pub const fn PSCHED_NS2TICKS(x: i64) -> i64 { x >> PSCHED_SHIFT }
pub const PSCHED_TICKS_PER_SEC: i64 = PSCHED_NS2TICKS(NSEC_PER_SEC as i64);
pub const PSCHED_PASTPERFECT: i32 = 0;

#[inline]
pub unsafe fn psched_get_time() -> psched_time_t {
    PSCHED_NS2TICKS(ktime_get_ns() as i64) as u64
}

#[repr(C)]
pub struct qdisc_watchdog { pub timer: hrtimer, pub qdisc: *mut Qdisc }

extern "C" {
    pub fn qdisc_watchdog_init_clockid(wd: *mut qdisc_watchdog, qdisc: *mut Qdisc, clockid: clockid_t);
    pub fn qdisc_watchdog_init(wd: *mut qdisc_watchdog, qdisc: *mut Qdisc);
    pub fn qdisc_watchdog_schedule_range_ns(wd: *mut qdisc_watchdog, expires: u64, delta_ns: u64);
    pub fn qdisc_watchdog_cancel(wd: *mut qdisc_watchdog);
    pub static mut pfifo_qdisc_ops: Qdisc_ops;
    pub static mut bfifo_qdisc_ops: Qdisc_ops;
    pub static mut pfifo_head_drop_qdisc_ops: Qdisc_ops;
    pub fn fifo_set_limit(q: *mut Qdisc, limit: u32) -> ::core::ffi::c_int;
    pub fn fifo_create_dflt(sch: *mut Qdisc, ops: *mut Qdisc_ops, limit: u32, extack: *mut netlink_ext_ack) -> *mut Qdisc;
    pub fn register_qdisc(qops: *mut Qdisc_ops) -> ::core::ffi::c_int;
    pub fn unregister_qdisc(qops: *mut Qdisc_ops);
    pub fn qdisc_get_default(id: *mut ::core::ffi::c_char, len: usize);
    pub fn qdisc_set_default(id: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn qdisc_hash_add(q: *mut Qdisc, invisible: bool);
    pub fn qdisc_hash_del(q: *mut Qdisc);
    pub fn qdisc_lookup(dev: *mut net_device, handle: u32) -> *mut Qdisc;
    pub fn qdisc_lookup_rcu(dev: *mut net_device, handle: u32) -> *mut Qdisc;
    pub fn qdisc_get_rtab(r: *mut tc_ratespec, tab: *mut nlattr, extack: *mut netlink_ext_ack) -> *mut qdisc_rate_table;
    pub fn qdisc_put_rtab(tab: *mut qdisc_rate_table);
    pub fn qdisc_put_stab(tab: *mut qdisc_size_table);
    pub fn sch_direct_xmit(skb: *mut sk_buff, q: *mut Qdisc, dev: *mut net_device, txq: *mut netdev_queue, root_lock: *mut spinlock_t, validate: bool) -> bool;
    pub fn __qdisc_run(q: *mut Qdisc);
    pub static rtm_tca_policy: [nla_policy; (TCA_MAX + 1) as usize];
}

#[inline] pub unsafe fn qdisc_watchdog_schedule_ns(wd: *mut qdisc_watchdog, expires: u64) { qdisc_watchdog_schedule_range_ns(wd, expires, 0); }
#[inline] pub unsafe fn qdisc_watchdog_schedule(wd: *mut qdisc_watchdog, expires: psched_time_t) { qdisc_watchdog_schedule_ns(wd, PSCHED_TICKS2NS(expires as i64) as u64); }

#[inline]
pub unsafe fn qdisc_run(q: *mut Qdisc) -> *mut sk_buff {
    if qdisc_run_begin(q) { __qdisc_run(q); qdisc_run_end(q) } else { core::ptr::null_mut() }
}

#[inline] pub unsafe fn psched_mtu(dev: *const net_device) -> u32 { READ_ONCE((*dev).mtu) + (*dev).hard_header_len }
#[inline] pub unsafe fn qdisc_net(q: *mut Qdisc) -> *mut net { dev_net((*q).dev_queue.dev) }

#[repr(C)] pub struct tc_query_caps_base { pub type_: tc_setup_type, pub caps: *mut ::core::ffi::c_void }
#[repr(C)] pub struct tc_cbs_qopt_offload { pub enable: u8, pub queue: i32, pub hicredit: i32, pub locredit: i32, pub idleslope: i32, pub sendslope: i32 }
#[repr(C)] pub struct tc_etf_qopt_offload { pub enable: u8, pub queue: i32 }
#[repr(C)] pub struct tc_mqprio_caps { pub validate_queue_counts: bool }
#[repr(C)] pub struct tc_mqprio_qopt_offload { pub qopt: tc_mqprio_qopt, pub extack: *mut netlink_ext_ack, pub mode: u16, pub shaper: u16, pub flags: u32, pub min_rate: [u64; TC_QOPT_MAX_QUEUE as usize], pub max_rate: [u64; TC_QOPT_MAX_QUEUE as usize], pub preemptible_tcs: ::core::ffi::c_ulong }
#[repr(C)] pub struct tc_taprio_caps { pub supports_queue_max_sdu: bool, pub gate_mask_per_txq: bool, pub broken_mqprio: bool }
#[repr(C)] pub enum tc_taprio_qopt_cmd { TAPRIO_CMD_REPLACE, TAPRIO_CMD_DESTROY, TAPRIO_CMD_STATS, TAPRIO_CMD_QUEUE_STATS }
#[repr(C)] pub struct tc_taprio_qopt_stats { pub window_drops: u64, pub tx_overruns: u64 }
#[repr(C)] pub struct tc_taprio_qopt_queue_stats { pub queue: ::core::ffi::c_int, pub stats: tc_taprio_qopt_stats }
#[repr(C)] pub struct tc_taprio_sched_entry { pub command: u8, pub gate_mask: u32, pub interval: u32 }
#[repr(C)] pub union tc_taprio_qopt_offload_union { pub stats: tc_taprio_qopt_stats, pub queue_stats: tc_taprio_qopt_queue_stats, pub replace: tc_taprio_qopt_offload_replace }
#[repr(C)] pub struct tc_taprio_qopt_offload_replace { pub mqprio: tc_mqprio_qopt_offload, pub extack: *mut netlink_ext_ack, pub base_time: ktime_t, pub cycle_time: u64, pub cycle_time_extension: u64, pub max_sdu: [u32; TC_MAX_QUEUE as usize], pub num_entries: usize, pub entries: [tc_taprio_sched_entry; 0] }
#[repr(C)] pub struct tc_taprio_qopt_offload { pub cmd: tc_taprio_qopt_cmd, pub data: tc_taprio_qopt_offload_union }

// When CONFIG_NET_SCH_TAPRIO is disabled, these are the C inline stubs returning NULL / doing nothing.
extern "C" { pub fn taprio_offload_get(offload: *mut tc_taprio_qopt_offload) -> *mut tc_taprio_qopt_offload; pub fn taprio_offload_free(offload: *mut tc_taprio_qopt_offload); }

#[inline] pub unsafe fn skb_txtime_consumed(skb: *mut sk_buff) { (*skb).tstamp = ktime_set(0, 0); }
#[inline] pub unsafe fn tc_qdisc_stats_dump(sch: *mut Qdisc, cl: ::core::ffi::c_ulong, arg: *mut qdisc_walker) -> bool { if (*arg).count >= (*arg).skip && ((*arg).fn_.unwrap()(sch, cl, arg) < 0) { (*arg).stop = 1; return false; } (*arg).count += 1; true }
#[inline] pub unsafe fn qdisc_warn_nonwc(txt: *const ::core::ffi::c_char, qdisc: *mut Qdisc) { if (*qdisc).flags & TCQ_F_WARN_NONWC == 0 { pr_warn(txt, (*qdisc).ops.id, (*qdisc).handle >> 16); (*qdisc).flags |= TCQ_F_WARN_NONWC; } }
#[inline] pub unsafe fn qdisc_peek_len(sch: *mut Qdisc) -> u32 { let skb = ((*sch).ops.peek.unwrap())(sch); if skb.is_null() { qdisc_warn_nonwc(c"qdisc_peek_len".as_ptr(), sch); return 0; } qdisc_pkt_len(skb) }
#[inline] pub unsafe fn qdisc_lock_init(sch: *mut Qdisc, ops: *const Qdisc_ops) { spin_lock_init(&mut (*sch).q.lock); if (*ops).static_flags & TCQ_F_INGRESS != 0 || ops == &noqueue_qdisc_ops { return; } lockdep_register_key(&mut (*sch).root_lock_key); lockdep_set_class(&mut (*sch).q.lock, &(*sch).root_lock_key); }
#[inline] pub unsafe fn qdisc_lock_uninit(sch: *mut Qdisc, ops: *const Qdisc_ops) { if (*ops).static_flags & TCQ_F_INGRESS != 0 || ops == &noqueue_qdisc_ops { return; } lockdep_unregister_key(&mut (*sch).root_lock_key); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
