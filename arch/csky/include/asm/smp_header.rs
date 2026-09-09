/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
// #include <linux/cpumask.h>
// #include <linux/irqreturn.h>
// #include <linux/threads.h>

// These types and functions are supplied by the corresponding translated
// Linux dependencies.
#[repr(C)]
pub enum cpumask {}

#[repr(C)]
pub struct thread_info {
    pub cpu: i32,
}

#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub fn setup_smp();

    pub fn setup_smp_ipi();

    pub fn arch_send_call_function_ipi_mask(mask: *mut cpumask);

    pub fn arch_send_call_function_single_ipi(cpu: i32);

    pub fn set_send_ipi(func: Option<unsafe extern "C" fn(mask: *const cpumask)>, irq: i32);

    pub fn __cpu_disable() -> i32;

    pub fn current_thread_info() -> *mut thread_info;
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn raw_smp_processor_id() -> i32 {
    (*current_thread_info()).cpu
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn __cpu_die(_cpu: u32) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
