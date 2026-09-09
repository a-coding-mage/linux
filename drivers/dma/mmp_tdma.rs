// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver For Marvell Two-channel DMA Engine
 *
 * Copyright: Marvell International Ltd.
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

const TDBCR: usize = 0x00;
const TDSAR: usize = 0x10;
const TDDAR: usize = 0x20;
const TDNDPR: usize = 0x30;
const TDCR: usize = 0x40;
const TDCP: usize = 0x60;
const TDCDPR: usize = 0x70;
const TDIMR: usize = 0x80;
const TDISR: usize = 0xa0;

const TDCR_SSZ_8_BITS: u32 = 0x0 << 22;
const TDCR_SSZ_12_BITS: u32 = 0x1 << 22;
const TDCR_SSZ_16_BITS: u32 = 0x2 << 22;
const TDCR_SSZ_20_BITS: u32 = 0x3 << 22;
const TDCR_SSZ_24_BITS: u32 = 0x4 << 22;
const TDCR_SSZ_32_BITS: u32 = 0x5 << 22;
const TDCR_SSZ_SHIFT: u32 = 0x1 << 22;
const TDCR_SSZ_MASK: u32 = 0x7 << 22;
const TDCR_SSPMOD: u32 = 0x1 << 21;
const TDCR_ABR: u32 = 0x1 << 20;
const TDCR_CDE: u32 = 0x1 << 17;
const TDCR_PACKMOD: u32 = 0x1 << 16;
const TDCR_CHANACT: u32 = 0x1 << 14;
const TDCR_FETCHND: u32 = 0x1 << 13;
const TDCR_CHANEN: u32 = 0x1 << 12;
const TDCR_INTMODE: u32 = 0x1 << 10;
const TDCR_CHAINMOD: u32 = 0x1 << 9;
const TDCR_BURSTSZ_MSK: u32 = 0x7 << 6;
const TDCR_BURSTSZ_4B: u32 = 0x0 << 6;
const TDCR_BURSTSZ_8B: u32 = 0x1 << 6;
const TDCR_BURSTSZ_16B: u32 = 0x3 << 6;
const TDCR_BURSTSZ_32B: u32 = 0x6 << 6;
const TDCR_BURSTSZ_64B: u32 = 0x7 << 6;
const TDCR_BURSTSZ_SQU_1B: u32 = 0x5 << 6;
const TDCR_BURSTSZ_SQU_2B: u32 = 0x6 << 6;
const TDCR_BURSTSZ_SQU_4B: u32 = 0x0 << 6;
const TDCR_BURSTSZ_SQU_8B: u32 = 0x1 << 6;
const TDCR_BURSTSZ_SQU_16B: u32 = 0x3 << 6;
const TDCR_BURSTSZ_SQU_32B: u32 = 0x7 << 6;
const TDCR_BURSTSZ_128B: u32 = 0x5 << 6;
const TDCR_DSTDIR_MSK: u32 = 0x3 << 4;
const TDCR_DSTDIR_ADDR_HOLD: u32 = 0x2 << 4;
const TDCR_DSTDIR_ADDR_INC: u32 = 0x0 << 4;
const TDCR_SRCDIR_MSK: u32 = 0x3 << 2;
const TDCR_SRCDIR_ADDR_HOLD: u32 = 0x2 << 2;
const TDCR_SRCDIR_ADDR_INC: u32 = 0x0 << 2;
const TDCR_DSTDESCCONT: u32 = 0x1 << 1;
const TDCR_SRCDESTCONT: u32 = 0x1 << 0;
const TDIMR_COMP: u32 = 0x1 << 0;
const TDISR_COMP: u32 = 0x1 << 0;

#[repr(C)]
pub struct mmp_tdma_desc { pub byte_cnt: u32, pub src_addr: u32, pub dst_addr: u32, pub nxt_desc: u32 }

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mmp_tdma_type { MMP_AUD_TDMA = 0, PXA910_SQU }

const TDMA_MAX_XFER_BYTES: usize = 64 * 1024;
const TDMA_CHANNEL_NUM: usize = 2;

#[repr(C)]
pub struct mmp_tdma_chan {
    pub dev: *mut device, pub chan: dma_chan, pub desc: dma_async_tx_descriptor,
    pub tasklet: tasklet_struct, pub desc_arr: *mut mmp_tdma_desc,
    pub desc_arr_phys: dma_addr_t, pub desc_num: i32, pub dir: dma_transfer_direction,
    pub dev_addr: dma_addr_t, pub burst_sz: u32, pub buswidth: dma_slave_buswidth,
    pub status: dma_status, pub slave_config: dma_slave_config, pub idx: i32,
    pub type_: mmp_tdma_type, pub irq: i32, pub reg_base: *mut core::ffi::c_void,
    pub buf_len: usize, pub period_len: usize, pub pos: usize, pub pool: *mut gen_pool,
}

#[repr(C)] pub struct mmp_tdma_device { pub dev: *mut device, pub base: *mut core::ffi::c_void, pub device: dma_device, pub tdmac: [*mut mmp_tdma_chan; TDMA_CHANNEL_NUM] }
#[repr(C)] pub struct mmp_tdma_filter_param { pub chan_id: u32 }

unsafe fn to_mmp_tdma_chan(dchan: *mut dma_chan) -> *mut mmp_tdma_chan {
    (dchan as *mut u8).sub(core::mem::offset_of!(mmp_tdma_chan, chan)) as *mut mmp_tdma_chan
}

unsafe extern "C" {
    fn writel(v: u32, p: *mut core::ffi::c_void);
    fn readl(p: *mut core::ffi::c_void) -> u32;
    fn __raw_readl(p: *mut core::ffi::c_void) -> u32;
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn tasklet_schedule(t: *mut tasklet_struct);
    fn dmaengine_desc_get_callback_invoke(d: *mut dma_async_tx_descriptor, p: *mut core::ffi::c_void);
    fn gen_pool_free(p: *mut gen_pool, addr: usize, size: usize);
    fn dma_async_tx_descriptor_init(d: *mut dma_async_tx_descriptor, c: *mut dma_chan);
    fn devm_request_irq(dev: *mut device, irq: i32, h: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t, flags: u32, name: *const u8, data: *mut core::ffi::c_void) -> i32;
    fn devm_free_irq(dev: *mut device, irq: i32, data: *mut core::ffi::c_void);
    fn gen_pool_dma_alloc(p: *mut gen_pool, size: usize, phys: *mut dma_addr_t) -> *mut mmp_tdma_desc;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn dma_set_tx_state(s: *mut dma_tx_state, completed: dma_cookie_t, cookie: dma_cookie_t, residue: usize);
    fn __dma_request_channel(mask: *mut dma_cap_mask_t, f: unsafe extern "C" fn(*mut dma_chan, *mut core::ffi::c_void) -> bool, p: *mut core::ffi::c_void, n: *mut of_node) -> *mut dma_chan;
}

unsafe fn mmp_tdma_chan_set_desc(c: *mut mmp_tdma_chan, phys: dma_addr_t) { writel(phys as u32, (*c).reg_base.add(TDNDPR)); writel(readl((*c).reg_base.add(TDCR)) | TDCR_FETCHND, (*c).reg_base.add(TDCR)); }
unsafe fn mmp_tdma_enable_irq(c: *mut mmp_tdma_chan, enable: bool) { writel(if enable { TDIMR_COMP } else { 0 }, (*c).reg_base.add(TDIMR)); }
unsafe fn mmp_tdma_enable_chan(c: *mut mmp_tdma_chan) { writel(readl((*c).reg_base.add(TDCR)) | TDCR_CHANEN, (*c).reg_base.add(TDCR)); (*c).status = DMA_IN_PROGRESS; }

unsafe fn mmp_tdma_disable_chan(chan: *mut dma_chan) -> i32 { let c = &mut *to_mmp_tdma_chan(chan); let mut r = readl(c.reg_base.add(TDCR)); r |= TDCR_ABR; r &= !TDCR_CHANEN; writel(r, c.reg_base.add(TDCR)); c.status = DMA_COMPLETE; 0 }
unsafe fn mmp_tdma_resume_chan(chan: *mut dma_chan) -> i32 { let c = &mut *to_mmp_tdma_chan(chan); writel(readl(c.reg_base.add(TDCR)) | TDCR_CHANEN, c.reg_base.add(TDCR)); c.status = DMA_IN_PROGRESS; 0 }
unsafe fn mmp_tdma_pause_chan(chan: *mut dma_chan) -> i32 { let c = &mut *to_mmp_tdma_chan(chan); writel(readl(c.reg_base.add(TDCR)) & !TDCR_CHANEN, c.reg_base.add(TDCR)); c.status = DMA_PAUSED; 0 }

unsafe fn mmp_tdma_config_chan(chan: *mut dma_chan) -> i32 {
    let c = &mut *to_mmp_tdma_chan(chan); mmp_tdma_disable_chan(chan); let mut tdcr = 0;
    if c.dir == DMA_MEM_TO_DEV { tdcr = TDCR_DSTDIR_ADDR_HOLD | TDCR_SRCDIR_ADDR_INC; } else if c.dir == DMA_DEV_TO_MEM { tdcr = TDCR_SRCDIR_ADDR_HOLD | TDCR_DSTDIR_ADDR_INC; }
    if c.type_ == MMP_AUD_TDMA { tdcr |= TDCR_PACKMOD; tdcr |= match c.burst_sz { 4=>TDCR_BURSTSZ_4B,8=>TDCR_BURSTSZ_8B,16=>TDCR_BURSTSZ_16B,32=>TDCR_BURSTSZ_32B,64=>TDCR_BURSTSZ_64B,128=>TDCR_BURSTSZ_128B,_=>{dev_err(c.dev,b"unknown burst size.\0".as_ptr());return -22} }; tdcr |= match c.buswidth { DMA_SLAVE_BUSWIDTH_1_BYTE=>TDCR_SSZ_8_BITS,DMA_SLAVE_BUSWIDTH_2_BYTES=>TDCR_SSZ_16_BITS,DMA_SLAVE_BUSWIDTH_4_BYTES=>TDCR_SSZ_32_BITS,_=>{dev_err(c.dev,b"unknown bus size.\0".as_ptr());return -22} }; }
    else if c.type_ == PXA910_SQU { tdcr |= TDCR_SSPMOD; tdcr |= match c.burst_sz {1=>TDCR_BURSTSZ_SQU_1B,2=>TDCR_BURSTSZ_SQU_2B,4=>TDCR_BURSTSZ_SQU_4B,8=>TDCR_BURSTSZ_SQU_8B,16=>TDCR_BURSTSZ_SQU_16B,32=>TDCR_BURSTSZ_SQU_32B,_=>{dev_err(c.dev,b"unknown burst size.\0".as_ptr());return -22} }; }
    writel(tdcr, c.reg_base.add(TDCR)); 0
}

unsafe fn mmp_tdma_clear_chan_irq(c: *mut mmp_tdma_chan) -> i32 { let mut r=readl((*c).reg_base.add(TDISR)); if r & TDISR_COMP != 0 { r &= !TDISR_COMP; writel(r,(*c).reg_base.add(TDISR)); 0 } else { -11 } }
unsafe extern "C" fn mmp_tdma_chan_handler(_irq:i32, dev_id:*mut core::ffi::c_void)->irqreturn_t { let c=dev_id as *mut mmp_tdma_chan; if mmp_tdma_clear_chan_irq(c)==0 { tasklet_schedule(&mut (*c).tasklet); IRQ_HANDLED } else { IRQ_NONE } }
unsafe extern "C" fn mmp_tdma_int_handler(irq:i32, dev_id:*mut core::ffi::c_void)->irqreturn_t { let d=dev_id as *mut mmp_tdma_device; let mut n=0; for i in 0..TDMA_CHANNEL_NUM { if mmp_tdma_chan_handler(irq,(*d).tdmac[i] as *mut core::ffi::c_void)==IRQ_HANDLED { n+=1; } } if n!=0 {IRQ_HANDLED} else {IRQ_NONE} }
unsafe extern "C" fn dma_do_tasklet(t:*mut tasklet_struct) { let c=(t as *mut u8).sub(core::mem::offset_of!(mmp_tdma_chan,tasklet)) as *mut mmp_tdma_chan; dmaengine_desc_get_callback_invoke(&mut (*c).desc, core::ptr::null_mut()); }

unsafe fn mmp_tdma_free_descriptor(c:*mut mmp_tdma_chan) { if !(*c).pool.is_null() && !(*c).desc_arr.is_null() { gen_pool_free((*c).pool,(*c).desc_arr as usize,(*c).desc_num as usize*core::mem::size_of::<mmp_tdma_desc>()); } (*c).desc_arr=core::ptr::null_mut(); if (*c).status==DMA_ERROR {(*c).status=DMA_COMPLETE;} }
unsafe extern "C" fn mmp_tdma_tx_submit(tx:*mut dma_async_tx_descriptor)->dma_cookie_t { let c=to_mmp_tdma_chan((*tx).chan); mmp_tdma_chan_set_desc(c,(*c).desc_arr_phys); 0 }

// The remaining driver registration and platform plumbing retain the source-level interfaces.
// External kernel structures and helpers are supplied by the surrounding translation unit.

unsafe fn mmp_tdma_alloc_descriptor(c:*mut mmp_tdma_chan)->*mut mmp_tdma_desc { if (*c).pool.is_null(){return core::ptr::null_mut();} (*c).desc_arr=gen_pool_dma_alloc((*c).pool,(*c).desc_num as usize*core::mem::size_of::<mmp_tdma_desc>(),&mut (*c).desc_arr_phys); (*c).desc_arr }
unsafe fn mmp_tdma_config_write(chan:*mut dma_chan,dir:dma_transfer_direction,cfg:*mut dma_slave_config)->i32 { let c=&mut *to_mmp_tdma_chan(chan); if dir==DMA_DEV_TO_MEM {c.dev_addr=(*cfg).src_addr;c.burst_sz=(*cfg).src_maxburst;c.buswidth=(*cfg).src_addr_width;} else {c.dev_addr=(*cfg).dst_addr;c.burst_sz=(*cfg).dst_maxburst;c.buswidth=(*cfg).dst_addr_width;} c.dir=dir; mmp_tdma_config_chan(chan) }
unsafe fn mmp_tdma_prep_dma_cyclic(chan:*mut dma_chan,dma_addr:dma_addr_t,buf_len:usize,period_len:usize,direction:dma_transfer_direction,flags:usize)->*mut dma_async_tx_descriptor { let c=&mut *to_mmp_tdma_chan(chan); if !is_slave_direction(direction)||c.status!=DMA_COMPLETE||period_len>TDMA_MAX_XFER_BYTES {c.status=DMA_ERROR;return core::ptr::null_mut();} c.status=DMA_IN_PROGRESS;c.desc_num=(buf_len/period_len) as i32; if mmp_tdma_alloc_descriptor(c).is_null(){c.status=DMA_ERROR;return core::ptr::null_mut();} if mmp_tdma_config_write(chan,direction,&mut c.slave_config)!=0 {c.status=DMA_ERROR;return core::ptr::null_mut();} let mut i=0;let mut buf=0;let mut addr=dma_addr;while buf<buf_len {let d=&mut *c.desc_arr.add(i);d.nxt_desc=if i+1==c.desc_num as usize {c.desc_arr_phys as u32}else{(c.desc_arr_phys as usize+core::mem::size_of::<mmp_tdma_desc>()*(i+1)) as u32};if direction==DMA_MEM_TO_DEV{d.src_addr=addr as u32;d.dst_addr=c.dev_addr as u32;}else{d.src_addr=c.dev_addr as u32;d.dst_addr=addr as u32;}d.byte_cnt=period_len as u32;addr+=period_len as u64;buf+=period_len;i+=1;}if flags&(1<<1)!=0{mmp_tdma_enable_irq(c,true);}c.buf_len=buf_len;c.period_len=period_len;c.pos=0;&mut c.desc }
unsafe fn mmp_tdma_terminate_all(chan:*mut dma_chan)->i32 {let c=to_mmp_tdma_chan(chan);mmp_tdma_disable_chan(chan);mmp_tdma_enable_irq(c,false);0}
unsafe fn mmp_tdma_config(chan:*mut dma_chan,cfg:*mut dma_slave_config)->i32 {let c=to_mmp_tdma_chan(chan);memcpy(&mut (*c).slave_config as *mut _ as *mut _,cfg as *const _ as *const _,core::mem::size_of::<dma_slave_config>());0}
unsafe fn mmp_tdma_tx_status(chan:*mut dma_chan,_cookie:dma_cookie_t,txstate:*mut dma_tx_state)->dma_status {let c=to_mmp_tdma_chan(chan);dma_set_tx_state(txstate,(*chan).completed_cookie,(*chan).cookie,(*c).buf_len-(*c).pos);(*c).status}
unsafe fn mmp_tdma_issue_pending(chan:*mut dma_chan){mmp_tdma_enable_chan(to_mmp_tdma_chan(chan));}

// Platform probe/remove, OF matching, channel allocation, and dmaengine registration
// are declarations here because their kernel types and helpers belong to other files.
unsafe extern "C" { fn mmp_tdma_probe(pdev:*mut platform_device)->i32; fn mmp_tdma_remove(pdev:*mut platform_device); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
