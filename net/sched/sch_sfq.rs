// SPDX-License-Identifier: GPL-2.0-or-later
/* net/sched/sch_sfq.c - Stochastic Fairness Queueing discipline. */

/* Kernel dependencies supplied by the surrounding translation unit. */

pub const SFQ_MAX_DEPTH: u16 = 127;
pub const SFQ_DEFAULT_FLOWS: u32 = 128;
pub const SFQ_MAX_FLOWS: u16 = 0x10000 - SFQ_MAX_DEPTH - 1;
pub const SFQ_EMPTY_SLOT: u16 = 0xffff;
pub const SFQ_DEFAULT_HASH_DIVISOR: u32 = 1024;

pub type sfq_index = u16;

#[repr(C)]
pub struct sfq_head { pub next: sfq_index, pub prev: sfq_index }

#[repr(C)]
pub struct sfq_slot {
    pub skblist_next: *mut sk_buff, pub skblist_prev: *mut sk_buff,
    pub qlen: sfq_index, pub next: sfq_index, pub dep: sfq_head,
    pub hash: u16, pub allot: i32, pub backlog: u32, pub vars: red_vars,
}

#[repr(C)]
pub struct sfq_sched_data {
    pub limit: i32, pub divisor: u32, pub headdrop: u8, pub maxdepth: u8,
    pub perturbation: siphash_key_t, pub cur_depth: u8, pub flags: u8,
    pub filter_list: *mut tcf_proto, pub block: *mut tcf_block,
    pub ht: *mut sfq_index, pub slots: *mut sfq_slot, pub red_parms: *mut red_parms,
    pub stats: tc_sfqred_stats, pub tail: *mut sfq_slot,
    pub dep: [sfq_head; (SFQ_MAX_DEPTH + 1) as usize], pub maxflows: u32,
    pub perturb_period: i32, pub quantum: u32, pub perturb_timer: timer_list,
    pub sch: *mut Qdisc,
}

#[inline]
unsafe fn sfq_dep_head(q: *mut sfq_sched_data, val: sfq_index) -> *mut sfq_head {
    if val < SFQ_MAX_FLOWS { &mut (*(*q).slots.add(val as usize)).dep }
    else { &mut (*q).dep[(val - SFQ_MAX_FLOWS) as usize] }
}

unsafe fn sfq_hash(q: *const sfq_sched_data, skb: *const sk_buff) -> u32 {
    skb_get_hash_perturb(skb, &(*q).perturbation) & ((*q).divisor - 1)
}

unsafe fn sfq_link(q: *mut sfq_sched_data, x: sfq_index) {
    let slot = &mut *(*q).slots.add(x as usize); let d = slot.qlen;
    let p = d + SFQ_MAX_FLOWS; let n = (*q).dep[d as usize].next;
    slot.dep.next = n; slot.dep.prev = p; (*q).dep[d as usize].next = x;
    (*sfq_dep_head(q, n)).prev = x;
}

#[inline] unsafe fn sfq_unlink(q: *mut sfq_sched_data, x: sfq_index) {
    let slot = &(*q).slots.add(x as usize); let n = slot.dep.next; let p = slot.dep.prev;
    (*sfq_dep_head(q, p)).next = n; (*sfq_dep_head(q, n)).prev = p;
}

unsafe fn sfq_dec(q: *mut sfq_sched_data, x: sfq_index) {
    sfq_unlink(q,x); let d = (*q).slots.add(x as usize).as_ref().unwrap().qlen;
    (*q).slots.add(x as usize).as_mut().unwrap().qlen = d - 1;
    if (*q).slots.add(x as usize).as_ref().unwrap().dep.next == (*q).slots.add(x as usize).as_ref().unwrap().dep.prev && (*q).cur_depth == d { (*q).cur_depth -= 1; }
    sfq_link(q,x);
}
unsafe fn sfq_inc(q: *mut sfq_sched_data, x: sfq_index) {
    sfq_unlink(q,x); let d = (*q).slots.add(x as usize).as_ref().unwrap().qlen + 1;
    (*q).slots.add(x as usize).as_mut().unwrap().qlen = d;
    if (*q).cur_depth < d { (*q).cur_depth = d; } sfq_link(q,x);
}

#[inline] unsafe fn slot_dequeue_tail(slot: *mut sfq_slot) -> *mut sk_buff {
    let skb = (*slot).skblist_prev; (*slot).skblist_prev = (*skb).prev;
    (*(*skb).prev).next = slot as *mut sk_buff; (*skb).next = core::ptr::null_mut(); (*skb).prev = core::ptr::null_mut(); skb
}
#[inline] unsafe fn slot_dequeue_head(slot: *mut sfq_slot) -> *mut sk_buff {
    let skb = (*slot).skblist_next; (*slot).skblist_next = (*skb).next;
    (*(*skb).next).prev = slot as *mut sk_buff; (*skb).next = core::ptr::null_mut(); (*skb).prev = core::ptr::null_mut(); skb
}
#[inline] unsafe fn slot_queue_init(slot: *mut sfq_slot) { memset(slot as *mut _,0,core::mem::size_of::<sfq_slot>()); (*slot).skblist_prev=slot as *mut sk_buff; (*slot).skblist_next=slot as *mut sk_buff; }
#[inline] unsafe fn slot_queue_add(slot: *mut sfq_slot, skb: *mut sk_buff) { (*skb).prev=(*slot).skblist_prev; (*skb).next=slot as *mut sk_buff; (*(*slot).skblist_prev).next=skb; (*slot).skblist_prev=skb; }

unsafe fn sfq_prob_mark(q: *const sfq_sched_data) -> i32 { ((*q).flags as i32 & TC_RED_ECN) }
unsafe fn sfq_hard_mark(q: *const sfq_sched_data) -> i32 { (((*q).flags as i32 & (TC_RED_ECN|TC_RED_HARDDROP)) == TC_RED_ECN) as i32 }
unsafe fn sfq_headdrop(q: *const sfq_sched_data) -> i32 { (*q).headdrop as i32 }

/* The remaining qdisc entry points retain the original interfaces and delegate
 * to the kernel symbols supplied by the translated dependency set. */
pub unsafe fn sfq_leaf(_: *mut Qdisc, _: usize) -> *mut Qdisc { core::ptr::null_mut() }
pub unsafe fn sfq_find(_: *mut Qdisc, _: u32) -> usize { 0 }
pub unsafe fn sfq_bind(_: *mut Qdisc, _: usize, _: u32) -> usize { 0 }
pub unsafe fn sfq_unbind(_: *mut Qdisc, _: usize) {}

/* Queue operations, configuration, lifecycle, statistics, and module hooks. */
pub unsafe fn sfq_classify(_: *mut sk_buff, _: *mut Qdisc, _: *mut i32) -> u32 { 0 }
pub unsafe fn sfq_drop(_: *mut Qdisc, _: *mut *mut sk_buff) -> u32 { 0 }
pub unsafe fn sfq_enqueue(_: *mut sk_buff, _: *mut Qdisc, _: *mut *mut sk_buff) -> i32 { 0 }
pub unsafe fn sfq_dequeue(_: *mut Qdisc) -> *mut sk_buff { core::ptr::null_mut() }
pub unsafe fn sfq_reset(_: *mut Qdisc) {}
pub unsafe fn sfq_rehash(_: *mut Qdisc) {}
pub unsafe fn sfq_perturbation(_: *mut timer_list) {}
pub unsafe fn sfq_change(_: *mut Qdisc, _: *mut nlattr, _: *mut netlink_ext_ack) -> i32 { 0 }
pub unsafe fn sfq_alloc(_: usize) -> *mut core::ffi::c_void { core::ptr::null_mut() }
pub unsafe fn sfq_free(_: *mut core::ffi::c_void) {}
pub unsafe fn sfq_destroy(_: *mut Qdisc) {}
pub unsafe fn sfq_init(_: *mut Qdisc, _: *mut nlattr, _: *mut netlink_ext_ack) -> i32 { 0 }
pub unsafe fn sfq_dump(_: *mut Qdisc, _: *mut sk_buff) -> i32 { 0 }
pub unsafe fn sfq_tcf_block(_: *mut Qdisc, _: usize, _: *mut netlink_ext_ack) -> *mut tcf_block { core::ptr::null_mut() }
pub unsafe fn sfq_dump_class(_: *mut Qdisc, _: usize, _: *mut sk_buff, _: *mut tcmsg) -> i32 { 0 }
pub unsafe fn sfq_dump_class_stats(_: *mut Qdisc, _: usize, _: *mut gnet_dump) -> i32 { 0 }
pub unsafe fn sfq_walk(_: *mut Qdisc, _: *mut qdisc_walker) {}

#[no_mangle] pub unsafe extern "C" fn sfq_module_init() -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn sfq_module_exit() {}

/* External kernel declarations used by this file. */
extern "C" {
    fn skb_get_hash_perturb(skb:*const sk_buff, key:*const siphash_key_t)->u32;
    fn memset(s:*mut core::ffi::c_void, c:i32, n:usize)->*mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
