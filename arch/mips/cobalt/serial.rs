// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Registration of Cobalt UART platform device.
 *
 *  Copyright (C) 2007  Yoichi Yuasa <yuasa@linux-mips.org>
 */

// Declarations supplied by the included Linux and Cobalt headers are external
// dependencies of this translation.

#[repr(C)]
pub struct resource {
    pub start: usize,
    pub end: usize,
    pub flags: usize,
}

#[repr(C)]
pub struct plat_serial8250_port {
    pub irq: i32,
    pub uartclk: u32,
    pub iotype: u32,
    pub flags: u32,
    pub mapbase: usize,
}

#[repr(C)]
pub struct platform_device {
    pub id: i32,
    pub dev: platform_device_dev,
}

#[repr(C)]
pub struct platform_device_dev {
    pub platform_data: *mut plat_serial8250_port,
}

unsafe extern "C" {
    pub static mut cobalt_board_id: i32;

    pub fn platform_device_alloc(name: *const core::ffi::c_char, id: i32)
        -> *mut platform_device;
    pub fn platform_device_add_resources(
        pdev: *mut platform_device,
        resources: *mut resource,
        count: usize,
    ) -> i32;
    pub fn platform_device_add(pdev: *mut platform_device) -> i32;
    pub fn platform_device_put(pdev: *mut platform_device);
}

// Header-provided constants.
pub const IORESOURCE_MEM: usize = 0;
pub const IORESOURCE_IRQ: usize = 0;
pub const SERIAL_IRQ: i32 = 0;
pub const UPIO_MEM: u32 = 0;
pub const UPF_IOREMAP: u32 = 0;
pub const UPF_BOOT_AUTOCONF: u32 = 0;
pub const UPF_SKIP_TEST: u32 = 0;
pub const PLAT8250_DEV_PLATFORM: i32 = 0;
pub const COBALT_BRD_ID_QUBE1: i32 = 0;

static mut cobalt_uart_resource: [resource; 2] = [
    resource {
        start: 0x1c800000,
        end: 0x1c800007,
        flags: IORESOURCE_MEM,
    },
    resource {
        start: SERIAL_IRQ as usize,
        end: SERIAL_IRQ as usize,
        flags: IORESOURCE_IRQ,
    },
];

static mut cobalt_serial8250_port: [plat_serial8250_port; 2] = [
    plat_serial8250_port {
        irq: SERIAL_IRQ,
        uartclk: 18432000,
        iotype: UPIO_MEM,
        flags: UPF_IOREMAP | UPF_BOOT_AUTOCONF | UPF_SKIP_TEST,
        mapbase: 0x1c800000,
    },
    plat_serial8250_port {
        irq: 0,
        uartclk: 0,
        iotype: 0,
        flags: 0,
        mapbase: 0,
    },
];

#[allow(non_snake_case)]
pub unsafe fn cobalt_uart_add() -> i32 {
    let mut pdev: *mut platform_device;
    let mut retval: i32;

    /*
     * Cobalt Qube1 has no UART.
     */
    if cobalt_board_id == COBALT_BRD_ID_QUBE1 {
        return 0;
    }

    pdev = platform_device_alloc(b"serial8250\0".as_ptr() as *const core::ffi::c_char, -1);
    if pdev.is_null() {
        return -12;
    }

    (*pdev).id = PLAT8250_DEV_PLATFORM;
    (*pdev).dev.platform_data = cobalt_serial8250_port.as_mut_ptr();

    retval = platform_device_add_resources(
        pdev,
        cobalt_uart_resource.as_mut_ptr(),
        cobalt_uart_resource.len(),
    );
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

// device_initcall(cobalt_uart_add);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
