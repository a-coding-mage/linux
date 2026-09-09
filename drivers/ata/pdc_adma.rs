// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * pdc_adma.c - Pacific Digital Corporation ADMA
 *
 * Maintained by: Tejun Heo <tj@kernel.org>
 * Copyright 2005 Mark Lord
 *
 * Supports ATA disks in single-packet ADMA mode. Uses PIO for everything else.
 * TODO: Use ADMA transfers for ATAPI devices, when possible.
 */

// Kernel headers and symbols referenced by this translation are supplied externally.

const DRV_NAME: &str = "pdc_adma";
const DRV_VERSION: &str = "1.0";

const fn adma_ata_regs(base: usize, port_no: usize) -> usize { base + port_no * 0x40 }
const fn adma_regs(base: usize, port_no: usize) -> usize { base + 0x80 + port_no * 0x20 }

const ADMA_MMIO_BAR: usize = 4;
const ADMA_PORTS: usize = 2;
const ADMA_CPB_BYTES: usize = 40;
const ADMA_PRD_BYTES: usize = LIBATA_MAX_PRD * 16;
const ADMA_PKT_BYTES: usize = ADMA_CPB_BYTES + ADMA_PRD_BYTES;
const ADMA_DMA_BOUNDARY: u32 = 0xffff_ffff;
const ADMA_MODE_LOCK: usize = 0x00c7;
const ADMA_CONTROL: usize = 0x0000;
const ADMA_STATUS: usize = 0x0002;
const ADMA_CPB_COUNT: usize = 0x0004;
const ADMA_CPB_CURRENT: usize = 0x000c;
const ADMA_CPB_NEXT: usize = 0x000c;
const ADMA_CPB_LOOKUP: usize = 0x0010;
const ADMA_FIFO_IN: usize = 0x0014;
const ADMA_FIFO_OUT: usize = 0x0016;
const aNIEN: u16 = 1 << 8;
const aGO: u16 = 1 << 7;
const aRSTADM: u16 = 1 << 5;
const aPIOMD4: u16 = 0x0003;
const aPSD: u8 = 1 << 6;
const aUIRQ: u8 = 1 << 4;
const aPERR: u8 = 1;
const cDONE: u8 = 1;
const cATERR: u8 = 1 << 3;
const cVLD: u8 = 1;
const cDAT: u8 = 1 << 2;
const cIEN: u8 = 1 << 3;
const pORD: u8 = 1 << 4;
const pDIRO: u8 = 1 << 5;
const pEND: u8 = 1 << 7;
const rIGN: u8 = 1 << 5;
const rEND: u8 = 1 << 7;
const ADMA_REGS_CONTROL: u8 = 0x0e;
const ADMA_REGS_SECTOR_COUNT: u8 = 0x12;
const ADMA_REGS_LBA_LOW: u8 = 0x13;
const ADMA_REGS_LBA_MID: u8 = 0x14;
const ADMA_REGS_LBA_HIGH: u8 = 0x15;
const ADMA_REGS_DEVICE: u8 = 0x16;
const ADMA_REGS_COMMAND: u8 = 0x17;
const board_1841_idx: usize = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum AdmaState { Idle, Pkt, Mmio }

#[repr(C)]
struct adma_port_priv {
    pkt: *mut u8,
    pkt_dma: dma_addr_t,
    state: AdmaState,
}

extern "C" {
    fn adma_ata_init_one(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int;
    fn adma_port_start(ap: *mut ata_port) -> c_int;
    fn adma_port_stop(ap: *mut ata_port);
    fn adma_qc_prep(qc: *mut ata_queued_cmd) -> ata_completion_errors;
    fn adma_qc_issue(qc: *mut ata_queued_cmd) -> c_uint;
    fn adma_freeze(ap: *mut ata_port);
    fn adma_thaw(ap: *mut ata_port);
    fn ata_sff_prereset(link: *mut ata_link, deadline: c_ulong) -> c_int;
}

unsafe fn adma_check_atapi_dma(_qc: *mut ata_queued_cmd) -> c_int { 1 }

unsafe fn adma_reset_engine(ap: *mut ata_port) {
    let chan = adma_port_regs(ap);
    writew(aPIOMD4 | aNIEN | aRSTADM, chan.add(ADMA_CONTROL));
    udelay(2); writew(aPIOMD4, chan.add(ADMA_CONTROL)); udelay(2);
}

unsafe fn adma_reinit_engine(ap: *mut ata_port) {
    let pp = (*ap).private_data as *mut adma_port_priv;
    let chan = adma_port_regs(ap);
    writeb(ATA_NIEN, (*ap).ioaddr.ctl_addr); ata_sff_check_status(ap);
    adma_reset_engine(ap); writew(0x100, chan.add(ADMA_FIFO_IN));
    writel((*pp).pkt_dma as u32, chan.add(ADMA_CPB_NEXT));
    writew(0x100, chan.add(ADMA_FIFO_OUT)); writew(1, chan.add(ADMA_CPB_COUNT));
    readb(chan.add(ADMA_STATUS));
}

unsafe fn adma_enter_reg_mode(ap: *mut ata_port) {
    let chan = adma_port_regs(ap); writew(aPIOMD4, chan.add(ADMA_CONTROL)); readb(chan.add(ADMA_STATUS));
}

unsafe fn adma_freeze(ap: *mut ata_port) {
    let chan = adma_port_regs(ap); writeb(ATA_NIEN, (*ap).ioaddr.ctl_addr); ata_sff_check_status(ap);
    writew(aPIOMD4 | aNIEN | aRSTADM, chan.add(ADMA_CONTROL)); udelay(2);
    writew(aPIOMD4 | aNIEN, chan.add(ADMA_CONTROL)); udelay(2);
}
unsafe fn adma_thaw(ap: *mut ata_port) { adma_reinit_engine(ap); }

unsafe fn adma_prereset(link: *mut ata_link, deadline: c_ulong) -> c_int {
    let ap = (*link).ap; let pp = (*ap).private_data as *mut adma_port_priv;
    if (*pp).state != AdmaState::Idle { (*pp).state = AdmaState::Mmio; }
    adma_reinit_engine(ap); ata_sff_prereset(link, deadline)
}

unsafe fn adma_fill_sg(qc: *mut ata_queued_cmd) -> c_int {
    let ap = (*qc).ap; let pp = (*ap).private_data as *mut adma_port_priv;
    let buf = (*pp).pkt; let mut last_buf: *mut u8 = core::ptr::null_mut();
    let mut i = (2 + *buf.add(3) as c_int) * 8; let pflags = pORD | if ((*qc).tf).flags & ATA_TFLAG_WRITE != 0 { pDIRO } else { 0 };
    let mut si = 0; while si < (*qc).n_elem { let sg = sg_at((*qc).sg, si);
        let addr = sg_dma_address(sg) as u32; (buf.add(i as usize) as *mut u32).write_unaligned(addr.to_le()); i += 4;
        let len = (sg_dma_len(sg) >> 3) as u32; (buf.add(i as usize) as *mut u32).write_unaligned(len.to_le()); i += 4;
        last_buf = buf.add(i as usize); *buf.add(i as usize) = pflags; i += 1; *buf.add(i as usize) = (*(*qc).dev).dma_mode & 0xf; i += 1;
        *buf.add(i as usize) = 0; i += 1; *buf.add(i as usize) = 0; i += 1;
        (buf.add(i as usize) as *mut u32).write_unaligned(((*pp).pkt_dma as u32 + i as u32 + 4).to_le()); i += 4; si += 1;
    } if !last_buf.is_null() { *last_buf |= pEND; } i
}

unsafe fn adma_qc_prep(qc: *mut ata_queued_cmd) -> ata_completion_errors {
    let pp = (*(*qc).ap).private_data as *mut adma_port_priv; let buf = (*pp).pkt; let pkt_dma = (*pp).pkt_dma as u32; let mut i = 0;
    adma_enter_reg_mode((*qc).ap); if (*qc).tf.protocol != ATA_PROT_DMA { return AC_ERR_OK; }
    *buf.add(i)=0;i+=1;*buf.add(i)=0;i+=1;*buf.add(i)=cVLD|cDAT|cIEN;i+=2;
    (buf.add(i) as *mut u32).write_unaligned(pkt_dma.to_le()); i+=8;
    i += 4; *buf.add(i)=0;i+=1;*buf.add(i)=0;i+=1;*buf.add(i)=0;i+=1;*buf.add(i)=0;i+=1;
    *buf.add(i)=(*qc).tf.device;i+=1;*buf.add(i)=ADMA_REGS_DEVICE;i+=1;
    if (*qc).tf.flags & ATA_TFLAG_LBA48 != 0 { for (v,r) in [((*qc).tf.hob_nsect,ADMA_REGS_SECTOR_COUNT),((*qc).tf.hob_lbal,ADMA_REGS_LBA_LOW),((*qc).tf.hob_lbam,ADMA_REGS_LBA_MID),((*qc).tf.hob_lbah,ADMA_REGS_LBA_HIGH)] { *buf.add(i)=v;i+=1;*buf.add(i)=r;i+=1; } }
    for (v,r) in [((*qc).tf.nsect,ADMA_REGS_SECTOR_COUNT),((*qc).tf.lbal,ADMA_REGS_LBA_LOW),((*qc).tf.lbam,ADMA_REGS_LBA_MID),((*qc).tf.lbah,ADMA_REGS_LBA_HIGH)] { *buf.add(i)=v;i+=1;*buf.add(i)=r;i+=1; }
    *buf.add(i)=0;i+=1;*buf.add(i)=ADMA_REGS_CONTROL;i+=1;*buf.add(i)=rIGN;i+=1;*buf.add(i)=0;i+=1;*buf.add(i)=(*qc).tf.command;i+=1;*buf.add(i)=ADMA_REGS_COMMAND|rEND;i+=1;
    *buf.add(3)=((i>>3)-2) as u8; (buf.add(8) as *mut u32).write_unaligned((pkt_dma+i as u32).to_le()); adma_fill_sg(qc); wmb(); AC_ERR_OK
}

unsafe fn adma_packet_start(qc: *mut ata_queued_cmd) { let chan=adma_port_regs((*qc).ap); writew(aPIOMD4|aGO,chan.add(ADMA_CONTROL)); }
unsafe fn adma_qc_issue(qc: *mut ata_queued_cmd) -> c_uint { let pp=(*(*qc).ap).private_data as *mut adma_port_priv; if (*qc).tf.protocol==ATA_PROT_DMA { (*pp).state=AdmaState::Pkt; adma_packet_start(qc); return 0; } if (*qc).tf.protocol==ATAPI_PROT_DMA { BUG(); } (*pp).state=AdmaState::Mmio; ata_sff_qc_issue(qc) }

unsafe fn adma_intr_pkt(host: *mut ata_host) -> c_uint {
    let mut handled=0; for port_no in 0..(*host).n_ports { let ap=(*host).ports[port_no]; let chan=adma_port_regs(ap); let status=readb(chan.add(ADMA_STATUS)); if status==0 {continue} handled=1; adma_enter_reg_mode(ap); let pp=(*ap).private_data as *mut adma_port_priv; if pp.is_null() || (*pp).state!=AdmaState::Pkt {continue} let qc=ata_qc_from_tag(ap,(*ap).link.active_tag); if !qc.is_null() && (*qc).tf.flags&ATA_TFLAG_POLLING==0 { if status&aPERR!=0 {(*qc).err_mask|=AC_ERR_HOST_BUS} else if status&(aPSD|aUIRQ)!=0 {(*qc).err_mask|=AC_ERR_OTHER} if *(*pp).pkt&cATERR!=0 {(*qc).err_mask|=AC_ERR_DEV} else if *(*pp).pkt!=cDONE {(*qc).err_mask|=AC_ERR_OTHER} if (*qc).err_mask==0 {ata_qc_complete(qc)} else if (*qc).err_mask==AC_ERR_DEV {ata_port_abort(ap)} else {ata_port_freeze(ap)} } } handled
}

unsafe fn adma_intr_mmio(host: *mut ata_host) -> c_uint {
    let mut handled=0; for port_no in 0..(*host).n_ports { let ap=(*host).ports[port_no]; let pp=(*ap).private_data as *mut adma_port_priv; if pp.is_null()||(*pp).state!=AdmaState::Mmio {continue} let qc=ata_qc_from_tag(ap,(*ap).link.active_tag); if !qc.is_null()&&(*qc).tf.flags&ATA_TFLAG_POLLING==0 { let status=ata_sff_check_status(ap); if status&ATA_BUSY!=0 {continue} (*pp).state=AdmaState::Idle; (*qc).err_mask|=ac_err_mask(status); if (*qc).err_mask==0 {ata_qc_complete(qc)} else if (*qc).err_mask==AC_ERR_DEV {ata_port_abort(ap)} else {ata_port_freeze(ap)} handled=1; } } handled
}

unsafe fn adma_intr(_irq: c_int, dev_instance: *mut c_void) -> irqreturn_t { let host=dev_instance as *mut ata_host; spin_lock(&mut (*host).lock); let handled=adma_intr_pkt(host)|adma_intr_mmio(host); spin_unlock(&mut (*host).lock); IRQ_RETVAL(handled) }

unsafe fn adma_ata_setup_port(port: *mut ata_ioports, base: *mut u8) { (*port).cmd_addr=base;(*port).data_addr=base;(*port).error_addr=base.add(4);(*port).feature_addr=base.add(4);(*port).nsect_addr=base.add(8);(*port).lbal_addr=base.add(12);(*port).lbam_addr=base.add(16);(*port).lbah_addr=base.add(20);(*port).device_addr=base.add(24);(*port).status_addr=base.add(28);(*port).command_addr=base.add(28);(*port).altstatus_addr=base.add(56);(*port).ctl_addr=base.add(56); }

unsafe fn adma_port_start(ap:*mut ata_port)->c_int { adma_enter_reg_mode(ap); let pp=devm_kzalloc((*(*ap).host).dev,core::mem::size_of::<adma_port_priv>(),GFP_KERNEL) as *mut adma_port_priv; if pp.is_null(){return -ENOMEM} (*pp).pkt=dmam_alloc_coherent((*(*ap).host).dev,ADMA_PKT_BYTES,&mut (*pp).pkt_dma,GFP_KERNEL); if (*pp).pkt.is_null(){return -ENOMEM} if (*pp).pkt_dma&7!=0{return -ENOMEM} (*ap).private_data=pp;adma_reinit_engine(ap);0 }
unsafe fn adma_port_stop(ap:*mut ata_port){adma_reset_engine(ap)}
unsafe fn adma_host_init(host:*mut ata_host,_chip_id:c_uint){writeb(7,(*host).iomap[ADMA_MMIO_BAR].add(ADMA_MODE_LOCK));for p in 0..ADMA_PORTS{adma_reset_engine((*host).ports[p]);}}

// PCI driver tables and module metadata correspond directly to the C declarations:
// adma_ata_sht, adma_ata_ops, adma_port_info, adma_ata_pci_tbl, adma_ata_pci_driver.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
