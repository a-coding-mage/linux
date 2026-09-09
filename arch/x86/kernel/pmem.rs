// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2015, Christoph Hellwig.
 * Copyright (c) 2015, Intel Corporation.
 */

// Declarations supplied by the Linux kernel headers.
#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

extern "C" {
    fn walk_iomem_res_desc(
        desc: u64,
        flags: u64,
        start: u64,
        end: u64,
        arg: *mut core::ffi::c_void,
        callback: unsafe fn(*mut resource, *mut core::ffi::c_void) -> i32,
    ) -> i32;
    fn platform_device_alloc(name: *const core::ffi::c_char, id: i32) -> *mut platform_device;
    fn platform_device_add(pdev: *mut platform_device) -> i32;
    fn platform_device_put(pdev: *mut platform_device);
}

// Values supplied by the Linux kernel headers.
const IORES_DESC_PERSISTENT_MEMORY_LEGACY: u64 = 12;
const IORESOURCE_MEM: u64 = 0x0000_0200;
const ENOMEM: i32 = 12;

unsafe fn found(_res: *mut resource, _data: *mut core::ffi::c_void) -> i32 {
    1
}

unsafe fn register_e820_pmem() -> i32 {
    let pdev: *mut platform_device;
    let mut rc: i32;

    rc = walk_iomem_res_desc(
        IORES_DESC_PERSISTENT_MEMORY_LEGACY,
        IORESOURCE_MEM,
        0,
        u64::MAX,
        core::ptr::null_mut(),
        found,
    );
    if rc <= 0 {
        return 0;
    }

    /*
     * See drivers/nvdimm/e820.c for the implementation, this is
     * simply here to trigger the module to load on demand.
     */
    pdev = platform_device_alloc(b"e820_pmem\0".as_ptr() as *const core::ffi::c_char, -1);
    if pdev.is_null() {
        return -ENOMEM;
    }

    rc = platform_device_add(pdev);
    if rc != 0 {
        platform_device_put(pdev);
    }

    rc
}

// device_initcall(register_e820_pmem);
// The kernel build system registers register_e820_pmem as a device initcall.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
