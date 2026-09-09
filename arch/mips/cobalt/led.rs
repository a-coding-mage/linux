// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Registration of Cobalt LED platform device.
 *
 *  Copyright (C) 2007 Yoichi Yuasa <yuasa@linux-mips.org>
 */

// Declarations supplied by the Linux kernel and cobalt platform headers.
#[repr(C)]
pub struct Resource {
    pub start: usize,
    pub end: usize,
    pub flags: u64,
}

#[repr(C)]
pub struct PlatformDevice {
    _private: [u8; 0],
}

extern "C" {
    static mut cobalt_board_id: i32;

    fn platform_device_alloc(name: *const u8, id: i32) -> *mut PlatformDevice;
    fn platform_device_add_resources(
        pdev: *mut PlatformDevice,
        resources: *const Resource,
        count: u32,
    ) -> i32;
    fn platform_device_add(pdev: *mut PlatformDevice) -> i32;
    fn platform_device_put(pdev: *mut PlatformDevice);
}

// Values supplied by cobalt.h.
extern "C" {
    static COBALT_BRD_ID_QUBE1: i32;
    static COBALT_BRD_ID_QUBE2: i32;
}

const ENOMEM: i32 = 12;
const IORESOURCE_MEM: u64 = 0x0000_0200;

static mut COBALT_LED_RESOURCE: Resource = Resource {
    start: 0x1c00_0000,
    end: 0x1c00_0000,
    flags: IORESOURCE_MEM,
};

unsafe fn cobalt_led_add() -> i32 {
    let pdev: *mut PlatformDevice;
    let retval: i32;

    if cobalt_board_id == COBALT_BRD_ID_QUBE1
        || cobalt_board_id == COBALT_BRD_ID_QUBE2
    {
        pdev = platform_device_alloc(b"cobalt-qube-leds\0".as_ptr(), -1);
    } else {
        pdev = platform_device_alloc(b"cobalt-raq-leds\0".as_ptr(), -1);
    }

    if pdev.is_null() {
        return -ENOMEM;
    }

    retval = platform_device_add_resources(pdev, &COBALT_LED_RESOURCE, 1);
    if retval != 0 {
        platform_device_put(pdev);
        return retval;
    }

    retval = platform_device_add(pdev);
    if retval != 0 {
        platform_device_put(pdev);
        return retval;
    }

    0
}

// Equivalent of device_initcall(cobalt_led_add).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
