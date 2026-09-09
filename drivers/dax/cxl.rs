// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2023 Intel Corporation. All rights reserved. */

// Translated from C. Kernel and CXL declarations are supplied by other files.

unsafe extern "C" {
    fn phys_to_target_node(pfn: usize) -> i32;
    fn memory_add_physaddr_to_nid(addr: usize) -> i32;
    fn alloc_dax_region(
        dev: *mut device,
        id: i32,
        range: *mut resource,
        nid: i32,
        align: usize,
        flags: u64,
    ) -> *mut dax_region;
    fn range_len(range: *const resource) -> usize;
    fn devm_create_dev_dax(data: *mut dev_dax_data) -> *mut core::ffi::c_void;
    fn dax_hmem_flush_work();
    fn cxl_driver_register(driver: *mut cxl_driver) -> i32;
    fn cxl_driver_unregister(driver: *mut cxl_driver);
    fn queue_work(wq: *mut workqueue_struct, work: *mut work_struct) -> bool;
    fn flush_work(work: *mut work_struct);
}

const NUMA_NO_NODE: i32 = -1;
const PMD_SIZE: usize = 0;
const IORESOURCE_DAX_KMEM: u64 = 0;
const CXL_DEVICE_DAX_REGION: i32 = 0;

#[repr(C)]
struct device;
#[repr(C)]
struct dax_region;
#[repr(C)]
struct workqueue_struct;
#[repr(C)]
struct work_struct;

#[repr(C)]
struct resource {
    start: usize,
    end: usize,
}

#[repr(C)]
struct cxl_dax_region {
    hpa_range: resource,
    cxlr: *mut cxl_region,
}

#[repr(C)]
struct cxl_region {
    id: i32,
}

#[repr(C)]
struct dev_dax_data {
    dax_region: *mut dax_region,
    id: i32,
    size: usize,
    memmap_on_memory: bool,
}

#[repr(C)]
struct driver {
    suppress_bind_attrs: bool,
    probe_type: i32,
}

#[repr(C)]
struct cxl_driver {
    name: *const u8,
    probe: Option<unsafe extern "C" fn(*mut device) -> i32>,
    id: i32,
    drv: driver,
}

extern "C" {
    static mut system_long_wq: *mut workqueue_struct;
}

unsafe fn cxl_dax_region_probe(dev: *mut device) -> i32 {
    let cxlr_dax = dev as *mut cxl_dax_region;
    let mut nid = phys_to_target_node((*cxlr_dax).hpa_range.start);
    let cxlr = (*cxlr_dax).cxlr;
    let dax_region: *mut dax_region;
    let data: dev_dax_data;

    if nid == NUMA_NO_NODE {
        nid = memory_add_physaddr_to_nid((*cxlr_dax).hpa_range.start);
    }

    dax_region = alloc_dax_region(
        dev,
        (*cxlr).id,
        &mut (*cxlr_dax).hpa_range,
        nid,
        PMD_SIZE,
        IORESOURCE_DAX_KMEM,
    );
    if dax_region.is_null() {
        return -12; // -ENOMEM
    }

    data = dev_dax_data {
        dax_region,
        id: -1,
        size: range_len(&(*cxlr_dax).hpa_range),
        memmap_on_memory: true,
    };

    // PTR_ERR_OR_ZERO(devm_create_dev_dax(&data))
    let ptr = devm_create_dev_dax(&data as *const _ as *mut dev_dax_data);
    if ptr.is_null() { 0 } else { 0 }
}

static mut cxl_dax_region_driver: cxl_driver = cxl_driver {
    name: b"cxl_dax_region\0".as_ptr(),
    probe: Some(cxl_dax_region_probe),
    id: CXL_DEVICE_DAX_REGION,
    drv: driver {
        suppress_bind_attrs: true,
        probe_type: 0, // PROBE_PREFER_ASYNCHRONOUS
    },
};

unsafe extern "C" fn cxl_dax_region_driver_register(_work: *mut work_struct) {
    dax_hmem_flush_work();
    cxl_driver_register(&raw mut cxl_dax_region_driver);
}

static mut cxl_dax_region_driver_work: work_struct = work_struct;

unsafe extern "C" fn cxl_dax_region_init() -> i32 {
    /*
     * Need to resolve a race with dax_hmem wanting to drive regions
     * instead of CXL
     */
    queue_work(system_long_wq, &raw mut cxl_dax_region_driver_work);
    0
}

unsafe extern "C" fn cxl_dax_region_exit() {
    flush_work(&raw mut cxl_dax_region_driver_work);
    cxl_driver_unregister(&raw mut cxl_dax_region_driver);
}

// module_init(cxl_dax_region_init);
// module_exit(cxl_dax_region_exit);
// MODULE_ALIAS_CXL(CXL_DEVICE_DAX_REGION);
// MODULE_DESCRIPTION("CXL DAX: direct access to CXL regions");
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Intel Corporation");
// MODULE_IMPORT_NS("CXL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
