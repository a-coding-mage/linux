// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/sh/boards/se/7721/setup.c
 *
 * Copyright (C) 2008 Renesas Solutions Corp.
 *
 * Hitachi UL SolutionEngine 7721 Support.
 */

// C build-time attributes and included declarations are supplied by the
// surrounding kernel translation unit.

extern "C" {
    static PA_LED: usize;
    static PA_MRSHPC_IO: usize;
    static MRSHPC_IRQ0: usize;

    fn mrshpc_setup_windows();
    fn platform_add_devices(devices: *mut *mut platform_device, count: usize) -> i32;
    fn init_se7721_IRQ();
    fn __raw_writew(value: u16, address: usize);
}

#[repr(C)]
struct heartbeat_data {
    bit_pos: *mut u8,
    nr_bits: usize,
}

#[repr(C)]
struct resource {
    start: usize,
    end: usize,
    flags: usize,
}

#[repr(C)]
struct device {
    platform_data: *mut heartbeat_data,
}

#[repr(C)]
struct platform_device {
    name: *const u8,
    id: i32,
    dev: device,
    num_resources: usize,
    resource: *mut resource,
}

const IORESOURCE_MEM: usize = 0x0000_0200;
const IORESOURCE_MEM_16BIT: usize = 0x0010_0000;
const IORESOURCE_IO: usize = 0x0000_0100;
const IORESOURCE_IRQ: usize = 0x0000_0000;

static mut heartbeat_bit_pos: [u8; 8] = [8, 9, 10, 11, 12, 13, 14, 15];

static mut heartbeat_data: heartbeat_data = heartbeat_data {
    bit_pos: unsafe { heartbeat_bit_pos.as_mut_ptr() },
    nr_bits: 8,
};

static mut heartbeat_resource: resource = resource {
    start: unsafe { PA_LED },
    end: unsafe { PA_LED },
    flags: IORESOURCE_MEM | IORESOURCE_MEM_16BIT,
};

static mut heartbeat_device: platform_device = platform_device {
    name: b"heartbeat\0".as_ptr(),
    id: -1,
    dev: device {
        platform_data: unsafe { &mut heartbeat_data },
    },
    num_resources: 1,
    resource: unsafe { &mut heartbeat_resource },
};

static mut cf_ide_resources: [resource; 3] = [
    resource {
        start: unsafe { PA_MRSHPC_IO } + 0x1f0,
        end: unsafe { PA_MRSHPC_IO } + 0x1f0 + 8,
        flags: IORESOURCE_IO,
    },
    resource {
        start: unsafe { PA_MRSHPC_IO } + 0x1f0 + 0x206,
        end: unsafe { PA_MRSHPC_IO } + 0x1f0 + 8 + 0x206 + 8,
        flags: IORESOURCE_IO,
    },
    resource {
        start: unsafe { MRSHPC_IRQ0 },
        end: 0,
        flags: IORESOURCE_IRQ,
    },
];

static mut cf_ide_device: platform_device = platform_device {
    name: b"pata_platform\0".as_ptr(),
    id: -1,
    dev: device { platform_data: core::ptr::null_mut() },
    num_resources: 3,
    resource: unsafe { cf_ide_resources.as_mut_ptr() },
};

static mut se7721_devices: [*mut platform_device; 2] = [
    unsafe { &mut cf_ide_device },
    unsafe { &mut heartbeat_device },
];

#[allow(non_snake_case)]
unsafe extern "C" fn se7721_devices_setup() -> i32 {
    mrshpc_setup_windows();
    platform_add_devices(se7721_devices.as_mut_ptr(), 2)
}

// device_initcall(se7721_devices_setup);

unsafe extern "C" fn se7721_setup(_cmdline_p: *mut *mut u8) {
    /* for USB */
    __raw_writew(0x0000, 0xA405010C); /* PGCR */
    __raw_writew(0x0000, 0xA405010E); /* PHCR */
    __raw_writew(0x00AA, 0xA4050118); /* PPCR */
    __raw_writew(0x0000, 0xA4050124); /* PSELA */
}

/*
 * The Machine Vector
 */
#[repr(C)]
struct sh_machine_vector {
    mv_name: *const u8,
    mv_setup: unsafe extern "C" fn(*mut *mut u8),
    mv_init_irq: unsafe extern "C" fn(),
}

static mut mv_se7721: sh_machine_vector = sh_machine_vector {
    mv_name: b"Solution Engine 7721\0".as_ptr(),
    mv_setup: se7721_setup,
    mv_init_irq: init_se7721_IRQ,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
