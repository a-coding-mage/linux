// SPDX-License-Identifier: GPL-2.0-only
/*
 *    pata_oldpiix.c - Intel PATA/SATA controllers
 *
 *    Rust translation of the original implementation.
 *
 * Includes and kernel-provided symbols are supplied by the surrounding
 * translation environment.
 */

const DRV_NAME: &str = "pata_oldpiix";
const DRV_VERSION: &str = "0.5.5";

/* oldpiix_pre_reset - probe begin */
unsafe fn oldpiix_pre_reset(link: *mut ata_link, deadline: c_ulong) -> c_int {
    let ap = (*link).ap;
    let pdev = to_pci_dev((*(*ap).host).dev);
    let oldpiix_enable_bits: [pci_bits; 2] = [
        pci_bits { reg: 0x41u32, width: 1u32, mask: 0x80u64, val: 0x80u64 },
        pci_bits { reg: 0x43u32, width: 1u32, mask: 0x80u64, val: 0x80u64 },
    ];

    if !pci_test_config_bits(pdev, &oldpiix_enable_bits[(*ap).port_no as usize]) {
        return -ENOENT;
    }

    ata_sff_prereset(link, deadline)
}

/* oldpiix_set_piomode - Initialize host controller PATA PIO timings */
unsafe fn oldpiix_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    let pio = (*adev).pio_mode.wrapping_sub(XFER_PIO_0);
    let dev = to_pci_dev((*(*ap).host).dev);
    let idetm_port: u32 = if (*ap).port_no != 0 { 0x42 } else { 0x40 };
    let mut idetm_data: u16 = 0;
    let mut control: c_int = 0;
    let timings: [[u8; 2]; 5] = [[0, 0], [0, 0], [1, 0], [2, 1], [2, 3]];

    if pio > 1 { control |= 1; }
    if ata_pio_need_iordy(adev) { control |= 2; }
    if (*adev).class == ATA_DEV_ATA { control |= 4; }

    pci_read_config_word(dev, idetm_port, &mut idetm_data);
    if (*adev).devno == 0 {
        idetm_data &= 0xCCE0;
        idetm_data |= control as u16;
    } else {
        idetm_data &= 0xCC0E;
        idetm_data |= (control << 4) as u16;
    }
    idetm_data |= ((timings[pio as usize][0] as u16) << 12)
        | ((timings[pio as usize][1] as u16) << 8);
    pci_write_config_word(dev, idetm_port, idetm_data);
    (*ap).private_data = adev as *mut c_void;
}

/* oldpiix_set_dmamode - Initialize host controller PATA DMA timings */
unsafe fn oldpiix_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    let dev = to_pci_dev((*(*ap).host).dev);
    let idetm_port: u8 = if (*ap).port_no != 0 { 0x42 } else { 0x40 };
    let mut idetm_data: u16 = 0;
    let timings: [[u8; 2]; 5] = [[0, 0], [0, 0], [1, 0], [2, 1], [2, 3]];
    let mwdma = (*adev).dma_mode.wrapping_sub(XFER_MW_DMA_0);
    let needed_pio: [u32; 3] = [XFER_PIO_0, XFER_PIO_3, XFER_PIO_4];
    let pio = needed_pio[mwdma as usize].wrapping_sub(XFER_PIO_0);
    let mut control: u32 = 3;

    if (*adev).class == ATA_DEV_ATA { control |= 4; }
    if (*adev).pio_mode < needed_pio[mwdma as usize] { control |= 8; }

    pci_read_config_word(dev, idetm_port as u32, &mut idetm_data);
    if (*adev).devno == 0 {
        idetm_data &= 0xCCE0;
        idetm_data |= control as u16;
    } else {
        idetm_data &= 0xCC0E;
        idetm_data |= (control << 4) as u16;
    }
    idetm_data |= ((timings[pio as usize][0] as u16) << 12)
        | ((timings[pio as usize][1] as u16) << 8);
    pci_write_config_word(dev, idetm_port as u32, idetm_data);
    (*ap).private_data = adev as *mut c_void;
}

/* oldpiix_qc_issue - command issue */
unsafe fn oldpiix_qc_issue(qc: *mut ata_queued_cmd) -> c_uint {
    let ap = (*qc).ap;
    let adev = (*qc).dev;
    if adev as *mut c_void != (*ap).private_data {
        oldpiix_set_piomode(ap, adev);
        if ata_dma_enabled(adev) { oldpiix_set_dmamode(ap, adev); }
    }
    ata_bmdma_qc_issue(qc)
}

static oldpiix_sht: scsi_host_template = ATA_BMDMA_SHT!(DRV_NAME);

static mut oldpiix_pata_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma_port_ops,
    qc_issue: Some(oldpiix_qc_issue),
    cable_detect: Some(ata_cable_40wire),
    set_piomode: Some(oldpiix_set_piomode),
    set_dmamode: Some(oldpiix_set_dmamode),
    reset: ata_port_reset_operations { prereset: Some(oldpiix_pre_reset) },
};

/* oldpiix_init_one - Register PIIX ATA PCI device with kernel services */
unsafe fn oldpiix_init_one(
    pdev: *mut pci_dev,
    ent: *const pci_device_id,
) -> c_int {
    let info = ata_port_info {
        flags: ATA_FLAG_SLAVE_POSS,
        pio_mask: ATA_PIO4,
        mwdma_mask: ATA_MWDMA12_ONLY,
        port_ops: &oldpiix_pata_ops,
    };
    let ppi: [*const ata_port_info; 2] = [&info, core::ptr::null()];

    ata_print_version_once(&mut (*pdev).dev, DRV_VERSION);
    ata_pci_bmdma_init_one(pdev, ppi.as_ptr(), &oldpiix_sht, core::ptr::null_mut(), 0)
}

static oldpiix_pci_tbl: [pci_device_id; 2] = [
    PCI_VDEVICE!(INTEL, 0x1230),
    pci_device_id {},
];

static mut oldpiix_pci_driver: pci_driver = pci_driver {
    name: DRV_NAME,
    id_table: oldpiix_pci_tbl.as_ptr(),
    probe: Some(oldpiix_init_one),
    remove: Some(ata_pci_remove_one),
    /* CONFIG_PM_SLEEP conditional fields preserved by the build configuration. */
    suspend: Some(ata_pci_device_suspend),
    resume: Some(ata_pci_device_resume),
};

/* module_pci_driver(oldpiix_pci_driver); */
/* MODULE_AUTHOR("Alan Cox"); */
/* MODULE_DESCRIPTION("SCSI low-level driver for early PIIX series controllers"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_DEVICE_TABLE(pci, oldpiix_pci_tbl); */
/* MODULE_VERSION(DRV_VERSION); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
