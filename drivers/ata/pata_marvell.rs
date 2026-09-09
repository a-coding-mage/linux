// SPDX-License-Identifier: GPL-2.0-only
/*
 * Marvell PATA driver.
 *
 * For the moment we drive the PATA port in legacy mode. That
 * isn't making full use of the device functionality but it is
 * easy to get working.
 *
 * (c) 2006 Red Hat
 */

// Linux kernel dependencies are supplied by the surrounding translation.

const DRV_NAME: &str = "pata_marvell";
const DRV_VERSION: &str = "0.1.6";

/**
 * marvell_pata_active - check if PATA is active
 * @pdev: PCI device
 *
 * Returns 1 if the PATA port may be active. We know how to check this
 * for the 6145 but not the other devices
 */
unsafe fn marvell_pata_active(pdev: *mut pci_dev) -> i32 {
    let mut devices: u32;
    let mut barp: *mut core::ffi::c_void;

    // We don't yet know how to do this for other devices
    if (*pdev).device != 0x6145 {
        return 1;
    }

    barp = pci_iomap(pdev, 5, 0x10);
    if barp.is_null() {
        return -ENOMEM;
    }

    devices = ioread32(barp.add(0x0c));
    pci_iounmap(pdev, barp);

    if devices & 0x10 != 0 {
        return 1;
    }
    0
}

/**
 * marvell_pre_reset - probe begin
 * @link: link
 * @deadline: deadline jiffies for the operation
 *
 * Perform the PATA port setup we need.
 */
unsafe fn marvell_pre_reset(link: *mut ata_link, deadline: c_ulong) -> i32 {
    let ap = (*link).ap;
    let pdev = to_pci_dev((*(*ap).host).dev);

    if (*pdev).device == 0x6145 && (*ap).port_no == 0
        && marvell_pata_active(pdev) == 0
    {
        return -ENOENT;
    }

    ata_sff_prereset(link, deadline)
}

unsafe fn marvell_cable_detect(ap: *mut ata_port) -> i32 {
    // Cable type
    match (*ap).port_no {
        0 => {
            if (*ap).ioaddr.bmdma_addr.is_null() {
                return ATA_CBL_PATA_UNK;
            }
            if ioread8((*ap).ioaddr.bmdma_addr.add(1)) & 1 != 0 {
                return ATA_CBL_PATA40;
            }
            ATA_CBL_PATA80
        }
        1 => ATA_CBL_SATA, // Legacy SATA port
        _ => {
            BUG();
            0 // Our BUG macro needs the right markup
        }
    }
}

// No PIO or DMA methods needed for this device

static marvell_sht: scsi_host_template = scsi_host_template {
    // ATA_BMDMA_SHT(DRV_NAME)
};

static mut marvell_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma_port_ops,
    cable_detect: Some(marvell_cable_detect),
    reset: ata_port_operations_reset {
        prereset: Some(marvell_pre_reset),
    },
};

/**
 * marvell_init_one - Register Marvell ATA PCI device with kernel services
 * @pdev: PCI device to register
 * @id: PCI device ID
 *
 * Called from kernel PCI layer.
 *
 * LOCKING:
 * Inherited from PCI layer (may sleep).
 *
 * RETURNS:
 * Zero on success, or -ERRNO value.
 */
unsafe fn marvell_init_one(
    pdev: *mut pci_dev,
    _id: *const pci_device_id,
) -> i32 {
    static info: ata_port_info = ata_port_info {
        flags: ATA_FLAG_SLAVE_POSS,
        pio_mask: ATA_PIO4,
        mwdma_mask: ATA_MWDMA2,
        udma_mask: ATA_UDMA5,
        port_ops: &marvell_ops,
    };
    static info_sata: ata_port_info = ata_port_info {
        // Slave possible as its magically mapped not real
        flags: ATA_FLAG_SLAVE_POSS,
        pio_mask: ATA_PIO4,
        mwdma_mask: ATA_MWDMA2,
        udma_mask: ATA_UDMA6,
        port_ops: &marvell_ops,
    };
    let mut ppi: [*const ata_port_info; 2] = [&info, &info_sata];

    if (*pdev).device == 0x6101 {
        ppi[1] = &ata_dummy_port_info;
    }

    // Preserved from #if IS_ENABLED(CONFIG_SATA_AHCI).
    if !marvell_pata_active(pdev) {
        dev_info(&(*pdev).dev, "PATA port not active, deferring to AHCI driver.\n");
        return -ENODEV;
    }

    ata_pci_bmdma_init_one(pdev, ppi.as_ptr(), &marvell_sht, core::ptr::null_mut(), 0)
}

static marvell_pci_tbl: [pci_device_id; 7] = [
    PCI_DEVICE(0x11AB, 0x6101),
    PCI_DEVICE(0x11AB, 0x6121),
    PCI_DEVICE(0x11AB, 0x6123),
    PCI_DEVICE(0x11AB, 0x6145),
    PCI_DEVICE(0x1B4B, 0x91A0),
    PCI_DEVICE(0x1B4B, 0x91A4),
    pci_device_id {}, // terminate list
];

static mut marvell_pci_driver: pci_driver = pci_driver {
    name: DRV_NAME,
    id_table: marvell_pci_tbl.as_ptr(),
    probe: Some(marvell_init_one),
    remove: Some(ata_pci_remove_one),
    // CONFIG_PM_SLEEP conditional fields preserved by the surrounding build.
};

module_pci_driver!(marvell_pci_driver);

module_author!("Alan Cox");
module_description!("SCSI low-level driver for Marvell ATA in legacy mode");
module_license!("GPL");
module_device_table!(pci, marvell_pci_tbl);
module_version!(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
