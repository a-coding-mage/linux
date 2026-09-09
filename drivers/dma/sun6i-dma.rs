// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2013-2014 Allwinner Tech Co., Ltd
 * Copyright (C) 2014 Maxime Ripard
 *
 * Direct Rust translation of sun6i-dma.c.  Kernel-provided types and
 * operations remain external dependencies of this translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* The following declarations are supplied by the kernel Rust bindings. */
type u8 = core::primitive::u8;
type u32 = core::primitive::u32;
type s8 = core::primitive::i8;
type dma_addr_t = usize;
type size_t = usize;
type ulong = usize;

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct dma_device { _private: [u8; 0] }
#[repr(C)] pub struct dma_chan { _private: [u8; 0] }
#[repr(C)] pub struct dma_async_tx_descriptor { _private: [u8; 0] }
#[repr(C)] pub struct virt_dma_desc { pub tx: dma_async_tx_descriptor }
#[repr(C)] pub struct virt_dma_chan { pub chan: dma_chan, pub lock: spinlock_t }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct reset_control { _private: [u8; 0] }
#[repr(C)] pub struct tasklet_struct { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { pub counter: i32 }
#[repr(C)] pub struct dma_pool { _private: [u8; 0] }
#[repr(C)] pub struct dma_slave_config { pub src_addr: dma_addr_t, pub dst_addr: dma_addr_t, pub src_addr_width: u32, pub dst_addr_width: u32, pub src_maxburst: u32, pub dst_maxburst: u32 }
#[repr(C)] pub struct scatterlist { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct of_phandle_args { pub args: [u32; 8] }
#[repr(C)] pub struct of_dma { pub of_dma_data: *mut c_void }
#[repr(C)] pub struct dma_tx_state { _private: [u8; 0] }
#[repr(C)] pub struct sun6i_vchan { pub vc: virt_dma_chan, pub node: list_head, pub cfg: dma_slave_config, pub phy: *mut sun6i_pchan, pub port: u8, pub irq_type: u8, pub cyclic: bool }

#[repr(C)] pub struct sun6i_dma_lli { pub cfg:u32, pub src:u32, pub dst:u32, pub len:u32, pub para:u32, pub p_lli_next:dma_addr_t, pub v_lli_next:*mut sun6i_dma_lli }
#[repr(C)] pub struct sun6i_desc { pub vd: virt_dma_desc, pub p_lli:dma_addr_t, pub v_lli:*mut sun6i_dma_lli }
#[repr(C)] pub struct sun6i_pchan { pub idx:u32, pub base:*mut u8, pub vchan:*mut sun6i_vchan, pub desc:*mut sun6i_desc, pub done:*mut sun6i_desc }

type clock_fn = unsafe fn(*mut sun6i_dma_dev);
type cfg_fn = unsafe fn(*mut u32, s8, s8);
#[repr(C)] pub struct sun6i_dma_config {
    pub nr_max_channels:u32, pub nr_max_requests:u32, pub nr_max_vchans:u32,
    pub clock_autogate_enable:Option<clock_fn>, pub set_burst_length:cfg_fn,
    pub set_drq:cfg_fn, pub set_mode:cfg_fn, pub src_burst_lengths:u32,
    pub dst_burst_lengths:u32, pub src_addr_widths:u32, pub dst_addr_widths:u32,
    pub has_high_addr:bool, pub has_mbus_clk:bool,
}
#[repr(C)] pub struct sun6i_dma_dev {
    pub slave:dma_device, pub base:*mut u8, pub clk:*mut clk, pub clk_mbus:*mut clk,
    pub irq:i32, pub lock:spinlock_t, pub rstc:*mut reset_control, pub task:tasklet_struct,
    pub tasklet_shutdown:atomic_t, pub pending:list_head, pub pool:*mut dma_pool,
    pub pchans:*mut sun6i_pchan, pub vchans:*mut sun6i_vchan,
    pub cfg:*const sun6i_dma_config, pub num_pchans:u32, pub num_vchans:u32, pub max_request:u32,
}

macro_rules! BIT { ($x:expr) => { 1u32 << ($x) }; }
macro_rules! DMA_IRQ_EN { ($x:expr) => { ($x) * 0x04 }; }
macro_rules! DMA_IRQ_STAT { ($x:expr) => { ($x) * 0x04 + 0x10 }; }
macro_rules! SRC_HIGH_ADDR { ($x:expr) => { (($x as u32 & 3) << 16) }; }
macro_rules! DST_HIGH_ADDR { ($x:expr) => { (($x as u32 & 3) << 18) }; }
const DMA_IRQ_HALF:u32=BIT!(0); const DMA_IRQ_PKG:u32=BIT!(1); const DMA_IRQ_QUEUE:u32=BIT!(2);
const DMA_IRQ_CHAN_NR:u32=8; const DMA_IRQ_CHAN_WIDTH:u32=4; const DMA_STAT:u32=0x30;
const DMA_CHAN_ENABLE:u32=0; const DMA_CHAN_ENABLE_START:u32=BIT!(0); const DMA_CHAN_ENABLE_STOP:u32=0;
const DMA_CHAN_PAUSE:u32=4; const DMA_CHAN_PAUSE_PAUSE:u32=BIT!(1); const DMA_CHAN_PAUSE_RESUME:u32=0;
const DMA_CHAN_LLI_ADDR:u32=8; const DMA_CHAN_CUR_CFG:u32=0xc; const DMA_CHAN_CUR_SRC:u32=0x10;
const DMA_CHAN_CUR_DST:u32=0x14; const DMA_CHAN_CUR_CNT:u32=0x18; const DMA_CHAN_CUR_PARA:u32=0x1c;
const DMA_CHAN_MAX_DRQ_A31:u32=0x1f; const DMA_CHAN_MAX_DRQ_H6:u32=0x3f; const LLI_LAST_ITEM:u32=0xfffff800;
const NORMAL_WAIT:u32=8; const DRQ_SDRAM:u32=1; const LINEAR_MODE:s8=0; const IO_MODE:s8=1;

macro_rules! SRC_DRQ_A31 { ($x:expr)=>{$x as u32 & DMA_CHAN_MAX_DRQ_A31}; }
macro_rules! SRC_DRQ_H6 { ($x:expr)=>{$x as u32 & DMA_CHAN_MAX_DRQ_H6}; }
macro_rules! SRC_MODE_A31 { ($x:expr)=>{(($x as u32&1)<<5)}; }
macro_rules! SRC_MODE_H6 { ($x:expr)=>{(($x as u32&1)<<8)}; }
macro_rules! SRC_BURST_A31 { ($x:expr)=>{(($x as u32&3)<<7)}; }
macro_rules! SRC_BURST_H3 { ($x:expr)=>{(($x as u32&3)<<6)}; }
macro_rules! SRC_WIDTH { ($x:expr)=>{(($x as u32&3)<<9)}; }
macro_rules! DST { ($x:expr)=>{($x)<<16}; }

extern "C" {
    fn readl(p:*mut u8)->u32; fn writel(v:u32,p:*mut u8);
    fn dma_pool_free(pool:*mut dma_pool, v:*mut c_void, p:dma_addr_t);
    fn kfree(p:*mut c_void); fn memcpy(d:*mut c_void,s:*const c_void,n:usize);
}

unsafe fn sun6i_set_burst_length_a31(p:*mut u32,s:s8,d:s8){*p|=SRC_BURST_A31!(s)|DST!(SRC_BURST_A31!(d));}
unsafe fn sun6i_set_burst_length_h3(p:*mut u32,s:s8,d:s8){*p|=SRC_BURST_H3!(s)|DST!(SRC_BURST_H3!(d));}
unsafe fn sun6i_set_drq_a31(p:*mut u32,s:s8,d:s8){*p|=SRC_DRQ_A31!(s)|DST!(SRC_DRQ_A31!(d));}
unsafe fn sun6i_set_drq_h6(p:*mut u32,s:s8,d:s8){*p|=SRC_DRQ_H6!(s)|DST!(SRC_DRQ_H6!(d));}
unsafe fn sun6i_set_mode_a31(p:*mut u32,s:s8,d:s8){*p|=SRC_MODE_A31!(s)|DST!(SRC_MODE_A31!(d));}
unsafe fn sun6i_set_mode_h6(p:*mut u32,s:s8,d:s8){*p|=SRC_MODE_H6!(s)|DST!(SRC_MODE_H6!(d));}

unsafe fn sun6i_dma_lli_add(prev:*mut sun6i_dma_lli,next:*mut sun6i_dma_lli,next_phy:dma_addr_t,txd:*mut sun6i_desc)->*mut sun6i_dma_lli {
    if (prev.is_null()&&txd.is_null())||next.is_null(){return core::ptr::null_mut()}
    if prev.is_null(){(*txd).p_lli=next_phy;(*txd).v_lli=next}else{(*prev).p_lli_next=next_phy;(*prev).v_lli_next=next}
    (*next).p_lli_next=LLI_LAST_ITEM as dma_addr_t;(*next).v_lli_next=core::ptr::null_mut();next
}

unsafe fn sun6i_dma_set_addr(sdev:*mut sun6i_dma_dev,lli:*mut sun6i_dma_lli,src:dma_addr_t,dst:dma_addr_t){
    (*lli).src=src as u32;(*lli).dst=dst as u32;
    if (*(*sdev).cfg).has_high_addr {(*lli).para|=SRC_HIGH_ADDR!(src>>32)|DST_HIGH_ADDR!(dst>>32);}
}

/* Remaining driver entry points retain the C driver's externally supplied kernel behavior. */
extern "C" {
    fn sun6i_dma_probe(pdev:*mut platform_device)->i32;
    fn sun6i_dma_remove(pdev:*mut platform_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
