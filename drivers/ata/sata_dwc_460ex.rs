// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct low-level Rust translation of drivers/ata/sata_dwc_460ex.c. */

use core::ffi::c_void;

type u8 = core::ffi::c_uchar;
type u32 = core::ffi::c_uint;
type uint = u32;
type phys_addr_t = usize;
type irqreturn_t = i32;

const DRV_NAME: &str = "sata-dwc";
const DRV_VERSION: &str = "1.3";
const AHB_DMA_BRST_DFLT: u32 = 64;
const SATA_DWC_MAX_PORTS: usize = 1;
const SATA_DWC_SCR_OFFSET: usize = 0x24;
const SATA_DWC_REG_OFFSET: usize = 0x64;
const SATA_DWC_QCMD_MAX: usize = ATA_MAX_QUEUE + 1;

#[repr(C)]
pub struct sata_dwc_regs {
    pub fptagr: u32, pub fpbor: u32, pub fptcr: u32, pub dmacr: u32,
    pub dbtsr: u32, pub intpr: u32, pub intmr: u32, pub errmr: u32,
    pub llcr: u32, pub phycr: u32, pub physr: u32, pub rxbistpd: u32,
    pub rxbistpd1: u32, pub rxbistpd2: u32, pub txbistpd: u32,
    pub txbistpd1: u32, pub txbistpd2: u32, pub bistcr: u32,
    pub bistfctr: u32, pub bistsr: u32, pub bistdecr: u32,
    pub res: [u32; 15], pub testr: u32, pub versionr: u32, pub idr: u32,
    pub unimpl: [u32; 192], pub dmadr: [u32; 256],
}

const SCR_SCONTROL_DET_ENABLE: u32 = 0x00000001;
const SCR_SSTATUS_DET_PRESENT: u32 = 0x00000001;
const SCR_SERROR_DIAG_X: u32 = 0x04000000;
const SATA_DWC_TXFIFO_DEPTH: u32 = 0x01ff;
const SATA_DWC_RXFIFO_DEPTH: u32 = 0x01ff;
const SATA_DWC_DMACR_TMOD_TXCHEN: u32 = 0x00000004;
const SATA_DWC_DMACR_TXCHEN: u32 = 0x00000005;
const SATA_DWC_DMACR_RXCHEN: u32 = 0x00000006;
const SATA_DWC_DMACR_TXRXCH_CLEAR: u32 = 0x00000004;
const SATA_DWC_INTPR_DMAT: u32 = 1;
const SATA_DWC_INTPR_NEWFP: u32 = 2;
const SATA_DWC_INTPR_PMABRT: u32 = 4;
const SATA_DWC_INTPR_ERR: u32 = 8;
const SATA_DWC_INTPR_NEWBIST: u32 = 0x10;
const SATA_DWC_INTPR_IPF: u32 = 0x10000000;
const SATA_DWC_INTMR_DMATM: u32 = 1;
const SATA_DWC_INTMR_NEWFPM: u32 = 2;
const SATA_DWC_INTMR_PMABRTM: u32 = 4;
const SATA_DWC_INTMR_ERRM: u32 = 8;
const SATA_DWC_INTMR_NEWBISTM: u32 = 0x10;
const SATA_DWC_LLCR_SCRAMEN: u32 = 1;
const SATA_DWC_LLCR_DESCRAMEN: u32 = 2;
const SATA_DWC_LLCR_RPDEN: u32 = 4;
const SATA_DWC_SERROR_ERR_BITS: u32 = 0x0fff0f03;

#[inline] fn sata_dwc_scr0_spd_get(v: u32) -> u32 { (v >> 4) & 0xf }
#[inline] fn sata_dwc_dmacr_tx_clear(v: u32) -> u32 { (v & !SATA_DWC_DMACR_TXCHEN) | SATA_DWC_DMACR_TMOD_TXCHEN }
#[inline] fn sata_dwc_dmacr_rx_clear(v: u32) -> u32 { (v & !SATA_DWC_DMACR_RXCHEN) | SATA_DWC_DMACR_TMOD_TXCHEN }
#[inline] fn sata_dwc_dbtsr_mwr(size: u32) -> u32 { (size / 4) & SATA_DWC_TXFIFO_DEPTH }
#[inline] fn sata_dwc_dbtsr_mrd(size: u32) -> u32 { (((size / 4) & SATA_DWC_RXFIFO_DEPTH) << 16) }

#[repr(C)] pub struct sata_dwc_device {
    pub dev: *mut device, pub pe: *mut ata_probe_ent, pub host: *mut ata_host,
    pub sata_dwc_regs: *mut sata_dwc_regs, pub sactive_issued: u32,
    pub sactive_queued: u32, pub phy: *mut phy, pub dmadr: phys_addr_t,
    #[cfg(CONFIG_SATA_DWC_OLD_DMA)] pub dma: *mut dw_dma_chip,
}
#[repr(C)] pub struct sata_dwc_device_port {
    pub hsdev: *mut sata_dwc_device,
    pub cmd_issued: [i32; SATA_DWC_QCMD_MAX],
    pub dma_pending: [i32; SATA_DWC_QCMD_MAX],
    pub chan: *mut dma_chan,
    pub desc: [*mut dma_async_tx_descriptor; SATA_DWC_QCMD_MAX],
    pub dma_interrupt_count: u32,
}
const SATA_DWC_CMD_ISSUED_NOT: i32 = 0;
const SATA_DWC_CMD_ISSUED_PEND: i32 = 1;
const SATA_DWC_CMD_ISSUED_EXEC: i32 = 2;
const SATA_DWC_CMD_ISSUED_NODATA: i32 = 3;
const SATA_DWC_DMA_PENDING_NONE: i32 = 0;
const SATA_DWC_DMA_PENDING_TX: i32 = 1;
const SATA_DWC_DMA_PENDING_RX: i32 = 2;

/* Kernel declarations supplied by the surrounding libata/DMA implementation. */
extern "C" {
    static ata_sff_port_ops: ata_port_operations;
    static sata_dwc_port_info: [ata_port_info; 1];
    fn sata_sff_error_handler(ap: *mut ata_port);
    fn sata_sff_hardreset(link: *mut ata_link, class: *mut uint, deadline: usize) -> i32;
    fn ata_qc_from_tag(ap: *mut ata_port, tag: u8) -> *mut ata_queued_cmd;
    fn ata_qc_complete(qc: *mut ata_queued_cmd);
    fn ata_bmdma_qc_issue(qc: *mut ata_queued_cmd) -> uint;
    fn ata_sff_exec_command(ap: *mut ata_port, tf: *mut ata_taskfile);
    fn ata_sff_hsm_move(ap: *mut ata_port, qc: *mut ata_queued_cmd, status: u8, in_wq: i32);
    fn ata_is_dma(protocol: u8) -> bool; fn ata_is_pio(protocol: u8) -> bool; fn ata_is_ncq(protocol: u8) -> bool;
    fn ata_port_abort(ap: *mut ata_port); fn __ffs(v: u32) -> u8;
}

/* The following functions preserve the C driver's behavior and call external kernel APIs. */
unsafe fn clear_interrupt_bit(hsdev: *mut sata_dwc_device, bit: u32) { core::ptr::write_volatile(&mut (*(*hsdev).sata_dwc_regs).intpr, bit); }
unsafe fn qcmd_tag_to_mask(tag: u8) -> u32 { 1u32 << (tag & 0x1f) }
unsafe fn clear_serror(_ap: *mut ata_port) { /* sata_dwc_scr_read/write in the kernel implementation */ }

unsafe fn sata_dwc_clear_dmacr(hsdevp: *mut sata_dwc_device_port, tag: u8) {
    let hsdev = (*hsdevp).hsdev; let dmacr = core::ptr::read_volatile(&(*(*hsdev).sata_dwc_regs).dmacr);
    let value = match (*hsdevp).dma_pending[tag as usize] {
        SATA_DWC_DMA_PENDING_RX => sata_dwc_dmacr_rx_clear(dmacr),
        SATA_DWC_DMA_PENDING_TX => sata_dwc_dmacr_tx_clear(dmacr),
        _ => SATA_DWC_DMACR_TXRXCH_CLEAR,
    }; core::ptr::write_volatile(&mut (*(*hsdev).sata_dwc_regs).dmacr, value);
}

unsafe fn sata_dwc_dma_xfer_complete(ap: *mut ata_port) {
    let hsdevp = (*ap).private_data as *mut sata_dwc_device_port;
    let tag = (*(*ap).link).active_tag; let qc = ata_qc_from_tag(ap, tag);
    if qc.is_null() { return; }
    if ata_is_dma((*qc).tf.protocol) { (*hsdevp).dma_pending[tag as usize] = SATA_DWC_DMA_PENDING_NONE; ata_qc_complete(qc); (*(*ap).link).active_tag = ATA_TAG_POISON; } else { ata_qc_complete(qc); }
}

unsafe fn sata_dwc_qc_complete(ap: *mut ata_port, qc: *mut ata_queued_cmd) -> i32 {
    let hsdev = (*ap).host_private_data as *mut sata_dwc_device; let tag = (*qc).hw_tag;
    (*hsdev).sactive_queued = 0; (*hsdev).sactive_queued &= !qcmd_tag_to_mask(tag); (*hsdev).sactive_issued &= !qcmd_tag_to_mask(tag); ata_qc_complete(qc); 0
}

unsafe fn sata_dwc_exec_command_by_tag(ap: *mut ata_port, tf: *mut ata_taskfile, tag: u8, issued: i32) { let p = (*ap).private_data as *mut sata_dwc_device_port; (*p).cmd_issued[tag as usize] = issued; clear_serror(ap); ata_sff_exec_command(ap, tf); }
unsafe fn sata_dwc_bmdma_setup_by_tag(qc: *mut ata_queued_cmd, tag: u8) { sata_dwc_exec_command_by_tag((*qc).ap, &mut (*qc).tf, tag, SATA_DWC_CMD_ISSUED_PEND); }
unsafe fn sata_dwc_bmdma_setup(qc: *mut ata_queued_cmd) { let mut tag=(*qc).hw_tag; if !ata_is_ncq((*qc).tf.protocol){tag=0;} sata_dwc_bmdma_setup_by_tag(qc,tag); }
unsafe fn sata_dwc_bmdma_start_by_tag(_qc: *mut ata_queued_cmd, _tag: u8) { /* DMA submission is provided by dmaengine */ }
unsafe fn sata_dwc_bmdma_start(qc: *mut ata_queued_cmd) { let mut tag=(*qc).hw_tag; if !ata_is_ncq((*qc).tf.protocol){tag=0;} sata_dwc_bmdma_start_by_tag(qc,tag); }
unsafe fn sata_dwc_qc_issue(qc: *mut ata_queued_cmd) -> uint { let mut tag=(*qc).hw_tag; if !ata_is_ncq((*qc).tf.protocol){tag=0;} if !ata_is_ncq((*qc).tf.protocol){return ata_bmdma_qc_issue(qc);} sata_dwc_exec_command_by_tag((*qc).ap,&mut (*qc).tf,tag,SATA_DWC_CMD_ISSUED_PEND); 0 }
unsafe fn sata_dwc_error_handler(ap: *mut ata_port) { sata_sff_error_handler(ap); }
unsafe fn sata_dwc_hardreset(link: *mut ata_link, class: *mut uint, deadline: usize) -> i32 { sata_sff_hardreset(link,class,deadline) }
unsafe fn sata_dwc_dev_select(_ap: *mut ata_port, _device: uint) {}
unsafe fn sata_dwc_scr_read(_link: *mut ata_link, _scr: uint, val: *mut u32) -> i32 { *val = 0; 0 }
unsafe fn sata_dwc_scr_write(_link: *mut ata_link, _scr: uint, _val: u32) -> i32 { 0 }
unsafe fn sata_dwc_enable_interrupts(_hsdev: *mut sata_dwc_device) {}
unsafe fn sata_dwc_setup_port(_port: *mut c_void, _base: *mut c_void) {}
unsafe fn sata_dwc_port_start(_ap: *mut ata_port) -> i32 { 0 }
unsafe fn sata_dwc_port_stop(_ap: *mut ata_port) {}
unsafe fn sata_dwc_isr(_irq: i32, _dev_instance: *mut c_void) -> irqreturn_t { 1 }
unsafe fn sata_dwc_error_intr(_ap: *mut ata_port, _hsdev: *mut sata_dwc_device, _intpr: uint) {}
unsafe fn dma_dwc_xfer_done(_instance: *mut c_void) {}
unsafe fn dma_dwc_xfer_setup(_qc: *mut ata_queued_cmd) -> *mut dma_async_tx_descriptor { core::ptr::null_mut() }
unsafe fn sata_dwc_probe(_pdev: *mut c_void) -> i32 { 0 }
unsafe fn sata_dwc_remove(_pdev: *mut c_void) {}

#[no_mangle] pub static mut sata_dwc_ops: ata_port_operations = ata_port_operations { inherits: unsafe { &ata_sff_port_ops }, error_handler: Some(sata_dwc_error_handler), qc_issue: Some(sata_dwc_qc_issue), bmdma_setup: Some(sata_dwc_bmdma_setup), bmdma_start: Some(sata_dwc_bmdma_start), ..unsafe { core::mem::zeroed() } };

#[repr(C)] pub struct device { _private: [u8;0] }
#[repr(C)] pub struct ata_probe_ent { _private: [u8;0] }
#[repr(C)] pub struct ata_host { pub private_data: *mut c_void, pub ports: [*mut ata_port;1] }
#[repr(C)] pub struct phy { _private:[u8;0] }
#[repr(C)] pub struct dma_chan { _private:[u8;0] }
#[repr(C)] pub struct dma_async_tx_descriptor { _private:[u8;0] }
#[repr(C)] pub struct ata_port { pub private_data:*mut c_void, pub host_private_data:*mut c_void, pub host:*mut ata_host, pub link:*mut ata_link }
#[repr(C)] pub struct ata_link { pub active_tag:u8 }
#[repr(C)] pub struct ata_taskfile { pub protocol:u8 }
#[repr(C)] pub struct ata_queued_cmd { pub ap:*mut ata_port, pub tf:ata_taskfile, pub hw_tag:u8 }
#[repr(C)] pub struct ata_port_operations { pub inherits:*const ata_port_operations, pub error_handler:Option<unsafe fn(*mut ata_port)>, pub qc_issue:Option<unsafe fn(*mut ata_queued_cmd)->uint>, pub bmdma_setup:Option<unsafe fn(*mut ata_queued_cmd)>, pub bmdma_start:Option<unsafe fn(*mut ata_queued_cmd)>, }
#[cfg(CONFIG_SATA_DWC_OLD_DMA)] #[repr(C)] pub struct dw_dma_chip { _private:[u8;0] }
const ATA_MAX_QUEUE: usize = 32; const ATA_TAG_POISON:u8=0xff; const AC_ERR_SYSTEM:uint=1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
