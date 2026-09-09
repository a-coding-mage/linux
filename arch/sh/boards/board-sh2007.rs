// SPDX-License-Identifier: GPL-2.0
/*
 * SH-2007 board support.
 *
 * Copyright (C) 2003, 2004  SUGIOKA Toshinobu
 * Copyright (C) 2010  Hitoshi Mitake <mitake@dcl.info.waseda.ac.jp>
 */

// C dependencies supplied by the surrounding kernel translation.

/* Dummy supplies, where voltage doesn't matter */
static mut DUMMY_SUPPLIES: [regulator_consumer_supply; 4] = [
    REGULATOR_SUPPLY!("vddvario", "smsc911x.0"),
    REGULATOR_SUPPLY!("vdd33a", "smsc911x.0"),
    REGULATOR_SUPPLY!("vddvario", "smsc911x.1"),
    REGULATOR_SUPPLY!("vdd33a", "smsc911x.1"),
];

static mut smc911x_info: smsc911x_platform_config = smsc911x_platform_config {
    flags: SMSC911X_USE_32BIT,
    irq_polarity: SMSC911X_IRQ_POLARITY_ACTIVE_LOW,
    irq_type: SMSC911X_IRQ_TYPE_PUSH_PULL,
};

static mut smsc9118_0_resources: [resource; 2] = [
    resource {
        start: SMC0_BASE,
        end: SMC0_BASE + 0xff,
        flags: IORESOURCE_MEM,
    },
    resource {
        start: evt2irq(0x240),
        end: evt2irq(0x240),
        flags: IORESOURCE_IRQ,
    },
];

static mut smsc9118_1_resources: [resource; 2] = [
    resource {
        start: SMC1_BASE,
        end: SMC1_BASE + 0xff,
        flags: IORESOURCE_MEM,
    },
    resource {
        start: evt2irq(0x280),
        end: evt2irq(0x280),
        flags: IORESOURCE_IRQ,
    },
];

static mut smsc9118_0_device: platform_device = platform_device {
    name: "smsc911x",
    id: 0,
    num_resources: ARRAY_SIZE!(smsc9118_0_resources),
    resource: smsc9118_0_resources.as_mut_ptr(),
    dev: device {
        platform_data: &raw mut smc911x_info as *mut _,
    },
};

static mut smsc9118_1_device: platform_device = platform_device {
    name: "smsc911x",
    id: 1,
    num_resources: ARRAY_SIZE!(smsc9118_1_resources),
    resource: smsc9118_1_resources.as_mut_ptr(),
    dev: device {
        platform_data: &raw mut smc911x_info as *mut _,
    },
};

static mut cf_resources: [resource; 3] = [
    resource {
        start: CF_BASE + CF_OFFSET,
        end: CF_BASE + CF_OFFSET + 0x0f,
        flags: IORESOURCE_MEM,
    },
    resource {
        start: CF_BASE + CF_OFFSET + 0x206,
        end: CF_BASE + CF_OFFSET + 0x20f,
        flags: IORESOURCE_MEM,
    },
    resource {
        start: evt2irq(0x2c0),
        end: evt2irq(0x2c0),
        flags: IORESOURCE_IRQ,
    },
];

static mut cf_device: platform_device = platform_device {
    name: "pata_platform",
    id: 0,
    num_resources: ARRAY_SIZE!(cf_resources),
    resource: cf_resources.as_mut_ptr(),
};

static mut sh2007_devices: [*mut platform_device; 3] = [
    &raw mut smsc9118_0_device,
    &raw mut smsc9118_1_device,
    &raw mut cf_device,
];

unsafe fn sh2007_io_init() -> i32 {
    regulator_register_fixed(0, DUMMY_SUPPLIES.as_mut_ptr(), ARRAY_SIZE!(DUMMY_SUPPLIES));

    platform_add_devices(sh2007_devices.as_mut_ptr(), ARRAY_SIZE!(sh2007_devices));
    0
}

subsys_initcall!(sh2007_io_init);

unsafe fn sh2007_init_irq() {
    plat_irq_setup_pins(IRQ_MODE_IRQ);
}

/*
 * Initialize the board
 */
unsafe fn sh2007_setup(_cmdline_p: *mut *mut i8) {
    pr_info!("SH-2007 Setup...");

    /* setup wait control registers for area 5 */
    __raw_writel(CS5BCR_D, CS5BCR);
    __raw_writel(CS5WCR_D, CS5WCR);
    __raw_writel(CS5PCR_D, CS5PCR);

    pr_cont!(" done.\n");
}

/*
 * The Machine Vector
 */
static mut mv_sh2007: sh_machine_vector = sh_machine_vector {
    mv_setup: Some(sh2007_setup),
    mv_name: "sh2007",
    mv_init_irq: Some(sh2007_init_irq),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
