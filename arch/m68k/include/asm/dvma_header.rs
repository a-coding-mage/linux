/* SPDX-License-Identifier: GPL-2.0 */
/* DMA_RESET and IRQ entry/exit retain the original operations through these low-level declarations. */
/* Translation of include/asm-m68k/dma.h. */

pub const DVMA_PAGE_SHIFT: usize = 13;
pub const DVMA_PAGE_SIZE: usize = 1usize << DVMA_PAGE_SHIFT;
pub const DVMA_PAGE_MASK: usize = !(DVMA_PAGE_SIZE - 1);

#[inline]
pub const fn dvma_page_align(addr: usize) -> usize {
    (addr + DVMA_PAGE_SIZE - 1) & DVMA_PAGE_MASK
}

extern "C" {
    pub fn dvma_init();
    pub fn dvma_map_iommu(kaddr: usize, baddr: usize, len: i32) -> i32;
    pub fn dvma_map_align(kaddr: usize, len: i32, align: i32) -> usize;
    pub fn dvma_malloc_align(len: usize, align: usize) -> *mut core::ffi::c_void;
    pub fn dvma_unmap(baddr: *mut core::ffi::c_void);
    pub fn dvma_free(vaddr: *mut core::ffi::c_void);
}

#[inline]
pub unsafe fn dvma_malloc(x: usize) -> *mut core::ffi::c_void { dvma_malloc_align(x, 0) }
#[inline]
pub unsafe fn dvma_map(x: usize, y: i32) -> usize { dvma_map_align(x, y, 0) }
#[inline]
pub unsafe fn dvma_map_vme(x: usize, y: i32) -> usize { dvma_map(x, y) & 0xfffff }
#[inline]
pub unsafe fn dvma_map_align_vme(x: usize, y: i32, z: i32) -> usize {
    dvma_map_align(x, y, z) & 0xfffff
}

#[cfg(feature = "CONFIG_SUN3")]
pub const DVMA_PMEG_START: usize = 10;
#[cfg(feature = "CONFIG_SUN3")]
pub const DVMA_PMEG_END: usize = 16;
#[cfg(feature = "CONFIG_SUN3")]
pub const DVMA_START: usize = 0xf00000;
#[cfg(feature = "CONFIG_SUN3")]
pub const DVMA_END: usize = 0xfe0000;
#[cfg(feature = "CONFIG_SUN3")]
pub const DVMA_SIZE: usize = DVMA_END - DVMA_START;
#[cfg(feature = "CONFIG_SUN3")]
pub const IOMMU_TOTAL_ENTRIES: usize = 128;
#[cfg(feature = "CONFIG_SUN3")]
pub const IOMMU_ENTRIES: usize = 120;
#[cfg(feature = "CONFIG_SUN3")]
pub const DVMA_REGION_SIZE: usize = 0x10000;
#[cfg(feature = "CONFIG_SUN3")]
#[inline] pub const fn dvma_align(addr: usize) -> usize { (addr + DVMA_REGION_SIZE - 1) & !(DVMA_REGION_SIZE - 1) }
#[cfg(feature = "CONFIG_SUN3")]
#[inline] pub const fn dvma_vtop(x: usize) -> usize { x & 0xffffff }
#[cfg(feature = "CONFIG_SUN3")]
#[inline] pub const fn dvma_ptov(x: usize) -> usize { x | 0xf000000 }
#[cfg(feature = "CONFIG_SUN3")]
#[inline] pub const fn dvma_vtovme(x: usize) -> usize { x & 0x00fffff }
#[cfg(feature = "CONFIG_SUN3")]
#[inline] pub const fn dvma_vmetov(x: usize) -> usize { x | 0xff00000 }
#[cfg(feature = "CONFIG_SUN3")]
pub const dvma_vtob: fn(usize) -> usize = dvma_vtop;
#[cfg(feature = "CONFIG_SUN3")]
pub const dvma_btov: fn(usize) -> usize = dvma_ptov;

#[cfg(feature = "CONFIG_SUN3")]
extern "C" { pub fn sun3_dvma_init(); }
#[cfg(feature = "CONFIG_SUN3")]
#[inline] pub fn dvma_map_cpu(_kaddr: usize, _vaddr: usize, _len: i32) -> i32 { 0 }
#[cfg(feature = "CONFIG_SUN3")]
#[inline] pub fn dvma_unmap_iommu(_baddr: usize, _len: i32) {}

#[cfg(not(feature = "CONFIG_SUN3"))]
pub const DVMA_START: usize = 0;
#[cfg(not(feature = "CONFIG_SUN3"))]
pub const DVMA_END: usize = 0xf00000;
#[cfg(not(feature = "CONFIG_SUN3"))]
pub const DVMA_SIZE: usize = DVMA_END - DVMA_START;
#[cfg(not(feature = "CONFIG_SUN3"))]
pub const IOMMU_TOTAL_ENTRIES: usize = 2048;
#[cfg(not(feature = "CONFIG_SUN3"))]
pub const IOMMU_ENTRIES: usize = IOMMU_TOTAL_ENTRIES - 0x80;
#[cfg(not(feature = "CONFIG_SUN3"))]
#[inline] pub const fn dvma_vtob(x: usize) -> usize { x & 0x00ffffff }
#[cfg(not(feature = "CONFIG_SUN3"))]
#[inline] pub const fn dvma_btov(x: usize) -> usize { x | 0xff000000 }
#[cfg(not(feature = "CONFIG_SUN3"))]
#[inline] pub fn sun3_dvma_init() {}
#[cfg(not(feature = "CONFIG_SUN3"))]
extern "C" {
    pub fn dvma_map_cpu(kaddr: usize, vaddr: usize, len: i32) -> i32;
    pub fn dvma_unmap_iommu(baddr: usize, len: i32);
}

#[cfg(not(feature = "CONFIG_SUN3"))]
#[repr(C)]
pub struct sparc_dma_registers {
    pub cond_reg: usize,
    pub st_addr: usize,
    pub cnt: usize,
    pub dma_test: usize,
}

#[cfg(not(feature = "CONFIG_SUN3"))]
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum dvma_rev { dvmarev0, dvmaesc1, dvmarev1, dvmarev2, dvmarev3, dvmarevplus, dvmahme }

#[cfg(not(feature = "CONFIG_SUN3"))]
#[inline] pub const fn dma_hascount(rev: dvma_rev) -> bool { rev == dvma_rev::dvmaesc1 }

#[cfg(not(feature = "CONFIG_SUN3"))]
#[repr(C)]
pub struct Linux_SBus_DMA {
    pub next: *mut Linux_SBus_DMA,
    pub SBus_dev: *mut linux_sbus_device,
    pub regs: *mut sparc_dma_registers,
    pub node: i32,
    pub running: i32,
    pub allocated: i32,
    pub addr: usize,
    pub nbytes: i32,
    pub realbytes: i32,
    pub revision: dvma_rev,
}
#[cfg(not(feature = "CONFIG_SUN3"))]
extern "C" { pub static mut dma_chain: *mut Linux_SBus_DMA; }
#[cfg(not(feature = "CONFIG_SUN3"))]
extern "C" { pub type linux_sbus_device; }

/* Register constants and DMA helper macros. */
#[cfg(not(feature = "CONFIG_SUN3"))]
pub const DMA_DEVICE_ID: usize = 0xf0000000;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_VERS0: usize = 0;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_ESCV1: usize = 0x40000000;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_VERS1: usize = 0x80000000;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_VERS2: usize = 0xa0000000;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_VERHME: usize = 0xb0000000;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_VERSPLUS: usize = 0x90000000;
#[cfg(not(feature = "CONFIG_SUN3"))]
pub const DMA_HNDL_INTR: usize = 1;
#[cfg(not(feature = "CONFIG_SUN3"))]
pub const DMA_HNDL_ERROR: usize = 2;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_FIFO_STDRAIN: usize = 0x40;
#[cfg(not(feature = "CONFIG_SUN3"))]
pub const DMA_FIFO_ISDRAIN: usize = 0x0000000c;
#[cfg(not(feature = "CONFIG_SUN3"))]
pub const DMA_INT_ENAB: usize = 0x10;
#[cfg(not(feature = "CONFIG_SUN3"))]
pub const DMA_FIFO_INV: usize = 0x20;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_ACC_SZ_ERR: usize = 0x40;
#[cfg(not(feature = "CONFIG_SUN3"))]
pub const DMA_ST_WRITE: usize = 0x100;
#[cfg(not(feature = "CONFIG_SUN3"))]
pub const DMA_ENABLE: usize = 0x200;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_PEND_READ: usize = 0x400;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_ESC_BURST: usize = 0x800;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_READ_AHEAD: usize = 0x1800;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_DSBL_RD_DRN: usize = 0x1000;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_BCNT_ENAB: usize = 0x2000;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_TERM_CNTR: usize = 0x4000;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_CSR_DISAB: usize = 0x10000;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_SCSI_DISAB: usize = 0x20000;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_ADD_ENABLE: usize = 0x40000;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_BRST_SZ: usize = 0x000c0000;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_BRST64: usize = 0x80000;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_BRST32: usize = 0x40000;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_BRST16: usize = 0;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_BRST0: usize = 0x80000;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_ADDR_DISAB: usize = 0x100000;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_2CLKS: usize = 0x200000;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_CNTR_DISAB: usize = 0x00800000;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_AUTO_NADDR: usize = 0x01000000;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_SCSI_ON: usize = 0x02000000;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_LOADED_ADDR: usize = 0x04000000;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_LOADED_NADDR: usize = 0x08000000;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_BURST1: usize = 1;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_BURST2: usize = 2;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_BURST4: usize = 4;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_BURST8: usize = 8;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_BURST16: usize = 0x10;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_BURST32: usize = 0x20;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_BURST64: usize = 0x40;
#[cfg(not(feature = "CONFIG_SUN3"))] pub const DMA_BURSTBITS: usize = 0x7f;
#[cfg(not(feature = "CONFIG_SUN3"))]
pub const DMA_RST_SCSI: usize = 0x80;
#[cfg(not(feature = "CONFIG_SUN3"))]
pub const DMA_3CLKS: usize = 0x00400000;
#[cfg(not(feature = "CONFIG_SUN3"))]
#[inline] pub const fn dma_maxend(addr: usize) -> usize { 0x01000000 - (addr & 0x00ffffff) }
#[cfg(not(feature = "CONFIG_SUN3"))]
#[inline] pub unsafe fn dma_error_p(regs: *const sparc_dma_registers) -> usize { (*regs).cond_reg & DMA_HNDL_ERROR }
#[cfg(not(feature = "CONFIG_SUN3"))]
#[inline] pub unsafe fn dma_irq_p(regs: *const sparc_dma_registers) -> usize { (*regs).cond_reg & (DMA_HNDL_INTR | DMA_HNDL_ERROR) }
#[cfg(not(feature = "CONFIG_SUN3"))]
#[inline] pub unsafe fn dma_write_p(regs: *const sparc_dma_registers) -> usize { (*regs).cond_reg & DMA_ST_WRITE }
#[cfg(not(feature = "CONFIG_SUN3"))]
#[inline] pub unsafe fn dma_off(regs: *mut sparc_dma_registers) { (*regs).cond_reg &= !DMA_ENABLE; }
#[cfg(not(feature = "CONFIG_SUN3"))]
#[inline] pub unsafe fn dma_intsoff(regs: *mut sparc_dma_registers) { (*regs).cond_reg &= !DMA_INT_ENAB; }
#[cfg(not(feature = "CONFIG_SUN3"))]
#[inline] pub unsafe fn dma_intson(regs: *mut sparc_dma_registers) { (*regs).cond_reg |= DMA_INT_ENAB; }
#[cfg(not(feature = "CONFIG_SUN3"))]
#[inline] pub unsafe fn dma_puntfifo(regs: *mut sparc_dma_registers) { (*regs).cond_reg |= DMA_FIFO_INV; }
#[cfg(not(feature = "CONFIG_SUN3"))]
#[inline] pub unsafe fn dma_setstart(regs: *mut sparc_dma_registers, addr: usize) { (*regs).st_addr = addr; }
#[cfg(not(feature = "CONFIG_SUN3"))]
#[inline] pub unsafe fn dma_begindma_w(regs: *mut sparc_dma_registers) { (*regs).cond_reg |= DMA_ST_WRITE | DMA_ENABLE | DMA_INT_ENAB; }
#[cfg(not(feature = "CONFIG_SUN3"))]
#[inline] pub unsafe fn dma_begindma_r(regs: *mut sparc_dma_registers) { (*regs).cond_reg |= (DMA_ENABLE | DMA_INT_ENAB) & !DMA_ST_WRITE; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
