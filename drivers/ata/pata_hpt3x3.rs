/*
 *\tpata_hpt3x3\t\t-\tHPT3x3 driver
 *\t(c) Copyright 2005-2006 Red Hat
 *
 *\tWas pata_hpt34x but the naming was confusing as it supported the
 *\t343 and 363 so it has been renamed.
 *
 *\tBased on:
 *\tlinux/drivers/ide/pci/hpt34x.c\t\tVersion 0.40\tSept 10, 2002
 *\tCopyright (C) 1998-2000\tAndre Hedrick <andre@linux-ide.org>
 *
 *\tMay be copied or modified under the terms of the GNU General Public
 *\tLicense
 */

// C dependencies supplied by the surrounding kernel translation.

const DRV_NAME: &str = "pata_hpt3x3";
const DRV_VERSION: &str = "0.6.1";

/** PIO setup */
unsafe fn hpt3x3_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut r1: u32 = 0;
    let mut r2: u32 = 0;
    let dn: i32 = 2 * (*ap).port_no + (*adev).devno;

    pci_read_config_dword(pdev, 0x44, &mut r1);
    pci_read_config_dword(pdev, 0x48, &mut r2);
    // Load the PIO timing number
    r1 &= !(7u32 << (3 * dn));
    r1 |= ((*adev).pio_mode - XFER_PIO_0) << (3 * dn);
    r2 &= !(0x11u32 << dn); // Clear MWDMA and UDMA bits

    pci_write_config_dword(pdev, 0x44, r1);
    pci_write_config_dword(pdev, 0x48, r2);
}

#[cfg(feature = "CONFIG_PATA_HPT3X3_DMA")]
unsafe fn hpt3x3_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut r1: u32 = 0;
    let mut r2: u32 = 0;
    let dn: i32 = 2 * (*ap).port_no + (*adev).devno;
    let mode_num = (*adev).dma_mode & 0x0F;

    pci_read_config_dword(pdev, 0x44, &mut r1);
    pci_read_config_dword(pdev, 0x48, &mut r2);
    // Load the timing number
    r1 &= !(7u32 << (3 * dn));
    r1 |= mode_num << (3 * dn);
    r2 &= !(0x11u32 << dn); // Clear MWDMA and UDMA bits

    if (*adev).dma_mode >= XFER_UDMA_0 {
        r2 |= 0x01u32 << dn; // Ultra mode
    } else {
        r2 |= 0x10u32 << dn; // MWDMA
    }

    pci_write_config_dword(pdev, 0x44, r1);
    pci_write_config_dword(pdev, 0x48, r2);
}

#[cfg(feature = "CONFIG_PATA_HPT3X3_DMA")]
unsafe fn hpt3x3_freeze(ap: *mut ata_port) {
    let mmio = (*ap).ioaddr.bmdma_addr;
    iowrite8(ioread8(mmio.add(ATA_DMA_CMD as usize)) & !ATA_DMA_START,
             mmio.add(ATA_DMA_CMD as usize));
    ata_sff_dma_pause(ap);
    ata_sff_freeze(ap);
}

#[cfg(feature = "CONFIG_PATA_HPT3X3_DMA")]
unsafe fn hpt3x3_bmdma_setup(qc: *mut ata_queued_cmd) {
    let ap = (*qc).ap;
    let mut r = ioread8((*ap).ioaddr.bmdma_addr.add(ATA_DMA_STATUS as usize));
    r |= ATA_DMA_INTR | ATA_DMA_ERR;
    iowrite8(r, (*ap).ioaddr.bmdma_addr.add(ATA_DMA_STATUS as usize));
    ata_bmdma_setup(qc);
}

#[cfg(feature = "CONFIG_PATA_HPT3X3_DMA")]
unsafe fn hpt3x3_atapi_dma(_qc: *mut ata_queued_cmd) -> i32 { 1 }

static mut hpt3x3_sht: scsi_host_template = ATA_BMDMA_SHT!(DRV_NAME);

static mut hpt3x3_port_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma_port_ops,
    cable_detect: Some(ata_cable_40wire),
    set_piomode: Some(hpt3x3_set_piomode),
    #[cfg(feature = "CONFIG_PATA_HPT3X3_DMA")]
    set_dmamode: Some(hpt3x3_set_dmamode),
    #[cfg(feature = "CONFIG_PATA_HPT3X3_DMA")]
    bmdma_setup: Some(hpt3x3_bmdma_setup),
    #[cfg(feature = "CONFIG_PATA_HPT3X3_DMA")]
    check_atapi_dma: Some(hpt3x3_atapi_dma),
    #[cfg(feature = "CONFIG_PATA_HPT3X3_DMA")]
    freeze: Some(hpt3x3_freeze),
};

unsafe fn hpt3x3_init_chipset(dev: *mut pci_dev) {
    let mut cmd: u16 = 0;
    pci_write_config_word(dev, 0x80, 0x00);
    pci_read_config_word(dev, PCI_COMMAND, &mut cmd);
    if cmd & PCI_COMMAND_MEMORY != 0 {
        pci_write_config_byte(dev, PCI_LATENCY_TIMER, 0xF0);
    } else {
        pci_write_config_byte(dev, PCI_LATENCY_TIMER, 0x20);
    }
}

unsafe fn hpt3x3_init_one(pdev: *mut pci_dev, _id: *const pci_device_id) -> i32 {
    static mut info: ata_port_info = ata_port_info {
        flags: ATA_FLAG_SLAVE_POSS,
        pio_mask: ATA_PIO4,
        #[cfg(feature = "CONFIG_PATA_HPT3X3_DMA")]
        mwdma_mask: ATA_MWDMA2,
        #[cfg(feature = "CONFIG_PATA_HPT3X3_DMA")]
        udma_mask: ATA_UDMA2,
        port_ops: &hpt3x3_port_ops,
    };
    static offset_cmd: [u8; 2] = [0x20, 0x28];
    static offset_ctl: [u8; 2] = [0x36, 0x3E];
    let ppi: [*const ata_port_info; 2] = [&info, core::ptr::null()];
    let host: *mut ata_host;
    let mut rc: i32;
    let base: *mut core::ffi::c_void;

    hpt3x3_init_chipset(pdev);
    ata_print_version_once(&(*pdev).dev, DRV_VERSION);
    host = ata_host_alloc_pinfo(&(*pdev).dev, ppi.as_ptr(), 2);
    if host.is_null() { return -ENOMEM; }
    rc = pcim_enable_device(pdev);
    if rc != 0 { return rc; }
    rc = pcim_iomap_regions(pdev, 1 << 4, DRV_NAME);
    if rc == -EBUSY { pcim_pin_device(pdev); }
    if rc != 0 { return rc; }
    (*host).iomap = pcim_iomap_table(pdev);
    rc = dma_set_mask_and_coherent(&mut (*pdev).dev, ATA_DMA_MASK);
    if rc != 0 { return rc; }
    base = (*host).iomap[4];

    for i in 0..(*host).n_ports {
        let ap = (*host).ports[i as usize];
        let ioaddr = &mut (*ap).ioaddr;
        ioaddr.cmd_addr = base.add(offset_cmd[i as usize] as usize);
        ioaddr.altstatus_addr = base.add(offset_ctl[i as usize] as usize);
        ioaddr.ctl_addr = base.add(offset_ctl[i as usize] as usize);
        ioaddr.scr_addr = core::ptr::null_mut();
        ata_sff_std_ports(ioaddr);
        ioaddr.bmdma_addr = base.add((8 * i) as usize);
        ata_port_pbar_desc(ap, 4, -1, "ioport");
        ata_port_pbar_desc(ap, 4, offset_cmd[i as usize] as i32, "cmd");
    }
    pci_set_master(pdev);
    ata_host_activate(host, (*pdev).irq, ata_bmdma_interrupt, IRQF_SHARED, &hpt3x3_sht)
}

#[cfg(feature = "CONFIG_PM_SLEEP")]
unsafe fn hpt3x3_reinit_one(dev: *mut pci_dev) -> i32 {
    let host = pci_get_drvdata(dev);
    let rc = ata_pci_device_do_resume(dev);
    if rc != 0 { return rc; }
    hpt3x3_init_chipset(dev);
    ata_host_resume(host);
    0
}

static hpt3x3: [pci_device_id; 2] = [
    PCI_VDEVICE!(TTI, PCI_DEVICE_ID_TTI_HPT343),
    pci_device_id {},
];

static mut hpt3x3_pci_driver: pci_driver = pci_driver {
    name: DRV_NAME,
    id_table: &hpt3x3,
    probe: Some(hpt3x3_init_one),
    remove: Some(ata_pci_remove_one),
    #[cfg(feature = "CONFIG_PM_SLEEP")]
    suspend: Some(ata_pci_device_suspend),
    #[cfg(feature = "CONFIG_PM_SLEEP")]
    resume: Some(hpt3x3_reinit_one),
};

module_pci_driver!(hpt3x3_pci_driver);
module_author!("Alan Cox");
module_description!("low-level driver for the Highpoint HPT343/363");
module_license!("GPL");
module_device_table!(pci, hpt3x3);
module_version!(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
