// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Registration of Cobalt RTC platform device.
 *
 *  Copyright (C) 2007  Yoichi Yuasa <yuasa@linux-mips.org>
 */

// Linux kernel dependencies supplied by the surrounding translation.

#[repr(C)]
pub struct resource {
    pub start: u64,
    pub end: u64,
    pub flags: u64,
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

// These values are supplied by the corresponding Linux kernel headers.
pub const IORESOURCE_IO: u64 = 0x0000_0100;
pub const IORESOURCE_IRQ: u64 = 0x0000_0200;

extern "C" {
    static RTC_IRQ: u64;

    fn platform_device_alloc(name: *const core::ffi::c_char, id: i32) -> *mut platform_device;
    fn platform_device_add_resources(
        pdev: *mut platform_device,
        resources: *const resource,
        num: usize,
    ) -> i32;
    fn platform_device_add(pdev: *mut platform_device) -> i32;
    fn platform_device_put(pdev: *mut platform_device);
}

static mut COBALT_RTC_RESOURCE: [resource; 2] = [
    resource {
        start: 0x70,
        end: 0x77,
        flags: IORESOURCE_IO,
    },
    resource {
        start: 0,
        end: 0,
        flags: IORESOURCE_IRQ,
    },
];

#[inline]
unsafe fn cobalt_rtc_resource_init() {
    COBALT_RTC_RESOURCE[1].start = RTC_IRQ;
    COBALT_RTC_RESOURCE[1].end = RTC_IRQ;
}

unsafe fn cobalt_rtc_add() -> i32 {
    cobalt_rtc_resource_init();

    let pdev: *mut platform_device;
    let retval: i32;

    pdev = platform_device_alloc(b"rtc_cmos\0".as_ptr() as *const core::ffi::c_char, -1);
    if pdev.is_null() {
        return -12; // -ENOMEM
    }

    retval = platform_device_add_resources(
        pdev,
        COBALT_RTC_RESOURCE.as_ptr(),
        COBALT_RTC_RESOURCE.len(),
    );
    if retval != 0 {
        platform_device_put(pdev);
        return retval;
    }

    let retval = platform_device_add(pdev);
    if retval != 0 {
        platform_device_put(pdev);
        return retval;
    }

    0
}

// device_initcall(cobalt_rtc_add);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
