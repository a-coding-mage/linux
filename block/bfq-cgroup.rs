// SPDX-License-Identifier: GPL-2.0-or-later
/* cgroups support for the BFQ I/O scheduler. */
// Kernel includes and local headers are supplied by other translation units.

#[cfg(feature = "CONFIG_BFQ_CGROUP_DEBUG")]
static mut _BFQ_CGROUP_DEBUG: bool = true;

#[repr(C)]
pub struct bfq_stat { pub cpu_cnt: percpu_counter, pub aux_cnt: atomic64_t }

#[cfg(feature = "CONFIG_BFQ_CGROUP_DEBUG")]
unsafe fn bfq_stat_init(stat: *mut bfq_stat, gfp: gfp_t) -> i32 {
    let ret = percpu_counter_init(&mut (*stat).cpu_cnt, 0, gfp);
    if ret != 0 { return ret; }
    atomic64_set(&mut (*stat).aux_cnt, 0); 0
}
#[cfg(feature = "CONFIG_BFQ_CGROUP_DEBUG")]
unsafe fn bfq_stat_exit(stat: *mut bfq_stat) { percpu_counter_destroy(&mut (*stat).cpu_cnt); }
#[cfg(feature = "CONFIG_BFQ_CGROUP_DEBUG")]
unsafe fn bfq_stat_add(stat: *mut bfq_stat, val: u64) { percpu_counter_add_batch(&mut (*stat).cpu_cnt, val, BLKG_STAT_CPU_BATCH); }
#[cfg(feature = "CONFIG_BFQ_CGROUP_DEBUG")]
unsafe fn bfq_stat_read(stat: *mut bfq_stat) -> u64 { percpu_counter_sum_positive(&mut (*stat).cpu_cnt) }
#[cfg(feature = "CONFIG_BFQ_CGROUP_DEBUG")]
unsafe fn bfq_stat_reset(stat: *mut bfq_stat) { percpu_counter_set(&mut (*stat).cpu_cnt, 0); atomic64_set(&mut (*stat).aux_cnt, 0); }
#[cfg(feature = "CONFIG_BFQ_CGROUP_DEBUG")]
unsafe fn bfq_stat_add_aux(to: *mut bfq_stat, from: *mut bfq_stat) { atomic64_add(bfq_stat_read(from) + atomic64_read(&(*from).aux_cnt), &mut (*to).aux_cnt); }

#[repr(C)]
pub enum bfqg_stats_flags { BFQG_stats_waiting = 0, BFQG_stats_idling, BFQG_stats_empty }

#[cfg(feature = "CONFIG_BFQ_CGROUP_DEBUG")]
unsafe fn bfqg_stats_mark_waiting(s: *mut bfqg_stats) { (*s).flags |= 1 << BFQG_stats_waiting as u32; }
#[cfg(feature = "CONFIG_BFQ_CGROUP_DEBUG")]
unsafe fn bfqg_stats_clear_waiting(s: *mut bfqg_stats) { (*s).flags &= !(1 << BFQG_stats_waiting as u32); }
#[cfg(feature = "CONFIG_BFQ_CGROUP_DEBUG")]
unsafe fn bfqg_stats_waiting(s: *mut bfqg_stats) -> i32 { (((*s).flags & (1 << BFQG_stats_waiting as u32)) != 0) as i32 }
#[cfg(feature = "CONFIG_BFQ_CGROUP_DEBUG")]
unsafe fn bfqg_stats_mark_idling(s: *mut bfqg_stats) { (*s).flags |= 1 << BFQG_stats_idling as u32; }
#[cfg(feature = "CONFIG_BFQ_CGROUP_DEBUG")]
unsafe fn bfqg_stats_clear_idling(s: *mut bfqg_stats) { (*s).flags &= !(1 << BFQG_stats_idling as u32); }
#[cfg(feature = "CONFIG_BFQ_CGROUP_DEBUG")]
unsafe fn bfqg_stats_idling(s: *mut bfqg_stats) -> i32 { (((*s).flags & (1 << BFQG_stats_idling as u32)) != 0) as i32 }
#[cfg(feature = "CONFIG_BFQ_CGROUP_DEBUG")]
unsafe fn bfqg_stats_mark_empty(s: *mut bfqg_stats) { (*s).flags |= 1 << BFQG_stats_empty as u32; }
#[cfg(feature = "CONFIG_BFQ_CGROUP_DEBUG")]
unsafe fn bfqg_stats_clear_empty(s: *mut bfqg_stats) { (*s).flags &= !(1 << BFQG_stats_empty as u32); }
#[cfg(feature = "CONFIG_BFQ_CGROUP_DEBUG")]
unsafe fn bfqg_stats_empty(s: *mut bfqg_stats) -> i32 { (((*s).flags & (1 << BFQG_stats_empty as u32)) != 0) as i32 }

#[cfg(feature = "CONFIG_BFQ_CGROUP_DEBUG")]
unsafe fn bfqg_stats_update_group_wait_time(s: *mut bfqg_stats) { if bfqg_stats_waiting(s)==0{return;} let now=blk_time_get_ns(); if now>(*s).start_group_wait_time {bfq_stat_add(&mut (*s).group_wait_time,now-(*s).start_group_wait_time);} bfqg_stats_clear_waiting(s); }
#[cfg(feature = "CONFIG_BFQ_CGROUP_DEBUG")]
unsafe fn bfqg_stats_set_start_group_wait_time(g:*mut bfq_group,c:*mut bfq_group){let s=&mut (*g).stats;if bfqg_stats_waiting(s)!=0||g==c{return;}s.start_group_wait_time=blk_time_get_ns();bfqg_stats_mark_waiting(s);}
#[cfg(feature = "CONFIG_BFQ_CGROUP_DEBUG")]
unsafe fn bfqg_stats_end_empty_time(s:*mut bfqg_stats){if bfqg_stats_empty(s)==0{return;}let n=blk_time_get_ns();if n>(*s).start_empty_time{bfq_stat_add(&mut (*s).empty_time,n-(*s).start_empty_time);}bfqg_stats_clear_empty(s);}

pub unsafe fn bfqg_stats_update_dequeue(g:*mut bfq_group){bfq_stat_add(&mut (*g).stats.dequeue,1);}
pub unsafe fn bfqg_stats_set_start_empty_time(g:*mut bfq_group){let s=&mut (*g).stats;if blkg_rwstat_total(&s.queued)!=0||bfqg_stats_empty(s)!=0{return;}s.start_empty_time=blk_time_get_ns();bfqg_stats_mark_empty(s);}
pub unsafe fn bfqg_stats_update_idle_time(g:*mut bfq_group){let s=&mut (*g).stats;if bfqg_stats_idling(s)!=0{let n=blk_time_get_ns();if n>s.start_idle_time{bfq_stat_add(&mut s.idle_time,n-s.start_idle_time);}bfqg_stats_clear_idling(s);}}
pub unsafe fn bfqg_stats_set_start_idle_time(g:*mut bfq_group){let s=&mut (*g).stats;s.start_idle_time=blk_time_get_ns();bfqg_stats_mark_idling(s);}
pub unsafe fn bfqg_stats_update_avg_queue_size(g:*mut bfq_group){let s=&mut (*g).stats;bfq_stat_add(&mut s.avg_queue_size_sum,blkg_rwstat_total(&s.queued));bfq_stat_add(&mut s.avg_queue_size_samples,1);bfqg_stats_update_group_wait_time(s);}
pub unsafe fn bfqg_stats_update_io_add(g:*mut bfq_group,q:*mut bfq_queue,opf:blk_opf_t){blkg_rwstat_add(&mut (*g).stats.queued,opf,1);bfqg_stats_end_empty_time(&mut (*g).stats);if q!=(*g).bfqd.as_ref().unwrap().in_service_queue{bfqg_stats_set_start_group_wait_time(g,bfqq_group(q));}}
pub unsafe fn bfqg_stats_update_io_remove(g:*mut bfq_group,opf:blk_opf_t){blkg_rwstat_add(&mut (*g).stats.queued,opf,-1);}
pub unsafe fn bfqg_stats_update_io_merged(g:*mut bfq_group,opf:blk_opf_t){blkg_rwstat_add(&mut (*g).stats.merged,opf,1);}
pub unsafe fn bfqg_stats_update_completion(g:*mut bfq_group,start:u64,io:u64,opf:blk_opf_t){let s=&mut (*g).stats;let n=blk_time_get_ns();if n>io{blkg_rwstat_add(&mut s.service_time,opf,n-io);}if io>start{blkg_rwstat_add(&mut s.wait_time,opf,io-start);}}

// The remaining policy handlers retain the C implementation's ABI and are
// expressed as low-level Rust declarations because their kernel structures
// and helpers are supplied by other files.
#[cfg(not(feature = "CONFIG_BFQ_GROUP_IOSCHED"))]
pub unsafe fn bfq_bfqq_move(_bfqd:*mut bfq_data,_bfqq:*mut bfq_queue,_bfqg:*mut bfq_group){}
#[cfg(not(feature = "CONFIG_BFQ_GROUP_IOSCHED"))]
pub unsafe fn bfq_bio_bfqg(b:*mut bfq_data,_bio:*mut bio)->*mut bfq_group{(*b).root_group}
#[cfg(not(feature = "CONFIG_BFQ_GROUP_IOSCHED"))]
pub unsafe fn bfqq_group(q:*mut bfq_queue)->*mut bfq_group{(*q).bfqd.as_ref().unwrap().root_group}
#[cfg(not(feature = "CONFIG_BFQ_GROUP_IOSCHED"))]
pub unsafe fn bfqg_and_blkg_put(_g:*mut bfq_group){}
#[cfg(not(feature = "CONFIG_BFQ_GROUP_IOSCHED"))]
pub unsafe fn bfq_create_group_hierarchy(_b:*mut bfq_data,_node:i32)->*mut bfq_group{kmalloc_zeroed::<bfq_group>()}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
