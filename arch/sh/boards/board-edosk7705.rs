// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/boards/renesas/edosk7705/setup.c
 *
 * Copyright (C) 2000  Kazumoto Kojima
 *
 * Hitachi SolutionEngine Support.
 *
 * Modified for edosk7705 development
 * board by S. Dunn, 2003.
 */

// The following names are supplied by the Linux SH platform dependencies.

pub const SMC_IOBASE: usize = 0xA2000000;
pub const SMC_IO_OFFSET: usize = 0x300;
pub const SMC_IOADDR: usize = SMC_IOBASE + SMC_IO_OFFSET;

pub const ETHERNET_IRQ: i32 = evt2irq(0x320);

extern "C" {
    fn evt2irq(event: u32) -> i32;
    fn make_imask_irq(irq: i32);
    fn platform_add_devices(devices: *mut *mut platform_device, count: usize) -> i32;
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct smc91x_platdata {
    pub flags: u32,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct resource {
    pub start: usize,
    pub end: usize,
    pub flags: u32,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct device {
    pub platform_data: *mut core::ffi::c_void,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct platform_device {
    pub name: *const core::ffi::c_char,
    pub id: i32,
    pub num_resources: usize,
    pub resource: *mut resource,
    pub dev: device,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct sh_machine_vector {
    pub mv_name: *const core::ffi::c_char,
    pub mv_init_irq: Option<unsafe extern "C" fn()>,
}

pub const SMC91X_USE_16BIT: u32 = 1 << 0;
pub const SMC91X_IO_SHIFT_1: u32 = 1 << 1;
pub const IORESOURCE_IRQ_LOWLEVEL: u32 = 1 << 2;
pub const IORESOURCE_MEM: u32 = 1 << 3;
pub const IORESOURCE_IRQ: u32 = 1 << 4;
pub const SZ_32: usize = 32;

unsafe extern "C" fn sh_edosk7705_init_irq() {
    make_imask_irq(ETHERNET_IRQ);
}

/* eth initialization functions */
static mut smc91x_info: smc91x_platdata = smc91x_platdata {
    flags: SMC91X_USE_16BIT | SMC91X_IO_SHIFT_1 | IORESOURCE_IRQ_LOWLEVEL,
};

static mut smc91x_res: [resource; 2] = [
    resource {
        start: SMC_IOADDR,
        end: SMC_IOADDR + SZ_32 - 1,
        flags: IORESOURCE_MEM,
    },
    resource {
        start: ETHERNET_IRQ as usize,
        end: ETHERNET_IRQ as usize,
        flags: IORESOURCE_IRQ,
    },
];

static mut smc91x_dev: platform_device = platform_device {
    name: b"smc91x\0".as_ptr() as *const core::ffi::c_char,
    id: -1,
    num_resources: 2,
    resource: core::ptr::addr_of_mut!(smc91x_res) as *mut resource,
    dev: device {
        platform_data: core::ptr::addr_of_mut!(smc91x_info) as *mut core::ffi::c_void,
    },
};

/* platform init code */
static mut edosk7705_devices: [*mut platform_device; 1] = [
    core::ptr::addr_of_mut!(smc91x_dev),
];

unsafe extern "C" fn init_edosk7705_devices() -> i32 {
    platform_add_devices(edosk7705_devices.as_mut_ptr(), edosk7705_devices.len())
}

// device_initcall(init_edosk7705_devices);

/*
 * The Machine Vector
 */
static mut mv_edosk7705: sh_machine_vector = sh_machine_vector {
    mv_name: b"EDOSK7705\0".as_ptr() as *const core::ffi::c_char,
    mv_init_irq: Some(sh_edosk7705_init_irq),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
