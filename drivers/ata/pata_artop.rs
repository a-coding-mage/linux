// SPDX-License-Identifier: GPL-2.0-only
/*
 *    pata_artop.c - ARTOP ATA controller driver
 *
 *    Direct Rust translation of the implementation source.
 */

// Kernel dependencies and build-time configuration are supplied externally.

const DRV_NAME: &str = "pata_artop";
const DRV_VERSION: &str = "0.4.8";

static mut CLOCK: i32 = 0;

unsafe fn artop62x0_pre_reset(link: *mut ata_link, deadline: c_ulong) -> c_int {
    static ART_OP_ENABLE_BITS: [pci_bits; 2] = [
        pci_bits { reg: 0x4a, width: 1, mask: 0x02, val: 0x02 },
        pci_bits { reg: 0x4a, width: 1, mask: 0x04, val: 0x04 },
    ];
    let ap = (*link).ap;
    let pdev = to_pci_dev((*(*ap).host).dev);
    if ((*pdev).device & 1) != 0
        && !pci_test_config_bits(pdev, &ART_OP_ENABLE_BITS[(*ap).port_no as usize])
    {
        return -ENOENT;
    }
    ata_sff_prereset(link, deadline)
}

unsafe fn artop6260_cable_detect(ap: *mut ata_port) -> c_int {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut tmp: u8 = 0;
    pci_read_config_byte(pdev, 0x49, &mut tmp);
    if (tmp & (1 << (*ap).port_no)) != 0 { ATA_CBL_PATA40 } else { ATA_CBL_PATA80 }
}

unsafe fn artop6210_load_piomode(ap: *mut ata_port, adev: *mut ata_device, pio: c_uint) {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let dn = (*adev).devno + 2 * (*ap).port_no;
    static TIMING: [[u16; 5]; 2] = [[0x0000, 0x000a, 0x0008, 0x0303, 0x0301], [0x0700, 0x070a, 0x0708, 0x0403, 0x0401]];
    pci_write_config_word(pdev, 0x40 + 2 * dn, TIMING[CLOCK as usize][pio as usize]);
}

unsafe fn artop6210_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let dn = (*adev).devno + 2 * (*ap).port_no;
    let mut ultra: u8 = 0;
    artop6210_load_piomode(ap, adev, (*adev).pio_mode - XFER_PIO_0);
    pci_read_config_byte(pdev, 0x54, &mut ultra);
    ultra &= !(3 << (2 * dn));
    pci_write_config_byte(pdev, 0x54, ultra);
}

unsafe fn artop6260_load_piomode(ap: *mut ata_port, adev: *mut ata_device, pio: c_uint) {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let dn = (*adev).devno + 2 * (*ap).port_no;
    static TIMING: [[u8; 5]; 2] = [[0x00, 0x0a, 0x08, 0x33, 0x31], [0x70, 0x7a, 0x78, 0x43, 0x41]];
    pci_write_config_byte(pdev, 0x40 + dn, TIMING[CLOCK as usize][pio as usize]);
}

unsafe fn artop6260_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut ultra: u8 = 0;
    artop6260_load_piomode(ap, adev, (*adev).pio_mode - XFER_PIO_0);
    pci_read_config_byte(pdev, 0x44 + (*ap).port_no, &mut ultra);
    ultra &= !(7 << (4 * (*adev).devno));
    pci_write_config_byte(pdev, 0x44 + (*ap).port_no, ultra);
}

unsafe fn artop6210_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    let pio = if (*adev).dma_mode == XFER_MW_DMA_0 { 1 } else { 4 };
    let pdev = to_pci_dev((*(*ap).host).dev);
    let dn = (*adev).devno + 2 * (*ap).port_no;
    let mut ultra: u8 = 0;
    artop6210_load_piomode(ap, adev, pio);
    pci_read_config_byte(pdev, 0x54, &mut ultra);
    ultra &= !(3 << (2 * dn));
    if (*adev).dma_mode >= XFER_UDMA_0 {
        let mut mode = (*adev).dma_mode - XFER_UDMA_0 + 1 - CLOCK as u32;
        if mode == 0 { mode = 1; }
        ultra |= (mode << (2 * dn)) as u8;
    }
    pci_write_config_byte(pdev, 0x54, ultra);
}

unsafe fn artop6260_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    let pio = if (*adev).dma_mode == XFER_MW_DMA_0 { 1 } else { 4 };
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut ultra: u8 = 0;
    artop6260_load_piomode(ap, adev, pio);
    pci_read_config_byte(pdev, 0x44 + (*ap).port_no, &mut ultra);
    ultra &= !(7 << (4 * (*adev).devno));
    if (*adev).dma_mode >= XFER_UDMA_0 {
        let mut mode = (*adev).dma_mode - XFER_UDMA_0 + 1 - CLOCK as u32;
        if mode == 0 { mode = 1; }
        ultra |= (mode << (4 * (*adev).devno)) as u8;
    }
    pci_write_config_byte(pdev, 0x44 + (*ap).port_no, ultra);
}

unsafe fn artop6210_qc_defer(qc: *mut ata_queued_cmd) -> c_int {
    let host = (*qc).ap.as_ref().unwrap().host;
    let alt = (*host).ports[1 ^ (*qc).ap.as_ref().unwrap().port_no as usize];
    let rc = ata_std_qc_defer(qc);
    if rc != 0 { return rc; }
    if !alt.is_null() && (*alt).qc_active != 0 { return ATA_DEFER_PORT; }
    0
}

// Operations, PCI tables, module registration, and PM callbacks are represented
// using the corresponding external kernel types and constructors.
static mut artop_sht: scsi_host_template = ATA_BMDMA_SHT!(DRV_NAME);
static mut artop6210_ops: ata_port_operations = ata_port_operations { inherits: &ata_bmdma_port_ops, cable_detect: Some(ata_cable_40wire), set_piomode: Some(artop6210_set_piomode), set_dmamode: Some(artop6210_set_dmamode), prereset: Some(artop62x0_pre_reset), qc_defer: Some(artop6210_qc_defer) };
static mut artop6260_ops: ata_port_operations = ata_port_operations { inherits: &ata_bmdma_port_ops, cable_detect: Some(artop6260_cable_detect), set_piomode: Some(artop6260_set_piomode), set_dmamode: Some(artop6260_set_dmamode), prereset: Some(artop62x0_pre_reset), qc_defer: None };

unsafe fn atp8xx_fixup(pdev: *mut pci_dev) {
    let mut reg: u8 = 0;
    match (*pdev).device {
        0x0005 => pci_write_config_byte(pdev, 0x54, 0),
        0x0008 | 0x0009 => {
            pci_read_config_byte(pdev, 0x49, &mut reg); pci_write_config_byte(pdev, 0x49, reg & !0x30);
            pci_read_config_byte(pdev, PCI_LATENCY_TIMER, &mut reg); if reg <= 0x80 { pci_write_config_byte(pdev, PCI_LATENCY_TIMER, 0x90); }
            pci_read_config_byte(pdev, 0x4a, &mut reg); pci_write_config_byte(pdev, 0x4a, (reg & !1) | 0x80);
        }, _ => {}
    }
}

unsafe fn artop_init_one(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static INFO_6210: ata_port_info = ata_port_info { flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: ATA_MWDMA2, udma_mask: ATA_UDMA2, port_ops: &artop6210_ops };
    static INFO_626X: ata_port_info = ata_port_info { flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: ATA_MWDMA2, udma_mask: ATA_UDMA4, port_ops: &artop6260_ops };
    static INFO_628X: ata_port_info = ata_port_info { flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: ATA_MWDMA2, udma_mask: ATA_UDMA5, port_ops: &artop6260_ops };
    static INFO_628X_FAST: ata_port_info = ata_port_info { flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: ATA_MWDMA2, udma_mask: ATA_UDMA6, port_ops: &artop6260_ops };
    let mut ppi: [*const ata_port_info; 2] = [core::ptr::null(), core::ptr::null()];
    let mut rc: c_int;
    ata_print_version_once(&mut (*pdev).dev, DRV_VERSION);
    rc = pcim_enable_device(pdev); if rc != 0 { return rc; }
    match (*id).driver_data {
        0 => ppi[0] = &INFO_6210,
        1 => ppi[0] = &INFO_626X,
        2 => { if (inb(pci_resource_start(pdev, 4)) & 0x10) != 0 { ppi[0] = &INFO_628X_FAST; } else { ppi[0] = &INFO_628X; } },
        _ => {}
    }
    BUG_ON!(ppi[0].is_null());
    atp8xx_fixup(pdev);
    ata_pci_bmdma_init_one(pdev, &ppi, &artop_sht, core::ptr::null_mut(), 0)
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn atp8xx_reinit_one(pdev: *mut pci_dev) -> c_int {
    let host = pci_get_drvdata(pdev);
    let rc = ata_pci_device_do_resume(pdev);
    if rc != 0 { return rc; }
    atp8xx_fixup(pdev);
    ata_host_resume(host);
    0
}

static artop_pci_tbl: [pci_device_id; 6] = [
    PCI_VDEVICE!(ARTOP, 0x0005, 0), PCI_VDEVICE!(ARTOP, 0x0006, 1),
    PCI_VDEVICE!(ARTOP, 0x0007, 1), PCI_VDEVICE!(ARTOP, 0x0008, 2),
    PCI_VDEVICE!(ARTOP, 0x0009, 2), pci_device_id::default(),
];

// module_pci_driver(artop_pci_driver);
// MODULE_AUTHOR, MODULE_DESCRIPTION, MODULE_LICENSE, MODULE_DEVICE_TABLE,
// and MODULE_VERSION are kernel build metadata.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
