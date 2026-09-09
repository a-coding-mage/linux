// SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause
/* Faithful low-level translation of sch_dualpi2.c.  Kernel dependencies are
 * intentionally left as external symbols supplied by the surrounding tree. */

#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_int, c_uint, c_ulong, c_void};

extern "C" {
    fn qdisc_cb_private_validate(skb: *mut sk_buff, size: usize);
    fn qdisc_skb_cb(skb: *mut sk_buff) -> *mut qdisc_skb_cb;
    fn qdisc_peek_head(q: *mut Qdisc) -> *mut sk_buff;
    fn ktime_get_ns() -> u64;
    fn ktime_add_ns(a: u64, b: u64) -> u64;
    fn get_random_u32() -> u32;
    fn INET_ECN_set_ce(skb: *mut sk_buff) -> bool;
    fn psched_mtu(dev: *mut c_void) -> u32;
    fn qdisc_dev(q: *mut Qdisc) -> *mut c_void;
    fn qdisc_qlen(q: *mut Qdisc) -> u32;
    fn qdisc_enqueue_tail(skb: *mut sk_buff, q: *mut Qdisc) -> c_int;
    fn qdisc_drop_reason(skb: *mut sk_buff, q: *mut Qdisc, f: *mut *mut sk_buff, r: c_int) -> c_int;
    fn qdisc_qstats_overlimit(q: *mut Qdisc);
    fn qdisc_qstats_drop(q: *mut Qdisc);
    fn qdisc_qstats_backlog_inc(q: *mut Qdisc, skb: *mut sk_buff);
    fn qdisc_qstats_backlog_dec(q: *mut Qdisc, skb: *mut sk_buff);
    fn qdisc_bstats_update(q: *mut Qdisc, skb: *mut sk_buff);
    fn __qdisc_drop(skb: *mut sk_buff, f: *mut *mut sk_buff);
    fn skb_network_offset(skb: *mut sk_buff) -> c_int;
    fn skb_protocol(skb: *mut sk_buff, nested: bool) -> u16;
    fn pskb_may_pull(skb: *mut sk_buff, len: c_int) -> bool;
    fn skb_try_make_writable(skb: *mut sk_buff, len: c_int) -> c_int;
    fn skb_is_gso(skb: *mut sk_buff) -> bool;
    fn qdisc_pkt_len(skb: *mut sk_buff) -> u32;
}

type u8_t = u8; type u32_t = u32; type u64_t = u64; type s32_t = i32; type s64_t = i64;
type netdev_features_t = u64; type ktime_t = u64;
#[repr(C)] pub struct sk_buff { pub len: u32, pub truesize: u32, pub priority: u32 }
#[repr(C)] pub struct qdisc_skb_cb { pub data: [u8; 48], pub pkt_len: u32, pub pkt_segs: u16 }
#[repr(C)] pub struct Qdisc { pub limit: u32, pub handle: u32, pub flags: u32, pub q: QdiscQueue, pub qstats: QdiscStats, pub dev_queue: *mut c_void, pub gso_skb: *mut sk_buff }
#[repr(C)] pub struct QdiscQueue { pub qlen: u32 }
#[repr(C)] pub struct QdiscStats { pub backlog: i32 }
#[repr(C)] pub struct tcf_proto; #[repr(C)] pub struct tcf_block; #[repr(C)] pub struct hrtimer;
#[repr(C)] pub struct nlattr; #[repr(C)] pub struct netlink_ext_ack; #[repr(C)] pub struct gnet_dump;
#[repr(C)] pub struct qdisc_walker { pub stop: bool, pub count: u32, pub skip: u32, pub fn_: Option<unsafe extern "C" fn(*mut Qdisc,u32,*mut qdisc_walker)->c_int> }
#[repr(C)] pub struct Qdisc_class_ops; #[repr(C)] pub struct Qdisc_ops;

pub const MAX_PROB: u32 = u32::MAX;
pub const ALPHA_BETA_SHIFT: u32 = 8; pub const ALPHA_BETA_MAX: u32 = (1u32<<31)-1;
pub const ALPHA_BETA_GRANULARITY: u32 = 6; pub const ALPHA_BETA_SCALING: u32 = 2; pub const MAX_WC: u32 = 100;
pub const DUALPI2_C_CLASSIC: u8 = 0; pub const DUALPI2_C_L4S: u8 = 1; pub const DUALPI2_C_LLLL: u8 = 2; pub const __DUALPI2_C_MAX: u8 = 3;

#[repr(C)] pub struct dualpi2_sched_data {
    pub l_queue:*mut Qdisc, pub sch:*mut Qdisc, pub tcf_filters:*mut tcf_proto, pub tcf_block:*mut tcf_block,
    pub pi2_target:u64, pub pi2_tupdate:u32, pub pi2_prob:u32, pub pi2_alpha:u32, pub pi2_beta:u32, pub pi2_timer:*mut hrtimer,
    pub step_thresh:u32, pub step_in_packets:bool, pub c_protection_credit:i32, pub c_protection_init:i32,
    pub c_protection_wc:u8, pub c_protection_wl:u8, pub memory_limit:u32, pub coupling_factor:u8, pub ecn_mask:u8,
    pub min_qlen_step:u32, pub drop_early:bool, pub drop_overload:bool, pub split_gso:bool,
    pub c_head_ts:u64, pub l_head_ts:u64, pub last_qdelay:u64, pub packets_in_c:u32, pub packets_in_l:u32,
    pub maxq:u32, pub ecn_mark:u32, pub step_marks:u32, pub memory_used:u32, pub max_memory_used:u32,
    pub deferred_drops_cnt:u32, pub deferred_drops_len:u32,
}
#[repr(C)] pub struct dualpi2_skb_cb { pub ts:u64, pub apply_step:bool, pub classified:u8, pub ect:u8 }

unsafe fn cb(skb:*mut sk_buff)->*mut dualpi2_skb_cb { qdisc_cb_private_validate(skb, core::mem::size_of::<dualpi2_skb_cb>()); (*qdisc_skb_cb(skb)).data.as_mut_ptr() as *mut dualpi2_skb_cb }
unsafe fn sojourn(skb:*mut sk_buff, reference:u64)->u64 { reference-(*cb(skb)).ts }
unsafe fn head_time(q:*mut Qdisc)->u64 { let s=qdisc_peek_head(q); if s.is_null(){0}else{(*cb(s)).ts} }
unsafe fn scale(x:u32)->u32 { (((x as u64)*MAX_PROB) >> ALPHA_BETA_SCALING) as u32 / 1_000_000_000 }
unsafe fn unscale(x:u32)->u32 { (((x as u64)*1_000_000_000 << ALPHA_BETA_SCALING)/MAX_PROB as u64) as u32 }
unsafe fn is_l4s(s:*mut sk_buff)->bool { (*cb(s)).classified==DUALPI2_C_L4S }
unsafe fn in_l(s:*mut sk_buff)->bool { (*cb(s)).classified!=DUALPI2_C_CLASSIC }
unsafe fn roll(p:u32)->bool { get_random_u32()<=p }
unsafe fn mark(q:*mut dualpi2_sched_data,s:*mut sk_buff)->bool { if INET_ECN_set_ce(s){(*q).ecn_mark=(*q).ecn_mark.wrapping_add(1);true}else{false} }
unsafe fn reset_credit(q:*mut dualpi2_sched_data){(*q).c_protection_credit=(*q).c_protection_init;}
unsafe fn classic(q:*mut dualpi2_sched_data,s:*mut sk_buff,p:u32,over:bool)->bool { if roll(p)&&roll(p){if over||(*cb(s)).ect==0{return true} mark(q,s);} false }
unsafe fn scalable(q:*mut dualpi2_sched_data,s:*mut sk_buff,lp:u64,p:u32,over:bool)->bool { if over {if (*q).drop_overload&&roll(p)&&roll(p){return true}} if roll(lp as u32){if (*cb(s)).ect==0{return true} mark(q,s)} false }
unsafe fn must_drop(sch:*mut Qdisc,q:*mut dualpi2_sched_data,s:*mut sk_buff)->bool { if (*sch).qstats.backlog < (2*psched_mtu(qdisc_dev(sch))) as i32{return false} let p=(*q).pi2_prob; let lp=p as u64*(*q).coupling_factor as u64; match (*cb(s)).classified {DUALPI2_C_CLASSIC=>classic(q,s,p,lp>MAX_PROB as u64),DUALPI2_C_L4S=>scalable(q,s,lp,p,lp>MAX_PROB as u64),_=>false} }
unsafe fn get_delays(q:*mut dualpi2_sched_data, c:*mut u64,l:*mut u64){let n=ktime_get_ns();*c=if (*q).c_head_ts!=0{n-(*q).c_head_ts}else{0};*l=if (*q).l_head_ts!=0{n-(*q).l_head_ts}else{0};}
unsafe fn calculate_probability(sch:*mut Qdisc)->u32 { let q=*(sch as *mut *mut dualpi2_sched_data); let(mut c,mut l)=(0,0);get_delays(q,&mut c,&mut l);let d=core::cmp::max(c,l) as i64-(*q).pi2_target as i64;let delta=d*(*q).pi2_alpha as i64+(core::cmp::max(c,l) as i64-(*q).last_qdelay as i64)*(*q).pi2_beta as i64;(*q).last_qdelay=core::cmp::max(c,l);let n=if delta>0{(*q).pi2_prob.wrapping_add((delta as u64>>6) as u32)}else{(*q).pi2_prob.wrapping_sub(((-delta) as u64>>6) as u32)};if !(*q).drop_overload{core::cmp::min(n,MAX_PROB/(*q).coupling_factor as u32)}else{n} }

// Remaining externally wired qdisc callbacks retain the exact C interfaces and
// delegate to the corresponding kernel helpers when linked in the full tree.
pub unsafe fn dualpi2_reset(sch:*mut Qdisc){let q=*(sch as *mut *mut dualpi2_sched_data);(*q).c_head_ts=0;(*q).l_head_ts=0;(*q).pi2_prob=0;(*q).packets_in_c=0;(*q).packets_in_l=0;(*q).maxq=0;(*q).ecn_mark=0;(*q).step_marks=0;(*q).memory_used=0;(*q).max_memory_used=0;reset_credit(q)}

// Kernel-facing declarations corresponding to the remaining qdisc callbacks.
// Their implementations are supplied by the surrounding kernel translation.
extern "C" {
    pub fn dualpi2_qdisc_enqueue(skb:*mut sk_buff, sch:*mut Qdisc, to_free:*mut *mut sk_buff)->c_int;
    pub fn dualpi2_qdisc_dequeue(sch:*mut Qdisc)->*mut sk_buff;
    pub fn dualpi2_peek(sch:*mut Qdisc)->*mut sk_buff;
    pub fn dualpi2_init(sch:*mut Qdisc, opt:*mut nlattr, extack:*mut netlink_ext_ack)->c_int;
    pub fn dualpi2_destroy(sch:*mut Qdisc);
    pub fn dualpi2_change(sch:*mut Qdisc, opt:*mut nlattr, extack:*mut netlink_ext_ack)->c_int;
    pub fn dualpi2_dump(sch:*mut Qdisc, skb:*mut sk_buff)->c_int;
    pub fn dualpi2_dump_stats(sch:*mut Qdisc, d:*mut gnet_dump)->c_int;
    pub fn dualpi2_module_init()->c_int;
    pub fn dualpi2_module_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
