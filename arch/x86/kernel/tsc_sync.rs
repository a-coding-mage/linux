// SPDX-License-Identifier: GPL-2.0
/*
 * check TSC synchronization.
 *
 * Copyright (C) 2006, Red Hat, Inc., Ingo Molnar
 *
 * We check whether all boot CPUs have their TSC's synchronized,
 * print a warning if not and turn off the TSC clock-source.
 *
 * The warp-check is point-to-point between two CPUs, the CPU
 * initiating the bootup is the 'source CPU', the freshly booting
 * CPU is the 'target CPU'.
 *
 * Only two CPUs may participate - they can enter in any order.
 * ( The serial nature of the boot logic and the CPU hotplug lock
 *   protects against more than 2 CPUs entering this code. )
 */

#[repr(C)]
struct TscAdjust {
    bootval: i64,
    adjusted: i64,
    nextcheck: libc::c_ulong,
    warned: bool,
}

static mut TSC_ADJUST: TscAdjust = TscAdjust { bootval: 0, adjusted: 0, nextcheck: 0, warned: false };
static mut TSC_SYNC_CHECK_TIMER: TimerList = TimerList { _private: [] };

static mut TSC_ASYNC_RESETS: bool = false;

pub unsafe fn mark_tsc_async_resets(reason: *mut libc::c_char) {
    if TSC_ASYNC_RESETS { return; }
    TSC_ASYNC_RESETS = true;
    pr_info!("tsc: Marking TSC async resets true due to %s\n", reason);
}

pub unsafe fn tsc_verify_tsc_adjust(resume: bool) {
    let adj = this_cpu_ptr(&raw mut TSC_ADJUST);
    let mut curval: i64 = 0;
    if !boot_cpu_has(X86_FEATURE_TSC_ADJUST) || check_tsc_unstable() { return; }
    if !resume && time_before(jiffies, (*adj).nextcheck) { return; }
    (*adj).nextcheck = jiffies + HZ;
    rdmsrq!(MSR_IA32_TSC_ADJUST, curval);
    if (*adj).adjusted == curval { return; }
    wrmsrq!(MSR_IA32_TSC_ADJUST, (*adj).adjusted);
    if !(*adj).warned || resume {
        pr_warn!(FW_BUG "TSC ADJUST differs: CPU%u %lld --> %lld. Restoring\n", smp_processor_id(), (*adj).adjusted, curval);
        (*adj).warned = true;
    }
}

const SYNC_CHECK_INTERVAL: libc::c_ulong = HZ * 600;

unsafe fn tsc_sync_check_timer_fn(_unused: *mut TimerList) {
    tsc_verify_tsc_adjust(false);
    let mut next_cpu = cpumask_next(raw_smp_processor_id(), cpu_online_mask);
    if next_cpu >= nr_cpu_ids { next_cpu = cpumask_first(cpu_online_mask); }
    TSC_SYNC_CHECK_TIMER.expires += SYNC_CHECK_INTERVAL;
    add_timer_on(&raw mut TSC_SYNC_CHECK_TIMER, next_cpu);
}

unsafe fn start_sync_check_timer() -> libc::c_int {
    if !cpu_feature_enabled(X86_FEATURE_TSC_ADJUST) || tsc_clocksource_reliable { return 0; }
    timer_setup(&raw mut TSC_SYNC_CHECK_TIMER, tsc_sync_check_timer_fn, 0);
    TSC_SYNC_CHECK_TIMER.expires = jiffies + SYNC_CHECK_INTERVAL;
    add_timer(&raw mut TSC_SYNC_CHECK_TIMER);
    0
}

unsafe fn tsc_sanitize_first_cpu(cur: *mut TscAdjust, mut bootval: i64, cpu: libc::c_uint, bootcpu: bool) {
    if bootcpu && bootval != 0 {
        if likely(!TSC_ASYNC_RESETS) {
            pr_warn!(FW_BUG "TSC ADJUST: CPU%u: %lld force to 0\n", cpu, bootval);
            wrmsrq!(MSR_IA32_TSC_ADJUST, 0);
            bootval = 0;
        } else { pr_info!("TSC ADJUST: CPU%u: %lld NOT forced to 0\n", cpu, bootval); }
    }
    (*cur).adjusted = bootval;
}

pub unsafe fn tsc_store_and_check_tsc_adjust(bootcpu: bool) -> bool {
    let cur = this_cpu_ptr(&raw mut TSC_ADJUST);
    let cpu = smp_processor_id();
    let mut bootval: i64 = 0;
    if !boot_cpu_has(X86_FEATURE_TSC_ADJUST) { return false; }
    rdmsrq!(MSR_IA32_TSC_ADJUST, bootval);
    (*cur).bootval = bootval;
    (*cur).nextcheck = jiffies + HZ;
    (*cur).warned = false;
    (*cur).adjusted = bootval;
    let mask = topology_core_cpumask(cpu);
    let refcpu = if !mask.is_null() { cpumask_any_but(mask, cpu) } else { nr_cpu_ids };
    if refcpu >= nr_cpu_ids {
        tsc_sanitize_first_cpu(cur, bootval, cpu, bootcpu);
        return false;
    }
    let reference = per_cpu_ptr(&raw mut TSC_ADJUST, refcpu);
    if bootval != (*reference).bootval { printk_once!(FW_BUG "TSC ADJUST differs within socket(s), fixing all errors\n"); }
    if bootval != (*reference).adjusted {
        (*cur).adjusted = (*reference).adjusted;
        wrmsrq!(MSR_IA32_TSC_ADJUST, (*reference).adjusted);
    }
    true
}

static mut START_COUNT: AtomicT = AtomicT::new(0);
static mut STOP_COUNT: AtomicT = AtomicT::new(0);
static mut TEST_RUNS: AtomicT = AtomicT::new(0);
static mut SYNC_LOCK: ArchSpinlockT = ARCH_SPIN_LOCK_UNLOCKED;
static mut LAST_TSC: CyclesT = 0;
static mut MAX_WARP: CyclesT = 0;
static mut NR_WARPS: libc::c_int = 0;
static mut RANDOM_WARPS: libc::c_int = 0;

unsafe fn check_tsc_warp(timeout: libc::c_uint) -> CyclesT {
    let start = rdtsc_ordered();
    let end = start + (tsc_khz * timeout as CyclesT);
    let mut now = start;
    let mut cur_max_warp: CyclesT = 0;
    let mut cur_warps = 0;
    let mut i: libc::c_int = 0;
    loop {
        arch_spin_lock(&raw mut SYNC_LOCK);
        let prev = LAST_TSC; now = rdtsc_ordered(); LAST_TSC = now;
        arch_spin_unlock(&raw mut SYNC_LOCK);
        if !(i & 7) != 0 {
            if now > end || i > 10000000 { break; }
            cpu_relax(); touch_nmi_watchdog();
        }
        if prev > now {
            arch_spin_lock(&raw mut SYNC_LOCK);
            MAX_WARP = max(MAX_WARP, prev - now); cur_max_warp = MAX_WARP;
            if cur_warps != NR_WARPS { RANDOM_WARPS += 1; }
            NR_WARPS += 1; cur_warps = NR_WARPS;
            arch_spin_unlock(&raw mut SYNC_LOCK);
        }
        i += 1;
    }
    WARN!(now - start == 0, "Warning: zero tsc calibration delta: %Ld [max: %Ld]\n", now - start, end - start);
    cur_max_warp
}

#[inline]
unsafe fn loop_timeout(cpu: libc::c_int) -> libc::c_uint { if cpumask_weight(topology_core_cpumask(cpu)) > 1 { 2 } else { 20 } }

unsafe fn tsc_sync_mark_tsc_unstable(_work: *mut WorkStruct) { mark_tsc_unstable("check_tsc_sync_source failed"); }
static mut TSC_SYNC_WORK: WorkStruct = WorkStruct { _private: [] };

unsafe fn check_tsc_sync_source(__cpu: *mut libc::c_void) {
    let cpu = __cpu as libc::c_ulong as libc::c_uint;
    let cpus = 2;
    atomic_set(&raw mut TEST_RUNS, if boot_cpu_has(X86_FEATURE_TSC_ADJUST) { 3 } else { 1 });
    'retry: loop {
        while atomic_read(&raw mut START_COUNT) != cpus - 1 { cpu_relax(); }
        atomic_inc(&raw mut START_COUNT); check_tsc_warp(loop_timeout(cpu as libc::c_int));
        while atomic_read(&raw mut STOP_COUNT) != cpus - 1 { cpu_relax(); }
        if NR_WARPS == 0 { atomic_set(&raw mut TEST_RUNS, 0); pr_debug!("TSC synchronization [CPU#%d -> CPU#%u]: passed\n", smp_processor_id(), cpu); }
        else if atomic_dec_and_test(&raw mut TEST_RUNS) || RANDOM_WARPS != 0 {
            atomic_set(&raw mut TEST_RUNS, 0); pr_warn!("TSC synchronization [CPU#%d -> CPU#%u]:\n", smp_processor_id(), cpu); pr_warn!("Measured %Ld cycles TSC warp between CPUs, turning off TSC clock.\n", MAX_WARP); if RANDOM_WARPS != 0 { pr_warn!("TSC warped randomly between CPUs\n"); } schedule_work(&raw mut TSC_SYNC_WORK);
        }
        atomic_set(&raw mut START_COUNT, 0); RANDOM_WARPS = 0; NR_WARPS = 0; MAX_WARP = 0; LAST_TSC = 0; atomic_inc(&raw mut STOP_COUNT);
        if atomic_read(&raw mut TEST_RUNS) <= 0 { break 'retry; }
    }
}

pub unsafe fn check_tsc_sync_target() {
    let cur = this_cpu_ptr(&raw mut TSC_ADJUST); let cpu = smp_processor_id(); let cpus = 2;
    if unsynchronized_tsc() || tsc_store_and_check_tsc_adjust(false) || tsc_clocksource_reliable { return; }
    smp_call_function_single(cpumask_first(cpu_online_mask), check_tsc_sync_source, cpu as libc::c_ulong as *mut libc::c_void, 0);
    loop {
        atomic_inc(&raw mut START_COUNT); while atomic_read(&raw mut START_COUNT) != cpus { cpu_relax(); }
        let mut cur_max_warp = check_tsc_warp(loop_timeout(cpu)); let gbl_max_warp = MAX_WARP;
        atomic_inc(&raw mut STOP_COUNT); while atomic_read(&raw mut STOP_COUNT) != cpus { cpu_relax(); }
        atomic_set(&raw mut STOP_COUNT, 0); if atomic_read(&raw mut TEST_RUNS) == 0 { return; }
        if cur_max_warp == 0 { cur_max_warp = 0u64.wrapping_sub(gbl_max_warp); }
        (*cur).adjusted = (*cur).adjusted.wrapping_add(cur_max_warp as i64);
        pr_warn!("TSC ADJUST compensate: CPU%u observed %lld warp. Adjust: %lld\n", cpu, cur_max_warp, (*cur).adjusted);
        wrmsrq!(MSR_IA32_TSC_ADJUST, (*cur).adjusted);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
