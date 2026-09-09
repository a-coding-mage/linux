// SPDX-License-Identifier: GPL-2.0-only
/*
 * pata-cs5530.c - CS5530 PATA for new ATA layer
 * (C) 2005 Red Hat Inc
 *
 * based upon cs5530.c by Mark Lord.
 * Loosely based on the piix & svwks drivers.
 */

const DRV_NAME: &[u8] = b"pata_cs5530\0";
const DRV_VERSION: &[u8] = b"0.7.4\0";

unsafe fn cs5530_port_base(ap: *mut ata_port) -> *mut core::ffi::c_void {
    let bmdma = (*ap).ioaddr.bmdma_addr as usize;
    ((bmdma & !0x0f) + 0x20 + 0x10 * (*ap).port_no as usize) as *mut core::ffi::c_void
}

unsafe fn cs5530_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    static CS5530_PIO_TIMINGS: [[u32; 5]; 2] = [
        [0x00009172, 0x00012171, 0x00020080, 0x00032010, 0x00040010],
        [0xd1329172, 0x71212171, 0x30200080, 0x20102010, 0x00100010],
    ];
    let mut base = cs5530_port_base(ap) as usize;
    let tuning = ioread32((base + 0x04) as *mut core::ffi::c_void);
    let format = if (tuning & 0x80000000) != 0 { 1 } else { 0 };
    if (*adev).devno != 0 {
        base += 0x08;
    }
    iowrite32(
        CS5530_PIO_TIMINGS[format][((*adev).pio_mode - XFER_PIO_0) as usize],
        base as *mut core::ffi::c_void,
    );
}

unsafe fn cs5530_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    let base = cs5530_port_base(ap) as usize;
    let tuning = ioread32((base + 0x04) as *mut core::ffi::c_void);
    let mut timing: u32 = match (*adev).dma_mode {
        XFER_UDMA_0 => 0x00921250,
        XFER_UDMA_1 => 0x00911140,
        XFER_UDMA_2 => 0x00911030,
        XFER_MW_DMA_0 => 0x00077771,
        XFER_MW_DMA_1 => 0x00012121,
        XFER_MW_DMA_2 => 0x00002020,
        _ => { BUG(); 0 }
    };
    timing |= tuning & 0x80000000;
    if (*adev).devno == 0 {
        iowrite32(timing, (base + 0x04) as *mut core::ffi::c_void);
    } else {
        let mut new_tuning = tuning;
        if (timing & 0x00100000) != 0 {
            new_tuning |= 0x00100000;
        } else {
            new_tuning &= !0x00100000;
        }
        iowrite32(new_tuning, (base + 0x04) as *mut core::ffi::c_void);
        iowrite32(timing, (base + 0x0c) as *mut core::ffi::c_void);
    }
    let mut reg = ioread8((*ap).ioaddr.bmdma_addr.add(ATA_DMA_STATUS as usize));
    reg |= 1 << (5 + (*adev).devno);
    iowrite8(reg, (*ap).ioaddr.bmdma_addr.add(ATA_DMA_STATUS as usize));
    (*ap).private_data = adev as *mut core::ffi::c_void;
}

unsafe fn cs5530_qc_issue(qc: *mut ata_queued_cmd) -> u32 {
    let ap = (*qc).ap;
    let adev = (*qc).dev;
    let prev = (*ap).private_data as *mut ata_device;
    if ata_dma_enabled(adev) && adev != prev && !prev.is_null()
        && (ata_using_udma(adev) != ata_using_udma(prev))
    {
        cs5530_set_dmamode(ap, adev);
    }
    ata_bmdma_qc_issue(qc)
}

static CS5530_SHT: scsi_host_template = scsi_host_template {
    base: ATA_BASE_SHT(DRV_NAME),
    sg_tablesize: LIBATA_DUMB_MAX_PRD,
    dma_boundary: ATA_DMA_BOUNDARY,
};

static mut CS5530_PORT_OPS: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma_port_ops,
    qc_prep: Some(ata_bmdma_dumb_qc_prep),
    qc_issue: Some(cs5530_qc_issue),
    cable_detect: Some(ata_cable_40wire),
    set_piomode: Some(cs5530_set_piomode),
    set_dmamode: Some(cs5530_set_dmamode),
};

static PALMAX_DMI_TABLE: [dmi_system_id; 2] = [
    dmi_system_id {
        ident: b"Palmax PD1100\0",
        matches: [DMI_MATCH(DMI_SYS_VENDOR, b"Cyrix\0"), DMI_MATCH(DMI_PRODUCT_NAME, b"Caddis\0")],
    },
    dmi_system_id::default(),
];

unsafe fn cs5530_is_palmax() -> i32 {
    if dmi_check_system(PALMAX_DMI_TABLE.as_ptr()) != 0 {
        printk(KERN_INFO, b"Palmax PD1100: Disabling DMA on docking port.\0");
        return 1;
    }
    0
}

unsafe fn cs5530_init_chip() -> i32 {
    let mut master_0: *mut pci_dev = core::ptr::null_mut();
    let mut cs5530_0: *mut pci_dev = core::ptr::null_mut();
    let mut dev: *mut pci_dev = core::ptr::null_mut();
    while { dev = pci_get_device(PCI_VENDOR_ID_CYRIX, PCI_ANY_ID, dev); !dev.is_null() } {
        match (*dev).device {
            PCI_DEVICE_ID_CYRIX_PCI_MASTER => master_0 = pci_dev_get(dev),
            PCI_DEVICE_ID_CYRIX_5530_LEGACY => cs5530_0 = pci_dev_get(dev),
            _ => {}
        }
    }
    if master_0.is_null() {
        printk(KERN_ERR, b"pata_cs5530: unable to locate PCI MASTER function\0");
        goto_fail_put(master_0, cs5530_0);
        return -ENODEV;
    }
    if cs5530_0.is_null() {
        printk(KERN_ERR, b"pata_cs5530: unable to locate CS5530 LEGACY function\0");
        goto_fail_put(master_0, cs5530_0);
        return -ENODEV;
    }
    pci_set_master(cs5530_0);
    pci_try_set_mwi(cs5530_0);
    pci_write_config_byte(cs5530_0, PCI_CACHE_LINE_SIZE, 0x04);
    pci_write_config_word(cs5530_0, 0xd0, 0x5006);
    pci_write_config_byte(master_0, 0x40, 0x1e);
    pci_write_config_byte(master_0, 0x41, 0x14);
    pci_write_config_byte(master_0, 0x42, 0x00);
    pci_write_config_byte(master_0, 0x43, 0xc1);
    pci_dev_put(master_0);
    pci_dev_put(cs5530_0);
    0
}

unsafe fn goto_fail_put(master_0: *mut pci_dev, cs5530_0: *mut pci_dev) {
    pci_dev_put(master_0);
    pci_dev_put(cs5530_0);
}

unsafe fn cs5530_init_one(pdev: *mut pci_dev, _id: *const pci_device_id) -> i32 {
    static mut INFO: ata_port_info = ata_port_info { flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: ATA_MWDMA2, udma_mask: ATA_UDMA2, port_ops: &CS5530_PORT_OPS };
    static mut INFO_PALMAX_SECONDARY: ata_port_info = ata_port_info { flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: 0, udma_mask: 0, port_ops: &CS5530_PORT_OPS };
    let mut ppi = [&mut INFO as *mut ata_port_info, core::ptr::null_mut()];
    let rc = pcim_enable_device(pdev);
    if rc != 0 { return rc; }
    if cs5530_init_chip() != 0 { return -ENODEV; }
    if cs5530_is_palmax() != 0 { ppi[1] = &mut INFO_PALMAX_SECONDARY; }
    ata_pci_bmdma_init_one(pdev, ppi.as_ptr(), &CS5530_SHT, core::ptr::null_mut(), 0)
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn cs5530_reinit_one(pdev: *mut pci_dev) -> i32 {
    let host = pci_get_drvdata(pdev) as *mut ata_host;
    let rc = ata_pci_device_do_resume(pdev);
    if rc != 0 { return rc; }
    if cs5530_init_chip() != 0 { return -EIO; }
    ata_host_resume(host);
    0
}

static CS5530: [pci_device_id; 2] = [
    PCI_VDEVICE(CYRIX, PCI_DEVICE_ID_CYRIX_5530_IDE),
    pci_device_id::default(),
];

static mut CS5530_PCI_DRIVER: pci_driver = pci_driver {
    name: DRV_NAME,
    id_table: CS5530.as_ptr(),
    probe: Some(cs5530_init_one),
    remove: Some(ata_pci_remove_one),
    #[cfg(CONFIG_PM_SLEEP)]
    suspend: Some(ata_pci_device_suspend),
    #[cfg(CONFIG_PM_SLEEP)]
    resume: Some(cs5530_reinit_one),
};

module_pci_driver!(CS5530_PCI_DRIVER);
MODULE_AUTHOR!(b"Alan Cox\0");
MODULE_DESCRIPTION!(b"low-level driver for the Cyrix/NS/AMD 5530\0");
MODULE_LICENSE!(b"GPL\0");
MODULE_DEVICE_TABLE!(pci, CS5530);
MODULE_VERSION!(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
