// SPDX-License-Identifier: GPL-2.0-only
/*
 * pata_sl82c105.c 	- SL82C105 PATA for new ATA layer
 *			  (C) 2005 Red Hat Inc
 *			  (C) 2011 Bartlomiej Zolnierkiewicz
 *
 * Based in part on linux/drivers/ide/pci/sl82c105.c
 * 		SL82C105/Winbond 553 IDE driver
 *
 * and in part on the documentation and errata sheet
 *
 * Note: The controller like many controllers has shared timings for
 * PIO and DMA. We thus flip to the DMA timings in dma_start and flip back
 * in the dma_stop function. Thus we actually don't need a set_dmamode
 * method as the PIO method is always called and will set the right PIO
 * timing parameters.
 */

// Linux kernel headers and build-time configuration are supplied externally.

pub const DRV_NAME: &str = "pata_sl82c105";
pub const DRV_VERSION: &str = "0.3.3";

pub const CTRL_IDE_IRQB: u32 = 1 << 30;
pub const CTRL_IDE_IRQA: u32 = 1 << 28;
pub const CTRL_LEGIRQ: u32 = 1 << 11;
pub const CTRL_P1F16: u32 = 1 << 5;
pub const CTRL_P1EN: u32 = 1 << 4;
pub const CTRL_P0F16: u32 = 1 << 1;
pub const CTRL_P0EN: u32 = 1 << 0;

unsafe fn sl82c105_pre_reset(link: *mut ata_link, deadline: c_ulong) -> c_int {
    let sl82c105_enable_bits: [pci_bits; 2] = [
        pci_bits { reg: 0x40, width: 1, mask: 0x01, val: 0x01 },
        pci_bits { reg: 0x40, width: 1, mask: 0x10, val: 0x10 },
    ];
    let ap = (*link).ap;
    let pdev = to_pci_dev((*(*ap).host).dev);

    if (*ap).port_no != 0
        && pci_test_config_bits(pdev, &sl82c105_enable_bits[(*ap).port_no as usize]) == 0
    {
        return -ENOENT;
    }
    ata_sff_prereset(link, deadline)
}

unsafe fn sl82c105_configure_piomode(ap: *mut ata_port, adev: *mut ata_device, pio: c_int) {
    let pdev = to_pci_dev((*(*ap).host).dev);
    static PIO_TIMING: [u16; 5] = [0x50D, 0x407, 0x304, 0x242, 0x240];
    let mut dummy: u16 = 0;
    let timing = 0x44 + (8 * (*ap).port_no) + (4 * (*adev).devno);

    pci_write_config_word(pdev, timing, PIO_TIMING[pio as usize]);
    // Can we lose this oddity of the old driver
    pci_read_config_word(pdev, timing, &mut dummy);
}

unsafe fn sl82c105_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    sl82c105_configure_piomode(ap, adev, (*adev).pio_mode - XFER_PIO_0);
}

unsafe fn sl82c105_configure_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    let pdev = to_pci_dev((*(*ap).host).dev);
    static DMA_TIMING: [u16; 3] = [0x707, 0x201, 0x200];
    let mut dummy: u16 = 0;
    let timing = 0x44 + (8 * (*ap).port_no) + (4 * (*adev).devno);
    let dma = (*adev).dma_mode - XFER_MW_DMA_0;

    pci_write_config_word(pdev, timing, DMA_TIMING[dma as usize]);
    // Can we lose this oddity of the old driver
    pci_read_config_word(pdev, timing, &mut dummy);
}

unsafe fn sl82c105_reset_engine(ap: *mut ata_port) {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut val: u16 = 0;

    pci_read_config_word(pdev, 0x7E, &mut val);
    pci_write_config_word(pdev, 0x7E, val | 4);
    pci_write_config_word(pdev, 0x7E, val & !4);
}

unsafe fn sl82c105_bmdma_start(qc: *mut ata_queued_cmd) {
    let ap = (*qc).ap;

    udelay(100);
    sl82c105_reset_engine(ap);
    udelay(100);
    // Set the clocks for DMA
    sl82c105_configure_dmamode(ap, (*qc).dev);
    // Activate DMA
    ata_bmdma_start(qc);
}

unsafe fn sl82c105_bmdma_stop(qc: *mut ata_queued_cmd) {
    let ap = (*qc).ap;

    ata_bmdma_stop(qc);
    sl82c105_reset_engine(ap);
    udelay(100);
    // This will redo the initial setup of the DMA device to matching PIO timings
    sl82c105_set_piomode(ap, (*qc).dev);
}

unsafe fn sl82c105_qc_defer(qc: *mut ata_queued_cmd) -> c_int {
    let host = (*(*qc).ap).host;
    let alt = (*host).ports[1 ^ (*(*qc).ap).port_no as usize];
    let rc = ata_std_qc_defer(qc);

    if rc != 0 {
        return rc;
    }
    if !alt.is_null() && (*alt).qc_active != 0 {
        return ATA_DEFER_PORT;
    }
    0
}

unsafe fn sl82c105_sff_irq_check(ap: *mut ata_port) -> bool {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mask: u32 = if (*ap).port_no != 0 { CTRL_IDE_IRQB } else { CTRL_IDE_IRQA };
    let mut val: u32 = 0;

    pci_read_config_dword(pdev, 0x40, &mut val);
    (val & mask) != 0
}

static mut SL82C105_SHT: scsi_host_template = ATA_BMDMA_SHT!(DRV_NAME);

static mut SL82C105_PORT_OPS: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma_port_ops,
    qc_defer: Some(sl82c105_qc_defer),
    bmdma_start: Some(sl82c105_bmdma_start),
    bmdma_stop: Some(sl82c105_bmdma_stop),
    cable_detect: Some(ata_cable_40wire),
    set_piomode: Some(sl82c105_set_piomode),
    reset: ata_reset_operations { prereset: Some(sl82c105_pre_reset) },
    sff_irq_check: Some(sl82c105_sff_irq_check),
};

unsafe fn sl82c105_bridge_revision(pdev: *mut pci_dev) -> c_int {
    let bridge = pci_get_slot((*pdev).bus, PCI_DEVFN(PCI_SLOT((*pdev).devfn), 0));
    if bridge.is_null() { return -1; }
    if (*bridge).vendor != PCI_VENDOR_ID_WINBOND
        || (*bridge).device != PCI_DEVICE_ID_WINBOND_83C553
        || ((*bridge).class >> 8) != PCI_CLASS_BRIDGE_ISA
    {
        pci_dev_put(bridge);
        return -1;
    }
    let revision = (*bridge).revision;
    pci_dev_put(bridge);
    revision as c_int
}

unsafe fn sl82c105_fixup(pdev: *mut pci_dev) {
    let mut val: u32 = 0;
    pci_read_config_dword(pdev, 0x40, &mut val);
    val |= CTRL_P0EN | CTRL_P0F16 | CTRL_P1F16;
    pci_write_config_dword(pdev, 0x40, val);
}

unsafe fn sl82c105_init_one(dev: *mut pci_dev, _id: *const pci_device_id) -> c_int {
    static INFO_DMA: ata_port_info = ata_port_info {
        flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: ATA_MWDMA2,
        port_ops: &SL82C105_PORT_OPS,
    };
    static INFO_EARLY: ata_port_info = ata_port_info {
        flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: 0,
        port_ops: &SL82C105_PORT_OPS,
    };
    let mut ppi: [*const ata_port_info; 2] = [&INFO_EARLY, core::ptr::null()];
    let rc = pcim_enable_device(dev);
    if rc != 0 { return rc; }
    let rev = sl82c105_bridge_revision(dev);
    if rev == -1 {
        dev_warn!((*dev).dev, "pata_sl82c105: Unable to find bridge, disabling DMA\n");
    } else if rev <= 5 {
        dev_warn!((*dev).dev, "pata_sl82c105: Early bridge revision, no DMA available\n");
    } else {
        ppi[0] = &INFO_DMA;
    }
    sl82c105_fixup(dev);
    ata_pci_bmdma_init_one(dev, ppi.as_ptr(), &SL82C105_SHT, core::ptr::null_mut(), 0)
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn sl82c105_reinit_one(pdev: *mut pci_dev) -> c_int {
    let host = pci_get_drvdata(pdev);
    let rc = ata_pci_device_do_resume(pdev);
    if rc != 0 { return rc; }
    sl82c105_fixup(pdev);
    ata_host_resume(host);
    0
}

static SL82C105: [pci_device_id; 2] = [
    PCI_VDEVICE!(WINBOND, PCI_DEVICE_ID_WINBOND_82C105),
    pci_device_id::default(),
];

static mut SL82C105_PCI_DRIVER: pci_driver = pci_driver {
    name: DRV_NAME,
    id_table: SL82C105.as_ptr(),
    probe: Some(sl82c105_init_one),
    remove: Some(ata_pci_remove_one),
    #[cfg(CONFIG_PM_SLEEP)]
    suspend: Some(ata_pci_device_suspend),
    #[cfg(CONFIG_PM_SLEEP)]
    resume: Some(sl82c105_reinit_one),
};

module_pci_driver!(SL82C105_PCI_DRIVER);
module_author!("Alan Cox");
module_description!("low-level driver for Sl82c105");
module_license!("GPL");
module_device_table!(pci, SL82C105);
module_version!(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
