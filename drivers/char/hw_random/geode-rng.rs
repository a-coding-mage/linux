/*
 * RNG driver for AMD Geode RNGs
 *
 * Copyright 2005 (c) MontaVista Software, Inc.
 *
 * with the majority of the code coming from:
 *
 * Hardware driver for the Intel/AMD/VIA Random Number Generators (RNG)
 * (c) Copyright 2003 Red Hat Inc <jgarzik@redhat.com>
 *
 * derived from
 *
 * Hardware driver for the AMD 768 Random Number Generator (RNG)
 * (c) Copyright 2001 Red Hat Inc
 *
 * derived from
 *
 * Hardware driver for Intel i810 Random Number Generator (RNG)
 * Copyright 2000,2001 Jeff Garzik <jgarzik@pobox.com>
 * Copyright 2000,2001 Philipp Rumpf <prumpf@mandrakesoft.com>
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2. This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

// Kernel dependencies supplied by the surrounding build.

const PFX: &str = "KBUILD_MODNAME: ";
const GEODE_RNG_DATA_REG: usize = 0x50;
const GEODE_RNG_STATUS_REG: usize = 0x54;

/*
 * Data for PCI driver interface.
 *
 * This data only exists for exporting the supported PCI ids via
 * MODULE_DEVICE_TABLE. We do not actually register a pci_driver, because
 * someone else might one day want to register another driver on the same PCI id.
 */
static PCI_TBL: [pci_device_id; 2] = [
    pci_device_id::amd_lx_aes(),
    pci_device_id:: terminator(),
];

#[repr(C)]
struct amd_geode_priv {
    pcidev: *mut pci_dev,
    membase: *mut core::ffi::c_void,
}

unsafe fn geode_rng_data_read(rng: *mut hwrng, data: *mut u32) -> i32 {
    let priv_: *mut amd_geode_priv = (*rng).priv_ as *mut amd_geode_priv;
    let mem = (*priv_).membase as *mut u8;

    *data = readl(mem.add(GEODE_RNG_DATA_REG));

    4
}

unsafe fn geode_rng_data_present(rng: *mut hwrng, wait: i32) -> i32 {
    let priv_: *mut amd_geode_priv = (*rng).priv_ as *mut amd_geode_priv;
    let mem = (*priv_).membase as *mut u8;
    let mut data: i32 = 0;
    let mut i: i32 = 0;

    while i < 20 {
        data = if readl(mem.add(GEODE_RNG_STATUS_REG)) != 0 { 1 } else { 0 };
        if data != 0 || wait == 0 {
            break;
        }
        udelay(10);
        i += 1;
    }
    data
}

static mut geode_rng: hwrng = hwrng {
    name: "geode",
    data_present: Some(geode_rng_data_present),
    data_read: Some(geode_rng_data_read),
    priv_: 0,
};

unsafe fn geode_rng_init() -> i32 {
    let mut err: i32 = -ENODEV;
    let mut pdev: *mut pci_dev = core::ptr::null_mut();
    let mut ent: *const pci_device_id;
    let mut mem: *mut core::ffi::c_void;
    let mut rng_base: usize;
    let mut priv_: *mut amd_geode_priv;

    for_each_pci_dev!(pdev);
    ent = pci_match_id(PCI_TBL.as_ptr(), pdev);
    if !ent.is_null() {
        goto_found!(found);
    }
    /* Device not found. */
    return err;

found:
    priv_ = kzalloc_obj::<amd_geode_priv>();
    if priv_.is_null() {
        err = -ENOMEM;
        goto_put_dev!(put_dev);
    }

    rng_base = pci_resource_start(pdev, 0);
    if rng_base == 0 {
        goto_free_priv!(free_priv);
    }
    err = -ENOMEM;
    mem = ioremap(rng_base, 0x58);
    if mem.is_null() {
        goto_free_priv!(free_priv);
    }

    geode_rng.priv_ = priv_ as unsigned_long;
    (*priv_).membase = mem;
    (*priv_).pcidev = pdev;

    pr_info!("AMD Geode RNG detected\n");
    err = hwrng_register(&raw mut geode_rng);
    if err != 0 {
        pr_err!("{}RNG registering failed ({})\n", PFX, err);
        goto_err_unmap!(err_unmap);
    }
    return err;

err_unmap:
    iounmap(mem);
free_priv:
    kfree(priv_);
put_dev:
    pci_dev_put(pdev);
    err
}

unsafe fn geode_rng_exit() {
    let priv_: *mut amd_geode_priv = geode_rng.priv_ as *mut amd_geode_priv;
    hwrng_unregister(&raw mut geode_rng);
    iounmap((*priv_).membase);
    pci_dev_put((*priv_).pcidev);
    kfree(priv_);
}

module_init!(geode_rng_init);
module_exit!(geode_rng_exit);

module_description!("H/W RNG driver for AMD Geode LX CPUs");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
