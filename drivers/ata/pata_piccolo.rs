/*
 *  pata_piccolo.c - Toshiba Piccolo PATA/SATA controller driver.
 *
 *  This is basically an update to ata_generic.c to add Toshiba Piccolo support
 *  then split out to keep ata_generic "clean".
 *
 *  Copyright 2005 Red Hat Inc, all rights reserved.
 *
 *  Elements from ide/pci/generic.c
 *      Copyright (C) 2001-2002 Andre Hedrick <andre@linux-ide.org>
 *      Portions (C) Copyright 2002 Red Hat Inc <alan@linux-ide.org>
 *
 *  May be copied or modified under the terms of the GNU General Public License
 *
 *  The timing data tables/programming info are courtesy of the NetBSD driver
 */

// Dependencies are supplied by the surrounding kernel/libata environment.

const DRV_NAME: &str = "pata_piccolo";
const DRV_VERSION: &str = "0.0.1";

unsafe fn tosh_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    static PIO: [u16; 6] = [0x0566, 0x0433, 0x0311, 0x0201, 0x0200, 0x0100];
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut conf: u16 = 0;
    pci_read_config_word(pdev, 0x50, &mut conf);
    conf &= 0xE088;
    conf |= PIO[((*adev).pio_mode - XFER_PIO_0) as usize];
    pci_write_config_word(pdev, 0x50, conf);
}

unsafe fn tosh_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut conf: u32 = 0;
    pci_read_config_dword(pdev, 0x5C, &mut conf);
    conf &= 0x78FFE088; // Keep the other bits
    if (*adev).dma_mode >= XFER_UDMA_0 {
        let udma = (*adev).dma_mode - XFER_UDMA_0;
        conf |= 0x80000000;
        conf |= ((udma + 2) as u32) << 28;
        conf |= ((2 - udma) as u32) * 0x111; // spread into three nibbles
    } else {
        static MWDMA: [u32; 4] = [0x0655, 0x0200, 0x0200, 0x0100];
        conf |= MWDMA[((*adev).dma_mode - XFER_MW_DMA_0) as usize];
    }
    pci_write_config_dword(pdev, 0x5C, conf);
}

static TOSH_SHT: scsi_host_template = ATA_BMDMA_SHT!(DRV_NAME);

static mut TOSH_PORT_OPS: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma_port_ops,
    cable_detect: Some(ata_cable_unknown),
    set_piomode: Some(tosh_set_piomode),
    set_dmamode: Some(tosh_set_dmamode),
};

/**
 * ata_tosh_init_one - attach generic IDE
 * @dev: PCI device found
 * @id: match entry
 *
 * Called each time a matching IDE interface is found. We check if the
 * interface is one we wish to claim and if so we perform any chip
 * specific hacks then let the ATA layer do the heavy lifting.
 */
unsafe fn ata_tosh_init_one(
    dev: *mut pci_dev,
    _id: *const pci_device_id,
) -> i32 {
    static INFO: ata_port_info = ata_port_info {
        flags: ATA_FLAG_SLAVE_POSS,
        pio_mask: ATA_PIO5,
        mwdma_mask: ATA_MWDMA2,
        udma_mask: ATA_UDMA2,
        port_ops: &TOSH_PORT_OPS,
    };
    let ppi: [*const ata_port_info; 2] = [&INFO, &ata_dummy_port_info];
    // Just one port for the moment
    ata_pci_bmdma_init_one(dev, ppi.as_ptr(), &TOSH_SHT, core::ptr::null_mut(), 0)
}

static ATA_TOSH: [pci_device_id; 5] = [
    PCI_DEVICE!(PCI_VENDOR_ID_TOSHIBA, PCI_DEVICE_ID_TOSHIBA_PICCOLO_1),
    PCI_DEVICE!(PCI_VENDOR_ID_TOSHIBA, PCI_DEVICE_ID_TOSHIBA_PICCOLO_2),
    PCI_DEVICE!(PCI_VENDOR_ID_TOSHIBA, PCI_DEVICE_ID_TOSHIBA_PICCOLO_3),
    PCI_DEVICE!(PCI_VENDOR_ID_TOSHIBA, PCI_DEVICE_ID_TOSHIBA_PICCOLO_5),
    pci_device_id { ..Default::default() },
];

static mut ATA_TOSH_PCI_DRIVER: pci_driver = pci_driver {
    name: DRV_NAME,
    id_table: ATA_TOSH.as_ptr(),
    probe: Some(ata_tosh_init_one),
    remove: Some(ata_pci_remove_one),
    // CONFIG_PM_SLEEP conditionally supplies suspend and resume in the C source.
};

// Equivalent of module_pci_driver(ata_tosh_pci_driver).
// MODULE_AUTHOR("Alan Cox");
// MODULE_DESCRIPTION("Low level driver for Toshiba Piccolo ATA");
// MODULE_LICENSE("GPL");
// MODULE_DEVICE_TABLE(pci, ata_tosh);
// MODULE_VERSION(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
