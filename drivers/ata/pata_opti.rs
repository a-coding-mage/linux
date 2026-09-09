// SPDX-License-Identifier: GPL-2.0-only
/*
 * pata_opti.c - ATI PATA for new ATA layer
 *               (C) 2005 Red Hat Inc
 *
 * Based on linux/drivers/ide/pci/opti621.c Version 0.7 Sept 10, 2002
 *
 * Copyright (C) 1996-1998 Linus Torvalds & authors (see below)
 *
 * Authors:
 * Jaromir Koutek <miri@punknet.cz>, Jan Harkes <jaharkes@cwi.nl>,
 * Mark Lord <mlord@pobox.com>
 * Some parts of code are from ali14xx.c and from rz1000.c.
 *
 * Also consulted the FreeBSD prototype driver by Kevin Day.
 */

// Dependencies supplied by the surrounding kernel translation.

const DRV_NAME: &str = "pata_opti";
const DRV_VERSION: &str = "0.2.9";

enum {
    READ_REG = 0,   // index of Read cycle timing register
    WRITE_REG = 1,  // index of Write cycle timing register
    CNTRL_REG = 3,  // index of Control register
    STRAP_REG = 5,  // index of Strap register
    MISC_REG = 6,   // index of Miscellaneous register
}

/**
 * opti_pre_reset - probe begin
 * @link: ATA link
 * @deadline: deadline jiffies for the operation
 *
 * Set up cable type and use generic probe init
 */
unsafe fn opti_pre_reset(link: *mut ata_link, deadline: c_ulong) -> c_int {
    let ap = (*link).ap;
    let pdev = to_pci_dev((*(*ap).host).dev);
    let opti_enable_bits: [pci_bits; 2] = [
        pci_bits { reg: 0x45, width: 1, mask: 0x80, val: 0x00 },
        pci_bits { reg: 0x40, width: 1, mask: 0x08, val: 0x00 },
    ];

    if !pci_test_config_bits(pdev, &opti_enable_bits[(*ap).port_no as usize]) {
        return -ENOENT;
    }

    ata_sff_prereset(link, deadline)
}

/**
 * opti_write_reg - control register setup
 * @ap: ATA port
 * @val: value
 * @reg: control register number
 *
 * The Opti uses magic 'trapdoor' register accesses to do configuration.
 */
unsafe fn opti_write_reg(ap: *mut ata_port, val: u8, reg: c_int) {
    let regio = (*ap).ioaddr.cmd_addr;

    // These 3 unlock the control register access
    ioread16(regio.add(1));
    ioread16(regio.add(1));
    iowrite8(3, regio.add(2));

    // Do the I/O
    iowrite8(val, regio.add(reg as usize));

    // Relock
    iowrite8(0x83, regio.add(2));
}

/**
 * opti_set_piomode - set initial PIO mode data
 * @ap: ATA interface
 * @adev: ATA device
 *
 * Called to do the PIO mode setup. Timing numbers are taken from the
 * FreeBSD driver then precomputed to keep the code clean.
 */
unsafe fn opti_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    let pair = ata_dev_pair(adev);
    let clock: usize;
    let pio = ((*adev).pio_mode - XFER_PIO_0) as usize;
    let regio = (*ap).ioaddr.cmd_addr;
    let mut addr: u8;

    // Address table precomputed with prefetch off and a DCLK of 2
    static ADDR_TIMING: [[u8; 5]; 2] = [
        [0x30, 0x20, 0x20, 0x10, 0x10],
        [0x20, 0x20, 0x10, 0x10, 0x10],
    ];
    static DATA_REC_TIMING: [[u8; 5]; 2] = [
        [0x6B, 0x56, 0x42, 0x32, 0x31],
        [0x58, 0x44, 0x32, 0x22, 0x21],
    ];

    iowrite8(0xff, regio.add(5));
    clock = (ioread16(regio.add(5)) & 1) as usize;

    // The address setup time is shared and must suit both devices if present.
    addr = ADDR_TIMING[clock][pio];
    if !pair.is_null() {
        // Hardware constraint
        let pair_addr = ADDR_TIMING[clock][((*pair).pio_mode - XFER_PIO_0) as usize];
        if pair_addr > addr {
            addr = pair_addr;
        }
    }

    // Commence primary programming sequence
    opti_write_reg(ap, (*adev).devno, MISC_REG);
    opti_write_reg(ap, DATA_REC_TIMING[clock][pio], READ_REG);
    opti_write_reg(ap, DATA_REC_TIMING[clock][pio], WRITE_REG);
    opti_write_reg(ap, addr, MISC_REG);

    // Programming sequence complete, override strapping
    opti_write_reg(ap, 0x85, CNTRL_REG);
}

static opti_sht: scsi_host_template = scsi_host_template {
    ATA_PIO_SHT!(DRV_NAME),
};

static mut opti_port_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_sff_port_ops,
    cable_detect: Some(ata_cable_40wire),
    set_piomode: Some(opti_set_piomode),
    reset: ata_port_reset_operations { prereset: Some(opti_pre_reset) },
};

unsafe fn opti_init_one(dev: *mut pci_dev, _id: *const pci_device_id) -> c_int {
    static info: ata_port_info = ata_port_info {
        flags: ATA_FLAG_SLAVE_POSS,
        pio_mask: ATA_PIO4,
        port_ops: &opti_port_ops,
    };
    let ppi: [*const ata_port_info; 2] = [&info, core::ptr::null()];

    ata_print_version_once(&mut (*dev).dev, DRV_VERSION);
    ata_pci_sff_init_one(dev, ppi.as_ptr(), &opti_sht, core::ptr::null_mut(), 0)
}

static opti: [pci_device_id; 3] = [
    PCI_VDEVICE!(OPTI, PCI_DEVICE_ID_OPTI_82C621, driver_data: 0),
    PCI_VDEVICE!(OPTI, PCI_DEVICE_ID_OPTI_82C825, driver_data: 1),
    pci_device_id {},
];

static mut opti_pci_driver: pci_driver = pci_driver {
    name: DRV_NAME,
    id_table: opti.as_ptr(),
    probe: Some(opti_init_one),
    remove: Some(ata_pci_remove_one),
    // CONFIG_PM_SLEEP conditionally supplies suspend and resume.
    suspend: Some(ata_pci_device_suspend),
    resume: Some(ata_pci_device_resume),
};

module_pci_driver!(opti_pci_driver);

MODULE_AUTHOR!("Alan Cox");
MODULE_DESCRIPTION!("low-level driver for Opti 621/621X");
MODULE_LICENSE!("GPL");
MODULE_DEVICE_TABLE!(pci, opti);
MODULE_VERSION!(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
