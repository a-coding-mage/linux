// SPDX-License-Identifier: GPL-2.0-only
/*
 * pata-cs5535.c - CS5535 PATA for new ATA layer
 * (C) 2005-2006 Red Hat Inc
 * Alan Cox <alan@lxorguk.ukuu.org.uk>
 *
 * Based upon cs5535.c from AMD as cleaned up and made readable and Linux
 * style by Wolfgang Zuleger and Alexander Kiausch.
 * Loosely based on the piix & svwks drivers.
 */

// Linux kernel headers and build-time configuration are supplied externally.

const DRV_NAME: &str = "pata_cs5535";
const DRV_VERSION: &str = "0.2.12";

const MSR_ATAC_BASE: u32 = 0x51300000;
const ATAC_GLD_MSR_CAP: u32 = MSR_ATAC_BASE + 0;
const ATAC_GLD_MSR_CONFIG: u32 = MSR_ATAC_BASE + 0x01;
const ATAC_GLD_MSR_SMI: u32 = MSR_ATAC_BASE + 0x02;
const ATAC_GLD_MSR_ERROR: u32 = MSR_ATAC_BASE + 0x03;
const ATAC_GLD_MSR_PM: u32 = MSR_ATAC_BASE + 0x04;
const ATAC_GLD_MSR_DIAG: u32 = MSR_ATAC_BASE + 0x05;
const ATAC_IO_BAR: u32 = MSR_ATAC_BASE + 0x08;
const ATAC_RESET: u32 = MSR_ATAC_BASE + 0x10;
const ATAC_CH0D0_PIO: u32 = MSR_ATAC_BASE + 0x20;
const ATAC_CH0D0_DMA: u32 = MSR_ATAC_BASE + 0x21;
const ATAC_CH0D1_PIO: u32 = MSR_ATAC_BASE + 0x22;
const ATAC_CH0D1_DMA: u32 = MSR_ATAC_BASE + 0x23;
const ATAC_PCI_ABRTERR: u32 = MSR_ATAC_BASE + 0x24;

const ATAC_BM0_CMD_PRIM: u32 = 0x00;
const ATAC_BM0_STS_PRIM: u32 = 0x02;
const ATAC_BM0_PRD: u32 = 0x04;
const CS5535_CABLE_DETECT: u32 = 0x48;

unsafe fn cs5535_cable_detect(ap: *mut ata_port) -> i32 {
    let mut cable: u8 = 0;
    let pdev = to_pci_dev((*(*ap).host).dev);
    pci_read_config_byte(pdev, CS5535_CABLE_DETECT, &mut cable);
    if cable & 1 != 0 { ATA_CBL_PATA80 } else { ATA_CBL_PATA40 }
}

unsafe fn cs5535_set_piomode(_ap: *mut ata_port, adev: *mut ata_device) {
    const PIO_TIMINGS: [u16; 5] = [0xF7F4, 0xF173, 0x8141, 0x5131, 0x1131];
    const PIO_CMD_TIMINGS: [u16; 5] = [0xF7F4, 0x53F3, 0x13F1, 0x5131, 0x1131];
    let mut reg: u32 = 0;
    let mut dummy: u32 = 0;
    let pair = ata_dev_pair(adev);
    let mode = (*adev).pio_mode - XFER_PIO_0;
    let mut cmdmode = mode;
    if !pair.is_null() {
        let pairmode = (*pair).pio_mode - XFER_PIO_0;
        cmdmode = core::cmp::min(mode, pairmode);
        if cmdmode < pairmode {
            wrmsr(ATAC_CH0D0_PIO + 2 * (*pair).devno,
                  ((PIO_CMD_TIMINGS[cmdmode as usize] as u32) << 16) |
                  PIO_TIMINGS[pairmode as usize] as u32, 0);
        }
    }
    wrmsr(ATAC_CH0D0_PIO + 2 * (*adev).devno,
          ((PIO_CMD_TIMINGS[cmdmode as usize] as u32) << 16) |
          PIO_TIMINGS[mode as usize] as u32, 0);
    rdmsr(ATAC_CH0D0_DMA + 2 * (*adev).devno, &mut reg, &mut dummy);
    wrmsr(ATAC_CH0D0_DMA + 2 * (*adev).devno, reg | 0x80000000u32, 0);
}

unsafe fn cs5535_set_dmamode(_ap: *mut ata_port, adev: *mut ata_device) {
    const UDMA_TIMINGS: [u32; 5] = [0x7F7436A1, 0x7F733481, 0x7F723261, 0x7F713161, 0x7F703061];
    const MWDMA_TIMINGS: [u32; 3] = [0x7F0FFFF3, 0x7F035352, 0x7F024241];
    let mut reg: u32 = 0;
    let mut dummy: u32 = 0;
    let mode = (*adev).dma_mode;
    rdmsr(ATAC_CH0D0_DMA + 2 * (*adev).devno, &mut reg, &mut dummy);
    reg &= 0x80000000u32;
    if mode >= XFER_UDMA_0 { reg |= UDMA_TIMINGS[(mode - XFER_UDMA_0) as usize]; }
    else { reg |= MWDMA_TIMINGS[(mode - XFER_MW_DMA_0) as usize]; }
    wrmsr(ATAC_CH0D0_DMA + 2 * (*adev).devno, reg, 0);
}

static mut cs5535_sht: scsi_host_template = scsi_host_template { ATA_BMDMA_SHT!(DRV_NAME) };
static mut cs5535_port_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma_port_ops,
    cable_detect: Some(cs5535_cable_detect),
    set_piomode: Some(cs5535_set_piomode),
    set_dmamode: Some(cs5535_set_dmamode),
};

unsafe fn cs5535_init_one(dev: *mut pci_dev, _id: *const pci_device_id) -> i32 {
    static INFO: ata_port_info = ata_port_info {
        flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: ATA_MWDMA2,
        udma_mask: ATA_UDMA4, port_ops: &cs5535_port_ops,
    };
    let ppi = [&INFO, &ata_dummy_port_info];
    ata_pci_bmdma_init_one(dev, ppi.as_ptr(), &cs5535_sht, core::ptr::null_mut(), 0)
}

static cs5535: [pci_device_id; 3] = [
    PCI_VDEVICE!(NS, PCI_DEVICE_ID_NS_CS5535_IDE),
    PCI_VDEVICE!(AMD, PCI_DEVICE_ID_AMD_CS5535_IDE),
    pci_device_id {},
];

static mut cs5535_pci_driver: pci_driver = pci_driver {
    name: DRV_NAME, id_table: cs5535.as_ptr(), probe: Some(cs5535_init_one),
    remove: Some(ata_pci_remove_one),
    // CONFIG_PM_SLEEP conditionally supplies suspend and resume handlers.
};

module_pci_driver!(cs5535_pci_driver);
module_author!("Alan Cox, Jens Altmann, Wolfgan Zuleger, Alexander Kiausch");
module_description!("low-level driver for the NS/AMD 5535");
module_license!("GPL");
module_device_table!(pci, cs5535);
module_version!(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
