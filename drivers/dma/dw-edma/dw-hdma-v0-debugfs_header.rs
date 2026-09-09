/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2023 Cai Huoqing
 * Synopsys DesignWare HDMA v0 debugfs
 *
 * Author: Cai Huoqing <cai.huoqing@linux.dev>
 */

// Dependency supplied by <linux/dma/edma.h>.

// CONFIG_DEBUG_FS is a build-time C configuration condition; the equivalent
// Rust configuration is preserved here.
#[cfg(CONFIG_DEBUG_FS)]
unsafe extern "C" {
    pub fn dw_hdma_v0_debugfs_on(dw: *mut dw_edma);
}

#[cfg(not(CONFIG_DEBUG_FS))]
#[inline]
pub unsafe fn dw_hdma_v0_debugfs_on(_dw: *mut dw_edma) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
