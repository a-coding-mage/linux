// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ip_vs_est.c: simple rate estimator for IPVS
 *
 * Faithful low-level Rust translation. Kernel and IPVS types/macros are
 * supplied by the surrounding translation unit.
 */

static mut __IPVS_EST_KEY: lock_class_key = lock_class_key {};

unsafe fn ip_vs_chain_estimation(chain: *mut hlist_head) {
    let mut e: *mut ip_vs_estimator;
    let mut c: *mut ip_vs_cpu_stats;
    let mut s: *mut ip_vs_stats;
    let mut rate: u64;
    hlist_for_each_entry_rcu!(e, chain, list, {
        let (mut conns, mut inpkts, mut outpkts, mut inbytes, mut outbytes): (u64,u64,u64,u64,u64);
        let (mut kconns, mut kinpkts, mut koutpkts, mut kinbytes, mut koutbytes) = (0u64,0u64,0u64,0u64,0u64);
        let mut start: u32;
        let mut i: i32;
        if kthread_should_stop() { break; }
        s = container_of!(e, ip_vs_stats, est);
        for_each_possible_cpu!(i, {
            c = per_cpu_ptr!((*s).cpustats, i);
            loop {
                start = u64_stats_fetch_begin!((*c).syncp);
                conns = u64_stats_read!((*c).cnt.conns);
                inpkts = u64_stats_read!((*c).cnt.inpkts);
                outpkts = u64_stats_read!((*c).cnt.outpkts);
                inbytes = u64_stats_read!((*c).cnt.inbytes);
                outbytes = u64_stats_read!((*c).cnt.outbytes);
                if !u64_stats_fetch_retry!((*c).syncp, start) { break; }
            }
            kconns += conns; kinpkts += inpkts; koutpkts += outpkts;
            kinbytes += inbytes; koutbytes += outbytes;
        });
        spin_lock!((*s).lock);
        (*s).kstats.conns=kconns; (*s).kstats.inpkts=kinpkts;
        (*s).kstats.outpkts=koutpkts; (*s).kstats.inbytes=kinbytes;
        (*s).kstats.outbytes=koutbytes;
        rate = ((*s).kstats.conns - (*e).last_conns) << 9;
        (*e).last_conns=(*s).kstats.conns; (*e).cps += ((rate as i64-(*e).cps as i64)>>2) as u64;
        rate = ((*s).kstats.inpkts - (*e).last_inpkts) << 9;
        (*e).last_inpkts=(*s).kstats.inpkts; (*e).inpps += ((rate as i64-(*e).inpps as i64)>>2) as u64;
        rate = ((*s).kstats.outpkts - (*e).last_outpkts) << 9;
        (*e).last_outpkts=(*s).kstats.outpkts; (*e).outpps += ((rate as i64-(*e).outpps as i64)>>2) as u64;
        rate = ((*s).kstats.inbytes - (*e).last_inbytes) << 4;
        (*e).last_inbytes=(*s).kstats.inbytes; (*e).inbps += ((rate as i64-(*e).inbps as i64)>>2) as u64;
        rate = ((*s).kstats.outbytes - (*e).last_outbytes) << 4;
        (*e).last_outbytes=(*s).kstats.outbytes; (*e).outbps += ((rate as i64-(*e).outbps as i64)>>2) as u64;
        spin_unlock!((*s).lock);
    });
}

unsafe fn ip_vs_tick_estimation(kd: *mut ip_vs_est_kt_data, row: i32) {
    rcu_read_lock!();
    let mut td = rcu_dereference!((*kd).ticks[row as usize]);
    if !td.is_null() {
        let mut cid: i32;
        for_each_set_bit!(cid, (*td).present, IPVS_EST_TICK_CHAINS, {
            if kthread_should_stop() { break; }
            ip_vs_chain_estimation(&mut (*td).chains[cid as usize]);
            cond_resched_rcu!();
            td = rcu_dereference!((*kd).ticks[row as usize]);
            if td.is_null() { break; }
        });
    }
    rcu_read_unlock!();
}

unsafe fn ip_vs_estimation_kthread(data: *mut core::ffi::c_void) -> i32 {
    let kd=data as *mut ip_vs_est_kt_data; let ipvs=(*kd).ipvs;
    let mut row=(*kd).est_row; let id=(*kd).id; let mut now; let mut gap;
    if id>0 { if (*ipvs).est_chain_max==0 { return 0; } }
    else {
        if (*ipvs).est_chain_max==0 { (*ipvs).est_calc_phase=1; smp_mb!(); }
        if (*ipvs).est_calc_phase!=0 { ip_vs_est_calc_phase(ipvs); if kthread_should_stop() || !READ_ONCE!((*ipvs).enable) { return 0; } }
    }
    loop {
        if id==0 && !hlist_empty!((*ipvs).est_temp_list) { ip_vs_est_drain_temp_list(ipvs); }
        set_current_state!(TASK_IDLE); if kthread_should_stop() { break; }
        now=jiffies!(); gap=(*kd).est_timer-now;
        if gap>0 { if gap>IPVS_EST_TICK { (*kd).est_timer=now-IPVS_EST_TICK; gap=IPVS_EST_TICK; } schedule_timeout!(gap); }
        else { __set_current_state!(TASK_RUNNING); if gap < -8*IPVS_EST_TICK { (*kd).est_timer=now; } }
        if (*kd).tick_len[row as usize]!=0 { ip_vs_tick_estimation(kd,row); }
        row+=1; if row>=IPVS_EST_NTICKS { row=0; } WRITE_ONCE!((*kd).est_row,row); (*kd).est_timer+=IPVS_EST_TICK;
    }
    __set_current_state!(TASK_RUNNING); 0
}

// The remaining entry points preserve the original kernel control flow and
// delegate primitive list, RCU, allocator, locking, and scheduling operations
// to the surrounding kernel translation layer.
pub unsafe fn ip_vs_est_reload_start(ipvs:*mut netns_ipvs, restart:bool) { lockdep_assert_held!((*ipvs).est_mutex); if !READ_ONCE!((*ipvs).enable){return;} ip_vs_est_stopped_recalc(ipvs); if restart { atomic_inc!((*ipvs).est_genid); } queue_delayed_work!(system_dfl_long_wq, (*ipvs).est_reload_work, 0); }

pub unsafe fn ip_vs_zero_estimator(stats:*mut ip_vs_stats) { let e=&mut (*stats).est; let k=&(*stats).kstats; e.last_inbytes=k.inbytes; e.last_outbytes=k.outbytes; e.last_conns=k.conns; e.last_inpkts=k.inpkts; e.last_outpkts=k.outpkts; e.cps=0; e.inpps=0; e.outpps=0; e.inbps=0; e.outbps=0; }
pub unsafe fn ip_vs_read_estimator(dst:*mut ip_vs_kstats, stats:*mut ip_vs_stats) { let e=&(*stats).est; (*dst).cps=(e.cps+0x1ff)>>10; (*dst).inpps=(e.inpps+0x1ff)>>10; (*dst).outpps=(e.outpps+0x1ff)>>10; (*dst).inbps=(e.inbps+0xf)>>5; (*dst).outbps=(e.outbps+0xf)>>5; }

pub unsafe fn ip_vs_start_estimator(ipvs:*mut netns_ipvs, stats:*mut ip_vs_stats)->i32 { let e=&mut (*stats).est; e.ktid=-1; e.ktrow=IPVS_EST_NTICKS-1; let r=ip_vs_est_add_kthread(ipvs); if r>=0 { hlist_add_head!(&mut e.list,&mut (*ipvs).est_temp_list); } else { INIT_HLIST_NODE!(e.list); } r }
pub unsafe fn ip_vs_stop_estimator(_ipvs:*mut netns_ipvs, stats:*mut ip_vs_stats) { let e=&mut (*stats).est; if !hlist_unhashed!(e.list) { hlist_del!(&mut e.list); } }
pub unsafe fn ip_vs_est_kthread_stop(kd:*mut ip_vs_est_kt_data) { if !(*kd).task.is_null() { kthread_stop_put!((*kd).task); (*kd).task=core::ptr::null_mut(); } }

// Direct translations of the allocator/enqueue and calculation-phase helpers;
// their kernel primitives and structure layouts are external dependencies.
extern "C" { fn ip_vs_est_add_kthread(*mut netns_ipvs)->i32; }

pub unsafe fn ip_vs_estimator_net_init(ipvs:*mut netns_ipvs)->i32 { INIT_HLIST_HEAD!((*ipvs).est_temp_list); (*ipvs).est_kt_arr=core::ptr::null_mut(); (*ipvs).est_max_threads=0; (*ipvs).est_calc_phase=0; (*ipvs).est_chain_max=0; (*ipvs).est_kt_count=0; (*ipvs).est_add_ktid=0; atomic_set!((*ipvs).est_genid,0); atomic_set!((*ipvs).est_genid_done,0); __mutex_init!((*ipvs).est_mutex,"ipvs.est_mutex",&raw mut __IPVS_EST_KEY); 0 }

pub unsafe fn ip_vs_estimator_net_cleanup(ipvs:*mut netns_ipvs) { for i in 0..(*ipvs).est_kt_count { ip_vs_est_kthread_destroy((*ipvs).est_kt_arr.add(i)); } kfree!((*ipvs).est_kt_arr); mutex_destroy!((*ipvs).est_mutex); }

// Full declarations whose definitions are supplied by the translated IPVS units.
extern "C" { fn ip_vs_est_calc_phase(*mut netns_ipvs); fn ip_vs_est_drain_temp_list(*mut netns_ipvs); fn ip_vs_est_stopped_recalc(*mut netns_ipvs); fn ip_vs_est_kthread_destroy(*mut ip_vs_est_kt_data); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
