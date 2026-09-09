// SPDX-License-Identifier: GPL-2.0
/* Watchdog support on powerpc systems. */

// C includes and configuration-dependent declarations are supplied by the kernel environment.

static mut wd_cpus_enabled: cpumask_t = cpumask_t::default();
static mut wd_panic_timeout_tb: u64 = 0;
static mut wd_smp_panic_timeout_tb: u64 = 0;
static mut wd_timer_period_ms: u64 = 0;
static mut wd_hrtimer: PerCpu<hrtimer> = PerCpu::uninit();
static mut wd_timer_tb: PerCpu<u64> = PerCpu::uninit();
static mut __wd_smp_lock: c_ulong = 0;
static mut __wd_reporting: c_ulong = 0;
static mut __wd_nmi_output: c_ulong = 0;
static mut wd_smp_cpus_pending: cpumask_t = cpumask_t::default();
static mut wd_smp_cpus_stuck: cpumask_t = cpumask_t::default();
static mut wd_smp_last_reset_tb: u64 = 0;

#[cfg(CONFIG_PPC_PSERIES)]
static mut wd_timeout_pct: u64 = 0;

unsafe fn wd_try_report() -> bool {
    if __wd_reporting != 0 { return false; }
    __wd_reporting = 1;
    true
}

unsafe fn wd_end_reporting() {
    smp_mb();
    WARN_ON_ONCE(__wd_reporting == 0);
    WRITE_ONCE(&mut __wd_reporting, 0);
}

unsafe fn wd_smp_lock(flags: *mut c_ulong) {
    raw_local_irq_save(flags);
    hard_irq_disable();
    while unlikely(test_and_set_bit_lock(0, &mut __wd_smp_lock)) {
        raw_local_irq_restore(*flags);
        spin_until_cond(!test_bit(0, &__wd_smp_lock));
        raw_local_irq_save(flags);
        hard_irq_disable();
    }
}

unsafe fn wd_smp_unlock(flags: *mut c_ulong) {
    clear_bit_unlock(0, &mut __wd_smp_lock);
    raw_local_irq_restore(*flags);
}

unsafe fn wd_lockup_ipi(regs: *mut pt_regs) {
    let cpu = raw_smp_processor_id();
    let tb = get_tb();
    pr_emerg!("CPU %d Hard LOCKUP\n", cpu);
    pr_emerg!("CPU %d TB:%lld, last heartbeat TB:%lld (%lldms ago)\n", cpu, tb,
        per_cpu!(wd_timer_tb, cpu), tb_to_ns(tb - per_cpu!(wd_timer_tb, cpu)) / 1000000);
    print_modules();
    print_irqtrace_events(current);
    if !regs.is_null() { show_regs(regs); } else { dump_stack(); }
    xchg(&mut __wd_nmi_output, 1);
}

unsafe fn set_cpu_stuck(cpu: c_int) -> bool {
    cpumask_set_cpu(cpu, &mut wd_smp_cpus_stuck);
    cpumask_clear_cpu(cpu, &mut wd_smp_cpus_pending);
    smp_mb();
    if cpumask_empty(&wd_smp_cpus_pending) {
        wd_smp_last_reset_tb = get_tb();
        cpumask_andnot(&mut wd_smp_cpus_pending, &wd_cpus_enabled, &wd_smp_cpus_stuck);
        return true;
    }
    false
}

unsafe fn watchdog_smp_panic(cpu: c_int) {
    static mut wd_smp_cpus_ipi: cpumask_t = cpumask_t::default();
    let mut flags: c_ulong = 0;
    wd_smp_lock(&mut flags);
    let tb = get_tb();
    let last_reset = wd_smp_last_reset_tb;
    if (tb.wrapping_sub(last_reset) as i64) < wd_smp_panic_timeout_tb as i64 { wd_smp_unlock(&mut flags); return; }
    if cpumask_test_cpu(cpu, &wd_smp_cpus_pending) { wd_smp_unlock(&mut flags); return; }
    if !wd_try_report() { wd_smp_unlock(&mut flags); return; }
    for_each_online_cpu!(c => {
        if !cpumask_test_cpu(c, &wd_smp_cpus_pending) || c == cpu { continue; }
        cpumask_set_cpu(c, &mut wd_smp_cpus_ipi);
        if set_cpu_stuck(c) { break; }
    });
    if cpumask_empty(&wd_smp_cpus_ipi) { wd_end_reporting(); wd_smp_unlock(&mut flags); return; }
    wd_smp_unlock(&mut flags);
    pr_emerg!("CPU %d detected hard LOCKUP on other CPUs %*pbl\n", cpu, cpumask_pr_args(&wd_smp_cpus_ipi));
    pr_emerg!("CPU %d TB:%lld, last SMP heartbeat TB:%lld (%lldms ago)\n", cpu, tb, last_reset, tb_to_ns(tb-last_reset)/1000000);
    if sysctl_hardlockup_all_cpu_backtrace || (hardlockup_si_mask & SYS_INFO_ALL_BT) != 0 {
        trigger_allbutcpu_cpu_backtrace(cpu);
        cpumask_clear(&mut wd_smp_cpus_ipi);
    } else {
        for_each_cpu!(c, &wd_smp_cpus_ipi, { smp_send_nmi_ipi(c, wd_lockup_ipi, 1000000); cpumask_clear_cpu(c, &mut wd_smp_cpus_ipi); });
    }
    sys_info(hardlockup_si_mask & !SYS_INFO_ALL_BT);
    if hardlockup_panic { nmi_panic(core::ptr::null_mut(), "Hard LOCKUP"); }
    wd_end_reporting();
}

unsafe fn wd_smp_clear_cpu_pending(cpu: c_int) {
    if !cpumask_test_cpu(cpu, &wd_smp_cpus_pending) {
        if unlikely(cpumask_test_cpu(cpu, &wd_smp_cpus_stuck)) {
            let regs = get_irq_regs(); let mut flags = 0;
            pr_emerg!("CPU %d became unstuck TB:%lld\n", cpu, get_tb()); print_irqtrace_events(current);
            if !regs.is_null() { show_regs(regs); } else { dump_stack(); }
            wd_smp_lock(&mut flags); cpumask_clear_cpu(cpu, &mut wd_smp_cpus_stuck); wd_smp_unlock(&mut flags);
        } else if unlikely(cpumask_empty(&wd_smp_cpus_pending)) { }
        return;
    }
    cpumask_clear_cpu(cpu, &mut wd_smp_cpus_pending); smp_mb();
    if cpumask_empty(&wd_smp_cpus_pending) {
        let mut flags = 0; wd_smp_lock(&mut flags);
        if cpumask_empty(&wd_smp_cpus_pending) { wd_smp_last_reset_tb = get_tb(); cpumask_andnot(&mut wd_smp_cpus_pending, &wd_cpus_enabled, &wd_smp_cpus_stuck); }
        wd_smp_unlock(&mut flags);
    }
}

unsafe fn watchdog_timer_interrupt(cpu: c_int) {
    let tb = get_tb(); per_cpu!(wd_timer_tb, cpu) = tb; wd_smp_clear_cpu_pending(cpu);
    if (tb.wrapping_sub(wd_smp_last_reset_tb) as i64) >= wd_smp_panic_timeout_tb as i64 { watchdog_smp_panic(cpu); }
    if __wd_nmi_output != 0 && xchg(&mut __wd_nmi_output, 0) != 0 { printk_trigger_flush(); }
}

// The remaining interrupt/timer entry points retain the C control flow and external kernel interfaces.
unsafe fn watchdog_timer_fn(hrtimer: *mut hrtimer) -> hrtimer_restart {
    let cpu = smp_processor_id();
    if watchdog_enabled & WATCHDOG_HARDLOCKUP_ENABLED == 0 || !cpumask_test_cpu(cpu, &watchdog_cpumask) { return HRTIMER_NORESTART; }
    watchdog_timer_interrupt(cpu); hrtimer_forward_now(hrtimer, ms_to_ktime(wd_timer_period_ms)); HRTIMER_RESTART
}

unsafe fn soft_nmi_interrupt(regs: *mut pt_regs) -> c_int {
    let mut flags = 0; let cpu = raw_smp_processor_id();
    WARN_ON_ONCE(!regs_irqs_disabled(regs));
    if !cpumask_test_cpu(cpu, &wd_cpus_enabled) { return 0; }
    this_cpu_inc!(irq_stat.soft_nmi_irqs);
    let tb = get_tb();
    if tb.wrapping_sub(per_cpu!(wd_timer_tb, cpu)) >= wd_panic_timeout_tb {
        wd_smp_lock(&mut flags);
        if cpumask_test_cpu(cpu, &wd_smp_cpus_stuck) { wd_smp_unlock(&mut flags); return 0; }
        if !wd_try_report() {
            wd_smp_unlock(&mut flags);
            mtspr(SPRN_DEC, 100 * tb_ticks_per_usec * 1000);
            return 0;
        }
        set_cpu_stuck(cpu); wd_smp_unlock(&mut flags);
        pr_emerg!("CPU %d self-detected hard LOCKUP @ %pS\n", cpu, (*regs).nip as *const c_void);
        pr_emerg!("CPU %d TB:%lld, last heartbeat TB:%lld (%lldms ago)\n", cpu, tb, per_cpu!(wd_timer_tb, cpu), tb_to_ns(tb-per_cpu!(wd_timer_tb,cpu))/1000000);
        print_modules(); print_irqtrace_events(current); show_regs(regs); xchg(&mut __wd_nmi_output, 1);
        if sysctl_hardlockup_all_cpu_backtrace || (hardlockup_si_mask & SYS_INFO_ALL_BT) != 0 { trigger_allbutcpu_cpu_backtrace(cpu); }
        sys_info(hardlockup_si_mask & !SYS_INFO_ALL_BT);
        if hardlockup_panic { nmi_panic(regs, "Hard LOCKUP"); }
        wd_end_reporting();
    }
    if wd_panic_timeout_tb < 0x7fffffff { mtspr(SPRN_DEC, wd_panic_timeout_tb); }
    0
}

unsafe fn start_watchdog(_arg: *mut c_void) {
    let hrtimer = this_cpu_ptr(&mut wd_hrtimer); let cpu = smp_processor_id(); let mut flags = 0;
    if cpumask_test_cpu(cpu, &wd_cpus_enabled) { WARN_ON(1); return; }
    if watchdog_enabled & WATCHDOG_HARDLOCKUP_ENABLED == 0 || !cpumask_test_cpu(cpu, &watchdog_cpumask) { return; }
    wd_smp_lock(&mut flags); cpumask_set_cpu(cpu, &mut wd_cpus_enabled);
    if cpumask_weight(&wd_cpus_enabled) == 1 { cpumask_set_cpu(cpu, &mut wd_smp_cpus_pending); wd_smp_last_reset_tb = get_tb(); }
    wd_smp_unlock(&mut flags); *this_cpu_ptr(&mut wd_timer_tb) = get_tb();
    hrtimer_setup(hrtimer, watchdog_timer_fn, CLOCK_MONOTONIC, HRTIMER_MODE_REL);
    hrtimer_start(hrtimer, ms_to_ktime(wd_timer_period_ms), HRTIMER_MODE_REL_PINNED);
}

unsafe fn start_watchdog_on_cpu(cpu: c_uint) -> c_int { smp_call_function_single(cpu, start_watchdog, core::ptr::null_mut(), true) }

unsafe fn stop_watchdog(_arg: *mut c_void) {
    let hrtimer = this_cpu_ptr(&mut wd_hrtimer); let cpu = smp_processor_id(); let mut flags = 0;
    if !cpumask_test_cpu(cpu, &wd_cpus_enabled) { return; }
    hrtimer_cancel(hrtimer); wd_smp_lock(&mut flags); cpumask_clear_cpu(cpu, &mut wd_cpus_enabled); wd_smp_unlock(&mut flags); wd_smp_clear_cpu_pending(cpu);
}

unsafe fn stop_watchdog_on_cpu(cpu: c_uint) -> c_int { smp_call_function_single(cpu, stop_watchdog, core::ptr::null_mut(), true) }

pub unsafe fn arch_touch_nmi_watchdog() {
    let ticks = tb_ticks_per_usec * wd_timer_period_ms * 1000; let cpu = smp_processor_id();
    if !cpumask_test_cpu(cpu, &watchdog_cpumask) { return; }
    let tb = get_tb(); if tb.wrapping_sub(per_cpu!(wd_timer_tb, cpu)) >= ticks { per_cpu!(wd_timer_tb, cpu)=tb; wd_smp_clear_cpu_pending(cpu); }
}
EXPORT_SYMBOL!(arch_touch_nmi_watchdog);

unsafe fn watchdog_calc_timeouts() {
    let mut threshold = watchdog_thresh;
    #[cfg(CONFIG_PPC_PSERIES)] { threshold += (READ_ONCE(wd_timeout_pct) * threshold) / 100; }
    wd_panic_timeout_tb = threshold * ppc_tb_freq; wd_smp_panic_timeout_tb = wd_panic_timeout_tb * 3 / 2; wd_timer_period_ms = watchdog_thresh * 1000 * 2 / 5;
}

pub unsafe fn watchdog_hardlockup_stop() { for_each_cpu!(cpu, &wd_cpus_enabled, { stop_watchdog_on_cpu(cpu); }); }
pub unsafe fn watchdog_hardlockup_start() { watchdog_calc_timeouts(); for_each_cpu_and!(cpu, cpu_online_mask, &watchdog_cpumask, { start_watchdog_on_cpu(cpu); }); }

pub unsafe fn watchdog_hardlockup_probe() -> c_int {
    let err = cpuhp_setup_state_nocalls(CPUHP_AP_ONLINE_DYN, "powerpc/watchdog:online", start_watchdog_on_cpu, stop_watchdog_on_cpu);
    if err < 0 { pr_warn!("could not be initialized"); return err; } 0
}

#[cfg(CONFIG_PPC_PSERIES)]
pub unsafe fn watchdog_hardlockup_set_timeout_pct(pct: u64) {
    pr_info!("Set the NMI watchdog timeout factor to %llu%%\n", pct); WRITE_ONCE(&mut wd_timeout_pct, pct); lockup_detector_reconfigure();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
