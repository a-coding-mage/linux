// SPDX-License-Identifier: GPL-2.0-only
/*
 *    pata_radisys.c - Intel PATA/SATA controllers
 *
 *	(C) 2006 Red Hat <alan@lxorguk.ukuu.org.uk>
 *
 *    Some parts based on ata_piix.c by Jeff Garzik and others.
 *
 *    A PIIX relative, this device has a single ATA channel and no
 *    slave timings, SITRE or PPE. In that sense it is a close relative
 *    of the original PIIX. It does however support UDMA 33/66 per channel
 *    although no other modes/timings. Also lacking is 32bit I/O on the ATA
 *    port.
 */

// C header dependencies are supplied by the surrounding kernel bindings.

const DRV_NAME: &str = "pata_radisys";
const DRV_VERSION: &str = "0.4.4";

/// radisys_set_piomode - Initialize host controller PATA PIO timings
/// @ap: ATA port
/// @adev: Device whose timings we are configuring
///
/// Set PIO mode for device, in host controller PCI config space.
///
/// LOCKING: None (inherited from caller).
unsafe fn radisys_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    let pio: usize = ((*adev).pio_mode - XFER_PIO_0) as usize;
    let dev: *mut pci_dev = to_pci_dev((*(*ap).host).dev);
    let mut idetm_data: u16 = 0;
    let mut control: i32 = 0;

    // See Intel Document 298600-004 for the timing programming rules
    // for PIIX/ICH. Note that the early PIIX does not have the slave
    // timing port at 0x44. The Radisys is a relative of the PIIX
    // but not the same so be careful.
    static TIMINGS: [[u8; 2]; 5] = [
        [0, 0], // Check me
        [0, 0],
        [1, 1],
        [2, 2],
        [3, 3],
    ];

    if pio > 0 {
        control |= 1; // TIME1 enable
    }
    if ata_pio_need_iordy(adev) {
        control |= 2; // IE IORDY
    }

    pci_read_config_word(dev, 0x40, &mut idetm_data);

    // Enable IE and TIME as appropriate. Clear the other drive timing bits
    idetm_data &= 0xCCCC;
    idetm_data |= (control as u16) << (4 * (*adev).devno);
    idetm_data |= ((TIMINGS[pio][0] as u16) << 12) |
        ((TIMINGS[pio][1] as u16) << 8);
    pci_write_config_word(dev, 0x40, idetm_data);

    // Track which port is configured
    (*ap).private_data = adev as *mut core::ffi::c_void;
}

/// radisys_set_dmamode - Initialize host controller PATA DMA timings
/// @ap: Port whose timings we are configuring
/// @adev: Device to program
///
/// Set MWDMA mode for device, in host controller PCI config space.
///
/// LOCKING: None (inherited from caller).
unsafe fn radisys_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    let dev: *mut pci_dev = to_pci_dev((*(*ap).host).dev);
    let mut idetm_data: u16 = 0;
    let mut udma_enable: u8 = 0;
    static TIMINGS: [[u8; 2]; 5] = [[0, 0], [0, 0], [1, 1], [2, 2], [3, 3]];

    // MWDMA is driven by the PIO timings. We must also enable IORDY unconditionally.
    pci_read_config_word(dev, 0x40, &mut idetm_data);
    pci_read_config_byte(dev, 0x48, &mut udma_enable);

    if (*adev).dma_mode < XFER_UDMA_0 {
        let mwdma = ((*adev).dma_mode - XFER_MW_DMA_0) as usize;
        let needed_pio: [u32; 3] = [XFER_PIO_0, XFER_PIO_3, XFER_PIO_4];
        let pio = (needed_pio[mwdma] - XFER_PIO_0) as usize;
        let mut control: u16 = 3; // IORDY|TIME0

        // If the drive MWDMA is faster than it can do PIO then we must force PIO0 for PIO cycles.
        if (*adev).pio_mode < needed_pio[mwdma] {
            control = 1;
        }

        // Mask out the relevant control and timing bits we will load. Also clear the other drive TIME register as a precaution
        idetm_data &= 0xCCCC;
        idetm_data |= control << (4 * (*adev).devno);
        idetm_data |= ((TIMINGS[pio][0] as u16) << 12) | ((TIMINGS[pio][1] as u16) << 8);
        udma_enable &= !(1u8 << (*adev).devno);
    } else {
        let mut udma_mode: u8 = 0;
        // UDMA66 on: UDMA 33 and 66 are switchable via register 0x4A
        pci_read_config_byte(dev, 0x4A, &mut udma_mode);
        if (*adev).xfer_mode == XFER_UDMA_2 {
            udma_mode &= !(2u8 << ((*adev).devno * 4));
        } else {
            udma_mode |= 2u8 << ((*adev).devno * 4);
        }
        pci_write_config_byte(dev, 0x4A, udma_mode);
        udma_enable |= 1u8 << (*adev).devno;
    }
    pci_write_config_word(dev, 0x40, idetm_data);
    pci_write_config_byte(dev, 0x48, udma_enable);
    // Track which port is configured
    (*ap).private_data = adev as *mut core::ffi::c_void;
}

/// radisys_qc_issue - command issue
unsafe fn radisys_qc_issue(qc: *mut ata_queued_cmd) -> u32 {
    let ap = (*qc).ap;
    let adev = (*qc).dev;
    if adev as *mut core::ffi::c_void != (*ap).private_data {
        // UDMA timing is not shared
        if (*adev).dma_mode < XFER_UDMA_0 || !ata_dma_enabled(adev) {
            if ata_dma_enabled(adev) {
                radisys_set_dmamode(ap, adev);
            } else if (*adev).pio_mode != 0 {
                radisys_set_piomode(ap, adev);
            }
        }
    }
    ata_bmdma_qc_issue(qc)
}

static radisys_sht: scsi_host_template = scsi_host_template {
    /* ATA_BMDMA_SHT(DRV_NAME) */
};

static mut radisys_pata_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma_port_ops,
    qc_issue: Some(radisys_qc_issue),
    cable_detect: Some(ata_cable_unknown),
    set_piomode: Some(radisys_set_piomode),
    set_dmamode: Some(radisys_set_dmamode),
};

unsafe fn radisys_init_one(pdev: *mut pci_dev, _ent: *const pci_device_id) -> i32 {
    static INFO: ata_port_info = ata_port_info {
        flags: ATA_FLAG_SLAVE_POSS,
        pio_mask: ATA_PIO4,
        mwdma_mask: ATA_MWDMA12_ONLY,
        udma_mask: ATA_UDMA24_ONLY,
        port_ops: &radisys_pata_ops,
    };
    let ppi: [*const ata_port_info; 2] = [&INFO, core::ptr::null()];
    ata_print_version_once(&mut (*pdev).dev, DRV_VERSION);
    ata_pci_bmdma_init_one(pdev, ppi.as_ptr(), &radisys_sht, core::ptr::null_mut(), 0)
}

static radisys_pci_tbl: [pci_device_id; 2] = [
    PCI_VDEVICE!(RADISYS, 0x8201),
    pci_device_id::default(),
];

static mut radisys_pci_driver: pci_driver = pci_driver {
    name: DRV_NAME,
    id_table: radisys_pci_tbl.as_ptr(),
    probe: Some(radisys_init_one),
    remove: Some(ata_pci_remove_one),
    // CONFIG_PM_SLEEP: suspend = ata_pci_device_suspend, resume = ata_pci_device_resume
};

// module_pci_driver(radisys_pci_driver);
// MODULE_AUTHOR("Alan Cox");
// MODULE_DESCRIPTION("SCSI low-level driver for Radisys R82600 controllers");
// MODULE_LICENSE("GPL");
// MODULE_DEVICE_TABLE(pci, radisys_pci_tbl);
// MODULE_VERSION(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
