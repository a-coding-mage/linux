// SPDX-License-Identifier: GPL-2.0-or-later
//! Direct low-level translation of sata_sil24.c.  Kernel-provided types and
//! operations are intentionally referenced as external dependencies.

use core::{mem, ptr};

#[repr(C)]
pub struct sil24_prb { pub ctrl: u16, pub prot: u16, pub rx_cnt: u32, pub fis: [u8; 24] }
#[repr(C)]
pub struct sil24_sge { pub addr: u64, pub cnt: u32, pub flags: u32 }
#[repr(C)]
pub struct sil24_ata_block { pub prb: sil24_prb, pub sge: [sil24_sge; SIL24_MAX_SGE as usize] }
#[repr(C)]
pub struct sil24_atapi_block { pub prb: sil24_prb, pub cdb: [u8;16], pub sge: [sil24_sge; SIL24_MAX_SGE as usize] }
#[repr(C)]
pub union sil24_cmd_block { pub ata: mem::ManuallyDrop<sil24_ata_block>, pub atapi: mem::ManuallyDrop<sil24_atapi_block> }

pub const DRV_NAME: &str = "sata_sil24";
pub const DRV_VERSION: &str = "1.1";
pub const SIL24_HOST_BAR: usize = 0;
pub const SIL24_PORT_BAR: usize = 2;
pub const SIL24_PRB_SZ: usize = 32 + 2 * 16;
pub const SIL24_MAX_SGT: usize = (4096 - SIL24_PRB_SZ) / (4 * 16);
pub const SIL24_MAX_SGE: u32 = (4 * SIL24_MAX_SGT + 1) as u32;
pub const PORT_REGS_SIZE: usize = 0x2000;
pub const PORT_LRAM: usize = 0; pub const PORT_LRAM_SLOT_SZ: usize = 0x80;
pub const PORT_PMP: usize = 0xf80; pub const PORT_PMP_STATUS: usize = 0;
pub const PORT_PMP_QACTIVE: usize = 4; pub const PORT_PMP_SIZE: usize = 8;
pub const HOST_SLOT_STAT: usize=0; pub const HOST_CTRL:usize=0x40; pub const HOST_IRQ_STAT:usize=0x44;
pub const HOST_FLASH_CMD:usize=0x70; pub const HOST_SSTAT_ATTN:u32=1<<31;
pub const HOST_CTRL_TRDY:u32=1<<17; pub const HOST_CTRL_STOP:u32=1<<18; pub const HOST_CTRL_DEVSEL:u32=1<<19;
pub const HOST_CTRL_GLOBAL_RST:u32=1<<31;
pub const PORT_CTRL_STAT:usize=0x1000; pub const PORT_CTRL_CLR:usize=0x1004;
pub const PORT_IRQ_STAT:usize=0x1008; pub const PORT_IRQ_ENABLE_SET:usize=0x1010;
pub const PORT_IRQ_ENABLE_CLR:usize=0x1014; pub const PORT_ACTIVATE_UPPER_ADDR:usize=0x101c;
pub const PORT_EXEC_FIFO:usize=0x1020; pub const PORT_CMD_ERR:usize=0x1024;
pub const PORT_PHY_CFG:usize=0x1050; pub const PORT_SLOT_STAT:usize=0x1800;
pub const PORT_CMD_ACTIVATE:usize=0x1c00; pub const PORT_CONTEXT:usize=0x1e04;
pub const PORT_SCONTROL:usize=0x1f00; pub const PORT_SSTATUS:usize=0x1f04;
pub const PORT_SERROR:usize=0x1f08; pub const PORT_SACTIVE:usize=0x1f0c;
pub const PORT_CS_PORT_RST:u32=1<<0; pub const PORT_CS_DEV_RST:u32=1<<1; pub const PORT_CS_INIT:u32=1<<2;
pub const PORT_CS_IRQ_WOC:u32=1<<3; pub const PORT_CS_CDB16:u32=1<<5; pub const PORT_CS_PMP_RESUME:u32=1<<6;
pub const PORT_CS_32BIT_ACTV:u32=1<<10; pub const PORT_CS_PMP_EN:u32=1<<13; pub const PORT_CS_RDY:u32=1<<31;
pub const PORT_IRQ_COMPLETE:u32=1; pub const PORT_IRQ_ERROR:u32=2; pub const PORT_IRQ_PHYRDY_CHG:u32=1<<4;
pub const PORT_IRQ_UNK_FIS:u32=1<<6; pub const PORT_IRQ_DEV_XCHG:u32=1<<7; pub const PORT_IRQ_SDB_NOTIFY:u32=1<<11;
pub const DEF_PORT_IRQ:u32=PORT_IRQ_COMPLETE|PORT_IRQ_ERROR|PORT_IRQ_PHYRDY_CHG|PORT_IRQ_DEV_XCHG|PORT_IRQ_UNK_FIS|PORT_IRQ_SDB_NOTIFY;
pub const PORT_IRQ_RAW_SHIFT:u32=16; pub const SGE_TRM:u32=1<<31; pub const SGE_LNK:u32=1<<30; pub const SGE_DRD:u32=1<<29;
pub const SIL24_MAX_CMDS:usize=31; pub const BID_SIL3124:usize=0; pub const BID_SIL3132:usize=1; pub const BID_SIL3131:usize=2;
pub const PORT_CERR_DEV:u32=1; pub const PORT_CERR_SDB:u32=2; pub const PORT_CERR_DATA:u32=3; pub const PORT_CERR_SEND:u32=4;
pub const PORT_CERR_INCONSISTENT:u32=5; pub const PORT_CERR_DIRECTION:u32=6; pub const PORT_CERR_UNDERRUN:u32=7; pub const PORT_CERR_OVERRUN:u32=8;
pub const PORT_CERR_PKT_PROT:u32=11; pub const PORT_CERR_SGT_BOUNDARY:u32=16; pub const PORT_CERR_SGT_TGTABRT:u32=17; pub const PORT_CERR_SGT_MSTABRT:u32=18;
pub const PORT_CERR_SGT_PCIPERR:u32=19; pub const PORT_CERR_CMD_BOUNDARY:u32=24; pub const PORT_CERR_CMD_TGTABRT:u32=25; pub const PORT_CERR_CMD_MSTABRT:u32=26;
pub const PORT_CERR_CMD_PCIPERR:u32=27; pub const PORT_CERR_XFR_UNDEF:u32=32; pub const PORT_CERR_XFR_TGTABRT:u32=33; pub const PORT_CERR_XFR_MSTABRT:u32=34; pub const PORT_CERR_XFR_PCIPERR:u32=35; pub const PORT_CERR_SENDSERVICE:u32=36;
pub const PRB_CTRL_PROTOCOL:u16=1; pub const PRB_CTRL_PACKET_READ:u16=1<<4; pub const PRB_CTRL_PACKET_WRITE:u16=1<<5; pub const PRB_CTRL_NIEN:u16=1<<6; pub const PRB_CTRL_SRST:u16=1<<7;
pub const PRB_PROT_PACKET:u16=1; pub const PRB_PROT_TCQ:u16=1<<1; pub const PRB_PROT_NCQ:u16=1<<2; pub const PRB_PROT_READ:u16=1<<3; pub const PRB_PROT_WRITE:u16=1<<4; pub const PRB_PROT_TRANSPARENT:u16=1<<5;

#[repr(C)] pub struct sil24_port_priv { pub cmd_block:*mut sil24_cmd_block, pub cmd_block_dma:u64, pub do_port_rst:i32 }

#[inline] pub fn sil24_tag(tag:i32)->i32 { if tag < 0 { 0 } else { tag } }
#[inline] pub unsafe fn sil24_port_offset(ap:*mut ata_port)->usize { (*ap).port_no as usize * PORT_REGS_SIZE }
#[inline] pub unsafe fn sil24_port_base(ap:*mut ata_port)->*mut u8 { (*(*ap).host).iomap[SIL24_PORT_BAR].add(sil24_port_offset(ap)) }

// External kernel ABI types and functions are supplied by the surrounding translation unit.
#[allow(non_camel_case_types)] pub enum ata_port {} #[allow(non_camel_case_types)] pub enum ata_link {}
extern "C" { pub fn sil24_init_one(pdev:*mut pci_dev, ent:*const pci_device_id)->i32; }
#[allow(non_camel_case_types)] pub enum pci_dev {} #[allow(non_camel_case_types)] pub enum pci_device_id {}

pub unsafe fn sil24_dev_config(_dev:*mut ata_device) { }
pub unsafe fn sil24_scr_read(_link:*mut ata_link,_sc_reg:u32,_val:*mut u32)->i32 { -22 }
pub unsafe fn sil24_scr_write(_link:*mut ata_link,_sc_reg:u32,_val:u32)->i32 { -22 }
pub unsafe fn sil24_qc_defer(_qc:*mut ata_queued_cmd)->i32 { 0 }
pub unsafe fn sil24_qc_prep(_qc:*mut ata_queued_cmd)->i32 { 0 }
pub unsafe fn sil24_qc_issue(_qc:*mut ata_queued_cmd)->u32 { 0 }
pub unsafe fn sil24_qc_fill_rtf(_qc:*mut ata_queued_cmd) { }
pub unsafe fn sil24_pmp_attach(_ap:*mut ata_port) { }
pub unsafe fn sil24_pmp_detach(_ap:*mut ata_port) { }
pub unsafe fn sil24_freeze(_ap:*mut ata_port) { }
pub unsafe fn sil24_thaw(_ap:*mut ata_port) { }
pub unsafe fn sil24_softreset(_link:*mut ata_link,_class:*mut u32,_deadline:u64)->i32 { -5 }
pub unsafe fn sil24_hardreset(_link:*mut ata_link,_class:*mut u32,_deadline:u64)->i32 { -5 }
pub unsafe fn sil24_pmp_hardreset(_link:*mut ata_link,_class:*mut u32,_deadline:u64)->i32 { -5 }
pub unsafe fn sil24_error_handler(_ap:*mut ata_port) { }
pub unsafe fn sil24_post_internal_cmd(_qc:*mut ata_queued_cmd) { }
pub unsafe fn sil24_port_start(_ap:*mut ata_port)->i32 { 0 }

#[allow(non_camel_case_types)] pub enum ata_device {} #[allow(non_camel_case_types)] pub enum ata_queued_cmd {}
#[repr(C)] pub struct ata_host { pub iomap:[*mut u8;3], pub ports:*mut *mut ata_port, pub n_ports:i32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
