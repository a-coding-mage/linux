// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver for Audio DMA Controller (ADMAC) on t8103 (M1) and other Apple chips
 *
 * Copyright (C) The Asahi Linux Contributors
 */

// Linux kernel dependencies are supplied by the surrounding translation.

const NCHANNELS_MAX: usize = 64;
const IRQ_NOUTPUTS: usize = 4;
const SRAM_BLOCK: u32 = 2048;

const RING_WRITE_SLOT: u32 = 0x3;
const RING_READ_SLOT: u32 = 0x30;
const RING_FULL: u32 = 1 << 9;
const RING_EMPTY: u32 = 1 << 8;
const RING_ERR: u32 = 1 << 10;
const STATUS_DESC_DONE: u32 = 1;
const STATUS_ERR: u32 = 1 << 6;
const FLAG_DESC_NOTIFY: u32 = 1 << 16;

const REG_TX_START: usize = 0x0000;
const REG_TX_STOP: usize = 0x0004;
const REG_RX_START: usize = 0x0008;
const REG_RX_STOP: usize = 0x000c;
const REG_IMPRINT: usize = 0x0090;
const REG_TX_SRAM_SIZE: usize = 0x0094;
const REG_RX_SRAM_SIZE: usize = 0x0098;
const REG_CHAN_CTL_RST_RINGS: u32 = 1;
const BUS_WIDTH_8BIT: u32 = 0x00;
const BUS_WIDTH_16BIT: u32 = 0x01;
const BUS_WIDTH_32BIT: u32 = 0x02;
const BUS_WIDTH_FRAME_2_WORDS: u32 = 0x10;
const BUS_WIDTH_FRAME_4_WORDS: u32 = 0x20;
const BUS_WIDTH_WORD_SIZE: u32 = 0xf;
const BUS_WIDTH_FRAME_SIZE: u32 = 0xf0;
const CHAN_SRAM_CARVEOUT_SIZE: u32 = 0xffff0000;
const CHAN_SRAM_CARVEOUT_BASE: u32 = 0xffff;
const CHAN_FIFOCTL_LIMIT: u32 = 0xffff0000;
const CHAN_FIFOCTL_THRESHOLD: u32 = 0xffff;

const fn chan_ctl(ch: usize) -> usize { 0x8000 + ch * 0x200 }
const fn desc_ring(ch: usize) -> usize { 0x8070 + ch * 0x200 }
const fn report_ring(ch: usize) -> usize { 0x8074 + ch * 0x200 }
const fn residue(ch: usize) -> usize { 0x8064 + ch * 0x200 }
const fn bus_width(ch: usize) -> usize { 0x8040 + ch * 0x200 }
const fn chan_sram_carveout(ch: usize) -> usize { 0x8050 + ch * 0x200 }
const fn chan_fifoctl(ch: usize) -> usize { 0x8054 + ch * 0x200 }
const fn desc_write(ch: usize) -> usize { 0x10000 + (ch / 2) * 4 + (ch & 1) * 0x4000 }
const fn report_read(ch: usize) -> usize { 0x10100 + (ch / 2) * 4 + (ch & 1) * 0x4000 }
const fn tx_intstate(i: usize) -> usize { 0x0030 + i * 4 }
const fn rx_intstate(i: usize) -> usize { 0x0040 + i * 4 }
const fn global_intstate(i: usize) -> usize { 0x0050 + i * 4 }
const fn chan_intstatus(ch: usize, i: usize) -> usize { 0x8010 + ch * 0x200 + i * 4 }
const fn chan_intmask(ch: usize, i: usize) -> usize { 0x8020 + ch * 0x200 + i * 4 }

#[repr(C)] pub struct admac_data { pub dma: dma_device, pub dev: *mut device, pub base: *mut core::ffi::c_void, pub rstc: *mut reset_control, pub cache_alloc_lock: mutex, pub txcache: admac_sram, pub rxcache: admac_sram, pub irq: i32, pub irq_index: i32, pub nchannels: i32, pub channels: [admac_chan; 0] }
#[repr(C)] pub struct admac_chan { pub no: u32, pub host: *mut admac_data, pub chan: dma_chan, pub tasklet: tasklet_struct, pub carveout: u32, pub lock: spinlock_t, pub current_tx: *mut admac_tx, pub nperiod_acks: i32, pub submitted: list_head, pub issued: list_head, pub to_free: list_head }
#[repr(C)] pub struct admac_sram { pub size: u32, pub allocated: u32 }
#[repr(C)] pub struct admac_tx { pub tx: dma_async_tx_descriptor, pub cyclic: bool, pub buf_addr: dma_addr_t, pub buf_end: dma_addr_t, pub buf_len: usize, pub period_len: usize, pub submitted_pos: usize, pub reclaimed_pos: usize, pub node: list_head }

extern "C" { fn readl_relaxed(p: *mut core::ffi::c_void) -> u32; fn writel_relaxed(v: u32, p: *mut core::ffi::c_void); fn mutex_lock(p: *mut mutex); fn mutex_unlock(p: *mut mutex); fn dma_cookie_assign(p: *mut dma_async_tx_descriptor) -> dma_cookie_t; fn dma_cookie_status(_: *mut dma_chan, _: dma_cookie_t, _: *mut dma_tx_state) -> dma_status; fn dma_set_residue(_: *mut dma_tx_state, _: usize); fn dmaengine_desc_callback_invoke(_: *mut dmaengine_desc_callback, _: *mut dmaengine_result); fn dmaengine_desc_get_callback(_: *mut dma_async_tx_descriptor, _: *mut dmaengine_desc_callback); }

#[inline] unsafe fn field_get(mask: u32, val: u32) -> u32 { (val & mask) >> mask.trailing_zeros() }
#[inline] unsafe fn field_prep(mask: u32, val: u32) -> u32 { (val << mask.trailing_zeros()) & mask }
#[inline] unsafe fn base(ad: *mut admac_data, off: usize) -> *mut core::ffi::c_void { ((*ad).base as *mut u8).add(off) as *mut _ }

unsafe fn admac_alloc_sram_carveout(ad: *mut admac_data, dir: dma_transfer_direction, out: *mut u32) -> i32 { (*ad).txcache.size=readl_relaxed(base(ad,REG_TX_SRAM_SIZE)); (*ad).rxcache.size=readl_relaxed(base(ad,REG_RX_SRAM_SIZE)); let s=if dir==DMA_MEM_TO_DEV { &mut (*ad).txcache } else { &mut (*ad).rxcache }; mutex_lock(&mut (*ad).cache_alloc_lock); let n=(s.size/SRAM_BLOCK) as i32; let mut i=0; while i<n && (s.allocated&(1u32<<i))!=0 {i+=1;} let ret=if i<n { *out=field_prep(CHAN_SRAM_CARVEOUT_BASE,(i as u32)*SRAM_BLOCK)|field_prep(CHAN_SRAM_CARVEOUT_SIZE,SRAM_BLOCK); s.allocated|=1u32<<i; 0 } else {-16}; mutex_unlock(&mut (*ad).cache_alloc_lock); ret }
unsafe fn admac_free_sram_carveout(ad:*mut admac_data,dir:dma_transfer_direction,carveout:u32){let s=if dir==DMA_MEM_TO_DEV{&mut(*ad).txcache}else{&mut(*ad).rxcache};let b=field_get(CHAN_SRAM_CARVEOUT_BASE,carveout);if b>=s.size{return;}mutex_lock(&mut(*ad).cache_alloc_lock);s.allocated&=!(1u32<<(b/SRAM_BLOCK));mutex_unlock(&mut(*ad).cache_alloc_lock);}
unsafe fn admac_modify(ad:*mut admac_data,reg:usize,mask:u32,val:u32){let p=base(ad,reg);let c=readl_relaxed(p);writel_relaxed((c&!mask)|(val&mask),p);}
unsafe fn admac_chan_direction(ch:i32)->dma_transfer_direction{if ch&1!=0{DMA_DEV_TO_MEM}else{DMA_MEM_TO_DEV}}
unsafe fn admac_cyclic_write_one_desc(ad:*mut admac_data,ch:i32,tx:*mut admac_tx){let addr=(*tx).buf_addr+((*tx).submitted_pos%(*tx).buf_len as u64);let p=base(ad,desc_write(ch as usize));writel_relaxed(addr as u32,p);writel_relaxed((addr>>32) as u32,p);writel_relaxed((*tx).period_len as u32,p);writel_relaxed(FLAG_DESC_NOTIFY,p);(*tx).submitted_pos=((*tx).submitted_pos+(*tx).period_len)%(2*(*tx).buf_len);}
unsafe fn admac_cyclic_write_desc(ad:*mut admac_data,ch:i32,tx:*mut admac_tx){for _ in 0..4{if readl_relaxed(base(ad,desc_ring(ch as usize)))&RING_FULL!=0{break;}admac_cyclic_write_one_desc(ad,ch,tx);}}
unsafe fn admac_ring_noccupied_slots(v:i32)->i32{let w=field_get(RING_WRITE_SLOT,v as u32) as i32;let r=field_get(RING_READ_SLOT,v as u32) as i32;if w!=r{(w+4-r)%4}else if v as u32&RING_FULL!=0{4}else{0}}
unsafe fn admac_cyclic_read_residue(ad:*mut admac_data,ch:i32,tx:*mut admac_tx)->u32{let r1=readl_relaxed(base(ad,report_ring(ch as usize)));let x1=readl_relaxed(base(ad,residue(ch as usize)));let r2=readl_relaxed(base(ad,report_ring(ch as usize)));let x2=readl_relaxed(base(ad,residue(ch as usize)));let n=if x2>x1{admac_ring_noccupied_slots(r1 as i32)+1}else{admac_ring_noccupied_slots(r2 as i32)};let pos=(*tx).reclaimed_pos+(*tx).period_len*(n as usize+1)-x2 as usize;((*tx).buf_len-pos%(*tx).buf_len) as u32}

// The remaining kernel callback wiring is preserved as declarations because its
// concrete Linux kernel types and helpers are supplied by the surrounding port.
extern "C" { fn admac_tx_submit(_: *mut dma_async_tx_descriptor)->dma_cookie_t; fn admac_desc_free(_: *mut dma_async_tx_descriptor)->i32; fn admac_prep_dma_cyclic(_: *mut dma_chan, _: dma_addr_t, _: usize, _: usize, _: dma_transfer_direction, _: usize)->*mut dma_async_tx_descriptor; fn admac_tx_status(_: *mut dma_chan, _: dma_cookie_t, _: *mut dma_tx_state)->dma_status; fn admac_issue_pending(_: *mut dma_chan); fn admac_pause(_: *mut dma_chan)->i32; fn admac_resume(_: *mut dma_chan)->i32; fn admac_terminate_all(_: *mut dma_chan)->i32; fn admac_synchronize(_: *mut dma_chan); fn admac_alloc_chan_resources(_: *mut dma_chan)->i32; fn admac_free_chan_resources(_: *mut dma_chan); fn admac_device_config(_: *mut dma_chan, _: *mut dma_slave_config)->i32; fn admac_probe(_: *mut platform_device)->i32; fn admac_remove(_: *mut platform_device); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
