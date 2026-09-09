// SPDX-License-Identifier: GPL-2.0-only
/*
 * New ATA layer SC1200 driver        Alan Cox <alan@lxorguk.ukuu.org.uk>
 *
 * TODO: Mode selection filtering
 * TODO: Needs custom DMA cleanup code
 *
 * Based very heavily on
 *
 * linux/drivers/ide/pci/sc1200.c     Version 0.91  28-Jan-2003
 *
 * Copyright (C) 2000-2002           Mark Lord <mlord@pobox.com>
 * May be copied or modified under the terms of the GNU General Public License
 *
 * Development of this chipset driver was funded
 * by the nice folks at National Semiconductor.
 */

// Kernel and libata declarations are supplied by the surrounding tree.

const DRV_NAME: &str = "pata_sc1200";
const DRV_VERSION: &str = "0.2.6";

const SC1200_REV_A: u8 = 0x00;
const SC1200_REV_B1: u8 = 0x01;
const SC1200_REV_B3: u8 = 0x02;
const SC1200_REV_C1: u8 = 0x03;
const SC1200_REV_D1: u8 = 0x04;

unsafe fn sc1200_clock() -> i32 {
    /* Magic registers that give us the chipset data */
    let chip_id: u8 = inb(0x903c);
    let silicon_rev: u8 = inb(0x903d);
    let mut pci_clock: u16;

    if chip_id == 0x04 && silicon_rev < SC1200_REV_B1 {
        return 0; /* 33 MHz mode */
    }

    /* Clock generator configuration 0x901E its 8/9 are the PCI clocking
       0/3 is 33Mhz 1 is 48 2 is 66 */
    pci_clock = inw(0x901e);
    pci_clock >>= 8;
    pci_clock &= 0x03;
    if pci_clock == 3 {
        pci_clock = 0;
    }
    pci_clock as i32
}

unsafe fn sc1200_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    static PIO_TIMINGS: [[u32; 5]; 4] = [
        [0x00009172, 0x00012171, 0x00020080, 0x00032010, 0x00040010],
        [0xd1329172, 0x71212171, 0x30200080, 0x20102010, 0x00100010],
        [0xfaa3f4f3, 0xc23232b2, 0x513101c1, 0x31213121, 0x10211021],
        [0xfff4fff4, 0xf35353d3, 0x814102f1, 0x42314231, 0x11311131],
    ];

    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut format: u32 = 0;
    let reg: u32 = 0x40 + 0x10 * (*ap).port_no;
    let mode: i32 = (*adev).pio_mode - XFER_PIO_0;

    pci_read_config_dword(pdev, reg + 4, &mut format);
    format >>= 31;
    format += sc1200_clock() as u32;
    pci_write_config_dword(pdev, reg + 8 * (*adev).devno, PIO_TIMINGS[format as usize][mode as usize]);
}

unsafe fn sc1200_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    static UDMA_TIMING: [[u32; 3]; 3] = [
        [0x00921250, 0x00911140, 0x00911030],
        [0x00932470, 0x00922260, 0x00922140],
        [0x009436a1, 0x00933481, 0x00923261],
    ];
    static MWDMA_TIMING: [[u32; 3]; 3] = [
        [0x00077771, 0x00012121, 0x00002020],
        [0x000bbbb2, 0x00024241, 0x00013131],
        [0x000ffff3, 0x00035352, 0x00015151],
    ];

    let clock = sc1200_clock() as usize;
    let pdev = to_pci_dev((*(*ap).host).dev);
    let reg: u32 = 0x40 + 0x10 * (*ap).port_no;
    let mode = (*adev).dma_mode;
    let format: u32;

    if mode >= XFER_UDMA_0 {
        format = UDMA_TIMING[clock][(mode - XFER_UDMA_0) as usize];
    } else {
        format = MWDMA_TIMING[clock][(mode - XFER_MW_DMA_0) as usize];
    }

    if (*adev).devno == 0 {
        let mut timings: u32 = 0;
        pci_read_config_dword(pdev, reg + 4, &mut timings);
        timings &= 0x80000000u32;
        timings |= format;
        pci_write_config_dword(pdev, reg + 4, timings);
    } else {
        pci_write_config_dword(pdev, reg + 12, format);
    }
}

unsafe fn sc1200_qc_issue(qc: *mut ata_queued_cmd) -> u32 {
    let ap = (*qc).ap;
    let adev = (*qc).dev;
    let prev = (*ap).private_data as *mut ata_device;

    /* See if the DMA settings could be wrong */
    if ata_dma_enabled(adev) && adev != prev && !prev.is_null() {
        /* Maybe, but do the channels match MWDMA/UDMA ? */
        if (ata_using_udma(adev) && !ata_using_udma(prev))
            || (ata_using_udma(prev) && !ata_using_udma(adev))
        {
            /* Switch the mode bits */
            sc1200_set_dmamode(ap, adev);
        }
    }

    ata_bmdma_qc_issue(qc)
}

unsafe fn sc1200_qc_defer(qc: *mut ata_queued_cmd) -> i32 {
    let host = (*(*qc).ap).host;
    let alt = (*host).ports[1 ^ (*(*qc).ap).port_no];
    let rc = ata_std_qc_defer(qc);

    /* First apply the usual rules */
    if rc != 0 {
        return rc;
    }

    /* Now apply serialization rules. Only allow a command if the
       other channel state machine is idle */
    if !alt.is_null() && (*alt).qc_active != 0 {
        return ATA_DEFER_PORT;
    }
    0
}

// The following static callback/table/module declarations correspond directly
// to the C driver objects and use types and constants supplied by libata.
static SC1200_SHT: scsi_host_template = scsi_host_template {
    sg_tablesize: LIBATA_DUMB_MAX_PRD,
    dma_boundary: ATA_DMA_BOUNDARY,
    ..ATA_BASE_SHT!(DRV_NAME)
};

static mut SC1200_PORT_OPS: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma_port_ops,
    qc_prep: Some(ata_bmdma_dumb_qc_prep),
    qc_issue: Some(sc1200_qc_issue),
    qc_defer: Some(sc1200_qc_defer),
    cable_detect: Some(ata_cable_40wire),
    set_piomode: Some(sc1200_set_piomode),
    set_dmamode: Some(sc1200_set_dmamode),
};

unsafe fn sc1200_init_one(dev: *mut pci_dev, _id: *const pci_device_id) -> i32 {
    static INFO: ata_port_info = ata_port_info {
        flags: ATA_FLAG_SLAVE_POSS,
        pio_mask: ATA_PIO4,
        mwdma_mask: ATA_MWDMA2,
        udma_mask: ATA_UDMA2,
        port_ops: &SC1200_PORT_OPS,
    };
    let ppi: [*const ata_port_info; 2] = [&INFO, core::ptr::null()];

    ata_pci_bmdma_init_one(dev, ppi.as_ptr(), &SC1200_SHT, core::ptr::null_mut(), 0)
}

static SC1200: [pci_device_id; 2] = [
    PCI_VDEVICE!(NS, PCI_DEVICE_ID_NS_SCx200_IDE),
    pci_device_id::default(),
];

static SC1200_PCI_DRIVER: pci_driver = pci_driver {
    name: DRV_NAME,
    id_table: SC1200.as_ptr(),
    probe: Some(sc1200_init_one),
    remove: Some(ata_pci_remove_one),
    // CONFIG_PM_SLEEP conditionally supplies suspend and resume callbacks.
    ..pci_driver::default()
};

module_pci_driver!(SC1200_PCI_DRIVER);
module_author!("Alan Cox, Mark Lord");
module_description!("low-level driver for the NS/AMD SC1200");
module_license!("GPL");
module_device_table!(pci, SC1200);
module_version!(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
