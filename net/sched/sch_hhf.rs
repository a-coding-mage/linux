// SPDX-License-Identifier: GPL-2.0-only
/* net/sched/sch_hhf.c - Heavy-Hitter Filter (HHF) */

// Linux kernel dependencies are supplied by the surrounding translation.

const HH_FLOWS_CNT: usize = 1024;
const HHF_ARRAYS_CNT: usize = 4;
const HHF_ARRAYS_LEN: usize = 1024;
const HHF_BIT_MASK_LEN: u32 = 10;
const HHF_BIT_MASK: u32 = 0x3ff;
const WDRR_BUCKET_CNT: usize = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum wdrr_bucket_idx { WDRR_BUCKET_FOR_HH = 0, WDRR_BUCKET_FOR_NON_HH = 1 }

#[repr(C)]
pub struct hh_flow_state {
    pub hash_id: u32,
    pub hit_timestamp: u32,
    pub flowchain: list_head,
}

#[repr(C)]
pub struct wdrr_bucket {
    pub head: *mut sk_buff,
    pub tail: *mut sk_buff,
    pub bucketchain: list_head,
    pub deficit: i32,
}

#[repr(C)]
pub struct hhf_sched_data {
    pub buckets: [wdrr_bucket; WDRR_BUCKET_CNT],
    pub perturbation: siphash_key_t,
    pub quantum: u32,
    pub drop_overlimit: u32,
    pub hh_flows: *mut list_head,
    pub hh_flows_limit: u32,
    pub hh_flows_overlimit: u32,
    pub hh_flows_total_cnt: u32,
    pub hh_flows_current_cnt: u32,
    pub hhf_arrays: [*mut u32; HHF_ARRAYS_CNT],
    pub hhf_arrays_reset_timestamp: u32,
    pub hhf_valid_bits: [*mut libc::c_ulong; HHF_ARRAYS_CNT],
    pub new_buckets: list_head,
    pub old_buckets: list_head,
    pub hhf_reset_timeout: u32,
    pub hhf_admit_bytes: u32,
    pub hhf_evict_timeout: u32,
    pub hhf_non_hh_weight: u32,
}

extern "C" {
    type list_head; type sk_buff; type Qdisc; type nlattr; type netlink_ext_ack;
    type gnet_dump; type siphash_key_t; type nla_policy; type Qdisc_ops;
    fn jiffies() -> u32;
}

#[inline] unsafe fn hhf_time_stamp() -> u32 { jiffies() }
#[inline] fn hhf_time_before(a: u32, b: u32) -> bool { (a.wrapping_sub(b) as i32) < 0 }

unsafe fn seek_list(hash: u32, _head: *mut list_head, _q: *mut hhf_sched_data) -> *mut hh_flow_state { todo!("kernel list traversal") }
unsafe fn alloc_new_hh(_head: *mut list_head, _q: *mut hhf_sched_data) -> *mut hh_flow_state { todo!("kernel allocation") }

unsafe fn hhf_classify(_skb: *mut sk_buff, _sch: *mut Qdisc) -> wdrr_bucket_idx {
    todo!("kernel qdisc and bitmap dependencies")
}

unsafe fn dequeue_head(bucket: *mut wdrr_bucket) -> *mut sk_buff {
    let skb = (*bucket).head;
    (*bucket).head = (*skb).next;
    skb_mark_not_on_list(skb);
    skb
}

unsafe fn bucket_add(bucket: *mut wdrr_bucket, skb: *mut sk_buff) {
    if (*bucket).head.is_null() { (*bucket).head = skb; }
    else { (*(*bucket).tail).next = skb; }
    (*bucket).tail = skb;
    (*skb).next = core::ptr::null_mut();
}

unsafe fn hhf_drop(sch: *mut Qdisc, to_free: *mut *mut sk_buff) -> u32 {
    let q = qdisc_priv(sch);
    let mut bucket = &mut (*q).buckets[wdrr_bucket_idx::WDRR_BUCKET_FOR_HH as usize] as *mut wdrr_bucket;
    if (*bucket).head.is_null() { bucket = &mut (*q).buckets[1] as *mut wdrr_bucket; }
    if !(*bucket).head.is_null() {
        let skb = dequeue_head(bucket); qdisc_qlen_dec(sch); qdisc_qstats_backlog_dec(sch, skb);
        qdisc_drop(skb, sch, to_free);
    }
    bucket.offset_from((*q).buckets.as_mut_ptr()) as u32
}

unsafe fn hhf_enqueue(skb: *mut sk_buff, sch: *mut Qdisc, to_free: *mut *mut sk_buff) -> i32 {
    let q = qdisc_priv(sch); let idx = hhf_classify(skb, sch) as usize;
    let bucket = &mut (*q).buckets[idx] as *mut wdrr_bucket;
    bucket_add(bucket, skb); qdisc_qstats_backlog_inc(sch, skb);
    if list_empty(&mut (*bucket).bucketchain) {
        let weight = if idx == 0 { 1 } else { (*q).hhf_non_hh_weight };
        if idx == 0 { list_add_tail(&mut (*bucket).bucketchain, &mut (*q).old_buckets); }
        else { list_add_tail(&mut (*bucket).bucketchain, &mut (*q).new_buckets); }
        (*bucket).deficit = (weight * (*q).quantum) as i32;
    }
    qdisc_qlen_inc(sch);
    if (*sch).q.qlen <= (*sch).limit { return NET_XMIT_SUCCESS; }
    let prev_backlog = (*sch).qstats.backlog; (*q).drop_overlimit = (*q).drop_overlimit.wrapping_add(1);
    if hhf_drop(sch, to_free) == idx as u32 { return NET_XMIT_CN; }
    qdisc_tree_reduce_backlog(sch, 1, prev_backlog - (*sch).qstats.backlog); NET_XMIT_SUCCESS
}

unsafe fn hhf_dequeue(_sch: *mut Qdisc) -> *mut sk_buff { todo!("kernel qdisc dependencies") }
unsafe fn hhf_reset_classifier(_q: *mut hhf_sched_data) { todo!("kernel list and allocation dependencies") }
unsafe fn hhf_reset(sch: *mut Qdisc) { while { let skb = hhf_dequeue(sch); !skb.is_null() } { /* rtnl_kfree_skbs */ } hhf_reset_classifier(qdisc_priv(sch)); }
unsafe fn hhf_destroy(_sch: *mut Qdisc) { todo!("kernel allocation dependencies") }
unsafe fn hhf_change(_sch: *mut Qdisc, _opt: *mut nlattr, _extack: *mut netlink_ext_ack) -> i32 { todo!("netlink dependencies") }
unsafe fn hhf_init(_sch: *mut Qdisc, _opt: *mut nlattr, _extack: *mut netlink_ext_ack) -> i32 { todo!("kernel allocation dependencies") }
unsafe fn hhf_dump(_sch: *mut Qdisc, _skb: *mut sk_buff) -> i32 { todo!("netlink dependencies") }
unsafe fn hhf_dump_stats(_sch: *mut Qdisc, _d: *mut gnet_dump) -> i32 { todo!("statistics dependencies") }

// External kernel declarations used by the literal translation.
extern "C" {
    fn qdisc_priv(sch: *mut Qdisc) -> *mut hhf_sched_data;
    fn skb_mark_not_on_list(skb: *mut sk_buff);
    fn list_empty(h: *mut list_head) -> bool;
    fn list_add_tail(n: *mut list_head, h: *mut list_head);
    fn qdisc_qlen_dec(sch: *mut Qdisc); fn qdisc_qlen_inc(sch: *mut Qdisc);
    fn qdisc_qstats_backlog_dec(sch: *mut Qdisc, skb: *mut sk_buff);
    fn qdisc_qstats_backlog_inc(sch: *mut Qdisc, skb: *mut sk_buff);
    fn qdisc_drop(skb: *mut sk_buff, sch: *mut Qdisc, to_free: *mut *mut sk_buff);
    fn qdisc_tree_reduce_backlog(sch: *mut Qdisc, pkts: u32, bytes: u32);
    static NET_XMIT_SUCCESS: i32; static NET_XMIT_CN: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
