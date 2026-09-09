/* SPDX-License-Identifier: GPL-2.0+ */
// Copyright 2017 IBM Corp.

// Dependencies supplied by the surrounding kernel translation include
// linux/bitfield.h and linux/pci.h.

pub const PNV_OCXL_TL_MAX_TEMPLATE: u32 = 63;
pub const PNV_OCXL_TL_BITS_PER_RATE: u32 = 4;
pub const PNV_OCXL_TL_RATE_BUF_SIZE: u32 =
    (PNV_OCXL_TL_MAX_TEMPLATE + 1) * PNV_OCXL_TL_BITS_PER_RATE / 8;

pub const PNV_OCXL_ATSD_TIMEOUT: u32 = 1;

// TLB Management Instructions
pub const PNV_OCXL_ATSD_LNCH: u32 = 0x00;
// Radix Invalidate
pub const PNV_OCXL_ATSD_LNCH_R: u64 = 1u64 << (63 - 0);
// Radix Invalidation Control
// 0b00 Just invalidate TLB.
// 0b01 Invalidate just Page Walk Cache.
// 0b10 Invalidate TLB, Page Walk Cache, and any
// caching of Partition and Process Table Entries.
pub const PNV_OCXL_ATSD_LNCH_RIC: u64 = 0x3u64 << (63 - 2);
// Number and Page Size of translations to be invalidated
pub const PNV_OCXL_ATSD_LNCH_LP: u64 = 0xffu64 << (63 - 10);
// Invalidation Criteria
// 0b00 Invalidate just the target VA.
// 0b01 Invalidate matching PID.
pub const PNV_OCXL_ATSD_LNCH_IS: u64 = 0x3u64 << (63 - 12);
// 0b1: Process Scope, 0b0: Partition Scope
pub const PNV_OCXL_ATSD_LNCH_PRS: u64 = 1u64 << (63 - 13);
// Invalidation Flag
pub const PNV_OCXL_ATSD_LNCH_B: u64 = 1u64 << (63 - 14);
// Actual Page Size to be invalidated
// 000 4KB
// 101 64KB
// 001 2MB
// 010 1GB
pub const PNV_OCXL_ATSD_LNCH_AP: u64 = 0x7u64 << (63 - 17);
// Defines the large page select
// L=0b0 for 4KB pages
// L=0b1 for large pages)
pub const PNV_OCXL_ATSD_LNCH_L: u64 = 1u64 << (63 - 18);
// Process ID
pub const PNV_OCXL_ATSD_LNCH_PID: u64 = 0xfffffu64 << (63 - 38);
// NoFlush – Assumed to be 0b0
pub const PNV_OCXL_ATSD_LNCH_F: u64 = 1u64 << (63 - 39);
pub const PNV_OCXL_ATSD_LNCH_OCAPI_SLBI: u64 = 1u64 << (63 - 40);
pub const PNV_OCXL_ATSD_LNCH_OCAPI_SINGLETON: u64 = 1u64 << (63 - 41);
pub const PNV_OCXL_ATSD_AVA: u32 = 0x08;
pub const PNV_OCXL_ATSD_AVA_AVA: u64 = ((1u64 << 52) - 1) << (63 - 51);
pub const PNV_OCXL_ATSD_STAT: u32 = 0x10;

extern "C" {
    pub fn pnv_ocxl_get_actag(dev: *mut pci_dev, base: *mut u16, enabled: *mut u16, supported: *mut u16) -> i32;
    pub fn pnv_ocxl_get_pasid_count(dev: *mut pci_dev, count: *mut i32) -> i32;
    pub fn pnv_ocxl_get_tl_cap(dev: *mut pci_dev, cap: *mut i64, rate_buf: *mut i8, rate_buf_size: i32) -> i32;
    pub fn pnv_ocxl_set_tl_conf(dev: *mut pci_dev, cap: i64, rate_buf_phys: u64, rate_buf_size: i32) -> i32;
    pub fn pnv_ocxl_get_xsl_irq(dev: *mut pci_dev, hwirq: *mut i32) -> i32;
    pub fn pnv_ocxl_unmap_xsl_regs(dsisr: *mut core::ffi::c_void, dar: *mut core::ffi::c_void, tfc: *mut core::ffi::c_void, pe_handle: *mut core::ffi::c_void);
    pub fn pnv_ocxl_map_xsl_regs(dev: *mut pci_dev, dsisr: *mut *mut core::ffi::c_void, dar: *mut *mut core::ffi::c_void, tfc: *mut *mut core::ffi::c_void, pe_handle: *mut *mut core::ffi::c_void) -> i32;
    pub fn pnv_ocxl_spa_setup(dev: *mut pci_dev, spa_mem: *mut core::ffi::c_void, pe_mask: i32, platform_data: *mut *mut core::ffi::c_void) -> i32;
    pub fn pnv_ocxl_spa_release(platform_data: *mut core::ffi::c_void);
    pub fn pnv_ocxl_spa_remove_pe_from_cache(platform_data: *mut core::ffi::c_void, pe_handle: i32) -> i32;
    pub fn pnv_ocxl_map_lpar(dev: *mut pci_dev, lparid: u64, lpcr: u64, arva: *mut *mut core::ffi::c_void) -> i32;
    pub fn pnv_ocxl_unmap_lpar(arva: *mut core::ffi::c_void);
    pub fn pnv_ocxl_tlb_invalidate(arva: *mut core::ffi::c_void, pid: usize, addr: usize, page_size: usize);
}

// Supplied by linux/pci.h.
#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
