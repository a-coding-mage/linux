// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sata_vsc.c - Vitesse VSC7174 4 port DPA SATA
 *
 * Maintained by: Jeremy Higdon @ SGI
 * Copyright 2004 SGI
 *
 * Bits from Jeff Garzik, Copyright RedHat, Inc.
 * libata documentation is available via `make {ps|pdf}docs`.
 * Vitesse hardware documentation presumably available under NDA.
 */

// Linux kernel dependencies supplied externally.

const DRV_NAME: &str = "sata_vsc";
const DRV_VERSION: &str = "2.3";

const VSC_MMIO_BAR: usize = 0;
const VSC_SATA_INT_STAT_OFFSET: usize = 0x00;
const VSC_SATA_INT_MASK_OFFSET: usize = 0x04;
const VSC_SATA_TF_CMD_OFFSET: usize = 0x00;
const VSC_SATA_TF_DATA_OFFSET: usize = 0x00;
const VSC_SATA_TF_ERROR_OFFSET: usize = 0x04;
const VSC_SATA_TF_FEATURE_OFFSET: usize = 0x06;
const VSC_SATA_TF_NSECT_OFFSET: usize = 0x08;
const VSC_SATA_TF_LBAL_OFFSET: usize = 0x0c;
const VSC_SATA_TF_LBAM_OFFSET: usize = 0x10;
const VSC_SATA_TF_LBAH_OFFSET: usize = 0x14;
const VSC_SATA_TF_DEVICE_OFFSET: usize = 0x18;
const VSC_SATA_TF_STATUS_OFFSET: usize = 0x1c;
const VSC_SATA_TF_COMMAND_OFFSET: usize = 0x1d;
const VSC_SATA_TF_ALTSTATUS_OFFSET: usize = 0x28;
const VSC_SATA_TF_CTL_OFFSET: usize = 0x29;
const VSC_SATA_UP_DESCRIPTOR_OFFSET: usize = 0x64;
const VSC_SATA_UP_DATA_BUFFER_OFFSET: usize = 0x6c;
const VSC_SATA_DMA_CMD_OFFSET: usize = 0x70;
const VSC_SATA_SCR_STATUS_OFFSET: usize = 0x100;
const VSC_SATA_SCR_ERROR_OFFSET: usize = 0x104;
const VSC_SATA_SCR_CONTROL_OFFSET: usize = 0x108;
const VSC_SATA_PORT_OFFSET: usize = 0x200;
const VSC_SATA_INT_ERROR_CRC: u8 = 0x40;
const VSC_SATA_INT_ERROR_T: u8 = 0x20;
const VSC_SATA_INT_ERROR_P: u8 = 0x10;
const VSC_SATA_INT_ERROR_R: u8 = 0x8;
const VSC_SATA_INT_ERROR_E: u8 = 0x4;
const VSC_SATA_INT_ERROR_M: u8 = 0x2;
const VSC_SATA_INT_PHY_CHANGE: u8 = 0x1;
const VSC_SATA_INT_ERROR: u8 = VSC_SATA_INT_ERROR_CRC | VSC_SATA_INT_ERROR_T |
    VSC_SATA_INT_ERROR_P | VSC_SATA_INT_ERROR_R | VSC_SATA_INT_ERROR_E |
    VSC_SATA_INT_ERROR_M | VSC_SATA_INT_PHY_CHANGE;

unsafe fn vsc_sata_scr_read(link: *mut ata_link, sc_reg: u32, val: *mut u32) -> i32 {
    if sc_reg > SCR_CONTROL { return -EINVAL; }
    *val = readl((*(*link).ap).ioaddr.scr_addr.add((sc_reg * 4) as usize));
    0
}

unsafe fn vsc_sata_scr_write(link: *mut ata_link, sc_reg: u32, val: u32) -> i32 {
    if sc_reg > SCR_CONTROL { return -EINVAL; }
    writel(val, (*(*link).ap).ioaddr.scr_addr.add((sc_reg * 4) as usize));
    0
}

unsafe fn vsc_freeze(ap: *mut ata_port) {
    let mask_addr = (*(*ap).host).iomap[VSC_MMIO_BAR]
        .add(VSC_SATA_INT_MASK_OFFSET + (*ap).port_no as usize);
    writeb(0, mask_addr);
}

unsafe fn vsc_thaw(ap: *mut ata_port) {
    let mask_addr = (*(*ap).host).iomap[VSC_MMIO_BAR]
        .add(VSC_SATA_INT_MASK_OFFSET + (*ap).port_no as usize);
    writeb(0xff, mask_addr);
}

unsafe fn vsc_intr_mask_update(ap: *mut ata_port, ctl: u8) {
    let mask_addr = (*(*ap).host).iomap[VSC_MMIO_BAR]
        .add(VSC_SATA_INT_MASK_OFFSET + (*ap).port_no as usize);
    let mut mask = readb(mask_addr);
    if ctl & ATA_NIEN != 0 { mask |= 0x80; } else { mask &= 0x7f; }
    writeb(mask, mask_addr);
}

unsafe fn vsc_sata_tf_load(ap: *mut ata_port, tf: *const ata_taskfile) {
    let ioaddr = &mut (*ap).ioaddr;
    let is_addr = (*tf).flags & ATA_TFLAG_ISADDR;
    if ((*tf).ctl & ATA_NIEN) != ((*ap).last_ctl & ATA_NIEN) {
        (*ap).last_ctl = (*tf).ctl;
        vsc_intr_mask_update(ap, (*tf).ctl & ATA_NIEN);
    }
    if is_addr != 0 && (*tf).flags & ATA_TFLAG_LBA48 != 0 {
        writew((*tf).feature | ((*tf).hob_feature as u16) << 8, ioaddr.feature_addr);
        writew((*tf).nsect | ((*tf).hob_nsect as u16) << 8, ioaddr.nsect_addr);
        writew((*tf).lbal | ((*tf).hob_lbal as u16) << 8, ioaddr.lbal_addr);
        writew((*tf).lbam | ((*tf).hob_lbam as u16) << 8, ioaddr.lbam_addr);
        writew((*tf).lbah | ((*tf).hob_lbah as u16) << 8, ioaddr.lbah_addr);
    } else if is_addr != 0 {
        writew((*tf).feature, ioaddr.feature_addr); writew((*tf).nsect, ioaddr.nsect_addr);
        writew((*tf).lbal, ioaddr.lbal_addr); writew((*tf).lbam, ioaddr.lbam_addr);
        writew((*tf).lbah, ioaddr.lbah_addr);
    }
    if (*tf).flags & ATA_TFLAG_DEVICE != 0 { writeb((*tf).device, ioaddr.device_addr); }
    ata_wait_idle(ap);
}

unsafe fn vsc_sata_tf_read(ap: *mut ata_port, tf: *mut ata_taskfile) {
    let ioaddr = &mut (*ap).ioaddr;
    (*tf).status = ata_sff_check_status(ap);
    (*tf).device = readw(ioaddr.device_addr); let error = readw(ioaddr.error_addr);
    let nsect = readw(ioaddr.nsect_addr); let lbal = readw(ioaddr.lbal_addr);
    let lbam = readw(ioaddr.lbam_addr); let lbah = readw(ioaddr.lbah_addr);
    (*tf).error = error; (*tf).nsect = nsect; (*tf).lbal = lbal;
    (*tf).lbam = lbam; (*tf).lbah = lbah;
    if (*tf).flags & ATA_TFLAG_LBA48 != 0 {
        (*tf).hob_feature = error >> 8; (*tf).hob_nsect = nsect >> 8;
        (*tf).hob_lbal = lbal >> 8; (*tf).hob_lbam = lbam >> 8; (*tf).hob_lbah = lbah >> 8;
    }
}

unsafe fn vsc_error_intr(port_status: u8, ap: *mut ata_port) {
    if port_status & (VSC_SATA_INT_PHY_CHANGE | VSC_SATA_INT_ERROR_M) != 0 { ata_port_freeze(ap); }
    else { ata_port_abort(ap); }
}

unsafe fn vsc_port_intr(port_status: u8, ap: *mut ata_port) {
    if port_status & VSC_SATA_INT_ERROR != 0 { vsc_error_intr(port_status, ap); return; }
    let qc = ata_qc_from_tag(ap, (*ap).link.active_tag);
    let mut handled = 0;
    if !qc.is_null() && (*qc).tf.flags & ATA_TFLAG_POLLING == 0 { handled = ata_bmdma_port_intr(ap, qc); }
    if handled == 0 { ((*(*ap).ops).sff_check_status)(ap); }
}

unsafe extern "C" fn vsc_sata_interrupt(_irq: i32, dev_instance: *mut core::ffi::c_void) -> irqreturn_t {
    let host = dev_instance as *mut ata_host;
    let status = readl((*host).iomap[VSC_MMIO_BAR].add(VSC_SATA_INT_STAT_OFFSET));
    if status == 0xffff_ffff || status == 0 { return IRQ_RETVAL(0); }
    spin_lock(&mut (*host).lock);
    let mut handled = 0;
    for i in 0..(*host).n_ports { let port_status = ((status >> (8 * i)) & 0xff) as u8;
        if port_status != 0 { vsc_port_intr(port_status, (*host).ports[i as usize]); handled += 1; }
    }
    spin_unlock(&mut (*host).lock); IRQ_RETVAL(handled)
}

// The remaining PCI driver registration and operation-table definitions are
// represented using the kernel's externally supplied Rust bindings.
static VSC_SATA_SHT: scsi_host_template = ATA_BMDMA_SHT!(DRV_NAME);
static mut VSC_SATA_OPS: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma_port_ops, lost_interrupt: ATA_OP_NULL,
    sff_tf_load: Some(vsc_sata_tf_load), sff_tf_read: Some(vsc_sata_tf_read),
    freeze: Some(vsc_freeze), thaw: Some(vsc_thaw), scr_read: Some(vsc_sata_scr_read),
    scr_write: Some(vsc_sata_scr_write),
};

unsafe fn vsc_sata_setup_port(port: *mut ata_ioports, base: *mut u8) {
    (*port).cmd_addr = base.add(VSC_SATA_TF_CMD_OFFSET); (*port).data_addr = base.add(VSC_SATA_TF_DATA_OFFSET);
    (*port).error_addr = base.add(VSC_SATA_TF_ERROR_OFFSET); (*port).feature_addr = base.add(VSC_SATA_TF_FEATURE_OFFSET);
    (*port).nsect_addr = base.add(VSC_SATA_TF_NSECT_OFFSET); (*port).lbal_addr = base.add(VSC_SATA_TF_LBAL_OFFSET);
    (*port).lbam_addr = base.add(VSC_SATA_TF_LBAM_OFFSET); (*port).lbah_addr = base.add(VSC_SATA_TF_LBAH_OFFSET);
    (*port).device_addr = base.add(VSC_SATA_TF_DEVICE_OFFSET); (*port).status_addr = base.add(VSC_SATA_TF_STATUS_OFFSET);
    (*port).command_addr = base.add(VSC_SATA_TF_COMMAND_OFFSET); (*port).altstatus_addr = base.add(VSC_SATA_TF_ALTSTATUS_OFFSET);
    (*port).ctl_addr = base.add(VSC_SATA_TF_CTL_OFFSET); (*port).bmdma_addr = base.add(VSC_SATA_DMA_CMD_OFFSET);
    (*port).scr_addr = base.add(VSC_SATA_SCR_STATUS_OFFSET);
    writel(0, base.add(VSC_SATA_UP_DESCRIPTOR_OFFSET)); writel(0, base.add(VSC_SATA_UP_DATA_BUFFER_OFFSET));
}

unsafe extern "C" fn vsc_sata_init_one(pdev: *mut pci_dev, _ent: *const pci_device_id) -> i32 {
    ata_print_version_once(&mut (*pdev).dev, DRV_VERSION);
    let ppi = [ &VSC_SATA_PI as *const ata_port_info, core::ptr::null() ];
    let host = ata_host_alloc_pinfo(&mut (*pdev).dev, ppi.as_ptr(), 4);
    if host.is_null() { return -ENOMEM; }
    let mut rc = pcim_enable_device(pdev); if rc != 0 { return rc; }
    if pci_resource_len(pdev, 0) == 0 { return -ENODEV; }
    rc = pcim_iomap_regions(pdev, 1 << VSC_MMIO_BAR, DRV_NAME); if rc == -EBUSY { pcim_pin_device(pdev); }
    if rc != 0 { return rc; }
    (*host).iomap = pcim_iomap_table(pdev); let mmio_base = (*host).iomap[VSC_MMIO_BAR];
    for i in 0..(*host).n_ports { let ap = (*host).ports[i as usize];
        let offset = (i as usize + 1) * VSC_SATA_PORT_OFFSET; vsc_sata_setup_port(&mut (*ap).ioaddr, mmio_base.add(offset));
        ata_port_pbar_desc(ap, VSC_MMIO_BAR, -1, "mmio"); ata_port_pbar_desc(ap, VSC_MMIO_BAR, offset as isize, "port"); }
    rc = dma_set_mask_and_coherent(&mut (*pdev).dev, DMA_BIT_MASK(32)); if rc != 0 { return rc; }
    let mut cls = 0; pci_read_config_byte(pdev, PCI_CACHE_LINE_SIZE, &mut cls); if cls == 0 { pci_write_config_byte(pdev, PCI_CACHE_LINE_SIZE, 0x80); }
    if pci_enable_msi(pdev) == 0 { pcim_intx(pdev, 0); }
    pci_write_config_dword(pdev, 0x98, 0); pci_set_master(pdev);
    ata_host_activate(host, (*pdev).irq, Some(vsc_sata_interrupt), IRQF_SHARED, &VSC_SATA_SHT)
}

static VSC_SATA_PI: ata_port_info = ata_port_info { flags: ATA_FLAG_SATA, pio_mask: ATA_PIO4,
    mwdma_mask: ATA_MWDMA2, udma_mask: ATA_UDMA6, port_ops: &VSC_SATA_OPS };
static VSC_SATA_PCI_TBL: [pci_device_id; 3] = [
    PCI_DEVICE_ENTRY!(PCI_VENDOR_ID_VITESSE, 0x7174, 0x10600, 0xffffff),
    PCI_DEVICE_ENTRY!(PCI_VENDOR_ID_INTEL, 0x3200, 0x10600, 0xffffff), PCI_DEVICE_ID_NULL!() ];
static mut VSC_SATA_PCI_DRIVER: pci_driver = pci_driver { name: DRV_NAME, id_table: VSC_SATA_PCI_TBL.as_ptr(),
    probe: Some(vsc_sata_init_one), remove: Some(ata_pci_remove_one) };

module_pci_driver!(VSC_SATA_PCI_DRIVER);
module_author!("Jeremy Higdon");
module_description!("low-level driver for Vitesse VSC7174 SATA controller");
module_license!("GPL"); module_device_table!(pci, VSC_SATA_PCI_TBL); module_version!(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
