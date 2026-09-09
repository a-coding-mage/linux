/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2024, NVIDIA CORPORATION & AFFILIATES. All rights reserved
 *
 * DMA operations that map physical memory through IOMMU.
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/device.h, linux/dma-direction.h

#[cfg(CONFIG_IOMMU_DMA)]
#[inline]
pub unsafe fn use_dma_iommu(dev: *mut device) -> bool {
    dev_dma_iommu(dev)
}

#[cfg(not(CONFIG_IOMMU_DMA))]
#[inline]
pub unsafe fn use_dma_iommu(_dev: *mut device) -> bool {
    false
}

pub unsafe extern "C" fn iommu_dma_map_phys(
    dev: *mut device,
    phys: phys_addr_t,
    size: usize,
    dir: dma_data_direction,
    attrs: c_ulong,
) -> dma_addr_t;

pub unsafe extern "C" fn iommu_dma_unmap_phys(
    dev: *mut device,
    dma_handle: dma_addr_t,
    size: usize,
    dir: dma_data_direction,
    attrs: c_ulong,
);

pub unsafe extern "C" fn iommu_dma_map_sg(
    dev: *mut device,
    sg: *mut scatterlist,
    nents: c_int,
    dir: dma_data_direction,
    attrs: c_ulong,
) -> c_int;

pub unsafe extern "C" fn iommu_dma_unmap_sg(
    dev: *mut device,
    sg: *mut scatterlist,
    nents: c_int,
    dir: dma_data_direction,
    attrs: c_ulong,
);

pub unsafe extern "C" fn iommu_dma_alloc(
    dev: *mut device,
    size: usize,
    handle: *mut dma_addr_t,
    gfp: gfp_t,
    attrs: c_ulong,
) -> *mut c_void;

pub unsafe extern "C" fn iommu_dma_mmap(
    dev: *mut device,
    vma: *mut vm_area_struct,
    cpu_addr: *mut c_void,
    dma_addr: dma_addr_t,
    size: usize,
    attrs: c_ulong,
) -> c_int;

pub unsafe extern "C" fn iommu_dma_get_sgtable(
    dev: *mut device,
    sgt: *mut sg_table,
    cpu_addr: *mut c_void,
    dma_addr: dma_addr_t,
    size: usize,
    attrs: c_ulong,
) -> c_int;

pub unsafe extern "C" fn iommu_dma_get_merge_boundary(dev: *mut device) -> c_ulong;
pub unsafe extern "C" fn iommu_dma_opt_mapping_size() -> usize;
pub unsafe extern "C" fn iommu_dma_max_mapping_size(dev: *mut device) -> usize;

pub unsafe extern "C" fn iommu_dma_free(
    dev: *mut device,
    size: usize,
    cpu_addr: *mut c_void,
    handle: dma_addr_t,
    attrs: c_ulong,
);

pub unsafe extern "C" fn iommu_dma_alloc_noncontiguous(
    dev: *mut device,
    size: usize,
    dir: dma_data_direction,
    gfp: gfp_t,
    attrs: c_ulong,
) -> *mut sg_table;

pub unsafe extern "C" fn iommu_dma_free_noncontiguous(
    dev: *mut device,
    size: usize,
    sgt: *mut sg_table,
    dir: dma_data_direction,
);

pub unsafe extern "C" fn iommu_dma_vmap_noncontiguous(
    dev: *mut device,
    size: usize,
    sgt: *mut sg_table,
) -> *mut c_void;

#[inline]
pub unsafe fn iommu_dma_vunmap_noncontiguous(_dev: *mut device, vaddr: *mut c_void) {
    vunmap(vaddr);
}

pub unsafe extern "C" fn iommu_dma_mmap_noncontiguous(
    dev: *mut device,
    vma: *mut vm_area_struct,
    size: usize,
    sgt: *mut sg_table,
) -> c_int;

pub unsafe extern "C" fn iommu_dma_sync_single_for_cpu(
    dev: *mut device,
    dma_handle: dma_addr_t,
    size: usize,
    dir: dma_data_direction,
);

pub unsafe extern "C" fn iommu_dma_sync_single_for_device(
    dev: *mut device,
    dma_handle: dma_addr_t,
    size: usize,
    dir: dma_data_direction,
);

pub unsafe extern "C" fn iommu_dma_sync_sg_for_cpu(
    dev: *mut device,
    sgl: *mut scatterlist,
    nelems: c_int,
    dir: dma_data_direction,
);

pub unsafe extern "C" fn iommu_dma_sync_sg_for_device(
    dev: *mut device,
    sgl: *mut scatterlist,
    nelems: c_int,
    dir: dma_data_direction,
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
