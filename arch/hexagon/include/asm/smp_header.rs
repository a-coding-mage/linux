/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * SMP definitions for the Hexagon architecture
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

// Dependency supplied externally: linux/cpumask.h

/// Equivalent of the C macro `raw_smp_processor_id()`.
#[macro_export]
macro_rules! raw_smp_processor_id {
    () => {
        current_thread_info().cpu
    };
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ipi_message_type {
    IPI_NOP = 0,
    IPI_RESCHEDULE = 1,
    IPI_CALL_FUNC,
    IPI_CPU_STOP,
    IPI_TIMER,
}

unsafe extern "C" {
    pub fn send_ipi(cpumask: *const cpumask, msg: ipi_message_type);
    pub fn smp_start_cpus();
    pub fn arch_send_call_function_single_ipi(cpu: i32);
    pub fn arch_send_call_function_ipi_mask(mask: *const cpumask);
    pub fn smp_vm_unmask_irq(info: *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
