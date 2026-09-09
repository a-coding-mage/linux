/*
 * drivers/dma/fsl_raid.h
 *
 * Freescale RAID Engine device driver
 *
 * Author:
 *	Harninder Rai <harninder.rai@freescale.com>
 *	Naveen Burmi <naveenburmi@freescale.com>
 *
 * Rewrite:
 *	Xuelin Shi <xuelin.shi@freescale.com>

 * Copyright (c) 2010-2012 Freescale Semiconductor, Inc.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in the
 *       documentation and/or other materials provided with the distribution.
 *     * Neither the name of Freescale Semiconductor nor the
 *       names of its contributors may be used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * ALTERNATIVELY, this software may be distributed under the terms of the
 * GNU General Public License ("GPL") as published by the Free Software
 * Foundation, either version 2 of that License or (at your option) any later
 * version.
 */

pub const FSL_RE_MAX_CHANS: usize = 4;
pub const FSL_RE_DPAA_MODE: u32 = 1u32 << 30;
pub const FSL_RE_NON_DPAA_MODE: u32 = 1u32 << 31;
pub const FSL_RE_GFM_POLY: u32 = 0x1d000000;
pub const FSL_RE_ADD_JOB_SHIFT: u32 = 16;
pub const FSL_RE_RMVD_JOB_SHIFT: u32 = 16;
#[inline] pub const fn FSL_RE_ADD_JOB(x: u32) -> u32 { x << 16 }
#[inline] pub const fn FSL_RE_RMVD_JOB(x: u32) -> u32 { x << 16 }
pub const FSL_RE_CFG1_CBSI: u32 = 0x08000000;
pub const FSL_RE_CFG1_CBS0: u32 = 0x00080000;
pub const FSL_RE_SLOT_FULL_SHIFT: u32 = 8;
#[inline] pub const fn FSL_RE_SLOT_FULL(x: u32) -> u32 { x >> 8 }
pub const FSL_RE_SLOT_AVAIL_SHIFT: u32 = 8;
#[inline] pub const fn FSL_RE_SLOT_AVAIL(x: u32) -> u32 { x >> 8 }
pub const FSL_RE_PQ_OPCODE: u32 = 0x1B;
pub const FSL_RE_XOR_OPCODE: u32 = 0x1A;
pub const FSL_RE_MOVE_OPCODE: u32 = 0x8;
pub const FSL_RE_FRAME_ALIGN: usize = 16;
pub const FSL_RE_BLOCK_SIZE: u32 = 0x3;
pub const FSL_RE_CACHEABLE_IO: u32 = 0x0;
pub const FSL_RE_BUFFER_OUTPUT: u32 = 0x0;
pub const FSL_RE_INTR_ON_ERROR: u32 = 0x1;
pub const FSL_RE_DATA_DEP: u32 = 0x1;
pub const FSL_RE_ENABLE_DPI: u32 = 0x0;
pub const FSL_RE_RING_SIZE: u32 = 0x400;
pub const FSL_RE_RING_SIZE_MASK: u32 = FSL_RE_RING_SIZE - 1;
pub const FSL_RE_RING_SIZE_SHIFT: u32 = 8;
pub const FSL_RE_ADDR_BIT_SHIFT: u32 = 4;
pub const FSL_RE_ADDR_BIT_MASK: u32 = (1u32 << FSL_RE_ADDR_BIT_SHIFT) - 1;
pub const FSL_RE_ERROR: u32 = 0x40000000;
pub const FSL_RE_INTR: u32 = 0x80000000;
pub const FSL_RE_CLR_INTR: u32 = 0x80000000;
pub const FSL_RE_PAUSE: u32 = 0x80000000;
pub const FSL_RE_ENABLE: u32 = 0x80000000;
pub const FSL_RE_REG_LIODN_MASK: u32 = 0x00000FFF;

pub const FSL_RE_CDB_OPCODE_MASK: u32 = 0xF8000000;
pub const FSL_RE_CDB_OPCODE_SHIFT: u32 = 27;
pub const FSL_RE_CDB_EXCLEN_MASK: u32 = 0x03000000;
pub const FSL_RE_CDB_EXCLEN_SHIFT: u32 = 24;
pub const FSL_RE_CDB_EXCLQ1_MASK: u32 = 0x00F00000;
pub const FSL_RE_CDB_EXCLQ1_SHIFT: u32 = 20;
pub const FSL_RE_CDB_EXCLQ2_MASK: u32 = 0x000F0000;
pub const FSL_RE_CDB_EXCLQ2_SHIFT: u32 = 16;
pub const FSL_RE_CDB_BLKSIZE_MASK: u32 = 0x0000C000;
pub const FSL_RE_CDB_BLKSIZE_SHIFT: u32 = 14;
pub const FSL_RE_CDB_CACHE_MASK: u32 = 0x00003000;
pub const FSL_RE_CDB_CACHE_SHIFT: u32 = 12;
pub const FSL_RE_CDB_BUFFER_MASK: u32 = 0x00000800;
pub const FSL_RE_CDB_BUFFER_SHIFT: u32 = 11;
pub const FSL_RE_CDB_ERROR_MASK: u32 = 0x00000400;
pub const FSL_RE_CDB_ERROR_SHIFT: u32 = 10;
pub const FSL_RE_CDB_NRCS_MASK: u32 = 0x0000003C;
pub const FSL_RE_CDB_NRCS_SHIFT: u32 = 6;
pub const FSL_RE_CDB_DEPEND_MASK: u32 = 0x00000008;
pub const FSL_RE_CDB_DEPEND_SHIFT: u32 = 3;
pub const FSL_RE_CDB_DPI_MASK: u32 = 0x00000004;
pub const FSL_RE_CDB_DPI_SHIFT: u32 = 2;

pub const FSL_RE_CF_DESC_SIZE: usize = 320;
pub const FSL_RE_CF_CDB_SIZE: usize = 512;
pub const FSL_RE_CF_CDB_ALIGN: usize = 64;

#[repr(C)]
pub struct fsl_re_ctrl {
    pub global_config: u32, pub rsvd1: [u8; 4], pub galois_field_config: u32, pub rsvd2: [u8; 4],
    pub jq_wrr_config: u32, pub rsvd3: [u8; 4], pub crc_config: u32, pub rsvd4: [u8; 228],
    pub system_reset: u32, pub rsvd5: [u8; 252], pub global_status: u32, pub rsvd6: [u8; 832],
    pub re_liodn_base: u32, pub rsvd7: [u8; 1712], pub re_version_id: u32, pub re_version_id_2: u32,
    pub rsvd8: [u8; 512], pub host_config: u32,
}

#[repr(C)]
pub struct fsl_re_chan_cfg {
    pub jr_config_0: u32, pub jr_config_1: u32, pub jr_interrupt_status: u32, pub rsvd1: [u8; 4],
    pub jr_command: u32, pub rsvd2: [u8; 4], pub jr_status: u32, pub rsvd3: [u8; 228],
    pub inbring_base_h: u32, pub inbring_base_l: u32, pub inbring_size: u32, pub rsvd4: [u8; 4],
    pub inbring_slot_avail: u32, pub rsvd5: [u8; 4], pub inbring_add_job: u32, pub rsvd6: [u8; 4],
    pub inbring_cnsmr_indx: u32, pub rsvd7: [u8; 220], pub oubring_base_h: u32, pub oubring_base_l: u32,
    pub oubring_size: u32, pub rsvd8: [u8; 4], pub oubring_job_rmvd: u32, pub rsvd9: [u8; 4],
    pub oubring_slot_full: u32, pub rsvd10: [u8; 4], pub oubring_prdcr_indx: u32,
}

#[repr(C)] pub struct fsl_re_move_cdb { pub cdb32: u32 }

pub const FSL_RE_DPI_APPS_MASK: u32 = 0xC0000000;
pub const FSL_RE_DPI_APPS_SHIFT: u32 = 30;
pub const FSL_RE_DPI_REF_MASK: u32 = 0x30000000;
pub const FSL_RE_DPI_REF_SHIFT: u32 = 28;
pub const FSL_RE_DPI_GUARD_MASK: u32 = 0x0C000000;
pub const FSL_RE_DPI_GUARD_SHIFT: u32 = 26;
pub const FSL_RE_DPI_ATTR_MASK: u32 = 0x03000000;
pub const FSL_RE_DPI_ATTR_SHIFT: u32 = 24;
pub const FSL_RE_DPI_META_MASK: u32 = 0x0000FFFF;

#[repr(C)] pub struct fsl_re_dpi { pub dpi32: u32, pub ref_: u32 }
#[repr(C)] pub struct fsl_re_xor_cdb { pub cdb32: u32, pub gfm: [u8; 16], pub dpi_dest_spec: fsl_re_dpi, pub dpi_src_spec: [fsl_re_dpi; 16] }
#[repr(C)] pub struct fsl_re_noop_cdb { pub cdb32: u32 }
#[repr(C)] pub struct fsl_re_pq_cdb { pub cdb32: u32, pub gfm_q1: [u8; 16], pub gfm_q2: [u8; 16], pub dpi_dest_spec: [fsl_re_dpi; 2], pub dpi_src_spec: [fsl_re_dpi; 16] }

pub const FSL_RE_CF_ADDR_HIGH_MASK: u32 = 0x000000FF;
pub const FSL_RE_CF_EXT_MASK: u32 = 0x80000000;
pub const FSL_RE_CF_EXT_SHIFT: u32 = 31;
pub const FSL_RE_CF_FINAL_MASK: u32 = 0x40000000;
pub const FSL_RE_CF_FINAL_SHIFT: u32 = 30;
pub const FSL_RE_CF_LENGTH_MASK: u32 = 0x000FFFFF;
pub const FSL_RE_CF_BPID_MASK: u32 = 0x00FF0000;
pub const FSL_RE_CF_BPID_SHIFT: u32 = 16;
pub const FSL_RE_CF_OFFSET_MASK: u32 = 0x00001FFF;
#[repr(C)] pub struct fsl_re_cmpnd_frame { pub addr_high: u32, pub addr_low: u32, pub efrl32: u32, pub rbro32: u32 }

pub const FSL_RE_HWDESC_LIODN_MASK: u32 = 0x3F000000;
pub const FSL_RE_HWDESC_LIODN_SHIFT: u32 = 24;
pub const FSL_RE_HWDESC_BPID_MASK: u32 = 0x00FF0000;
pub const FSL_RE_HWDESC_BPID_SHIFT: u32 = 16;
pub const FSL_RE_HWDESC_ELIODN_MASK: u32 = 0x0000F000;
pub const FSL_RE_HWDESC_ELIODN_SHIFT: u32 = 12;
pub const FSL_RE_HWDESC_FMT_SHIFT: u32 = 29;
pub const FSL_RE_HWDESC_FMT_MASK: u32 = 0x3 << FSL_RE_HWDESC_FMT_SHIFT;
#[repr(C)] pub struct fsl_re_hw_desc { pub lbea32: u32, pub addr_low: u32, pub fmt32: u32, pub status: u32 }

#[repr(C)]
pub struct fsl_re_drv_private {
    pub total_chans: u8,
    pub dma_dev: dma_device,
    pub base: *mut core::ffi::c_void,
    pub re_jrs: [*mut fsl_re_chan; FSL_RE_MAX_CHANS],
    pub cf_desc_pool: *mut dma_pool,
    pub hw_desc_pool: *mut dma_pool,
}

#[repr(C)]
pub struct fsl_re_chan {
    pub name: [core::ffi::c_char; 16], pub desc_lock: spinlock_t, pub ack_q: list_head, pub active_q: list_head,
    pub submit_q: list_head, pub free_q: list_head, pub dev: *mut device, pub re_dev: *mut fsl_re_drv_private,
    pub chan: dma_chan, pub jrregs: *mut fsl_re_chan_cfg, pub irq: i32, pub irqtask: tasklet_struct, pub alloc_count: u32,
    pub inb_phys_addr: dma_addr_t, pub inb_ring_virt_addr: *mut fsl_re_hw_desc, pub inb_count: u32,
    pub oub_phys_addr: dma_addr_t, pub oub_ring_virt_addr: *mut fsl_re_hw_desc, pub oub_count: u32,
}

#[repr(C)]
pub struct fsl_re_desc {
    pub async_tx: dma_async_tx_descriptor, pub node: list_head, pub hwdesc: fsl_re_hw_desc,
    pub re_chan: *mut fsl_re_chan, pub cf_addr: *mut core::ffi::c_void, pub cf_paddr: dma_addr_t,
    pub cdb_addr: *mut core::ffi::c_void, pub cdb_paddr: dma_addr_t, pub status: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
