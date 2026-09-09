// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/sh/boards/se/7751/setup.c
 *
 * Copyright (C) 2000  Kazumoto Kojima
 *
 * Hitachi SolutionEngine Support.
 *
 * Modified for 7751 Solution Engine by
 * Ian da Silva and Jeremy Siegel, 2001.
 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/init.h, linux/platform_device.h, asm/machvec.h,
// mach-se/mach/se7751.h, asm/io.h, and asm/heartbeat.h.

extern "C" {
    fn platform_add_devices(devices: *mut *mut platform_device, count: usize) -> i32;
    fn init_7751se_IRQ();
}

extern static PA_LED: usize;

#[repr(C)]
pub struct heartbeat_data {
    pub bit_pos: *mut u8,
    pub nr_bits: usize,
}

#[repr(C)]
pub struct resource {
    pub start: usize,
    pub end: usize,
    pub flags: usize,
}

#[repr(C)]
pub struct device {
    pub platform_data: *mut heartbeat_data,
}

#[repr(C)]
pub struct platform_device {
    pub name: *const u8,
    pub id: i32,
    pub dev: device,
    pub num_resources: usize,
    pub resource: *mut resource,
}

#[repr(C)]
pub struct sh_machine_vector {
    pub mv_name: *const u8,
    pub mv_init_irq: unsafe extern "C" fn(),
}

const IORESOURCE_MEM: usize = 0x0000_0200;

static mut heartbeat_bit_pos: [u8; 8] = [8, 9, 10, 11, 12, 13, 14, 15];

static mut heartbeat_data: heartbeat_data = heartbeat_data {
    bit_pos: unsafe { heartbeat_bit_pos.as_mut_ptr() },
    nr_bits: heartbeat_bit_pos.len(),
};

static mut heartbeat_resources: [resource; 1] = [resource {
    start: unsafe { PA_LED },
    end: unsafe { PA_LED },
    flags: IORESOURCE_MEM,
}];

static mut heartbeat_device: platform_device = platform_device {
    name: b"heartbeat\0".as_ptr(),
    id: -1,
    dev: device {
        platform_data: unsafe { &mut heartbeat_data },
    },
    num_resources: heartbeat_resources.len(),
    resource: unsafe { heartbeat_resources.as_mut_ptr() },
};

static mut se7751_devices: [*mut platform_device; 1] = [unsafe { &mut heartbeat_device }];

unsafe extern "C" fn se7751_devices_setup() -> i32 {
    platform_add_devices(se7751_devices.as_mut_ptr(), se7751_devices.len())
}

// device_initcall(se7751_devices_setup);

/*
 * The Machine Vector
 */
static mut mv_7751se: sh_machine_vector = sh_machine_vector {
    mv_name: b"7751 SolutionEngine\0".as_ptr(),
    mv_init_irq: init_7751se_IRQ,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
