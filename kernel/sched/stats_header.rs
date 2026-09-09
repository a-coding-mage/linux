/* SPDX-License-Identifier: GPL-2.0 */

#[cfg(CONFIG_SCHEDSTATS)]
extern "C" {
    pub static mut sched_schedstats: static_key_false;
}

#[cfg(CONFIG_SCHEDSTATS)]
#[inline]
pub unsafe fn rq_sched_info_arrive(rq: *mut rq, delta: u64) {
    if !rq.is_null() { (*rq).rq_sched_info.run_delay += delta; (*rq).rq_sched_info.pcount += 1; }
}
#[cfg(CONFIG_SCHEDSTATS)]
#[inline]
pub unsafe fn rq_sched_info_depart(rq: *mut rq, delta: u64) { if !rq.is_null() { (*rq).rq_cpu_time += delta; } }
#[cfg(CONFIG_SCHEDSTATS)]
#[inline]
pub unsafe fn rq_sched_info_dequeue(rq: *mut rq, delta: u64) { if !rq.is_null() { (*rq).rq_sched_info.run_delay += delta; } }

#[cfg(CONFIG_SCHEDSTATS)]
macro_rules! schedstat_enabled { () => { unsafe { static_branch_unlikely(&sched_schedstats) } }; }
#[cfg(CONFIG_SCHEDSTATS)] macro_rules! __schedstat_inc { ($var:expr) => { $var += 1 }; }
#[cfg(CONFIG_SCHEDSTATS)] macro_rules! schedstat_inc { ($var:expr) => { if schedstat_enabled!() { $var += 1; } }; }
#[cfg(CONFIG_SCHEDSTATS)] macro_rules! __schedstat_add { ($var:expr, $amt:expr) => { $var += $amt }; }
#[cfg(CONFIG_SCHEDSTATS)] macro_rules! schedstat_add { ($var:expr, $amt:expr) => { if schedstat_enabled!() { $var += $amt; } }; }
#[cfg(CONFIG_SCHEDSTATS)] macro_rules! __schedstat_set { ($var:expr, $val:expr) => { $var = $val }; }
#[cfg(CONFIG_SCHEDSTATS)] macro_rules! schedstat_set { ($var:expr, $val:expr) => { if schedstat_enabled!() { $var = $val; } }; }
#[cfg(CONFIG_SCHEDSTATS)] macro_rules! schedstat_val { ($var:expr) => { $var }; }
#[cfg(CONFIG_SCHEDSTATS)] macro_rules! schedstat_val_or_zero { ($var:expr) => { if schedstat_enabled!() { $var } else { 0 } }; }

extern "C" {
    fn __update_stats_wait_start(rq: *mut rq, p: *mut task_struct, stats: *mut sched_statistics);
    fn __update_stats_wait_end(rq: *mut rq, p: *mut task_struct, stats: *mut sched_statistics);
    fn __update_stats_enqueue_sleeper(rq: *mut rq, p: *mut task_struct, stats: *mut sched_statistics);
}

#[cfg(CONFIG_SCHEDSTATS)]
#[inline] pub unsafe fn check_schedstat_required() {
    if schedstat_enabled!() { return; }
    if trace_sched_stat_wait_enabled() || trace_sched_stat_sleep_enabled() || trace_sched_stat_iowait_enabled() || trace_sched_stat_blocked_enabled() || trace_sched_stat_runtime_enabled() {
        printk_deferred_once(c"Scheduler tracepoints stat_sleep, stat_iowait, stat_blocked and stat_runtime require the kernel parameter schedstats=enable or kernel.sched_schedstats=1\n");
    }
}
#[cfg(not(CONFIG_SCHEDSTATS))] #[inline] pub unsafe fn rq_sched_info_arrive(_: *mut rq, _: u64) {}
#[cfg(not(CONFIG_SCHEDSTATS))] #[inline] pub unsafe fn rq_sched_info_dequeue(_: *mut rq, _: u64) {}
#[cfg(not(CONFIG_SCHEDSTATS))] #[inline] pub unsafe fn rq_sched_info_depart(_: *mut rq, _: u64) {}
#[cfg(not(CONFIG_SCHEDSTATS))] macro_rules! schedstat_enabled { () => { 0 }; }
#[cfg(not(CONFIG_SCHEDSTATS))] macro_rules! __schedstat_inc { ($var:expr) => {}; }
#[cfg(not(CONFIG_SCHEDSTATS))] macro_rules! schedstat_inc { ($var:expr) => {}; }
#[cfg(not(CONFIG_SCHEDSTATS))] macro_rules! __schedstat_add { ($var:expr, $amt:expr) => {}; }
#[cfg(not(CONFIG_SCHEDSTATS))] macro_rules! schedstat_add { ($var:expr, $amt:expr) => {}; }
#[cfg(not(CONFIG_SCHEDSTATS))] macro_rules! __schedstat_set { ($var:expr, $val:expr) => {}; }
#[cfg(not(CONFIG_SCHEDSTATS))] macro_rules! schedstat_set { ($var:expr, $val:expr) => {}; }
#[cfg(not(CONFIG_SCHEDSTATS))] macro_rules! schedstat_val { ($var:expr) => { 0 }; }
#[cfg(not(CONFIG_SCHEDSTATS))] macro_rules! schedstat_val_or_zero { ($var:expr) => { 0 }; }
#[cfg(not(CONFIG_SCHEDSTATS))] macro_rules! __update_stats_wait_start { ($rq:expr,$p:expr,$stats:expr) => {}; }
#[cfg(not(CONFIG_SCHEDSTATS))] macro_rules! __update_stats_wait_end { ($rq:expr,$p:expr,$stats:expr) => {}; }
#[cfg(not(CONFIG_SCHEDSTATS))] macro_rules! __update_stats_enqueue_sleeper { ($rq:expr,$p:expr,$stats:expr) => {}; }
#[cfg(not(CONFIG_SCHEDSTATS))] macro_rules! check_schedstat_required { () => {}; }

#[inline] pub unsafe fn __schedstats_from_se(se: *mut sched_entity) -> *mut sched_statistics {
    #[cfg(CONFIG_FAIR_GROUP_SCHED)] if !entity_is_task(se) { return &mut (*container_of!(se, cfs_tg_state, se)).stats; }
    &mut (*task_of(se)).stats
}

#[cfg(CONFIG_PSI)] extern "C" { fn psi_task_change(task:*mut task_struct, clear:i32, set:i32); fn psi_task_switch(prev:*mut task_struct,next:*mut task_struct,sleep:bool); }
#[cfg(CONFIG_PSI)]
#[inline] pub unsafe fn psi_enqueue(p:*mut task_struct, flags:i32) {
    let mut clear=0; let mut set=0; if static_branch_likely(&psi_disabled) || flags & ENQUEUE_RESTORE != 0 || task_on_cpu(task_rq(p),p) { return; }
    if (*p).se.sched_delayed { WARN_ON_ONCE(!(flags & ENQUEUE_MIGRATED != 0)); if (*p).in_memstall { set |= TSK_MEMSTALL; } if (*p).in_iowait { set |= TSK_IOWAIT; } }
    else if flags & ENQUEUE_MIGRATED != 0 { set=TSK_RUNNING; if (*p).in_memstall { set |= TSK_MEMSTALL|TSK_MEMSTALL_RUNNING; } }
    else { if (*p).in_iowait { clear |= TSK_IOWAIT; } set=TSK_RUNNING; if (*p).in_memstall { set |= TSK_MEMSTALL_RUNNING; } }
    psi_task_change(p,clear,set);
}
#[cfg(CONFIG_PSI)] #[inline] pub unsafe fn psi_dequeue(p:*mut task_struct,flags:i32) { if static_branch_likely(&psi_disabled)||flags&DEQUEUE_SAVE!=0{return;} if flags&DEQUEUE_SLEEP!=0 && (*p).psi_flags&TSK_ONCPU!=0{return;} psi_task_change(p,(*p).psi_flags,0); }
#[cfg(CONFIG_PSI)] #[inline] pub unsafe fn psi_ttwu_dequeue(p:*mut task_struct) { if static_branch_likely(&psi_disabled)||(*p).psi_flags==0{return;} let mut rf=core::mem::MaybeUninit::uninit(); let rq=__task_rq_lock(p,rf.as_mut_ptr()); psi_task_change(p,(*p).psi_flags,0); __task_rq_unlock(rq,p,rf.as_mut_ptr()); }
#[cfg(CONFIG_PSI)] #[inline] pub unsafe fn psi_sched_switch(prev:*mut task_struct,next:*mut task_struct,sleep:bool) { if !static_branch_likely(&psi_disabled){psi_task_switch(prev,next,sleep);} }
#[cfg(not(CONFIG_PSI))] #[inline] pub unsafe fn psi_enqueue(_: *mut task_struct, _: bool) {}
#[cfg(not(CONFIG_PSI))] #[inline] pub unsafe fn psi_dequeue(_: *mut task_struct, _: bool) {}
#[cfg(not(CONFIG_PSI))] #[inline] pub unsafe fn psi_ttwu_dequeue(_: *mut task_struct) {}
#[cfg(not(CONFIG_PSI))] #[inline] pub unsafe fn psi_sched_switch(_: *mut task_struct, _: *mut task_struct, _: bool) {}

#[cfg(CONFIG_SCHED_INFO)]
#[inline] pub unsafe fn sched_info_dequeue(rq:*mut rq,t:*mut task_struct) { if (*t).sched_info.last_queued==0{return;} let delta=rq_clock(rq)-(*t).sched_info.last_queued; (*t).sched_info.last_queued=0; (*t).sched_info.run_delay+=delta; if delta>(*t).sched_info.max_run_delay { (*t).sched_info.max_run_delay=delta; ktime_get_real_ts64(&mut (*t).sched_info.max_run_delay_ts); } if delta!=0 && ((*t).sched_info.min_run_delay==0 || delta<(*t).sched_info.min_run_delay){(*t).sched_info.min_run_delay=delta;} rq_sched_info_dequeue(rq,delta); }
#[cfg(CONFIG_SCHED_INFO)] #[inline] pub unsafe fn sched_info_enqueue(rq:*mut rq,t:*mut task_struct){if (*t).sched_info.last_queued==0{(*t).sched_info.last_queued=rq_clock(rq);}}
#[cfg(CONFIG_SCHED_INFO)] #[inline] pub unsafe fn sched_info_depart(rq:*mut rq,t:*mut task_struct){let delta=rq_clock(rq)-(*t).sched_info.last_arrival;rq_sched_info_depart(rq,delta);if task_is_running(t){sched_info_enqueue(rq,t);}}
#[cfg(CONFIG_SCHED_INFO)] #[inline] pub unsafe fn sched_info_switch(rq:*mut rq,prev:*mut task_struct,next:*mut task_struct){if prev!=(*rq).idle{sched_info_depart(rq,prev);}if next!=(*rq).idle{sched_info_arrive(rq,next);}}
#[cfg(CONFIG_SCHED_INFO)] #[inline] pub unsafe fn sched_info_arrive(rq:*mut rq,t:*mut task_struct){if (*t).sched_info.last_queued==0{return;}let now=rq_clock(rq);let delta=now-(*t).sched_info.last_queued;(*t).sched_info.last_queued=0;(*t).sched_info.run_delay+=delta;(*t).sched_info.last_arrival=now;(*t).sched_info.pcount+=1;if delta>(*t).sched_info.max_run_delay{(*t).sched_info.max_run_delay=delta;ktime_get_real_ts64(&mut (*t).sched_info.max_run_delay_ts);}if delta!=0&&((*t).sched_info.min_run_delay==0||delta<(*t).sched_info.min_run_delay){(*t).sched_info.min_run_delay=delta;}rq_sched_info_arrive(rq,delta);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
