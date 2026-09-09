// SPDX-License-Identifier: GPL-2.0-only
/*
 * Virtual EISA root driver.
 * Acts as a placeholder if we don't have a proper EISA bridge.
 *
 * (C) 2003 Marc Zyngier <maz@wild-wind.fr.eu.org>
 * modified for SNI usage by Thomas Bogendoerfer
 */

// C dependencies supplied by the surrounding kernel translation.

/* The default EISA device parent (virtual root device).
 * Now use a platform device, since that's the obvious choice. */

static mut eisa_root_dev: platform_device = platform_device {
    name: b"eisa\0".as_ptr() as *const _,
    id: 0,
};

static mut eisa_bus_root: eisa_root_device = eisa_root_device {
    dev: unsafe { &mut eisa_root_dev.dev as *mut _ },
    bus_base_addr: 0,
    res: unsafe { &mut ioport_resource as *mut _ },
    slots: EISA_MAX_SLOTS,
    dma_mask: 0xffff_ffff,
    force_probe: 1,
};

pub unsafe fn sni_eisa_root_init() -> i32
{
    let r: i32;

    r = platform_device_register(&mut eisa_root_dev);
    if r == 0 {
        return r;
    }

    dev_set_drvdata(&mut eisa_root_dev.dev, &mut eisa_bus_root as *mut _);

    if eisa_root_register(&mut eisa_bus_root) != 0 {
        /* A real bridge may have been registered before
         * us. So quietly unregister. */
        platform_device_unregister(&mut eisa_root_dev);
        return -1;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
