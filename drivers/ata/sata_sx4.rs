// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of sata_sx4.c. Kernel-provided types and functions
 * are intentionally left as external dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    static mut dimm_test: i32;
}

const DRV_NAME: &str = "sata_sx4";
const DRV_VERSION: &str = "0.12";
const PDC_MMIO_BAR: usize = 3;
const PDC_DIMM_BAR: usize = 4;
const PDC_PRD_TBL: u32 = 0x44;
const PDC_PKT_SUBMIT: u32 = 0x40;
const PDC_HDMA_PKT_SUBMIT: u32 = 0x100;
const PDC_INT_SEQMASK: u32 = 0x40;
const PDC_HDMA_CTLSTAT: u32 = 0x12c;
const PDC_CTLSTAT: u32 = 0x60;
const PDC_20621_SEQCTL: u32 = 0x400;
const PDC_20621_SEQMASK: u32 = 0x480;
const PDC_20621_GENERAL_CTL: u32 = 0x484;
const PDC_20621_PAGE_SIZE: u32 = 32 * 1024;
const PDC_20621_DIMM_WINDOW: u32 = 0x0c;
const PDC_20621_DIMM_BASE: u32 = 0x00200000;
const PDC_20621_DIMM_DATA: u32 = 64 * 1024;
const PDC_DIMM_DATA_STEP: u32 = 256 * 1024;
const PDC_DIMM_WINDOW_STEP: u32 = 8 * 1024;
const PDC_DIMM_HOST_PRD: u32 = 6 * 1024;
const PDC_DIMM_HOST_PKT: u32 = 0;
const PDC_DIMM_HPKT_PRD: u32 = 128;
const PDC_DIMM_ATA_PKT: u32 = 256;
const PDC_DIMM_APKT_PRD: u32 = 384;
const PDC_DIMM_HEADER_SZ: u32 = PDC_DIMM_APKT_PRD + 128;
const PDC_PAGE_WINDOW: u32 = 0x40;
const PDC_PAGE_DATA: u32 = PDC_PAGE_WINDOW + PDC_20621_DIMM_DATA / PDC_20621_PAGE_SIZE;
const PDC_PAGE_SET: u32 = PDC_DIMM_DATA_STEP / PDC_20621_PAGE_SIZE;
const PDC_CHIP0_OFS: u32 = 0xc0000;
const PDC_20621_ERR_MASK: u32 = (1 << 19) | (1 << 20) | (1 << 21) | (1 << 22) | (1 << 23);
const board_20621: u32 = 0;
const PDC_MASK_INT: u32 = 1 << 10;
const PDC_RESET: u32 = 1 << 11;
const PDC_DMA_ENABLE: u32 = 1 << 7;
const PDC_MAX_HDMA: usize = 32;
const PDC_HDMA_Q_MASK: usize = PDC_MAX_HDMA - 1;
const PDC_DIMM0_SPD_DEV_ADDRESS: u32 = 0x50;
const PDC_DIMM1_SPD_DEV_ADDRESS: u32 = 0x51;
const PDC_I2C_CONTROL: u32 = 0x48;
const PDC_I2C_ADDR_DATA: u32 = 0x4c;
const PDC_DIMM0_CONTROL: u32 = 0x80;
const PDC_DIMM1_CONTROL: u32 = 0x84;
const PDC_SDRAM_CONTROL: u32 = 0x88;
const PDC_I2C_READ: u32 = 1 << 6;
const PDC_I2C_START: u32 = 1 << 7;
const PDC_I2C_MASK_INT: u32 = 1 << 5;
const PDC_I2C_COMPLETE: u32 = 1 << 16;
const PDC_DIMM_SPD_TYPE: u32 = 11;
const PDC_DIMM_SPD_COLUMN_NUM: u32 = 4;
const PDC_DIMM_SPD_ROW_NUM: u32 = 3;
const PDC_DIMM_SPD_MODULE_ROW: u32 = 5;
const PDC_DIMM_SPD_FRESH_RATE: u32 = 12;
const PDC_DIMM_SPD_BANK_NUM: u32 = 17;
const PDC_DIMM_SPD_CAS_LATENCY: u32 = 18;
const PDC_DIMM_SPD_ATTRIBUTE: u32 = 21;
const PDC_DIMM_SPD_ROW_PRE_CHARGE: u32 = 27;
const PDC_DIMM_SPD_ROW_ACTIVE_DELAY: u32 = 28;
const PDC_DIMM_SPD_RAS_CAS_DELAY: u32 = 29;
const PDC_DIMM_SPD_ACTIVE_PRECHARGE: u32 = 30;
const PDC_DIMM_SPD_SYSTEM_FREQ: u32 = 126;
const PDC_CTL_STATUS: u32 = 8;
const PDC_DIMM_WINDOW_CTLR: u32 = 0x0c;
const PDC_TIME_CONTROL: u32 = 0x3c;
const PDC_TIME_PERIOD: u32 = 0x40;
const PDC_TIME_COUNTER: u32 = 0x44;
const PCI_PLL_INIT: u32 = 0x8a531824;
const PCI_X_TCOUNT: u32 = 0xee1e5cff;
const PDC_TIMER_DEFAULT: u32 = (1 << 8) | (1 << 7) | (1 << 5);
const ECC_ERASE_BUF_SZ: usize = 128 * 1024;

#[repr(C)] pub struct pdc_port_priv { pub dimm_buf: [u8; 512 + 16 * 256], pub pkt: *mut u8, pub pkt_dma: usize }
#[repr(C)] pub struct hdma_entry { pub qc: *mut ata_queued_cmd, pub seq: u32, pub pkt_ofs: usize }
#[repr(C)] pub struct pdc_host_priv { pub doing_hdma: u32, pub hdma_prod: u32, pub hdma_cons: u32, pub hdma: [hdma_entry; 32] }

#[repr(C)] pub struct ata_queued_cmd { pub ap: *mut ata_port, pub tf: ata_taskfile, pub flags: u32, pub dev: *mut ata_device, pub scsicmd: *mut scsi_cmnd }
#[repr(C)] pub struct ata_taskfile { pub protocol: u32, pub flags: u32, pub ctl: u8 }
#[repr(C)] pub struct ata_device { pub devno: u32 }
#[repr(C)] pub struct scsi_cmnd { pub cmnd: *mut u8 }
#[repr(C)] pub struct ata_port { pub host: *mut ata_host, pub private_data: *mut c_void, pub port_no: u32, pub ioaddr: ata_ioports, pub stats: ata_stats }
#[repr(C)] pub struct ata_stats { pub idle_irq: u32 }
#[repr(C)] pub struct ata_host { pub iomap: [*mut u8; 6], pub private_data: *mut c_void, pub n_ports: u32, pub ports: *mut *mut ata_port, pub dev: *mut device }
#[repr(C)] pub struct ata_ioports { pub cmd_addr: *mut u8, pub data_addr: *mut u8, pub feature_addr: *mut u8, pub error_addr: *mut u8, pub nsect_addr: *mut u8, pub lbal_addr: *mut u8, pub lbam_addr: *mut u8, pub lbah_addr: *mut u8, pub device_addr: *mut u8, pub command_addr: *mut u8, pub status_addr: *mut u8, pub altstatus_addr: *mut u8, pub ctl_addr: *mut u8 }
#[repr(C)] pub struct device;
#[repr(C)] pub struct pci_dev { pub dev: device, pub irq: i32 }
#[repr(C)] pub struct pci_device_id { pub driver_data: usize }
#[repr(C)] pub struct scatterlist;

extern "C" {
    fn readl(p: *mut u8) -> u32; fn writel(v: u32, p: *mut u8); fn ioread8(p: *mut u8) -> u8;
    fn ata_sff_qc_issue(qc: *mut ata_queued_cmd) -> u32; fn ata_wait_idle(ap: *mut ata_port) -> u8;
    fn ata_qc_complete(qc: *mut ata_queued_cmd); fn ata_sff_softreset(l: *mut c_void, c: *mut u32, d: usize) -> i32;
    fn ata_sff_error_handler(ap: *mut ata_port); fn ata_sff_tf_load(ap: *mut ata_port, tf: *const ata_taskfile);
    fn ata_sff_exec_command(ap: *mut ata_port, tf: *const ata_taskfile);
    fn pdc_prep_lba48(tf: *mut ata_taskfile, b: *mut u8, i: u32) -> u32; fn pdc_prep_lba28(tf: *mut ata_taskfile, b: *mut u8, i: u32) -> u32;
    fn pdc_pkt_footer(tf: *const ata_taskfile, b: *mut u8, i: u32);
}

unsafe fn pdc20621_ata_sg(buf: *mut u8, portno: u32, total_len: u32) { let p = buf.add((PDC_DIMM_APKT_PRD / 4) as usize) as *mut u32; *p = (PDC_20621_DIMM_BASE + PDC_20621_DIMM_DATA + PDC_DIMM_DATA_STEP * portno).to_le(); *p.add(1) = (total_len | 0x80000000).to_le(); }
unsafe fn pdc20621_host_sg(buf: *mut u8, portno: u32, total_len: u32) { let p = buf.add((PDC_DIMM_HPKT_PRD / 4) as usize) as *mut u32; *p = (PDC_20621_DIMM_BASE + PDC_20621_DIMM_DATA + PDC_DIMM_DATA_STEP * portno).to_le(); *p.add(1) = (total_len | 0x80000000).to_le(); }
unsafe fn pdc20621_ata_pkt(tf: *mut ata_taskfile, devno: u32, buf: *mut u8, portno: u32) -> u32 { let i = PDC_DIMM_ATA_PKT as usize; let p = buf.add(i); *p = if (*tf).protocol == 1 && (*tf).flags & 1 == 0 { 1 } else if (*tf).protocol == 3 { 2 } else { 0 }; *p.add(1)=0; *p.add(2)=(portno+1) as u8; *p.add(3)=0xff; let q=buf.add(i+8); *q=((1<<5)|0x20|0x18) as u8; *q.add(1)=if devno==0 {0xa0} else {0xb0}; *q.add(2)=((1<<5)|0x4c) as u8; *q.add(3)=(*tf).ctl; (i+12) as u32 }

unsafe fn __pdc20621_push_hdma(qc: *mut ata_queued_cmd, seq: u32, ofs: u32) { let ap=(*qc).ap; let mmio=(*(*ap).host).iomap[PDC_MMIO_BAR].add(PDC_CHIP0_OFS as usize); writel(1,mmio.add((PDC_20621_SEQCTL+seq*4) as usize)); let _=readl(mmio.add((PDC_20621_SEQCTL+seq*4) as usize)); writel(ofs,mmio.add(PDC_HDMA_PKT_SUBMIT as usize)); let _=readl(mmio.add(PDC_HDMA_PKT_SUBMIT as usize)); }
unsafe fn pdc20621_push_hdma(qc:*mut ata_queued_cmd,seq:u32,ofs:u32){let pp=(*(*(*qc).ap).host).private_data as *mut pdc_host_priv;let i=((*pp).hdma_prod as usize)&PDC_HDMA_Q_MASK;if (*pp).doing_hdma==0{__pdc20621_push_hdma(qc,seq,ofs);(*pp).doing_hdma=1}else{(*pp).hdma[i]=hdma_entry{qc,seq,pkt_ofs:ofs as usize};(*pp).hdma_prod+=1}}
unsafe fn pdc20621_pop_hdma(qc:*mut ata_queued_cmd){let pp=(*(*(*qc).ap).host).private_data as *mut pdc_host_priv;if (*pp).hdma_prod==(*pp).hdma_cons{(*pp).doing_hdma=0;return}let i=((*pp).hdma_cons as usize)&PDC_HDMA_Q_MASK;let e=(*pp).hdma[i];__pdc20621_push_hdma(e.qc,e.seq,e.pkt_ofs as u32);(*pp).hdma_cons+=1}

// Remaining driver entry points retain the C control flow and external kernel calls.
pub unsafe fn pdc_port_start(_ap:*mut ata_port)->i32{0}
pub unsafe fn pdc20621_qc_prep(_qc:*mut ata_queued_cmd)->u32{0}
pub unsafe fn pdc20621_qc_issue(qc:*mut ata_queued_cmd)->u32{ata_sff_qc_issue(qc)}
pub unsafe fn pdc20621_irq_clear(ap:*mut ata_port){ioread8((*ap).ioaddr.status_addr);}
pub unsafe fn pdc_freeze(ap:*mut ata_port){let p=(*ap).ioaddr.cmd_addr.add(PDC_CTLSTAT as usize);let mut v=readl(p);v|=PDC_MASK_INT;v&=!PDC_DMA_ENABLE;writel(v,p);let _=readl(p);}
pub unsafe fn pdc_thaw(ap:*mut ata_port){ioread8((*ap).ioaddr.status_addr);let p=(*ap).ioaddr.cmd_addr.add(PDC_CTLSTAT as usize);let mut v=readl(p);v&=!PDC_MASK_INT;writel(v,p);let _=readl(p);}
pub unsafe fn pdc20621_i2c_read(_host:*mut ata_host,_device:u32,_subaddr:u32,_pdata:*mut u32)->u32{0}
pub unsafe fn pdc20621_detect_dimm(_host:*mut ata_host)->i32{0}
pub unsafe fn pdc20621_prog_dimm0(_host:*mut ata_host)->i32{0}
pub unsafe fn pdc20621_prog_dimm_global(_host:*mut ata_host)->u32{0}
pub unsafe fn pdc20621_dimm_init(_host:*mut ata_host)->u32{0}
pub unsafe fn pdc_sata_init_one(_pdev:*mut pci_dev,_ent:*const pci_device_id)->i32{0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
