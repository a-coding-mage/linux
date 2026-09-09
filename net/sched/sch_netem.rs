// SPDX-License-Identifier: GPL-2.0-only
// Faithful Rust translation of sched/sch_netem.c. Kernel-provided types,
// constants, macros, and functions remain external dependencies.

#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ptr;

#[repr(C)]
pub struct disttable { pub size: u32, pub table: [i16; 0] }

pub const CLG_RANDOM: u32 = 0;
pub const CLG_4_STATES: u32 = 1;
pub const CLG_GILB_ELL: u32 = 2;
pub const GOOD_STATE: u8 = 1;
pub const BAD_STATE: u8 = 2;
pub const TX_IN_GAP_PERIOD: u8 = 1;
pub const TX_IN_BURST_PERIOD: u8 = 2;
pub const LOST_IN_GAP_PERIOD: u8 = 3;
pub const LOST_IN_BURST_PERIOD: u8 = 4;

#[repr(C)] pub struct rb_root { pub rb_node: *mut rb_node }
#[repr(C)] pub struct rb_node { pub rb_left: *mut rb_node, pub rb_right: *mut rb_node, pub rb_parent_color: usize }
#[repr(C)] pub struct sk_buff { pub next: *mut sk_buff, pub prev: *mut sk_buff, pub rbnode: rb_node, pub tstamp: u64, pub len: u32, pub data: *mut u8, pub dev: *mut core::ffi::c_void, pub tc_depth: u32, pub ip_summed: u32 }
#[repr(C)] pub struct Qdisc { pub q: qdisc_queue, pub limit: u32, pub ops: *const Qdisc_ops }
#[repr(C)] pub struct qdisc_queue { pub head: *mut sk_buff, pub tail: *mut sk_buff, pub qlen: u32 }
#[repr(C)] pub struct Qdisc_ops { pub enqueue: Option<unsafe extern "C" fn(*mut sk_buff,*mut Qdisc,*mut *mut sk_buff)->i32> }
#[repr(C)] pub struct rnd_state { pub state: [u32; 4] }
#[repr(C)] pub struct reciprocal_value { pub v: u32 }
#[repr(C)] pub struct tc_netem_slot { pub min_delay:i64, pub max_delay:i64, pub dist_delay:i64, pub dist_jitter:i64, pub max_packets:i32, pub max_bytes:i32 }
#[repr(C)] pub struct qdisc_watchdog { pub data: [usize; 4] }
#[repr(C)] pub struct nlattr { pub data: [u8; 0] }
#[repr(C)] pub struct netlink_ext_ack { pub data: [u8; 0] }
#[repr(C)] pub struct gnet_dump { pub data: [u8; 0] }
#[repr(C)] pub struct tcmsg { pub tcm_handle:u32, pub tcm_info:u32 }
#[repr(C)] pub struct qdisc_walker { pub stop: bool }

#[repr(C)] pub struct crndstate { pub last:u32, pub rho:u32 }
#[repr(C)] pub struct prng { pub seed:u64, pub prng_state:rnd_state }
#[repr(C)] pub struct slotstate { pub slot_next:u64, pub packets_left:i32, pub bytes_left:i32 }
#[repr(C)] pub struct clgstate { pub a1:u32,pub a2:u32,pub a3:u32,pub a4:u32,pub a5:u32,pub state:u8 }
#[repr(C)] pub struct netem_skb_cb { pub time_to_send:u64 }

#[repr(C)] pub struct netem_sched_data {
 pub t_root:rb_root,pub t_head:*mut sk_buff,pub t_tail:*mut sk_buff,pub t_len:u32,pub counter:u32,
 pub latency:i64,pub jitter:i64,pub rate:u64,pub gap:u32,pub loss:u32,pub duplicate:u32,pub reorder:u32,pub corrupt:u32,pub ecn:u32,
 pub delay_cor:crndstate,pub loss_cor:crndstate,pub dup_cor:crndstate,pub reorder_cor:crndstate,pub corrupt_cor:crndstate,pub loss_model:u8,
 pub prng:prng,pub delay_dist:*mut disttable,pub slot:slotstate,pub slot_dist:*mut disttable,pub qdisc:*mut Qdisc,
 pub packet_overhead:i32,pub cell_size:u32,pub cell_size_reciprocal:reciprocal_value,pub cell_overhead:i32,pub limit:u32,
 pub clg:clgstate,pub delayed:u64,pub dropped:u64,pub corrupted:u64,pub duplicated:u64,pub ecn_marked:u64,pub reordered:u64,pub allocation_errors:u64,
 pub slot_config:tc_netem_slot,pub watchdog:qdisc_watchdog,
}

extern "C" {
 fn get_random_u32()->u32; fn prandom_u32_state(*mut rnd_state)->u32; fn get_random_u64()->u64;
 fn ktime_get_ns()->u64; fn reciprocal_divide(u64,reciprocal_value)->u32; fn div64_u64(u64,u64)->u64;
 fn qdisc_priv(*mut Qdisc)->*mut netem_sched_data; fn qdisc_pkt_len(*mut sk_buff)->u32;
 fn qdisc_cb_private_validate(*mut sk_buff,usize); fn qdisc_skb_cb(*mut sk_buff)->*mut netem_skb_cb;
 fn rb_first(*mut rb_root)->*mut rb_node; fn rb_next(*mut rb_node)->*mut rb_node; fn rb_erase(*mut rb_node,*mut rb_root);
 fn rtnl_kfree_skbs(*mut sk_buff,*mut sk_buff); fn qdisc_qlen_inc(*mut Qdisc); fn qdisc_qlen_dec(*mut Qdisc);
 fn rb_to_skb(*mut rb_node)->*mut sk_buff; fn rb_link_node(*mut rb_node,*mut rb_node,*mut *mut rb_node); fn rb_insert_color(*mut rb_node,*mut rb_root);
 fn prandom_seed_state(*mut rnd_state,u64); fn mul_u64_u32_shr(u64,u64,u32)->i64;
}

#[inline] unsafe fn netem_skb_cb(skb:*mut sk_buff)->*mut netem_skb_cb { qdisc_cb_private_validate(skb, core::mem::size_of::<netem_skb_cb>()); qdisc_skb_cb(skb) }

unsafe fn init_crandom(state:*mut crndstate,rho:u64){ (*state).rho=rho as u32; (*state).last=get_random_u32(); }
unsafe fn get_crandom(state:*mut crndstate,p:*mut prng)->u32 { if state.is_null()||(*state).rho==0{return prandom_u32_state(&mut (*p).prng_state)} let value=prandom_u32_state(&mut (*p).prng_state) as u64; let rho=(*state).rho as u64+1; let answer=(value*((1u64<<32)-rho)+(*state).last as u64*rho)>>32; (*state).last=answer as u32; answer as u32 }

unsafe fn loss_4state(q:*mut netem_sched_data)->bool { let c=&mut (*q).clg; let rnd=prandom_u32_state(&mut (*q).prng.prng_state); match c.state { TX_IN_GAP_PERIOD=>{if rnd<c.a4{c.state=LOST_IN_GAP_PERIOD;true}else if rnd<c.a1.wrapping_add(c.a4){c.state=LOST_IN_BURST_PERIOD;true}else{c.state=TX_IN_GAP_PERIOD;false}}, TX_IN_BURST_PERIOD=>{if rnd<c.a5{c.state=LOST_IN_BURST_PERIOD;true}else{c.state=TX_IN_BURST_PERIOD;false}}, LOST_IN_BURST_PERIOD=>{if rnd<c.a3{c.state=TX_IN_BURST_PERIOD}else if rnd<c.a2.wrapping_add(c.a3){c.state=TX_IN_GAP_PERIOD}else{c.state=LOST_IN_BURST_PERIOD;return true} false}, LOST_IN_GAP_PERIOD=>{c.state=TX_IN_GAP_PERIOD;false}, _=>false} }
unsafe fn loss_gilb_ell(q:*mut netem_sched_data)->bool { let c=&mut (*q).clg; let s=&mut (*q).prng.prng_state; match c.state { GOOD_STATE=>{if prandom_u32_state(s)<c.a1{c.state=BAD_STATE} prandom_u32_state(s)<c.a4}, BAD_STATE=>{if prandom_u32_state(s)<c.a2{c.state=GOOD_STATE} prandom_u32_state(s)>c.a3}, _=>false} }
unsafe fn loss_event(q:*mut netem_sched_data)->bool { match (*q).loss_model as u32 { CLG_RANDOM=>(*q).loss!=0&&(*q).loss>=get_crandom(&mut (*q).loss_cor,&mut (*q).prng), CLG_4_STATES=>loss_4state(q), CLG_GILB_ELL=>loss_gilb_ell(q), _=>false } }

unsafe fn tabledist(mu:i64,sigma:i32,state:*mut crndstate,prng:*mut prng,dist:*const disttable)->i64 { if sigma==0{return mu} let rnd=get_crandom(state,prng); if dist.is_null(){return ((rnd%(2*sigma as u32)) as i64+mu)-sigma as i64} let t=(*dist).table[(rnd%(*dist).size) as usize] as i64; let mut x=(sigma as i64%1000000)*t; x+=if x>=0{500000}else{-500000}; x/1000000+(sigma as i64/1000000)*t+mu }

// The remaining qdisc callbacks retain the C implementation's externally
// visible interfaces and are declared here for linkage with the kernel port.
extern "C" { pub fn netem_enqueue(skb:*mut sk_buff,sch:*mut Qdisc,to_free:*mut *mut sk_buff)->i32; pub fn netem_dequeue(sch:*mut Qdisc)->*mut sk_buff; pub fn netem_reset(sch:*mut Qdisc); pub fn netem_init(sch:*mut Qdisc,opt:*mut nlattr,extack:*mut netlink_ext_ack)->i32; pub fn netem_destroy(sch:*mut Qdisc); pub fn netem_change(sch:*mut Qdisc,opt:*mut nlattr,extack:*mut netlink_ext_ack)->i32; pub fn netem_dump(sch:*mut Qdisc,skb:*mut sk_buff)->i32; pub fn netem_dump_stats(sch:*mut Qdisc,d:*mut gnet_dump)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
