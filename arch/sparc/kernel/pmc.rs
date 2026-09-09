// SPDX-License-Identifier: GPL-2.0
/* pmc - Driver implementation for power management functions
 * of Power Management Controller (PMC) on SPARCstation-Voyager.
 *
 * Copyright (c) 2002 Eric Brower (ebrower@usa.net)
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

/* Debug
 *
 * #define PMC_DEBUG_LED
 * #define PMC_NO_IDLE
 */

const PMC_OBPNAME: &str = "SUNW,pmc";
const PMC_DEVNAME: &str = "pmc";

const PMC_IDLE_REG: usize = 0x00;
const PMC_IDLE_ON: u8 = 0x01;

static mut regs: *mut u8 = core::ptr::null_mut();

#[inline]
unsafe fn pmc_readb(offs: usize) -> u8 {
    sbus_readb(regs.add(offs))
}

#[inline]
unsafe fn pmc_writeb(val: u8, offs: usize) {
    sbus_writeb(val, regs.add(offs));
}

/*
 * CPU idle callback function
 * See .../arch/sparc/kernel/process.c
 */
unsafe fn pmc_swift_idle() {
    // C conditional compilation: PMC_DEBUG_LED is disabled unless enabled by
    // the build configuration.
    #[cfg(feature = "PMC_DEBUG_LED")]
    set_auxio(0x00, AUXIO_LED);

    pmc_writeb(pmc_readb(PMC_IDLE_REG) | PMC_IDLE_ON, PMC_IDLE_REG);

    #[cfg(feature = "PMC_DEBUG_LED")]
    set_auxio(AUXIO_LED, 0x00);
}

unsafe fn pmc_probe(op: *mut platform_device) -> i32 {
    regs = of_ioremap(
        (*op).resource.as_ptr(),
        0,
        resource_size(&(*op).resource[0]),
        PMC_OBPNAME,
    );
    if regs.is_null() {
        printk(KERN_ERR, "%s: unable to map registers\n", PMC_DEVNAME);
        return -ENODEV;
    }

    // C conditional compilation: PMC_NO_IDLE is disabled unless enabled by
    // the build configuration.
    #[cfg(not(feature = "PMC_NO_IDLE"))]
    {
        /* Assign power management IDLE handler */
        sparc_idle = Some(pmc_swift_idle);
    }

    printk(KERN_INFO, "%s: power management initialized\n", PMC_DEVNAME);
    0
}

static pmc_match: [of_device_id; 2] = [
    of_device_id {
        name: PMC_OBPNAME,
    },
    of_device_id {},
];

static mut pmc_driver: platform_driver = platform_driver {
    driver: driver {
        name: "pmc",
        of_match_table: pmc_match.as_ptr(),
    },
    probe: Some(pmc_probe),
};

unsafe fn pmc_init() -> i32 {
    platform_driver_register(&raw mut pmc_driver)
}

/* This driver is not critical to the boot process
 * and is easiest to ioremap when SBus is already
 * initialized, so we install ourselves thusly:
 */
__initcall!(pmc_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
