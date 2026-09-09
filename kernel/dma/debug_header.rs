/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2008 Advanced Micro Devices, Inc.
 *
 * Author: Joerg Roedel <joerg.roedel@amd.com>
 */

// CONFIG_DMA_API_DEBUG selects the externally implemented debug hooks.
#[cfg(feature = "CONFIG_DMA_API_DEBUG")]
extern "C" {
    pub fn debug_dma_map_phys(
        dev: *mut device,
        phys: phys_addr_t,
        size: usize,
        direction: ::std::os::raw::c_int,
        dma_addr: dma_addr_t,
        attrs: ::std::os::raw::c_ulong,
    );

    pub fn debug_dma_unmap_phys(
        dev: *mut device,
        addr: dma_addr_t,
        size: usize,
        direction: ::std::os::raw::c_int,
        attrs: ::std::os::raw::c_ulong,
    );

    pub fn debug_dma_map_sg(
        dev: *mut device,
        sg: *mut scatterlist,
        nents: ::std::os::raw::c_int,
        mapped_ents: ::std::os::raw::c_int,
        direction: ::std::os::raw::c_int,
        attrs: ::std::os::raw::c_ulong,
    );

    pub fn debug_dma_unmap_sg(
        dev: *mut device,
        sglist: *mut scatterlist,
        nelems: ::std::os::raw::c_int,
        dir: ::std::os::raw::c_int,
        attrs: ::std::os::raw::c_ulong,
    );

    pub fn debug_dma_alloc_coherent(
        dev: *mut device,
        size: usize,
        dma_addr: dma_addr_t,
        virt: *mut ::std::ffi::c_void,
        attrs: ::std::os::raw::c_ulong,
    );

    pub fn debug_dma_free_coherent(
        dev: *mut device,
        size: usize,
        virt: *mut ::std::ffi::c_void,
        addr: dma_addr_t,
        attrs: ::std::os::raw::c_ulong,
    );

    pub fn debug_dma_sync_single_for_cpu(
        dev: *mut device,
        dma_handle: dma_addr_t,
        size: usize,
        direction: ::std::os::raw::c_int,
    );

    pub fn debug_dma_sync_single_for_device(
        dev: *mut device,
        dma_handle: dma_addr_t,
        size: usize,
        direction: ::std::os::raw::c_int,
    );

    pub fn debug_dma_sync_sg_for_cpu(
        dev: *mut device,
        sg: *mut scatterlist,
        nelems: ::std::os::raw::c_int,
        direction: ::std::os::raw::c_int,
    );

    pub fn debug_dma_sync_sg_for_device(
        dev: *mut device,
        sg: *mut scatterlist,
        nelems: ::std::os::raw::c_int,
        direction: ::std::os::raw::c_int,
    );

    pub fn debug_dma_alloc_pages(
        dev: *mut device,
        page: *mut page,
        size: usize,
        direction: ::std::os::raw::c_int,
        dma_addr: dma_addr_t,
    );

    pub fn debug_dma_free_pages(
        dev: *mut device,
        page: *mut page,
        size: usize,
        direction: ::std::os::raw::c_int,
        dma_addr: dma_addr_t,
    );
}

#[cfg(not(feature = "CONFIG_DMA_API_DEBUG"))]
pub unsafe fn debug_dma_map_phys(
    _dev: *mut device,
    _phys: phys_addr_t,
    _size: usize,
    _direction: ::std::os::raw::c_int,
    _dma_addr: dma_addr_t,
    _attrs: ::std::os::raw::c_ulong,
) {
}

#[cfg(not(feature = "CONFIG_DMA_API_DEBUG"))]
pub unsafe fn debug_dma_unmap_phys(
    _dev: *mut device,
    _addr: dma_addr_t,
    _size: usize,
    _direction: ::std::os::raw::c_int,
    _attrs: ::std::os::raw::c_ulong,
) {
}

#[cfg(not(feature = "CONFIG_DMA_API_DEBUG"))]
pub unsafe fn debug_dma_map_sg(
    _dev: *mut device,
    _sg: *mut scatterlist,
    _nents: ::std::os::raw::c_int,
    _mapped_ents: ::std::os::raw::c_int,
    _direction: ::std::os::raw::c_int,
    _attrs: ::std::os::raw::c_ulong,
) {
}

#[cfg(not(feature = "CONFIG_DMA_API_DEBUG"))]
pub unsafe fn debug_dma_unmap_sg(
    _dev: *mut device,
    _sglist: *mut scatterlist,
    _nelems: ::std::os::raw::c_int,
    _dir: ::std::os::raw::c_int,
    _attrs: ::std::os::raw::c_ulong,
) {
}

#[cfg(not(feature = "CONFIG_DMA_API_DEBUG"))]
pub unsafe fn debug_dma_alloc_coherent(
    _dev: *mut device,
    _size: usize,
    _dma_addr: dma_addr_t,
    _virt: *mut ::std::ffi::c_void,
    _attrs: ::std::os::raw::c_ulong,
) {
}

#[cfg(not(feature = "CONFIG_DMA_API_DEBUG"))]
pub unsafe fn debug_dma_free_coherent(
    _dev: *mut device,
    _size: usize,
    _virt: *mut ::std::ffi::c_void,
    _addr: dma_addr_t,
    _attrs: ::std::os::raw::c_ulong,
) {
}

#[cfg(not(feature = "CONFIG_DMA_API_DEBUG"))]
pub unsafe fn debug_dma_sync_single_for_cpu(
    _dev: *mut device,
    _dma_handle: dma_addr_t,
    _size: usize,
    _direction: ::std::os::raw::c_int,
) {
}

#[cfg(not(feature = "CONFIG_DMA_API_DEBUG"))]
pub unsafe fn debug_dma_sync_single_for_device(
    _dev: *mut device,
    _dma_handle: dma_addr_t,
    _size: usize,
    _direction: ::std::os::raw::c_int,
) {
}

#[cfg(not(feature = "CONFIG_DMA_API_DEBUG"))]
pub unsafe fn debug_dma_sync_sg_for_cpu(
    _dev: *mut device,
    _sg: *mut scatterlist,
    _nelems: ::std::os::raw::c_int,
    _direction: ::std::os::raw::c_int,
) {
}

#[cfg(not(feature = "CONFIG_DMA_API_DEBUG"))]
pub unsafe fn debug_dma_sync_sg_for_device(
    _dev: *mut device,
    _sg: *mut scatterlist,
    _nelems: ::std::os::raw::c_int,
    _direction: ::std::os::raw::c_int,
) {
}

#[cfg(not(feature = "CONFIG_DMA_API_DEBUG"))]
pub unsafe fn debug_dma_alloc_pages(
    _dev: *mut device,
    _page: *mut page,
    _size: usize,
    _direction: ::std::os::raw::c_int,
    _dma_addr: dma_addr_t,
) {
}

#[cfg(not(feature = "CONFIG_DMA_API_DEBUG"))]
pub unsafe fn debug_dma_free_pages(
    _dev: *mut device,
    _page: *mut page,
    _size: usize,
    _direction: ::std::os::raw::c_int,
    _dma_addr: dma_addr_t,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
