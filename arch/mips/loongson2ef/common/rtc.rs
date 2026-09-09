// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Lemote Fuloong platform support
 *
 *  Copyright(c) 2010 Arnaud Patard <apatard@mandriva.com>
 */

// Linux dependencies supplied by the surrounding kernel translation.
use core::ffi::c_int;

// RTC_PORT(), RTC_IRQ, IORESOURCE_IO, and IORESOURCE_IRQ are C macros/constants
// from <linux/mc146818rtc.h> and the platform resource headers.
const RTC_PORT_0: usize = 0;
const RTC_PORT_1: usize = 1;
const RTC_IRQ: usize = 0;
const IORESOURCE_IO: usize = 0;
const IORESOURCE_IRQ: usize = 0;

#[repr(C)]
pub struct resource {
    pub start: usize,
    pub end: usize,
    pub flags: usize,
}

#[repr(C)]
pub struct platform_device {
    pub name: *const u8,
    pub id: c_int,
    pub resource: *mut resource,
    pub num_resources: usize,
}

extern "C" {
    fn platform_device_register(device: *mut platform_device) -> c_int;
}

static mut loongson_rtc_resources: [resource; 2] = [
    resource {
        start: RTC_PORT_0,
        end: RTC_PORT_1,
        flags: IORESOURCE_IO,
    },
    resource {
        start: RTC_IRQ,
        end: RTC_IRQ,
        flags: IORESOURCE_IRQ,
    },
];

static mut loongson_rtc_device: platform_device = platform_device {
    name: b"rtc_cmos\0".as_ptr(),
    id: -1,
    resource: core::ptr::addr_of_mut!(loongson_rtc_resources) as *mut resource,
    num_resources: 2,
};

#[inline]
pub unsafe fn loongson_rtc_platform_init() -> c_int {
    platform_device_register(core::ptr::addr_of_mut!(loongson_rtc_device));
    0
}

// C: device_initcall(loongson_rtc_platform_init);
// The initcall registration is provided by the surrounding kernel runtime.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
