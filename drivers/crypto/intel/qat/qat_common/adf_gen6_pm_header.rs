/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2025 Intel Corporation */

// Dependencies: linux/bits.h and linux/time.h provide BIT, GENMASK, and
// USEC_PER_SEC in the original header.

#[repr(C)]
pub struct adf_accel_dev {
    _private: [u8; 0],
}

/* Power management */
pub const ADF_GEN6_PM_POLL_DELAY_US: u32 = 20;
pub const ADF_GEN6_PM_POLL_TIMEOUT_US: u32 = 1_000_000;
pub const ADF_GEN6_PM_STATUS: u32 = 0x50A00C;
pub const ADF_GEN6_PM_INTERRUPT: u32 = 0x50A028;

/* Power management source in ERRSOU2 and ERRMSK2 */
pub const ADF_GEN6_PM_SOU: u32 = 1u32 << 18;

/* cpm_pm_interrupt bitfields */
pub const ADF_GEN6_PM_DRV_ACTIVE: u32 = 1u32 << 20;

pub const ADF_GEN6_PM_DEFAULT_IDLE_FILTER: u32 = 0x6;

/* cpm_pm_status bitfields */
pub const ADF_GEN6_PM_INIT_STATE: u32 = 1u32 << 21;
pub const ADF_GEN6_PM_CPM_PM_STATE_MASK: u32 = ((1u32 << (22 - 20 + 1)) - 1) << 20;

/* fusectl0 bitfields */
pub const ADF_GEN6_PM_ENABLE_PM_MASK: u32 = 1u32 << 21;
pub const ADF_GEN6_PM_ENABLE_PM_IDLE_MASK: u32 = 1u32 << 22;
pub const ADF_GEN6_PM_ENABLE_DEEP_PM_IDLE_MASK: u32 = 1u32 << 23;

/* cpm_pm_fw_init bitfields */
pub const ADF_GEN6_PM_IDLE_FILTER_MASK: u32 = ((1u32 << (5 - 3 + 1)) - 1) << 3;
pub const ADF_GEN6_PM_IDLE_ENABLE_MASK: u32 = 1u32 << 2;

/* ssm_pm_enable bitfield */
pub const ADF_GEN6_PM_SSM_PM_ENABLE_MASK: u32 = 1u32 << 0;

/* ssm_pm_domain_status bitfield */
pub const ADF_GEN6_PM_DOMAIN_POWERED_UP_MASK: u32 = 1u32 << 0;

// Under CONFIG_DEBUG_FS, this is an external function declaration.
// Without CONFIG_DEBUG_FS, the original header provides an empty inline function.
#[cfg(CONFIG_DEBUG_FS)]
extern "C" {
    pub fn adf_gen6_init_dev_pm_data(accel_dev: *mut adf_accel_dev);
}

#[cfg(not(CONFIG_DEBUG_FS))]
#[inline]
pub unsafe fn adf_gen6_init_dev_pm_data(_accel_dev: *mut adf_accel_dev) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
