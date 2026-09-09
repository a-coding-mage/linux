/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/kernel_stat.h. C preprocessor configuration conditions
// are preserved as Rust cfg conditions where their intent is file-local.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum cpu_usage_stat {
    CPUTIME_USER,
    CPUTIME_NICE,
    CPUTIME_SYSTEM,
    CPUTIME_SOFTIRQ,
    CPUTIME_IRQ,
    CPUTIME_IDLE,
    CPUTIME_IOWAIT,
    CPUTIME_STEAL,
    CPUTIME_GUEST,
    CPUTIME_GUEST_NICE,
    #[cfg(feature = "CONFIG_SCHED_CORE")]
    CPUTIME_FORCEIDLE,
    NR_STATS,
}

#[repr(C)]
pub struct kernel_cpustat {
    #[cfg(feature = "CONFIG_NO_HZ_COMMON")]
    pub idle_dyntick: bool,
    #[cfg(feature = "CONFIG_NO_HZ_COMMON")]
    pub idle_elapse: bool,
    #[cfg(feature = "CONFIG_NO_HZ_COMMON")]
    pub idle_sleeptime_seq: seqcount_t,
    #[cfg(feature = "CONFIG_NO_HZ_COMMON")]
    pub idle_entrytime: u64,
    #[cfg(feature = "CONFIG_NO_HZ_COMMON")]
    pub idle_stealtime: [u64; 2],
    pub cpustat: [u64; NR_STATS as usize],
}

#[repr(C)]
pub struct kernel_stat {
    pub irqs_sum: ::core::ffi::c_ulong,
    pub softirqs: [u32; NR_SOFTIRQS as usize],
}

// Must have preemption disabled for this to be meaningful.
// C macros: kstat_this_cpu, kcpustat_this_cpu, kstat_cpu(cpu), kcpustat_cpu(cpu).

extern "C" {
    pub fn nr_context_switches_cpu(cpu: i32) -> u64;
    pub fn nr_context_switches() -> u64;
    pub fn kstat_irqs_cpu(irq: u32, cpu: i32) -> u32;
    pub fn kstat_incr_irq_this_cpu(irq: u32);
    pub fn kstat_irqs_usr(irq: u32) -> u32;
    pub fn get_cpu_idle_time_us(cpu: i32, last_update_time: *mut u64) -> u64;
    pub fn get_cpu_iowait_time_us(cpu: i32, last_update_time: *mut u64) -> u64;
    pub fn account_user_time(tsk: *mut task_struct, cputime: u64);
    pub fn account_guest_time(tsk: *mut task_struct, cputime: u64);
    pub fn account_system_time(tsk: *mut task_struct, user: i32, cputime: u64);
    pub fn account_system_index_time(tsk: *mut task_struct, cputime: u64, usage: cpu_usage_stat);
    pub fn account_steal_time(cputime: u64);
    pub fn account_idle_time(cputime: u64);
}

pub unsafe fn kstat_incr_softirqs_this_cpu(irq: u32) {
    __this_cpu_inc(kstat.softirqs[irq as usize]);
}

pub unsafe fn kstat_softirqs_cpu(irq: u32, cpu: i32) -> u32 {
    kstat_cpu(cpu).softirqs[irq as usize]
}

pub unsafe fn kstat_cpu_softirqs_sum(cpu: i32) -> u32 {
    let mut sum: u32 = 0;
    let mut i: i32 = 0;
    while i < NR_SOFTIRQS {
        sum = sum.wrapping_add(kstat_softirqs_cpu(i as u32, cpu));
        i += 1;
    }
    sum
}

#[cfg(feature = "CONFIG_GENERIC_IRQ_STAT_SNAPSHOT")]
extern "C" {
    pub fn kstat_snapshot_irqs();
    pub fn kstat_get_irq_since_snapshot(irq: u32) -> u32;
}
#[cfg(not(feature = "CONFIG_GENERIC_IRQ_STAT_SNAPSHOT"))]
pub unsafe fn kstat_snapshot_irqs() {}
#[cfg(not(feature = "CONFIG_GENERIC_IRQ_STAT_SNAPSHOT"))]
pub unsafe fn kstat_get_irq_since_snapshot(_irq: u32) -> u32 { 0 }

pub unsafe fn kstat_cpu_irqs_sum(cpu: u32) -> ::core::ffi::c_ulong {
    kstat_cpu(cpu as i32).irqs_sum
}

#[cfg(all(feature = "CONFIG_NO_HZ_COMMON", feature = "CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE"))]
pub unsafe fn kcpustat_dyntick_start(_now: u64) {}
#[cfg(all(feature = "CONFIG_NO_HZ_COMMON", feature = "CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE"))]
pub unsafe fn kcpustat_dyntick_stop(_now: u64) {}
#[cfg(all(feature = "CONFIG_NO_HZ_COMMON", feature = "CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE"))]
pub unsafe fn kcpustat_irq_enter(_now: u64) {}
#[cfg(all(feature = "CONFIG_NO_HZ_COMMON", feature = "CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE"))]
pub unsafe fn kcpustat_irq_exit(_now: u64) {}
#[cfg(all(feature = "CONFIG_NO_HZ_COMMON", feature = "CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE"))]
pub unsafe fn kcpustat_idle_dyntick() -> bool { false }

#[cfg(feature = "CONFIG_NO_HZ_COMMON")]
extern "C" {
    pub fn kcpustat_field_idle(cpu: i32) -> u64;
    pub fn kcpustat_field_iowait(cpu: i32) -> u64;
}

#[cfg(not(feature = "CONFIG_NO_HZ_COMMON"))]
pub unsafe fn kcpustat_field_idle(cpu: i32) -> u64 { kcpustat_cpu(cpu).cpustat[CPUTIME_IDLE as usize] }
#[cfg(not(feature = "CONFIG_NO_HZ_COMMON"))]
pub unsafe fn kcpustat_field_iowait(cpu: i32) -> u64 { kcpustat_cpu(cpu).cpustat[CPUTIME_IOWAIT as usize] }
#[cfg(not(feature = "CONFIG_NO_HZ_COMMON"))]
pub unsafe fn kcpustat_idle_dyntick() -> bool { false }

pub unsafe fn kcpustat_field_default(usage: cpu_usage_stat, cpu: i32) -> u64 {
    if usage == CPUTIME_IDLE { return kcpustat_field_idle(cpu); }
    if usage == CPUTIME_IOWAIT { return kcpustat_field_iowait(cpu); }
    kcpustat_cpu(cpu).cpustat[usage as usize]
}

pub unsafe fn kcpustat_cpu_fetch_default(dst: *mut kernel_cpustat, cpu: i32) {
    *dst = kcpustat_cpu(cpu);
    (*dst).cpustat[CPUTIME_IDLE as usize] = kcpustat_field_idle(cpu);
    (*dst).cpustat[CPUTIME_IOWAIT as usize] = kcpustat_field_iowait(cpu);
}

#[cfg(feature = "CONFIG_VIRT_CPU_ACCOUNTING_GEN")]
extern "C" {
    pub fn kcpustat_field(usage: cpu_usage_stat, cpu: i32) -> u64;
    pub fn kcpustat_cpu_fetch(dst: *mut kernel_cpustat, cpu: i32);
}
#[cfg(not(feature = "CONFIG_VIRT_CPU_ACCOUNTING_GEN"))]
pub unsafe fn kcpustat_field(usage: cpu_usage_stat, cpu: i32) -> u64 { kcpustat_field_default(usage, cpu) }
#[cfg(not(feature = "CONFIG_VIRT_CPU_ACCOUNTING_GEN"))]
pub unsafe fn kcpustat_cpu_fetch(dst: *mut kernel_cpustat, cpu: i32) { kcpustat_cpu_fetch_default(dst, cpu) }

#[cfg(feature = "CONFIG_VIRT_CPU_ACCOUNTING_NATIVE")]
pub unsafe fn account_process_tick(tsk: *mut task_struct, _user: i32) {
    if !kcpustat_idle_dyntick() { vtime_flush(tsk); }
}
#[cfg(not(feature = "CONFIG_VIRT_CPU_ACCOUNTING_NATIVE"))]
extern "C" { pub fn account_process_tick(tsk: *mut task_struct, user: i32); }

#[cfg(feature = "CONFIG_SCHED_CORE")]
extern "C" { pub fn __account_forceidle_time(tsk: *mut task_struct, delta: u64); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
