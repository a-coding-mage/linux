/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2023 Cai Huoqing
 * Synopsys DesignWare HDMA v0 core
 *
 * Author: Cai Huoqing <cai.huoqing@linux.dev>
 */

// Dependency provided by <linux/dma/edma.h>.
#[repr(C)]
pub struct dw_edma {
    _private: [u8; 0],
}

/* HDMA core register */
unsafe extern "C" {
    pub fn dw_hdma_v0_core_register(dw: *mut dw_edma);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
