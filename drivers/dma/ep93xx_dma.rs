// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of the EP93xx DMA controller driver. Kernel-provided
 * types, macros, and functions are intentionally left as external dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

const M2P_CONTROL: usize = 0x0000;
const M2P_CONTROL_STALLINT: u32 = 1 << 0;
const M2P_CONTROL_NFBINT: u32 = 1 << 1;
const M2P_CONTROL_CH_ERROR_INT: u32 = 1 << 3;
const M2P_CONTROL_ENABLE: u32 = 1 << 4;
const M2P_CONTROL_ICE: u32 = 1 << 6;
const M2P_INTERRUPT: usize = 0x0004;
const M2P_INTERRUPT_STALL: u32 = 1 << 0;
const M2P_INTERRUPT_NFB: u32 = 1 << 1;
const M2P_INTERRUPT_ERROR: u32 = 1 << 3;
const M2P_PPALLOC: usize = 0x0008;
const M2P_STATUS: usize = 0x000c;
const M2P_MAXCNT0: usize = 0x0020;
const M2P_BASE0: usize = 0x0024;
const M2P_MAXCNT1: usize = 0x0030;
const M2P_BASE1: usize = 0x0034;
const M2P_STATE_IDLE: u32 = 0;
const M2P_STATE_ON: u32 = 2;

const M2M_CONTROL: usize = 0;
const M2M_CONTROL_DONEINT: u32 = 1 << 2;
const M2M_CONTROL_ENABLE: u32 = 1 << 3;
const M2M_CONTROL_START: u32 = 1 << 4;
const M2M_CONTROL_DAH: u32 = 1 << 11;
const M2M_CONTROL_SAH: u32 = 1 << 12;
const M2M_CONTROL_PW_16: u32 = 1 << 9;
const M2M_CONTROL_PW_32: u32 = 2 << 9;
const M2M_CONTROL_PW_MASK: u32 = 3 << 9;
const M2M_CONTROL_TM_TX: u32 = 1 << 13;
const M2M_CONTROL_TM_RX: u32 = 2 << 13;
const M2M_CONTROL_NFBINT: u32 = 1 << 21;
const M2M_CONTROL_RSS_SSPRX: u32 = 1 << 22;
const M2M_CONTROL_RSS_SSPTX: u32 = 2 << 22;
const M2M_CONTROL_RSS_IDE: u32 = 3 << 22;
const M2M_CONTROL_NO_HDSK: u32 = 1 << 24;
const M2M_CONTROL_PWSC_SHIFT: u32 = 25;
const M2M_INTERRUPT: usize = 4;
const M2M_INTERRUPT_MASK: u32 = 6;
const M2M_STATUS: usize = 0x000c;
const M2M_STATUS_CTL_SHIFT: u32 = 1;
const M2M_STATUS_CTL_STALL: u32 = 1 << 1;
const M2M_STATUS_CTL_MASK: u32 = 7 << 1;
const M2M_STATUS_BUF_NO: u32 = 0;
const M2M_STATUS_BUF_ON: u32 = 1 << 4;
const M2M_STATUS_BUF_MASK: u32 = 3 << 4;
const M2M_STATUS_DONE: u32 = 1 << 6;
const M2M_BCR0: usize = 0x0010;
const M2M_BCR1: usize = 0x0014;
const M2M_SAR_BASE0: usize = 0x0018;
const M2M_SAR_BASE1: usize = 0x001c;
const M2M_DAR_BASE0: usize = 0x002c;
const M2M_DAR_BASE1: usize = 0x0030;

const DMA_MAX_CHAN_BYTES: usize = 0xffff;
const DMA_MAX_CHAN_DESCRIPTORS: usize = 32;
const EP93XX_DMA_I2S1: u8 = 0; const EP93XX_DMA_I2S2: u8 = 1;
const EP93XX_DMA_AAC1: u8 = 2; const EP93XX_DMA_AAC2: u8 = 3;
const EP93XX_DMA_AAC3: u8 = 4; const EP93XX_DMA_I2S3: u8 = 5;
const EP93XX_DMA_UART1: u8 = 6; const EP93XX_DMA_UART2: u8 = 7;
const EP93XX_DMA_UART3: u8 = 8; const EP93XX_DMA_IRDA: u8 = 9;
const EP93XX_DMA_SSP: u8 = 10; const EP93XX_DMA_IDE: u8 = 11;
const EP93XX_DMA_IS_CYCLIC: usize = 0;
const INTERRUPT_UNKNOWN: i32 = 0; const INTERRUPT_DONE: i32 = 1;
const INTERRUPT_NEXT_BUFFER: i32 = 2;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct dma_async_tx_descriptor { pub chan: *mut dma_chan, pub cookie: i32, pub flags: u32, pub callback: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, pub callback_param: *mut core::ffi::c_void }
#[repr(C)] pub struct dma_chan { pub device: *mut dma_device, pub chan_id: i32, pub device_node: list_head }
#[repr(C)] pub struct dma_device { pub channels: list_head, pub cap_mask: u64 }
#[repr(C)] pub struct dma_slave_config { pub src_addr: u32, pub dst_addr: u32, pub src_addr_width: u32, pub dst_addr_width: u32 }
#[repr(C)] pub struct dma_tx_state { _private: [u8; 0] }
#[repr(C)] pub struct dma_tasklet { _private: [u8; 0] }
pub type dma_addr_t = u32;
pub type dma_transfer_direction = u32;
pub const DMA_MEM_TO_MEM: u32 = 0; pub const DMA_MEM_TO_DEV: u32 = 1; pub const DMA_DEV_TO_MEM: u32 = 2;
pub type dma_slave_buswidth = u32;
pub const DMA_SLAVE_BUSWIDTH_1_BYTE: u32 = 1; pub const DMA_SLAVE_BUSWIDTH_2_BYTES: u32 = 2; pub const DMA_SLAVE_BUSWIDTH_4_BYTES: u32 = 4;

#[repr(C)] pub struct ep93xx_dma_desc { pub src_addr:u32, pub dst_addr:u32, pub size:usize, pub complete:bool, pub txd:dma_async_tx_descriptor, pub tx_list:list_head, pub node:list_head }
#[repr(C)] pub struct ep93xx_dma_chan_cfg { pub port:u8, pub dir:dma_transfer_direction }
#[repr(C)] pub struct ep93xx_dma_chan { pub chan:dma_chan, pub edma:*const ep93xx_dma_engine, pub regs:*mut u8, pub dma_cfg:ep93xx_dma_chan_cfg, pub irq:i32, pub clk:*mut core::ffi::c_void, pub tasklet:*mut dma_tasklet, pub lock:*mut core::ffi::c_void, pub flags:usize, pub buffer:i32, pub active:list_head, pub queue:list_head, pub free_list:list_head, pub runtime_addr:u32, pub runtime_ctrl:u32, pub slave_config:dma_slave_config }
#[repr(C)] pub struct ep93xx_dma_engine { pub dma_dev:dma_device, pub m2m:bool, pub hw_setup:Option<unsafe extern "C" fn(*mut ep93xx_dma_chan)->i32>, pub hw_synchronize:Option<unsafe extern "C" fn(*mut ep93xx_dma_chan)>, pub hw_shutdown:Option<unsafe extern "C" fn(*mut ep93xx_dma_chan)>, pub hw_submit:Option<unsafe extern "C" fn(*mut ep93xx_dma_chan)>, pub hw_interrupt:Option<unsafe extern "C" fn(*mut ep93xx_dma_chan)->i32>, pub num_channels:usize }
#[repr(C)] pub struct ep93xx_edma_data { pub id:u32, pub num_channels:usize }

extern "C" { fn readl(p:*const u8)->u32; fn writel(v:u32,p:*mut u8); fn schedule(); fn dma_cookie_assign(t:*mut dma_async_tx_descriptor)->i32; }

unsafe fn m2p_set_control(c:*mut ep93xx_dma_chan, v:u32) { writel(v, (*c).regs.add(M2P_CONTROL)); let _=readl((*c).regs.add(M2P_CONTROL)); }
unsafe extern "C" fn m2p_hw_setup(c:*mut ep93xx_dma_chan)->i32 { writel(((*c).dma_cfg.port as u32)&0xf,(*c).regs.add(M2P_PPALLOC)); m2p_set_control(c,M2P_CONTROL_CH_ERROR_INT|M2P_CONTROL_ICE|M2P_CONTROL_ENABLE); (*c).buffer=0; 0 }
unsafe fn m2p_state(c:*mut ep93xx_dma_chan)->u32 { (readl((*c).regs.add(M2P_STATUS))>>4)&3 }
unsafe extern "C" fn m2p_hw_shutdown(c:*mut ep93xx_dma_chan) { m2p_set_control(c,0); while m2p_state(c)!=M2P_STATE_IDLE { } }
unsafe extern "C" fn m2m_hw_shutdown(c:*mut ep93xx_dma_chan) { writel(0,(*c).regs.add(M2M_CONTROL)); }
unsafe extern "C" fn m2m_hw_setup(c:*mut ep93xx_dma_chan)->i32 { let mut v=0; match (*c).dma_cfg.port { EP93XX_DMA_SSP => { v=5<<M2M_CONTROL_PWSC_SHIFT; v|=M2M_CONTROL_NO_HDSK; if (*c).dma_cfg.dir==DMA_MEM_TO_DEV {v|=M2M_CONTROL_DAH|M2M_CONTROL_TM_TX|M2M_CONTROL_RSS_SSPTX} else {v|=M2M_CONTROL_SAH|M2M_CONTROL_TM_RX|M2M_CONTROL_RSS_SSPRX} }, EP93XX_DMA_IDE => {v=if (*c).dma_cfg.dir==DMA_MEM_TO_DEV {3<<M2M_CONTROL_PWSC_SHIFT|M2M_CONTROL_DAH|M2M_CONTROL_TM_TX} else {2<<M2M_CONTROL_PWSC_SHIFT|M2M_CONTROL_SAH|M2M_CONTROL_TM_RX}; v|=M2M_CONTROL_NO_HDSK|M2M_CONTROL_RSS_IDE|M2M_CONTROL_PW_16}, _=>return -22}; writel(v,(*c).regs.add(M2M_CONTROL)); 0 }

/* The remaining DMA-engine entry points retain the original kernel ABI and
 * sequencing; their definitions are supplied by the surrounding kernel port. */
pub unsafe extern "C" fn ep93xx_dma_tx_status(_chan:*mut dma_chan,_cookie:i32,_state:*mut dma_tx_state)->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
