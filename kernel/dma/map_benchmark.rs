// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020 HiSilicon Limited.
 */

// Kernel includes and build-time symbols are supplied by the surrounding tree.

#[repr(C)]
pub struct map_benchmark_data {
    pub bparam: map_benchmark,
    pub dev: *mut device,
    pub debugfs: *mut dentry,
    pub dir: dma_data_direction,
    pub sum_map_100ns: atomic64_t,
    pub sum_unmap_100ns: atomic64_t,
    pub sum_sq_map: atomic64_t,
    pub sum_sq_unmap: atomic64_t,
    pub loops: atomic64_t,
}

#[repr(C)]
pub struct map_benchmark_ops {
    pub prepare: Option<unsafe extern "C" fn(*mut map_benchmark_data) -> *mut core::ffi::c_void>,
    pub unprepare: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub initialize_data: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub do_map: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub do_unmap: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
}

#[repr(C)]
pub struct dma_single_map_param {
    pub dev: *mut device,
    pub addr: dma_addr_t,
    pub xbuf: *mut core::ffi::c_void,
    pub npages: u32,
    pub dma_dir: u32,
}

unsafe extern "C" fn dma_single_map_benchmark_prepare(map: *mut map_benchmark_data) -> *mut core::ffi::c_void {
    let params = kzalloc(core::mem::size_of::<dma_single_map_param>(), GFP_KERNEL) as *mut dma_single_map_param;
    if params.is_null() { return core::ptr::null_mut(); }
    (*params).npages = (*map).bparam.granule;
    (*params).dma_dir = (*map).bparam.dma_dir;
    (*params).dev = (*map).dev;
    (*params).xbuf = alloc_pages_exact((*params).npages as usize * PAGE_SIZE, GFP_KERNEL);
    if (*params).xbuf.is_null() { kfree(params as *mut core::ffi::c_void); return core::ptr::null_mut(); }
    params as *mut core::ffi::c_void
}

unsafe extern "C" fn dma_single_map_benchmark_unprepare(mparam: *mut core::ffi::c_void) {
    let params = mparam as *mut dma_single_map_param;
    free_pages_exact((*params).xbuf, (*params).npages as usize * PAGE_SIZE);
    kfree(params as *mut core::ffi::c_void);
}

unsafe extern "C" fn dma_single_map_benchmark_initialize_data(mparam: *mut core::ffi::c_void) {
    let params = mparam as *mut dma_single_map_param;
    /*
     * for a non-coherent device, if we don't stain them in the
     * cache, this will give an underestimate of the real-world
     * overhead of BIDIRECTIONAL or TO_DEVICE mappings;
     * 66 means everything goes well! 66 is lucky.
     */
    if (*params).dma_dir != DMA_FROM_DEVICE { memset((*params).xbuf, 0x66, (*params).npages as usize * PAGE_SIZE); }
}

unsafe extern "C" fn dma_single_map_benchmark_do_map(mparam: *mut core::ffi::c_void) -> i32 {
    let params = mparam as *mut dma_single_map_param;
    (*params).addr = dma_map_single((*params).dev, (*params).xbuf, (*params).npages as usize * PAGE_SIZE, (*params).dma_dir);
    if dma_mapping_error((*params).dev, (*params).addr) { pr_err!("dma_map_single failed on %s\n", dev_name((*params).dev)); return -ENOMEM; }
    0
}

unsafe extern "C" fn dma_single_map_benchmark_do_unmap(mparam: *mut core::ffi::c_void) {
    let params = mparam as *mut dma_single_map_param;
    dma_unmap_single((*params).dev, (*params).addr, (*params).npages as usize * PAGE_SIZE, (*params).dma_dir);
}

pub static mut dma_single_map_benchmark_ops: map_benchmark_ops = map_benchmark_ops {
    prepare: Some(dma_single_map_benchmark_prepare), unprepare: Some(dma_single_map_benchmark_unprepare),
    initialize_data: Some(dma_single_map_benchmark_initialize_data), do_map: Some(dma_single_map_benchmark_do_map),
    do_unmap: Some(dma_single_map_benchmark_do_unmap),
};

#[repr(C)]
pub struct dma_sg_map_param {
    pub sgt: sg_table,
    pub dev: *mut device,
    pub npages: u32,
    pub dma_dir: u32,
    pub buf: *mut *mut core::ffi::c_void,
}

// The remaining implementation depends on Linux scatterlist, kthread, DMA,
// debugfs, PCI, platform-driver, and module APIs supplied by the kernel tree.
// Preserve the source-level entry points and external interfaces here.
extern "C" {
    pub fn dma_sg_map_benchmark_prepare(map: *mut map_benchmark_data) -> *mut core::ffi::c_void;
    pub fn dma_sg_map_benchmark_unprepare(mparam: *mut core::ffi::c_void);
    pub fn dma_sg_map_benchmark_initialize_data(mparam: *mut core::ffi::c_void);
    pub fn dma_sg_map_benchmark_do_map(mparam: *mut core::ffi::c_void) -> i32;
    pub fn dma_sg_map_benchmark_do_unmap(mparam: *mut core::ffi::c_void);
    pub fn map_benchmark_thread(data: *mut core::ffi::c_void) -> i32;
    pub fn do_map_benchmark(map: *mut map_benchmark_data) -> i32;
    pub fn map_benchmark_ioctl(file: *mut file, cmd: u32, arg: usize) -> isize;
    pub fn map_benchmark_remove_debugfs(data: *mut core::ffi::c_void);
    pub fn map_benchmark_platform_probe(pdev: *mut platform_device) -> i32;
    pub fn map_benchmark_pci_probe(pdev: *mut pci_dev, id: *const pci_device_id) -> i32;
    pub fn map_benchmark_init() -> i32;
    pub fn map_benchmark_cleanup();
}

// External kernel types, constants, functions, and driver-registration objects
// are intentionally referenced rather than implemented in this isolated file.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
