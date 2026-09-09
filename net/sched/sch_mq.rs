// SPDX-License-Identifier: GPL-2.0-only
/*
 * net/sched/sch_mq.c  Classful multiqueue dummy scheduler
 *
 * Copyright (c) 2009 Patrick McHardy <kaber@trash.net>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)] pub struct Qdisc { _private: [u8; 0] }
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct netdev_queue { _private: [u8; 0] }
#[repr(C)] pub struct nlattr { _private: [u8; 0] }
#[repr(C)] pub struct netlink_ext_ack { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct tcmsg { pub tcm_parent: u32, pub tcm_handle: u32, pub tcm_info: u32 }
#[repr(C)] pub struct gnet_dump { _private: [u8; 0] }
#[repr(C)] pub struct qdisc_walker { pub stop: bool, pub skip: c_uint, pub count: c_uint }
#[repr(C)] pub struct Qdisc_ops { _private: [u8; 0] }
#[repr(C)] pub struct Qdisc_class_ops { _private: [u8; 0] }
#[repr(C)] pub struct gnet_stats_basic_sync { pub bytes: u64, pub packets: u64 }
#[repr(C)] pub struct gnet_stats_queue { pub qlen: u32, pub backlog: u32, pub drops: u32, pub requeues: u32, pub overlimits: u32 }
#[repr(C)] pub struct mq_sched { pub qdiscs: *mut *mut Qdisc }

#[repr(C)] pub struct tc_mq_qopt_offload {
    pub command: c_int,
    pub handle: u32,
    pub stats: mq_stats,
    pub graft_params: mq_graft_params,
}
#[repr(C)] pub struct mq_stats { pub bstats: *mut c_void, pub qstats: *mut c_void }
#[repr(C)] pub struct mq_graft_params { pub queue: c_ulong, pub child_handle: u32 }

extern "C" {
    fn qdisc_dev(sch: *mut Qdisc) -> *mut net_device;
    fn qdisc_priv(sch: *mut Qdisc) -> *mut mq_sched;
    fn tc_can_offload(dev: *mut net_device) -> bool;
    fn netdev_get_tx_queue(dev: *mut net_device, n: c_uint) -> *mut netdev_queue;
    fn qdisc_offload_dump_helper(sch: *mut Qdisc, kind: c_int, opt: *mut tc_mq_qopt_offload) -> c_int;
    fn qdisc_put(q: *mut Qdisc);
    fn kfree(p: *mut c_void);
    fn kzalloc_objs<T>(p: T, n: c_uint) -> T;
    fn qdisc_create_dflt(q: *mut netdev_queue, ops: *const Qdisc_ops, handle: u32, extack: *mut netlink_ext_ack) -> *mut Qdisc;
    fn get_default_qdisc_ops(dev: *mut net_device, n: c_uint) -> *const Qdisc_ops;
    fn dev_graft_qdisc(q: *mut netdev_queue, new: *mut Qdisc) -> *mut Qdisc;
    fn qdisc_hash_add(q: *mut Qdisc, invisible: bool);
    fn gnet_stats_basic_sync_init(s: *mut gnet_stats_basic_sync);
    fn gnet_stats_add_basic(s: *mut gnet_stats_basic_sync, cpu: *mut c_void, b: *mut c_void, running: bool);
    fn gnet_stats_add_queue(s: *mut gnet_stats_queue, cpu: *mut c_void, q: *mut c_void);
    fn qdisc_qlen_lockless(q: *const Qdisc) -> c_uint;
    fn qdisc_lock(s: *mut Qdisc) -> *mut c_void;
    fn spin_lock_bh(lock: *mut c_void);
    fn spin_unlock_bh(lock: *mut c_void);
    fn u64_stats_read(p: *const u64) -> u64;
    fn _bstats_set(b: *mut c_void, bytes: u64, packets: u64);
    fn qdisc_offload_graft_helper(dev: *mut net_device, sch: *mut Qdisc, new: *mut Qdisc, old: *mut Qdisc, kind: c_int, opt: *mut tc_mq_qopt_offload, extack: *mut netlink_ext_ack);
    fn dev_deactivate(dev: *mut net_device, remove: bool);
    fn dev_activate(dev: *mut net_device);
    fn rtnl_dereference<T>(p: *mut T) -> *mut T;
    fn gnet_stats_copy_basic(d: *mut gnet_dump, cpu: *mut c_void, b: *mut c_void, running: bool) -> c_int;
    fn qdisc_qstats_copy(d: *mut gnet_dump, sch: *mut Qdisc) -> c_int;
    fn tc_qdisc_stats_dump(sch: *mut Qdisc, cl: c_ulong, arg: *mut qdisc_walker) -> bool;
}

const TC_MQ_CREATE: c_int = 0;
const TC_MQ_DESTROY: c_int = 1;
const TC_MQ_STATS: c_int = 2;
const TC_MQ_GRAFT: c_int = 3;
const TC_SETUP_QDISC_MQ: c_int = 0;
const TC_H_ROOT: u32 = 0xffff_ffff;
const TCQ_F_ONETXQUEUE: u32 = 1 << 0;
const TCQ_F_NOPARENT: u32 = 1 << 1;
const TCQ_F_MQROOT: u32 = 1 << 2;
const IFF_UP: u32 = 1;
const EOPNOTSUPP: c_int = 95;
const ENOMEM: c_int = 12;

#[inline] unsafe fn tc_h_maj(h: u32) -> u32 { h & 0xffff_0000 }
#[inline] unsafe fn tc_h_min(h: u32) -> u32 { h & 0x0000_ffff }
#[inline] unsafe fn tc_h_make(maj: u32, min: u32) -> u32 { maj | min }

unsafe fn mq_offload(sch: *mut Qdisc, cmd: c_int) -> c_int {
    let dev = qdisc_dev(sch);
    let mut opt = tc_mq_qopt_offload { command: cmd, handle: 0, stats: mq_stats { bstats: core::ptr::null_mut(), qstats: core::ptr::null_mut() }, graft_params: mq_graft_params { queue: 0, child_handle: 0 } };
    if !tc_can_offload(dev) { return -EOPNOTSUPP; }
    qdisc_offload_dump_helper(sch, TC_SETUP_QDISC_MQ, &mut opt)
}

unsafe fn mq_offload_stats(sch: *mut Qdisc) -> c_int { mq_offload(sch, TC_MQ_STATS) }

#[no_mangle] pub unsafe extern "C" fn mq_destroy_common(sch: *mut Qdisc) {
    let dev = qdisc_dev(sch); let priv_ = qdisc_priv(sch);
    if (*priv_).qdiscs.is_null() { return; }
    let mut ntx = 0;
    while ntx < 0 { qdisc_put(*(*priv_).qdiscs.add(ntx as usize)); ntx += 1; }
    kfree((*priv_).qdiscs as *mut c_void);
}

unsafe fn mq_destroy(sch: *mut Qdisc) { mq_offload(sch, TC_MQ_DESTROY); mq_destroy_common(sch); }

#[no_mangle] pub unsafe extern "C" fn mq_init_common(sch: *mut Qdisc, _opt: *mut nlattr, _extack: *mut netlink_ext_ack, _qdisc_ops: *const Qdisc_ops) -> c_int { if (*qdisc_dev(sch)).is_null() { return -EOPNOTSUPP; } 0 }
#[no_mangle] pub unsafe extern "C" fn mq_attach(_sch: *mut Qdisc) {}
#[no_mangle] pub unsafe extern "C" fn mq_dump_common(_sch: *mut Qdisc, _skb: *mut sk_buff) {}
#[no_mangle] pub unsafe extern "C" fn mq_select_queue(_sch: *mut Qdisc, _tcm: *mut tcmsg) -> *mut netdev_queue { core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn mq_find(_sch: *mut Qdisc, _classid: u32) -> c_ulong { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
