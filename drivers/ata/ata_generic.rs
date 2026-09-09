/*
 *  ata_generic.c - Generic PATA/SATA controller driver.
 *  Copyright 2005 Red Hat Inc, all rights reserved.
 *
 *  Elements from ide/pci/generic.c
 *	    Copyright (C) 2001-2002	Andre Hedrick <andre@linux-ide.org>
 *	    Portions (C) Copyright 2002  Red Hat Inc <alan@linux-ide.org>
 *
 *  May be copied or modified under the terms of the GNU General Public License
 *
 *  Driver for PCI IDE interfaces implementing the standard bus mastering
 *  interface functionality. This assumes the BIOS did the drive set up and
 *  tuning for us. By default we do not grab all IDE class devices as they
 *  may have other drivers or need fixups to avoid problems. Instead we keep
 *  a default list of stuff without documentation/driver that appears to
 *  work.
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

const DRV_NAME: &str = "ata_generic";
const DRV_VERSION: &str = "0.2.15";

/* A generic parallel ATA driver using libata */
const ATA_GEN_CLASS_MATCH: u64 = 1 << 0;
const ATA_GEN_FORCE_DMA: u64 = 1 << 1;
const ATA_GEN_INTEL_IDER: u64 = 1 << 2;

/*
 * generic_set_mode - mode setting
 * @link: link to set up
 * @unused: returned device on error
 *
 * Use a non standard set_mode function. We don't want to be tuned.
 * The BIOS configured everything. Our job is not to fiddle. We
 * read the dma enabled bits from the PCI configuration of the device
 * and respect them.
 */
unsafe fn generic_set_mode(link: *mut ata_link, _unused: *mut *mut ata_device) -> i32 {
    let ap = (*link).ap;
    let driver_data = (*(*ap).host).private_data as usize as u64;
    let mut dma_enabled: i32 = 0;

    if driver_data & ATA_GEN_FORCE_DMA != 0 {
        dma_enabled = 0xff;
    } else if (*ap).ioaddr.bmdma_addr != 0 {
        // Bits 5 and 6 indicate if DMA is active on master/slave.
        dma_enabled = ioread8((*ap).ioaddr.bmdma_addr + ATA_DMA_STATUS) as i32;
    }

    let mut dev: *mut ata_device = core::ptr::null_mut();
    ata_for_each_dev!(dev, link, ENABLED, {
        (*dev).pio_mode = XFER_PIO_0;
        (*dev).dma_mode = XFER_MW_DMA_0;
        if dma_enabled & (1 << (5 + (*dev).devno)) != 0 {
            let mut xfer_mask = ata_id_xfermask((*dev).id);
            let name: *const i8;
            if xfer_mask & (ATA_MASK_MWDMA | ATA_MASK_UDMA) != 0 {
                name = ata_mode_string(xfer_mask);
            } else {
                name = c"DMA".as_ptr();
                xfer_mask |= ata_xfer_mode2mask(XFER_MW_DMA_0);
            }
            ata_dev_info!((*dev), "configured for %s\n", name);
            (*dev).xfer_mode = ata_xfer_mask2mode(xfer_mask);
            (*dev).xfer_shift = ata_xfer_mode2shift((*dev).xfer_mode);
            (*dev).flags &= !ATA_DFLAG_PIO;
        } else {
            ata_dev_info!((*dev), "configured for PIO\n");
            (*dev).xfer_mode = XFER_PIO_0;
            (*dev).xfer_shift = ATA_SHIFT_PIO;
            (*dev).flags |= ATA_DFLAG_PIO;
        }
    });
    0
}

static generic_sht: scsi_host_template = ATA_BMDMA_SHT!(DRV_NAME);

static mut generic_port_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma_port_ops,
    cable_detect: Some(ata_cable_unknown),
    set_mode: Some(generic_set_mode),
};

static mut all_generic_ide: i32 = 0; // Set to claim all devices.

/* identify Intel IDE-R devices */
unsafe fn is_intel_ider(dev: *mut pci_dev) -> i32 {
    let mut r: u32 = 0;
    let mut t: u16 = 0;
    pci_read_config_dword(dev, 0xF8, &mut r);
    if r != 0 { return 0; }
    pci_read_config_word(dev, 0x40, &mut t);
    if t != 0 { return 0; }
    pci_write_config_word(dev, 0x40, 1);
    pci_read_config_word(dev, 0x40, &mut t);
    if t != 0 {
        pci_write_config_word(dev, 0x40, 0);
        return 0;
    }
    1
}

unsafe fn ata_generic_init_one(dev: *mut pci_dev, id: *const pci_device_id) -> i32 {
    let mut command: u16 = 0;
    static info: ata_port_info = ata_port_info {
        flags: ATA_FLAG_SLAVE_POSS,
        pio_mask: ATA_PIO4,
        mwdma_mask: ATA_MWDMA2,
        udma_mask: ATA_UDMA5,
        port_ops: &generic_port_ops,
    };
    let ppi: [*const ata_port_info; 2] = [&info, core::ptr::null()];

    if ((*id).driver_data & ATA_GEN_CLASS_MATCH) != 0 && all_generic_ide == 0 { return -ENODEV; }
    if ((*id).driver_data & ATA_GEN_INTEL_IDER) != 0 && all_generic_ide == 0 && is_intel_ider(dev) == 0 { return -ENODEV; }
    if (*dev).vendor == PCI_VENDOR_ID_UMC && (*dev).device == PCI_DEVICE_ID_UMC_UM8886A && (PCI_FUNC((*dev).devfn) & 1) == 0 { return -ENODEV; }
    if (*dev).vendor == PCI_VENDOR_ID_OPTI && (*dev).device == PCI_DEVICE_ID_OPTI_82C558 && (PCI_FUNC((*dev).devfn) & 1) == 0 { return -ENODEV; }
    pci_read_config_word(dev, PCI_COMMAND, &mut command);
    if command & PCI_COMMAND_IO == 0 { return -ENODEV; }
    if (*dev).vendor == PCI_VENDOR_ID_AL { ata_pci_bmdma_clear_simplex(dev); }
    if (*dev).vendor == PCI_VENDOR_ID_ATI {
        let rc = pcim_enable_device(dev);
        if rc < 0 { return rc; }
        pcim_pin_device(dev);
    }
    ata_pci_bmdma_init_one(dev, ppi.as_ptr(), &generic_sht, (*id).driver_data as *mut core::ffi::c_void, 0)
}

// PCI ID table; configuration-dependent Toshiba entries are retained by intent.
static ata_generic: [pci_device_id; 12] = [
    PCI_DEVICE!(PCI_VENDOR_ID_PCTECH, PCI_DEVICE_ID_PCTECH_SAMURAI_IDE),
    PCI_DEVICE!(PCI_VENDOR_ID_HOLTEK, PCI_DEVICE_ID_HOLTEK_6565),
    PCI_DEVICE!(PCI_VENDOR_ID_UMC, PCI_DEVICE_ID_UMC_UM8673F),
    PCI_DEVICE!(PCI_VENDOR_ID_UMC, PCI_DEVICE_ID_UMC_UM8886A),
    PCI_DEVICE!(PCI_VENDOR_ID_UMC, PCI_DEVICE_ID_UMC_UM8886BF),
    PCI_DEVICE!(PCI_VENDOR_ID_HINT, PCI_DEVICE_ID_HINT_VXPROII_IDE),
    PCI_DEVICE!(PCI_VENDOR_ID_VIA, PCI_DEVICE_ID_VIA_82C561),
    PCI_DEVICE!(PCI_VENDOR_ID_OPTI, PCI_DEVICE_ID_OPTI_82C558),
    PCI_DEVICE_DATA!(PCI_VENDOR_ID_CENATEK, PCI_DEVICE_ID_CENATEK_IDE, ATA_GEN_FORCE_DMA),
    PCI_DEVICE_CLASS_DATA!(PCI_VENDOR_ID_INTEL, PCI_CLASS_STORAGE_IDE << 8, 0xFFFFFF00u32, ATA_GEN_INTEL_IDER),
    PCI_DEVICE_CLASS_DATA!(PCI_CLASS_STORAGE_IDE << 8, 0xFFFFFF00u32, ATA_GEN_CLASS_MATCH),
    PCI_DEVICE!(0, 0),
];

static mut ata_generic_pci_driver: pci_driver = pci_driver {
    name: DRV_NAME,
    id_table: &ata_generic,
    probe: Some(ata_generic_init_one),
    remove: Some(ata_pci_remove_one),
    // CONFIG_PM_SLEEP conditionally supplies suspend and resume.
};

// module_pci_driver(ata_generic_pci_driver);
// MODULE_AUTHOR("Alan Cox");
// MODULE_DESCRIPTION("low-level driver for generic ATA");
// MODULE_LICENSE("GPL");
// MODULE_DEVICE_TABLE(pci, ata_generic);
// MODULE_VERSION(DRV_VERSION);
// module_param(all_generic_ide, int, 0);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
