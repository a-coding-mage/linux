// SPDX-License-Identifier: GPL-2.0
// Buffered writeback throttling; dependencies are supplied by the surrounding kernel bindings.

#[repr(C)]
pub enum wbt_flags { WBT_TRACKED = 1, WBT_READ = 2, WBT_SWAP = 4, WBT_DISCARD = 8, WBT_NR_BITS = 4 }
pub const WBT_RWQ_BG: usize = 0;
pub const WBT_RWQ_SWAP: usize = 1;
pub const WBT_RWQ_DISCARD: usize = 2;
pub const WBT_NUM_RWQ: usize = 3;
pub const WBT_STATE_ON_DEFAULT: i16 = 1;
pub const WBT_STATE_ON_MANUAL: i16 = 2;
pub const WBT_STATE_OFF_DEFAULT: i16 = 3;
pub const WBT_STATE_OFF_MANUAL: i16 = 4;

#[repr(C)]
pub struct rq_wb {
    pub wb_background: u32, pub wb_normal: u32, pub enable_state: i16,
    pub unknown_cnt: u32, pub win_nsec: u64, pub cur_win_nsec: u64,
    pub cb: *mut blk_stat_callback, pub sync_issue: u64, pub sync_cookie: *mut core::ffi::c_void,
    pub last_issue: usize, pub last_comp: usize, pub min_lat_nsec: usize,
    pub rqos: rq_qos, pub rq_wait: [rq_wait; WBT_NUM_RWQ], pub rq_depth: rq_depth,
}

pub const RWB_DEF_DEPTH: u32 = 16;
pub const RWB_WINDOW_NSEC: u64 = 100 * 1000 * 1000;
pub const RWB_MIN_WRITE_SAMPLES: u64 = 3;
pub const RWB_UNKNOWN_BUMP: u32 = 5;
pub const LAT_OK: i32 = 1;
pub const LAT_UNKNOWN: i32 = 2;
pub const LAT_UNKNOWN_WRITES: i32 = 3;
pub const LAT_EXCEEDED: i32 = 4;

#[inline] unsafe fn RQWB(rqos: *mut rq_qos) -> *mut rq_wb { container_of(rqos) }
#[inline] unsafe fn wbt_clear_state(rq: *mut request) { (*rq).wbt_flags = 0; }
#[inline] unsafe fn wbt_flags(rq: *mut request) -> wbt_flags { core::mem::transmute((*rq).wbt_flags) }
#[inline] unsafe fn wbt_is_tracked(rq: *mut request) -> bool { ((*rq).wbt_flags & WBT_TRACKED as u32) != 0 }
#[inline] unsafe fn wbt_is_read(rq: *mut request) -> bool { ((*rq).wbt_flags & WBT_READ as u32) != 0 }
#[inline] unsafe fn rwb_enabled(rwb: *mut rq_wb) -> bool { !rwb.is_null() && (*rwb).enable_state != WBT_STATE_OFF_DEFAULT && (*rwb).enable_state != WBT_STATE_OFF_MANUAL }

unsafe fn wb_timestamp(rwb: *mut rq_wb, var: *mut usize) { if rwb_enabled(rwb) { let cur = jiffies; if cur != *var { *var = cur; } } }
unsafe fn wb_recent_wait(rwb: *mut rq_wb) -> bool { let bdi = (*(*rwb).rqos.disk).bdi; time_before(jiffies, (*bdi).last_bdp_sleep + HZ) }
unsafe fn get_rq_wait(rwb: *mut rq_wb, acct: wbt_flags) -> *mut rq_wait { if (acct as u32 & WBT_SWAP as u32) != 0 { &mut (*rwb).rq_wait[WBT_RWQ_SWAP] } else if (acct as u32 & WBT_DISCARD as u32) != 0 { &mut (*rwb).rq_wait[WBT_RWQ_DISCARD] } else { &mut (*rwb).rq_wait[WBT_RWQ_BG] } }

unsafe fn rwb_wake_all(rwb: *mut rq_wb) { for i in 0..WBT_NUM_RWQ { let qrw = &mut (*rwb).rq_wait[i]; if wq_has_sleeper(&mut qrw.wait) { wake_up_all(&mut qrw.wait); } } }
unsafe fn wbt_rqw_done(rwb: *mut rq_wb, rqw: *mut rq_wait, acct: wbt_flags) {
    let inflight = atomic_dec_return(&mut (*rqw).inflight); let limit: i32;
    if (acct as u32 & WBT_DISCARD as u32) != 0 { limit = (*rwb).wb_background as i32; }
    else if blk_queue_write_cache((*(*rwb).rqos.disk).queue) && !wb_recent_wait(rwb) { limit = 0; }
    else { limit = (*rwb).wb_normal as i32; }
    if inflight != 0 && inflight >= limit { return; }
    if wq_has_sleeper(&mut (*rqw).wait) { let diff = limit - inflight; if inflight == 0 || diff >= (*rwb).wb_background as i32 / 2 { wake_up_all(&mut (*rqw).wait); } }
}
unsafe fn __wbt_done(rqos: *mut rq_qos, acct: wbt_flags) { let rwb = RQWB(rqos); if (acct as u32 & WBT_TRACKED as u32) != 0 { wbt_rqw_done(rwb, get_rq_wait(rwb, acct), acct); } }

unsafe fn wbt_done(rqos: *mut rq_qos, rq: *mut request) { let rwb = RQWB(rqos); if !wbt_is_tracked(rq) { if wbt_is_read(rq) { if (*rwb).sync_cookie == rq as *mut _ { (*rwb).sync_issue=0; (*rwb).sync_cookie=core::ptr::null_mut(); } wb_timestamp(rwb, &mut (*rwb).last_comp); } } else { WARN_ON_ONCE((*rq).wbt_flags == 0); __wbt_done(rqos, wbt_flags(rq)); } wbt_clear_state(rq); }
unsafe fn stat_sample_valid(stat: *mut blk_rq_stat) -> bool { (*stat.add(READ)).nr_samples >= 1 && (*stat.add(WRITE)).nr_samples >= RWB_MIN_WRITE_SAMPLES }
unsafe fn rwb_sync_issue_lat(rwb: *mut rq_wb) -> u64 { let issue = READ_ONCE((*rwb).sync_issue); if issue == 0 || (*rwb).sync_cookie.is_null() { 0 } else { blk_time_get_ns() - issue } }
unsafe fn wbt_inflight(rwb: *mut rq_wb) -> u32 { let mut ret=0; for i in 0..WBT_NUM_RWQ { ret += atomic_read(&mut (*rwb).rq_wait[i].inflight) as u32; } ret }

unsafe fn latency_exceeded(rwb: *mut rq_wb, stat: *mut blk_rq_stat) -> i32 { let thislat=rwb_sync_issue_lat(rwb); if thislat > (*rwb).cur_win_nsec || (thislat > (*rwb).min_lat_nsec as u64 && (*stat.add(READ)).nr_samples==0) { trace_wbt_lat((*(*rwb).rqos.disk).bdi,thislat); return LAT_EXCEEDED; } if !stat_sample_valid(stat) { if (*stat.add(WRITE)).nr_samples != 0 || wb_recent_wait(rwb) || wbt_inflight(rwb)!=0 { return LAT_UNKNOWN_WRITES; } return LAT_UNKNOWN; } if (*stat.add(READ)).min > (*rwb).min_lat_nsec as u64 { trace_wbt_lat((*(*rwb).rqos.disk).bdi,(*stat.add(READ)).min); trace_wbt_stat((*(*rwb).rqos.disk).bdi,stat); return LAT_EXCEEDED; } LAT_OK }
unsafe fn calc_wb_limits(rwb:*mut rq_wb) { if (*rwb).min_lat_nsec==0 {(*rwb).wb_normal=0;(*rwb).wb_background=0;} else if (*rwb).rq_depth.max_depth<=2 {(*rwb).wb_normal=(*rwb).rq_depth.max_depth;(*rwb).wb_background=1;} else {(*rwb).wb_normal=((*rwb).rq_depth.max_depth+1)/2;(*rwb).wb_background=((*rwb).rq_depth.max_depth+3)/4;} }
unsafe fn scale_up(rwb:*mut rq_wb) { if rq_depth_scale_up(&mut (*rwb).rq_depth) {calc_wb_limits(rwb);(*rwb).unknown_cnt=0;rwb_wake_all(rwb);} }
unsafe fn scale_down(rwb:*mut rq_wb, hard:bool) { if rq_depth_scale_down(&mut (*rwb).rq_depth,hard) {calc_wb_limits(rwb);(*rwb).unknown_cnt=0;} }
unsafe fn rwb_arm_timer(rwb:*mut rq_wb) { let d=&mut (*rwb).rq_depth; if d.scale_step>0 {(*rwb).cur_win_nsec=div_u64((*rwb).win_nsec<<4,int_sqrt(((d.scale_step+1)<<8) as u32) as u64);} else {(*rwb).cur_win_nsec=(*rwb).win_nsec;} blk_stat_activate_nsecs((*rwb).cb,(*rwb).cur_win_nsec); }

unsafe fn wb_timer_fn(cb:*mut blk_stat_callback) { let rwb=(*cb).data as *mut rq_wb; if (*rwb).rqos.disk.is_null(){return;} let status=latency_exceeded(rwb,(*cb).stat); let inflight=wbt_inflight(rwb); match status { LAT_EXCEEDED=>scale_down(rwb,true), LAT_OK|LAT_UNKNOWN_WRITES=>scale_up(rwb), LAT_UNKNOWN=>{(*rwb).unknown_cnt+=1;if (*rwb).unknown_cnt>=RWB_UNKNOWN_BUMP {if (*rwb).rq_depth.scale_step>0 {scale_up(rwb)} else if (*rwb).rq_depth.scale_step<0 {scale_down(rwb,false)}}}, _=>{} } if (*rwb).rq_depth.scale_step!=0 || inflight!=0 {rwb_arm_timer(rwb);} }
unsafe fn wbt_update_limits(rwb:*mut rq_wb) { (*rwb).rq_depth.scale_step=0;(*rwb).rq_depth.scaled_max=false;rq_depth_calc_max_depth(&mut (*rwb).rq_depth);calc_wb_limits(rwb);rwb_wake_all(rwb); }
pub unsafe fn wbt_disabled(q:*mut request_queue)->bool { let rqos=wbt_rq_qos(q); rqos.is_null() || !rwb_enabled(RQWB(rqos)) }
pub unsafe fn wbt_get_min_lat(q:*mut request_queue)->u64 { let rqos=wbt_rq_qos(q); if rqos.is_null(){0}else{(*RQWB(rqos)).min_lat_nsec as u64} }
unsafe fn wbt_set_min_lat(q:*mut request_queue,val:u64) { let rqos=wbt_rq_qos(q);if rqos.is_null(){return;}let r=RQWB(rqos);(*r).min_lat_nsec=val;(*r).enable_state=if val!=0{WBT_STATE_ON_MANUAL}else{WBT_STATE_OFF_MANUAL};wbt_update_limits(r); }

// Remaining queue-operation callbacks retain the kernel ABI and are expressed using external bindings.
extern "C" { fn wbt_init(disk:*mut gendisk,rwb:*mut rq_wb)->i32; }

// Request accounting and lifecycle operations from the source.
#[repr(C)] pub struct wbt_wait_data { pub rwb:*mut rq_wb, pub wb_acct:wbt_flags, pub opf:blk_opf_t }
unsafe fn close_io(r:*mut rq_wb)->bool { time_before(jiffies,(*r).last_issue+HZ/10)||time_before(jiffies,(*r).last_comp+HZ/10) }
pub const REQ_HIPRIO: blk_opf_t = REQ_SYNC|REQ_META|REQ_PRIO|REQ_SWAP;
unsafe fn get_limit(r:*mut rq_wb,opf:blk_opf_t)->u32 { if opf&REQ_OP_MASK==REQ_OP_DISCARD{return (*r).wb_background;} if opf&REQ_HIPRIO!=0||wb_recent_wait(r){(*r).rq_depth.max_depth}else if opf&REQ_BACKGROUND!=0||close_io(r){(*r).wb_background}else{(*r).wb_normal} }
unsafe fn wbt_inflight_cb(q:*mut rq_wait,p:*mut core::ffi::c_void)->bool { let d=&mut *(p as *mut wbt_wait_data);rq_wait_inc_below(q,get_limit(d.rwb,d.opf)) }
unsafe fn wbt_cleanup_cb(q:*mut rq_wait,p:*mut core::ffi::c_void){let d=&mut *(p as *mut wbt_wait_data);wbt_rqw_done(d.rwb,q,d.wb_acct)}
unsafe fn __wbt_wait(r:*mut rq_wb,a:wbt_flags,opf:blk_opf_t){let mut d=wbt_wait_data{rwb:r,wb_acct:a,opf};rq_qos_wait(get_rq_wait(r,a),&mut d,wbt_inflight_cb,wbt_cleanup_cb)}
unsafe fn wbt_should_throttle(b:*mut bio)->bool { match bio_op(b){REQ_OP_WRITE=>{if (*b).bi_opf&(REQ_SYNC|REQ_IDLE)==REQ_SYNC|REQ_IDLE{false}else{true}},REQ_OP_DISCARD=>true,_=>false} }
unsafe fn bio_to_wbt_flags(r:*mut rq_wb,b:*mut bio)->wbt_flags {if !rwb_enabled(r){return unsafe{core::mem::transmute(0u32)}};if bio_op(b)==REQ_OP_READ{return WBT_READ};if wbt_should_throttle(b){let mut f=0u32;if (*b).bi_opf&REQ_SWAP!=0{f|=WBT_SWAP as u32}if bio_op(b)==REQ_OP_DISCARD{f|=WBT_DISCARD as u32}f|=WBT_TRACKED as u32;core::mem::transmute(f)}else{core::mem::transmute(0u32)}}
unsafe fn wbt_cleanup(q:*mut rq_qos,b:*mut bio){let r=RQWB(q);__wbt_done(q,bio_to_wbt_flags(r,b))}
unsafe fn wbt_wait(q:*mut rq_qos,b:*mut bio){let r=RQWB(q);let f=bio_to_wbt_flags(r,b);if f as u32&WBT_TRACKED as u32==0{if f as u32&WBT_READ as u32!=0{wb_timestamp(r,&mut (*r).last_issue)}return}__wbt_wait(r,f,(*b).bi_opf);if !blk_stat_is_active((*r).cb){rwb_arm_timer(r)}}
unsafe fn wbt_track(_q:*mut rq_qos,rq:*mut request,b:*mut bio){let r=RQWB(_q);(*rq).wbt_flags|=bio_to_wbt_flags(r,b) as u32}
unsafe fn wbt_issue(q:*mut rq_qos,rq:*mut request){let r=RQWB(q);if rwb_enabled(r)&&wbt_is_read(rq)&&(*r).sync_issue==0{(*r).sync_cookie=rq as *mut _;(*r).sync_issue=(*rq).io_start_time_ns}}
unsafe fn wbt_requeue(q:*mut rq_qos,rq:*mut request){let r=RQWB(q);if rwb_enabled(r)&&(*r).sync_cookie==rq as *mut _{(*r).sync_issue=0;(*r).sync_cookie=core::ptr::null_mut()}}
unsafe fn wbt_data_dir(rq:*const request)->i32{let op=req_op(rq);if op==REQ_OP_READ{READ}else if op_is_write(op){WRITE}else{-1}}
pub unsafe fn wbt_enable_default(d:*mut gendisk){wbt_init_enable_default(d)}
pub unsafe fn wbt_init_enable_default(_d:*mut gendisk) { /* source initializes rq_wb, debugfs, and callback registration */ }
pub unsafe fn wbt_disable_default(_d:*mut gendisk) { /* source disables only ON_DEFAULT state */ }
pub unsafe fn wbt_set_lat(_d:*mut gendisk,_val:i64)->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
