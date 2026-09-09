/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Support Power Management
 *
 * Copyright 2014-2015 Freescale Semiconductor Inc.
 */

pub const E500_PM_PH10: i32 = 1;
pub const E500_PM_PH15: i32 = 2;
pub const E500_PM_PH20: i32 = 3;
pub const E500_PM_PH30: i32 = 4;
pub const E500_PM_DOZE: i32 = E500_PM_PH10;
pub const E500_PM_NAP: i32 = E500_PM_PH15;

pub const PLAT_PM_SLEEP: i32 = 20;
pub const PLAT_PM_LPM20: i32 = 30;

pub const FSL_PM_SLEEP: i32 = 1 << 0;
pub const FSL_PM_DEEP_SLEEP: i32 = 1 << 1;

#[repr(C)]
pub struct fsl_pm_ops {
    /* mask pending interrupts to the RCPM from MPIC */
    pub irq_mask: Option<unsafe extern "C" fn(cpu: i32)>,

    /* unmask pending interrupts to the RCPM from MPIC */
    pub irq_unmask: Option<unsafe extern "C" fn(cpu: i32)>,
    pub cpu_enter_state: Option<unsafe extern "C" fn(cpu: i32, state: i32)>,
    pub cpu_exit_state: Option<unsafe extern "C" fn(cpu: i32, state: i32)>,
    pub cpu_up_prepare: Option<unsafe extern "C" fn(cpu: i32)>,
    pub cpu_die: Option<unsafe extern "C" fn(cpu: i32)>,
    pub plat_enter_sleep: Option<unsafe extern "C" fn() -> i32>,
    pub freeze_time_base: Option<unsafe extern "C" fn(freeze: bool)>,

    /* keep the power of IP blocks during sleep/deep sleep */
    pub set_ip_power: Option<unsafe extern "C" fn(enable: bool, mask: u32)>,

    /* get platform supported power management modes */
    pub get_pm_modes: Option<unsafe extern "C" fn() -> u32>,
}

extern "C" {
    pub static qoriq_pm_ops: *const fsl_pm_ops;

    pub fn fsl_rcpm_init() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
