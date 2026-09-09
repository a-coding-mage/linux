// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/sh/boards/se/7780/setup.c
 *
 * Copyright (C) 2006,2007  Nobuhiro Iwamatsu
 *
 * Hitachi UL SolutionEngine 7780 Support.
 */

/* Dependencies are supplied by the surrounding kernel translation. */

/* Heartbeat */
static mut heartbeat_resource: resource = resource {
    start: PA_LED,
    end: PA_LED,
    flags: IORESOURCE_MEM | IORESOURCE_MEM_16BIT,
};

static mut heartbeat_device: platform_device = platform_device {
    name: "heartbeat",
    id: -1,
    num_resources: 1,
    resource: unsafe { &raw mut heartbeat_resource },
};

/* SMC91x */
static mut smc91x_eth_resources: [resource; 2] = [
    resource {
        name: "smc91x-regs",
        start: PA_LAN + 0x300,
        end: PA_LAN + 0x300 + 0x10,
        flags: IORESOURCE_MEM,
    },
    resource {
        start: SMC_IRQ,
        end: SMC_IRQ,
        flags: IORESOURCE_IRQ,
    },
];

static mut smc91x_eth_device: platform_device = platform_device {
    name: "smc91x",
    id: 0,
    dev: device {
        dma_mask: core::ptr::null_mut(), /* don't use dma */
        coherent_dma_mask: 0xffff_ffff,
    },
    num_resources: 2,
    resource: unsafe { &raw mut smc91x_eth_resources[0] },
};

static mut se7780_devices: [*mut platform_device; 2] = [
    unsafe { &raw mut heartbeat_device },
    unsafe { &raw mut smc91x_eth_device },
];

unsafe extern "C" {
    fn platform_add_devices(devices: *mut *mut platform_device, count: usize) -> i32;
    fn __raw_writew(value: u16, address: usize);
    fn __raw_readw(address: usize) -> u16;
    fn printk(format: *const core::ffi::c_char, ...);
    fn init_se7780_IRQ();
}

unsafe fn se7780_devices_setup() -> i32 {
    platform_add_devices(
        &raw mut se7780_devices[0],
        se7780_devices.len(),
    )
}

/* device_initcall(se7780_devices_setup); */

const GPIO_PHCR: usize = 0xFFEA000E;
const GPIO_PMSELR: usize = 0xFFEA0080;
const GPIO_PECR: usize = 0xFFEA0008;

unsafe fn se7780_setup(_cmdline_p: *mut *mut core::ffi::c_char) {
    /* "SH-Linux" on LED Display */
    __raw_writew(b'S' as u16, PA_LED_DISP + (DISP_SEL0_ADDR << 1));
    __raw_writew(b'H' as u16, PA_LED_DISP + (DISP_SEL1_ADDR << 1));
    __raw_writew(b'-' as u16, PA_LED_DISP + (DISP_SEL2_ADDR << 1));
    __raw_writew(b'L' as u16, PA_LED_DISP + (DISP_SEL3_ADDR << 1));
    __raw_writew(b'i' as u16, PA_LED_DISP + (DISP_SEL4_ADDR << 1));
    __raw_writew(b'n' as u16, PA_LED_DISP + (DISP_SEL5_ADDR << 1));
    __raw_writew(b'u' as u16, PA_LED_DISP + (DISP_SEL6_ADDR << 1));
    __raw_writew(b'x' as u16, PA_LED_DISP + (DISP_SEL7_ADDR << 1));

    printk(b"Hitachi UL Solutions Engine 7780SE03 support.\n\0".as_ptr() as *const core::ffi::c_char);

    /*
     * PCI REQ/GNT setting
     *   REQ0/GNT0 -> USB
     *   REQ1/GNT1 -> PC Card
     *   REQ2/GNT2 -> Serial ATA
     *   REQ3/GNT3 -> PCI slot
     */
    __raw_writew(0x0213, FPGA_REQSEL);

    /* GPIO setting */
    __raw_writew(0x0000, GPIO_PECR);
    __raw_writew(__raw_readw(GPIO_PHCR) & 0xfff3, GPIO_PHCR);
    __raw_writew(0x0c00, GPIO_PMSELR);

    /* iVDR Power ON */
    __raw_writew(0x0001, FPGA_IVDRPW);
}

/*
 * The Machine Vector
 */
static mut mv_se7780: sh_machine_vector = sh_machine_vector {
    mv_name: "Solution Engine 7780",
    mv_setup: Some(se7780_setup),
    mv_init_irq: Some(init_se7780_IRQ),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
