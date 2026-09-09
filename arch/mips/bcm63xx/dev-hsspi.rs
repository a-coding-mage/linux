/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2012 Jonas Gorski <jonas.gorski@gmail.com>
 */

// C dependencies: linux/init.h, linux/kernel.h, linux/platform_device.h,
// bcm63xx_cpu.h, bcm63xx_dev_hsspi.h, and bcm63xx_regs.h.

use core::ffi::c_char;

extern "C" {
    fn BCMCPU_IS_6328() -> bool;
    fn BCMCPU_IS_6362() -> bool;
    fn bcm63xx_regset_address(regset: i32) -> isize;
    fn bcm63xx_get_irq_number(irq: i32) -> isize;
    fn platform_device_register(device: *mut platform_device) -> i32;
}

// External constants supplied by bcm63xx headers.
const RSET_HSSPI: i32 = 0;
const RSET_HSSPI_SIZE: isize = 0;
const IRQ_HSSPI: i32 = 0;
const IORESOURCE_MEM: u64 = 0;
const IORESOURCE_IRQ: u64 = 0;
const ENODEV: i32 = 19;

#[repr(C)]
struct resource {
    start: isize,
    end: isize,
    flags: u64,
}

#[repr(C)]
struct platform_device {
    name: *const c_char,
    id: i32,
    num_resources: usize,
    resource: *mut resource,
}

static mut spi_resources: [resource; 2] = [
    resource {
        start: -1,
        end: -1,
        flags: IORESOURCE_MEM,
    },
    resource {
        start: -1,
        end: 0,
        flags: IORESOURCE_IRQ,
    },
];

static mut bcm63xx_hsspi_device: platform_device = platform_device {
    name: b"bcm63xx-hsspi\0".as_ptr() as *const c_char,
    id: 0,
    num_resources: 2,
    resource: core::ptr::null_mut(),
};

pub unsafe extern "C" fn bcm63xx_hsspi_register() -> i32 {
    if !BCMCPU_IS_6328() && !BCMCPU_IS_6362() {
        return -ENODEV;
    }

    spi_resources[0].start = bcm63xx_regset_address(RSET_HSSPI);
    spi_resources[0].end = spi_resources[0].start;
    spi_resources[0].end += RSET_HSSPI_SIZE - 1;
    spi_resources[1].start = bcm63xx_get_irq_number(IRQ_HSSPI);
    bcm63xx_hsspi_device.resource = spi_resources.as_mut_ptr();

    platform_device_register(&mut bcm63xx_hsspi_device)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
