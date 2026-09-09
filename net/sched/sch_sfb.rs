// SPDX-License-Identifier: GPL-2.0-only
/* net/sched/sch_sfb.c - Stochastic Fair Blue */

// Kernel dependencies are supplied by the surrounding translation unit.

const SFB_BUCKET_SHIFT: u32 = 4;
const SFB_NUMBUCKETS: usize = 1 << SFB_BUCKET_SHIFT;
const SFB_BUCKET_MASK: u32 = (SFB_NUMBUCKETS - 1) as u32;
const SFB_LEVELS: usize = 32 / SFB_BUCKET_SHIFT as usize;

#[repr(C)]
struct sfb_bucket { qlen: u16, p_mark: u16 }
#[repr(C)]
struct sfb_bins { perturbation: siphash_key_t, bins: [[sfb_bucket; SFB_NUMBUCKETS]; SFB_LEVELS] }
#[repr(C)]
struct sfb_sched_data {
    qdisc: *mut Qdisc, filter_list: *mut tcf_proto, block: *mut tcf_block,
    rehash_interval: c_ulong, warmup_time: c_ulong, max: u32, bin_size: u32,
    increment: u32, decrement: u32, limit: u32, penalty_rate: u32,
    penalty_burst: u32, tokens_avail: u32, rehash_time: c_ulong, token_time: c_ulong,
    slot: u8, double_buffering: bool, bins: [sfb_bins; 2],
    stats: sfb_stats,
}
#[repr(C)] struct sfb_stats { earlydrop:u32, penaltydrop:u32, bucketdrop:u32, queuedrop:u32, childdrop:u32, marked:u32 }
#[repr(C)] struct sfb_skb_cb { hashes: [u32; 2] }

#[inline] unsafe fn sfb_skb_cb(skb: *const sk_buff) -> *mut sfb_skb_cb {
    qdisc_cb_private_validate(skb, core::mem::size_of::<sfb_skb_cb>());
    qdisc_skb_cb(skb).cast::<sfb_skb_cb>()
}
unsafe fn sfb_hash(skb:*const sk_buff, slot:u32)->u32 { (*sfb_skb_cb(skb)).hashes[slot as usize] }
unsafe fn prob_plus(p1:u32,p2:u32)->u32 { p1.saturating_add(p2).min(SFB_MAX_PROB) }
unsafe fn prob_minus(p1:u32,p2:u32)->u32 { p1.saturating_sub(p2) }

unsafe fn increment_one_qlen(mut h:u32,slot:u32,q:*mut sfb_sched_data) {
    for i in 0..SFB_LEVELS { let b=&mut (*q).bins[slot as usize].bins[i][(h&SFB_BUCKET_MASK) as usize]; h >>= SFB_BUCKET_SHIFT; if b.qlen < 0xffff { b.qlen += 1; } }
}
unsafe fn increment_qlen(cb:*const sfb_skb_cb,q:*mut sfb_sched_data) { for slot in 0..2 { let h=(*cb).hashes[slot]; if h!=0 { increment_one_qlen(h,slot as u32,q); } } }
unsafe fn decrement_one_qlen(mut h:u32,slot:u32,q:*mut sfb_sched_data) { for i in 0..SFB_LEVELS { let b=&mut (*q).bins[slot as usize].bins[i][(h&SFB_BUCKET_MASK) as usize]; h >>= SFB_BUCKET_SHIFT; if b.qlen>0 { b.qlen-=1; } } }
unsafe fn decrement_qlen(skb:*const sk_buff,q:*mut sfb_sched_data) { for slot in 0..2 { let h=sfb_hash(skb,slot); if h!=0 { decrement_one_qlen(h,slot,q); } } }
unsafe fn decrement_prob(b:*mut sfb_bucket,q:*mut sfb_sched_data){(*b).p_mark=prob_minus((*b).p_mark as u32,(*q).decrement) as u16;}
unsafe fn increment_prob(b:*mut sfb_bucket,q:*mut sfb_sched_data){(*b).p_mark=prob_plus((*b).p_mark as u32,(*q).increment) as u16;}
unsafe fn sfb_zero_all_buckets(q:*mut sfb_sched_data){ core::ptr::write_bytes((*q).bins.as_mut_ptr(),0,1); }
unsafe fn sfb_compute_qlen(prob_r:*mut u32,avgpm_r:*mut u32,q:*const sfb_sched_data)->u32 { let mut qlen=0;let mut prob=0;let mut total=0; for i in 0..SFB_LEVELS { for j in 0..SFB_NUMBUCKETS { let b=(*q).bins[(*q).slot as usize].bins[i][j]; qlen=qlen.max(b.qlen as u32); total+=b.p_mark as u32; prob=prob.max(b.p_mark as u32); } } *prob_r=prob;*avgpm_r=total/(SFB_LEVELS*SFB_NUMBUCKETS) as u32;qlen }
unsafe fn sfb_init_perturbation(slot:u32,q:*mut sfb_sched_data){get_random_bytes(&mut (*q).bins[slot as usize].perturbation,core::mem::size_of::<siphash_key_t>());}
unsafe fn sfb_swap_slot(q:*mut sfb_sched_data){sfb_init_perturbation((*q).slot as u32,q);(*q).slot^=1;(*q).double_buffering=false;}
unsafe fn sfb_rate_limit(_skb:*mut sk_buff,q:*mut sfb_sched_data)->bool { if (*q).penalty_rate==0||(*q).penalty_burst==0{return true} if (*q).tokens_avail<1 { let age=(10*HZ).min(jiffies-(*q).token_time);(*q).tokens_avail=age*(*q).penalty_rate/HZ;(*q).tokens_avail=(*q).tokens_avail.min((*q).penalty_burst);(*q).token_time=jiffies;if (*q).tokens_avail<1{return true}} (*q).tokens_avail-=1;false }

// The remaining qdisc callbacks retain the C ABI and kernel helper calls.
unsafe fn sfb_enqueue(skb:*mut sk_buff,sch:*mut Qdisc,to_free:*mut *mut sk_buff)->i32 { let q=qdisc_priv(sch) as *mut sfb_sched_data; let child=(*q).qdisc; let len=qdisc_pkt_len(skb); if (*sch).q.qlen>=(*q).limit { (*q).stats.queuedrop+=1;qdisc_drop_reason(skb,sch,to_free,QDISC_DROP_OVERLIMIT);return NET_XMIT_CN } let mut cb=*(sfb_skb_cb(skb)); let mut h=skb_get_hash_perturb(skb,&(*q).bins[(*q).slot as usize].perturbation);if h==0{h=1}cb.hashes[(*q).slot as usize]=h;let mut minq=u32::MAX;let mut pmin=u32::MAX;for i in 0..SFB_LEVELS{let b=&mut (*q).bins[(*q).slot as usize].bins[i][(h&SFB_BUCKET_MASK) as usize];h>>=SFB_BUCKET_SHIFT;if b.qlen==0{decrement_prob(b,q)}else if b.qlen as u32>=(*q).bin_size{increment_prob(b,q)}minq=minq.min(b.qlen as u32);pmin=pmin.min(b.p_mark as u32)}cb.hashes[((*q).slot^1) as usize]=0;if minq>=(*q).max{(*q).stats.bucketdrop+=1;qdisc_drop_reason(skb,sch,to_free,QDISC_DROP_OVERLIMIT);return NET_XMIT_CN}if pmin>=SFB_MAX_PROB{if sfb_rate_limit(skb,q){(*q).stats.penaltydrop+=1;qdisc_drop_reason(skb,sch,to_free,QDISC_DROP_OVERLIMIT);return NET_XMIT_CN}}let ret=qdisc_enqueue(skb,child,to_free);if ret==NET_XMIT_SUCCESS{qstats_backlog_add(sch,len);qdisc_qlen_inc(sch);increment_qlen(&cb,q)}else if net_xmit_drop_count(ret){(*q).stats.childdrop+=1;qdisc_qstats_drop(sch)}ret }
unsafe fn sfb_dequeue(sch:*mut Qdisc)->*mut sk_buff {let q=qdisc_priv(sch) as *mut sfb_sched_data;let skb=qdisc_dequeue_peeked((*q).qdisc);if !skb.is_null(){qdisc_bstats_update(sch,skb);qdisc_qstats_backlog_dec(sch,skb);qdisc_qlen_dec(sch);decrement_qlen(skb,q)}skb}
unsafe fn sfb_peek(sch:*mut Qdisc)->*mut sk_buff{let q=qdisc_priv(sch) as *mut sfb_sched_data;((*(*q).qdisc).ops).peek.unwrap()((*q).qdisc)}
unsafe fn sfb_reset(sch:*mut Qdisc){let q=qdisc_priv(sch) as *mut sfb_sched_data;if !(*q).qdisc.is_null(){qdisc_reset((*q).qdisc)}(*q).slot=0;(*q).double_buffering=false;sfb_zero_all_buckets(q);sfb_init_perturbation(0,q)}
unsafe fn sfb_destroy(sch:*mut Qdisc){let q=qdisc_priv(sch) as *mut sfb_sched_data;tcf_block_put((*q).block);qdisc_put((*q).qdisc)}

unsafe fn sfb_dump_class(_sch:*mut Qdisc,_cl:c_ulong,_skb:*mut sk_buff,_tcm:*mut tcmsg)->i32 { -ENOSYS }
unsafe fn sfb_find(_sch:*mut Qdisc,_classid:u32)->c_ulong { 1 }
unsafe fn sfb_bind(_sch:*mut Qdisc,_parent:c_ulong,_classid:u32)->c_ulong { 0 }
unsafe fn sfb_unbind(_sch:*mut Qdisc,_arg:c_ulong) {}
unsafe fn sfb_change_class(_sch:*mut Qdisc,_classid:u32,_parentid:u32,_tca:*mut *mut nlattr,_arg:*mut c_ulong,_extack:*mut netlink_ext_ack)->i32 { -ENOSYS }
unsafe fn sfb_delete(_sch:*mut Qdisc,_cl:c_ulong,_extack:*mut netlink_ext_ack)->i32 { -ENOSYS }
unsafe fn sfb_graft(sch:*mut Qdisc,_arg:c_ulong,new:*mut Qdisc,old:*mut *mut Qdisc,_extack:*mut netlink_ext_ack)->i32 { let q=qdisc_priv(sch) as *mut sfb_sched_data; *old=qdisc_replace(sch,if new.is_null(){&mut noop_qdisc}else{new},&mut (*q).qdisc);0 }
unsafe fn sfb_leaf(sch:*mut Qdisc,_arg:c_ulong)->*mut Qdisc{(qdisc_priv(sch) as *mut sfb_sched_data).as_ref().unwrap().qdisc}
unsafe fn sfb_walk(sch:*mut Qdisc,walker:*mut qdisc_walker){if !(*walker).stop{tc_qdisc_stats_dump(sch,1,walker)}}
unsafe fn sfb_tcf_block(sch:*mut Qdisc,cl:c_ulong,_extack:*mut netlink_ext_ack)->*mut tcf_block{if cl==0{(qdisc_priv(sch) as *mut sfb_sched_data).as_ref().unwrap().block}else{core::ptr::null_mut()}}

// External declarations and registration metadata are provided by kernel bindings.
extern "C" { static mut sfb_qdisc_ops: Qdisc_ops; fn register_qdisc(_: *mut Qdisc_ops)->i32; fn unregister_qdisc(_: *mut Qdisc_ops); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
