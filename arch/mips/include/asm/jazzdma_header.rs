/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Helpfile for jazzdma.c -- Mips Jazz R4030 DMA controller support
 */

/* Prototypes and macros */
extern "C" {
    pub fn vdma_alloc(paddr: usize, size: usize) -> usize;
    pub fn vdma_free(laddr: usize) -> i32;
    pub fn vdma_phys2log(paddr: usize) -> usize;
    pub fn vdma_log2phys(laddr: usize) -> usize;
    pub fn vdma_stats(); /* for debugging only */

    pub fn vdma_enable(channel: i32);
    pub fn vdma_disable(channel: i32);
    pub fn vdma_set_mode(channel: i32, mode: i32);
    pub fn vdma_set_addr(channel: i32, addr: isize);
    pub fn vdma_set_count(channel: i32, count: i32);
    pub fn vdma_get_residue(channel: i32) -> i32;
    pub fn vdma_get_enable(channel: i32) -> i32;
}

/* some definitions used by the driver functions */
pub const VDMA_PAGESIZE: usize = 4096;
pub const VDMA_PGTBL_ENTRIES: usize = 4096;
pub const VDMA_PGTBL_SIZE: usize = core::mem::size_of::<VDMA_PGTBL_ENTRY>() * VDMA_PGTBL_ENTRIES;
pub const VDMA_PAGE_EMPTY: u32 = 0xff000000;

/*
 * Macros to get page no. and offset of a given address
 * Note that VDMA_PAGE() works for physical addresses only
 */
#[macro_export]
macro_rules! VDMA_PAGE {
    ($a:expr) => {
        (($a as u32) >> 12)
    };
}

#[macro_export]
macro_rules! VDMA_OFFSET {
    ($a:expr) => {
        (($a as u32) & (VDMA_PAGESIZE as u32 - 1))
    };
}

/* VDMA pagetable entry description */
#[repr(C)]
pub struct VDMA_PGTBL_ENTRY {
    pub frame: u32, /* physical frame no. */
    pub owner: u32, /* owner of this entry (0=free) */
}

/*
 * DMA channel control registers
 * in the R4030 MCT_ADR chip
 */
pub const JAZZ_R4030_CHNL_MODE: u32 = 0xE0000100; /* 8 DMA Channel Mode Registers, */
                                                  /* 0xE0000100,120,140... */
pub const JAZZ_R4030_CHNL_ENABLE: u32 = 0xE0000108; /* 8 DMA Channel Enable Regs, */
                                                    /* 0xE0000108,128,148... */
pub const JAZZ_R4030_CHNL_COUNT: u32 = 0xE0000110; /* 8 DMA Channel Byte Cnt Regs, */
                                                   /* 0xE0000110,130,150... */
pub const JAZZ_R4030_CHNL_ADDR: u32 = 0xE0000118; /* 8 DMA Channel Address Regs, */
                                                  /* 0xE0000118,138,158... */

/* channel enable register bits */
pub const R4030_CHNL_ENABLE: u32 = 1 << 0;
pub const R4030_CHNL_WRITE: u32 = 1 << 1;
pub const R4030_TC_INTR: u32 = 1 << 8;
pub const R4030_MEM_INTR: u32 = 1 << 9;
pub const R4030_ADDR_INTR: u32 = 1 << 10;

/* Channel mode register bits */
pub const R4030_MODE_ATIME_40: u32 = 0;
pub const R4030_MODE_ATIME_80: u32 = 1;
pub const R4030_MODE_ATIME_120: u32 = 2;
pub const R4030_MODE_ATIME_160: u32 = 3;
pub const R4030_MODE_ATIME_200: u32 = 4;
pub const R4030_MODE_ATIME_240: u32 = 5;
pub const R4030_MODE_ATIME_280: u32 = 6;
pub const R4030_MODE_ATIME_320: u32 = 7;
pub const R4030_MODE_WIDTH_8: u32 = 1 << 3; /* device data bus width */
pub const R4030_MODE_WIDTH_16: u32 = 2 << 3;
pub const R4030_MODE_WIDTH_32: u32 = 3 << 3;
pub const R4030_MODE_INTR_EN: u32 = 1 << 5;
pub const R4030_MODE_BURST: u32 = 1 << 6; /* Rev. 2 only */
pub const R4030_MODE_FAST_ACK: u32 = 1 << 7; /* Rev. 2 only */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
