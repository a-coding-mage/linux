// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mach-footbridge/isa.c
 *
 *  Copyright (C) 2004 Russell King.
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int};

extern "C" {
    static IRQ_ISA_RTC_ALARM: u32;
    static IRQ_ISA_UART: u32;
    static IRQ_ISA_UART2: u32;
    static IORESOURCE_IO: u64;
    static IORESOURCE_IRQ: u64;
    static UPIO_PORT: u32;
    static UPF_BOOT_AUTOCONF: u32;
    static UPF_SKIP_TEST: u32;
    static PLAT8250_DEV_PLATFORM: c_int;

    fn isa_rtc_init();
    fn platform_device_register(device: *mut platform_device) -> c_int;
    fn printk(fmt: *const c_char, ...) -> c_int;
}

#[repr(C)]
pub struct resource {
    pub start: u64,
    pub end: u64,
    pub flags: u64,
}

#[repr(C)]
pub struct platform_device {
    pub name: *const c_char,
    pub id: c_int,
    pub dev: device,
    pub resource: *mut resource,
    pub num_resources: usize,
}

#[repr(C)]
pub struct device {
    pub platform_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct plat_serial8250_port {
    pub iobase: u32,
    pub irq: u32,
    pub uartclk: u32,
    pub regshift: u32,
    pub iotype: u32,
    pub flags: u32,
}

static mut rtc_resources: [resource; 2] = [
    resource {
        start: 0x70,
        end: 0x73,
        flags: unsafe { IORESOURCE_IO },
    },
    resource {
        start: unsafe { IRQ_ISA_RTC_ALARM as u64 },
        end: unsafe { IRQ_ISA_RTC_ALARM as u64 },
        flags: unsafe { IORESOURCE_IRQ },
    },
];

static mut rtc_device: platform_device = platform_device {
    name: b"rtc_cmos\0".as_ptr() as *const c_char,
    id: -1,
    dev: device {
        platform_data: core::ptr::null_mut(),
    },
    resource: unsafe { rtc_resources.as_mut_ptr() },
    num_resources: 2,
};

static mut serial_resources: [resource; 2] = [
    resource {
        start: 0x3f8,
        end: 0x3ff,
        flags: unsafe { IORESOURCE_IO },
    },
    resource {
        start: 0x2f8,
        end: 0x2ff,
        flags: unsafe { IORESOURCE_IO },
    },
];

static mut serial_platform_data: [plat_serial8250_port; 3] = [
    plat_serial8250_port {
        iobase: 0x3f8,
        irq: unsafe { IRQ_ISA_UART },
        uartclk: 1843200,
        regshift: 0,
        iotype: unsafe { UPIO_PORT },
        flags: unsafe { UPF_BOOT_AUTOCONF | UPF_SKIP_TEST },
    },
    plat_serial8250_port {
        iobase: 0x2f8,
        irq: unsafe { IRQ_ISA_UART2 },
        uartclk: 1843200,
        regshift: 0,
        iotype: unsafe { UPIO_PORT },
        flags: unsafe { UPF_BOOT_AUTOCONF | UPF_SKIP_TEST },
    },
    plat_serial8250_port {
        iobase: 0,
        irq: 0,
        uartclk: 0,
        regshift: 0,
        iotype: 0,
        flags: 0,
    },
];

static mut serial_device: platform_device = platform_device {
    name: b"serial8250\0".as_ptr() as *const c_char,
    id: unsafe { PLAT8250_DEV_PLATFORM },
    dev: device {
        platform_data: unsafe { serial_platform_data.as_mut_ptr() as *mut core::ffi::c_void },
    },
    resource: unsafe { serial_resources.as_mut_ptr() },
    num_resources: 2,
};

unsafe extern "C" fn footbridge_isa_init() -> c_int {
    let mut err: c_int = 0;

    /* Personal server doesn't have RTC */
    isa_rtc_init();
    err = platform_device_register(&mut rtc_device);
    if err != 0 {
        printk(b"Unable to register RTC device: %d\n\0".as_ptr() as *const c_char, err);
    }

    err = platform_device_register(&mut serial_device);
    if err != 0 {
        printk(b"Unable to register serial device: %d\n\0".as_ptr() as *const c_char, err);
    }
    0
}

// Equivalent of arch_initcall(footbridge_isa_init).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
