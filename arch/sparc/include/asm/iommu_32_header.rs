/* SPDX-License-Identifier: GPL-2.0 */
/* iommu.h: Definitions for the sun4m IOMMU.
 *
 * Copyright (C) 1996 David S. Miller (davem@caip.rutgers.edu)
 */

/* Dependencies supplied by the surrounding translation unit: page constants,
 * bit-map and IOPTE types, and the SBUS write helper. */

#[repr(C)]
pub struct iommu_regs {
    /* First page */
    pub control: u32,             /* IOMMU control */
    pub base: u32,                /* Physical base of iopte page table */
    pub _unused1: [u32; 3],
    pub tlbflush: u32,            /* write only */
    pub pageflush: u32,           /* write only */
    pub _unused2: [u32; 1017],
    /* Second page */
    pub afsr: u32,                /* Async-fault status register */
    pub afar: u32,                /* Async-fault physical address */
    pub _unused3: [u32; 2],
    pub sbuscfg0: u32,            /* SBUS configuration registers, per-slot */
    pub sbuscfg1: u32,
    pub sbuscfg2: u32,
    pub sbuscfg3: u32,
    pub mfsr: u32,                /* Memory-fault status register */
    pub mfar: u32,                /* Memory-fault physical address */
    pub _unused4: [u32; 1014],
    /* Third page */
    pub mid: u32,                 /* IOMMU module-id */
}

pub const IOMMU_CTRL_IMPL: u32 = 0xf0000000;
pub const IOMMU_CTRL_VERS: u32 = 0x0f000000;
pub const IOMMU_CTRL_RNGE: u32 = 0x0000001c;
pub const IOMMU_RNGE_16MB: u32 = 0x00000000;
pub const IOMMU_RNGE_32MB: u32 = 0x00000004;
pub const IOMMU_RNGE_64MB: u32 = 0x00000008;
pub const IOMMU_RNGE_128MB: u32 = 0x0000000c;
pub const IOMMU_RNGE_256MB: u32 = 0x00000010;
pub const IOMMU_RNGE_512MB: u32 = 0x00000014;
pub const IOMMU_RNGE_1GB: u32 = 0x00000018;
pub const IOMMU_RNGE_2GB: u32 = 0x0000001c;
pub const IOMMU_CTRL_ENAB: u32 = 0x00000001;

pub const IOMMU_AFSR_ERR: u32 = 0x80000000;
pub const IOMMU_AFSR_LE: u32 = 0x40000000;
pub const IOMMU_AFSR_TO: u32 = 0x20000000;
pub const IOMMU_AFSR_BE: u32 = 0x10000000;
pub const IOMMU_AFSR_SIZE: u32 = 0x0e000000;
pub const IOMMU_AFSR_S: u32 = 0x01000000;
pub const IOMMU_AFSR_RESV: u32 = 0x00f00000;
pub const IOMMU_AFSR_ME: u32 = 0x00080000;
pub const IOMMU_AFSR_RD: u32 = 0x00040000;
pub const IOMMU_AFSR_FAV: u32 = 0x00020000;

pub const IOMMU_SBCFG_SAB30: u32 = 0x00010000;
pub const IOMMU_SBCFG_BA16: u32 = 0x00000004;
pub const IOMMU_SBCFG_BA8: u32 = 0x00000002;
pub const IOMMU_SBCFG_BYPASS: u32 = 0x00000001;

pub const IOMMU_MFSR_ERR: u32 = 0x80000000;
pub const IOMMU_MFSR_S: u32 = 0x01000000;
pub const IOMMU_MFSR_CPU: u32 = 0x00800000;
pub const IOMMU_MFSR_ME: u32 = 0x00080000;
pub const IOMMU_MFSR_PERR: u32 = 0x00006000;
pub const IOMMU_MFSR_BM: u32 = 0x00001000;
pub const IOMMU_MFSR_C: u32 = 0x00000800;
pub const IOMMU_MFSR_RTYP: u32 = 0x000000f0;

pub const IOMMU_MID_SBAE: u32 = 0x001f0000;
pub const IOMMU_MID_SE: u32 = 0x00100000;
pub const IOMMU_MID_SB3: u32 = 0x00080000;
pub const IOMMU_MID_SB2: u32 = 0x00040000;
pub const IOMMU_MID_SB1: u32 = 0x00020000;
pub const IOMMU_MID_SB0: u32 = 0x00010000;
pub const IOMMU_MID_MID: u32 = 0x0000000f;

pub const IOPTE_PAGE: u32 = 0x07ffff00;
pub const IOPTE_CACHE: u32 = 0x00000080;
pub const IOPTE_WRITE: u32 = 0x00000004;
pub const IOPTE_VALID: u32 = 0x00000002;
pub const IOPTE_WAZ: u32 = 0x00000001;

#[repr(C)]
pub struct iommu_struct {
    pub regs: *mut iommu_regs,
    pub page_table: *mut iopte_t,
    /* For convenience */
    pub start: u32,
    pub end: u32,
    pub usemap: bit_map,
}

extern "C" {
    fn sbus_writel(value: u32, address: *mut u32);
}

pub unsafe fn iommu_invalidate(regs: *mut iommu_regs) {
    sbus_writel(0, core::ptr::addr_of_mut!((*regs).tlbflush));
}

pub unsafe fn iommu_invalidate_page(regs: *mut iommu_regs, ba: u32) {
    sbus_writel(ba & PAGE_MASK, core::ptr::addr_of_mut!((*regs).pageflush));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
