/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2018 Christoph Hellwig.
 *
 * DMA operations that map physical memory directly without using an IOMMU.
 */

// Dependencies: linux/dma-direct.h and linux/memremap.h.
use std::os::raw::{c_int, c_ulong};

extern "C" {
    pub fn dma_direct_get_sgtable(
        dev: *mut device,
        sgt: *mut sg_table,
        cpu_addr: *mut core::ffi::c_void,
        dma_addr: dma_addr_t,
        size: usize,
        attrs: c_ulong,
    ) -> c_int;
    pub fn dma_direct_can_mmap(dev: *mut device) -> bool;
    pub fn dma_direct_mmap(
        dev: *mut device,
        vma: *mut vm_area_struct,
        cpu_addr: *mut core::ffi::c_void,
        dma_addr: dma_addr_t,
        size: usize,
        attrs: c_ulong,
    ) -> c_int;
    pub fn dma_direct_map_phys(
        dev: *mut device,
        phys: phys_addr_t,
        size: usize,
        dir: dma_data_direction,
        attrs: c_ulong,
        flush: bool,
    ) -> dma_addr_t;
    pub fn dma_direct_need_sync(dev: *mut device, dma_addr: dma_addr_t) -> bool;
    pub fn dma_direct_map_sg(
        dev: *mut device,
        sgl: *mut scatterlist,
        nents: c_int,
        dir: dma_data_direction,
        attrs: c_ulong,
    ) -> c_int;
    pub fn dma_direct_all_ram_mapped(dev: *mut device) -> bool;
    pub fn dma_direct_max_mapping_size(dev: *mut device) -> usize;
}

// CONFIG_ARCH_HAS_SYNC_DMA_FOR_DEVICE || CONFIG_SWIOTLB
extern "C" {
    pub fn dma_direct_sync_sg_for_device(
        dev: *mut device,
        sgl: *mut scatterlist,
        nents: c_int,
        dir: dma_data_direction,
    );
}

// CONFIG_ARCH_HAS_SYNC_DMA_FOR_CPU || CONFIG_ARCH_HAS_SYNC_DMA_FOR_CPU_ALL || CONFIG_SWIOTLB
extern "C" {
    pub fn dma_direct_unmap_sg(
        dev: *mut device,
        sgl: *mut scatterlist,
        nents: c_int,
        dir: dma_data_direction,
        attrs: c_ulong,
    );
    pub fn dma_direct_sync_sg_for_cpu(
        dev: *mut device,
        sgl: *mut scatterlist,
        nents: c_int,
        dir: dma_data_direction,
    );
}

pub unsafe fn dma_direct_sync_single_for_device(
    dev: *mut device,
    addr: dma_addr_t,
    size: usize,
    dir: dma_data_direction,
) {
    let paddr: phys_addr_t = dma_to_phys(dev, addr);

    swiotlb_sync_single_for_device(dev, paddr, size, dir);

    if !dev_is_dma_coherent(dev) {
        arch_sync_dma_for_device(paddr, size, dir);
        arch_sync_dma_flush();
    }
}

pub unsafe fn dma_direct_sync_single_for_cpu(
    dev: *mut device,
    addr: dma_addr_t,
    size: usize,
    dir: dma_data_direction,
    flush: bool,
) {
    let paddr: phys_addr_t = dma_to_phys(dev, addr);

    if !dev_is_dma_coherent(dev) {
        arch_sync_dma_for_cpu(paddr, size, dir);
        if flush {
            arch_sync_dma_flush();
        }
        arch_sync_dma_for_cpu_all();
    }

    swiotlb_sync_single_for_cpu(dev, paddr, size, dir);
}

pub unsafe fn dma_direct_unmap_phys(
    dev: *mut device,
    addr: dma_addr_t,
    size: usize,
    dir: dma_data_direction,
    attrs: c_ulong,
    flush: bool,
) {
    let phys: phys_addr_t;

    if attrs & (DMA_ATTR_MMIO | DMA_ATTR_REQUIRE_COHERENT) != 0 {
        // nothing to do: uncached and no swiotlb
        return;
    }

    phys = dma_to_phys(dev, addr);
    if attrs & DMA_ATTR_SKIP_CPU_SYNC == 0 {
        dma_direct_sync_single_for_cpu(dev, addr, size, dir, flush);
    }

    swiotlb_tbl_unmap_single(dev, phys, size, dir, attrs | DMA_ATTR_SKIP_CPU_SYNC);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
