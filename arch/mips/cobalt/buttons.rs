// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Cobalt buttons platform device.
 *
 *  Copyright (C) 2007  Yoichi Yuasa <yuasa@linux-mips.org>
 */

// External kernel declarations supplied by the surrounding Linux bindings.
#[repr(C)]
pub struct resource {
    pub start: usize,
    pub end: usize,
    pub flags: usize,
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

unsafe extern "C" {
    static IORESOURCE_MEM: usize;

    fn platform_device_alloc(name: *const core::ffi::c_char, id: i32) -> *mut platform_device;
    fn platform_device_add_resources(
        pdev: *mut platform_device,
        resources: *const resource,
        num: u32,
    ) -> i32;
    fn platform_device_add(pdev: *mut platform_device) -> i32;
    fn platform_device_put(pdev: *mut platform_device);
}

static mut COBALT_BUTTONS_RESOURCE: resource = resource {
    start: 0x1d000000,
    end: 0x1d000003,
    flags: 0,
};

unsafe fn cobalt_add_buttons() -> i32 {
    let pd: *mut platform_device;
    let error: i32;

    pd = platform_device_alloc(c"Cobalt buttons".as_ptr(), -1);
    if pd.is_null() {
        return -12;
    }

    COBALT_BUTTONS_RESOURCE.flags = IORESOURCE_MEM;
    error = platform_device_add_resources(pd, &raw const COBALT_BUTTONS_RESOURCE, 1);
    if error != 0 {
        platform_device_put(pd);
        return error;
    }

    error = platform_device_add(pd);
    if error != 0 {
        platform_device_put(pd);
        return error;
    }

    0
}

// Equivalent to device_initcall(cobalt_add_buttons); registration is provided
// by the surrounding kernel build system.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
