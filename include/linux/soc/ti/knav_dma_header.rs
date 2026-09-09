/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2014 Texas Instruments Incorporated
 * Authors: Sandeep Nair <sandeep_n@ti.com>
 *          Cyril Chemparathy <cyril@ti.com>
 *          Santosh Shilimkar <santosh.shilimkar@ti.com>
 */

/* Dependency: linux/dmaengine.h */

/* PKTDMA descriptor manipulation macros for host packet descriptor */
#[inline]
pub const fn MASK(x: u32) -> u32 { (1u32 << x) - 1 }
pub const KNAV_DMA_DESC_PKT_LEN_MASK: u32 = (1u32 << 22) - 1;
pub const KNAV_DMA_DESC_PKT_LEN_SHIFT: u32 = 0;
pub const KNAV_DMA_DESC_PS_INFO_IN_SOP: u32 = 1u32 << 22;
pub const KNAV_DMA_DESC_PS_INFO_IN_DESC: u32 = 0;
pub const KNAV_DMA_DESC_TAG_MASK: u32 = (1u32 << 8) - 1;
pub const KNAV_DMA_DESC_SAG_HI_SHIFT: u32 = 24;
pub const KNAV_DMA_DESC_STAG_LO_SHIFT: u32 = 16;
pub const KNAV_DMA_DESC_DTAG_HI_SHIFT: u32 = 8;
pub const KNAV_DMA_DESC_DTAG_LO_SHIFT: u32 = 0;
pub const KNAV_DMA_DESC_HAS_EPIB: u32 = 1u32 << 31;
pub const KNAV_DMA_DESC_NO_EPIB: u32 = 0;
pub const KNAV_DMA_DESC_PSLEN_SHIFT: u32 = 24;
pub const KNAV_DMA_DESC_PSLEN_MASK: u32 = (1u32 << 6) - 1;
pub const KNAV_DMA_DESC_ERR_FLAG_SHIFT: u32 = 20;
pub const KNAV_DMA_DESC_ERR_FLAG_MASK: u32 = (1u32 << 4) - 1;
pub const KNAV_DMA_DESC_PSFLAG_SHIFT: u32 = 16;
pub const KNAV_DMA_DESC_PSFLAG_MASK: u32 = (1u32 << 4) - 1;
pub const KNAV_DMA_DESC_RETQ_SHIFT: u32 = 0;
pub const KNAV_DMA_DESC_RETQ_MASK: u32 = (1u32 << 14) - 1;
pub const KNAV_DMA_DESC_BUF_LEN_MASK: u32 = (1u32 << 22) - 1;
pub const KNAV_DMA_DESC_EFLAGS_MASK: u32 = (1u32 << 4) - 1;
pub const KNAV_DMA_DESC_EFLAGS_SHIFT: u32 = 20;

pub const KNAV_DMA_NUM_EPIB_WORDS: usize = 4;
pub const KNAV_DMA_NUM_PS_WORDS: usize = 16;
pub const KNAV_DMA_NUM_SW_DATA_WORDS: usize = 4;
pub const KNAV_DMA_FDQ_PER_CHAN: usize = 4;

/* Tx channel scheduling priority */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum knav_dma_tx_priority {
    DMA_PRIO_HIGH = 0,
    DMA_PRIO_MED_H,
    DMA_PRIO_MED_L,
    DMA_PRIO_LOW,
}

/* Rx channel error handling mode during buffer starvation */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum knav_dma_rx_err_mode {
    DMA_DROP = 0,
    DMA_RETRY,
}

/* Rx flow size threshold configuration */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum knav_dma_rx_thresholds {
    DMA_THRESH_NONE = 0,
    DMA_THRESH_0 = 1,
    DMA_THRESH_0_1 = 3,
    DMA_THRESH_0_1_2 = 7,
}

/* Descriptor type */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum knav_dma_desc_type {
    DMA_DESC_HOST = 0,
    DMA_DESC_MONOLITHIC = 2,
}

#[repr(C)]
pub struct knav_dma_tx_cfg {
    pub filt_einfo: bool,
    pub filt_pswords: bool,
    pub priority: knav_dma_tx_priority,
}

#[repr(C)]
pub struct knav_dma_rx_cfg {
    pub einfo_present: bool,
    pub psinfo_present: bool,
    pub err_mode: knav_dma_rx_err_mode,
    pub desc_type: knav_dma_desc_type,
    pub psinfo_at_sop: bool,
    pub sop_offset: core::ffi::c_uint,
    pub dst_q: core::ffi::c_uint,
    pub thresh: knav_dma_rx_thresholds,
    pub fdq: [core::ffi::c_uint; KNAV_DMA_FDQ_PER_CHAN],
    pub sz_thresh0: core::ffi::c_uint,
    pub sz_thresh1: core::ffi::c_uint,
    pub sz_thresh2: core::ffi::c_uint,
}

#[repr(C)]
pub union knav_dma_cfg_u {
    pub tx: knav_dma_tx_cfg,
    pub rx: knav_dma_rx_cfg,
}

#[repr(C)]
pub struct knav_dma_cfg {
    pub direction: dma_transfer_direction,
    pub u: knav_dma_cfg_u,
}

#[repr(C, align(64))]
pub struct knav_dma_desc {
    pub desc_info: __le32,
    pub tag_info: __le32,
    pub packet_info: __le32,
    pub buff_len: __le32,
    pub buff: __le32,
    pub next_desc: __le32,
    pub orig_len: __le32,
    pub orig_buff: __le32,
    pub epib: [__le32; KNAV_DMA_NUM_EPIB_WORDS],
    pub psdata: [__le32; KNAV_DMA_NUM_PS_WORDS],
    pub sw_data: [u32; KNAV_DMA_NUM_SW_DATA_WORDS],
}

/* CONFIG_KEYSTONE_NAVIGATOR_DMA controls whether these are external APIs or stubs. */
#[cfg(feature = "CONFIG_KEYSTONE_NAVIGATOR_DMA")]
extern "C" {
    pub fn knav_dma_open_channel(dev: *mut device, name: *const core::ffi::c_char,
                                 config: *mut knav_dma_cfg) -> *mut core::ffi::c_void;
    pub fn knav_dma_close_channel(channel: *mut core::ffi::c_void);
    pub fn knav_dma_get_flow(channel: *mut core::ffi::c_void) -> core::ffi::c_int;
    pub fn knav_dma_device_ready() -> bool;
}

#[cfg(not(feature = "CONFIG_KEYSTONE_NAVIGATOR_DMA"))]
#[inline]
pub unsafe fn knav_dma_open_channel(_dev: *mut device, _name: *const core::ffi::c_char,
                                    _config: *mut knav_dma_cfg) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_KEYSTONE_NAVIGATOR_DMA"))]
#[inline]
pub unsafe fn knav_dma_close_channel(_channel: *mut core::ffi::c_void) {}

#[cfg(not(feature = "CONFIG_KEYSTONE_NAVIGATOR_DMA"))]
#[inline]
pub unsafe fn knav_dma_get_flow(_channel: *mut core::ffi::c_void) -> core::ffi::c_int {
    -22
}

#[cfg(not(feature = "CONFIG_KEYSTONE_NAVIGATOR_DMA"))]
#[inline]
pub unsafe fn knav_dma_device_ready() -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
