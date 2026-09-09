// SPDX-License-Identifier: GPL-2.0-only
/*
 *    pata_netcell.c - Netcell PATA driver
 *
 *	(c) 2006 Red Hat
 */

// Translated from C. Kernel-provided types, functions, constants, and macros
// are supplied by the surrounding translation environment.

const DRV_NAME: &str = "pata_netcell";
const DRV_VERSION: &str = "0.1.7";

/* No PIO or DMA methods needed for this device */

unsafe fn netcell_read_id(
    adev: *mut ata_device,
    tf: *mut ata_taskfile,
    id: *mut __le16,
) -> unsigned_int {
    let err_mask = ata_do_dev_read_id(adev, tf, id);

    /* Firmware forgets to mark words 85-87 valid */
    if err_mask == 0 {
        *id.add(ATA_ID_CSF_DEFAULT as usize) |= cpu_to_le16(0x4000);
    }
    err_mask
}

static netcell_sht: scsi_host_template = scsi_host_template {
    // ATA_BMDMA_SHT(DRV_NAME)
    ..ATA_BMDMA_SHT(DRV_NAME)
};

static mut netcell_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma_port_ops,
    cable_detect: Some(ata_cable_80wire),
    read_id: Some(netcell_read_id),
};

/**
 *	netcell_init_one - Register Netcell ATA PCI device with kernel services
 *	@pdev: PCI device to register
 *	@ent: Entry in netcell_pci_tbl matching with @pdev
 *
 *	Called from kernel PCI layer.
 *
 *	LOCKING:
 *	Inherited from PCI layer (may sleep).
 *
 *	RETURNS:
 *	Zero on success, or -ERRNO value.
 */

unsafe fn netcell_init_one(
    pdev: *mut pci_dev,
    ent: *const pci_device_id,
) -> int {
    let info = ata_port_info {
        flags: ATA_FLAG_SLAVE_POSS,
        /* Actually we don't really care about these as the
           firmware deals with it */
        pio_mask: ATA_PIO4,
        mwdma_mask: ATA_MWDMA2,
        udma_mask: ATA_UDMA5, /* UDMA 133 */
        port_ops: &netcell_ops,
    };
    let port_info: [*const ata_port_info; 2] = [&info, core::ptr::null()];
    let rc: int;

    ata_print_version_once(&mut (*pdev).dev, DRV_VERSION);

    rc = pcim_enable_device(pdev);
    if rc != 0 {
        return rc;
    }

    /* Any chip specific setup/optimisation/messages here */
    ata_pci_bmdma_clear_simplex(pdev);

    /* And let the library code do the work */
    ata_pci_bmdma_init_one(pdev, port_info.as_ptr(), &netcell_sht, core::ptr::null(), 0)
}

static netcell_pci_tbl: [pci_device_id; 2] = [
    PCI_VDEVICE(NETCELL, PCI_DEVICE_ID_REVOLUTION),

    /* terminate list */
    pci_device_id {},
];

static mut netcell_pci_driver: pci_driver = pci_driver {
    name: DRV_NAME,
    id_table: netcell_pci_tbl.as_ptr(),
    probe: Some(netcell_init_one),
    remove: Some(ata_pci_remove_one),
    #[cfg(CONFIG_PM_SLEEP)]
    suspend: Some(ata_pci_device_suspend),
    #[cfg(CONFIG_PM_SLEEP)]
    resume: Some(ata_pci_device_resume),
};

module_pci_driver!(netcell_pci_driver);

MODULE_AUTHOR!("Alan Cox");
MODULE_DESCRIPTION!("SCSI low-level driver for Netcell PATA RAID");
MODULE_LICENSE!("GPL");
MODULE_DEVICE_TABLE!(pci, netcell_pci_tbl);
MODULE_VERSION!(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
