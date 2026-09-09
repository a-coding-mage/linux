// SPDX-License-Identifier: GPL-2.0-only
/*
 * pata_triflex.c - Compaq PATA for new ATA layer
 *
 * Direct Rust translation of the original implementation. Kernel-provided
 * types, constants, functions, and macros are intentionally external.
 */

const DRV_NAME: &str = "pata_triflex";
const DRV_VERSION: &str = "0.2.8";

unsafe fn triflex_prereset(link: *mut ata_link, deadline: c_ulong) -> c_int {
    static TRIFLEX_ENABLE_BITS: [pci_bits; 2] = [
        pci_bits { reg: 0x80, width: 1, mask: 0x01, val: 0x01 },
        pci_bits { reg: 0x80, width: 1, mask: 0x02, val: 0x02 },
    ];

    let ap = (*link).ap;
    let pdev = to_pci_dev((*(*ap).host).dev);

    if !pci_test_config_bits(pdev, &TRIFLEX_ENABLE_BITS[(*ap).port_no as usize]) {
        return -ENOENT;
    }

    ata_sff_prereset(link, deadline)
}

unsafe fn triflex_load_timing(ap: *mut ata_port, adev: *mut ata_device, speed: c_int) {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut timing: u32 = 0;
    let mut triflex_timing: u32;
    let mut old_triflex_timing: u32 = 0;
    let channel_offset: c_int = if (*ap).port_no != 0 { 0x74 } else { 0x70 };
    let is_slave: u32 = if (*adev).devno != 0 { 1 } else { 0 };

    pci_read_config_dword(pdev, channel_offset, &mut old_triflex_timing);
    triflex_timing = old_triflex_timing;

    timing = match speed {
        XFER_MW_DMA_2 => 0x0103,
        XFER_MW_DMA_1 => 0x0203,
        XFER_MW_DMA_0 => 0x0808,
        XFER_SW_DMA_2 | XFER_SW_DMA_1 | XFER_SW_DMA_0 => 0x0F0F,
        XFER_PIO_4 => 0x0202,
        XFER_PIO_3 => 0x0204,
        XFER_PIO_2 => 0x0404,
        XFER_PIO_1 => 0x0508,
        XFER_PIO_0 => 0x0808,
        _ => BUG(),
    };
    triflex_timing &= !(0xFFFFu32 << (16 * is_slave));
    triflex_timing |= timing << (16 * is_slave);

    if triflex_timing != old_triflex_timing {
        pci_write_config_dword(pdev, channel_offset, triflex_timing);
    }
}

unsafe fn triflex_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    triflex_load_timing(ap, adev, (*adev).pio_mode);
}

unsafe fn triflex_bmdma_start(qc: *mut ata_queued_cmd) {
    triflex_load_timing((*qc).ap, (*qc).dev, (*(*qc).dev).dma_mode);
    ata_bmdma_start(qc);
}

unsafe fn triflex_bmdma_stop(qc: *mut ata_queued_cmd) {
    ata_bmdma_stop(qc);
    triflex_load_timing((*qc).ap, (*qc).dev, (*(*qc).dev).pio_mode);
}

static triflex_sht: scsi_host_template = scsi_host_template {
    /* ATA_BMDMA_SHT(DRV_NAME) */
};

static mut triflex_port_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma_port_ops,
    bmdma_start: Some(triflex_bmdma_start),
    bmdma_stop: Some(triflex_bmdma_stop),
    cable_detect: Some(ata_cable_40wire),
    set_piomode: Some(triflex_set_piomode),
    reset: ata_reset_operations { prereset: Some(triflex_prereset) },
};

unsafe fn triflex_init_one(dev: *mut pci_dev, _id: *const pci_device_id) -> c_int {
    static info: ata_port_info = ata_port_info {
        flags: ATA_FLAG_SLAVE_POSS,
        pio_mask: ATA_PIO4,
        mwdma_mask: ATA_MWDMA2,
        port_ops: &triflex_port_ops,
    };
    let ppi: [*const ata_port_info; 2] = [&info, core::ptr::null()];

    ata_print_version_once(&mut (*dev).dev, DRV_VERSION);
    ata_pci_bmdma_init_one(dev, ppi.as_ptr(), &triflex_sht, core::ptr::null_mut(), 0)
}

static triflex: [pci_device_id; 2] = [
    pci_device_id { vendor: COMPAQ, device: PCI_DEVICE_ID_COMPAQ_TRIFLEX_IDE },
    pci_device_id {},
];

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn triflex_ata_pci_device_suspend(pdev: *mut pci_dev, mesg: pm_message_t) -> c_int {
    let host = pci_get_drvdata(pdev);
    ata_host_suspend(host, mesg);
    pci_save_state(pdev);
    0
}

static mut triflex_pci_driver: pci_driver = pci_driver {
    name: DRV_NAME,
    id_table: triflex.as_ptr(),
    probe: Some(triflex_init_one),
    remove: Some(ata_pci_remove_one),
    #[cfg(CONFIG_PM_SLEEP)]
    suspend: Some(triflex_ata_pci_device_suspend),
    #[cfg(CONFIG_PM_SLEEP)]
    resume: Some(ata_pci_device_resume),
};

// module_pci_driver(triflex_pci_driver);
// MODULE_AUTHOR("Alan Cox");
// MODULE_DESCRIPTION("low-level driver for Compaq Triflex");
// MODULE_LICENSE("GPL");
// MODULE_DEVICE_TABLE(pci, triflex);
// MODULE_VERSION(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
