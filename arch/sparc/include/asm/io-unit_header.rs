/* SPDX-License-Identifier: GPL-2.0 */
/* io-unit.h: Definitions for the sun4d IO-UNIT.
 *
 * Copyright (C) 1997,1998 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
 */

/* The io-unit handles all virtual to physical address translations
 * that occur between the SBUS and physical memory.  Access by the
 * cpu to IO registers and similar go over the xdbus so are
 * translated by the on chip SRMMU.  The io-unit and the srmmu do
 * not need to have the same translations at all, in fact most
 * of the time the translations they handle are a disjunct set.
 * Basically the io-unit handles all dvma sbus activity.
 */

/* AIEEE, unlike the nice sun4m, these monsters have
 * fixed DMA range 64M
 */

pub const IOUNIT_DMA_BASE: u32 = 0xfc000000; /* TOP - 64M */
pub const IOUNIT_DMA_SIZE: u32 = 0x04000000; /* 64M */
/* We use last 1M for sparc_dvma_malloc */
pub const IOUNIT_DVMA_SIZE: u32 = 0x00100000; /* 1M */

/* The format of an iopte in the external page tables */
pub const IOUPTE_PAGE: u32 = 0xffffff00; /* Physical page number (PA[35:12]) */
pub const IOUPTE_CACHE: u32 = 0x00000080; /* Cached (in Viking/MXCC) */
/* XXX Jakub, find out how to program SBUS streaming cache on XDBUS/sun4d.
 * XXX Actually, all you should need to do is find out where the registers
 * XXX are and copy over the sparc64 implementation I wrote.  There may be
 * XXX some horrible hwbugs though, so be careful.  -DaveM
 */
pub const IOUPTE_STREAM: u32 = 0x00000040; /* Translation can use streaming cache */
pub const IOUPTE_INTRA: u32 = 0x00000008; /* SBUS direct slot->slot transfer */
pub const IOUPTE_WRITE: u32 = 0x00000004; /* Writeable */
pub const IOUPTE_VALID: u32 = 0x00000002; /* IOPTE is valid */
pub const IOUPTE_PARITY: u32 = 0x00000001; /* Parity is checked during DVMA */

#[repr(C)]
pub struct iounit_struct {
    pub bmap: [core::ffi::c_ulong;
        ((IOUNIT_DMA_SIZE >> (PAGE_SHIFT + 3)) as usize) /
            core::mem::size_of::<core::ffi::c_ulong>()],
    pub lock: spinlock_t,
    /* __iomem */
    pub page_table: *mut iopte_t,
    pub rotor: [core::ffi::c_ulong; 3],
    pub limit: [core::ffi::c_ulong; 4],
}

pub const IOUNIT_BMAP1_START: u32 = 0x00000000;
pub const IOUNIT_BMAP1_END: u32 = IOUNIT_DMA_SIZE >> (PAGE_SHIFT + 1);
pub const IOUNIT_BMAP2_START: u32 = IOUNIT_BMAP1_END;
pub const IOUNIT_BMAP2_END: u32 = IOUNIT_BMAP2_START +
    (IOUNIT_DMA_SIZE >> (PAGE_SHIFT + 2));
pub const IOUNIT_BMAPM_START: u32 = IOUNIT_BMAP2_END;
pub const IOUNIT_BMAPM_END: u32 = (IOUNIT_DMA_SIZE - IOUNIT_DVMA_SIZE) >> PAGE_SHIFT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
