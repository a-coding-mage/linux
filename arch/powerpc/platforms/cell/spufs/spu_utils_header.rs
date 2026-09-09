/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * utils.h: Utilities for SPU-side of the context switch operation.
 *
 * (C) Copyright IBM 2005
 */

/* 64-bit safe EA. */
#[repr(C)]
pub union addr64 {
    pub ull: u64,
    pub ui: [u32; 2],
}

/* 128-bit register template. */
#[repr(C)]
pub union spu_reg128v {
    pub slot: [u32; 4],
    pub v: vector_unsigned_int,
}

/* DMA list structure. */
#[repr(C)]
pub struct dma_list_elem {
    pub size: u32,
    pub ea_low: u32,
}

/* Declare storage for 8-byte aligned DMA list. */
pub static mut dma_list: [dma_list_elem; 15] = [
    dma_list_elem { size: 0, ea_low: 0 },
    dma_list_elem { size: 0, ea_low: 0 },
    dma_list_elem { size: 0, ea_low: 0 },
    dma_list_elem { size: 0, ea_low: 0 },
    dma_list_elem { size: 0, ea_low: 0 },
    dma_list_elem { size: 0, ea_low: 0 },
    dma_list_elem { size: 0, ea_low: 0 },
    dma_list_elem { size: 0, ea_low: 0 },
    dma_list_elem { size: 0, ea_low: 0 },
    dma_list_elem { size: 0, ea_low: 0 },
    dma_list_elem { size: 0, ea_low: 0 },
    dma_list_elem { size: 0, ea_low: 0 },
    dma_list_elem { size: 0, ea_low: 0 },
    dma_list_elem { size: 0, ea_low: 0 },
    dma_list_elem { size: 0, ea_low: 0 },
];

/* External definition for storage declared in crt0. */
extern "C" {
    pub static mut regs_spill: [spu_reg128v; NR_SPU_SPILL_REGS];
}

/* Compute LSCSA byte offset for a given field. */
#[allow(unused_macros)]
macro_rules! LSCSA_BYTE_OFFSET {
    ($field:tt) => {
        (unsafe {
            (&(*(0 as *const spu_lscsa)).$field as *const _ as usize)
                - (&(*(0 as *const spu_lscsa)).gprs[0].slot[0] as *const _ as usize)
        })
    };
}

#[allow(unused_macros)]
macro_rules! LSCSA_QW_OFFSET {
    ($field:tt) => { LSCSA_BYTE_OFFSET!($field) >> 4 };
}

pub unsafe fn set_event_mask() {
    let event_mask: u32 = 0;
    spu_writech(SPU_WrEventMask, event_mask);
}

pub unsafe fn set_tag_mask() {
    let tag_mask: u32 = 1;
    spu_writech(MFC_WrTagMask, tag_mask);
}

pub unsafe fn build_dma_list(lscsa_ea: addr64) {
    let mut ea_low: u32 = lscsa_ea.ui[1];
    ea_low = ea_low.wrapping_add(LSCSA_BYTE_OFFSET!(ls[16384]) as u32);

    for i in 0..15 {
        dma_list[i].size = 16384;
        dma_list[i].ea_low = ea_low;
        ea_low = ea_low.wrapping_add(16384);
    }
}

pub unsafe fn enqueue_putllc(lscsa_ea: addr64) {
    let ls: u32 = 0;
    let size: u32 = 128;
    let tag_id: u32 = 0;
    let cmd: u32 = 0xB4; /* PUTLLC */

    spu_writech(MFC_LSA, ls);
    spu_writech(MFC_EAH, lscsa_ea.ui[0]);
    spu_writech(MFC_EAL, lscsa_ea.ui[1]);
    spu_writech(MFC_Size, size);
    spu_writech(MFC_TagID, tag_id);
    spu_writech(MFC_Cmd, cmd);
}

pub unsafe fn set_tag_update() {
    let update_any: u32 = 1;
    spu_writech(MFC_WrTagUpdate, update_any);
}

pub unsafe fn read_tag_status() {
    spu_readch(MFC_RdTagStat);
}

pub unsafe fn read_llar_status() {
    spu_readch(MFC_RdAtomicStat);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
