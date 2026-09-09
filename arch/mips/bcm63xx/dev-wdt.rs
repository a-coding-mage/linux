/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2008 Florian Fainelli <florian@openwrt.org>
 */

use core::ffi::c_void;

/* Kernel and BCM63xx declarations are supplied by the surrounding tree. */
extern "C" {
    fn bcm63xx_regset_address(regset: i32) -> usize;
    fn platform_device_register(device: *mut platform_device) -> i32;
}

#[repr(C)]
struct resource {
    start: usize,
    end: usize,
    flags: u64,
}

#[repr(C)]
struct bcm7038_wdt_platform_data {
    clk_name: *const i8,
}

#[repr(C)]
struct device {
    platform_data: *mut c_void,
}

#[repr(C)]
struct platform_device {
    name: *const i8,
    id: i32,
    num_resources: usize,
    resource: *mut resource,
    dev: device,
}

/* These values are preprocessor definitions provided by the BCM63xx headers. */
const IORESOURCE_MEM: u64 = 0x0000_0200;
const RSET_WDT: i32 = 0;
const RSET_WDT_SIZE: usize = 0;

static mut WDT_RESOURCES: [resource; 1] = [resource {
    start: usize::MAX, /* filled at runtime */
    end: usize::MAX, /* filled at runtime */
    flags: IORESOURCE_MEM,
}];

static mut BCM63XX_WDT_PDATA: bcm7038_wdt_platform_data =
    bcm7038_wdt_platform_data {
        clk_name: b"periph\0".as_ptr() as *const i8,
    };

static mut BCM63XX_WDT_DEVICE: platform_device = platform_device {
    name: b"bcm63xx-wdt\0".as_ptr() as *const i8,
    id: -1,
    num_resources: 1,
    resource: core::ptr::null_mut(),
    dev: device {
        platform_data: core::ptr::null_mut(),
    },
};

/* __init / arch_initcall registration is supplied by the kernel build system. */
#[allow(non_snake_case)]
unsafe fn bcm63xx_wdt_register() -> i32 {
    WDT_RESOURCES[0].start = bcm63xx_regset_address(RSET_WDT);
    WDT_RESOURCES[0].end = WDT_RESOURCES[0].start;
    WDT_RESOURCES[0].end = WDT_RESOURCES[0]
        .end
        .wrapping_add(RSET_WDT_SIZE.wrapping_sub(1));

    BCM63XX_WDT_DEVICE.resource = WDT_RESOURCES.as_mut_ptr();
    BCM63XX_WDT_DEVICE.dev.platform_data =
        &mut BCM63XX_WDT_PDATA as *mut bcm7038_wdt_platform_data as *mut c_void;

    platform_device_register(&mut BCM63XX_WDT_DEVICE)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
