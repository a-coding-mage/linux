/*
 * RNG driver for AMD RNGs
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

// Linux kernel dependencies supplied by other translation units.

const DRV_NAME: &str = "AMD768-HWRNG";

const RNGDATA: usize = 0x00;
const RNGDONE: usize = 0x04;
const PMBASE_OFFSET: usize = 0xF0;
const PMBASE_SIZE: usize = 8;

/* Data for PCI driver interface; exported through MODULE_DEVICE_TABLE. */
static pci_tbl: [pci_device_id; 3] = [
    pci_vdevice(AMD, 0x7443),
    pci_vdevice(AMD, 0x746b),
    pci_device_id {},
];

#[repr(C)]
struct amd768_priv {
    iobase: *mut core::ffi::c_void,
    pcidev: *mut pci_dev,
    pmbase: u32,
}

unsafe fn amd_rng_read(rng: *mut hwrng, buf: *mut core::ffi::c_void,
                       max: usize, wait: bool) -> isize {
    let mut data = buf as *mut u32;
    let priv_ = (*rng).priv_ as *mut amd768_priv;
    let mut read: usize = 0;
    /* We will wait at maximum one time per read */
    let mut timeout: isize = (max / 4 + 1) as isize;

    /* RNG data is available when RNGDONE is set to 1. */
    while read < max {
        if ioread32((*priv_).iobase.add(RNGDONE)) == 0 {
            if wait {
                /* Delay given by datasheet */
                usleep_range(128, 196);
                if timeout == 0 {
                    return read as isize;
                }
                timeout -= 1;
            } else {
                return 0;
            }
        } else {
            *data = ioread32((*priv_).iobase.add(RNGDATA));
            data = data.add(1);
            read += 4;
        }
    }
    read as isize
}

unsafe fn amd_rng_init(rng: *mut hwrng) -> i32 {
    let priv_ = (*rng).priv_ as *mut amd768_priv;
    let mut rnen: u8 = 0;

    pci_read_config_byte((*priv_).pcidev, 0x40, &mut rnen);
    rnen |= 1u8 << 7; /* RNG on */
    pci_write_config_byte((*priv_).pcidev, 0x40, rnen);

    pci_read_config_byte((*priv_).pcidev, 0x41, &mut rnen);
    rnen |= 1u8 << 7; /* PMIO enable */
    pci_write_config_byte((*priv_).pcidev, 0x41, rnen);
    0
}

unsafe fn amd_rng_cleanup(rng: *mut hwrng) {
    let priv_ = (*rng).priv_ as *mut amd768_priv;
    let mut rnen: u8 = 0;

    pci_read_config_byte((*priv_).pcidev, 0x40, &mut rnen);
    rnen &= !(1u8 << 7); /* RNG off */
    pci_write_config_byte((*priv_).pcidev, 0x40, rnen);
}

static mut amd_rng: hwrng = hwrng {
    name: "amd",
    init: Some(amd_rng_init),
    cleanup: Some(amd_rng_cleanup),
    read: Some(amd_rng_read),
    ..hwrng::default()
};

unsafe fn amd_rng_mod_init() -> i32 {
    let mut err: i32;
    let mut pdev: *mut pci_dev = core::ptr::null_mut();
    let mut ent: *const pci_device_id;
    let mut pmbase: u32 = 0;
    let mut priv_: *mut amd768_priv;

    for_each_pci_dev!(pdev) {
        ent = pci_match_id(pci_tbl.as_ptr(), pdev);
        if !ent.is_null() { goto!(found); }
    }
    /* Device not found. */
    return -ENODEV;

    found: {
        err = pci_read_config_dword(pdev, 0x58, &mut pmbase);
        if err != 0 {
            err = pcibios_err_to_errno(err);
            goto!(put_dev);
        }

        pmbase &= 0x0000FF00;
        if pmbase == 0 {
            err = -EIO;
            goto!(put_dev);
        }

        priv_ = kzalloc_obj::<amd768_priv>();
        if priv_.is_null() {
            err = -ENOMEM;
            goto!(put_dev);
        }

        if !request_region(pmbase as usize + PMBASE_OFFSET, PMBASE_SIZE, DRV_NAME) {
            dev_err!((*pdev).dev, "AMD768-HWRNG region 0x%x already in use!\n", pmbase + 0xF0);
            err = -EBUSY;
            goto!(out);
        }

        (*priv_).iobase = ioport_map(pmbase as usize + PMBASE_OFFSET, PMBASE_SIZE);
        if (*priv_).iobase.is_null() {
            pr_err!("AMD768-HWRNGCannot map ioport\n");
            err = -EINVAL;
            goto!(err_iomap);
        }

        amd_rng.priv_ = priv_ as usize;
        (*priv_).pmbase = pmbase;
        (*priv_).pcidev = pdev;

        pr_info!("AMD768-HWRNG detected\n");
        err = hwrng_register(&mut amd_rng);
        if err != 0 {
            pr_err!("AMD768-HWRNG registering failed (%d)\n", err);
            goto!(err_hwrng);
        }
        return 0;

        err_hwrng: ioport_unmap((*priv_).iobase);
        err_iomap: release_region(pmbase as usize + PMBASE_OFFSET, PMBASE_SIZE);
        out: kfree(priv_ as *mut core::ffi::c_void);
        put_dev: pci_dev_put(pdev);
    }
    err
}

unsafe fn amd_rng_mod_exit() {
    let priv_ = amd_rng.priv_ as *mut amd768_priv;
    hwrng_unregister(&mut amd_rng);
    ioport_unmap((*priv_).iobase);
    release_region((*priv_).pmbase as usize + PMBASE_OFFSET, PMBASE_SIZE);
    pci_dev_put((*priv_).pcidev);
    kfree(priv_ as *mut core::ffi::c_void);
}

module_init!(amd_rng_mod_init);
module_exit!(amd_rng_mod_exit);
module_author!("The Linux Kernel team");
module_description!("H/W RNG driver for AMD chipsets");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
