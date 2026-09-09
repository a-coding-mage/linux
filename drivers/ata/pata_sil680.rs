/*
 * pata_sil680.c 	- SIL680 PATA for new ATA layer
 *			  (C) 2005 Red Hat Inc
 *
 * based upon linux/drivers/ide/pci/siimage.c
 *
 * Copyright (C) 2001-2002 Andre Hedrick <andre@linux-ide.org>
 * Copyright (C) 2003 Red Hat <alan@redhat.com>
 *
 * May be copied or modified under the terms of the GNU General Public License
 */

// Linux kernel dependencies supplied by other translation units.

pub const DRV_NAME: &str = "pata_sil680";
pub const DRV_VERSION: &str = "0.4.9";
pub const SIL680_MMIO_BAR: usize = 5;

unsafe fn sil680_selreg(ap: *mut ata_port, r: i32) -> i32 {
    0xA0 + ((*ap).port_no << 4) + r
}

unsafe fn sil680_seldev(ap: *mut ata_port, adev: *mut ata_device, r: i32) -> i32 {
    0xA0 + ((*ap).port_no << 4) + r + ((*adev).devno << 1)
}

unsafe fn sil680_cable_detect(ap: *mut ata_port) -> i32 {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let addr = sil680_selreg(ap, 0);
    let mut ata66: u8 = 0;
    pci_read_config_byte(pdev, addr, &mut ata66);
    if ata66 & 1 != 0 { ATA_CBL_PATA80 } else { ATA_CBL_PATA40 }
}

unsafe fn sil680_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    let speed_p: [u16; 5] = [0x328A, 0x2283, 0x1104, 0x10C3, 0x10C1];
    let speed_t: [u16; 5] = [0x328A, 0x2283, 0x1281, 0x10C3, 0x10C1];
    let tfaddr = sil680_selreg(ap, 0x02);
    let addr = sil680_seldev(ap, adev, 0x04);
    let addr_mask = 0x80 + 4 * (*ap).port_no;
    let pdev = to_pci_dev((*(*ap).host).dev);
    let pio = (*adev).pio_mode - XFER_PIO_0;
    let mut lowest_pio = pio;
    let port_shift = 4 * (*adev).devno;
    let mut reg: u16 = 0;
    let mut mode: u8 = 0;
    let pair = ata_dev_pair(adev);
    if !pair.is_null() && (*adev).pio_mode > (*pair).pio_mode { lowest_pio = (*pair).pio_mode - XFER_PIO_0; }
    pci_write_config_word(pdev, addr, speed_p[pio as usize]);
    pci_write_config_word(pdev, tfaddr, speed_t[lowest_pio as usize]);
    pci_read_config_word(pdev, tfaddr - 2, &mut reg);
    pci_read_config_byte(pdev, addr_mask, &mut mode);
    reg &= !0x0200;
    mode &= !((3u8) << port_shift);
    if ata_pio_need_iordy(adev) { reg |= 0x0200; mode |= 1u8 << port_shift; }
    pci_write_config_word(pdev, tfaddr - 2, reg);
    pci_write_config_byte(pdev, addr_mask, mode);
}

unsafe fn sil680_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    let ultra_table: [[u8; 7]; 2] = [[0x0C, 0x07, 0x05, 0x04, 0x02, 0x01, 0xFF], [0x0F, 0x0B, 0x07, 0x05, 0x03, 0x02, 0x01]];
    let dma_table: [u16; 3] = [0x2208, 0x10C2, 0x10C1];
    let pdev = to_pci_dev((*(*ap).host).dev);
    let ma = sil680_seldev(ap, adev, 0x08);
    let ua = sil680_seldev(ap, adev, 0x0C);
    let addr_mask = 0x80 + 4 * (*ap).port_no;
    let port_shift = (*adev).devno * 4;
    let (mut scsc, mut mode, mut multi, mut ultra): (u8, u8, u16, u16) = (0, 0, 0, 0);
    pci_read_config_byte(pdev, 0x8A, &mut scsc); pci_read_config_byte(pdev, addr_mask, &mut mode);
    pci_read_config_word(pdev, ma, &mut multi); pci_read_config_word(pdev, ua, &mut ultra);
    ultra &= !0x3F; mode &= !(0x03u8 << port_shift); scsc = if scsc & 0x30 != 0 { 1 } else { 0 };
    if (*adev).dma_mode >= XFER_UDMA_0 { multi = 0x10C1; ultra |= ultra_table[scsc as usize][((*adev).dma_mode - XFER_UDMA_0) as usize]; mode |= 0x03u8 << port_shift; }
    else { multi = dma_table[((*adev).dma_mode - XFER_MW_DMA_0) as usize]; mode |= 0x02u8 << port_shift; }
    pci_write_config_byte(pdev, addr_mask, mode); pci_write_config_word(pdev, ma, multi); pci_write_config_word(pdev, ua, ultra);
}

unsafe fn sil680_sff_exec_command(ap: *mut ata_port, tf: *const ata_taskfile) {
    iowrite8((*tf).command, (*ap).ioaddr.command_addr);
    ioread8((*ap).ioaddr.bmdma_addr + ATA_DMA_CMD);
}

unsafe fn sil680_sff_irq_check(ap: *mut ata_port) -> bool {
    let pdev = to_pci_dev((*(*ap).host).dev); let addr = sil680_selreg(ap, 1); let mut val = 0u8;
    pci_read_config_byte(pdev, addr, &mut val); val & 0x08 != 0
}

static mut SIL680_SHT: scsi_host_template = scsi_host_template { ata_bmdma_sht: ATA_BMDMA_SHT!(DRV_NAME) };
static mut sil680_sht: scsi_host_template = SIL680_SHT;
static mut sil680_port_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma32_port_ops,
    sff_exec_command: Some(sil680_sff_exec_command), sff_irq_check: Some(sil680_sff_irq_check),
    cable_detect: Some(sil680_cable_detect), set_piomode: Some(sil680_set_piomode), set_dmamode: Some(sil680_set_dmamode),
};

unsafe fn sil680_init_chip(pdev: *mut pci_dev, try_mmio: *mut i32) -> u8 {
    let mut tmpbyte = 0u8;
    pci_write_config_byte(pdev, PCI_CACHE_LINE_SIZE, if (*pdev).revision != 0 { 1 } else { 255 });
    pci_write_config_byte(pdev, 0x80, 0); pci_write_config_byte(pdev, 0x84, 0);
    pci_read_config_byte(pdev, 0x8A, &mut tmpbyte); *try_mmio = 0;
    // CONFIG_PPC / machine_is(cell) conditional is preserved as build-time intent.
    match tmpbyte & 0x30 { 0x00 => pci_write_config_byte(pdev, 0x8A, tmpbyte | 0x10), 0x30 => pci_write_config_byte(pdev, 0x8A, tmpbyte & !0x20), _ => {} }
    pci_read_config_byte(pdev, 0x8A, &mut tmpbyte);
    pci_write_config_byte(pdev, 0xA1, 0x72); pci_write_config_word(pdev, 0xA2, 0x328A);
    pci_write_config_dword(pdev, 0xA4, 0x62DD62DD); pci_write_config_dword(pdev, 0xA8, 0x43924392); pci_write_config_dword(pdev, 0xAC, 0x40094009);
    pci_write_config_byte(pdev, 0xB1, 0x72); pci_write_config_word(pdev, 0xB2, 0x328A);
    pci_write_config_dword(pdev, 0xB4, 0x62DD62DD); pci_write_config_dword(pdev, 0xB8, 0x43924392); pci_write_config_dword(pdev, 0xBC, 0x40094009);
    tmpbyte & 0x30
}

unsafe fn sil680_init_one(pdev: *mut pci_dev, _id: *const pci_device_id) -> i32 {
    static mut INFO: ata_port_info = ata_port_info { flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: ATA_MWDMA2, udma_mask: ATA_UDMA6, port_ops: &sil680_port_ops };
    static mut INFO_SLOW: ata_port_info = ata_port_info { flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: ATA_MWDMA2, udma_mask: ATA_UDMA5, port_ops: &sil680_port_ops };
    let mut ppi: [*const ata_port_info; 2] = [&INFO, core::ptr::null()];
    let mut host: *mut ata_host;
    let mut rc: i32; let mut try_mmio: i32;
    ata_print_version_once(&(*pdev).dev, DRV_VERSION);
    rc = pcim_enable_device(pdev); if rc != 0 { return rc; }
    match sil680_init_chip(pdev, &mut try_mmio) { 0 => ppi[0] = &INFO_SLOW, 0x30 => return -ENODEV, _ => {} }
    if try_mmio == 0 { return ata_pci_bmdma_init_one(pdev, ppi.as_ptr(), &sil680_sht, core::ptr::null(), 0); }
    rc = pcim_iomap_regions(pdev, 1 << SIL680_MMIO_BAR, DRV_NAME); if rc != 0 { return ata_pci_bmdma_init_one(pdev, ppi.as_ptr(), &sil680_sht, core::ptr::null(), 0); }
    host = ata_host_alloc_pinfo(&(*pdev).dev, ppi.as_ptr(), 2); if host.is_null() { return -ENOMEM; }
    (*host).iomap = pcim_iomap_table(pdev);
    rc = dma_set_mask_and_coherent(&(*pdev).dev, ATA_DMA_MASK); if rc != 0 { return rc; } pci_set_master(pdev);
    let mmio_base = (*host).iomap[SIL680_MMIO_BAR];
    (*(*host).ports.add(0)).ioaddr.bmdma_addr = mmio_base; (*(*host).ports.add(0)).ioaddr.cmd_addr = mmio_base + 0x80; (*(*host).ports.add(0)).ioaddr.ctl_addr = mmio_base + 0x8a; (*(*host).ports.add(0)).ioaddr.altstatus_addr = mmio_base + 0x8a; ata_sff_std_ports(&mut (*(*host).ports.add(0)).ioaddr);
    (*(*host).ports.add(1)).ioaddr.bmdma_addr = mmio_base + 8; (*(*host).ports.add(1)).ioaddr.cmd_addr = mmio_base + 0xc0; (*(*host).ports.add(1)).ioaddr.ctl_addr = mmio_base + 0xca; (*(*host).ports.add(1)).ioaddr.altstatus_addr = mmio_base + 0xca; ata_sff_std_ports(&mut (*(*host).ports.add(1)).ioaddr);
    ata_host_activate(host, (*pdev).irq, ata_bmdma_interrupt, IRQF_SHARED, &sil680_sht)
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn sil680_reinit_one(pdev: *mut pci_dev) -> i32 {
    let host = pci_get_drvdata(pdev); let mut try_mmio = 0; let rc = ata_pci_device_do_resume(pdev); if rc != 0 { return rc; }
    sil680_init_chip(pdev, &mut try_mmio); ata_host_resume(host); 0
}

static mut SIL680: [pci_device_id; 2] = [pci_device_id { vendor: PCI_VENDOR_ID_CMD, device: PCI_DEVICE_ID_SII_680 }, pci_device_id::default()];
static mut SIL680_PCI_DRIVER: pci_driver = pci_driver { name: DRV_NAME, id_table: SIL680.as_ptr(), probe: Some(sil680_init_one), remove: Some(ata_pci_remove_one), suspend: Some(ata_pci_device_suspend), resume: Some(sil680_reinit_one) };

// module_pci_driver(sil680_pci_driver);
// MODULE_AUTHOR("Alan Cox"); MODULE_DESCRIPTION("low-level driver for SI680 PATA");
// MODULE_LICENSE("GPL"); MODULE_DEVICE_TABLE(pci, sil680); MODULE_VERSION(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
