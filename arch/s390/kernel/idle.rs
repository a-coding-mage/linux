// SPDX-License-Identifier: GPL-2.0
/*
 * Idle functions for s390.
 *
 * Copyright IBM Corp. 2014
 *
 * Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
 */

// Kernel and architecture declarations supplied by the surrounding build.

pub static mut s390_idle: s390_idle_data = s390_idle_data::default();

#[inline(always)]
unsafe fn __account_idle_time_irq() {
    let idle: *mut s390_idle_data = this_cpu_ptr(&raw mut s390_idle);
    let idle_time: c_ulong = (*idle).clock_idle_exit.tod - (*idle).clock_idle_enter.tod;

    account_idle_time(cputime_to_nsecs(idle_time));
}

#[inline(always)]
unsafe fn __account_idle_time_setup() {
    let idle: *mut s390_idle_data = this_cpu_ptr(&raw mut s390_idle);

    store_tod_clock_ext(&mut (*idle).clock_idle_enter);
    (*idle).timer_idle_enter = get_cpu_timer();
    (*idle).clock_idle_exit = (*idle).clock_idle_enter;
}

#[cfg(CONFIG_NO_HZ_COMMON)]
unsafe fn arch_cpu_in_idle_time(cpu: c_int) -> u64 {
    let idle: *mut s390_idle_data = &mut *per_cpu(&raw mut s390_idle, cpu);
    let mut now: tod_clock = tod_clock::default();
    let idle_time: u64;

    if !(*idle).in_idle {
        return 0;
    }
    store_tod_clock_ext(&mut now);
    if tod_after((*idle).clock_idle_exit.tod, (*idle).clock_idle_enter.tod) {
        idle_time = (*idle).clock_idle_exit.tod - (*idle).clock_idle_enter.tod;
    } else {
        idle_time = now.tod - (*idle).clock_idle_enter.tod;
    }
    cputime_to_nsecs(idle_time)
}

#[cfg(CONFIG_NO_HZ_COMMON)]
unsafe fn arch_cpu_idle_time(cpu: c_int, idx: cpu_usage_stat, compute_delta: bool) -> u64 {
    let kc: *mut kernel_cpustat = &mut *kcpustat_cpu(cpu);
    let cpustat: *mut u64 = (*kc).cpustat.as_mut_ptr();
    let mut seq: c_uint;
    let mut idle_time: u64;

    /* The open coded seqcount writer in entry.S relies on the raw counting
     * mechanism without any writer protection. */
    loop {
        seq = read_seqcount_begin(&(*kc).idle_sleeptime_seq);
        idle_time = *cpustat.add(idx as usize);
        if compute_delta {
            idle_time += arch_cpu_in_idle_time(cpu);
        }
        if !read_seqcount_retry(&(*kc).idle_sleeptime_seq, seq) {
            break;
        }
    }
    idle_time
}

#[cfg(CONFIG_NO_HZ_COMMON)]
pub unsafe fn arch_kcpustat_field_idle(cpu: c_int) -> u64 {
    arch_cpu_idle_time(cpu, CPUTIME_IDLE, !nr_iowait_cpu(cpu))
}

#[cfg(CONFIG_NO_HZ_COMMON)]
pub unsafe fn arch_kcpustat_field_iowait(cpu: c_int) -> u64 {
    arch_cpu_idle_time(cpu, CPUTIME_IOWAIT, nr_iowait_cpu(cpu))
}

pub unsafe fn account_idle_time_irq() {
    let idle: *mut s390_idle_data = this_cpu_ptr(&raw mut s390_idle);
    let kc: *mut kernel_cpustat = kcpustat_this_cpu;

    #[cfg(CONFIG_NO_HZ_COMMON)]
    write_seqcount_begin(&mut (*kc).idle_sleeptime_seq);
    (*idle).in_idle = false;
    __account_idle_time_irq();
    #[cfg(CONFIG_NO_HZ_COMMON)]
    write_seqcount_end(&mut (*kc).idle_sleeptime_seq);
}

#[inline(always)]
unsafe fn account_idle_time_setup() {
    #[cfg(CONFIG_NO_HZ_COMMON)]
    {
        let idle: *mut s390_idle_data = this_cpu_ptr(&raw mut s390_idle);
        let kc: *mut kernel_cpustat = kcpustat_this_cpu;

        raw_write_seqcount_begin(&mut (*kc).idle_sleeptime_seq);
        (*idle).in_idle = true;
        __account_idle_time_setup();
        raw_write_seqcount_end(&mut (*kc).idle_sleeptime_seq);
    }
    #[cfg(not(CONFIG_NO_HZ_COMMON))]
    {
        __account_idle_time_setup();
    }
}

pub unsafe fn arch_cpu_idle() {
    let idle: *mut s390_idle_data = this_cpu_ptr(&raw mut s390_idle);
    let psw_mask: c_ulong;

    /* Wait for external, I/O or machine check interrupt. */
    psw_mask = PSW_KERNEL_BITS | PSW_MASK_WAIT |
        PSW_MASK_IO | PSW_MASK_EXT | PSW_MASK_MCHECK;
    set_cpu_flag(CIF_ENABLED_WAIT);
    if smp_cpu_mtid != 0 {
        stcctm(MT_DIAG, smp_cpu_mtid, &mut (*idle).mt_cycles_enter as *mut _ as *mut u64);
    }
    account_idle_time_setup();
    bpon();
    __load_psw_mask(psw_mask);
}

pub unsafe fn arch_cpu_idle_enter() {}

pub unsafe fn arch_cpu_idle_exit() {}

pub unsafe fn arch_cpu_idle_dead() -> ! {
    cpu_die();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
