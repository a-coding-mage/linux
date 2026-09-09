/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright (c) 2024 NVIDIA Corporation & Affiliates */

// Dependency: <linux/dma-mapping.h>

pub struct dma_iova_state;
pub struct pci_p2pdma_map_state;
pub struct device;

// `dma_addr_t` is supplied by the Linux DMA mapping dependency.
pub type dma_addr_t = usize;

/*
 * struct hmm_dma_map - array of PFNs and DMA addresses
 *
 * @state: DMA IOVA state
 * @pfns: array of PFNs
 * @dma_list: array of DMA addresses
 * @dma_entry_size: size of each DMA entry in the array
 */
#[repr(C)]
pub struct hmm_dma_map {
    pub state: dma_iova_state,
    pub pfn_list: *mut ::core::ffi::c_ulong,
    pub dma_list: *mut dma_addr_t,
    pub dma_entry_size: usize,
}

unsafe extern "C" {
    pub fn hmm_dma_map_alloc(
        dev: *mut device,
        map: *mut hmm_dma_map,
        nr_entries: usize,
        dma_entry_size: usize,
    ) -> ::core::ffi::c_int;

    pub fn hmm_dma_map_free(dev: *mut device, map: *mut hmm_dma_map);

    pub fn hmm_dma_map_pfn(
        dev: *mut device,
        map: *mut hmm_dma_map,
        idx: usize,
        p2pdma_state: *mut pci_p2pdma_map_state,
    ) -> dma_addr_t;

    pub fn hmm_dma_unmap_pfn(
        dev: *mut device,
        map: *mut hmm_dma_map,
        idx: usize,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
