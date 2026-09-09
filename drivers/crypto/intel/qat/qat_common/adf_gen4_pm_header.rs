/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2022 Intel Corporation */

/* Dependency intent: Linux bits and time-unit constants are supplied externally. */

use core::ffi::c_uint;

#[repr(C)]
pub struct adf_accel_dev {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum qat_pm_host_msg {
    PM_NO_CHANGE = 0,
    PM_SET_MIN,
}

/* Power management registers */
pub const ADF_GEN4_PM_HOST_MSG: u32 = 0x50A01C;

/* Power management */
pub const ADF_GEN4_PM_POLL_DELAY_US: u32 = 20;
pub const ADF_GEN4_PM_POLL_TIMEOUT_US: u32 = USEC_PER_SEC;
pub const ADF_GEN4_PM_MSG_POLL_DELAY_US: u32 = 10 * USEC_PER_MSEC;
pub const ADF_GEN4_PM_STATUS: u32 = 0x50A00C;
pub const ADF_GEN4_PM_INTERRUPT: u32 = 0x50A028;

/* Power management source in ERRSOU2 and ERRMSK2 */
pub const ADF_GEN4_PM_SOU: u32 = 1u32 << 18;

pub const ADF_GEN4_PM_IDLE_INT_EN: u32 = 1u32 << 18;
pub const ADF_GEN4_PM_THROTTLE_INT_EN: u32 = 1u32 << 19;
pub const ADF_GEN4_PM_DRV_ACTIVE: u32 = 1u32 << 20;
pub const ADF_GEN4_PM_INIT_STATE: u32 = 1u32 << 21;
pub const ADF_GEN4_PM_INT_EN_DEFAULT: u32 =
    ADF_GEN4_PM_IDLE_INT_EN | ADF_GEN4_PM_THROTTLE_INT_EN;

pub const ADF_GEN4_PM_THR_STS: u32 = 1u32 << 0;
pub const ADF_GEN4_PM_IDLE_STS: u32 = 1u32 << 1;
pub const ADF_GEN4_PM_FW_INT_STS: u32 = 1u32 << 2;
pub const ADF_GEN4_PM_INT_STS_MASK: u32 =
    ADF_GEN4_PM_THR_STS | ADF_GEN4_PM_IDLE_STS | ADF_GEN4_PM_FW_INT_STS;

pub const ADF_GEN4_PM_MSG_PENDING: u32 = 1u32 << 0;
pub const ADF_GEN4_PM_MSG_PAYLOAD_BIT_MASK: u32 = ((1u32 << (28 - 1 + 1)) - 1) << 1;

pub const ADF_GEN4_PM_DEFAULT_IDLE_FILTER: u32 = 0x6;
pub const ADF_GEN4_PM_MAX_IDLE_FILTER: u32 = 0x7;
pub const ADF_GEN4_PM_DEFAULT_IDLE_SUPPORT: u32 = 0x1;

/* PM CSRs fields masks */
pub const ADF_GEN4_PM_DOMAIN_POWER_GATED_MASK: u32 = (1u32 << 16) - 1;
pub const ADF_GEN4_PM_SSM_PM_ENABLE_MASK: u32 = (1u32 << 16) - 1;
pub const ADF_GEN4_PM_IDLE_FILTER_MASK: u32 = ((1u32 << 3) - 1) << 3;
pub const ADF_GEN4_PM_IDLE_ENABLE_MASK: u32 = 1u32 << 2;
pub const ADF_GEN4_PM_ENABLE_PM_MASK: u32 = 1u32 << 21;
pub const ADF_GEN4_PM_ENABLE_PM_IDLE_MASK: u32 = 1u32 << 22;
pub const ADF_GEN4_PM_ENABLE_DEEP_PM_IDLE_MASK: u32 = 1u32 << 23;
pub const ADF_GEN4_PM_CURRENT_WP_MASK: u32 = ((1u32 << 9) - 1) << 11;
pub const ADF_GEN4_PM_CPM_PM_STATE_MASK: u32 = ((1u32 << 3) - 1) << 20;
pub const ADF_GEN4_PM_PENDING_WP_MASK: u32 = ((1u32 << 9) - 1) << 23;
pub const ADF_GEN4_PM_THR_VALUE_MASK: u32 = ((1u32 << 3) - 1) << 4;
pub const ADF_GEN4_PM_MIN_PWR_ACK_MASK: u32 = 1u32 << 7;
pub const ADF_GEN4_PM_MIN_PWR_ACK_PENDING_MASK: u32 = 1u32 << 17;
pub const ADF_GEN4_PM_CPR_ACTIVE_COUNT_MASK: u32 = 1u32 << 0;
pub const ADF_GEN4_PM_CPR_MANAGED_COUNT_MASK: u32 = 1u32 << 0;
pub const ADF_GEN4_PM_XLT_ACTIVE_COUNT_MASK: u32 = 1u32 << 1;
pub const ADF_GEN4_PM_XLT_MANAGED_COUNT_MASK: u32 = 1u32 << 1;
pub const ADF_GEN4_PM_DCPR_ACTIVE_COUNT_MASK: u32 = ((1u32 << 2) - 1) << 2;
pub const ADF_GEN4_PM_DCPR_MANAGED_COUNT_MASK: u32 = ((1u32 << 2) - 1) << 2;
pub const ADF_GEN4_PM_PKE_ACTIVE_COUNT_MASK: u32 = ((1u32 << 5) - 1) << 4;
pub const ADF_GEN4_PM_PKE_MANAGED_COUNT_MASK: u32 = ((1u32 << 5) - 1) << 4;
pub const ADF_GEN4_PM_WAT_ACTIVE_COUNT_MASK: u32 = ((1u32 << 5) - 1) << 9;
pub const ADF_GEN4_PM_WAT_MANAGED_COUNT_MASK: u32 = ((1u32 << 5) - 1) << 9;
pub const ADF_GEN4_PM_WCP_ACTIVE_COUNT_MASK: u32 = ((1u32 << 5) - 1) << 14;
pub const ADF_GEN4_PM_WCP_MANAGED_COUNT_MASK: u32 = ((1u32 << 5) - 1) << 14;
pub const ADF_GEN4_PM_UCS_ACTIVE_COUNT_MASK: u32 = ((1u32 << 2) - 1) << 19;
pub const ADF_GEN4_PM_UCS_MANAGED_COUNT_MASK: u32 = ((1u32 << 2) - 1) << 19;
pub const ADF_GEN4_PM_CPH_ACTIVE_COUNT_MASK: u32 = ((1u32 << 4) - 1) << 21;
pub const ADF_GEN4_PM_CPH_MANAGED_COUNT_MASK: u32 = ((1u32 << 4) - 1) << 21;
pub const ADF_GEN4_PM_ATH_ACTIVE_COUNT_MASK: u32 = ((1u32 << 4) - 1) << 25;
pub const ADF_GEN4_PM_ATH_MANAGED_COUNT_MASK: u32 = ((1u32 << 4) - 1) << 25;

extern "C" {
    pub fn adf_gen4_enable_pm(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_gen4_handle_pm_interrupt(accel_dev: *mut adf_accel_dev) -> bool;

    #[cfg(feature = "CONFIG_DEBUG_FS")]
    pub fn adf_gen4_init_dev_pm_data(accel_dev: *mut adf_accel_dev);
}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub unsafe fn adf_gen4_init_dev_pm_data(_accel_dev: *mut adf_accel_dev) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
