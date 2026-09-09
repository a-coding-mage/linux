// SPDX-License-Identifier: GPL-2.0-only
/*
 * pata_ns87415.c - NS87415 (and PARISC SUPERIO 87560) PATA
 *
 * Direct Rust translation of the original implementation.
 */

const DRV_NAME: &str = "pata_ns87415";
const DRV_VERSION: &str = "0.0.1";

unsafe fn ns87415_set_mode(ap: *mut ata_port, adev: *mut ata_device, mode: u8) {
    let dev = to_pci_dev((*(*ap).host).dev);
    let unit: i32 = 2 * (*ap).port_no as i32 + (*adev).devno as i32;
    let timing: i32 = 0x44 + 2 * unit;
    let t_clock: u32 = 1_000_000_000 / 33333;
    let mut t: ata_timing = core::mem::zeroed();
    let mut clocking: u16;
    let mut iordy: u8 = 0;
    let mut status: u8 = 0;

    ata_timing_compute(adev, (*adev).pio_mode, &mut t, t_clock, 0);
    clocking = 17 - clamp_val(t.active, 2, 17);
    clocking |= (16 - clamp_val(t.recover, 1, 16)) << 4;
    clocking |= clocking << 8;
    pci_write_config_word(dev, timing, clocking);

    pci_read_config_byte(dev, 0x42, &mut iordy);
    iordy &= !(1 << (4 + unit));
    if mode >= XFER_MW_DMA_0 || ata_pio_need_iordy(adev) == 0 {
        iordy |= 1 << (4 + unit);
    }

    pci_read_config_byte(dev, 0x43, &mut status);
    while status & 0x03 != 0 {
        udelay(1);
        pci_read_config_byte(dev, 0x43, &mut status);
    }
    pci_write_config_byte(dev, 0x42, iordy);
}

unsafe fn ns87415_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    ns87415_set_mode(ap, adev, (*adev).pio_mode);
}

unsafe fn ns87415_bmdma_setup(qc: *mut ata_queued_cmd) {
    let ap = (*qc).ap;
    let rw = (*qc).tf.flags & ATA_TFLAG_WRITE;
    let mut dmactl: u8;
    mb();
    iowrite32((*ap).bmdma_prd_dma, (*ap).ioaddr.bmdma_addr + ATA_DMA_TABLE_OFS);
    dmactl = ioread8((*ap).ioaddr.bmdma_addr + ATA_DMA_CMD);
    dmactl &= !(ATA_DMA_WR | ATA_DMA_START);
    dmactl |= ATA_DMA_INTR | ATA_DMA_ERR;
    if rw == 0 { dmactl |= ATA_DMA_WR; }
    iowrite8(dmactl, (*ap).ioaddr.bmdma_addr + ATA_DMA_CMD);
    ((*ap).ops).sff_exec_command.unwrap()(ap, &(*qc).tf);
}

unsafe fn ns87415_bmdma_start(qc: *mut ata_queued_cmd) {
    ns87415_set_mode((*qc).ap, (*qc).dev, (*qc).dev.as_ref().unwrap().dma_mode);
    ata_bmdma_start(qc);
}

unsafe fn ns87415_bmdma_stop(qc: *mut ata_queued_cmd) {
    ata_bmdma_stop(qc);
    ns87415_set_mode((*qc).ap, (*qc).dev, (*qc).dev.as_ref().unwrap().pio_mode);
}

unsafe fn ns87415_irq_clear(ap: *mut ata_port) {
    let mmio = (*ap).ioaddr.bmdma_addr;
    if mmio.is_null() { return; }
    iowrite8(ioread8(mmio + ATA_DMA_CMD) | ATA_DMA_INTR | ATA_DMA_ERR, mmio + ATA_DMA_CMD);
}

unsafe fn ns87415_check_atapi_dma(_qc: *mut ata_queued_cmd) -> i32 { -EOPNOTSUPP }

#[cfg(CONFIG_SUPERIO)]
const SUPERIO_IDE_MAX_RETRIES: i32 = 25;

#[cfg(CONFIG_SUPERIO)]
unsafe fn ns87560_read_buggy(port: *mut core::ffi::c_void) -> u8 {
    let mut tmp: u8;
    let mut retries = SUPERIO_IDE_MAX_RETRIES;
    loop {
        tmp = ioread8(port);
        if tmp != 0 { return tmp; }
        udelay(50);
        retries -= 1;
        if retries <= 0 { break; }
    }
    tmp
}

#[cfg(CONFIG_SUPERIO)]
unsafe fn ns87560_check_status(ap: *mut ata_port) -> u8 { ns87560_read_buggy((*ap).ioaddr.status_addr) }

#[cfg(CONFIG_SUPERIO)]
unsafe fn ns87560_tf_read(ap: *mut ata_port, tf: *mut ata_taskfile) {
    let ioaddr = &(*ap).ioaddr;
    (*tf).status = ns87560_check_status(ap);
    (*tf).error = ioread8(ioaddr.error_addr);
    (*tf).nsect = ioread8(ioaddr.nsect_addr);
    (*tf).lbal = ioread8(ioaddr.lbal_addr);
    (*tf).lbam = ioread8(ioaddr.lbam_addr);
    (*tf).lbah = ioread8(ioaddr.lbah_addr);
    (*tf).device = ns87560_read_buggy(ioaddr.device_addr);
    if (*tf).flags & ATA_TFLAG_LBA48 != 0 {
        iowrite8((*tf).ctl | ATA_HOB, ioaddr.ctl_addr);
        (*tf).hob_feature = ioread8(ioaddr.error_addr);
        (*tf).hob_nsect = ioread8(ioaddr.nsect_addr);
        (*tf).hob_lbal = ioread8(ioaddr.lbal_addr);
        (*tf).hob_lbam = ioread8(ioaddr.lbam_addr);
        (*tf).hob_lbah = ioread8(ioaddr.lbah_addr);
        iowrite8((*tf).ctl, ioaddr.ctl_addr);
        (*ap).last_ctl = (*tf).ctl;
    }
}

#[cfg(CONFIG_SUPERIO)]
unsafe fn ns87560_bmdma_status(ap: *mut ata_port) -> u8 {
    ns87560_read_buggy((*ap).ioaddr.bmdma_addr + ATA_DMA_STATUS)
}

static mut ns87415_pata_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma_port_ops,
    check_atapi_dma: Some(ns87415_check_atapi_dma), bmdma_setup: Some(ns87415_bmdma_setup),
    bmdma_start: Some(ns87415_bmdma_start), bmdma_stop: Some(ns87415_bmdma_stop),
    sff_irq_clear: Some(ns87415_irq_clear), cable_detect: Some(ata_cable_40wire),
    set_piomode: Some(ns87415_set_piomode),
};

#[cfg(CONFIG_SUPERIO)]
static mut ns87560_pata_ops: ata_port_operations = ata_port_operations {
    inherits: &ns87415_pata_ops, sff_tf_read: Some(ns87560_tf_read),
    sff_check_status: Some(ns87560_check_status), bmdma_status: Some(ns87560_bmdma_status),
};

static ns87415_sht: scsi_host_template = ATA_BMDMA_SHT!(DRV_NAME);

unsafe fn ns87415_fixup(pdev: *mut pci_dev) {
    pci_write_config_byte(pdev, 0x55, 0xEE);
    pci_write_config_byte(pdev, 0x54, 0xB7);
}

unsafe fn ns87415_init_one(pdev: *mut pci_dev, _ent: *const pci_device_id) -> i32 {
    static mut info: ata_port_info = ata_port_info { flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: ATA_MWDMA2, port_ops: &ns87415_pata_ops };
    let mut ppi: [*const ata_port_info; 2] = [&info, core::ptr::null()];
    #[cfg(CONFIG_SUPERIO)]
    {
        static mut info87560: ata_port_info = ata_port_info { flags: ATA_FLAG_SLAVE_POSS, pio_mask: ATA_PIO4, mwdma_mask: ATA_MWDMA2, port_ops: &ns87560_pata_ops };
        if PCI_SLOT((*pdev).devfn) == 0x0E { ppi[0] = &info87560; }
    }
    ata_print_version_once(&mut (*pdev).dev, DRV_VERSION);
    let rc = pcim_enable_device(pdev);
    if rc != 0 { return rc; }
    ns87415_fixup(pdev);
    ata_pci_bmdma_init_one(pdev, ppi.as_ptr(), &ns87415_sht, core::ptr::null_mut(), 0)
}

static ns87415_pci_tbl: [pci_device_id; 2] = [PCI_VDEVICE!(NS, PCI_DEVICE_ID_NS_87415), pci_device_id::default()];

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn ns87415_reinit_one(pdev: *mut pci_dev) -> i32 {
    let host = pci_get_drvdata(pdev);
    let rc = ata_pci_device_do_resume(pdev);
    if rc != 0 { return rc; }
    ns87415_fixup(pdev);
    ata_host_resume(host);
    0
}

static mut ns87415_pci_driver: pci_driver = pci_driver {
    name: DRV_NAME, id_table: ns87415_pci_tbl.as_ptr(), probe: Some(ns87415_init_one),
    remove: Some(ata_pci_remove_one),
    #[cfg(CONFIG_PM_SLEEP)] suspend: Some(ata_pci_device_suspend), resume: Some(ns87415_reinit_one),
};

module_pci_driver!(ns87415_pci_driver);
module_author!("Alan Cox");
module_description!("ATA low-level driver for NS87415 controllers");
module_license!("GPL");
module_device_table!(pci, ns87415_pci_tbl);
module_version!(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
