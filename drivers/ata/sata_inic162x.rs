// SPDX-License-Identifier: GPL-2.0-only
/*
 * sata_inic162x.c - Driver for Initio 162x SATA controllers
 *
 * Rust translation of the original Linux kernel implementation.
 * The kernel headers and symbols referenced below are supplied externally.
 */

const DRV_NAME: &str = "sata_inic162x";
const DRV_VERSION: &str = "0.4";

const MMIO_BAR_PCI: i32 = 5;
const MMIO_BAR_CARDBUS: i32 = 1;
const NR_PORTS: usize = 2;
const IDMA_CPB_TBL_SIZE: usize = 4 * 32;
const INIC_DMA_BOUNDARY: u32 = 0xffffff;
const HOST_ACTRL: usize = 0x08;
const HOST_CTL: usize = 0x7c;
const HOST_STAT: usize = 0x7e;
const HOST_IRQ_STAT: usize = 0xbc;
const HOST_IRQ_MASK: usize = 0xbe;
const PORT_SIZE: usize = 0x40;
const PORT_TF_DATA: usize = 0x00;
const PORT_TF_FEATURE: usize = 0x01;
const PORT_TF_NSECT: usize = 0x02;
const PORT_TF_LBAL: usize = 0x03;
const PORT_TF_LBAM: usize = 0x04;
const PORT_TF_LBAH: usize = 0x05;
const PORT_TF_DEVICE: usize = 0x06;
const PORT_TF_COMMAND: usize = 0x07;
const PORT_TF_ALT_STAT: usize = 0x08;
const PORT_IRQ_STAT: usize = 0x09;
const PORT_IRQ_MASK: usize = 0x0a;
const PORT_PRD_CTL: usize = 0x0b;
const PORT_PRD_ADDR: usize = 0x0c;
const PORT_PRD_XFERLEN: usize = 0x10;
const PORT_IDMA_CTL: usize = 0x14;
const PORT_CPB_CPBLAR: usize = 0x18;
const PORT_CPB_PTQFIFO: usize = 0x1c;
const PORT_IDMA_STAT: usize = 0x16;
const PORT_RPQ_FIFO: usize = 0x1e;
const PORT_RPQ_CNT: usize = 0x1f;
const PORT_SCR: usize = 0x20;
const HCTL_LEDEN: u16 = 1 << 3;
const HCTL_IRQOFF: u16 = 1 << 8;
const HCTL_FTHD0: u16 = 1 << 10;
const HCTL_FTHD1: u16 = 1 << 11;
const HCTL_PWRDWN: u16 = 1 << 12;
const HCTL_SOFTRST: u16 = 1 << 13;
const HCTL_RPGSEL: u16 = 1 << 15;
const HCTL_KNOWN_BITS: u16 = HCTL_IRQOFF | HCTL_PWRDWN | HCTL_SOFTRST | HCTL_RPGSEL;
const HIRQ_PORT0: u16 = 1 << 0;
const HIRQ_PORT1: u16 = 1 << 1;
const HIRQ_SOFT: u16 = 1 << 14;
const HIRQ_GLOBAL: u16 = 1 << 15;
const PIRQ_OFFLINE: u8 = 1 << 0;
const PIRQ_ONLINE: u8 = 1 << 1;
const PIRQ_COMPLETE: u8 = 1 << 2;
const PIRQ_FATAL: u8 = 1 << 3;
const PIRQ_ATA: u8 = 1 << 4;
const PIRQ_REPLY: u8 = 1 << 5;
const PIRQ_PENDING: u8 = 1 << 7;
const PIRQ_ERR: u8 = PIRQ_OFFLINE | PIRQ_ONLINE | PIRQ_FATAL;
const PIRQ_MASK_DEFAULT: u8 = PIRQ_REPLY | PIRQ_ATA;
const PIRQ_MASK_FREEZE: u8 = 0xff;
const PRD_CTL_START: u8 = 1 << 0;
const PRD_CTL_WR: u8 = 1 << 3;
const PRD_CTL_DMAEN: u8 = 1 << 7;
const IDMA_CTL_RST_ATA: u16 = 1 << 2;
const IDMA_CTL_RST_IDMA: u16 = 1 << 5;
const IDMA_CTL_GO: u16 = 1 << 7;
const IDMA_CTL_ATA_NIEN: u16 = 1 << 8;
const IDMA_STAT_PERR: u16 = 1 << 0;
const IDMA_STAT_CPBERR: u16 = 1 << 1;
const IDMA_STAT_LGCY: u16 = 1 << 3;
const IDMA_STAT_UIRQ: u16 = 1 << 4;
const IDMA_STAT_STPD: u16 = 1 << 5;
const IDMA_STAT_PSD: u16 = 1 << 6;
const IDMA_STAT_DONE: u16 = 1 << 7;
const IDMA_STAT_ERR: u16 = IDMA_STAT_PERR | IDMA_STAT_CPBERR;
const CPB_CTL_VALID: u8 = 1 << 0;
const CPB_CTL_QUEUED: u8 = 1 << 1;
const CPB_CTL_DATA: u8 = 1 << 2;
const CPB_CTL_IEN: u8 = 1 << 3;
const CPB_CTL_DEVDIR: u8 = 1 << 4;
const CPB_RESP_DONE: u8 = 1 << 0;
const CPB_RESP_REL: u8 = 1 << 1;
const CPB_RESP_IGNORED: u8 = 1 << 2;
const CPB_RESP_ATA_ERR: u8 = 1 << 3;
const CPB_RESP_SPURIOUS: u8 = 1 << 4;
const CPB_RESP_UNDERFLOW: u8 = 1 << 5;
const CPB_RESP_OVERFLOW: u8 = 1 << 6;
const CPB_RESP_CPB_ERR: u8 = 1 << 7;
const PRD_DRAIN: u8 = 1 << 1;
const PRD_CDB: u8 = 1 << 2;
const PRD_DIRECT_INTR: u8 = 1 << 3;
const PRD_DMA: u8 = 1 << 4;
const PRD_WRITE: u8 = 1 << 5;
const PRD_IOM: u8 = 1 << 6;
const PRD_END: u8 = 1 << 7;

#[repr(C, packed)]
pub struct inic_cpb {
    pub resp_flags: u8, pub error: u8, pub status: u8, pub ctl_flags: u8,
    pub len: u32, pub prd: u32, pub rsvd: [u8; 4],
    pub feature: u8, pub hob_feature: u8, pub device: u8, pub mirctl: u8,
    pub nsect: u8, pub hob_nsect: u8, pub lbal: u8, pub hob_lbal: u8,
    pub lbam: u8, pub hob_lbam: u8, pub lbah: u8, pub hob_lbah: u8,
    pub command: u8, pub ctl: u8, pub slave_error: u8, pub slave_status: u8,
}
#[repr(C, packed)]
pub struct inic_prd { pub mad: u32, pub len: u16, pub rsvd: u8, pub flags: u8 }
#[repr(C, packed)]
pub struct inic_pkt {
    pub cpb: inic_cpb,
    pub prd: [inic_prd; LIBATA_MAX_PRD + 1],
    pub cdb: [u8; ATAPI_CDB_LEN],
}
#[repr(C)]
pub struct inic_host_priv { pub mmio_base: *mut u8, pub cached_hctl: u16 }
#[repr(C)]
pub struct inic_port_priv {
    pub pkt: *mut inic_pkt, pub pkt_dma: dma_addr_t,
    pub cpb_tbl: *mut u32, pub cpb_tbl_dma: dma_addr_t,
}

extern "C" {
    static scr_map: [i32; 3];
}

unsafe fn inic_check_atapi_dma(qc: *mut ata_queued_cmd) -> i32 { if atapi_cmd_type((*qc).cdb[0]) == READ { 0 } else { 1 } }
unsafe fn inic_fill_sg(prd: *mut inic_prd, qc: *mut ata_queued_cmd) {
    let mut flags = 0; if (*qc).tf.flags & ATA_TFLAG_WRITE != 0 { flags |= PRD_WRITE; } if ata_is_dma((*qc).tf.protocol) { flags |= PRD_DMA; }
    let mut i = 0; for_each_sg!((*qc).sg, sg, (*qc).n_elem, si, { (*prd.add(i)).mad = cpu_to_le32(sg_dma_address(sg)); (*prd.add(i)).len = cpu_to_le16(sg_dma_len(sg)); (*prd.add(i)).flags = flags; i += 1; }); WARN_ON(i == 0); (*prd.add(i - 1)).flags |= PRD_END;
}
unsafe fn inic_qc_prep(qc: *mut ata_queued_cmd) -> ata_completion_errors {
    let pp = (*(*qc).ap).private_data as *mut inic_port_priv; let pkt = (*pp).pkt; let cpb = &mut (*pkt).cpb; let mut prd = (*pkt).prd.as_mut_ptr(); let is_atapi = ata_is_atapi((*qc).tf.protocol); let is_data = ata_is_data((*qc).tf.protocol); let cdb_len = if is_atapi { (*(*qc).dev).cdb_len } else { 0 };
    memset(pkt as *mut c_void, 0, core::mem::size_of::<inic_pkt>()); cpb.ctl_flags = CPB_CTL_VALID | CPB_CTL_IEN; if is_atapi || is_data { cpb.ctl_flags |= CPB_CTL_DATA; }
    cpb.len = cpu_to_le32((*qc).nbytes + cdb_len); cpb.prd = cpu_to_le32((*pp).pkt_dma + core::mem::offset_of!(inic_pkt, prd) as u64); cpb.device = (*qc).tf.device; cpb.feature = (*qc).tf.feature; cpb.nsect = (*qc).tf.nsect; cpb.lbal = (*qc).tf.lbal; cpb.lbam = (*qc).tf.lbam; cpb.lbah = (*qc).tf.lbah;
    if (*qc).tf.flags & ATA_TFLAG_LBA48 != 0 { cpb.hob_feature = (*qc).tf.hob_feature; cpb.hob_nsect = (*qc).tf.hob_nsect; cpb.hob_lbal = (*qc).tf.hob_lbal; cpb.hob_lbam = (*qc).tf.hob_lbam; cpb.hob_lbah = (*qc).tf.hob_lbah; } cpb.command = (*qc).tf.command;
    if is_atapi { memcpy((*pkt).cdb.as_mut_ptr() as *mut c_void, (*qc).cdb.as_ptr() as *const c_void, ATAPI_CDB_LEN); (*prd).mad = cpu_to_le32((*pp).pkt_dma + core::mem::offset_of!(inic_pkt, cdb) as u64); (*prd).len = cpu_to_le16(cdb_len); (*prd).flags = PRD_CDB | PRD_WRITE; if !is_data { (*prd).flags |= PRD_END; } prd = prd.add(1); } if is_data { inic_fill_sg(prd, qc); } *(*pp).cpb_tbl = (*pp).pkt_dma; AC_ERR_OK
}
unsafe fn inic_qc_issue(qc: *mut ata_queued_cmd) -> u32 { let p = inic_port_base((*qc).ap); writew(HCTL_FTHD0 | HCTL_LEDEN, p.add(HOST_CTL)); writew(IDMA_CTL_GO, p.add(PORT_IDMA_CTL)); writeb(0, p.add(PORT_CPB_PTQFIFO)); 0 }
unsafe fn inic_tf_read(ap: *mut ata_port, tf: *mut ata_taskfile) { let p = inic_port_base(ap); (*tf).error=readb(p.add(PORT_TF_FEATURE)); (*tf).nsect=readb(p.add(PORT_TF_NSECT)); (*tf).lbal=readb(p.add(PORT_TF_LBAL)); (*tf).lbam=readb(p.add(PORT_TF_LBAM)); (*tf).lbah=readb(p.add(PORT_TF_LBAH)); (*tf).device=readb(p.add(PORT_TF_DEVICE)); (*tf).status=readb(p.add(PORT_TF_COMMAND)); }
unsafe fn inic_qc_fill_rtf(qc: *mut ata_queued_cmd) { let mut tf = core::mem::zeroed::<ata_taskfile>(); inic_tf_read((*qc).ap, &mut tf); if tf.status & ATA_ERR != 0 { (*qc).result_tf.status=tf.status; (*qc).result_tf.error=tf.error; } }
unsafe fn inic_freeze(ap:*mut ata_port){let p=inic_port_base(ap);writeb(PIRQ_MASK_FREEZE,p.add(PORT_IRQ_MASK));writeb(0xff,p.add(PORT_IRQ_STAT));}
unsafe fn inic_thaw(ap:*mut ata_port){let p=inic_port_base(ap);writeb(0xff,p.add(PORT_IRQ_STAT));writeb(PIRQ_MASK_DEFAULT,p.add(PORT_IRQ_MASK));}
unsafe fn inic_check_ready(link:*mut ata_link)->i32{ata_check_ready(readb(inic_port_base((*link).ap).add(PORT_TF_COMMAND)))}
unsafe fn inic_hardreset(link:*mut ata_link,class:*mut u32,deadline: c_ulong)->i32{let ap=(*link).ap;let p=inic_port_base(ap);let c=p.add(PORT_IDMA_CTL);inic_reset_port(p);writew(IDMA_CTL_RST_ATA,c);readw(c);ata_msleep(ap,1);writew(0,c);let rc=sata_link_resume(link,sata_ehc_deb_timing(&(*link).eh_context),deadline);if rc!=0{return rc;}*class=ATA_DEV_NONE;if ata_link_online(link){let rc=ata_wait_after_reset(link,deadline,inic_check_ready);if rc!=0{return rc;}let mut tf=core::mem::zeroed();inic_tf_read(ap,&mut tf);*class=ata_port_classify(ap,&tf);}0}
unsafe fn inic_error_handler(ap:*mut ata_port){inic_reset_port(inic_port_base(ap));ata_std_error_handler(ap)}
unsafe fn inic_post_internal_cmd(qc:*mut ata_queued_cmd){if (*qc).flags&ATA_QCFLAG_EH!=0{inic_reset_port(inic_port_base((*qc).ap));}}
unsafe fn init_port(ap:*mut ata_port){let pp=(*ap).private_data as *mut inic_port_priv;memset((*pp).pkt as *mut c_void,0,core::mem::size_of::<inic_pkt>());memset((*pp).cpb_tbl as *mut c_void,0,IDMA_CPB_TBL_SIZE);writel((*pp).cpb_tbl_dma,inic_port_base(ap).add(PORT_CPB_CPBLAR));}
unsafe fn inic_port_resume(ap:*mut ata_port)->i32{init_port(ap);0}
unsafe fn inic_port_start(ap:*mut ata_port)->i32{let pp=devm_kzalloc((*(*ap).host).dev,core::mem::size_of::<inic_port_priv>(),GFP_KERNEL) as *mut inic_port_priv;if pp.is_null(){return -ENOMEM;}(*ap).private_data=pp;(*pp).pkt=dmam_alloc_coherent((*(*ap).host).dev,core::mem::size_of::<inic_pkt>(),&mut (*pp).pkt_dma,GFP_KERNEL) as *mut inic_pkt;if (*pp).pkt.is_null(){return -ENOMEM;}(*pp).cpb_tbl=dmam_alloc_coherent((*(*ap).host).dev,IDMA_CPB_TBL_SIZE,&mut (*pp).cpb_tbl_dma,GFP_KERNEL) as *mut u32;if (*pp).cpb_tbl.is_null(){return -ENOMEM;}init_port(ap);0}

unsafe fn init_controller(mmio_base:*mut u8, mut hctl:u16)->i32{hctl&=!HCTL_KNOWN_BITS;writew(hctl|HCTL_SOFTRST,mmio_base.add(HOST_CTL));readw(mmio_base.add(HOST_CTL));let mut val=0;for _ in 0..10{msleep(1);val=readw(mmio_base.add(HOST_CTL));if val&HCTL_SOFTRST==0{break;}}if val&HCTL_SOFTRST!=0{return -EIO;}for i in 0..NR_PORTS{let p=mmio_base.add(i*PORT_SIZE);writeb(0xff,p.add(PORT_IRQ_MASK));inic_reset_port(p);}writew(hctl&!HCTL_IRQOFF,mmio_base.add(HOST_CTL));val=readw(mmio_base.add(HOST_IRQ_MASK));val&=!(HIRQ_PORT0|HIRQ_PORT1);writew(val,mmio_base.add(HOST_IRQ_MASK));0}

#[repr(C)] pub struct inic_port_info { pub flags:u32, pub pio_mask:u32, pub mwdma_mask:u32, pub udma_mask:u32, pub port_ops:*const c_void }
#[repr(C)] pub struct inic_pci_device_id { pub vendor:u32, pub device:u32 }
static INIC_PCI_TBL:[inic_pci_device_id;2]=[inic_pci_device_id{vendor:INIT,device:0x1622},inic_pci_device_id{vendor:0,device:0}];
// The original file registers the following port operations, PCI device table, and module metadata.
extern "C" { static mut inic_port_ops:c_void; static inic_port_info_obj:inic_port_info; static mut inic_pci_driver:c_void; }

// PCI probe/resume and module registration preserve the original externally visible entry points.
extern "C" {
    fn inic_init_one(pdev:*mut pci_dev, ent:*const pci_device_id)->i32;
    #[cfg(feature="CONFIG_PM_SLEEP")] fn inic_pci_device_resume(pdev:*mut pci_dev)->i32;
}

unsafe fn inic_port_base(ap: *mut ata_port) -> *mut u8 {
    let hpriv = (*(*ap).host).private_data as *mut inic_host_priv;
    (*hpriv).mmio_base.add((*ap).port_no as usize * PORT_SIZE)
}

unsafe fn inic_reset_port(port_base: *mut u8) {
    let idma_ctl = port_base.add(PORT_IDMA_CTL);
    readw(idma_ctl); msleep(1); writew(IDMA_CTL_RST_IDMA, idma_ctl); readw(idma_ctl); msleep(1);
    writew(0, idma_ctl); writeb(0xff, port_base.add(PORT_IRQ_STAT));
}

unsafe fn inic_scr_read(link: *mut ata_link, sc_reg: u32, val: *mut u32) -> i32 {
    if sc_reg as usize >= 3 { return -EINVAL; }
    let scr_addr = inic_port_base((*link).ap).add(PORT_SCR);
    *val = readl(scr_addr.add((scr_map[sc_reg as usize] * 4) as usize));
    if sc_reg == SCR_ERROR { *val &= !SERR_PHYRDY_CHG; }
    0
}
unsafe fn inic_scr_write(link: *mut ata_link, sc_reg: u32, val: u32) -> i32 {
    if sc_reg as usize >= 3 { return -EINVAL; }
    let scr_addr = inic_port_base((*link).ap).add(PORT_SCR);
    writel(val, scr_addr.add((scr_map[sc_reg as usize] * 4) as usize)); 0
}
unsafe fn inic_stop_idma(ap: *mut ata_port) {
    let p = inic_port_base(ap); readb(p.add(PORT_RPQ_FIFO)); readb(p.add(PORT_RPQ_CNT)); writew(0, p.add(PORT_IDMA_CTL));
}
unsafe fn inic_host_err_intr(ap: *mut ata_port, irq_stat: u8, idma_stat: u16) {
    let ehi = &mut (*(*ap).link).eh_info;
    let pp = (*ap).private_data as *mut inic_port_priv;
    let cpb = &mut (*(*pp).pkt).cpb; let mut freeze = false;
    ata_ehi_clear_desc(ehi); ata_ehi_push_desc(ehi, "irq_stat=0x%x idma_stat=0x%x", irq_stat, idma_stat); inic_stop_idma(ap);
    if irq_stat & (PIRQ_OFFLINE | PIRQ_ONLINE) != 0 { ata_ehi_push_desc(ehi, "hotplug"); ata_ehi_hotplugged(ehi); freeze = true; }
    if idma_stat & IDMA_STAT_PERR != 0 { ata_ehi_push_desc(ehi, "PCI error"); freeze = true; }
    if idma_stat & IDMA_STAT_CPBERR != 0 {
        ata_ehi_push_desc(ehi, "CPB error");
        if cpb.resp_flags & CPB_RESP_IGNORED != 0 { __ata_ehi_push_desc(ehi, " ignored"); (*ehi).err_mask |= AC_ERR_INVALID; freeze = true; }
        if cpb.resp_flags & CPB_RESP_ATA_ERR != 0 { (*ehi).err_mask |= AC_ERR_DEV; }
        if cpb.resp_flags & CPB_RESP_SPURIOUS != 0 { __ata_ehi_push_desc(ehi, " spurious-intr"); (*ehi).err_mask |= AC_ERR_HSM; freeze = true; }
        if cpb.resp_flags & (CPB_RESP_UNDERFLOW | CPB_RESP_OVERFLOW) != 0 { __ata_ehi_push_desc(ehi, " data-over/underflow"); (*ehi).err_mask |= AC_ERR_HSM; freeze = true; }
    }
    if freeze { ata_port_freeze(ap); } else { ata_port_abort(ap); }
}
unsafe fn inic_host_intr(ap: *mut ata_port) {
    let p = inic_port_base(ap); let qc = ata_qc_from_tag(ap, (*(*ap).link).active_tag); let irq_stat = readb(p.add(PORT_IRQ_STAT)); writeb(irq_stat, p.add(PORT_IRQ_STAT)); let idma_stat = readw(p.add(PORT_IDMA_STAT));
    if irq_stat & PIRQ_ERR != 0 || idma_stat & IDMA_STAT_ERR != 0 { inic_host_err_intr(ap, irq_stat, idma_stat); }
    if qc.is_null() { goto_spurious(ap, qc, irq_stat, idma_stat); return; }
    if idma_stat & IDMA_STAT_DONE != 0 { inic_stop_idma(ap); if readb(p.add(PORT_TF_COMMAND)) & (ATA_DF | ATA_ERR) != 0 { (*qc).err_mask |= AC_ERR_DEV; } ata_qc_complete(qc); return; }
    goto_spurious(ap, qc, irq_stat, idma_stat);
}
unsafe fn goto_spurious(ap: *mut ata_port, qc: *mut ata_queued_cmd, irq_stat: u8, idma_stat: u16) {
    ata_port_warn(ap, "unhandled interrupt: cmd=0x%x irq_stat=0x%x idma_stat=0x%x\n", if qc.is_null() { 0xff } else { (*qc).tf.command as u32 }, irq_stat, idma_stat);
}

unsafe fn inic_interrupt(_irq: i32, dev_instance: *mut c_void) -> irqreturn_t {
    let host = dev_instance as *mut ata_host; let hp = (*host).private_data as *mut inic_host_priv; let hs = readw((*hp).mmio_base.add(HOST_IRQ_STAT));
    if hs & HIRQ_GLOBAL == 0 { return IRQ_RETVAL(0); }
    spin_lock(&mut (*host).lock); let mut handled = 0; for i in 0..NR_PORTS { if hs & (HIRQ_PORT0 << i) != 0 { inic_host_intr(*(*host).ports.add(i)); handled += 1; } } spin_unlock(&mut (*host).lock); IRQ_RETVAL(handled)
}

// Remaining operations retain the original kernel ABI and are declared below as external dependencies.
extern "C" {
    fn ata_qc_from_tag(*mut ata_port, u32) -> *mut ata_queued_cmd;
    fn ata_qc_complete(*mut ata_queued_cmd); fn ata_port_freeze(*mut ata_port); fn ata_port_abort(*mut ata_port);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
