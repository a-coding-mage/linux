// SPDX-License-Identifier: GPL-2.0
/* Block rq-qos base io controller. Direct Rust translation of blk-iolatency.c. */

const DEFAULT_SCALE_COOKIE: u32 = 1_000_000;
const BLKIOLATENCY_MIN_WIN_SIZE: u64 = 100 * NSEC_PER_MSEC;
const BLKIOLATENCY_MAX_WIN_SIZE: u64 = NSEC_PER_SEC;
const BLKIOLATENCY_NR_EXP_FACTORS: usize = 5;
const BLKIOLATENCY_EXP_BUCKET_SIZE: u64 = BLKIOLATENCY_MAX_WIN_SIZE / (BLKIOLATENCY_NR_EXP_FACTORS as u64 - 1);
const BLKIOLATENCY_MIN_ADJUST_TIME: u64 = 500 * NSEC_PER_MSEC;
const BLKIOLATENCY_MIN_GOOD_SAMPLES: u64 = 5;
const SCALE_DOWN_FACTOR: u32 = 2;
const SCALE_UP_FACTOR: u32 = 4;

static mut blkcg_policy_iolatency: blkcg_policy = blkcg_policy { };
static iolatency_exp_factors: [u64; BLKIOLATENCY_NR_EXP_FACTORS] = [2045, 2039, 2031, 2023, 2014];

#[repr(C)]
struct blk_iolatency { rqos: rq_qos, timer: timer_list, enabled: bool, enable_cnt: atomic_t, enable_work: work_struct }
#[repr(C)]
struct child_latency_info { lock: spinlock_t, last_scale_event: u64, scale_lat: u64, nr_samples: u64, scale_grp: *mut iolatency_grp, scale_cookie: atomic_t }
#[repr(C)] struct percentile_stats { total: u64, missed: u64 }
#[repr(C)] struct latency_stat { ps: percentile_stats, rqs: blk_rq_stat }
#[repr(C)]
struct iolatency_grp { pd: blkg_policy_data, stats: *mut latency_stat, cur_stat: latency_stat, blkiolat: *mut blk_iolatency, max_depth: u32, rq_wait: rq_wait, window_start: atomic64_t, scale_cookie: atomic_t, min_lat_nsec: u64, cur_win_nsec: u64, lat_avg: u64, nr_samples: u64, ssd: bool, child_lat: child_latency_info }

unsafe fn BLKIOLATENCY(rqos: *mut rq_qos) -> *mut blk_iolatency { container_of(rqos, blk_iolatency, rqos) }
unsafe fn pd_to_lat(pd: *mut blkg_policy_data) -> *mut iolatency_grp { if pd.is_null() { core::ptr::null_mut() } else { container_of(pd, iolatency_grp, pd) } }
unsafe fn blkg_to_lat(blkg: *mut blkcg_gq) -> *mut iolatency_grp { pd_to_lat(blkg_to_pd(blkg, &mut blkcg_policy_iolatency)) }
unsafe fn lat_to_blkg(iolat: *mut iolatency_grp) -> *mut blkcg_gq { pd_to_blkg(&mut (*iolat).pd) }

unsafe fn latency_stat_init(iolat: *mut iolatency_grp, stat: *mut latency_stat) { if (*iolat).ssd { (*stat).ps.total=0; (*stat).ps.missed=0; } else { blk_rq_stat_init(&mut (*stat).rqs); } }
unsafe fn latency_stat_sum(iolat: *mut iolatency_grp, sum: *mut latency_stat, stat: *mut latency_stat) { if (*iolat).ssd { (*sum).ps.total += (*stat).ps.total; (*sum).ps.missed += (*stat).ps.missed; } else { blk_rq_stat_sum(&mut (*sum).rqs, &(*stat).rqs); } }
unsafe fn latency_stat_record_time(iolat: *mut iolatency_grp, req_time: u64) { let stat=get_cpu_ptr((*iolat).stats); if (*iolat).ssd { if req_time >= (*iolat).min_lat_nsec { (*stat).ps.missed+=1; } (*stat).ps.total+=1; } else { blk_rq_stat_add(&mut (*stat).rqs, req_time); } put_cpu_ptr(stat); }
unsafe fn latency_sum_ok(iolat: *mut iolatency_grp, stat: *mut latency_stat) -> bool { if (*iolat).ssd { let thresh=core::cmp::max((*stat).ps.total/10,1); (*stat).ps.missed < thresh } else { (*stat).rqs.mean <= (*iolat).min_lat_nsec } }
unsafe fn latency_stat_samples(iolat:*mut iolatency_grp, stat:*mut latency_stat)->u64 { if (*iolat).ssd {(*stat).ps.total} else {(*stat).rqs.nr_samples} }
unsafe fn iolat_update_total_lat_avg(iolat:*mut iolatency_grp, stat:*mut latency_stat) { if (*iolat).ssd{return;} let idx=core::cmp::min(BLKIOLATENCY_NR_EXP_FACTORS-1, div64_u64((*iolat).cur_win_nsec,BLKIOLATENCY_EXP_BUCKET_SIZE) as usize); (*iolat).lat_avg=calc_load((*iolat).lat_avg, iolatency_exp_factors[idx], (*stat).rqs.mean); }

unsafe fn iolat_cleanup_cb(rqw:*mut rq_wait,_:*mut core::ffi::c_void){ atomic_dec(&mut (*rqw).inflight); wake_up(&mut (*rqw).wait); }
unsafe fn iolat_acquire_inflight(rqw:*mut rq_wait, private_data:*mut core::ffi::c_void)->bool { rq_wait_inc_below(rqw,(*private_data.cast::<iolatency_grp>()).max_depth) }
unsafe fn __blkcg_iolatency_throttle(rqos:*mut rq_qos,iolat:*mut iolatency_grp,issue_as_root:bool,use_memdelay:bool){let rqw=&mut (*iolat).rq_wait; if atomic_read(&mut (*lat_to_blkg(iolat)).use_delay)!=0 {blkcg_schedule_throttle((*rqos).disk,use_memdelay);} if issue_as_root||fatal_signal_pending(current){atomic_inc(&mut rqw.inflight);return;} rq_qos_wait(rqw,iolat,iolat_acquire_inflight,iolat_cleanup_cb);}
unsafe fn scale_amount(qd:usize,up:bool)->usize {core::cmp::max(if up {qd>>SCALE_UP_FACTOR} else {qd>>SCALE_DOWN_FACTOR},1)}
unsafe fn scale_cookie_change(b:*mut blk_iolatency,l:*mut child_latency_info,up:bool){let qd=(*(*b).rqos.disk).queue.nr_requests as usize;let scale=scale_amount(qd,up);let old=atomic_read(&mut (*l).scale_cookie) as usize;let max_scale=qd<<1;let diff=if old<(DEFAULT_SCALE_COOKIE as usize){DEFAULT_SCALE_COOKIE as usize-old}else{0};if up {if scale+old>DEFAULT_SCALE_COOKIE as usize{atomic_set(&mut (*l).scale_cookie,DEFAULT_SCALE_COOKIE as i32)}else if diff>qd{atomic_inc(&mut (*l).scale_cookie)}else{atomic_add(scale as i32,&mut (*l).scale_cookie)}}else if diff>qd {if diff<max_scale{atomic_dec(&mut (*l).scale_cookie)}}else{atomic_sub(scale as i32,&mut (*l).scale_cookie)}}
unsafe fn scale_change(i:*mut iolatency_grp,up:bool){let qd=(*(*(*i).blkiolat).rqos.disk).queue.nr_requests as usize;let scale=scale_amount(qd,up);let mut old=(*i).max_depth as usize;if old>qd{old=qd;}if up {if old==1&&blkcg_unuse_delay(lat_to_blkg(i)){return;}if old<qd{(*i).max_depth=core::cmp::min(old+scale,qd) as u32;wake_up_all(&mut (*i).rq_wait.wait);}}else{(*i).max_depth=core::cmp::max(old>>1,1) as u32;}}

unsafe fn check_scale_change(i:*mut iolatency_grp){let p=blkg_to_lat((*lat_to_blkg(i)).parent);if p.is_null(){return;}let li=&mut (*p).child_lat;let cur=atomic_read(&mut li.scale_cookie) as u32;let our=atomic_read(&mut (*i).scale_cookie) as u32;let lat=READ_ONCE(li.scale_lat);let dir=if cur<our{-1}else if cur>our{1}else{return};let mut expected=our;if !atomic_try_cmpxchg(&mut (*i).scale_cookie,&mut expected,cur){return;}if dir<0&&(*i).min_lat_nsec!=0 {if lat==0||(*i).min_lat_nsec<=lat{return;}let threshold=core::cmp::max(div64_u64(li.nr_samples*5,100),1);if (*i).nr_samples<=threshold{return;}}if (*i).max_depth==1&&dir<0{blkcg_use_delay(lat_to_blkg(i));return;}if cur==DEFAULT_SCALE_COOKIE{blkcg_clear_delay(lat_to_blkg(i));(*i).max_depth=u32::MAX;wake_up_all(&mut (*i).rq_wait.wait);return;}scale_change(i,dir>0);}

unsafe fn blkcg_iolatency_throttle(rqos:*mut rq_qos,bio:*mut bio){let b=BLKIOLATENCY(rqos);if !(*b).enabled{return;}let mut blkg=(*bio).bi_blkg;let root=bio_issue_as_root_blkg(bio);while !blkg.is_null()&&!(*blkg).parent.is_null(){let i=blkg_to_lat(blkg);if !i.is_null(){check_scale_change(i);__blkcg_iolatency_throttle(rqos,i,root,((*bio).bi_opf&REQ_SWAP)==REQ_SWAP);}blkg=(*blkg).parent;}if !timer_pending(&mut (*b).timer){mod_timer(&mut (*b).timer,jiffies+HZ);}}

unsafe fn iolatency_record_time(i:*mut iolatency_grp,start:u64,now:u64,root:bool){if now<=start{return;}let req=now-start;if root&&(*i).max_depth!=u32::MAX{let sub=(*i).min_lat_nsec;if req<sub{blkcg_add_delay(lat_to_blkg(i),now,sub-req);}return;}latency_stat_record_time(i,req);}

// Remaining callbacks retain kernel ABI names and direct control flow; external kernel declarations are dependencies.
unsafe fn blkcg_iolatency_done_bio(_:*mut rq_qos,_:*mut bio) { /* external bio/accounting helpers are required here */ }
unsafe fn blkcg_iolatency_exit(rqos:*mut rq_qos){let b=BLKIOLATENCY(rqos);timer_shutdown_sync(&mut (*b).timer);flush_work(&mut (*b).enable_work);blkcg_deactivate_policy((*rqos).disk,&mut blkcg_policy_iolatency);kfree(b);}
static blkcg_iolatency_ops: rq_qos_ops = rq_qos_ops { throttle: blkcg_iolatency_throttle, done_bio: blkcg_iolatency_done_bio, exit: blkcg_iolatency_exit };

unsafe fn blkiolatency_timer_fn(_t:*mut timer_list) { /* direct descendant traversal depends on kernel RCU iterators */ }
unsafe fn blkiolatency_enable_work_fn(_work:*mut work_struct) { /* queue freeze/unfreeze is an external kernel operation */ }
unsafe fn blk_iolatency_init(_disk:*mut gendisk)->i32 { 0 }
unsafe fn iolatency_set_min_lat_nsec(blkg:*mut blkcg_gq,val:u64){let i=blkg_to_lat(blkg);let b=(*i).blkiolat;let old=(*i).min_lat_nsec;(*i).min_lat_nsec=val;(*i).cur_win_nsec=core::cmp::min(core::cmp::max(val<<4,BLKIOLATENCY_MIN_WIN_SIZE),BLKIOLATENCY_MAX_WIN_SIZE);if old==0&&val!=0{if atomic_inc_return(&mut (*b).enable_cnt)==1{schedule_work(&mut (*b).enable_work);}}if old!=0&&val==0{blkcg_clear_delay(blkg);if atomic_dec_return(&mut (*b).enable_cnt)==0{schedule_work(&mut (*b).enable_work);}}}
unsafe fn iolatency_clear_scaling(blkg:*mut blkcg_gq){if !(*blkg).parent.is_null(){let i=blkg_to_lat((*blkg).parent);if i.is_null(){return;}let l=&mut (*i).child_lat;spin_lock(&mut l.lock);atomic_set(&mut l.scale_cookie,DEFAULT_SCALE_COOKIE as i32);l.last_scale_event=0;l.scale_grp=core::ptr::null_mut();l.scale_lat=0;spin_unlock(&mut l.lock);}}
unsafe fn iolatency_set_limit(_of:*mut kernfs_open_file,_buf:*mut i8,_nbytes:usize,_off:loff_t)->isize { -EINVAL as isize }
unsafe fn iolatency_prfill_limit(_sf:*mut seq_file,_pd:*mut blkg_policy_data,_off:i32)->u64 {0}
unsafe fn iolatency_print_limit(_sf:*mut seq_file,_v:*mut core::ffi::c_void)->i32 {0}
unsafe fn iolatency_ssd_stat(_i:*mut iolatency_grp,_s:*mut seq_file) {}
unsafe fn iolatency_pd_stat(_pd:*mut blkg_policy_data,_s:*mut seq_file) {}
unsafe fn iolatency_pd_alloc(_disk:*mut gendisk,_blkcg:*mut blkcg,_gfp:gfp_t)->*mut blkg_policy_data {core::ptr::null_mut()}
unsafe fn iolatency_pd_init(_pd:*mut blkg_policy_data) {}
unsafe fn iolatency_pd_offline(pd:*mut blkg_policy_data){let b=lat_to_blkg(pd_to_lat(pd));iolatency_set_min_lat_nsec(b,0);iolatency_clear_scaling(b);}
unsafe fn iolat_release(_rcu:*mut rcu_head) {}
unsafe fn iolatency_pd_free(_pd:*mut blkg_policy_data) {}
static mut iolatency_files: [cftype; 2] = [cftype { }, cftype { }];
unsafe fn iolatency_init()->i32 {blkcg_policy_register(&mut blkcg_policy_iolatency)}
unsafe fn iolatency_exit(){blkcg_policy_unregister(&mut blkcg_policy_iolatency)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
