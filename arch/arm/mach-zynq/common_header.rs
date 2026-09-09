/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * This file contains common function prototypes to avoid externs
 * in the c files.
 *
 *  Copyright (C) 2011 Xilinx
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    pub fn zynq_early_slcr_init() -> c_int;
    pub fn zynq_slcr_cpu_stop(cpu: c_int);
    pub fn zynq_slcr_cpu_start(cpu: c_int);
    pub fn zynq_slcr_cpu_state_read(cpu: c_int) -> bool;
    pub fn zynq_slcr_cpu_state_write(cpu: c_int, die: bool);
    pub fn zynq_slcr_get_device_id() -> c_uint;

    /* The following declarations are present when CONFIG_SMP is enabled. */
    #[cfg(feature = "CONFIG_SMP")]
    pub static mut zynq_secondary_trampoline: c_char;
    #[cfg(feature = "CONFIG_SMP")]
    pub static mut zynq_secondary_trampoline_jump: c_char;
    #[cfg(feature = "CONFIG_SMP")]
    pub static mut zynq_secondary_trampoline_end: c_char;
    #[cfg(feature = "CONFIG_SMP")]
    pub fn zynq_cpun_start(address: c_uint, cpu: c_int) -> c_int;
    #[cfg(feature = "CONFIG_SMP")]
    pub static zynq_smp_ops: smp_operations;

    pub static mut zynq_scu_base: *mut c_void;

    pub fn zynq_pm_late_init();
}

pub enum smp_operations {}

#[inline]
pub unsafe fn zynq_core_pm_init() {
    /* A9 clock gating */
    core::arch::asm!(
        "mrc  p15, 0, r12, c15, c0, 0",
        "orr  r12, r12, #1",
        "mcr  p15, 0, r12, c15, c0, 0",
        out("r12") _,
        options(nostack, preserves_flags)
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
