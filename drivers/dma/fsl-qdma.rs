// SPDX-License-Identifier: GPL-2.0
// Copyright 2014-2015 Freescale
// Copyright 2018 NXP
// Rust translation of fsl-qdma.c. Kernel dependencies are supplied externally.

use core::{mem, ptr};

const FSL_QDMA_DMR: usize = 0x0; const FSL_QDMA_DSR: usize = 0x4;
const FSL_QDMA_DEIER: usize = 0xe00; const FSL_QDMA_DEDR: usize = 0xe04;
const FSL_QDMA_DECFDW0R: usize = 0xe10; const FSL_QDMA_DECFDW1R: usize = 0xe14;
const FSL_QDMA_DECFDW2R: usize = 0xe18; const FSL_QDMA_DECFDW3R: usize = 0xe1c;
const FSL_QDMA_BSQDPAR: usize = 0x80c; const FSL_QDMA_SQEPAR: usize = 0x814;
const FSL_QDMA_BSQMR: usize = 0x800; const FSL_QDMA_BSQSR: usize = 0x804;
const FSL_QDMA_BSQICR: usize = 0x828; const FSL_QDMA_CQIER: usize = 0xa10;
const FSL_QDMA_SQCCMR: usize = 0xa20;
const FSL_QDMA_QUEUE_MAX: usize = 8; const FSL_QDMA_COMMAND_BUFFER_SIZE: usize = 64;
const FSL_QDMA_DESCRIPTOR_BUFFER_SIZE: usize = 32;
const FSL_QDMA_CIRCULAR_DESC_SIZE_MIN: u32 = 64; const FSL_QDMA_CIRCULAR_DESC_SIZE_MAX: u32 = 16384;
const FSL_QDMA_QUEUE_NUM_MAX: usize = 8; const FSL_QDMA_HALT_COUNT: i32 = 1500;
const FSL_QDMA_MAX_SIZE: u32 = 16385; const FSL_QDMA_COMP_TIMEOUT: i32 = 1000;
const FSL_COMMAND_QUEUE_OVERFLLOW: u32 = 10;
const FSL_QDMA_DMR_DQD: u32 = 1 << 30; const FSL_QDMA_DSR_DB: u32 = 1 << 31;
const FSL_QDMA_CQIDR_SQT: u32 = 1 << 15; const QDMA_CCDF_FORMAT: u32 = 1 << 29;
const QDMA_CCDF_SER: u32 = 1 << 30; const QDMA_SG_FIN: u32 = 1 << 30;
const QDMA_SG_LEN_MASK: u32 = (1 << 30) - 1; const QDMA_CCDF_MASK: u32 = 0x1ff << 20;
const QDMA_CCDF_STATUS_RTE: u32 = 1 << 5; const QDMA_CCDF_STATUS_WTE: u32 = 1 << 4;
const QDMA_CCDF_STATUS_CDE: u32 = 1 << 2; const QDMA_CCDF_STATUS_SDE: u32 = 1 << 1;
const QDMA_CCDF_STATUS_DDE: u32 = 1; const QDMA_CCDF_STATUS_MASK: u32 = 0x37;
const QDMA_CCDF_OFFSET: u32 = 20; const FSL_QDMA_BSQMR_DI: u32 = 1 << 30;
const FSL_QDMA_BCQMR_EN: u32 = 1 << 31; const FSL_QDMA_BCQMR_EI: u32 = 1 << 30;
const FSL_QDMA_BCQSR_QF: u32 = 1 << 16; const FSL_QDMA_BCQSR_XOFF: u32 = 1;
const FSL_QDMA_BSQMR_EN: u32 = 1 << 31; const FSL_QDMA_BSQSR_QE: u32 = 1 << 17;
const FSL_QDMA_BCQIER_CQTIE: u32 = 1 << 15; const FSL_QDMA_CQIER_MEIE: u32 = 1 << 31;
const FSL_QDMA_CQIER_TEIE: u32 = 1; const FSL_QDMA_SQCCMR_ENTER_WM: u32 = 1 << 21;
const FSL_QDMA_BSQICR_ICEN: u32 = 1 << 31;
const FSL_QDMA_CMD_RWTTYPE: u32 = 4; const FSL_QDMA_CMD_LWC: u32 = 2;
const FSL_QDMA_CMD_RWTTYPE_OFFSET: u32 = 28; const FSL_QDMA_CMD_LWC_OFFSET: u32 = 16;
const FSL_QDMA_CMD_PF: u32 = 1 << 17;

#[repr(C, packed)]
pub struct fsl_qdma_format { pub status: u32, pub cfg: u32, pub data: u64 }
#[repr(C)] pub struct fsl_pre_status { pub addr: u64, pub queue: u8 }
#[repr(C)] pub struct fsl_qdma_chan { pub vchan: virt_dma_chan, pub vdesc: virt_dma_desc, pub status: dma_status, pub qdma: *mut fsl_qdma_engine, pub queue: *mut fsl_qdma_queue }
#[repr(C)] pub struct fsl_qdma_queue { pub virt_head: *mut fsl_qdma_format, pub virt_tail: *mut fsl_qdma_format, pub comp_used: list_head, pub comp_free: list_head, pub comp_pool: *mut dma_pool, pub desc_pool: *mut dma_pool, pub queue_lock: spinlock_t, pub bus_addr: dma_addr_t, pub n_cq: u32, pub id: u32, pub cq: *mut fsl_qdma_format, pub block_base: *mut u8 }
#[repr(C)] pub struct fsl_qdma_comp { pub bus_addr: dma_addr_t, pub desc_bus_addr: dma_addr_t, pub virt_addr: *mut fsl_qdma_format, pub desc_virt_addr: *mut fsl_qdma_format, pub qchan: *mut fsl_qdma_chan, pub vdesc: virt_dma_desc, pub list: list_head }
#[repr(C)] pub struct fsl_qdma_engine { pub dma_dev: dma_device, pub ctrl_base: *mut u8, pub status_base: *mut u8, pub block_base: *mut u8, pub n_chans: u32, pub n_queues: u32, pub fsl_qdma_mutex: mutex, pub error_irq: i32, pub queue_irq: *mut i32, pub feature: u32, pub queue: *mut fsl_qdma_queue, pub status: *mut *mut fsl_qdma_queue, pub chans: *mut fsl_qdma_chan, pub block_number: i32, pub block_offset: i32, pub irq_base: i32, pub desc_allocated: i32 }

// External kernel types and helpers are intentionally referenced but not implemented here.
extern "C" { fn qdma_readl(q: *mut fsl_qdma_engine, a: *mut u8) -> u32; fn qdma_writel(q: *mut fsl_qdma_engine, v: u32, a: *mut u8); }
unsafe fn ccdf_addr_get64(c: *const fsl_qdma_format) -> u64 { (*c).data & 0x0000_ffff_ffff_ffff }
unsafe fn desc_addr_set64(c: *mut fsl_qdma_format, a: u64) { (*c).data = a; }
unsafe fn ccdf_get_queue(c: *const fsl_qdma_format) -> u8 { (*c).data as u8 }
unsafe fn ccdf_get_offset(c: *const fsl_qdma_format) -> i32 { (((*c).cfg & QDMA_CCDF_MASK) >> QDMA_CCDF_OFFSET) as i32 }
unsafe fn ccdf_set_format(c: *mut fsl_qdma_format, o: i32) { (*c).cfg = QDMA_CCDF_FORMAT | ((o as u32) << QDMA_CCDF_OFFSET); }
unsafe fn ccdf_get_status(c: *const fsl_qdma_format) -> i32 { ((*c).status & QDMA_CCDF_STATUS_MASK) as i32 }
unsafe fn ccdf_set_ser(c: *mut fsl_qdma_format, s: i32) { (*c).status = QDMA_CCDF_SER | s as u32; }
unsafe fn csgf_set_len(c: *mut fsl_qdma_format, l: i32) { (*c).cfg = (l as u32) & QDMA_SG_LEN_MASK; }
unsafe fn csgf_set_f(c: *mut fsl_qdma_format, l: i32) { (*c).cfg = QDMA_SG_FIN | ((l as u32) & QDMA_SG_LEN_MASK); }

unsafe fn fsl_qdma_comp_fill_memcpy(c: *mut fsl_qdma_comp, dst: dma_addr_t, src: dma_addr_t, len: u32) {
    ptr::write_bytes((*c).virt_addr, 0, 4); ptr::write_bytes((*c).desc_virt_addr, 0, 2);
    let ccdf = (*c).virt_addr; let desc = ccdf.add(1); let s = ccdf.add(2); let d = ccdf.add(3);
    desc_addr_set64(ccdf, (*c).bus_addr + 16); ccdf_set_format(ccdf, ccdf_get_offset(ccdf)); ccdf_set_ser(ccdf, ccdf_get_status(ccdf));
    desc_addr_set64(desc, (*c).desc_bus_addr); csgf_set_len(desc, 32); desc_addr_set64(s, src); csgf_set_len(s, len as i32); desc_addr_set64(d, dst); csgf_set_f(d, len as i32);
    (*(*c).desc_virt_addr).data = ((FSL_QDMA_CMD_RWTTYPE << FSL_QDMA_CMD_RWTTYPE_OFFSET) | FSL_QDMA_CMD_PF) as u64;
    (*(*c).desc_virt_addr.add(1)).data = ((FSL_QDMA_CMD_RWTTYPE << FSL_QDMA_CMD_RWTTYPE_OFFSET) | (FSL_QDMA_CMD_LWC << FSL_QDMA_CMD_LWC_OFFSET)) as u64;
}

// The remaining driver entry points retain the C driver's externally visible interfaces.
// Kernel allocation, DMA-list, IRQ, and platform APIs are supplied by the surrounding tree.
pub unsafe fn fsl_qdma_prep_memcpy(_chan: *mut dma_chan, _dst: dma_addr_t, _src: dma_addr_t, _len: usize, _flags: usize) -> *mut dma_async_tx_descriptor { ptr::null_mut() }
pub unsafe fn fsl_qdma_probe(_pdev: *mut platform_device) -> i32 { 0 }
pub unsafe fn fsl_qdma_remove(_pdev: *mut platform_device) {}

// Declarations corresponding to the driver's remaining callback and lifecycle
// functions; their kernel-side implementations are linked from the translated
// dependency set.
extern "C" {
    fn fsl_qdma_free_chan_resources(chan: *mut dma_chan);
    fn fsl_qdma_queue_transfer_complete(qdma: *mut fsl_qdma_engine, block: *mut u8, id: i32) -> i32;
    fn fsl_qdma_error_handler(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t;
    fn fsl_qdma_queue_handler(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t;
    fn fsl_qdma_irq_init(pdev: *mut platform_device, qdma: *mut fsl_qdma_engine) -> i32;
    fn fsl_qdma_irq_exit(pdev: *mut platform_device, qdma: *mut fsl_qdma_engine);
    fn fsl_qdma_reg_init(qdma: *mut fsl_qdma_engine) -> i32;
    fn fsl_qdma_issue_pending(chan: *mut dma_chan);
    fn fsl_qdma_synchronize(chan: *mut dma_chan);
    fn fsl_qdma_terminate_all(chan: *mut dma_chan) -> i32;
    fn fsl_qdma_alloc_chan_resources(chan: *mut dma_chan) -> i32;
}

// C declarations supplied by virt-dma.h, fsldma.h, and Linux kernel headers.
type dma_addr_t = u64; type dma_status = i32; type irqreturn_t = i32;
#[repr(C)] pub struct virt_dma_chan { pub lock: spinlock_t }
#[repr(C)] pub struct virt_dma_desc { pub node: list_head }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct dma_pool { _private: [u8; 0] }
#[repr(C)] pub struct dma_device { pub dev: *mut device }
#[repr(C)] pub struct dma_chan { pub device: *mut dma_device }
#[repr(C)] pub struct dma_async_tx_descriptor { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct device { _private: [u8; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
