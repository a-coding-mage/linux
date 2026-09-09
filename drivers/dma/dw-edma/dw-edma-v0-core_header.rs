/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2018-2019 Synopsys, Inc. and/or its affiliates.
 * Synopsys DesignWare eDMA v0 core
 *
 * Author: Gustavo Pimentel <gustavo.pimentel@synopsys.com>
 */

/* Dependency supplied by <linux/dma/edma.h>. */
#[repr(C)]
pub struct dw_edma {
    _private: [u8; 0],
}

/* eDMA core register */
unsafe extern "C" {
    pub fn dw_edma_v0_core_register(dw: *mut dw_edma);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
