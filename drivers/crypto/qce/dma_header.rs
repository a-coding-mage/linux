/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2011-2014, The Linux Foundation. All rights reserved.
 */

// Dependency equivalent of: #include <linux/dmaengine.h>
use core::ffi::c_void;
use crate::{device, dma_async_tx_callback, dma_chan, scatterlist, sg_table};

/* maximum data transfer block size between BAM and CE */
pub const QCE_BAM_BURST_SIZE: usize = 64;

pub const QCE_AUTHIV_REGS_CNT: usize = 16;
pub const QCE_AUTH_BYTECOUNT_REGS_CNT: usize = 4;
pub const QCE_CNTRIV_REGS_CNT: usize = 4;

#[repr(C)]
pub struct qce_result_dump {
    pub auth_iv: [u32; QCE_AUTHIV_REGS_CNT],
    pub auth_byte_count: [u32; QCE_AUTH_BYTECOUNT_REGS_CNT],
    pub encr_cntr_iv: [u32; QCE_CNTRIV_REGS_CNT],
    pub status: u32,
    pub status2: u32,
}

pub const QCE_IGNORE_BUF_SZ: usize = 2 * QCE_BAM_BURST_SIZE;
pub const QCE_RESULT_BUF_SZ: usize = 128; // ALIGN(sizeof(struct qce_result_dump), QCE_BAM_BURST_SIZE)

#[repr(C)]
pub struct qce_dma_data {
    pub txchan: *mut dma_chan,
    pub rxchan: *mut dma_chan,
    pub result_buf: *mut qce_result_dump,
    pub ignore_buf: *mut c_void,
}

extern "C" {
    pub fn devm_qce_dma_request(dev: *mut device, dma: *mut qce_dma_data) -> i32;
    pub fn qce_dma_prep_sgs(
        dma: *mut qce_dma_data,
        sg_in: *mut scatterlist,
        in_ents: i32,
        sg_out: *mut scatterlist,
        out_ents: i32,
        cb: dma_async_tx_callback,
        cb_param: *mut c_void,
    ) -> i32;
    pub fn qce_dma_issue_pending(dma: *mut qce_dma_data);
    pub fn qce_dma_terminate_all(dma: *mut qce_dma_data) -> i32;
    pub fn qce_sgtable_add(
        sgt: *mut sg_table,
        sg_add: *mut scatterlist,
        max_len: u32,
    ) -> *mut scatterlist;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
