// SPDX-License-Identifier: GPL-2.0-only
/*
 * pata_pdc202xx_old.c - Promise PDC202xx PATA for new ATA layer
 * (C) 2005 Red Hat Inc
 * Alan Cox <alan@lxorguk.ukuu.org.uk>
 * (C) 2007,2009,2010 Bartlomiej Zolnierkiewicz
 *
 * Based in part on linux/drivers/ide/pci/pdc202xx_old.c
 * First cut with LBA48/ATAPI
 * TODO: Channel interlock/reset on both required ?
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

const DRV_NAME: &[u8] = b"pata_pdc202xx_old\0";
const DRV_VERSION: &[u8] = b"0.4.3\0";

unsafe fn pdc2026x_cable_detect(ap: *mut ata_port) -> i32 {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut cis: u16 = 0;
    pci_read_config_word(pdev, 0x50, &mut cis);
    if cis & (1 << (10 + (*ap).port_no)) != 0 { ATA_CBL_PATA40 } else { ATA_CBL_PATA80 }
}

unsafe fn pdc202xx_exec_command(ap: *mut ata_port, tf: *const ata_taskfile) {
    iowrite8((*tf).command, (*ap).ioaddr.command_addr);
    ndelay(400);
}

unsafe fn pdc202xx_irq_check(ap: *mut ata_port) -> bool {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let master = pci_resource_start(pdev, 4);
    let sc1d = inb(master + 0x1d);
    if (*ap).port_no != 0 { (sc1d & 0x40) != 0 } else { (sc1d & 0x04) != 0 }
}

unsafe fn pdc202xx_configure_piomode(ap: *mut ata_port, adev: *mut ata_device, pio: i32) {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let port = 0x60 + 8 * (*ap).port_no + 4 * (*adev).devno;
    let pio_timing: [u16; 5] = [0x0913, 0x050C, 0x0308, 0x0206, 0x0104];
    let mut r_ap: u8 = 0;
    let mut r_bp: u8 = 0;
    pci_read_config_byte(pdev, port, &mut r_ap);
    pci_read_config_byte(pdev, port + 1, &mut r_bp);
    r_ap &= !0x3F;
    r_bp &= !0x1F;
    r_ap |= (pio_timing[pio as usize] >> 8) as u8;
    r_bp |= (pio_timing[pio as usize] & 0xFF) as u8;
    if ata_pio_need_iordy(adev) { r_ap |= 0x20; }
    if (*adev).class == ATA_DEV_ATA { r_ap |= 0x10; }
    pci_write_config_byte(pdev, port, r_ap);
    pci_write_config_byte(pdev, port + 1, r_bp);
}

unsafe fn pdc202xx_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    pdc202xx_configure_piomode(ap, adev, (*adev).pio_mode - XFER_PIO_0);
}

unsafe fn pdc202xx_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let port = 0x60 + 8 * (*ap).port_no + 4 * (*adev).devno;
    let udma_timing: [[u8; 2]; 6] = [[0x60,0x03],[0x40,0x02],[0x20,0x01],[0x40,0x02],[0x20,0x01],[0x20,0x01]];
    let mdma_timing: [[u8; 2]; 3] = [[0xe0,0x0f],[0x60,0x04],[0x60,0x03]];
    let (mut r_bp, mut r_cp) = (0u8, 0u8);
    pci_read_config_byte(pdev, port + 1, &mut r_bp);
    pci_read_config_byte(pdev, port + 2, &mut r_cp);
    r_bp &= !0xE0; r_cp &= !0x0F;
    if (*adev).dma_mode >= XFER_UDMA_0 {
        let t = udma_timing[((*adev).dma_mode - XFER_UDMA_0) as usize]; r_bp |= t[0]; r_cp |= t[1];
    } else {
        let t = mdma_timing[((*adev).dma_mode - XFER_MW_DMA_0) as usize]; r_bp |= t[0]; r_cp |= t[1];
    }
    pci_write_config_byte(pdev, port + 1, r_bp); pci_write_config_byte(pdev, port + 2, r_cp);
}

unsafe fn pdc2026x_bmdma_start(qc: *mut ata_queued_cmd) {
    let ap = (*qc).ap; let adev = (*qc).dev; let tf = &mut (*qc).tf;
    let sel66 = if (*ap).port_no != 0 { 0x08 } else { 0x02 };
    let master = (*(*ap).host).ports[0].ioaddr.bmdma_addr;
    let clock = (master as *mut u8).add(0x11); let atapi_reg = (master as *mut u8).add(0x20 + 4 * (*ap).port_no);
    if (*adev).dma_mode > XFER_UDMA_2 { iowrite8(ioread8(clock) | sel66, clock); } else { iowrite8(ioread8(clock) & !sel66, clock); }
    pdc202xx_set_dmamode(ap, (*qc).dev);
    if ((*tf).flags & ATA_TFLAG_LBA48) != 0 || (*tf).protocol == ATAPI_PROT_DMA {
        let mut len = ((*qc).nbytes / 2) as u32;
        len |= if ((*tf).flags & ATA_TFLAG_WRITE) != 0 { 0x06000000 } else { 0x05000000 };
        iowrite32(len, atapi_reg);
    }
    ata_bmdma_start(qc);
}

unsafe fn pdc2026x_bmdma_stop(qc: *mut ata_queued_cmd) {
    let ap = (*qc).ap; let adev = (*qc).dev; let tf = &(*qc).tf;
    let sel66 = if (*ap).port_no != 0 { 0x08 } else { 0x02 };
    let master = (*(*ap).host).ports[0].ioaddr.bmdma_addr;
    let clock = (master as *mut u8).add(0x11); let atapi_reg = (master as *mut u8).add(0x20 + 4 * (*ap).port_no);
    if (*tf).protocol == ATAPI_PROT_DMA || ((*tf).flags & ATA_TFLAG_LBA48) != 0 { iowrite32(0, atapi_reg); iowrite8(ioread8(clock) & !sel66, clock); }
    if (*adev).dma_mode > XFER_UDMA_2 { iowrite8(ioread8(clock) & !sel66, clock); }
    ata_bmdma_stop(qc); pdc202xx_set_piomode(ap, adev);
}

unsafe fn pdc2026x_dev_config(adev: *mut ata_device) { (*adev).max_sectors = 256; }
unsafe fn pdc2026x_port_start(ap: *mut ata_port) -> i32 {
    let bmdma = (*ap).ioaddr.bmdma_addr;
    if !bmdma.is_null() { let burst = ioread8((bmdma as *mut u8).add(0x1f)); iowrite8(burst | 1, (bmdma as *mut u8).add(0x1f)); }
    ata_bmdma_port_start(ap)
}
unsafe fn pdc2026x_check_atapi_dma(_qc: *mut ata_queued_cmd) -> i32 { 1 }

static mut pdc202xx_sht: scsi_host_template = scsi_host_template { /* ATA_BMDMA_SHT(DRV_NAME) */ };
static mut pdc2024x_port_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma_port_ops, cable_detect: Some(ata_cable_40wire), set_piomode: Some(pdc202xx_set_piomode), set_dmamode: Some(pdc202xx_set_dmamode),
    sff_exec_command: Some(pdc202xx_exec_command), sff_irq_check: Some(pdc202xx_irq_check),
};
static mut pdc2026x_port_ops: ata_port_operations = ata_port_operations {
    inherits: &pdc2024x_port_ops, check_atapi_dma: Some(pdc2026x_check_atapi_dma), bmdma_start: Some(pdc2026x_bmdma_start), bmdma_stop: Some(pdc2026x_bmdma_stop),
    cable_detect: Some(pdc2026x_cable_detect), dev_config: Some(pdc2026x_dev_config), port_start: Some(pdc2026x_port_start),
    sff_exec_command: Some(pdc202xx_exec_command), sff_irq_check: Some(pdc202xx_irq_check),
};

unsafe fn pdc202xx_init_one(dev: *mut pci_dev, id: *const pci_device_id) -> i32 {
    static mut info: [ata_port_info; 3] = [
        ata_port_info { flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: ATA_MWDMA2, udma_mask: ATA_UDMA2, port_ops: &pdc2024x_port_ops },
        ata_port_info { flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: ATA_MWDMA2, udma_mask: ATA_UDMA4, port_ops: &pdc2026x_port_ops },
        ata_port_info { flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: ATA_MWDMA2, udma_mask: ATA_UDMA5, port_ops: &pdc2026x_port_ops },
    ];
    let ppi = [&info[(*id).driver_data as usize], core::ptr::null()];
    if (*dev).device == PCI_DEVICE_ID_PROMISE_20265 {
        let bridge = (*dev).bus.self_;
        if !bridge.is_null() && (*bridge).vendor == PCI_VENDOR_ID_INTEL {
            if (*bridge).device == PCI_DEVICE_ID_INTEL_I960 || (*bridge).device == PCI_DEVICE_ID_INTEL_I960RM { return -ENODEV; }
        }
    }
    ata_pci_bmdma_init_one(dev, ppi.as_ptr(), &pdc202xx_sht, core::ptr::null_mut(), 0)
}

static mut pdc202xx: [pci_device_id; 6] = [
    pci_device_id { vendor: PCI_VENDOR_ID_PROMISE, device: PCI_DEVICE_ID_PROMISE_20246, driver_data: 0 },
    pci_device_id { vendor: PCI_VENDOR_ID_PROMISE, device: PCI_DEVICE_ID_PROMISE_20262, driver_data: 1 },
    pci_device_id { vendor: PCI_VENDOR_ID_PROMISE, device: PCI_DEVICE_ID_PROMISE_20263, driver_data: 1 },
    pci_device_id { vendor: PCI_VENDOR_ID_PROMISE, device: PCI_DEVICE_ID_PROMISE_20265, driver_data: 2 },
    pci_device_id { vendor: PCI_VENDOR_ID_PROMISE, device: PCI_DEVICE_ID_PROMISE_20267, driver_data: 2 },
    pci_device_id { vendor: 0, device: 0, driver_data: 0 },
];

static mut pdc202xx_pci_driver: pci_driver = pci_driver {
    name: DRV_NAME.as_ptr(), id_table: pdc202xx.as_ptr(), probe: Some(pdc202xx_init_one), remove: Some(ata_pci_remove_one),
    // CONFIG_PM_SLEEP: suspend = ata_pci_device_suspend, resume = ata_pci_device_resume
};

// module_pci_driver(pdc202xx_pci_driver)
// MODULE_AUTHOR("Alan Cox"); MODULE_DESCRIPTION("low-level driver for Promise 2024x and 20262-20267");
// MODULE_LICENSE("GPL"); MODULE_DEVICE_TABLE(pci, pdc202xx); MODULE_VERSION(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
