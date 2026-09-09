/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) ST-Ericsson SA 2011
 *
 * Author: Mian Yousaf Kaukab <mian.yousaf.kaukab@stericsson.com>
 */

// C dependency: <linux/dmaengine.h>

pub const UX500_MUSB_DMA_NUM_RX_TX_CHANNELS: u32 = 8;

#[repr(C)]
pub struct ux500_musb_board_data {
    pub dma_rx_param_array: *mut *mut core::ffi::c_void,
    pub dma_tx_param_array: *mut *mut core::ffi::c_void,
    pub dma_filter:
        Option<unsafe extern "C" fn(chan: *mut dma_chan, filter_param: *mut core::ffi::c_void) -> bool>,
}

unsafe extern "C" {
    pub fn ux500_add_usb(
        parent: *mut device,
        base: resource_size_t,
        irq: core::ffi::c_int,
        dma_rx_cfg: *mut core::ffi::c_int,
        dma_tx_cfg: *mut core::ffi::c_int,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
