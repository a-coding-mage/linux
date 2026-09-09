// SPDX-License-Identifier: GPL-2.0
/*
 * Renesas Technology Corp. SH7786 Urquell Support.
 *
 * Copyright (C) 2008  Kuninori Morimoto <morimoto.kuninori@renesas.com>
 * Copyright (C) 2009, 2010  Paul Mundt
 *
 * Based on board-sh7785lcr.c
 * Copyright (C) 2008  Yoshihiro Shimoda
 */

/* C kernel headers and build-time declarations are supplied by other files. */

/*
 * bit  1234 5678
 *----------------------------
 * SW1  0101 0010  -> Pck 33MHz version
 *     (1101 0010)    Pck 66MHz version
 * SW2  0x1x xxxx  -> little endian
 *                    29bit mode
 * SW47 0001 1000  -> CS0 : on-board flash
 *                    CS1 : SRAM, registers, LAN, PCMCIA
 *                    38400 bps for SCIF1
 *
 * Address
 * 0x00000000 - 0x04000000  (CS0)     Nor Flash
 * 0x04000000 - 0x04200000  (CS1)     SRAM
 * 0x05000000 - 0x05800000  (CS1)     on board register
 * 0x05800000 - 0x06000000  (CS1)     LAN91C111
 * 0x06000000 - 0x06400000  (CS1)     PCMCIA
 * 0x08000000 - 0x10000000  (CS2-CS3) DDR3
 * 0x10000000 - 0x14000000  (CS4)     PCIe
 * 0x14000000 - 0x14800000  (CS5)     Core0 LRAM/URAM
 * 0x14800000 - 0x15000000  (CS5)     Core1 LRAM/URAM
 * 0x18000000 - 0x1C000000  (CS6)     ATA/NAND-Flash
 * 0x1C000000 -             (CS7)     SH7786 Control register
 */

/* HeartBeat */
static mut heartbeat_resource: resource = resource {
    start: BOARDREG(SLEDR),
    end: BOARDREG(SLEDR),
    flags: IORESOURCE_MEM | IORESOURCE_MEM_16BIT,
};

static mut heartbeat_device: platform_device = platform_device {
    name: "heartbeat",
    id: -1,
    num_resources: 1,
    resource: &raw mut heartbeat_resource,
};

/* LAN91C111 */
static mut smc91x_info: smc91x_platdata = smc91x_platdata {
    flags: SMC91X_USE_16BIT | SMC91X_NOWAIT,
};

static mut smc91x_eth_resources: [resource; 2] = [
    resource {
        name: "SMC91C111",
        start: 0x05800300,
        end: 0x0580030f,
        flags: IORESOURCE_MEM,
    },
    resource {
        start: evt2irq(0x360),
        flags: IORESOURCE_IRQ,
    },
];

static mut smc91x_eth_device: platform_device = platform_device {
    name: "smc91x",
    num_resources: ARRAY_SIZE(smc91x_eth_resources),
    resource: smc91x_eth_resources.as_mut_ptr(),
    dev: device {
        platform_data: &raw mut smc91x_info,
    },
};

/* Nor Flash */
static mut nor_flash_partitions: [mtd_partition; 4] = [
    mtd_partition {
        name: "loader",
        offset: 0x00000000,
        size: SZ_512K,
        mask_flags: MTD_WRITEABLE, /* Read-only */
    },
    mtd_partition {
        name: "bootenv",
        offset: MTDPART_OFS_APPEND,
        size: SZ_512K,
        mask_flags: MTD_WRITEABLE, /* Read-only */
    },
    mtd_partition {
        name: "kernel",
        offset: MTDPART_OFS_APPEND,
        size: SZ_4M,
    },
    mtd_partition {
        name: "data",
        offset: MTDPART_OFS_APPEND,
        size: MTDPART_SIZ_FULL,
    },
];

static mut nor_flash_data: physmap_flash_data = physmap_flash_data {
    width: 2,
    parts: nor_flash_partitions.as_mut_ptr(),
    nr_parts: ARRAY_SIZE(nor_flash_partitions),
};

static mut nor_flash_resources: [resource; 1] = [resource {
    start: NOR_FLASH_ADDR,
    end: NOR_FLASH_ADDR + NOR_FLASH_SIZE - 1,
    flags: IORESOURCE_MEM,
}];

static mut nor_flash_device: platform_device = platform_device {
    name: "physmap-flash",
    dev: device {
        platform_data: &raw mut nor_flash_data,
    },
    num_resources: ARRAY_SIZE(nor_flash_resources),
    resource: nor_flash_resources.as_mut_ptr(),
};

static mut urquell_devices: [*mut platform_device; 3] = [
    &raw mut heartbeat_device,
    &raw mut smc91x_eth_device,
    &raw mut nor_flash_device,
];

unsafe fn urquell_devices_setup() -> i32 {
    /* USB */
    gpio_request(GPIO_FN_USB_OVC0, core::ptr::null());
    gpio_request(GPIO_FN_USB_PENC0, core::ptr::null());

    /* enable LAN */
    __raw_writew(
        __raw_readw(UBOARDREG(IRL2MSKR)) & !0x00000001,
        UBOARDREG(IRL2MSKR),
    );

    platform_add_devices(urquell_devices.as_mut_ptr(), ARRAY_SIZE(urquell_devices))
}

/* device_initcall(urquell_devices_setup); */

unsafe fn urquell_power_off() {
    __raw_writew(0xa5a5, UBOARDREG(SRSTR));
}

unsafe fn urquell_init_irq() {
    plat_irq_setup_pins(IRQ_MODE_IRL3210_MASK);
}

unsafe fn urquell_mode_pins() -> i32 {
    __raw_readw(UBOARDREG(MDSWMR))
}

unsafe fn urquell_clk_init() -> i32 {
    let clk: *mut clk;
    let ret: i32;

    /*
     * Only handle the EXTAL case, anyone interfacing a crystal
     * resonator will need to provide their own input clock.
     */
    if test_mode_pin(MODE_PIN9) != 0 {
        return -EINVAL;
    }

    clk = clk_get(core::ptr::null(), "extal");
    if IS_ERR(clk) {
        return PTR_ERR(clk);
    }
    ret = clk_set_rate(clk, 33333333);
    clk_put(clk);

    ret
}

/* Initialize the board */
unsafe fn urquell_setup(cmdline_p: *mut *mut u8) {
    printk(KERN_INFO "Renesas Technology Corp. Urquell support.\n");

    pm_power_off = Some(urquell_power_off);

    register_smp_ops(&shx3_smp_ops);
}

/*
 * The Machine Vector
 */
static mut mv_urquell: sh_machine_vector = sh_machine_vector {
    mv_name: "Urquell",
    mv_setup: Some(urquell_setup),
    mv_init_irq: Some(urquell_init_irq),
    mv_mode_pins: Some(urquell_mode_pins),
    mv_clk_init: Some(urquell_clk_init),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
