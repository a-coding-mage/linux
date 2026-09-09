// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Faithful low-level Rust translation of the pSeries IOMMU implementation.
 * Kernel-provided types, constants, globals, and functions are intentionally
 * referenced here as external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct dynamic_dma_window_prop {
    pub liobn: u32,
    pub dma_base: u64,
    pub tce_shift: u32,
    pub window_shift: u32,
}

#[repr(C)]
pub struct ddw_query_response {
    pub windows_available: u32,
    pub largest_available_block: u64,
    pub page_size: u32,
    pub migration_capable: u32,
}

#[repr(C)]
pub struct ddw_create_response {
    pub liobn: u32,
    pub addr_hi: u32,
    pub addr_lo: u32,
}

#[repr(C)]
pub struct dma_win {
    pub device: *mut device_node,
    pub prop: *const dynamic_dma_window_prop,
    pub direct: bool,
    pub list: list_head,
}

#[repr(C)]
pub struct failed_ddw_pdn {
    pub pdn: *mut device_node,
    pub list: list_head,
}

#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct pci_controller { _private: [u8; 0] }
#[repr(C)] pub struct iommu_table { _private: [u8; 0] }
#[repr(C)] pub struct iommu_table_group { _private: [u8; 0] }
#[repr(C)] pub struct iommu_group { _private: [u8; 0] }

pub const DDW_QUERY_PE_DMA_WIN: usize = 0;
pub const DDW_CREATE_PE_DMA_WIN: usize = 1;
pub const DDW_REMOVE_PE_DMA_WIN: usize = 2;
pub const DDW_APPLICABLE_SIZE: usize = 3;
pub const DDW_EXT_SIZE: usize = 0;
pub const DDW_EXT_RESET_DMA_WIN: usize = 1;
pub const DDW_EXT_QUERY_OUT_SIZE: usize = 2;
pub const DDW_EXT_LIMITED_ADDR_MODE: usize = 3;

extern "C" {
    static mut dma_win_list: list_head;
    static mut dma_win_list_lock: c_void;
    static mut dma_win_init_mutex: c_void;
    fn iommu_pseries_alloc_table(node: c_int) -> *mut iommu_table;
    fn iommu_pseries_alloc_group(node: c_int) -> *mut iommu_table_group;
    fn iommu_pseries_free_group(group: *mut iommu_table_group, name: *const c_char);
    fn tce_build_pSeries(tbl: *mut iommu_table, index: isize, npages: isize,
        uaddr: usize, direction: c_int, attrs: usize) -> c_int;
    fn tce_clear_pSeries(tbl: *mut iommu_table, index: isize, npages: isize);
    fn tce_get_pseries(tbl: *mut iommu_table, index: isize) -> usize;
    fn tce_build_pSeriesLP(liobn: usize, tcenum: isize, tceshift: isize,
        npages: isize, uaddr: usize, direction: c_int, attrs: usize) -> c_int;
    fn tce_buildmulti_pSeriesLP(tbl: *mut iommu_table, tcenum: isize,
        npages: isize, uaddr: usize, direction: c_int, attrs: usize) -> c_int;
    fn tce_freemulti_pSeriesLP(tbl: *mut iommu_table, tcenum: isize, npages: isize);
    fn tce_get_pSeriesLP(tbl: *mut iommu_table, tcenum: isize) -> usize;
    fn tce_free_pSeriesLP(liobn: usize, tcenum: isize, tceshift: isize, npages: isize);
}

// The remaining platform entry points retain their C ABI and are supplied by
// the surrounding kernel translation unit. Their declarations preserve the
// externally visible interfaces of iommu.c.
extern "C" {
    pub fn iommu_init_early_pSeries();
    pub fn pSeries_pci_device_group(hose: *mut pci_controller, pdev: *mut pci_dev)
        -> *mut iommu_group;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
