/*
 * Copyright (C) 2017 Spreadtrum Communications Inc.
 *
 * SPDX-License-Identifier: GPL-2.0
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

/* Linux DMA driver translation.  Kernel-provided types and functions are
 * intentionally left as external dependencies, as in the original source. */

const SPRD_DMA_CHN_REG_OFFSET: u32 = 0x1000;
const SPRD_DMA_CHN_REG_LENGTH: u32 = 0x40;
const SPRD_DMA_MEMCPY_MIN_SIZE: usize = 64;
const SPRD_DMA_GLB_PAUSE: u32 = 0x0;
const SPRD_DMA_GLB_FRAG_WAIT: u32 = 0x4;
const SPRD_DMA_GLB_REQ_PEND0_EN: u32 = 0x8;
const SPRD_DMA_GLB_REQ_PEND1_EN: u32 = 0xc;
const SPRD_DMA_GLB_INT_RAW_STS: u32 = 0x10;
const SPRD_DMA_GLB_INT_MSK_STS: u32 = 0x14;
const SPRD_DMA_GLB_REQ_STS: u32 = 0x18;
const SPRD_DMA_GLB_CHN_EN_STS: u32 = 0x1c;
const SPRD_DMA_GLB_DEBUG_STS: u32 = 0x20;
const SPRD_DMA_GLB_ARB_SEL_STS: u32 = 0x24;
const SPRD_DMA_GLB_2STAGE_GRP1: u32 = 0x28;
const SPRD_DMA_GLB_2STAGE_GRP2: u32 = 0x2c;
const SPRD_DMA_GLB_REQ_UID_OFFSET: u32 = 0x2000;
const SPRD_DMA_CHN_PAUSE: u32 = 0x0;
const SPRD_DMA_CHN_REQ: u32 = 0x4;
const SPRD_DMA_CHN_CFG: u32 = 0x8;
const SPRD_DMA_CHN_INTC: u32 = 0xc;
const SPRD_DMA_CHN_SRC_ADDR: u32 = 0x10;
const SPRD_DMA_CHN_DES_ADDR: u32 = 0x14;
const SPRD_DMA_CHN_FRG_LEN: u32 = 0x18;
const SPRD_DMA_CHN_BLK_LEN: u32 = 0x1c;
const SPRD_DMA_CHN_TRSC_LEN: u32 = 0x20;
const SPRD_DMA_CHN_TRSF_STEP: u32 = 0x24;
const SPRD_DMA_CHN_WARP_PTR: u32 = 0x28;
const SPRD_DMA_CHN_WARP_TO: u32 = 0x2c;
const SPRD_DMA_CHN_LLIST_PTR: u32 = 0x30;
const SPRD_DMA_CHN_FRAG_STEP: u32 = 0x34;
const SPRD_DMA_CHN_SRC_BLK_STEP: u32 = 0x38;
const SPRD_DMA_CHN_DES_BLK_STEP: u32 = 0x3c;

const fn bit(n: u32) -> u32 { 1u32 << n }
const fn genmask(h: u32, l: u32) -> u32 { (u32::MAX >> (31-h)) & (u32::MAX << l) }
const SPRD_DMA_GLB_2STAGE_EN: u32 = bit(24);
const SPRD_DMA_GLB_CHN_INT_MASK: u32 = genmask(23,20);
const SPRD_DMA_GLB_DEST_INT: u32 = bit(22);
const SPRD_DMA_GLB_SRC_INT: u32 = bit(20);
const SPRD_DMA_GLB_LIST_DONE_TRG: u32 = bit(19);
const SPRD_DMA_GLB_TRANS_DONE_TRG: u32 = bit(18);
const SPRD_DMA_GLB_BLOCK_DONE_TRG: u32 = bit(17);
const SPRD_DMA_GLB_FRAG_DONE_TRG: u32 = bit(16);
const SPRD_DMA_GLB_TRG_OFFSET: u32 = 16;
const SPRD_DMA_GLB_DEST_CHN_MASK: u32 = genmask(13,8);
const SPRD_DMA_GLB_DEST_CHN_OFFSET: u32 = 8;
const SPRD_DMA_GLB_SRC_CHN_MASK: u32 = genmask(5,0);
const SPRD_DMA_INT_MASK: u32 = genmask(4,0);
const SPRD_DMA_INT_CLR_OFFSET: u32 = 24;
const SPRD_DMA_FRAG_INT_EN: u32 = bit(0);
const SPRD_DMA_BLK_INT_EN: u32 = bit(1);
const SPRD_DMA_TRANS_INT_EN: u32 = bit(2);
const SPRD_DMA_LIST_INT_EN: u32 = bit(3);
const SPRD_DMA_CFG_ERR_INT_EN: u32 = bit(4);
const SPRD_DMA_CHN_EN: u32 = bit(0);
const SPRD_DMA_LINKLIST_EN: u32 = bit(4);
const SPRD_DMA_WAIT_BDONE_OFFSET: u32 = 24;
const SPRD_DMA_DONOT_WAIT_BDONE: u32 = 1;
const SPRD_DMA_REQ_EN: u32 = bit(0);
const SPRD_DMA_PAUSE_EN: u32 = bit(0);
const SPRD_DMA_PAUSE_STS: u32 = bit(2);
const SPRD_DMA_PAUSE_CNT: u32 = 0x2000;
const SPRD_DMA_HIGH_ADDR_MASK: u32 = genmask(31,28);
const SPRD_DMA_LOW_ADDR_MASK: u32 = genmask(31,0);
const SPRD_DMA_WRAP_ADDR_MASK: u32 = genmask(27,0);
const SPRD_DMA_HIGH_ADDR_OFFSET: u32 = 4;
const SPRD_DMA_FRAG_INT_STS: u32 = bit(16);
const SPRD_DMA_BLK_INT_STS: u32 = bit(17);
const SPRD_DMA_TRSC_INT_STS: u32 = bit(18);
const SPRD_DMA_LIST_INT_STS: u32 = bit(19);
const SPRD_DMA_CFGERR_INT_STS: u32 = bit(20);
const SPRD_DMA_CHN_INT_STS: u32 = SPRD_DMA_FRAG_INT_STS | SPRD_DMA_BLK_INT_STS | SPRD_DMA_TRSC_INT_STS | SPRD_DMA_LIST_INT_STS | SPRD_DMA_CFGERR_INT_STS;
const SPRD_DMA_SRC_DATAWIDTH_OFFSET: u32 = 30;
const SPRD_DMA_DES_DATAWIDTH_OFFSET: u32 = 28;
const SPRD_DMA_FIX_SEL_OFFSET: u32 = 21;
const SPRD_DMA_FIX_EN_OFFSET: u32 = 20;
const SPRD_DMA_FRG_LEN_MASK: u32 = genmask(16,0);
const SPRD_DMA_BLK_LEN_MASK: u32 = genmask(16,0);
const SPRD_DMA_TRSC_LEN_MASK: u32 = genmask(27,0);
const SPRD_DMA_DEST_TRSF_STEP_OFFSET: u32 = 16;
const SPRD_DMA_TRSF_STEP_MASK: u32 = genmask(15,0);
const SPRD_DMA_LLIST_HIGH_MASK: u32 = genmask(31,28);
const SPRD_DMA_LLIST_HIGH_SHIFT: u32 = 28;
const SPRD_DMA_NONE_STEP: u32 = 0;
const SPRD_DMA_BYTE_STEP: u32 = 1;
const SPRD_DMA_SHORT_STEP: u32 = 2;
const SPRD_DMA_WORD_STEP: u32 = 4;
const SPRD_DMA_DWORD_STEP: u32 = 8;
const SPRD_DMA_SOFTWARE_UID: u32 = 0;

#[repr(C)]
pub struct sprd_dma_chn_hw { pub pause:u32,pub r#req:u32,pub cfg:u32,pub intc:u32,pub src_addr:u32,pub des_addr:u32,pub frg_len:u32,pub blk_len:u32,pub trsc_len:u32,pub trsf_step:u32,pub wrap_ptr:u32,pub wrap_to:u32,pub llist_ptr:u32,pub frg_step:u32,pub src_blk_step:u32,pub des_blk_step:u32 }

/* The following declarations preserve the implementation interface.  Kernel
 * structures and helpers come from the corresponding Linux headers. */
extern "C" {
    fn sprd_dma_free_desc(vd: *mut core::ffi::c_void);
    fn sprd_dma_filter_fn(chan: *mut core::ffi::c_void, param: *mut core::ffi::c_void) -> bool;
}

/* File-local low-level helpers, expressed with raw pointers to preserve the
 * original MMIO and container-of behavior. */
unsafe fn sprd_dma_glb_update(base: *mut u8, reg:u32, mask:u32, val:u32) { let p=base.add(reg as usize) as *mut u32; let orig=core::ptr::read_volatile(p); core::ptr::write_volatile(p,(orig & !mask)|val); }
unsafe fn sprd_dma_chn_update(base: *mut u8, reg:u32, mask:u32, val:u32) { let p=base.add(reg as usize) as *mut u32; let orig=core::ptr::read_volatile(p); core::ptr::write_volatile(p,(orig & !mask)|val); }

/* Direct translations of the remaining driver operations are kept as C-ABI
 * entry points so external kernel wiring can provide the referenced types. */
#[no_mangle] pub unsafe extern "C" fn sprd_dma_get_src_addr(base:*mut u8)->u64 { let a=core::ptr::read_volatile(base.add(SPRD_DMA_CHN_SRC_ADDR as usize) as *const u32) as u64; let h=(core::ptr::read_volatile(base.add(SPRD_DMA_CHN_WARP_PTR as usize) as *const u32)&SPRD_DMA_HIGH_ADDR_MASK) as u64; a | (h << SPRD_DMA_HIGH_ADDR_OFFSET) }
#[no_mangle] pub unsafe extern "C" fn sprd_dma_get_dst_addr(base:*mut u8)->u64 { let a=core::ptr::read_volatile(base.add(SPRD_DMA_CHN_DES_ADDR as usize) as *const u32) as u64; let h=(core::ptr::read_volatile(base.add(SPRD_DMA_CHN_WARP_TO as usize) as *const u32)&SPRD_DMA_HIGH_ADDR_MASK) as u64; a | (h << SPRD_DMA_HIGH_ADDR_OFFSET) }

/* Remaining kernel-facing functions and driver registration are declarations;
 * their bodies depend on Linux DMA, virt-dma, OF, PM, and platform APIs. */
extern "C" {
    fn sprd_dma_enable(sdev:*mut core::ffi::c_void)->i32;
    fn sprd_dma_disable(sdev:*mut core::ffi::c_void);
    fn sprd_dma_start(schan:*mut core::ffi::c_void);
    fn sprd_dma_stop(schan:*mut core::ffi::c_void);
    fn sprd_dma_probe(pdev:*mut core::ffi::c_void)->i32;
    fn sprd_dma_remove(pdev:*mut core::ffi::c_void);
}

/* MODULE_LICENSE("GPL v2"); MODULE_DESCRIPTION("DMA driver for Spreadtrum");
 * MODULE_AUTHOR("Baolin Wang <baolin.wang@spreadtrum.com>");
 * MODULE_AUTHOR("Eric Long <eric.long@spreadtrum.com>"); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
