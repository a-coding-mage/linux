/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2007-2010 Freescale Semiconductor, Inc. All rights reserved.
 *
 * Author:
 *   Zhang Wei <wei.zhang@freescale.com>, Jul 2007
 *   Ebony Zhu <ebony.zhu@freescale.com>, May 2007
 */

// Translated from the Linux Freescale DMA controller header.

pub const FSL_DMA_MR_CS: u32 = 0x00000001;
pub const FSL_DMA_MR_CC: u32 = 0x00000002;
pub const FSL_DMA_MR_CA: u32 = 0x00000008;
pub const FSL_DMA_MR_EIE: u32 = 0x00000040;
pub const FSL_DMA_MR_XFE: u32 = 0x00000020;
pub const FSL_DMA_MR_EOLNIE: u32 = 0x00000100;
pub const FSL_DMA_MR_EOLSIE: u32 = 0x00000080;
pub const FSL_DMA_MR_EOSIE: u32 = 0x00000200;
pub const FSL_DMA_MR_CDSM: u32 = 0x00000010;
pub const FSL_DMA_MR_CTM: u32 = 0x00000004;
pub const FSL_DMA_MR_EMP_EN: u32 = 0x00200000;
pub const FSL_DMA_MR_EMS_EN: u32 = 0x00040000;
pub const FSL_DMA_MR_DAHE: u32 = 0x00002000;
pub const FSL_DMA_MR_SAHE: u32 = 0x00001000;
pub const FSL_DMA_MR_SAHTS_MASK: u32 = 0x0000C000;
pub const FSL_DMA_MR_DAHTS_MASK: u32 = 0x00030000;
pub const FSL_DMA_MR_BWC_MASK: u32 = 0x0f000000;
/* Bandwidth/pause control determines how many bytes a channel transfers before pausing. */
pub const FSL_DMA_MR_BWC: u32 = 0x0A000000;
pub const FSL_DMA_MR_EOTIE: u32 = 0x00000080;
pub const FSL_DMA_MR_PRC_RM: u32 = 0x00000800;

pub const FSL_DMA_SR_CH: u32 = 0x20;
pub const FSL_DMA_SR_PE: u32 = 0x10;
pub const FSL_DMA_SR_CB: u32 = 0x04;
pub const FSL_DMA_SR_TE: u32 = 0x80;
pub const FSL_DMA_SR_EOSI: u32 = 0x02;
pub const FSL_DMA_SR_EOLSI: u32 = 0x01;
pub const FSL_DMA_SR_EOCDI: u32 = 0x01;
pub const FSL_DMA_SR_EOLNI: u32 = 0x08;
pub const FSL_DMA_SATR_SBPATMU: u32 = 0x20000000;
pub const FSL_DMA_SATR_STRANSINT_RIO: u32 = 0x00c00000;
pub const FSL_DMA_SATR_SREADTYPE_SNOOP_READ: u32 = 0x00050000;
pub const FSL_DMA_SATR_SREADTYPE_BP_IORH: u32 = 0x00020000;
pub const FSL_DMA_SATR_SREADTYPE_BP_NREAD: u32 = 0x00040000;
pub const FSL_DMA_SATR_SREADTYPE_BP_MREAD: u32 = 0x00070000;
pub const FSL_DMA_DATR_DBPATMU: u32 = 0x20000000;
pub const FSL_DMA_DATR_DTRANSINT_RIO: u32 = 0x00c00000;
pub const FSL_DMA_DATR_DWRITETYPE_SNOOP_WRITE: u32 = 0x00050000;
pub const FSL_DMA_DATR_DWRITETYPE_BP_FLUSH: u32 = 0x00010000;
pub const FSL_DMA_EOL: u64 = 1;
pub const FSL_DMA_SNEN: u64 = 0x10;
pub const FSL_DMA_EOSIE: u32 = 8;
pub const FSL_DMA_NLDA_MASK: u64 = !0x1f;
pub const FSL_DMA_BCR_MAX_CNT: u32 = 0x03ffffff;
pub const FSL_DMA_DGSR_TE: u32 = 0x80;
pub const FSL_DMA_DGSR_CH: u32 = 0x20;
pub const FSL_DMA_DGSR_PE: u32 = 0x10;
pub const FSL_DMA_DGSR_EOLNI: u32 = 8;
pub const FSL_DMA_DGSR_CB: u32 = 4;
pub const FSL_DMA_DGSR_EOSI: u32 = 2;
pub const FSL_DMA_DGSR_EOLSI: u32 = 1;

pub type v64 = u64;
pub type v32 = u32;

#[repr(C, align(32))]
pub struct fsl_dma_ld_hw { pub src_addr: v64, pub dst_addr: v64, pub next_ln_addr: v64, pub count: v32, pub reserve: v32 }

#[repr(C, align(32))]
pub struct fsl_desc_sw { pub hw: fsl_dma_ld_hw, pub node: list_head, pub tx_list: list_head, pub async_tx: dma_async_tx_descriptor }

#[repr(C)]
pub struct fsldma_chan_regs { pub mr: u32, pub sr: u32, pub cdar: u64, pub sar: u64, pub dar: u64, pub bcr: u32, pub ndar: u64 }

pub struct fsldma_chan;
pub const FSL_DMA_MAX_CHANS_PER_DEVICE: usize = 8;
#[repr(C)]
pub struct fsldma_device { pub regs: *mut core::ffi::c_void, pub dev: *mut device, pub common: dma_device, pub chan: [*mut fsldma_chan; FSL_DMA_MAX_CHANS_PER_DEVICE], pub feature: u32, pub irq: i32, pub addr_bits: i32 }

pub const FSL_DMA_LITTLE_ENDIAN: u32 = 0;
pub const FSL_DMA_BIG_ENDIAN: u32 = 1;
pub const FSL_DMA_IP_MASK: u32 = 0x00000ff0;
pub const FSL_DMA_IP_85XX: u32 = 0x10;
pub const FSL_DMA_IP_83XX: u32 = 0x20;
pub const FSL_DMA_CHAN_PAUSE_EXT: u32 = 0x1000;
pub const FSL_DMA_CHAN_START_EXT: u32 = 0x2000;

#[cfg(feature = "CONFIG_PM")]
#[repr(C)] pub struct fsldma_chan_regs_save { pub mr: u32 }
#[cfg(feature = "CONFIG_PM")]
#[repr(C)] pub enum fsldma_pm_state { RUNNING = 0, SUSPENDED }

#[repr(C)]
pub struct fsldma_chan {
    pub name: [core::ffi::c_char; 8], pub regs: *mut fsldma_chan_regs, pub desc_lock: spinlock_t,
    pub ld_pending: list_head, pub ld_running: list_head, pub ld_completed: list_head,
    pub common: dma_chan, pub desc_pool: *mut dma_pool, pub dev: *mut device,
    pub irq: i32, pub id: i32, pub tasklet: tasklet_struct, pub feature: u32, pub idle: bool,
    #[cfg(feature = "CONFIG_PM")] pub regs_save: fsldma_chan_regs_save,
    #[cfg(feature = "CONFIG_PM")] pub pm_state: fsldma_pm_state,
    pub toggle_ext_pause: Option<unsafe extern "C" fn(*mut fsldma_chan, i32)>,
    pub toggle_ext_start: Option<unsafe extern "C" fn(*mut fsldma_chan, i32)>,
    pub set_src_loop_size: Option<unsafe extern "C" fn(*mut fsldma_chan, i32)>,
    pub set_dst_loop_size: Option<unsafe extern "C" fn(*mut fsldma_chan, i32)>,
    pub set_request_count: Option<unsafe extern "C" fn(*mut fsldma_chan, i32)>,
}

// `container_of` conversions and architecture/configuration-dependent I/O macros
// (`FSL_DMA_IN`, `FSL_DMA_OUT`, `DMA_TO_CPU`, and `CPU_TO_DMA`) retain their C
// semantics and are supplied by the surrounding kernel translation.

extern "C" {
    pub type list_head;
    pub type dma_async_tx_descriptor;
    pub type device;
    pub type dma_device;
    pub type spinlock_t;
    pub type dma_chan;
    pub type dma_pool;
    pub type tasklet_struct;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
