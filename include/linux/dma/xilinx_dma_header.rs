/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Xilinx DMA Engine drivers support header file
 *
 * Copyright (C) 2010-2014 Xilinx, Inc. All rights reserved.
 */

// Dependencies supplied by the Linux DMA mapping and DMA engine interfaces.

/**
 * struct xilinx_vdma_config - VDMA Configuration structure
 * @frm_dly: Frame delay
 * @gen_lock: Whether in gen-lock mode
 * @master: Master that it syncs to
 * @frm_cnt_en: Enable frame count enable
 * @park: Whether wants to park
 * @park_frm: Frame to park on
 * @coalesc: Interrupt coalescing threshold
 * @delay: Delay counter
 * @reset: Reset Channel
 * @ext_fsync: External Frame Sync source
 * @vflip_en:  Vertical Flip enable
 */
#[repr(C)]
pub struct xilinx_vdma_config {
    pub frm_dly: i32,
    pub gen_lock: i32,
    pub master: i32,
    pub frm_cnt_en: i32,
    pub park: i32,
    pub park_frm: i32,
    pub coalesc: i32,
    pub delay: i32,
    pub reset: i32,
    pub ext_fsync: i32,
    pub vflip_en: bool,
}

// External DMA channel type supplied by the DMA engine interface.
pub struct dma_chan;

unsafe extern "C" {
    pub fn xilinx_vdma_channel_set_config(
        dchan: *mut dma_chan,
        cfg: *mut xilinx_vdma_config,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
