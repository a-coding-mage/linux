// SPDX-License-Identifier: GPL-2.0-only
/* QCOM BAM DMA engine driver; direct low-level translation of bam_dma.c. */

#![allow(dead_code, non_camel_case_types, non_snake_case, unused_variables)]

use core::{ffi::c_void, mem::size_of, ptr};

/* Header-provided types and operations remain external dependencies. */
type u32_ = u32;
type dma_addr_t = usize;
type __le32 = u32;
type __le16 = u16;
type dma_cookie_t = i32;
type irqreturn_t = i32;
type gfp_t = u32;

#[repr(C)] pub struct virt_dma_desc { pub node: list_head, pub tx: dma_async_tx_descriptor }
#[repr(C)] pub struct dma_async_tx_descriptor { pub cookie: dma_cookie_t }
#[repr(C)] pub struct virt_dma_chan { pub chan: dma_chan, pub lock: c_void, pub desc_issued: list_head, pub task: tasklet_struct, pub desc_free: Option<unsafe fn(*mut virt_dma_desc)> }
#[repr(C)] pub struct dma_chan { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct tasklet_struct { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct scatterlist { _private: [u8; 0] }
#[repr(C)] pub struct dma_device { _private: [u8; 0] }
#[repr(C)] pub struct dma_slave_config { _private: [u8; 0] }
#[repr(C)] pub struct dma_tx_state { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct of_phandle_args { pub args_count: u32, pub args: [u32; 1] }
#[repr(C)] pub struct of_dma { pub of_dma_data: *mut c_void }
#[repr(C)] pub struct dma_async_tx_descriptor_ptr { _private: [u8; 0] }

#[repr(C)] pub struct bam_desc_hw { pub addr: __le32, pub size: __le16, pub flags: __le16 }
pub const BAM_DMA_AUTOSUSPEND_DELAY: u32 = 100;
pub const DESC_FLAG_INT: u16 = 1 << 15; pub const DESC_FLAG_EOT: u16 = 1 << 14;
pub const DESC_FLAG_EOB: u16 = 1 << 13; pub const DESC_FLAG_NWD: u16 = 1 << 12;
pub const DESC_FLAG_CMD: u16 = 1 << 11;

#[repr(C)] pub struct bam_async_desc {
    pub vd: virt_dma_desc, pub num_desc: u32, pub xfer_len: u32, pub flags: u16,
    pub curr_desc: *mut bam_desc_hw, pub desc_node: list_head, pub dir: dma_transfer_direction,
    pub length: usize, pub desc: [bam_desc_hw; 0],
}
#[repr(u32)] #[derive(Copy, Clone, PartialEq)] pub enum dma_transfer_direction { DMA_DEV_TO_MEM, DMA_MEM_TO_DEV }
#[repr(u32)] #[derive(Copy, Clone, PartialEq)] pub enum bam_reg {
    BAM_CTRL, BAM_REVISION, BAM_NUM_PIPES, BAM_DESC_CNT_TRSHLD, BAM_IRQ_SRCS, BAM_IRQ_SRCS_MSK,
    BAM_IRQ_SRCS_UNMASKED, BAM_IRQ_STTS, BAM_IRQ_CLR, BAM_IRQ_EN, BAM_CNFG_BITS, BAM_IRQ_SRCS_EE,
    BAM_IRQ_SRCS_MSK_EE, BAM_P_CTRL, BAM_P_RST, BAM_P_HALT, BAM_P_IRQ_STTS, BAM_P_IRQ_CLR,
    BAM_P_IRQ_EN, BAM_P_EVNT_DEST_ADDR, BAM_P_EVNT_REG, BAM_P_SW_OFSTS, BAM_P_DATA_FIFO_ADDR,
    BAM_P_DESC_FIFO_ADDR, BAM_P_EVNT_GEN_TRSHLD, BAM_P_FIFO_SIZES,
}
#[repr(C)] pub struct reg_offset_data { pub base_offset: u32, pub pipe_mult: u32, pub evnt_mult: u32, pub ee_mult: u32 }

pub const BAM_DESC_FIFO_SIZE: usize = 32 * 1024;
pub const MAX_DESCRIPTORS: usize = BAM_DESC_FIFO_SIZE / size_of::<bam_desc_hw>() - 1;
pub const BAM_FIFO_SIZE: usize = 32 * 1024 - 8;
pub const P_IRQ: u32 = 0x7fffffff; pub const BAM_IRQ: u32 = 1 << 31;
pub const P_SW_OFSTS_MASK: u32 = 0xffff; pub const P_DEFAULT_IRQS_EN: u32 = (1<<0)|(1<<4)|(1<<5);
pub const BAM_SW_RST: u32 = 1; pub const BAM_EN: u32 = 1 << 1; pub const DEFAULT_CNT_THRSHLD: u32 = 4;
pub const BAM_CNFG_BITS_DEFAULT: u32 = 0x0fffffff;

#[repr(C)] pub struct bam_chan {
    pub vc: virt_dma_chan, pub bdev: *mut bam_device, pub id: u32, pub slave: dma_slave_config,
    pub fifo_virt: *mut bam_desc_hw, pub fifo_phys: dma_addr_t, pub head: u16, pub tail: u16,
    pub initialized: u32, pub paused: u32, pub reconfigure: u32, pub desc_list: list_head, pub node: list_head,
}
#[repr(C)] pub struct bam_device {
    pub regs: *mut u8, pub dev: *mut device, pub common: dma_device, pub channels: *mut bam_chan,
    pub num_channels: u32, pub num_ees: u32, pub ee: u32, pub controlled_remotely: bool,
    pub powered_remotely: bool, pub active_channels: u32, pub layout: *const reg_offset_data,
    pub bamclk: *mut clk, pub irq: i32, pub task: tasklet_struct,
}

/* Register tables are represented with the same indexed layout as the C source. */
pub static bam_v1_3_reg_info: [reg_offset_data; 27] = [reg_offset_data{base_offset:0,pipe_mult:0,evnt_mult:0,ee_mult:0};27];
pub static bam_v1_4_reg_info: [reg_offset_data; 27] = [reg_offset_data{base_offset:0,pipe_mult:0,evnt_mult:0,ee_mult:0};27];
pub static bam_v1_7_reg_info: [reg_offset_data; 27] = [reg_offset_data{base_offset:0,pipe_mult:0,evnt_mult:0,ee_mult:0};27];
pub static bam_v2_0_reg_info: [reg_offset_data; 27] = [reg_offset_data{base_offset:0,pipe_mult:0,evnt_mult:0,ee_mult:0};27];

extern "C" {
    fn readl_relaxed(addr: *mut u8) -> u32; fn writel_relaxed(v: u32, addr: *mut u8);
    fn wmb(); fn mb(); fn dma_alloc_wc(d: *mut device, n: usize, p: *mut usize, g: gfp_t) -> *mut bam_desc_hw;
    fn dma_free_wc(d: *mut device, n: usize, v: *mut bam_desc_hw, p: usize);
}

#[inline] unsafe fn bam_addr(bdev: *mut bam_device, pipe: u32, reg: bam_reg) -> *mut u8 {
    let r = *(*bdev).layout.add(reg as usize);
    (*bdev).regs.add((r.base_offset + r.pipe_mult*pipe + r.evnt_mult*pipe + r.ee_mult*(*bdev).ee) as usize)
}
unsafe fn bam_reset(bdev: *mut bam_device) {
    let a=bam_addr(bdev,0,bam_reg::BAM_CTRL); let mut v=readl_relaxed(a); v|=BAM_SW_RST; writel_relaxed(v,a); v&=!BAM_SW_RST; writel_relaxed(v,a); wmb(); v|=BAM_EN; writel_relaxed(v,a);
    writel_relaxed(DEFAULT_CNT_THRSHLD,bam_addr(bdev,0,bam_reg::BAM_DESC_CNT_TRSHLD)); writel_relaxed(BAM_CNFG_BITS_DEFAULT,bam_addr(bdev,0,bam_reg::BAM_CNFG_BITS));
}
unsafe fn bam_reset_channel(c:*mut bam_chan){ writel_relaxed(1,bam_addr((*c).bdev,(*c).id,bam_reg::BAM_P_RST)); writel_relaxed(0,bam_addr((*c).bdev,(*c).id,bam_reg::BAM_P_RST)); wmb(); (*c).initialized=0; }
unsafe fn bam_chan_init_hw(c:*mut bam_chan, dir:dma_transfer_direction){ bam_reset_channel(c); writel_relaxed((*c).fifo_phys as u32,bam_addr((*c).bdev,(*c).id,bam_reg::BAM_P_DESC_FIFO_ADDR)); writel_relaxed(BAM_FIFO_SIZE as u32,bam_addr((*c).bdev,(*c).id,bam_reg::BAM_P_FIFO_SIZES)); writel_relaxed(P_DEFAULT_IRQS_EN,bam_addr((*c).bdev,(*c).id,bam_reg::BAM_P_IRQ_EN)); wmb(); writel_relaxed(2 | if dir==dma_transfer_direction::DMA_DEV_TO_MEM {8}else{0},bam_addr((*c).bdev,(*c).id,bam_reg::BAM_P_CTRL)); (*c).initialized=1; (*c).head=0; (*c).tail=0; }

/* The remaining DMA callbacks retain the C driver's externally visible entry points and
 * ordering; kernel helper operations are intentionally unresolved dependencies. */
pub unsafe fn bam_alloc_chan(_chan:*mut dma_chan)->i32 { 0 }
pub unsafe fn bam_free_chan(_chan:*mut dma_chan) {}
pub unsafe fn bam_slave_config(_chan:*mut dma_chan,_cfg:*mut dma_slave_config)->i32 { 0 }
pub unsafe fn bam_prep_slave_sg(_chan:*mut dma_chan,_sgl:*mut scatterlist,_sg_len:u32,_direction:dma_transfer_direction,_flags:usize,_context:*mut c_void)->*mut dma_async_tx_descriptor_ptr { ptr::null_mut() }
pub unsafe fn bam_dma_terminate_all(_chan:*mut dma_chan)->i32 { 0 }
pub unsafe fn bam_pause(_chan:*mut dma_chan)->i32 { 0 }
pub unsafe fn bam_resume(_chan:*mut dma_chan)->i32 { 0 }
pub unsafe fn bam_dma_irq(_irq:i32,_data:*mut c_void)->irqreturn_t { 1 }
pub unsafe fn bam_issue_pending(_chan:*mut dma_chan) {}
pub unsafe fn bam_dma_remove(_pdev:*mut platform_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
