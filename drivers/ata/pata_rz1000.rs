// SPDX-License-Identifier: GPL-2.0-only
/*
 *  RZ1000/1001 driver based upon
 *
 *  linux/drivers/ide/pci/rz1000.c Version 0.06 January 12, 2003
 *  Copyright (C) 1995-1998 Linus Torvalds & author (see below)
 *  Principal Author: mlord@pobox.com (Mark Lord)
 *
 *  See linux/MAINTAINERS for address of current maintainer.
 *
 *  This file provides support for disabling the buggy read-ahead
 *  mode of the RZ1000 IDE chipset, commonly used on Intel motherboards.
 */

// C headers and kernel-provided declarations are supplied by the surrounding
// translation unit.

const DRV_NAME: &str = "pata_rz1000";
const DRV_VERSION: &str = "0.2.4";

unsafe fn rz1000_set_mode(link: *mut ata_link, _unused: *mut *mut ata_device) -> i32 {
    let mut dev: *mut ata_device;

    // ata_for_each_dev(dev, link, ENABLED)
    unsafe {
        ata_for_each_dev(link, ENABLED, |d| {
            dev = d;
            // We don't really care
            (*dev).pio_mode = XFER_PIO_0;
            (*dev).xfer_mode = XFER_PIO_0;
            (*dev).xfer_shift = ATA_SHIFT_PIO;
            (*dev).flags |= ATA_DFLAG_PIO;
            ata_dev_info(dev, "configured for PIO\n");
        });
    }
    0
}

static RZ1000_SHT: scsi_host_template = ATA_PIO_SHT!(DRV_NAME);

static mut RZ1000_PORT_OPS: ata_port_operations = ata_port_operations {
    inherits: &ata_sff_port_ops,
    cable_detect: Some(ata_cable_40wire),
    set_mode: Some(rz1000_set_mode),
};

unsafe fn rz1000_fifo_disable(pdev: *mut pci_dev) -> i32 {
    let mut reg: u16 = 0;
    // Be exceptionally paranoid as we must be sure to apply the fix
    if pci_read_config_word(pdev, 0x40, &mut reg) != 0 {
        return -1;
    }
    reg &= 0xDFFF;
    if pci_write_config_word(pdev, 0x40, reg) != 0 {
        return -1;
    }
    dev_info(&(*pdev).dev, "disabled chipset readahead.\n");
    0
}

unsafe fn rz1000_init_one(
    pdev: *mut pci_dev,
    _ent: *const pci_device_id,
) -> i32 {
    static INFO: ata_port_info = ata_port_info {
        flags: ATA_FLAG_SLAVE_POSS,
        pio_mask: ATA_PIO4,
        port_ops: &RZ1000_PORT_OPS,
    };
    let ppi: [*const ata_port_info; 2] = [&INFO, core::ptr::null()];

    ata_print_version_once(&(*pdev).dev, DRV_VERSION);

    if rz1000_fifo_disable(pdev) == 0 {
        return ata_pci_sff_init_one(pdev, ppi.as_ptr(), &RZ1000_SHT, core::ptr::null(), 0);
    }

    dev_err(&(*pdev).dev, "failed to disable read-ahead on chipset.\n");
    // Not safe to use so skip
    -ENODEV
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn rz1000_reinit_one(pdev: *mut pci_dev) -> i32 {
    let host: *mut ata_host = pci_get_drvdata(pdev);
    let rc = ata_pci_device_do_resume(pdev);
    if rc != 0 {
        return rc;
    }

    // If this fails on resume (which is a "can't happen" case), we
    // must stop as any progress risks data loss
    if rz1000_fifo_disable(pdev) != 0 {
        panic!("rz1000 fifo");
    }

    ata_host_resume(host);
    0
}

static PATA_RZ1000: [pci_device_id; 3] = [
    PCI_VDEVICE!(PCTECH, PCI_DEVICE_ID_PCTECH_RZ1000),
    PCI_VDEVICE!(PCTECH, PCI_DEVICE_ID_PCTECH_RZ1001),
    PCI_DEVICE_ID_EMPTY,
];

static mut RZ1000_PCI_DRIVER: pci_driver = pci_driver {
    name: DRV_NAME,
    id_table: PATA_RZ1000.as_ptr(),
    probe: Some(rz1000_init_one),
    remove: Some(ata_pci_remove_one),
    #[cfg(CONFIG_PM_SLEEP)]
    suspend: Some(ata_pci_device_suspend),
    #[cfg(CONFIG_PM_SLEEP)]
    resume: Some(rz1000_reinit_one),
};

module_pci_driver!(RZ1000_PCI_DRIVER);

module_author!("Alan Cox");
module_description!("low-level driver for RZ1000 PCI ATA");
module_license!("GPL");
module_device_table!(pci, PATA_RZ1000);
module_version!(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
