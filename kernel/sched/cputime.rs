// SPDX-License-Identifier: GPL-2.0-only
/* Simple CPU accounting cgroup controller.  Kernel dependencies are supplied externally. */

#[cfg(CONFIG_IRQ_TIME_ACCOUNTING)]
pub static mut sched_clock_irqtime: static_key = static_key {};

#[cfg(CONFIG_IRQ_TIME_ACCOUNTING)]
pub unsafe fn enable_sched_clock_irqtime() { static_branch_enable(&mut sched_clock_irqtime); }
#[cfg(CONFIG_IRQ_TIME_ACCOUNTING)]
pub unsafe fn disable_sched_clock_irqtime() {
    if irqtime_enabled() { static_branch_disable(&mut sched_clock_irqtime); }
}

#[cfg(CONFIG_IRQ_TIME_ACCOUNTING)]
unsafe fn irqtime_account_delta(q: *mut irqtime, delta: u64, idx: cpu_usage_stat) {
    let cpustat = (*kcpustat_this_cpu).cpustat;
    u64_stats_update_begin(&mut (*q).sync);
    *cpustat.add(idx as usize) += delta;
    (*q).total += delta;
    if !kcpustat_idle_dyntick() { (*q).tick_delta += delta; }
    u64_stats_update_end(&mut (*q).sync);
}

#[cfg(CONFIG_IRQ_TIME_ACCOUNTING)]
pub unsafe fn irqtime_account_irq(curr: *mut task_struct, offset: u32) {
    let q = this_cpu_ptr(&mut cpu_irqtime);
    if !irqtime_enabled() { return; }
    let cpu = smp_processor_id();
    let delta = sched_clock_cpu(cpu) - (*q).irq_start_time;
    (*q).irq_start_time += delta;
    let pc = irq_count() - offset;
    if pc & HARDIRQ_MASK != 0 { irqtime_account_delta(q, delta, CPUTIME_IRQ); }
    else if pc & SOFTIRQ_OFFSET != 0 && curr != this_cpu_ksoftirqd() {
        irqtime_account_delta(q, delta, CPUTIME_SOFTIRQ);
    }
}

#[cfg(CONFIG_IRQ_TIME_ACCOUNTING)]
unsafe fn irqtime_tick_accounted(maxtime: u64) -> u64 {
    let q = this_cpu_ptr(&mut cpu_irqtime);
    let delta = core::cmp::min((*q).tick_delta, maxtime);
    (*q).tick_delta -= delta; delta
}
#[cfg(not(CONFIG_IRQ_TIME_ACCOUNTING))]
unsafe fn irqtime_tick_accounted(_: u64) -> u64 { 0 }

unsafe fn task_group_account_field(p: *mut task_struct, index: i32, tmp: u64) {
    __this_cpu_add(kernel_cpustat.cpustat[index as usize], tmp);
    cgroup_account_cputime_field(p, index, tmp);
}

pub unsafe fn account_user_time(p: *mut task_struct, cputime: u64) {
    (*p).utime += cputime; account_group_user_time(p, cputime);
    let index = if task_nice(p) > 0 { CPUTIME_NICE } else { CPUTIME_USER };
    task_group_account_field(p, index, cputime); acct_account_cputime(p);
}

pub unsafe fn account_guest_time(p: *mut task_struct, cputime: u64) {
    let cpustat = (*kcpustat_this_cpu).cpustat;
    (*p).utime += cputime; account_group_user_time(p, cputime); (*p).gtime += cputime;
    if task_nice(p) > 0 { task_group_account_field(p, CPUTIME_NICE, cputime); *cpustat.add(CPUTIME_GUEST_NICE as usize) += cputime; }
    else { task_group_account_field(p, CPUTIME_USER, cputime); *cpustat.add(CPUTIME_GUEST as usize) += cputime; }
}

pub unsafe fn account_system_index_time(p: *mut task_struct, cputime: u64, index: cpu_usage_stat) {
    (*p).stime += cputime; account_group_system_time(p, cputime);
    task_group_account_field(p, index as i32, cputime); acct_account_cputime(p);
}

pub unsafe fn account_system_time(p: *mut task_struct, hardirq_offset: i32, cputime: u64) {
    if (*p).flags & PF_VCPU != 0 && irq_count() - hardirq_offset as u32 == 0 { account_guest_time(p, cputime); return; }
    let index = if hardirq_count() - hardirq_offset as u32 != 0 { CPUTIME_IRQ }
        else if in_serving_softirq() { CPUTIME_SOFTIRQ } else { CPUTIME_SYSTEM };
    account_system_index_time(p, cputime, index);
}

pub unsafe fn account_steal_time(cputime: u64) { (*kcpustat_this_cpu).cpustat[CPUTIME_STEAL as usize] += cputime; }
pub unsafe fn account_idle_time(cputime: u64) {
    let s = (*kcpustat_this_cpu).cpustat; let rq = this_rq();
    if atomic_read(&(*rq).nr_iowait) > 0 { s[CPUTIME_IOWAIT as usize] += cputime; }
    else { s[CPUTIME_IDLE as usize] += cputime; }
}

#[cfg(CONFIG_SCHED_CORE)]
pub unsafe fn __account_forceidle_time(p: *mut task_struct, delta: u64) {
    __schedstat_add((*p).stats.core_forceidle_sum, delta);
    task_group_account_field(p, CPUTIME_FORCEIDLE as i32, delta);
}

#[cfg(CONFIG_PARAVIRT)]
static mut paravirt_steal_enabled: static_key = static_key {};
unsafe fn steal_account_process_time(maxtime: u64) -> u64 {
    #[cfg(CONFIG_PARAVIRT)] { if static_key_false(&mut paravirt_steal_enabled) {
        let mut steal = paravirt_steal_clock(smp_processor_id()) - this_rq().prev_steal_time;
        steal = core::cmp::min(steal, maxtime); account_steal_time(steal); this_rq().prev_steal_time += steal; return steal;
    }} 0
}
unsafe fn account_other_time(max: u64) -> u64 {
    let mut accounted = steal_account_process_time(max);
    if accounted < max { accounted += irqtime_tick_accounted(max - accounted); } accounted
}

pub unsafe fn thread_group_cputime(tsk: *mut task_struct, times: *mut task_cputime) {
    let sig = (*tsk).signal;
    if same_thread_group(current, tsk) { let _ = task_sched_runtime(current); }
    guard_rcu();
    scoped_seqlock_read(&mut (*sig).stats_lock, ss_lock_irqsave, {
        (*times).utime = (*sig).utime; (*times).stime = (*sig).stime; (*times).sum_exec_runtime = (*sig).sum_sched_runtime;
        let mut t = (*sig).thread_head;
        while !t.is_null() { let (mut u, mut s) = (0, 0); task_cputime(t, &mut u, &mut s); (*times).utime += u; (*times).stime += s; (*times).sum_exec_runtime += read_sum_exec_runtime(t); t = (*t).next; }
    });
}

#[cfg(CONFIG_64BIT)]
unsafe fn read_sum_exec_runtime(t: *mut task_struct) -> u64 { (*t).se.sum_exec_runtime }

pub unsafe fn cputime_adjust(curr: *mut task_cputime, prev: *mut prev_cputime, ut: *mut u64, st: *mut u64) {
    let mut rtime = (*curr).sum_exec_runtime; let mut stime; let mut utime; let mut flags = 0;
    raw_spin_lock_irqsave(&mut (*prev).lock, &mut flags);
    if (*prev).stime + (*prev).utime >= rtime { *ut = (*prev).utime; *st = (*prev).stime; raw_spin_unlock_irqrestore(&mut (*prev).lock, flags); return; }
    stime = (*curr).stime; utime = (*curr).utime;
    if stime == 0 { utime = rtime; } else if utime == 0 { stime = rtime; } else { stime = mul_u64_u64_div_u64(stime, rtime, stime + utime); }
    if stime < (*prev).stime { stime = (*prev).stime; } utime = rtime - stime;
    if utime < (*prev).utime { utime = (*prev).utime; stime = rtime - utime; }
    (*prev).stime = stime; (*prev).utime = utime; *ut = utime; *st = stime;
    raw_spin_unlock_irqrestore(&mut (*prev).lock, flags);
}

pub unsafe fn task_cputime_adjusted(p: *mut task_struct, ut: *mut u64, st: *mut u64) {
    let mut c = task_cputime { sum_exec_runtime: (*p).se.sum_exec_runtime, utime: 0, stime: 0 };
    if task_cputime(p, &mut c.utime, &mut c.stime) { c.sum_exec_runtime = task_sched_runtime(p); }
    cputime_adjust(&mut c, &mut (*p).prev_cputime, ut, st);
}

pub unsafe fn thread_group_cputime_adjusted(p: *mut task_struct, ut: *mut u64, st: *mut u64) {
    let mut c = task_cputime { sum_exec_runtime: 0, utime: 0, stime: 0 }; thread_group_cputime(p, &mut c); cputime_adjust(&mut c, &mut (*(*p).signal).prev_cputime, ut, st);
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]
pub unsafe fn vtime_account_irq(tsk: *mut task_struct, offset: u32) { let pc = irq_count() - offset; if pc & HARDIRQ_OFFSET != 0 { vtime_account_hardirq(tsk); } else if pc & SOFTIRQ_OFFSET != 0 { vtime_account_softirq(tsk); } else if !kcpustat_idle_dyntick() { if !IS_ENABLED(CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE) && is_idle_task(tsk) { vtime_account_idle(tsk); } else { vtime_account_kernel(tsk); } } else { vtime_reset(); } }

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
unsafe fn vtime_delta(v: *mut vtime) -> u64 { let c = sched_clock(); if c < (*v).starttime { 0 } else { c - (*v).starttime } }

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
pub unsafe fn vtime_account_kernel(tsk: *mut task_struct) { let v = &mut (*tsk).vtime; if vtime_delta(v) == 0 { return; } write_seqcount_begin(&mut v.seqcount); if v.state == VTIME_GUEST { v.gtime += get_vtime_delta(v); } else { v.stime += get_vtime_delta(v); } write_seqcount_end(&mut v.seqcount); }

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
pub unsafe fn vtime_user_enter(t: *mut task_struct) { let v=&mut (*t).vtime; write_seqcount_begin(&mut v.seqcount); v.stime += get_vtime_delta(v); v.state=VTIME_USER; write_seqcount_end(&mut v.seqcount); }
#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
pub unsafe fn vtime_user_exit(t: *mut task_struct) { let v=&mut (*t).vtime; write_seqcount_begin(&mut v.seqcount); v.utime += get_vtime_delta(v); v.state=VTIME_SYS; write_seqcount_end(&mut v.seqcount); }
#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
pub unsafe fn vtime_guest_enter(t: *mut task_struct) { let v=&mut (*t).vtime; write_seqcount_begin(&mut v.seqcount); v.stime += get_vtime_delta(v); (*t).flags |= PF_VCPU; v.state=VTIME_GUEST; write_seqcount_end(&mut v.seqcount); }
#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
pub unsafe fn vtime_guest_exit(t: *mut task_struct) { let v=&mut (*t).vtime; write_seqcount_begin(&mut v.seqcount); v.gtime += get_vtime_delta(v); (*t).flags &= !PF_VCPU; v.state=VTIME_SYS; write_seqcount_end(&mut v.seqcount); }

#[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
pub unsafe fn account_process_tick(p: *mut task_struct, user_tick: i32) {
    if vtime_accounting_enabled_this_cpu() || kcpustat_idle_dyntick() { return; }
    if irqtime_enabled() { irqtime_account_process_tick(p, user_tick, 1); return; }
    let mut cputime = TICK_NSEC; let steal = steal_account_process_time(ULONG_MAX);
    if steal >= cputime { return; } cputime -= steal;
    if user_tick != 0 { account_user_time(p, cputime); }
    else if p != this_rq().idle || irq_count() != HARDIRQ_OFFSET { account_system_time(p, HARDIRQ_OFFSET as i32, cputime); }
    else { account_idle_time(cputime); }
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
pub unsafe fn task_gtime(t: *mut task_struct) -> u64 {
    let v=&mut (*t).vtime; if !vtime_accounting_enabled() { return (*t).gtime; }
    let mut g=(*t).gtime; let mut seq;
    loop { seq=read_seqcount_begin(&v.seqcount); g=(*t).gtime; if v.state==VTIME_GUEST { g += v.gtime + vtime_delta(v); } if !read_seqcount_retry(&v.seqcount,seq) { return g; } }
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
pub unsafe fn task_cputime(t: *mut task_struct, ut: *mut u64, st: *mut u64) -> bool {
    let v=&mut (*t).vtime; if !vtime_accounting_enabled() { *ut=(*t).utime; *st=(*t).stime; return false; }
    let mut seq; loop { seq=read_seqcount_begin(&v.seqcount); *ut=(*t).utime; *st=(*t).stime;
        if v.state >= VTIME_SYS { let d=vtime_delta(v); if v.state==VTIME_SYS { *st += v.stime+d; } else { *ut += v.utime+d; } }
        if !read_seqcount_retry(&v.seqcount,seq) { return v.state >= VTIME_SYS; }
    }
}

#[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_GEN))]
pub unsafe fn task_cputime(t: *mut task_struct, ut: *mut u64, st: *mut u64) -> bool { *ut=(*t).utime; *st=(*t).stime; false }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
