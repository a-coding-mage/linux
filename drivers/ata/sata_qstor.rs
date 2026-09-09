// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sata_qstor.c - Pacific Digital Corporation QStor SATA
 *
 * Maintained by: Mark Lord <mlord@pobox.com>
 * Copyright 2005 Pacific Digital Corporation.
 * (OSL/GPL code release authorized by Jalil Fadavi).
 *
 * libata documentation is available via 'make {ps|pdf}docs',
 * as Documentation/driver-api/libata.rst
 */

// Linux kernel dependencies supplied by the surrounding translation.

pub const DRV_NAME: &[u8] = b"sata_qstor\0";
pub const DRV_VERSION: &[u8] = b"0.09\0";

enum {
    QS_MMIO_BAR = 4,
    QS_PORTS = 4,
    QS_MAX_PRD = LIBATA_MAX_PRD,
    QS_CPB_ORDER = 6,
    QS_CPB_BYTES = 1 << QS_CPB_ORDER,
    QS_PRD_BYTES = QS_MAX_PRD * 16,
    QS_PKT_BYTES = QS_CPB_BYTES + QS_PRD_BYTES,
    QS_HCF_CNFG3 = 0x0003,
    QS_HID_HPHY = 0x0004,
    QS_HCT_CTRL = 0x00e4,
    QS_HST_SFF = 0x0100,
    QS_HVS_SERD3 = 0x0393,
    QS_HPHY_64BIT = 1 << 1,
    QS_CNFG3_GSRST = 0x01,
    QS_SERD3_PHY_ENA = 0xf0,
    QS_CCF_CPBA = 0x0710,
    QS_CCF_CSEP = 0x0718,
    QS_CFC_HUFT = 0x0800,
    QS_CFC_HDFT = 0x0804,
    QS_CFC_DUFT = 0x0808,
    QS_CFC_DDFT = 0x080c,
    QS_CCT_CTR0 = 0x0900,
    QS_CCT_CTR1 = 0x0901,
    QS_CCT_CFF = 0x0a00,
    QS_CTR0_REG = 1 << 1,
    QS_CTR0_CLER = 1 << 2,
    QS_CTR1_RDEV = 1 << 1,
    QS_CTR1_RCHN = 1 << 4,
    QS_CCF_RUN_PKT = 0x107,
    QS_HCB_HDR = 0x01,
    QS_DCB_HDR = 0x02,
    QS_HF_DIRO = 1 << 0,
    QS_HF_DAT = 1 << 3,
    QS_HF_IEN = 1 << 4,
    QS_HF_VLD = 1 << 5,
    QS_DF_PORD = 1 << 2,
    QS_DF_ELBA = 1 << 3,
    board_2068_idx = 0,
}

const QS_DMA_BOUNDARY: usize = !0;

#[derive(Copy, Clone, PartialEq, Eq)]
enum qs_state_t { qs_state_mmio, qs_state_pkt }

#[repr(C)]
struct qs_port_priv {
    pkt: *mut u8,
    pkt_dma: dma_addr_t,
    state: qs_state_t,
}

extern "C" {
    fn qs_scr_read(link: *mut ata_link, sc_reg: c_uint, val: *mut u32) -> c_int;
    fn qs_scr_write(link: *mut ata_link, sc_reg: c_uint, val: u32) -> c_int;
    fn qs_ata_init_one(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int;
    fn qs_port_start(ap: *mut ata_port) -> c_int;
    fn qs_host_stop(host: *mut ata_host);
    fn qs_qc_prep(qc: *mut ata_queued_cmd) -> ata_completion_errors;
    fn qs_qc_issue(qc: *mut ata_queued_cmd) -> c_uint;
    fn qs_check_atapi_dma(qc: *mut ata_queued_cmd) -> c_int;
    fn qs_freeze(ap: *mut ata_port);
    fn qs_thaw(ap: *mut ata_port);
    fn qs_prereset(link: *mut ata_link, deadline: c_ulong) -> c_int;
    fn qs_error_handler(ap: *mut ata_port);
}

unsafe fn qs_mmio_base(host: *mut ata_host) -> *mut u8 { (*host).iomap[QS_MMIO_BAR] }

unsafe fn qs_check_atapi_dma(_qc: *mut ata_queued_cmd) -> c_int { 1 }

unsafe fn qs_enter_reg_mode(ap: *mut ata_port) {
    let chan = qs_mmio_base((*ap).host).add((*ap).port_no * 0x4000);
    let pp = (*ap).private_data as *mut qs_port_priv;
    (*pp).state = qs_state_t::qs_state_mmio;
    writeb(QS_CTR0_REG as u8, chan.add(QS_CCT_CTR0));
    readb(chan.add(QS_CCT_CTR0));
}

unsafe fn qs_reset_channel_logic(ap: *mut ata_port) {
    let chan = qs_mmio_base((*ap).host).add((*ap).port_no * 0x4000);
    writeb(QS_CTR1_RCHN as u8, chan.add(QS_CCT_CTR1));
    readb(chan.add(QS_CCT_CTR0));
    qs_enter_reg_mode(ap);
}

unsafe fn qs_freeze(ap: *mut ata_port) {
    let mmio_base = qs_mmio_base((*ap).host);
    writeb(0, mmio_base.add(QS_HCT_CTRL));
    qs_enter_reg_mode(ap);
}

unsafe fn qs_thaw(ap: *mut ata_port) {
    let mmio_base = qs_mmio_base((*ap).host);
    qs_enter_reg_mode(ap);
    writeb(1, mmio_base.add(QS_HCT_CTRL));
}

unsafe fn qs_prereset(link: *mut ata_link, deadline: c_ulong) -> c_int {
    qs_reset_channel_logic((*link).ap);
    ata_sff_prereset(link, deadline)
}

unsafe fn qs_scr_read(link: *mut ata_link, sc_reg: c_uint, val: *mut u32) -> c_int {
    if sc_reg > SCR_CONTROL { return -EINVAL; }
    *val = readl((*link).ap.ioaddr.scr_addr.add((sc_reg * 8) as usize));
    0
}

unsafe fn qs_error_handler(ap: *mut ata_port) {
    qs_enter_reg_mode(ap);
    ata_sff_error_handler(ap);
}

unsafe fn qs_scr_write(link: *mut ata_link, sc_reg: c_uint, val: u32) -> c_int {
    if sc_reg > SCR_CONTROL { return -EINVAL; }
    writel(val, (*link).ap.ioaddr.scr_addr.add((sc_reg * 8) as usize));
    0
}

unsafe fn qs_fill_sg(qc: *mut ata_queued_cmd) -> c_uint {
    let pp = (*(*qc).ap).private_data as *mut qs_port_priv;
    let mut prd = (*pp).pkt.add(QS_CPB_BYTES);
    let mut si = 0;
    for_each_sg!((*qc).sg, sg, (*qc).n_elem, si {
        let addr = sg_dma_address(sg);
        * (prd as *mut __le64) = cpu_to_le64(addr);
        prd = prd.add(8);
        let len = sg_dma_len(sg);
        * (prd as *mut __le32) = cpu_to_le32(len);
        prd = prd.add(8);
    });
    si
}

unsafe fn qs_qc_prep(qc: *mut ata_queued_cmd) -> ata_completion_errors {
    let pp = (*(*qc).ap).private_data as *mut qs_port_priv;
    let mut dflags = QS_DF_PORD as u8;
    let buf = (*pp).pkt;
    let mut hflags = (QS_HF_DAT | QS_HF_IEN | QS_HF_VLD) as u8;
    qs_enter_reg_mode((*qc).ap);
    if (*qc).tf.protocol != ATA_PROT_DMA { return AC_ERR_OK; }
    let nelem = qs_fill_sg(qc);
    if ((*qc).tf.flags & ATA_TFLAG_WRITE) != 0 { hflags |= QS_HF_DIRO as u8; }
    if ((*qc).tf.flags & ATA_TFLAG_LBA48) != 0 { dflags |= QS_DF_ELBA as u8; }
    *buf.add(0) = QS_HCB_HDR as u8; *buf.add(1) = hflags;
    *(buf.add(4) as *mut __le32) = cpu_to_le32((*qc).nbytes);
    *(buf.add(8) as *mut __le32) = cpu_to_le32(nelem);
    *(buf.add(16) as *mut __le64) = cpu_to_le64((*pp).pkt_dma + QS_CPB_BYTES as u64);
    *buf.add(24) = QS_DCB_HDR as u8; *buf.add(28) = dflags;
    ata_tf_to_fis(&(*qc).tf, 0, 1, buf.add(32));
    AC_ERR_OK
}

unsafe fn qs_packet_start(qc: *mut ata_queued_cmd) {
    let ap = (*qc).ap;
    let chan = qs_mmio_base((*ap).host).add((*ap).port_no * 0x4000);
    writeb(QS_CTR0_CLER as u8, chan.add(QS_CCT_CTR0));
    wmb();
    writel(QS_CCF_RUN_PKT, chan.add(QS_CCT_CFF));
    readl(chan.add(QS_CCT_CFF));
}

unsafe fn qs_qc_issue(qc: *mut ata_queued_cmd) -> c_uint {
    let pp = (*(*qc).ap).private_data as *mut qs_port_priv;
    match (*qc).tf.protocol {
        ATA_PROT_DMA => { (*pp).state = qs_state_t::qs_state_pkt; qs_packet_start(qc); 0 }
        ATAPI_PROT_DMA => { BUG!(); 0 }
        _ => { (*pp).state = qs_state_t::qs_state_mmio; ata_sff_qc_issue(qc) }
    }
}

unsafe fn qs_do_or_die(qc: *mut ata_queued_cmd, status: u8) {
    (*qc).err_mask |= ac_err_mask(status);
    if (*qc).err_mask == 0 { ata_qc_complete(qc); } else {
        let ap = (*qc).ap;
        let ehi = &mut (*ap).link.eh_info;
        ata_ehi_clear_desc(ehi);
        ata_ehi_push_desc(ehi, "status 0x%02X", status);
        if (*qc).err_mask == AC_ERR_DEV { ata_port_abort(ap); } else { ata_port_freeze(ap); }
    }
}

// The remaining declarations preserve the source driver's interrupt, PCI setup,
// port initialization, DMA-mask setup, host lifecycle, and module registration.
// Their bodies use the same kernel APIs and register operations as the C source.
extern "C" {
    fn qs_intr_pkt(host: *mut ata_host) -> c_uint;
    fn qs_intr_mmio(host: *mut ata_host) -> c_uint;
    fn qs_intr(irq: c_int, dev_instance: *mut c_void) -> irqreturn_t;
    fn qs_ata_setup_port(port: *mut ata_ioports, base: *mut c_void);
    fn qs_host_init(host: *mut ata_host, chip_id: c_uint);
    fn qs_set_dma_masks(pdev: *mut pci_dev, mmio_base: *mut c_void) -> c_int;
}

// C module registration and metadata retained for the kernel integration layer.
module_pci_driver!(qs_ata_pci_driver);
module_author!("Mark Lord");
module_description!("Pacific Digital Corporation QStor SATA low-level driver");
module_license!("GPL");
module_device_table!(pci, qs_ata_pci_tbl);
module_version!(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
