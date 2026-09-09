/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of net/sch_generic.h. External kernel types and helpers are
 * intentionally referenced as dependencies supplied by other translated files. */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)] pub struct Qdisc_ops { pub next: *mut Qdisc_ops, pub cl_ops: *const Qdisc_class_ops, pub id: [c_char; IFNAMSIZ], pub priv_size: c_int, pub static_flags: u32, pub enqueue: Option<unsafe extern "C" fn(*mut sk_buff,*mut Qdisc,*mut *mut sk_buff)->c_int>, pub dequeue: Option<unsafe extern "C" fn(*mut Qdisc)->*mut sk_buff>, pub peek: Option<unsafe extern "C" fn(*mut Qdisc)->*mut sk_buff>, pub init: Option<unsafe extern "C" fn(*mut Qdisc,*mut nlattr,*mut netlink_ext_ack)->c_int>, pub reset: Option<unsafe extern "C" fn(*mut Qdisc)>, pub destroy: Option<unsafe extern "C" fn(*mut Qdisc)>, pub change: Option<unsafe extern "C" fn(*mut Qdisc,*mut nlattr,*mut netlink_ext_ack)->c_int>, pub attach: Option<unsafe extern "C" fn(*mut Qdisc)>, pub change_tx_queue_len: Option<unsafe extern "C" fn(*mut Qdisc,u32)->c_int>, pub change_real_num_tx: Option<unsafe extern "C" fn(*mut Qdisc,u32)>, pub dump: Option<unsafe extern "C" fn(*mut Qdisc,*mut sk_buff)->c_int>, pub dump_stats: Option<unsafe extern "C" fn(*mut Qdisc,*mut gnet_dump)->c_int>, pub ingress_block_set: Option<unsafe extern "C" fn(*mut Qdisc,u32)>, pub egress_block_set: Option<unsafe extern "C" fn(*mut Qdisc,u32)>, pub ingress_block_get: Option<unsafe extern "C" fn(*mut Qdisc)->u32>, pub egress_block_get: Option<unsafe extern "C" fn(*mut Qdisc)->u32>, pub owner: *mut module }
#[repr(C)] pub struct Qdisc_class_ops { pub flags:u32, pub select_queue: Option<unsafe extern "C" fn(*mut Qdisc,*mut tcmsg)->*mut netdev_queue>, pub graft: Option<unsafe extern "C" fn(*mut Qdisc, c_ulong,*mut Qdisc,*mut *mut Qdisc,*mut netlink_ext_ack)->c_int>, pub leaf: Option<unsafe extern "C" fn(*mut Qdisc,c_ulong)->*mut Qdisc>, pub qlen_notify: Option<unsafe extern "C" fn(*mut Qdisc,c_ulong)>, pub find: Option<unsafe extern "C" fn(*mut Qdisc,u32)->c_ulong>, pub change: Option<unsafe extern "C" fn(*mut Qdisc,u32,u32,*mut *mut nlattr,*mut c_ulong,*mut netlink_ext_ack)->c_int>, pub delete: Option<unsafe extern "C" fn(*mut Qdisc,c_ulong,*mut netlink_ext_ack)->c_int>, pub walk: Option<unsafe extern "C" fn(*mut Qdisc,*mut qdisc_walker)>, pub tcf_block: Option<unsafe extern "C" fn(*mut Qdisc,c_ulong,*mut netlink_ext_ack)->*mut tcf_block>, pub bind_tcf: Option<unsafe extern "C" fn(*mut Qdisc,c_ulong,u32)->c_ulong>, pub unbind_tcf: Option<unsafe extern "C" fn(*mut Qdisc,c_ulong)>, pub dump: Option<unsafe extern "C" fn(*mut Qdisc,c_ulong,*mut sk_buff,*mut tcmsg)->c_int>, pub dump_stats: Option<unsafe extern "C" fn(*mut Qdisc,c_ulong,*mut gnet_dump)->c_int> }

pub type c_ulong = usize;
extern "C" { fn qdisc_reset(*mut Qdisc); fn qdisc_tree_reduce_backlog(*mut Qdisc,u32,u32); fn qdisc_root_sleeping(*const Qdisc)->*mut Qdisc; fn qdisc_lock(*mut Qdisc)->*mut spinlock_t; }

#[repr(C)] pub struct qdisc_rate_table { pub rate: tc_ratespec, pub data:[u32;256], pub next:*mut qdisc_rate_table, pub refcnt:c_int }
#[repr(C)] pub struct qdisc_size_table { pub rcu: rcu_head, pub list:list_head, pub szopts:tc_sizespec, pub refcnt:c_int, pub data:[u16;0] }
#[repr(C)] pub struct qdisc_skb_head { pub head:*mut sk_buff, pub tail:*mut sk_buff, pub qlen:u32, pub lock:spinlock_t }

pub const __QDISC_STATE_SCHED:u32=0; pub const __QDISC_STATE_DEACTIVATED:u32=1; pub const __QDISC_STATE_MISSED:u32=2; pub const __QDISC_STATE_DRAINING:u32=3;
pub const QDISC_STATE_MISSED:u64=1<<__QDISC_STATE_MISSED; pub const QDISC_STATE_DRAINING:u64=1<<__QDISC_STATE_DRAINING; pub const QDISC_STATE_NON_EMPTY:u64=QDISC_STATE_MISSED|QDISC_STATE_DRAINING;
pub const TCQ_F_BUILTIN:u32=1; pub const TCQ_F_INGRESS:u32=2; pub const TCQ_F_CAN_BYPASS:u32=4; pub const TCQ_F_MQROOT:u32=8; pub const TCQ_F_ONETXQUEUE:u32=0x10; pub const TCQ_F_WARN_NONWC:u32=1<<16; pub const TCQ_F_CPUSTATS:u32=0x20; pub const TCQ_F_NOPARENT:u32=0x40; pub const TCQ_F_INVISIBLE:u32=0x80; pub const TCQ_F_NOLOCK:u32=0x100; pub const TCQ_F_OFFLOADED:u32=0x200; pub const TCQ_F_DEQUEUE_DROPS:u32=0x400;

#[repr(C)] pub struct Qdisc { pub enqueue:Option<unsafe extern "C" fn(*mut sk_buff,*mut Qdisc,*mut *mut sk_buff)->c_int>, pub dequeue:Option<unsafe extern "C" fn(*mut Qdisc)->*mut sk_buff>, pub flags:u32, pub limit:u32, pub ops:*const Qdisc_ops, pub stab:*mut qdisc_size_table, pub hash:hlist_node, pub handle:u32, pub parent:u32, pub depth:c_int, pub dev_queue:*mut netdev_queue, pub rate_est:*mut net_rate_estimator, pub cpu_bstats:*mut gnet_stats_basic_sync, pub cpu_qstats:*mut gnet_stats_queue, pub pad:c_int, pub refcnt:refcount_t, pub gso_skb:sk_buff_head, pub next_sched:*mut Qdisc, pub skb_bad_txq:sk_buff_head, pub q:qdisc_skb_head, pub state:usize, pub bstats:gnet_stats_basic_sync, pub running:bool, pub qstats:gnet_stats_queue, pub to_free:*mut sk_buff, pub defer_count:atomic_long_t, pub defer_list:llist_head, pub seqlock:spinlock_t, pub rcu:rcu_head, pub dev_tracker:netdevice_tracker, pub root_lock_key:lock_class_key, pub privdata:[c_long;0] }

pub type c_long=isize; pub const QDISC_CB_PRIV_LEN:usize=20;
#[repr(C)] pub struct qdisc_skb_cb { pub pkt_len:u32, pub pkt_segs:u16, pub tc_classid:u16, pub data:[u8;QDISC_CB_PRIV_LEN], pub slave_dev_queue_mapping:u16, pub post_ct:u8, pub post_ct_snat:u8, pub post_ct_dnat:u8 }
#[repr(C)] pub struct tc_skb_cb { pub qdisc_cb:qdisc_skb_cb, pub drop_reason:u32, pub zone:u16, pub mru:u16 }
#[repr(C)] pub struct psched_ratecfg { pub rate_bytes_ps:u64, pub mult:u32, pub overhead:u16, pub mpu:u16, pub linklayer:u8, pub shift:u8 }
#[repr(C)] pub struct psched_pktrate { pub rate_pkts_ps:u64, pub mult:u32, pub shift:u8 }
#[repr(C)] pub struct mini_Qdisc { pub filter_list:*mut tcf_proto, pub block:*mut tcf_block, pub cpu_bstats:*mut gnet_stats_basic_sync, pub cpu_qstats:*mut gnet_stats_queue, pub rcu_state:usize }
#[repr(C)] pub struct mini_Qdisc_pair { pub miniq1:mini_Qdisc, pub miniq2:mini_Qdisc, pub p_miniq:*mut *mut mini_Qdisc }

#[inline] pub unsafe fn qdisc_refcount_inc(q:*mut Qdisc){ if (*q).flags&TCQ_F_BUILTIN==0 { refcount_inc(&mut (*q).refcnt); } }
#[inline] pub unsafe fn qdisc_refcount_dec_if_one(q:*mut Qdisc)->bool{ if (*q).flags&TCQ_F_BUILTIN!=0 {true}else{refcount_dec_if_one(&mut (*q).refcnt)} }
#[inline] pub unsafe fn qdisc_refcount_inc_nz(q:*mut Qdisc)->*mut Qdisc{ if (*q).flags&TCQ_F_BUILTIN!=0||refcount_inc_not_zero(&mut (*q).refcnt){q}else{core::ptr::null_mut()} }
#[inline] pub unsafe fn qdisc_is_percpu_stats(q:*const Qdisc)->bool{(*q).flags&TCQ_F_CPUSTATS!=0}
#[inline] pub unsafe fn qdisc_is_empty(q:*const Qdisc)->bool{ if qdisc_is_percpu_stats(q){((*q).state as u64&QDISC_STATE_NON_EMPTY)==0}else{(*q).q.qlen==0} }
#[inline] pub unsafe fn qdisc_qlen(q:*const Qdisc)->i32{(*q).q.qlen as i32}
#[inline] pub unsafe fn qdisc_qlen_inc(q:*mut Qdisc){(*q).q.qlen=(*q).q.qlen.wrapping_add(1)}
#[inline] pub unsafe fn qdisc_qlen_dec(q:*mut Qdisc){(*q).q.qlen=(*q).q.qlen.wrapping_sub(1)}
#[inline] pub unsafe fn qdisc_skb_cb(skb:*const sk_buff)->*mut qdisc_skb_cb{(*skb).cb.as_ptr() as *mut qdisc_skb_cb}
#[inline] pub unsafe fn qdisc_pkt_len(skb:*const sk_buff)->u32{(*qdisc_skb_cb(skb)).pkt_len}
#[inline] pub unsafe fn qdisc_pkt_segs(skb:*const sk_buff)->u32{(*qdisc_skb_cb(skb)).pkt_segs as u32}
#[inline] pub unsafe fn tc_skb_cb(skb:*const sk_buff)->*mut tc_skb_cb{(*skb).cb.as_ptr() as *mut tc_skb_cb}
#[inline] pub unsafe fn tcf_get_drop_reason(skb:*const sk_buff)->u32{(*tc_skb_cb(skb)).drop_reason}
#[inline] pub unsafe fn tcf_set_drop_reason(skb:*const sk_buff,reason:u32){(*tc_skb_cb(skb)).drop_reason=reason}
#[inline] pub unsafe fn psched_l2t_ns(r:*const psched_ratecfg,mut len:u32)->u64{len+=(*r).overhead as u32;if len<(*r).mpu as u32{len=(*r).mpu as u32} ;((len as u64)*(*r).mult as u64)>>(*r).shift}
#[inline] pub unsafe fn psched_pkt2t_ns(r:*const psched_pktrate,pkt_num:u32)->u64{((pkt_num as u64)*(*r).mult as u64)>>(*r).shift}

/* The remaining declarations retain the header's external kernel interfaces. */
extern "C" { pub static mut noop_qdisc:Qdisc; pub static mut noop_qdisc_ops:Qdisc_ops; pub static mut default_qdisc_ops:*const Qdisc_ops; pub fn qdisc_alloc(*mut netdev_queue,*const Qdisc_ops,*mut netlink_ext_ack)->*mut Qdisc; pub fn qdisc_free(*mut Qdisc); pub fn qdisc_destroy(*mut Qdisc); pub fn qdisc_put(*mut Qdisc); pub fn dev_init_scheduler(*mut net_device); pub fn dev_shutdown(*mut net_device); pub fn dev_activate(*mut net_device); pub fn dev_deactivate(*mut net_device,bool); pub fn qdisc_enqueue_tail(*mut sk_buff,*mut Qdisc)->c_int; }

/* External types supplied by the kernel translation units. */
#[allow(non_camel_case_types)] pub type IFNAMSIZ=usize;
extern "C" { fn refcount_inc(*mut refcount_t); fn refcount_dec_if_one(*mut refcount_t)->bool; fn refcount_inc_not_zero(*mut refcount_t)->bool; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
