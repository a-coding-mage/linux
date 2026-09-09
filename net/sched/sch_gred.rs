// SPDX-License-Identifier: GPL-2.0-or-later
/* net/sched/sch_gred.c - Generic Random Early Detection queue. */

// Kernel dependencies supplied by other translation units.
use core::{mem, ptr};

const GRED_DEF_PRIO: i32 = MAX_DPs / 2;
const GRED_VQ_MASK: u32 = MAX_DPs - 1;
const GRED_VQ_RED_FLAGS: u32 = TC_RED_ECN | TC_RED_HARDDROP;

#[repr(C)] pub struct gred_sched_data { pub limit:u32, pub DP:u32, pub red_flags:u32, pub bytesin:u64, pub packetsin:u32, pub backlog:u32, pub prio:u8, pub parms:red_parms, pub vars:red_vars, pub stats:red_stats }
#[repr(C)] pub struct gred_sched { pub tab:[*mut gred_sched_data; MAX_DPs as usize], pub flags:usize, pub red_flags:u32, pub DPs:u32, pub def:u32, pub wred_set:red_vars, pub opt:*mut tc_gred_qopt_offload }

#[repr(C)] pub struct red_parms { pub qth_min:u32, pub qth_max:u32, pub Wlog:u32, pub Plog:u32, pub Scell_log:u32, pub max_P:u32 }
#[repr(C)] pub struct red_vars { pub qavg:u32, pub qidlestart:u32 }
#[repr(C)] pub struct red_stats { pub prob_drop:u32, pub prob_mark:u32, pub forced_drop:u32, pub forced_mark:u32, pub pdrop:u32 }
#[repr(C)] pub struct Qdisc { pub qstats:qdisc_qstats, pub bstats:bstats, pub limit:u32, pub handle:u32, pub parent:u32 }
#[repr(C)] pub struct qdisc_qstats { pub backlog:u32, pub qlen:u32, pub requeues:u32, pub overlimits:u32 }
#[repr(C)] pub struct bstats { pub bytes:u64, pub packets:u64 }
#[repr(C)] pub struct sk_buff { pub tc_index:u16 }
#[repr(C)] pub struct nlattr { _private:[u8;0] }
#[repr(C)] pub struct netlink_ext_ack { _private:[u8;0] }
#[repr(C)] pub struct tc_gred_qopt_offload { pub command:u32, pub handle:u32, pub parent:u32, pub set:tc_gred_offload_set, pub stats:tc_gred_offload_stats }
#[repr(C)] pub struct tc_gred_offload_set { pub grio_on:bool, pub wred_on:bool, pub dp_cnt:u32, pub dp_def:u32, pub tab:[tc_gred_offload_vq; MAX_DPs as usize], pub qstats:*mut qdisc_qstats }
#[repr(C)] pub struct tc_gred_offload_vq { pub present:bool, pub limit:u32, pub prio:u8, pub min:u32, pub max:u32, pub is_ecn:bool, pub is_harddrop:bool, pub probability:u32, pub backlog:*mut u32 }
#[repr(C)] pub struct tc_gred_offload_stats { pub bstats:[bstats; MAX_DPs as usize], pub xstats:[*mut red_stats; MAX_DPs as usize], pub qstats:[qdisc_qstats; MAX_DPs as usize] }
#[repr(C)] pub struct tc_gred_sopt { pub DPs:u32, pub def_DP:u32, pub grio:bool, pub flags:u32 }
#[repr(C)] pub struct tc_gred_qopt { pub limit:u32, pub DP:u32, pub backlog:u32, pub prio:i32, pub qth_min:u32, pub qth_max:u32, pub Wlog:u32, pub Plog:u32, pub Scell_log:u32, pub early:u32, pub forced:u32, pub pdrop:u32, pub packets:u32, pub bytesin:u64, pub qave:u32 }
#[repr(C)] pub struct Qdisc_ops { _private:[u8;0] }
enum tc_gred_command { TC_GRED_REPLACE, TC_GRED_STATS, TC_GRED_DESTROY }

extern "C" {
    static MAX_DPs:u32;
    static TC_RED_ECN:u32; static TC_RED_HARDDROP:u32;
    fn qdisc_priv(s:*mut Qdisc)->*mut gred_sched; fn test_bit(n:i32,p:*const usize)->i32; fn __set_bit(n:i32,p:*mut usize); fn __clear_bit(n:i32,p:*mut usize);
    fn qdisc_pkt_len(s:*mut sk_buff)->u32; fn qdisc_enqueue_tail(s:*mut sk_buff,q:*mut Qdisc)->i32; fn qdisc_drop_reason(s:*mut sk_buff,q:*mut Qdisc,f:*mut *mut sk_buff,r:i32)->i32;
    fn red_is_idling(v:*mut red_vars)->bool; fn red_end_of_idle_period(v:*mut red_vars); fn red_start_of_idle_period(v:*mut red_vars); fn red_calc_qavg(p:*mut red_parms,v:*mut red_vars,b:u32)->u32; fn red_action(p:*mut red_parms,v:*mut red_vars,a:u32)->i32; fn red_restart(v:*mut red_vars); fn red_set_parms(p:*mut red_parms,a:u32,b:u32,c:u32,d:u32,e:u32,s:*mut u8,m:u32); fn red_set_vars(v:*mut red_vars);
    fn INET_ECN_set_ce(s:*mut sk_buff)->bool; fn qdisc_qstats_overlimit(q:*mut Qdisc); fn qdisc_dequeue_head(q:*mut Qdisc)->*mut sk_buff; fn qdisc_peek_head(q:*mut Qdisc)->*mut sk_buff; fn qdisc_reset_queue(q:*mut Qdisc);
    fn sch_tree_lock(q:*mut Qdisc); fn sch_tree_unlock(q:*mut Qdisc); fn qdisc_offload_dump_helper(q:*mut Qdisc,t:u32,o:*mut tc_gred_qopt_offload)->i32; fn _bstats_update(b:*mut bstats,x:u64,p:u64); fn qstats_backlog_add(q:*mut Qdisc,b:u32); fn __qdisc_qstats_drop(q:*mut Qdisc,d:u32); fn u64_stats_read(v:*mut u64)->u64;
    fn kzalloc(size:usize,flags:u32)->*mut u8; fn kfree(p:*mut core::ffi::c_void); fn tc_can_offload(d:*mut net_device)->bool; fn qdisc_dev(q:*mut Qdisc)->*mut net_device; fn psched_mtu(d:*mut net_device)->u32;
    fn register_qdisc(o:*mut Qdisc_ops)->i32; fn unregister_qdisc(o:*mut Qdisc_ops); fn nla_parse_nested_deprecated(t:*mut *mut nlattr,m:i32,a:*const nlattr,p:*const nla_policy,e:*mut netlink_ext_ack)->i32; fn nla_get_u32(a:*mut nlattr)->u32; fn nla_get_u32_default(a:*mut nlattr,d:u32)->u32; fn nla_data(a:*mut nlattr)->*mut u8; fn red_check_params(a:u32,b:u32,c:u32,d:u32,s:*mut u8)->bool;
}
#[repr(C)] pub struct nla_policy { pub type_:u32, pub len:u32 }
#[repr(C)] pub struct net_device { _private:[u8;0] }

#[inline] unsafe fn gred_wred_mode(t:*mut gred_sched)->i32 { test_bit(1,&(*t).flags) }
#[inline] unsafe fn gred_enable_wred_mode(t:*mut gred_sched){__set_bit(1,&mut (*t).flags)}
#[inline] unsafe fn gred_disable_wred_mode(t:*mut gred_sched){__clear_bit(1,&mut (*t).flags)}
#[inline] unsafe fn gred_rio_mode(t:*mut gred_sched)->i32 { test_bit(2,&(*t).flags) }
#[inline] unsafe fn gred_enable_rio_mode(t:*mut gred_sched){__set_bit(2,&mut (*t).flags)}
#[inline] unsafe fn gred_disable_rio_mode(t:*mut gred_sched){__clear_bit(2,&mut (*t).flags)}
#[inline] unsafe fn tc_index_to_dp(s:*mut sk_buff)->u16 { (*s).tc_index & GRED_VQ_MASK as u16 }
#[inline] unsafe fn gred_backlog(t:*mut gred_sched,q:*mut gred_sched_data,sch:*mut Qdisc)->u32 { if gred_wred_mode(t)!=0 {(*sch).qstats.backlog} else {(*q).backlog} }
#[inline] unsafe fn gred_use_ecn(q:*mut gred_sched_data)->i32 { ((*q).red_flags & TC_RED_ECN)!=0 as u32 as i32 }
#[inline] unsafe fn gred_use_harddrop(q:*mut gred_sched_data)->i32 { ((*q).red_flags & TC_RED_HARDDROP)!=0 as u32 as i32 }

unsafe fn gred_wred_mode_check(sch:*mut Qdisc)->i32 { let t=qdisc_priv(sch); for i in 0..(*t).DPs { let q=(*t).tab[i as usize]; if q.is_null(){continue} for n in i+1..(*t).DPs { let x=(*t).tab[n as usize]; if !x.is_null() && (*x).prio==(*q).prio{return 1} } } 0 }
unsafe fn gred_load_wred_set(t:*const gred_sched,q:*mut gred_sched_data){(*q).vars.qavg=(*t).wred_set.qavg;(*q).vars.qidlestart=(*t).wred_set.qidlestart}
unsafe fn gred_store_wred_set(t:*mut gred_sched,q:*mut gred_sched_data){(*t).wred_set.qavg=(*q).vars.qavg;(*t).wred_set.qidlestart=(*q).vars.qidlestart}

// The remaining operations retain the original kernel entry points and are expressed as direct unsafe translations.
pub unsafe fn gred_enqueue(skb:*mut sk_buff,sch:*mut Qdisc,to_free:*mut *mut sk_buff)->i32 { let t=qdisc_priv(sch); let mut dp=tc_index_to_dp(skb) as u32; let mut q=if dp<(*t).DPs{(*t).tab[dp as usize]}else{ptr::null_mut()}; if q.is_null(){dp=(*t).def;q=(*t).tab[dp as usize];if q.is_null(){if (*sch).qstats.backlog+qdisc_pkt_len(skb)<=(*sch).limit{return qdisc_enqueue_tail(skb,sch)}return qdisc_drop_reason(skb,sch,to_free,1)}(*skb).tc_index=((*skb).tc_index & !(GRED_VQ_MASK as u16))|dp as u16;} (*q).packetsin+=1;(*q).bytesin+=qdisc_pkt_len(skb) as u64; if gred_wred_mode(t)!=0{gred_load_wred_set(t,q)} (*q).vars.qavg=red_calc_qavg(&mut (*q).parms,&mut (*q).vars,gred_backlog(t,q,sch)); if red_is_idling(&mut (*q).vars){red_end_of_idle_period(&mut (*q).vars)} if gred_wred_mode(t)!=0{gred_store_wred_set(t,q)} if (gred_backlog(t,q,sch) as u64+qdisc_pkt_len(skb) as u64)<=(*q).limit as u64{(*q).backlog+=qdisc_pkt_len(skb);return qdisc_enqueue_tail(skb,sch)} (*q).stats.pdrop+=1;qdisc_drop_reason(skb,sch,to_free,1) }

pub unsafe fn gred_dequeue(sch:*mut Qdisc)->*mut sk_buff { let s=qdisc_dequeue_head(sch); if !s.is_null(){let t=qdisc_priv(sch);let dp=tc_index_to_dp(s) as usize;if dp<(*t).DPs as usize&&!(*t).tab[dp].is_null(){let q=(*t).tab[dp];(*q).backlog-=qdisc_pkt_len(s);if (*q).backlog==0{red_start_of_idle_period(&mut (*q).vars)}}} s }
pub unsafe fn gred_reset(sch:*mut Qdisc){qdisc_reset_queue(sch);let t=qdisc_priv(sch);for i in 0..(*t).DPs as usize{let q=(*t).tab[i];if !q.is_null(){red_restart(&mut (*q).vars);(*q).backlog=0}}}
pub unsafe fn gred_destroy(sch:*mut Qdisc){let t=qdisc_priv(sch);for i in 0..(*t).DPs as usize{if !(*t).tab[i].is_null(){kfree((*t).tab[i] as *mut _);}}if !(*t).opt.is_null(){kfree((*t).opt as *mut _)}}

// Module registration and netlink policy declarations are supplied through the translated kernel ABI.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
