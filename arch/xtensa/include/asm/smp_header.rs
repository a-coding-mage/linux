/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2013 Tensilica Inc.
 */

// The declarations below are present when CONFIG_SMP is enabled.
#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! raw_smp_processor_id {
    () => {
        current_thread_info().cpu
    };
}

#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! cpu_logical_map {
    ($cpu:expr) => {
        $cpu
    };
}

#[cfg(feature = "CONFIG_SMP")]
#[repr(C)]
pub struct start_info {
    pub stack: core::ffi::c_ulong,
}

#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub static mut start_info: start_info;

    pub fn arch_send_call_function_ipi_mask(mask: *const cpumask);
    pub fn arch_send_call_function_single_ipi(cpu: core::ffi::c_int);

    pub fn secondary_start_kernel();
    pub fn smp_init_cpus();
    pub fn secondary_init_irq();
    pub fn ipi_init();
    pub fn show_ipi_list(p: *mut seq_file, prec: core::ffi::c_int);
}

#[cfg(feature = "CONFIG_SMP")]
pub enum cpumask {}

#[cfg(feature = "CONFIG_SMP")]
pub enum seq_file {}

#[cfg(all(feature = "CONFIG_SMP", feature = "CONFIG_HOTPLUG_CPU"))]
extern "C" {
    pub fn __cpu_die(cpu: core::ffi::c_uint);
    pub fn __cpu_disable() -> core::ffi::c_int;
    pub fn cpu_die() -> !;
    pub fn cpu_restart();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
