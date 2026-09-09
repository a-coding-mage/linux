/* SPDX-License-Identifier: GPL-2.0 */
/* smp.h: Sparc64 specific SMP stuff.
 *
 * Copyright (C) 1996, 2008 David S. Miller (davem@davemloft.net)
 */

/* Dependencies from linux/threads.h, asm/asi.h, asm/starfire.h,
 * asm/spitfire.h, linux/cpumask.h, linux/cache.h, linux/bitops.h,
 * linux/atomic.h, and asm/percpu.h are supplied externally.
 */

#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub static mut cpu_sibling_map: cpumask_t;
    pub static mut cpu_core_map: [cpumask_t; NR_CPUS];

    pub fn smp_init_cpu_poke();
    pub fn scheduler_poke();

    pub fn arch_send_call_function_single_ipi(cpu: i32);
    pub fn arch_send_call_function_ipi_mask(mask: *const cpumask_t);

    pub fn hard_smp_processor_id() -> i32;

    pub fn smp_fill_in_sib_core_maps();
    pub fn cpu_play_dead() -> !;

    pub fn smp_fetch_global_regs();
    pub fn smp_fetch_global_pmu();

    pub fn smp_bogo(file: *mut seq_file);
    pub fn smp_info(file: *mut seq_file);

    pub fn smp_callin();
    pub fn cpu_panic();
    pub fn smp_synchronize_tick_client();
    pub fn smp_capture();
    pub fn smp_release();

    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    pub fn __cpu_disable() -> i32;
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    pub fn __cpu_die(cpu: u32);
}

#[cfg(feature = "CONFIG_SMP")]
#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! raw_smp_processor_id {
    () => {
        current_thread_info().cpu
    };
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline(always)]
pub const fn hard_smp_processor_id() -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[macro_export]
macro_rules! smp_fill_in_sib_core_maps {
    () => {{ }};
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[macro_export]
macro_rules! smp_fetch_global_regs {
    () => {{ }};
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[macro_export]
macro_rules! smp_fetch_global_pmu {
    () => {{ }};
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[macro_export]
macro_rules! smp_init_cpu_poke {
    () => {{ }};
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[macro_export]
macro_rules! scheduler_poke {
    () => {{ }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
