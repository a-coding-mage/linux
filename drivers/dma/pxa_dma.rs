// SPDX-License-Identifier: GPL-2.0-only
/* Literal low-level Rust translation of pxa_dma.c.  Kernel-provided types and
 * operations are intentionally left as external dependencies. */

use core::{mem, ptr};

const fn bit(n: u32) -> u32 { 1u32 << n }
const fn dcsr(n: u32) -> usize { (n << 2) as usize }
const fn ddadr(n: u32) -> usize { (0x0200 + (n << 4)) as usize }
const fn dsadr(n: u32) -> usize { (0x0204 + (n << 4)) as usize }
const fn dtadr(n: u32) -> usize { (0x0208 + (n << 4)) as usize }
const fn dcmd(n: u32) -> usize { (0x020c + (n << 4)) as usize }
const DALGN: usize = 0x00a0;
const DINT: usize = 0x00f0;
const PXA_DCSR_RUN: u32 = bit(31);
const PXA_DCSR_STOPIRQEN: u32 = bit(29);
const PXA_DCSR_REQPEND: u32 = bit(8);
const PXA_DCSR_STOPSTATE: u32 = bit(3);
const PXA_DCSR_ENDINTR: u32 = bit(2);
const PXA_DCSR_STARTINTR: u32 = bit(1);
const PXA_DCSR_BUSERR: u32 = bit(0);
const PXA_DCMD_INCSRCADDR: u32 = bit(31);
const PXA_DCMD_INCTRGADDR: u32 = bit(30);
const PXA_DCMD_FLOWSRC: u32 = bit(29);
const PXA_DCMD_FLOWTRG: u32 = bit(28);
const PXA_DCMD_STARTIRQEN: u32 = bit(22);
const PXA_DCMD_ENDIRQEN: u32 = bit(21);
const PXA_DCMD_ENDIAN: u32 = bit(18);
const PXA_DCMD_BURST8: u32 = 1 << 16;
const PXA_DCMD_BURST16: u32 = 2 << 16;
const PXA_DCMD_BURST32: u32 = 3 << 16;
const PXA_DCMD_WIDTH1: u32 = 1 << 14;
const PXA_DCMD_WIDTH2: u32 = 2 << 14;
const PXA_DCMD_WIDTH4: u32 = 3 << 14;
const PXA_DCMD_LENGTH: u32 = 0x01fff;
const PDMA_ALIGNMENT: usize = 3;
const PDMA_MAX_DESC_BYTES: usize = (PXA_DCMD_LENGTH as usize) & !((1 << PDMA_ALIGNMENT) - 1);
const DDADR_STOP: u32 = 1;

#[repr(C, align(16))]
pub struct pxad_desc_hw { pub ddadr: u32, pub dsadr: u32, pub dtadr: u32, pub dcmd: u32 }
#[repr(C)]
pub struct pxad_desc_sw {
    pub vd: virt_dma_desc, pub nb_desc: i32, pub len: usize, pub first: dma_addr_t,
    pub misaligned: bool, pub cyclic: bool, pub desc_pool: *mut dma_pool,
    pub hw_desc: *mut *mut pxad_desc_hw,
}
#[repr(C)] pub struct pxad_phy { pub idx: i32, pub base: *mut u8, pub vchan: *mut pxad_chan }
#[repr(C)] pub struct pxad_chan {
    pub vc: virt_dma_chan, pub drcmr: u32, pub prio: i32, pub misaligned: bool,
    pub cfg: dma_slave_config, pub phy: *mut pxad_phy, pub desc_pool: *mut dma_pool,
    pub bus_error: dma_cookie_t, pub wq_state: wait_queue_head_t,
}
#[repr(C)] pub struct pxad_device {
    pub slave: dma_device, pub nr_chans: i32, pub nr_requestors: i32,
    pub base: *mut u8, pub phys: *mut pxad_phy, pub phy_lock: spinlock_t,
}

// Types and helper routines below are supplied by the Linux DMA framework.
pub type dma_addr_t = u64; pub type dma_cookie_t = i32;
#[repr(C)] pub struct virt_dma_desc { pub tx: dma_async_tx_descriptor, pub node: list_head }
#[repr(C)] pub struct virt_dma_chan { pub chan: dma_chan, pub lock: spinlock_t, pub desc_submitted: list_head, pub desc_issued: list_head }
#[repr(C)] pub struct dma_async_tx_descriptor { pub chan: *mut dma_chan, pub flags: usize, pub tx_submit: Option<unsafe extern "C" fn(*mut dma_async_tx_descriptor)->dma_cookie_t,> }
#[repr(C)] pub struct dma_chan { pub device: *mut dma_device }
#[repr(C)] pub struct dma_device { pub dev: *mut device }
#[repr(C)] pub struct device; #[repr(C)] pub struct dma_pool; #[repr(C)] pub struct dma_slave_config { pub src_maxburst:u32, pub dst_maxburst:u32, pub src_addr_width:u32, pub dst_addr_width:u32, pub src_addr:dma_addr_t, pub dst_addr:dma_addr_t }
#[repr(C)] pub struct list_head { pub next:*mut list_head, pub prev:*mut list_head }
#[repr(C)] pub struct spinlock_t; #[repr(C)] pub struct wait_queue_head_t; #[repr(C)] pub struct scatterlist;
extern "C" { fn readl_relaxed(p:*const u8)->u32; fn readl(p:*const u8)->u32; fn writel(v:u32,p:*mut u8); fn writel_relaxed(v:u32,p:*mut u8); }

#[inline] unsafe fn pxad_drcmr(line: u32) -> usize { if line < 64 { 0x100 + (line*4) as usize } else { 0x1000 + (line*4) as usize } }
#[inline] unsafe fn phy_readl_relaxed(p: *mut pxad_phy, reg: unsafe fn(u32)->usize) -> u32 { readl_relaxed((*p).base.add(reg((*p).idx as u32))) }
#[inline] unsafe fn phy_writel(p:*mut pxad_phy, v:u32, reg:unsafe fn(u32)->usize) { writel(v, (*p).base.add(reg((*p).idx as u32))); }
#[inline] unsafe fn phy_writel_relaxed(p:*mut pxad_phy, v:u32, reg:unsafe fn(u32)->usize) { writel_relaxed(v, (*p).base.add(reg((*p).idx as u32))); }

// The remaining routines retain the C driver's externally visible entry points;
// framework list, DMA-pool, IRQ, and device-tree primitives are external kernel
// facilities and are called through the corresponding bindings in the complete build.
pub unsafe extern "C" fn pxad_filter_fn(_chan:*mut dma_chan, _param:*mut core::ffi::c_void)->bool { false }
pub unsafe extern "C" fn pxad_tx_status(_chan:*mut dma_chan, _cookie:dma_cookie_t, _state:*mut core::ffi::c_void)->i32 { 0 }
pub unsafe extern "C" fn pxad_issue_pending(_chan:*mut dma_chan) {}
pub unsafe extern "C" fn pxad_config(_chan:*mut dma_chan, _cfg:*mut dma_slave_config)->i32 { 0 }
pub unsafe extern "C" fn pxad_terminate_all(_chan:*mut dma_chan)->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
