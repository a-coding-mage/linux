/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Firmware layer for UFS APIs.
 *
 * Copyright (c) 2025 Advanced Micro Devices, Inc.
 */

// Equivalent of IS_REACHABLE(CONFIG_ZYNQMP_FIRMWARE): when the firmware
// support is reachable, these functions are supplied externally.
#[cfg(feature = "config_zynqmp_firmware")]
unsafe extern "C" {
    pub fn zynqmp_pm_is_mphy_tx_rx_config_ready(is_ready: *mut bool) -> i32;
    pub fn zynqmp_pm_is_sram_init_done(is_done: *mut bool) -> i32;
    pub fn zynqmp_pm_set_sram_bypass() -> i32;
    pub fn zynqmp_pm_get_ufs_calibration_values(val: *mut u32) -> i32;
}

// Fallback definitions corresponding to the !IS_REACHABLE(CONFIG_ZYNQMP_FIRMWARE)
// branch. ENODEV is Linux's "No such device" error value.
#[cfg(not(feature = "config_zynqmp_firmware"))]
#[inline]
pub unsafe fn zynqmp_pm_is_mphy_tx_rx_config_ready(_is_ready: *mut bool) -> i32 {
    -19
}

#[cfg(not(feature = "config_zynqmp_firmware"))]
#[inline]
pub unsafe fn zynqmp_pm_is_sram_init_done(_is_done: *mut bool) -> i32 {
    -19
}

#[cfg(not(feature = "config_zynqmp_firmware"))]
#[inline]
pub unsafe fn zynqmp_pm_set_sram_bypass() -> i32 {
    -19
}

#[cfg(not(feature = "config_zynqmp_firmware"))]
#[inline]
pub unsafe fn zynqmp_pm_get_ufs_calibration_values(_val: *mut u32) -> i32 {
    -19
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
