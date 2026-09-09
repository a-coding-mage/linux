/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2008 Maxime Bizon <mbizon@freebox.fr>
 */

/* Dependencies supplied by the Linux and BCM63xx headers are external. */

use core::ffi::{c_char, c_int, c_uint, c_ulong};

#[repr(C)]
pub struct resource {
    pub start: usize,
    pub end: usize,
    pub name: *const c_char,
    pub flags: c_ulong,
    pub parent: *mut resource,
    pub sibling: *mut resource,
    pub child: *mut resource,
}

#[repr(C)]
pub struct platform_device {
    pub name: *const c_char,
    pub id: c_int,
    pub num_resources: c_uint,
    pub resource: *mut resource,
}

const IORESOURCE_MEM: c_ulong = 0x0000_0200;
const IORESOURCE_IRQ: c_ulong = 0x0000_0400;

extern "C" {
    fn bcm63xx_regset_address(regset: c_uint) -> usize;
    fn bcm63xx_get_irq_number(irq: c_uint) -> usize;
    fn BCMCPU_IS_3368() -> bool;
    fn BCMCPU_IS_6358() -> bool;
    fn BCMCPU_IS_6368() -> bool;
    fn platform_device_register(device: *mut platform_device) -> c_int;
}

/* These constants are provided by bcm63xx_cpu.h and bcm63xx_dev_uart.h. */
extern "C" {
    static RSET_UART0: c_uint;
    static RSET_UART1: c_uint;
    static RSET_UART_SIZE: usize;
    static IRQ_UART0: c_uint;
    static IRQ_UART1: c_uint;
}

static mut uart0_resources: [resource; 2] = [
    resource {
        /* start & end filled at runtime */
        start: 0,
        end: 0,
        name: core::ptr::null(),
        flags: IORESOURCE_MEM,
        parent: core::ptr::null_mut(),
        sibling: core::ptr::null_mut(),
        child: core::ptr::null_mut(),
    },
    resource {
        /* start filled at runtime */
        start: 0,
        end: 0,
        name: core::ptr::null(),
        flags: IORESOURCE_IRQ,
        parent: core::ptr::null_mut(),
        sibling: core::ptr::null_mut(),
        child: core::ptr::null_mut(),
    },
];

static mut uart1_resources: [resource; 2] = [
    resource {
        /* start & end filled at runtime */
        start: 0,
        end: 0,
        name: core::ptr::null(),
        flags: IORESOURCE_MEM,
        parent: core::ptr::null_mut(),
        sibling: core::ptr::null_mut(),
        child: core::ptr::null_mut(),
    },
    resource {
        /* start filled at runtime */
        start: 0,
        end: 0,
        name: core::ptr::null(),
        flags: IORESOURCE_IRQ,
        parent: core::ptr::null_mut(),
        sibling: core::ptr::null_mut(),
        child: core::ptr::null_mut(),
    },
];

static mut bcm63xx_uart_devices: [platform_device; 2] = [
    platform_device {
        name: b"bcm63xx_uart\0" as *const [u8] as *const c_char,
        id: 0,
        num_resources: 2,
        resource: unsafe { uart0_resources.as_mut_ptr() },
    },
    platform_device {
        name: b"bcm63xx_uart\0" as *const [u8] as *const c_char,
        id: 1,
        num_resources: 2,
        resource: unsafe { uart1_resources.as_mut_ptr() },
    },
];

pub unsafe extern "C" fn bcm63xx_uart_register(id: c_uint) -> c_int {
    if id >= bcm63xx_uart_devices.len() as c_uint {
        return -19; /* -ENODEV */
    }

    if id == 1 && (!BCMCPU_IS_3368() && !BCMCPU_IS_6358() && !BCMCPU_IS_6368()) {
        return -19; /* -ENODEV */
    }

    if id == 0 {
        uart0_resources[0].start = bcm63xx_regset_address(RSET_UART0);
        uart0_resources[0].end = uart0_resources[0].start + RSET_UART_SIZE - 1;
        uart0_resources[1].start = bcm63xx_get_irq_number(IRQ_UART0);
    }

    if id == 1 {
        uart1_resources[0].start = bcm63xx_regset_address(RSET_UART1);
        uart1_resources[0].end = uart1_resources[0].start + RSET_UART_SIZE - 1;
        uart1_resources[1].start = bcm63xx_get_irq_number(IRQ_UART1);
    }

    platform_device_register(&mut bcm63xx_uart_devices[id as usize])
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
