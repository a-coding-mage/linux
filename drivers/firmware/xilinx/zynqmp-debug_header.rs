/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Xilinx Zynq MPSoC Firmware layer
 *
 *  Copyright (C) 2014-2018 Xilinx
 *
 *  Michal Simek <michal.simek@amd.com>
 *  Davorin Mista <davorin.mista@aggios.com>
 *  Jolly Shah <jollys@xilinx.com>
 *  Rajan Vaja <rajanv@xilinx.com>
 */

/* Equivalent of IS_REACHABLE(CONFIG_ZYNQMP_FIRMWARE_DEBUG). */
#[cfg(feature = "zynqmp_firmware_debug")]
extern "C" {
    pub fn zynqmp_pm_api_debugfs_init();
    pub fn zynqmp_pm_api_debugfs_exit();
}

#[cfg(not(feature = "zynqmp_firmware_debug"))]
#[inline]
pub unsafe fn zynqmp_pm_api_debugfs_init() {}

#[cfg(not(feature = "zynqmp_firmware_debug"))]
#[inline]
pub unsafe fn zynqmp_pm_api_debugfs_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
