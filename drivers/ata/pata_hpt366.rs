// SPDX-License-Identifier: GPL-2.0-only
/*
 * Libata driver for the highpoint 366 and 368 UDMA66 ATA controllers.
 *
 * This driver is heavily based upon:
 *
 * linux/drivers/ide/pci/hpt366.c\t\tVersion 0.36\tApril 25, 2003
 *
 * Copyright (C) 1999-2003\t\tAndre Hedrick <andre@linux-ide.org>
 * Portions Copyright (C) 2001\t        Sun Microsystems, Inc.
 * Portions Copyright (C) 2003\t\tRed Hat Inc
 *
 * TODO
 *\tLook into engine reset on timeout errors. Should not be required.
 */

const DRV_NAME: &str = "pata_hpt366";
const DRV_VERSION: &str = "0.6.13";

#[repr(C)]
struct hpt_clock {
    xfer_mode: u8,
    timing: u32,
}

static HPT366_40: [hpt_clock; 14] = [
    hpt_clock { xfer_mode: XFER_UDMA_4, timing: 0x900fd943 },
    hpt_clock { xfer_mode: XFER_UDMA_3, timing: 0x900ad943 },
    hpt_clock { xfer_mode: XFER_UDMA_2, timing: 0x900bd943 },
    hpt_clock { xfer_mode: XFER_UDMA_1, timing: 0x9008d943 },
    hpt_clock { xfer_mode: XFER_UDMA_0, timing: 0x9008d943 },
    hpt_clock { xfer_mode: XFER_MW_DMA_2, timing: 0xa008d943 },
    hpt_clock { xfer_mode: XFER_MW_DMA_1, timing: 0xa010d955 },
    hpt_clock { xfer_mode: XFER_MW_DMA_0, timing: 0xa010d9fc },
    hpt_clock { xfer_mode: XFER_PIO_4, timing: 0xc008d963 },
    hpt_clock { xfer_mode: XFER_PIO_3, timing: 0xc010d974 },
    hpt_clock { xfer_mode: XFER_PIO_2, timing: 0xc010d997 },
    hpt_clock { xfer_mode: XFER_PIO_1, timing: 0xc010d9c7 },
    hpt_clock { xfer_mode: XFER_PIO_0, timing: 0xc018d9d9 },
    hpt_clock { xfer_mode: 0, timing: 0x0120d9d9 },
];

static HPT366_33: [hpt_clock; 14] = [
    hpt_clock { xfer_mode: XFER_UDMA_4, timing: 0x90c9a731 },
    hpt_clock { xfer_mode: XFER_UDMA_3, timing: 0x90cfa731 },
    hpt_clock { xfer_mode: XFER_UDMA_2, timing: 0x90caa731 },
    hpt_clock { xfer_mode: XFER_UDMA_1, timing: 0x90cba731 },
    hpt_clock { xfer_mode: XFER_UDMA_0, timing: 0x90c8a731 },
    hpt_clock { xfer_mode: XFER_MW_DMA_2, timing: 0xa0c8a731 },
    hpt_clock { xfer_mode: XFER_MW_DMA_1, timing: 0xa0c8a732 }, // 0xa0c8a733
    hpt_clock { xfer_mode: XFER_MW_DMA_0, timing: 0xa0c8a797 },
    hpt_clock { xfer_mode: XFER_PIO_4, timing: 0xc0c8a731 },
    hpt_clock { xfer_mode: XFER_PIO_3, timing: 0xc0c8a742 },
    hpt_clock { xfer_mode: XFER_PIO_2, timing: 0xc0d0a753 },
    hpt_clock { xfer_mode: XFER_PIO_1, timing: 0xc0d0a7a3 }, // 0xc0d0a793
    hpt_clock { xfer_mode: XFER_PIO_0, timing: 0xc0d0a7aa }, // 0xc0d0a7a7
    hpt_clock { xfer_mode: 0, timing: 0x0120a7a7 },
];

static HPT366_25: [hpt_clock; 14] = [
    hpt_clock { xfer_mode: XFER_UDMA_4, timing: 0x90c98521 },
    hpt_clock { xfer_mode: XFER_UDMA_3, timing: 0x90cf8521 },
    hpt_clock { xfer_mode: XFER_UDMA_2, timing: 0x90cf8521 },
    hpt_clock { xfer_mode: XFER_UDMA_1, timing: 0x90cb8521 },
    hpt_clock { xfer_mode: XFER_UDMA_0, timing: 0x90cb8521 },
    hpt_clock { xfer_mode: XFER_MW_DMA_2, timing: 0xa0ca8521 },
    hpt_clock { xfer_mode: XFER_MW_DMA_1, timing: 0xa0ca8532 },
    hpt_clock { xfer_mode: XFER_MW_DMA_0, timing: 0xa0ca8575 },
    hpt_clock { xfer_mode: XFER_PIO_4, timing: 0xc0ca8521 },
    hpt_clock { xfer_mode: XFER_PIO_3, timing: 0xc0ca8532 },
    hpt_clock { xfer_mode: XFER_PIO_2, timing: 0xc0ca8542 },
    hpt_clock { xfer_mode: XFER_PIO_1, timing: 0xc0d08572 },
    hpt_clock { xfer_mode: XFER_PIO_0, timing: 0xc0d08585 },
    hpt_clock { xfer_mode: 0, timing: 0x01208585 },
];

static BAD_ATA33: &[&str] = &[
    "Maxtor 92720U8", "Maxtor 92040U6", "Maxtor 91360U4", "Maxtor 91020U3",
    "Maxtor 90845U3", "Maxtor 90650U2", "Maxtor 91360D8", "Maxtor 91190D7",
    "Maxtor 91020D6", "Maxtor 90845D5", "Maxtor 90680D4", "Maxtor 90510D3",
    "Maxtor 90340D2", "Maxtor 91152D8", "Maxtor 91008D7", "Maxtor 90845D6",
    "Maxtor 90840D6", "Maxtor 90720D5", "Maxtor 90648D5", "Maxtor 90576D4",
    "Maxtor 90510D4", "Maxtor 90432D3", "Maxtor 90288D2", "Maxtor 90256D2",
    "Maxtor 91000D8", "Maxtor 90910D8", "Maxtor 90875D7", "Maxtor 90840D7",
    "Maxtor 90750D6", "Maxtor 90625D5", "Maxtor 90500D4", "Maxtor 91728D8",
    "Maxtor 91512D7", "Maxtor 91303D6", "Maxtor 91080D5", "Maxtor 90845D4",
    "Maxtor 90680D4", "Maxtor 90648D3", "Maxtor 90432D2", NULL,
];

static BAD_ATA66_4: &[&str] = &[
    "IBM-DTLA-307075", "IBM-DTLA-307060", "IBM-DTLA-307045", "IBM-DTLA-307030",
    "IBM-DTLA-307020", "IBM-DTLA-307015", "IBM-DTLA-305040", "IBM-DTLA-305030",
    "IBM-DTLA-305020", "IC35L010AVER07-0", "IC35L020AVER07-0", "IC35L030AVER07-0",
    "IC35L040AVER07-0", "IC35L060AVER07-0", "WDC AC310200R", NULL,
];
static BAD_ATA66_3: &[&str] = &["WDC AC310200R", NULL];

static unsafe fn hpt36x_find_mode(ap: *mut ata_port, speed: i32) -> u32 {
    let mut clocks = (*(*ap).host).private_data as *mut hpt_clock;
    while (*clocks).xfer_mode != 0 {
        if (*clocks).xfer_mode as i32 == speed { return (*clocks).timing; }
        clocks = clocks.add(1);
    }
    BUG();
    0xffff_ffff
}

static unsafe fn hpt_dma_broken(dev: *const ata_device, modestr: *const i8, list: *const *const i8) -> i32 {
    let mut model_num = [0u8; ATA_ID_PROD_LEN + 1];
    ata_id_c_string((*dev).id, model_num.as_mut_ptr(), ATA_ID_PROD, model_num.len());
    let i = match_string(list, -1, model_num.as_ptr() as *const i8);
    if i >= 0 { ata_dev_warn(dev, modestr, list.add(i as usize)); return 1; }
    0
}

static unsafe fn hpt366_filter(adev: *mut ata_device, mut mask: u32) -> u32 {
    if (*adev).class == ATA_DEV_ATA {
        if hpt_dma_broken(adev, c"UDMA".as_ptr(), BAD_ATA33.as_ptr() as *const *const i8) != 0 { mask &= !ATA_MASK_UDMA; }
        if hpt_dma_broken(adev, c"UDMA3".as_ptr(), BAD_ATA66_3.as_ptr() as *const *const i8) != 0 { mask &= !(0xF8 << ATA_SHIFT_UDMA); }
        if hpt_dma_broken(adev, c"UDMA4".as_ptr(), BAD_ATA66_4.as_ptr() as *const *const i8) != 0 { mask &= !(0xF0 << ATA_SHIFT_UDMA); }
    } else if (*adev).class == ATA_DEV_ATAPI { mask &= !(ATA_MASK_MWDMA | ATA_MASK_UDMA); }
    mask
}

static unsafe fn hpt36x_cable_detect(ap: *mut ata_port) -> i32 {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut ata66 = 0u8;
    pci_read_config_byte(pdev, 0x5A, &mut ata66);
    if ata66 & 2 != 0 { ATA_CBL_PATA40 } else { ATA_CBL_PATA80 }
}

static unsafe fn hpt366_set_mode(ap: *mut ata_port, adev: *mut ata_device, mode: u8) {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let addr = 0x40 + 4 * (*adev).devno;
    let mask = if mode < XFER_MW_DMA_0 { 0xc1f8ffff } else if mode < XFER_UDMA_0 { 0x303800ff } else { 0x30070000 };
    let t = hpt36x_find_mode(ap, mode as i32);
    let mut reg = 0u32;
    pci_read_config_dword(pdev, addr, &mut reg);
    reg = ((reg & !mask) | (t & mask)) & !0xc0000000;
    pci_write_config_dword(pdev, addr, reg);
}

static unsafe fn hpt366_set_piomode(ap: *mut ata_port, adev: *mut ata_device) { hpt366_set_mode(ap, adev, (*adev).pio_mode); }
static unsafe fn hpt366_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) { hpt366_set_mode(ap, adev, (*adev).dma_mode); }

static unsafe fn hpt366_prereset(link: *mut ata_link, deadline: c_ulong) -> i32 {
    let ap = (*link).ap;
    let pdev = to_pci_dev((*(*ap).host).dev);
    let bits = pci_bits { reg: 0x50, width: 1, mask: 0x30, val: 0x30 };
    let mut mcr2 = 0u8;
    if !pci_test_config_bits(pdev, &bits) { return -ENOENT; }
    pci_read_config_byte(pdev, 0x51, &mut mcr2);
    if mcr2 & 0x80 != 0 { pci_write_config_byte(pdev, 0x51, mcr2 & !0x80); }
    ata_sff_prereset(link, deadline)
}

static unsafe fn hpt36x_init_chipset(dev: *mut pci_dev) {
    let mut mcr1 = 0u8;
    pci_write_config_byte(dev, PCI_CACHE_LINE_SIZE, L1_CACHE_BYTES / 4);
    pci_write_config_byte(dev, PCI_LATENCY_TIMER, 0x78);
    pci_write_config_byte(dev, PCI_MIN_GNT, 0x08);
    pci_write_config_byte(dev, PCI_MAX_LAT, 0x08);
    pci_read_config_byte(dev, 0x50, &mut mcr1);
    if mcr1 & 0x30 != 0 { pci_write_config_byte(dev, 0x50, mcr1 | 0x30); }
}

static unsafe fn hpt36x_init_one(dev: *mut pci_dev, _id: *const pci_device_id) -> i32 {
    let info_hpt366 = ata_port_info {
        flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: ATA_MWDMA2,
        udma_mask: ATA_UDMA4, port_ops: &hpt366_port_ops,
    };
    let ppi = [&info_hpt366 as *const ata_port_info, core::ptr::null()];
    let rc = pcim_enable_device(dev);
    if rc != 0 { return rc; }
    if (*dev).revision > 2 { return -ENODEV; }
    hpt36x_init_chipset(dev);
    let mut reg1 = 0u32;
    pci_read_config_dword(dev, 0x40, &mut reg1);
    let hpriv = match (reg1 & 0xf00) >> 8 { 9 => HPT366_40.as_ptr(), 5 => HPT366_25.as_ptr(), _ => HPT366_33.as_ptr() };
    ata_pci_bmdma_init_one(dev, ppi.as_ptr(), &hpt36x_sht, hpriv as *mut core::ffi::c_void, 0)
}

static hpt36x_sht: scsi_host_template = ATA_BMDMA_SHT!(DRV_NAME);
static hpt366_port_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma_port_ops,
    reset_prereset: Some(hpt366_prereset), cable_detect: Some(hpt36x_cable_detect),
    mode_filter: Some(hpt366_filter), set_piomode: Some(hpt366_set_piomode),
    set_dmamode: Some(hpt366_set_dmamode),
};

static hpt36x: [pci_device_id; 2] = [
    PCI_VDEVICE!(TTI, PCI_DEVICE_ID_TTI_HPT366),
    PCI_DEVICE_ID_EMPTY!(),
];

static hpt36x_pci_driver: pci_driver = pci_driver {
    name: DRV_NAME, id_table: hpt36x.as_ptr(), probe: Some(hpt36x_init_one),
    remove: Some(ata_pci_remove_one),
    #[cfg(CONFIG_PM_SLEEP)] suspend: Some(ata_pci_device_suspend),
    #[cfg(CONFIG_PM_SLEEP)] resume: Some(hpt36x_reinit_one),
};

module_pci_driver!(hpt36x_pci_driver);
module_author!("Alan Cox");
module_description!("low-level driver for the Highpoint HPT366/368");
module_license!("GPL");
module_device_table!(pci, hpt36x);
module_version!(DRV_VERSION);

#[cfg(CONFIG_PM_SLEEP)]
static unsafe fn hpt36x_reinit_one(dev: *mut pci_dev) -> i32 {
    let host = pci_get_drvdata(dev);
    let rc = ata_pci_device_do_resume(dev);
    if rc != 0 { return rc; }
    hpt36x_init_chipset(dev);
    ata_host_resume(host);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
