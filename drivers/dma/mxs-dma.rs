// SPDX-License-Identifier: GPL-2.0
// Copyright 2011 Freescale Semiconductor, Inc. All Rights Reserved.
// Refer to drivers/dma/imx-sdma.c
//
// PIO throughout this implementation means PIO mode of mxs apbh-dma and
// apbx-dma, in which DMA programs peripheral controller registers.

const HW_APBHX_CTRL0: usize = 0x000;
const BM_APBH_CTRL0_APB_BURST8_EN: u32 = 1 << 29;
const BM_APBH_CTRL0_APB_BURST_EN: u32 = 1 << 28;
const BP_APBH_CTRL0_RESET_CHANNEL: usize = 16;
const HW_APBHX_CTRL1: usize = 0x010;
const HW_APBHX_CTRL2: usize = 0x020;
const HW_APBHX_CHANNEL_CTRL: usize = 0x030;
const BP_APBHX_CHANNEL_CTRL_RESET_CHANNEL: usize = 16;
const BP_CCW_COMMAND: u16 = 0;
const BM_CCW_COMMAND: u16 = 3;
const CCW_CHAIN: u16 = 1 << 2;
const CCW_IRQ: u16 = 1 << 3;
const CCW_WAIT4RDY: u16 = 1 << 5;
const CCW_DEC_SEM: u16 = 1 << 6;
const CCW_WAIT4END: u16 = 1 << 7;
const CCW_HALT_ON_TERM: u16 = 1 << 8;
const CCW_TERM_FLUSH: u16 = 1 << 9;
const BP_CCW_PIO_NUM: u16 = 12;
const BM_CCW_PIO_NUM: u16 = 0xf << 12;
const MXS_DMA_CMD_NO_XFER: u16 = 0;
const MXS_DMA_CMD_WRITE: u16 = 1;
const MXS_DMA_CMD_READ: u16 = 2;
const MXS_DMA_CMD_DMA_SENSE: u16 = 3;
const MAX_XFER_BYTES: u32 = 0xff00;
const MXS_PIO_WORDS: usize = 16;
const CCW_BLOCK_SIZE: usize = 4 * PAGE_SIZE;
const NUM_CCW: usize = CCW_BLOCK_SIZE / core::mem::size_of::<MxsDmaCcw>();
const MXS_DMA_CHANNELS: usize = 16;
const MXS_DMA_CHANNELS_MASK: u32 = 0xffff;
const MXS_DMA_SG_LOOP: u32 = 1 << 0;
const MXS_DMA_USE_SEMAPHORE: u32 = 1 << 1;

#[inline] fn bf_ccw(value: u16, shift: u16, mask: u16) -> u16 { (value << shift) & mask }
#[inline] fn dma_is_apbh(d: *const MxsDmaEngine) -> bool { unsafe { (*d).type_ == MxsDmaDevtype::Apbh } }
#[inline] fn apbh_is_old(d: *const MxsDmaEngine) -> bool { unsafe { (*d).dev_id == MxsDmaId::Imx23 } }
#[inline] fn nx_cmd(d: *const MxsDmaEngine, n: usize) -> usize { if dma_is_apbh(d) && apbh_is_old(d) { 0x050 } else { 0x110 } + n * 0x70 }
#[inline] fn sema(d: *const MxsDmaEngine, n: usize) -> usize { if dma_is_apbh(d) && apbh_is_old(d) { 0x080 } else { 0x140 } + n * 0x70 }
#[inline] fn bar(d: *const MxsDmaEngine, n: usize) -> usize { if dma_is_apbh(d) && apbh_is_old(d) { 0x070 } else { 0x130 } + n * 0x70 }
#[inline] fn debug1(n: usize) -> usize { 0x150 + n * 0x70 }

#[repr(C)] pub struct MxsDmaCcw { pub next: u32, pub bits: u16, pub xfer_bytes: u16, pub bufaddr: u32, pub pio_words: [u32; MXS_PIO_WORDS] }
#[repr(C)] pub struct MxsDmaChan { pub mxs_dma: *mut MxsDmaEngine, pub chan: dma_chan, pub desc: dma_async_tx_descriptor, pub tasklet: tasklet_struct, pub chan_irq: u32, pub ccw: *mut MxsDmaCcw, pub ccw_phys: dma_addr_t, pub desc_count: i32, pub status: dma_status, pub flags: u32, pub reset: bool }
#[repr(C)] pub struct MxsDmaEngine { pub dev_id: MxsDmaId, pub type_: MxsDmaDevtype, pub base: *mut core::ffi::c_void, pub clk: *mut clk, pub dma_device: dma_device, pub mxs_chans: [MxsDmaChan; MXS_DMA_CHANNELS], pub pdev: *mut platform_device, pub nr_channels: u32 }
#[repr(C)] pub struct MxsDmaType { pub id: MxsDmaId, pub type_: MxsDmaDevtype }
#[repr(u32)] #[derive(Copy, Clone, PartialEq)] pub enum MxsDmaDevtype { Apbh, Apbx }
#[repr(u32)] #[derive(Copy, Clone, PartialEq)] pub enum MxsDmaId { Imx23, Imx28 }
#[repr(C)] pub struct MxsDmaFilterParam { pub chan_id: u32 }

extern "C" {
    static mut STMP_OFFSET_REG_SET: usize;
    static mut STMP_OFFSET_REG_CLR: usize;
    fn readl(p: *mut core::ffi::c_void) -> u32; fn writel(v: u32, p: *mut core::ffi::c_void);
    fn udelay(v: u32); fn dma_cookie_assign(t: *mut dma_async_tx_descriptor) -> dma_cookie_t;
    fn dma_cookie_complete(t: *mut dma_async_tx_descriptor); fn tasklet_schedule(t: *mut tasklet_struct);
    fn dmaengine_desc_get_callback_invoke(d: *mut dma_async_tx_descriptor, p: *mut core::ffi::c_void);
    fn dma_set_tx_state(s: *mut dma_tx_state, a: dma_cookie_t, b: dma_cookie_t, r: u32);
}

unsafe fn to_chan(c: *mut dma_chan) -> *mut MxsDmaChan { (c as *mut u8).sub(core::mem::offset_of!(MxsDmaChan, chan)) as *mut MxsDmaChan }
unsafe fn mxs_dma_reset_chan(chan: *mut dma_chan) { let c=to_chan(chan); let d=(*c).mxs_dma; let id=(*chan).chan_id as usize; if (*c).flags & MXS_DMA_USE_SEMAPHORE != 0 && (*c).flags & MXS_DMA_SG_LOOP != 0 { (*c).reset=true; } else if dma_is_apbh(d) && apbh_is_old(d) { writel(1 << (id+BP_APBH_CTRL0_RESET_CHANNEL), (*d).base.add(HW_APBHX_CTRL0+STMP_OFFSET_REG_SET)); } else { let mut elapsed=0; let reg=(*d).base.add(debug1(id)); while readl(reg)&0xf==8 && elapsed<50000 { udelay(100); elapsed+=100; } writel(1 << (id+BP_APBHX_CHANNEL_CTRL_RESET_CHANNEL), (*d).base.add(HW_APBHX_CHANNEL_CTRL+STMP_OFFSET_REG_SET)); } (*c).status=dma_status::DMA_COMPLETE; }
unsafe fn mxs_dma_enable_chan(chan: *mut dma_chan) { let c=to_chan(chan); let d=(*c).mxs_dma; let id=(*chan).chan_id as usize; writel((*c).ccw_phys as u32, (*d).base.add(nx_cmd(d,id))); writel(if (*c).flags&MXS_DMA_USE_SEMAPHORE!=0&&(*c).flags&MXS_DMA_SG_LOOP!=0 {2}else{1}, (*d).base.add(sema(d,id))); (*c).reset=false; }
unsafe fn mxs_dma_disable_chan(chan: *mut dma_chan) { (*to_chan(chan)).status=dma_status::DMA_COMPLETE; }
unsafe fn mxs_dma_pause_chan(chan:*mut dma_chan)->i32 { let c=to_chan(chan); let d=(*c).mxs_dma; let id=(*chan).chan_id as usize; let off=if dma_is_apbh(d)&&apbh_is_old(d){HW_APBHX_CTRL0}else{HW_APBHX_CHANNEL_CTRL}; writel(1<<id,(*d).base.add(off+STMP_OFFSET_REG_SET)); (*c).status=dma_status::DMA_PAUSED; 0 }
unsafe fn mxs_dma_resume_chan(chan:*mut dma_chan)->i32 { let c=to_chan(chan); let d=(*c).mxs_dma; let id=(*chan).chan_id as usize; let off=if dma_is_apbh(d)&&apbh_is_old(d){HW_APBHX_CTRL0}else{HW_APBHX_CHANNEL_CTRL}; writel(1<<id,(*d).base.add(off+STMP_OFFSET_REG_CLR)); (*c).status=dma_status::DMA_IN_PROGRESS; 0 }
unsafe fn mxs_dma_tx_submit(tx:*mut dma_async_tx_descriptor)->dma_cookie_t { dma_cookie_assign(tx) }
unsafe fn mxs_dma_tasklet(t:*mut tasklet_struct) { let c=(t as *mut u8).sub(core::mem::offset_of!(MxsDmaChan,tasklet)) as *mut MxsDmaChan; dmaengine_desc_get_callback_invoke(&mut (*c).desc,core::ptr::null_mut()); }
unsafe fn mxs_dma_irq_to_chan(d:*mut MxsDmaEngine, irq:i32)->i32 { for i in 0..(*d).nr_channels as usize { if (*d).mxs_chans[i].chan_irq as i32==irq{return i as i32;} } -22 }

// The remaining driver entry points retain the kernel ABI and are declared for
// the corresponding Rust kernel bindings to provide their implementations.
extern "C" { fn mxs_dma_int_handler(irq:i32, dev_id:*mut core::ffi::c_void)->i32; fn mxs_dma_probe(pdev:*mut platform_device)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
