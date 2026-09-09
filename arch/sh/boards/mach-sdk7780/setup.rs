// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/boards/renesas/sdk7780/setup.c
 *
 * Renesas Solutions SH7780 SDK Support
 * Copyright (C) 2008 Nicholas Beck <nbeck@mpc-data.co.uk>
 */
// C headers omitted; their supplied declarations are referenced below.

const GPIO_PECR: u32 = 0xFFEA0008;

/* Heartbeat */
static mut HEARTBEAT_RESOURCE: resource = resource {
    start: PA_LED,
    end: PA_LED,
    flags: IORESOURCE_MEM | IORESOURCE_MEM_16BIT,
};

static mut HEARTBEAT_DEVICE: platform_device = platform_device {
    name: "heartbeat",
    id: -1,
    num_resources: 1,
    resource: unsafe { &raw mut HEARTBEAT_RESOURCE },
    ..platform_device::DEFAULT
};

/* SMC91x */
static mut SMC91X_ETH_RESOURCES: [resource; 2] = [
    resource {
        name: "smc91x-regs",
        start: PA_LAN + 0x300,
        end: PA_LAN + 0x300 + 0x10,
        flags: IORESOURCE_MEM,
    },
    resource {
        start: IRQ_ETHERNET,
        end: IRQ_ETHERNET,
        flags: IORESOURCE_IRQ,
        ..resource::DEFAULT
    },
];

static mut SMC91X_ETH_DEVICE: platform_device = platform_device {
    name: "smc91x",
    id: 0,
    dev: device {
        dma_mask: core::ptr::null(), /* don't use dma */
        coherent_dma_mask: 0xffff_ffff,
        ..device::DEFAULT
    },
    num_resources: SMC91X_ETH_RESOURCES.len(),
    resource: unsafe { SMC91X_ETH_RESOURCES.as_mut_ptr() },
    ..platform_device::DEFAULT
};

static mut SDK7780_DEVICES: [*mut platform_device; 2] = [
    unsafe { &raw mut HEARTBEAT_DEVICE },
    unsafe { &raw mut SMC91X_ETH_DEVICE },
];

unsafe extern "C" fn sdk7780_devices_setup() -> i32 {
    platform_add_devices(SDK7780_DEVICES.as_mut_ptr(), SDK7780_DEVICES.len())
}

// C: device_initcall(sdk7780_devices_setup);

unsafe extern "C" fn sdk7780_setup(cmdline_p: *mut *mut core::ffi::c_char) {
    let ver: u16 = __raw_readw(FPGA_FPVERR);
    let date_stamp: u16 = __raw_readw(FPGA_FPDATER);

    printk(KERN_INFO, "Renesas Technology Europe SDK7780 support.\n");
    printk(
        KERN_INFO,
        "Board version: %d (revision %d), FPGA version: %d (revision %d), datestamp : %d\n",
        ((ver >> 12) & 0xf) as i32,
        ((ver >> 8) & 0xf) as i32,
        ((ver >> 4) & 0xf) as i32,
        (ver & 0xf) as i32,
        date_stamp as i32,
    );

    /* Setup pin mux'ing for PCIC */
    __raw_writew(0x0000, GPIO_PECR);
}

/*
 * The Machine Vector
 */
static mut MV_SE7780: sh_machine_vector = sh_machine_vector {
    mv_name: "Renesas SDK7780-R3",
    mv_setup: Some(sdk7780_setup),
    mv_init_irq: Some(init_sdk7780_IRQ),
    ..sh_machine_vector::DEFAULT
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
