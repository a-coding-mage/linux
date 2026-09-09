// SPDX-License-Identifier: GPL-2.0
/*
 *
 * linux/arch/sh/boards/se/7206/setup.c
 *
 * Copyright (C) 2006  Yoshinori Sato
 * Copyright (C) 2007 - 2008  Paul Mundt
 *
 * Hitachi 7206 SolutionEngine Support.
 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/init.h, linux/platform_device.h, linux/smc91x.h,
// mach-se/mach/se7206.h, asm/io.h, asm/machvec.h, asm/heartbeat.h

static mut smc91x_resources: [struct_resource; 2] = [
    struct_resource {
        name: c"smc91x-regs".as_ptr(),
        start: PA_SMSC + 0x300,
        end: PA_SMSC + 0x300 + 0x020 - 1,
        flags: IORESOURCE_MEM,
        ..unsafe { core::mem::zeroed() }
    },
    struct_resource {
        start: 64,
        end: 64,
        flags: IORESOURCE_IRQ,
        ..unsafe { core::mem::zeroed() }
    },
];

static mut smc91x_info: smc91x_platdata = smc91x_platdata {
    flags: SMC91X_USE_16BIT,
    ..unsafe { core::mem::zeroed() }
};

static mut smc91x_device: platform_device = platform_device {
    name: c"smc91x".as_ptr(),
    id: -1,
    dev: device {
        dma_mask: core::ptr::null_mut(),
        coherent_dma_mask: 0xffff_ffff,
        platform_data: unsafe { &raw mut smc91x_info as *mut _ },
        ..unsafe { core::mem::zeroed() }
    },
    num_resources: 2,
    resource: unsafe { &raw mut smc91x_resources as *mut _ },
    ..unsafe { core::mem::zeroed() }
};

static mut heartbeat_bit_pos: [u8; 8] = [8, 9, 10, 11, 12, 13, 14, 15];

static mut heartbeat_data: heartbeat_data = heartbeat_data {
    bit_pos: unsafe { &raw mut heartbeat_bit_pos as *mut _ },
    nr_bits: 8,
};

static mut heartbeat_resource: struct_resource = struct_resource {
    start: PA_LED,
    end: PA_LED,
    flags: IORESOURCE_MEM | IORESOURCE_MEM_32BIT,
    ..unsafe { core::mem::zeroed() }
};

static mut heartbeat_device: platform_device = platform_device {
    name: c"heartbeat".as_ptr(),
    id: -1,
    dev: device {
        platform_data: unsafe { &raw mut heartbeat_data as *mut _ },
        ..unsafe { core::mem::zeroed() }
    },
    num_resources: 1,
    resource: unsafe { &raw mut heartbeat_resource as *mut _ },
    ..unsafe { core::mem::zeroed() }
};

static mut se7206_devices: [*mut platform_device; 2] = unsafe {
    [
        &raw mut smc91x_device,
        &raw mut heartbeat_device,
    ]
};

unsafe extern "C" {
    fn platform_add_devices(devices: *mut *mut platform_device, count: usize) -> i32;
    fn init_se7206_IRQ();
}

unsafe fn se7206_devices_setup() -> i32 {
    platform_add_devices(se7206_devices.as_mut_ptr(), 2)
}

// device_initcall(se7206_devices_setup);

unsafe fn se7206_mode_pins() -> i32 {
    MODE_PIN1 | MODE_PIN2
}

/*
 * The Machine Vector
 */

static mut mv_se: sh_machine_vector = sh_machine_vector {
    mv_name: c"SolutionEngine".as_ptr(),
    mv_init_irq: Some(init_se7206_IRQ),
    mv_mode_pins: Some(se7206_mode_pins),
    ..unsafe { core::mem::zeroed() }
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
