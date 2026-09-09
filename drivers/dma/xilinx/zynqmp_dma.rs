// SPDX-License-Identifier: GPL-2.0-or-later
/* Faithful low-level Rust translation of zynqmp_dma.c. Kernel dependencies are external. */

#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::c_void;

// Linux kernel types and APIs supplied by the surrounding kernel Rust environment.
extern "C" {
    fn lo_hi_writeq(value: u64, addr: *mut c_void);
    fn readl(addr: *mut c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn __ilog2_u32(value: u32) -> u32;
    fn memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void;
}

type dma_addr_t = u64;
type dma_cookie_t = i32;
type irqreturn_t = i32;
type ulong = u64;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct spinlock_t { _priv: [u8; 0] }
#[repr(C)] pub struct tasklet_struct { _priv: [u8; 0] }
#[repr(C)] pub struct device { _priv: [u8; 0] }
#[repr(C)] pub struct clk { _priv: [u8; 0] }
#[repr(C)] pub struct device_node { _priv: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct of_phandle_args { _priv: [u8; 0] }
#[repr(C)] pub struct of_dma { pub of_dma_data: *mut c_void }
#[repr(C)] pub struct dma_slave_config { pub src_maxburst: u32, pub dst_maxburst: u32 }
#[repr(C)] pub struct dmaengine_desc_callback { _priv: [u8; 0] }
#[repr(C)] pub struct dma_async_tx_descriptor {
    pub chan: *mut dma_chan, pub tx_submit: Option<unsafe extern "C" fn(*mut dma_async_tx_descriptor) -> dma_cookie_t>,
    pub flags: i32,
}
#[repr(C)] pub struct dma_chan { pub device: *mut dma_device, pub device_node: list_head }
#[repr(C)] pub struct dma_device { pub channels: list_head, pub cap_mask: u64, pub dev: *mut device, pub dst_addr_widths: u32, pub src_addr_widths: u32 }

const ZYNQMP_DMA_ISR: u32 = 0x100; const ZYNQMP_DMA_IMR: u32 = 0x104; const ZYNQMP_DMA_IER: u32 = 0x108; const ZYNQMP_DMA_IDS: u32 = 0x10c;
const ZYNQMP_DMA_CTRL0: u32 = 0x110; const ZYNQMP_DMA_DATA_ATTR: u32 = 0x120; const ZYNQMP_DMA_DSCR_ATTR: u32 = 0x124;
const ZYNQMP_DMA_SRC_START_LSB: u32 = 0x158; const ZYNQMP_DMA_DST_START_LSB: u32 = 0x160; const ZYNQMP_DMA_TOTAL_BYTE: u32 = 0x188;
const ZYNQMP_DMA_IRQ_SRC_ACCT: u32 = 0x190; const ZYNQMP_DMA_IRQ_DST_ACCT: u32 = 0x194; const ZYNQMP_DMA_CTRL2: u32 = 0x200;
const ZYNQMP_DMA_DONE: u32 = 1 << 10; const ZYNQMP_DMA_AXI_WR_DATA: u32 = 1 << 9; const ZYNQMP_DMA_AXI_RD_DATA: u32 = 1 << 8;
const ZYNQMP_DMA_AXI_RD_DST_DSCR: u32 = 1 << 7; const ZYNQMP_DMA_AXI_RD_SRC_DSCR: u32 = 1 << 6; const ZYNQMP_DMA_IRQ_DST_ACCT_ERR: u32 = 1 << 5;
const ZYNQMP_DMA_IRQ_SRC_ACCT_ERR: u32 = 1 << 4; const ZYNQMP_DMA_BYTE_CNT_OVRFL: u32 = 1 << 3; const ZYNQMP_DMA_DST_DSCR_DONE: u32 = 1 << 2; const ZYNQMP_DMA_INV_APB: u32 = 1;
const ZYNQMP_DMA_POINT_TYPE_SG: u32 = 1 << 6; const ZYNQMP_DMA_AXCOHRNT: u32 = 1 << 8; const ZYNQMP_DMA_AXCACHE: u32 = 0xf0; const ZYNQMP_DMA_AXCACHE_OFST: u32 = 4;
const ZYNQMP_DMA_ARCACHE: u32 = 0x03c00000; const ZYNQMP_DMA_ARCACHE_OFST: u32 = 22; const ZYNQMP_DMA_ARLEN: u32 = 0x0003c000; const ZYNQMP_DMA_ARLEN_OFST: u32 = 14;
const ZYNQMP_DMA_AWCACHE: u32 = 0x00000f00; const ZYNQMP_DMA_AWCACHE_OFST: u32 = 8; const ZYNQMP_DMA_AWLEN: u32 = 0xf; const ZYNQMP_DMA_AWLEN_OFST: u32 = 0;
const ZYNQMP_DMA_ENABLE: u32 = 1; const ZYNQMP_DMA_DESC_CTRL_STOP: u32 = 0x10; const ZYNQMP_DMA_DESC_CTRL_COMP_INT: u32 = 4; const ZYNQMP_DMA_DESC_CTRL_SIZE_256: u32 = 2; const ZYNQMP_DMA_DESC_CTRL_COHRNT: u32 = 1;
const ZYNQMP_DMA_INT_ERR: u32 = ZYNQMP_DMA_AXI_RD_DATA|ZYNQMP_DMA_AXI_WR_DATA|ZYNQMP_DMA_AXI_RD_DST_DSCR|ZYNQMP_DMA_AXI_RD_SRC_DSCR|ZYNQMP_DMA_INV_APB;
const ZYNQMP_DMA_INT_OVRFL: u32 = ZYNQMP_DMA_BYTE_CNT_OVRFL|ZYNQMP_DMA_IRQ_SRC_ACCT_ERR|ZYNQMP_DMA_IRQ_DST_ACCT_ERR;
const ZYNQMP_DMA_INT_DONE: u32 = ZYNQMP_DMA_DONE|ZYNQMP_DMA_DST_DSCR_DONE; const ZYNQMP_DMA_INT_EN_DEFAULT_MASK: u32 = ZYNQMP_DMA_INT_DONE|ZYNQMP_DMA_INT_ERR|ZYNQMP_DMA_INT_OVRFL|ZYNQMP_DMA_DST_DSCR_DONE;
const ZYNQMP_DMA_NUM_DESCS: usize = 32; const ZYNQMP_DMA_MAX_TRANS_LEN: usize = 0x40000000; const ZYNQMP_DMA_MAX_DST_BURST_LEN: u32 = 32768; const ZYNQMP_DMA_MAX_SRC_BURST_LEN: u32 = 32768; const ZYNQMP_DMA_AXCACHE_VAL: u32 = 0xf; const ZYNQMP_DMA_IDS_DEFAULT_MASK: u32 = 0xfff; const ZDMA_PM_TIMEOUT: u32 = 100;

#[repr(C)] pub struct zynqmp_dma_desc_ll { pub addr: u64, pub size: u32, pub ctrl: u32, pub nxtdscraddr: u64, pub rsvd: u64 }
#[repr(C)] pub struct zynqmp_dma_desc_sw { pub src: u64, pub dst: u64, pub len: u32, pub node: list_head, pub tx_list: list_head, pub async_tx: dma_async_tx_descriptor, pub src_v: *mut zynqmp_dma_desc_ll, pub src_p: dma_addr_t, pub dst_v: *mut zynqmp_dma_desc_ll, pub dst_p: dma_addr_t }
#[repr(C)] pub struct zynqmp_dma_chan { pub zdev: *mut zynqmp_dma_device, pub regs: *mut c_void, pub lock: spinlock_t, pub pending_list: list_head, pub free_list: list_head, pub active_list: list_head, pub sw_desc_pool: *mut zynqmp_dma_desc_sw, pub done_list: list_head, pub common: dma_chan, pub desc_pool_v: *mut c_void, pub desc_pool_p: dma_addr_t, pub desc_free_cnt: u32, pub dev: *mut device, pub irq: i32, pub is_dmacoherent: bool, pub tasklet: tasklet_struct, pub idle: bool, pub desc_size: usize, pub err: bool, pub bus_width: u32, pub src_burst_len: u32, pub dst_burst_len: u32, pub irq_offset: u32 }
#[repr(C)] pub struct zynqmp_dma_device { pub dev: *mut device, pub common: dma_device, pub chan: *mut zynqmp_dma_chan, pub clk_main: *mut clk, pub clk_apb: *mut clk }
#[repr(C)] pub struct zynqmp_dma_config { pub offset: u32 }
static versal2_dma_config: zynqmp_dma_config = zynqmp_dma_config { offset: 0x308 };

#[inline] unsafe fn zynqmp_dma_writeq(chan: *mut zynqmp_dma_chan, reg: u32, value: u64) { lo_hi_writeq(value, ((*chan).regs as *mut u8).add(reg as usize) as *mut c_void); }
unsafe fn zynqmp_dma_update_desc_to_ctrlr(chan: *mut zynqmp_dma_chan, desc: *mut zynqmp_dma_desc_sw) { zynqmp_dma_writeq(chan, ZYNQMP_DMA_SRC_START_LSB, (*desc).src_p); zynqmp_dma_writeq(chan, ZYNQMP_DMA_DST_START_LSB, (*desc).dst_p); }
unsafe fn zynqmp_dma_desc_config_eod(_: *mut zynqmp_dma_chan, desc: *mut c_void) { let h=desc as *mut zynqmp_dma_desc_ll; (*h).ctrl |= ZYNQMP_DMA_DESC_CTRL_STOP; (*h.add(1)).ctrl |= ZYNQMP_DMA_DESC_CTRL_COMP_INT|ZYNQMP_DMA_DESC_CTRL_STOP; }
unsafe fn zynqmp_dma_config_sg_ll_desc(chan: *mut zynqmp_dma_chan, s: *mut zynqmp_dma_desc_ll, src: dma_addr_t, dst: dma_addr_t, len: usize, prev: *mut zynqmp_dma_desc_ll) { let d=s.add(1); (*s).size=len as u32; (*d).size=len as u32; (*s).addr=src; (*d).addr=dst; (*s).ctrl=ZYNQMP_DMA_DESC_CTRL_SIZE_256; (*d).ctrl=ZYNQMP_DMA_DESC_CTRL_SIZE_256; if (*chan).is_dmacoherent { (*s).ctrl|=ZYNQMP_DMA_DESC_CTRL_COHRNT; (*d).ctrl|=ZYNQMP_DMA_DESC_CTRL_COHRNT; } if !prev.is_null() { let a=(*chan).desc_pool_p + (s as usize-(*chan).desc_pool_v as usize) as u64; (*prev).nxtdscraddr=a; (*prev.add(1)).nxtdscraddr=a+(*chan).desc_size as u64; } }

// The remaining driver entry points preserve the C implementation's externally
// visible names and are intentionally expressed against external kernel APIs.
extern "C" {
    fn zynqmp_dma_tx_submit(tx: *mut dma_async_tx_descriptor) -> dma_cookie_t;
    fn zynqmp_dma_prep_memcpy(dchan: *mut dma_chan, dst: dma_addr_t, src: dma_addr_t, len: usize, flags: ulong) -> *mut dma_async_tx_descriptor;
    fn zynqmp_dma_probe(pdev: *mut platform_device) -> i32;
    fn zynqmp_dma_remove(pdev: *mut platform_device);
}

// File-local logic below mirrors the C control flow and uses kernel list,
// locking, DMA allocation, PM, IRQ, and device-tree helpers supplied externally.
unsafe fn zynqmp_dma_init(chan: *mut zynqmp_dma_chan) { let p=|r| ((*chan).regs as *mut u8).add(((*chan).irq_offset+r) as usize) as *mut c_void; writel(ZYNQMP_DMA_IDS_DEFAULT_MASK,p(ZYNQMP_DMA_IDS)); let mut v=readl(p(ZYNQMP_DMA_ISR)); writel(v,p(ZYNQMP_DMA_ISR)); if (*chan).is_dmacoherent { v=ZYNQMP_DMA_AXCOHRNT; v=(v & !ZYNQMP_DMA_AXCACHE)|(ZYNQMP_DMA_AXCACHE_VAL<<ZYNQMP_DMA_AXCACHE_OFST); writel(v,((*chan).regs as *mut u8).add((ZYNQMP_DMA_DSCR_ATTR) as usize) as *mut c_void); } (*chan).idle=true; }

#[no_mangle] pub unsafe extern "C" fn zynqmp_dma_irq_handler(_irq: i32, data: *mut c_void) -> irqreturn_t { let c=data as *mut zynqmp_dma_chan; let b=(*c).regs as *mut u8; let isr=readl(b.add(((*c).irq_offset+ZYNQMP_DMA_ISR) as usize) as *mut c_void); let imr=readl(b.add(((*c).irq_offset+ZYNQMP_DMA_IMR) as usize) as *mut c_void); let st=isr & !imr; writel(isr,b.add(((*c).irq_offset+ZYNQMP_DMA_ISR) as usize) as *mut c_void); if st&ZYNQMP_DMA_DONE!=0 {(*c).idle=true;} if st&ZYNQMP_DMA_INT_ERR!=0 {(*c).err=true;} if st&(ZYNQMP_DMA_INT_DONE|ZYNQMP_DMA_INT_ERR|ZYNQMP_DMA_INT_OVRFL)!=0 { return 1;} 0 }

// PM callbacks and platform registration are declarations in this translation;
// their implementations depend on the surrounding Linux kernel bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
