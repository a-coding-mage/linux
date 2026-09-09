// SPDX-License-Identifier: GPL-2.0-or-later
//
// acard-ahci.c - ACard AHCI SATA support
// Rust source-level translation; kernel dependencies are supplied externally.

pub const DRV_NAME: &str = "acard-ahci";
pub const DRV_VERSION: &str = "1.0";
pub const ACARD_AHCI_RX_FIS_SZ: usize = 128;
pub const AHCI_PCI_BAR: i32 = 5;

#[repr(C)]
pub struct acard_sg {
    pub addr: __le32,
    pub addr_hi: __le32,
    pub reserved: __le32,
    pub size: __le32,
}

pub const board_acard_ahci: board_ids = 0;
pub type board_ids = u32;

extern "C" {
    fn acard_ahci_qc_prep(qc: *mut ata_queued_cmd) -> ata_completion_errors;
    fn acard_ahci_qc_fill_rtf(qc: *mut ata_queued_cmd);
    fn acard_ahci_port_start(ap: *mut ata_port) -> i32;
    fn acard_ahci_init_one(pdev: *mut pci_dev, ent: *const pci_device_id) -> i32;
}

#[cfg(CONFIG_PM_SLEEP)]
extern "C" {
    fn acard_ahci_pci_device_suspend(pdev: *mut pci_dev, mesg: pm_message_t) -> i32;
    fn acard_ahci_pci_device_resume(pdev: *mut pci_dev) -> i32;
}

static mut acard_ahci_sht: scsi_host_template = scsi_host_template { /* AHCI_SHT("acard-ahci") */ };

static mut acard_ops: ata_port_operations = ata_port_operations {
    inherits: unsafe { &ahci_ops as *const _ as *mut _ },
    qc_prep: Some(acard_ahci_qc_prep),
    qc_fill_rtf: Some(acard_ahci_qc_fill_rtf),
    port_start: Some(acard_ahci_port_start),
};

static mut acard_ahci_port_info: [ata_port_info; 1] = [ata_port_info {
    private_data: AHCI_HFLAG_NO_NCQ as *mut core::ffi::c_void,
    flags: AHCI_FLAG_COMMON,
    pio_mask: ATA_PIO4,
    udma_mask: ATA_UDMA6,
    port_ops: unsafe { &acard_ops },
}];

static mut acard_ahci_pci_tbl: [pci_device_id; 2] = [
    pci_device_id { vendor: ARTOP, device: 0x000d, driver_data: board_acard_ahci as u64 },
    pci_device_id { vendor: 0, device: 0, driver_data: 0 },
];

static mut acard_ahci_pci_driver: pci_driver = pci_driver {
    name: DRV_NAME,
    id_table: unsafe { &acard_ahci_pci_tbl },
    probe: Some(acard_ahci_init_one),
    remove: Some(ata_pci_remove_one),
    #[cfg(CONFIG_PM_SLEEP)]
    suspend: Some(acard_ahci_pci_device_suspend),
    #[cfg(CONFIG_PM_SLEEP)]
    resume: Some(acard_ahci_pci_device_resume),
};

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn acard_ahci_pci_device_suspend(pdev: *mut pci_dev, mesg: pm_message_t) -> i32 {
    let host = pci_get_drvdata(pdev);
    let hpriv = (*host).private_data as *mut ahci_host_priv;
    let mmio = (*hpriv).mmio;
    let mut ctl: u32;
    if (mesg.event & PM_EVENT_SUSPEND) != 0 && ((*hpriv).flags & AHCI_HFLAG_NO_SUSPEND) != 0 {
        dev_err(&(*pdev).dev, "BIOS update required for suspend/resume\n");
        return -EIO;
    }
    if (mesg.event & PM_EVENT_SLEEP) != 0 {
        ctl = readl(mmio.add(HOST_CTL));
        ctl &= !HOST_IRQ_EN;
        writel(ctl, mmio.add(HOST_CTL));
        readl(mmio.add(HOST_CTL));
    }
    ata_pci_device_suspend(pdev, mesg)
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn acard_ahci_pci_device_resume(pdev: *mut pci_dev) -> i32 {
    let host = pci_get_drvdata(pdev);
    let mut rc = ata_pci_device_do_resume(pdev);
    if rc != 0 { return rc; }
    if (*pdev).dev.power.power_state.event == PM_EVENT_SUSPEND {
        rc = ahci_reset_controller(host);
        if rc != 0 { return rc; }
        ahci_init_controller(host);
    }
    ata_host_resume(host);
    0
}

unsafe fn acard_ahci_pci_print_info(host: *mut ata_host) {
    let pdev = to_pci_dev((*host).dev);
    let mut cc: u16 = 0;
    pci_read_config_word(pdev, 0x0a, &mut cc);
    let scc_s = if cc == PCI_CLASS_STORAGE_IDE { "IDE" } else if cc == PCI_CLASS_STORAGE_SATA { "SATA" } else if cc == PCI_CLASS_STORAGE_RAID { "RAID" } else { "unknown" };
    ahci_print_info(host, scc_s);
}

unsafe fn acard_ahci_fill_sg(qc: *mut ata_queued_cmd, cmd_tbl: *mut core::ffi::c_void) -> u32 {
    let acard_sg = cmd_tbl.add(AHCI_CMD_TBL_HDR_SZ) as *mut acard_sg;
    let mut last_si: usize = 0;
    let mut si: usize = 0;
    for_each_sg!((*qc).sg, sg, (*qc).n_elem, si, {
        let addr = sg_dma_address(sg);
        let sg_len = sg_dma_len(sg);
        (*acard_sg.add(si)).addr = cpu_to_le32((addr & 0xffffffff) as u32);
        (*acard_sg.add(si)).addr_hi = cpu_to_le32(((addr >> 16) >> 16) as u32);
        (*acard_sg.add(si)).size = cpu_to_le32(sg_len);
        last_si = si;
    });
    (*acard_sg.add(last_si)).size |= cpu_to_le32(1u32 << 31);
    si as u32
}

unsafe fn acard_ahci_qc_prep(qc: *mut ata_queued_cmd) -> ata_completion_errors {
    let ap = (*qc).ap;
    let pp = (*ap).private_data as *mut ahci_port_priv;
    let is_atapi = ata_is_atapi((*qc).tf.protocol);
    let cmd_tbl = (*pp).cmd_tbl.add((*qc).hw_tag * AHCI_CMD_TBL_SZ);
    ata_tf_to_fis(&(*qc).tf, (*(*qc).dev).link.pmp, 1, cmd_tbl);
    if is_atapi {
        memset(cmd_tbl.add(AHCI_CMD_TBL_CDB), 0, 32);
        memcpy(cmd_tbl.add(AHCI_CMD_TBL_CDB), (*qc).cdb, (*(*qc).dev).cdb_len);
    }
    if ((*qc).flags & ATA_QCFLAG_DMAMAP) != 0 { acard_ahci_fill_sg(qc, cmd_tbl as *mut _); }
    let mut opts = 5u32 | ((*(*qc).dev).link.pmp << 12);
    if ((*qc).tf.flags & ATA_TFLAG_WRITE) != 0 { opts |= AHCI_CMD_WRITE; }
    if is_atapi { opts |= AHCI_CMD_ATAPI | AHCI_CMD_PREFETCH; }
    ahci_fill_cmd_slot(pp, (*qc).hw_tag, opts);
    AC_ERR_OK
}

unsafe fn acard_ahci_qc_fill_rtf(qc: *mut ata_queued_cmd) {
    let pp = (*qc).ap.private_data as *mut ahci_port_priv;
    let mut rx_fis = (*pp).rx_fis;
    if (*pp).fbs_enabled { rx_fis = rx_fis.add((*(*qc).dev).link.pmp * ACARD_AHCI_RX_FIS_SZ); }
    if (*qc).tf.protocol == ATA_PROT_PIO && (*qc).dma_dir == DMA_FROM_DEVICE && ((*qc).flags & ATA_QCFLAG_EH) == 0 {
        ata_tf_from_fis(rx_fis.add(RX_FIS_PIO_SETUP), &mut (*qc).result_tf);
        (*qc).result_tf.status = *rx_fis.add(RX_FIS_PIO_SETUP + 15);
    } else { ata_tf_from_fis(rx_fis.add(RX_FIS_D2H_REG), &mut (*qc).result_tf); }
}

unsafe fn acard_ahci_port_start(ap: *mut ata_port) -> i32 {
    let hpriv = (*(*ap).host).private_data as *mut ahci_host_priv;
    let dev = (*(*ap).host).dev;
    let pp = devm_kzalloc(dev, core::mem::size_of::<ahci_port_priv>(), GFP_KERNEL) as *mut ahci_port_priv;
    if pp.is_null() { return -ENOMEM; }
    if ((*hpriv).cap & HOST_CAP_FBS) != 0 && sata_pmp_supported(ap) {
        let port_mmio = ahci_port_base(ap);
        let cmd = readl(port_mmio.add(PORT_CMD));
        if (cmd & PORT_CMD_FBSCP) != 0 { (*pp).fbs_supported = true; }
        else if ((*hpriv).flags & AHCI_HFLAG_YES_FBS) != 0 { dev_info(dev, "port %d can do FBS, forcing FBSCP\n", (*ap).port_no); (*pp).fbs_supported = true; }
        else { dev_warn(dev, "port %d is not capable of FBS\n", (*ap).port_no); }
    }
    let (dma_sz, rx_fis_sz) = if (*pp).fbs_supported { (AHCI_PORT_PRIV_FBS_DMA_SZ, ACARD_AHCI_RX_FIS_SZ * 16) } else { (AHCI_PORT_PRIV_DMA_SZ, ACARD_AHCI_RX_FIS_SZ) };
    let mut mem_dma: dma_addr_t = 0;
    let mem = dmam_alloc_coherent(dev, dma_sz, &mut mem_dma, GFP_KERNEL);
    if mem.is_null() { return -ENOMEM; }
    (*pp).cmd_slot = mem; (*pp).cmd_slot_dma = mem_dma;
    (*pp).rx_fis = mem.add(AHCI_CMD_SLOT_SZ); (*pp).rx_fis_dma = mem_dma + AHCI_CMD_SLOT_SZ;
    (*pp).cmd_tbl = (*pp).rx_fis.add(rx_fis_sz); (*pp).cmd_tbl_dma = (*pp).rx_fis_dma + rx_fis_sz;
    (*pp).intr_mask = DEF_PORT_IRQ;
    (*ap).private_data = pp as *mut _;
    ahci_port_resume(ap)
}

unsafe fn acard_ahci_init_one(pdev: *mut pci_dev, ent: *const pci_device_id) -> i32 {
    let board_id = (*ent).driver_data as usize;
    let mut pi = acard_ahci_port_info[board_id];
    let ppi = [&pi as *const _, core::ptr::null()];
    let dev = &mut (*pdev).dev;
    let hpriv = devm_kzalloc(dev, core::mem::size_of::<ahci_host_priv>(), GFP_KERNEL) as *mut ahci_host_priv;
    let mut rc = pcim_enable_device(pdev); if rc != 0 { return rc; }
    rc = pcim_request_all_regions(pdev, DRV_NAME); if rc == -EBUSY { pcim_pin_device(pdev); } if rc != 0 { return rc; }
    if hpriv.is_null() { return -ENOMEM; }
    (*hpriv).irq = (*pdev).irq; (*hpriv).flags |= pi.private_data as usize as u64;
    if ((*hpriv).flags & AHCI_HFLAG_NO_MSI) == 0 { pci_enable_msi(pdev); }
    (*hpriv).mmio = pcim_iomap(pdev, AHCI_PCI_BAR, 0); if (*hpriv).mmio.is_null() { return -ENOMEM; }
    ahci_save_initial_config(dev, hpriv);
    if ((*hpriv).cap & HOST_CAP_NCQ) != 0 { pi.flags |= ATA_FLAG_NCQ; }
    if ((*hpriv).cap & HOST_CAP_PMP) != 0 { pi.flags |= ATA_FLAG_PMP; }
    ahci_set_em_messages(hpriv, &mut pi);
    let n_ports = core::cmp::max(ahci_nr_ports((*hpriv).cap), fls((*hpriv).port_map));
    let host = ata_host_alloc_pinfo(dev, ppi.as_ptr(), n_ports); if host.is_null() { return -ENOMEM; }
    (*host).private_data = hpriv as *mut _;
    if ((*hpriv).cap & HOST_CAP_SSS) == 0 || ahci_ignore_sss { (*host).flags |= ATA_HOST_PARALLEL_SCAN; }
    for i in 0..(*host).n_ports { let ap = *(*host).ports.add(i); ata_port_pbar_desc(ap, AHCI_PCI_BAR, -1, "abar"); ata_port_pbar_desc(ap, AHCI_PCI_BAR, 0x100 + (*ap).port_no * 0x80, "port"); if ((*hpriv).port_map & (1 << i)) == 0 { (*ap).ops = &ata_dummy_port_ops; } }
    rc = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(if ((*hpriv).cap & HOST_CAP_64) != 0 { 64 } else { 32 })); if rc != 0 { dev_err(dev, "DMA enable failed\n"); return rc; }
    rc = ahci_reset_controller(host); if rc != 0 { return rc; }
    ahci_init_controller(host); acard_ahci_pci_print_info(host); pci_set_master(pdev);
    ahci_host_activate(host, &acard_ahci_sht)
}

// module_pci_driver(acard_ahci_pci_driver);
// MODULE_AUTHOR("Jeff Garzik"); MODULE_DESCRIPTION("ACard AHCI SATA low-level driver");
// MODULE_LICENSE("GPL"); MODULE_DEVICE_TABLE(pci, acard_ahci_pci_tbl); MODULE_VERSION(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
