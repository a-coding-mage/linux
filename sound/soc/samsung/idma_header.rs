/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2011 Samsung Electronics Co., Ltd
 *		http://www.samsung.com
 */

// Original C header included a guard: __SND_SOC_SAMSUNG_IDMA_H_

unsafe extern "C" {
    pub fn idma_reg_addr_init(regs: *mut core::ffi::c_void, addr: dma_addr_t);
}

/* dma_state */
pub const LPAM_DMA_STOP: i32 = 0;
pub const LPAM_DMA_START: i32 = 1;

pub const MAX_IDMA_PERIOD: i32 = 128 * 1024;
pub const MAX_IDMA_BUFFER: i32 = 160 * 1024;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
