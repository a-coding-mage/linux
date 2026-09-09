// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  sata_svw.c - ServerWorks / Apple K2 SATA
 *
 *  Maintained by: Benjamin Herrenschmidt <benh@kernel.crashing.org> and
 *                 Jeff Garzik <jgarzik@pobox.com>
 *                  Please ALWAYS copy linux-ide@vger.kernel.org
 *                 on emails.
 *
 *  Copyright 2003 Benjamin Herrenschmidt <benh@kernel.crashing.org>
 *
 *  Bits from Jeff Garzik, Copyright RedHat, Inc.
 *
 *  This driver probably works with non-Apple versions of the
 *  Broadcom chipset...
 *
 *  libata documentation is available via 'make {ps|pdf}docs',
 *  as Documentation/driver-api/libata.rst
 *
 *  Hardware documentation available under NDA.
 */

// Kernel and libata dependencies are supplied by other translation units.

const DRV_NAME: &str = "sata_svw";
const DRV_VERSION: &str = "2.3";

const K2_FLAG_SATA_8_PORTS: u32 = 1 << 24;
const K2_FLAG_NO_ATAPI_DMA: u32 = 1 << 25;
const K2_FLAG_BAR_POS_3: u32 = 1 << 26;
const K2_SATA_TF_CMD_OFFSET: usize = 0x00;
const K2_SATA_TF_DATA_OFFSET: usize = 0x00;
const K2_SATA_TF_ERROR_OFFSET: usize = 0x04;
const K2_SATA_TF_NSECT_OFFSET: usize = 0x08;
const K2_SATA_TF_LBAL_OFFSET: usize = 0x0c;
const K2_SATA_TF_LBAM_OFFSET: usize = 0x10;
const K2_SATA_TF_LBAH_OFFSET: usize = 0x14;
const K2_SATA_TF_DEVICE_OFFSET: usize = 0x18;
const K2_SATA_TF_CMDSTAT_OFFSET: usize = 0x1c;
const K2_SATA_TF_CTL_OFFSET: usize = 0x20;
const K2_SATA_DMA_CMD_OFFSET: usize = 0x30;
const K2_SATA_SCR_STATUS_OFFSET: usize = 0x40;
const K2_SATA_SCR_ERROR_OFFSET: usize = 0x44;
const K2_SATA_SCR_CONTROL_OFFSET: usize = 0x48;
const K2_SATA_SICR1_OFFSET: usize = 0x80;
const K2_SATA_SICR2_OFFSET: usize = 0x84;
const K2_SATA_SIM_OFFSET: usize = 0x88;
const K2_SATA_PORT_OFFSET: usize = 0x100;

const CHIP_SVW4: usize = 0;
const CHIP_SVW8: usize = 1;
const CHIP_SVW42: usize = 2;
const CHIP_SVW43: usize = 3;

unsafe fn k2_stat_check_status(ap: *mut ata_port) -> u8 { readl((*ap).ioaddr.status_addr) as u8 }

unsafe fn k2_sata_check_atapi_dma(qc: *mut ata_queued_cmd) -> i32 {
    let cmnd = (*(*qc).scsicmd).cmnd[0];
    if (*(*qc).ap).flags & K2_FLAG_NO_ATAPI_DMA != 0 { return -1; }
    match cmnd { READ_10 | READ_12 | READ_16 | WRITE_10 | WRITE_12 | WRITE_16 => 0, _ => -1 }
}

unsafe fn k2_sata_scr_read(link: *mut ata_link, sc_reg: u32, val: *mut u32) -> i32 {
    if sc_reg > SCR_CONTROL { return -EINVAL; }
    *val = readl((*(*link).ap).ioaddr.scr_addr.add((sc_reg * 4) as usize));
    0
}

unsafe fn k2_sata_scr_write(link: *mut ata_link, sc_reg: u32, val: u32) -> i32 {
    if sc_reg > SCR_CONTROL { return -EINVAL; }
    writel(val, (*(*link).ap).ioaddr.scr_addr.add((sc_reg * 4) as usize));
    0
}

unsafe fn k2_sata_softreset(link: *mut ata_link, class: *mut u32, deadline: c_ulong) -> i32 {
    let mmio = (*(*link).ap).ioaddr.bmdma_addr;
    let mut dmactl = readb(mmio.add(ATA_DMA_CMD as usize));
    if dmactl & ATA_DMA_START != 0 { dmactl &= !ATA_DMA_START; writeb(dmactl, mmio.add(ATA_DMA_CMD as usize)); }
    ata_sff_softreset(link, class, deadline)
}

unsafe fn k2_sata_hardreset(link: *mut ata_link, class: *mut u32, deadline: c_ulong) -> i32 {
    let mmio = (*(*link).ap).ioaddr.bmdma_addr;
    let mut dmactl = readb(mmio.add(ATA_DMA_CMD as usize));
    if dmactl & ATA_DMA_START != 0 { dmactl &= !ATA_DMA_START; writeb(dmactl, mmio.add(ATA_DMA_CMD as usize)); }
    sata_sff_hardreset(link, class, deadline)
}

unsafe fn k2_sata_tf_load(ap: *mut ata_port, tf: *const ata_taskfile) {
    let ioaddr = &mut (*ap).ioaddr;
    let is_addr = (*tf).flags & ATA_TFLAG_ISADDR;
    if (*tf).ctl != (*ap).last_ctl { writeb((*tf).ctl, ioaddr.ctl_addr); (*ap).last_ctl = (*tf).ctl; ata_wait_idle(ap); }
    if is_addr != 0 && (*tf).flags & ATA_TFLAG_LBA48 != 0 {
        writew((*tf).feature | ((*tf).hob_feature as u16) << 8, ioaddr.feature_addr);
        writew((*tf).nsect | ((*tf).hob_nsect as u16) << 8, ioaddr.nsect_addr);
        writew((*tf).lbal | ((*tf).hob_lbal as u16) << 8, ioaddr.lbal_addr);
        writew((*tf).lbam | ((*tf).hob_lbam as u16) << 8, ioaddr.lbam_addr);
        writew((*tf).lbah | ((*tf).hob_lbah as u16) << 8, ioaddr.lbah_addr);
    } else if is_addr != 0 {
        writew((*tf).feature, ioaddr.feature_addr); writew((*tf).nsect, ioaddr.nsect_addr);
        writew((*tf).lbal, ioaddr.lbal_addr); writew((*tf).lbam, ioaddr.lbam_addr); writew((*tf).lbah, ioaddr.lbah_addr);
    }
    if (*tf).flags & ATA_TFLAG_DEVICE != 0 { writeb((*tf).device, ioaddr.device_addr); }
    ata_wait_idle(ap);
}

unsafe fn k2_sata_tf_read(ap: *mut ata_port, tf: *mut ata_taskfile) {
    let ioaddr = &(*ap).ioaddr;
    (*tf).status = k2_stat_check_status(ap);
    (*tf).device = readw(ioaddr.device_addr); let error = readw(ioaddr.error_addr);
    let nsect = readw(ioaddr.nsect_addr); let lbal = readw(ioaddr.lbal_addr);
    let lbam = readw(ioaddr.lbam_addr); let lbah = readw(ioaddr.lbah_addr);
    (*tf).error = error; (*tf).nsect = nsect; (*tf).lbal = lbal; (*tf).lbam = lbam; (*tf).lbah = lbah;
    if (*tf).flags & ATA_TFLAG_LBA48 != 0 { (*tf).hob_feature = error >> 8; (*tf).hob_nsect = nsect >> 8; (*tf).hob_lbal = lbal >> 8; (*tf).hob_lbam = lbam >> 8; (*tf).hob_lbah = lbah >> 8; }
}

unsafe fn k2_bmdma_setup_mmio(qc: *mut ata_queued_cmd) {
    let ap = (*qc).ap; let rw = (*qc).tf.flags & ATA_TFLAG_WRITE; let mmio = (*ap).ioaddr.bmdma_addr;
    mb(); writel((*ap).bmdma_prd_dma, mmio.add(ATA_DMA_TABLE_OFS as usize));
    let mut dmactl = readb(mmio.add(ATA_DMA_CMD as usize)); dmactl &= !(ATA_DMA_WR | ATA_DMA_START); if rw == 0 { dmactl |= ATA_DMA_WR; }
    writeb(dmactl, mmio.add(ATA_DMA_CMD as usize));
    if (*qc).tf.protocol != ATA_PROT_DMA { ((*ap).ops).sff_exec_command.unwrap()(ap, &(*qc).tf); }
}

unsafe fn k2_bmdma_start_mmio(qc: *mut ata_queued_cmd) {
    let ap = (*qc).ap; let mmio = (*ap).ioaddr.bmdma_addr; let dmactl = readb(mmio.add(ATA_DMA_CMD as usize));
    writeb(dmactl | ATA_DMA_START, mmio.add(ATA_DMA_CMD as usize));
    if (*qc).tf.protocol == ATA_PROT_DMA { ((*ap).ops).sff_exec_command.unwrap()(ap, &(*qc).tf); }
}

unsafe fn k2_sata_show_info(m: *mut seq_file, shost: *mut Scsi_Host) -> i32 {
    let ap = ata_shost_to_port(shost); if ap.is_null() { return 0; }
    let mut np = pci_device_to_OF_node(to_pci_dev((*ap).host.as_ref().unwrap().dev)); if np.is_null() { return 0; }
    let index = if ap == (*ap).host.as_ref().unwrap().ports[0] { 0 } else { 1 };
    while !np.is_null() { let mut reg = 0u64; if of_property_read_reg(np, 0, &mut reg, core::ptr::null_mut()) == 0 && index as u64 == reg { seq_printf(m, "devspec: %pOF\n", np); break; } np = (*np).sibling; }
    0
}

unsafe fn k2_sata_setup_port(port: *mut ata_ioports, base: *mut u8) {
    (*port).cmd_addr = base.add(K2_SATA_TF_CMD_OFFSET); (*port).data_addr = base.add(K2_SATA_TF_DATA_OFFSET);
    (*port).feature_addr = base.add(K2_SATA_TF_ERROR_OFFSET); (*port).error_addr = base.add(K2_SATA_TF_ERROR_OFFSET);
    (*port).nsect_addr = base.add(K2_SATA_TF_NSECT_OFFSET); (*port).lbal_addr = base.add(K2_SATA_TF_LBAL_OFFSET); (*port).lbam_addr = base.add(K2_SATA_TF_LBAM_OFFSET); (*port).lbah_addr = base.add(K2_SATA_TF_LBAH_OFFSET); (*port).device_addr = base.add(K2_SATA_TF_DEVICE_OFFSET);
    (*port).command_addr = base.add(K2_SATA_TF_CMDSTAT_OFFSET); (*port).status_addr = base.add(K2_SATA_TF_CMDSTAT_OFFSET); (*port).altstatus_addr = base.add(K2_SATA_TF_CTL_OFFSET); (*port).ctl_addr = base.add(K2_SATA_TF_CTL_OFFSET);
    (*port).bmdma_addr = base.add(K2_SATA_DMA_CMD_OFFSET); (*port).scr_addr = base.add(K2_SATA_SCR_STATUS_OFFSET);
}

unsafe fn k2_sata_init_one(pdev: *mut pci_dev, ent: *const pci_device_id) -> i32 {
    ata_print_version_once(&mut (*pdev).dev, DRV_VERSION);
    let mut n_ports = 4;
    let ppi = &k2_port_info[(*ent).driver_data as usize] as *const ata_port_info;
    if (*ppi).flags & K2_FLAG_SATA_8_PORTS != 0 { n_ports = 8; }
    let host = ata_host_alloc_pinfo(&mut (*pdev).dev, &ppi, n_ports); if host.is_null() { return -ENOMEM; }
    let mut bar_pos = 5; if (*ppi).flags & K2_FLAG_BAR_POS_3 != 0 { bar_pos = 3; }
    let mut rc = pcim_enable_device(pdev); if rc != 0 { return rc; }
    if pci_resource_len(pdev, bar_pos) == 0 { pcim_pin_device(pdev); return -ENODEV; }
    rc = pcim_iomap_regions(pdev, 1 << bar_pos, DRV_NAME); if rc == -EBUSY { pcim_pin_device(pdev); } if rc != 0 { return rc; }
    (*host).iomap = pcim_iomap_table(pdev); let mmio_base = (*host).iomap[bar_pos];
    for i in 0..(*host).n_ports as usize { let ap = (*host).ports[i]; let offset = i * K2_SATA_PORT_OFFSET; k2_sata_setup_port(&mut (*ap).ioaddr, mmio_base.add(offset)); ata_port_pbar_desc(ap, 5, -1, "mmio"); ata_port_pbar_desc(ap, 5, offset as isize, "port"); }
    rc = dma_set_mask_and_coherent(&mut (*pdev).dev, ATA_DMA_MASK); if rc != 0 { return rc; }
    writel(readl(mmio_base.add(K2_SATA_SICR1_OFFSET)) & !0x00040000, mmio_base.add(K2_SATA_SICR1_OFFSET));
    writel(0xffffffff, mmio_base.add(K2_SATA_SCR_ERROR_OFFSET)); writel(0, mmio_base.add(K2_SATA_SIM_OFFSET));
    pci_set_master(pdev); ata_host_activate(host, (*pdev).irq, ata_bmdma_interrupt, IRQF_SHARED, &k2_sata_sht)
}

// PCI IDs: 0x0240 K2, 0x0241 Frodo4, 0x0242 Frodo8, 0x024a/0x024b HT1000,
// and 0x0410/0x0411 ServerWorks variants. Kernel PCI_VDEVICE initializers
// and module_pci_driver registration are supplied by the target ABI.
static mut k2_sata_ops: ata_port_operations = ata_port_operations { ..ata_bmdma_port_ops };
static mut k2_port_info: [ata_port_info; 4] = [ata_port_info { ..ata_port_info_zero }; 4];
static mut k2_sata_pci_tbl: [pci_device_id; 9] = [pci_device_id { ..pci_device_id_zero }; 9];
static mut k2_sata_pci_driver: pci_driver = pci_driver { ..pci_driver_zero };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
