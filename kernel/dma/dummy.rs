// SPDX-License-Identifier: GPL-2.0
/*
 * Dummy DMA ops that always fail.
 */
// Dependency equivalent of <linux/dma-map-ops.h> is supplied externally.

unsafe fn dma_dummy_mmap(
    _dev: *mut device,
    _vma: *mut vm_area_struct,
    _cpu_addr: *mut core::ffi::c_void,
    _dma_addr: dma_addr_t,
    _size: usize,
    _attrs: c_ulong,
) -> c_int {
    -ENXIO
}

unsafe fn dma_dummy_map_phys(
    _dev: *mut device,
    _phys: phys_addr_t,
    _size: usize,
    _dir: dma_data_direction,
    _attrs: c_ulong,
) -> dma_addr_t {
    DMA_MAPPING_ERROR
}

unsafe fn dma_dummy_unmap_phys(
    _dev: *mut device,
    _dma_handle: dma_addr_t,
    _size: usize,
    _dir: dma_data_direction,
    _attrs: c_ulong,
) {
    /*
     * Dummy ops doesn't support map_phys, so unmap_page should never be
     * called.
     */
    WARN_ON_ONCE(true);
}

unsafe fn dma_dummy_map_sg(
    _dev: *mut device,
    _sgl: *mut scatterlist,
    _nelems: c_int,
    _dir: dma_data_direction,
    _attrs: c_ulong,
) -> c_int {
    -EINVAL
}

unsafe fn dma_dummy_unmap_sg(
    _dev: *mut device,
    _sgl: *mut scatterlist,
    _nelems: c_int,
    _dir: dma_data_direction,
    _attrs: c_ulong,
) {
    /*
     * Dummy ops doesn't support map_sg, so unmap_sg should never be called.
     */
    WARN_ON_ONCE(true);
}

unsafe fn dma_dummy_supported(_hwdev: *mut device, _mask: u64) -> c_int {
    0
}

pub static dma_dummy_ops: dma_map_ops = dma_map_ops {
    mmap: Some(dma_dummy_mmap),
    map_phys: Some(dma_dummy_map_phys),
    unmap_phys: Some(dma_dummy_unmap_phys),
    map_sg: Some(dma_dummy_map_sg),
    unmap_sg: Some(dma_dummy_unmap_sg),
    dma_supported: Some(dma_dummy_supported),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
