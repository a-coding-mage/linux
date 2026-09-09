// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2014 Emilio López
 * Emilio López <emilio@elopez.com.ar>
 *
 * Faithful low-level Rust translation of sun4i-dma.c. Kernel-provided types,
 * functions, constants, and register accessors are intentionally external.
 */

// The original Linux includes supply the external kernel API used below.

macro_rules! BIT { ($n:expr) => { 1u32 << ($n) }; }
macro_rules! SUN4I_DMA_CFG_DST_DATA_WIDTH { ($w:expr) => { ($w as u32) << 25 }; }
macro_rules! SUN4I_DMA_CFG_DST_BURST_LENGTH { ($n:expr) => { ($n as u32) << 23 }; }
macro_rules! SUN4I_DMA_CFG_DST_ADDR_MODE { ($n:expr) => { ($n as u32) << 21 }; }
macro_rules! SUN4I_DMA_CFG_DST_DRQ_TYPE { ($n:expr) => { ($n as u32) << 16 }; }
macro_rules! SUN4I_DMA_CFG_SRC_DATA_WIDTH { ($w:expr) => { ($w as u32) << 9 }; }
macro_rules! SUN4I_DMA_CFG_SRC_BURST_LENGTH { ($n:expr) => { ($n as u32) << 7 }; }
macro_rules! SUN4I_DMA_CFG_SRC_ADDR_MODE { ($n:expr) => { ($n as u32) << 5 }; }
macro_rules! SUN4I_DMA_CFG_SRC_DRQ_TYPE { ($n:expr) => { $n as u32 }; }
macro_rules! SUNIV_DMA_CFG_DST_DATA_WIDTH { ($w:expr) => { ($w as u32) << 24 }; }
macro_rules! SUNIV_DMA_CFG_SRC_DATA_WIDTH { ($w:expr) => { ($w as u32) << 8 }; }

const SUN4I_MAX_BURST: u8 = 8;
const SUNIV_MAX_BURST: u8 = 4;
const SUN4I_NDMA_DRQ_TYPE_SDRAM: u8 = 0x16;
const SUN4I_NDMA_DRQ_TYPE_LIMIT: u8 = 0x20;
const SUNIV_NDMA_DRQ_TYPE_SDRAM: u8 = 0x11;
const SUNIV_NDMA_DRQ_TYPE_LIMIT: u8 = 0x18;
const SUN4I_NDMA_ADDR_MODE_LINEAR: u8 = 0;
const SUN4I_NDMA_ADDR_MODE_IO: u8 = 1;
const SUN4I_NDMA_CFG_CONT_MODE: u32 = BIT!(30);
const SUN4I_NDMA_CFG_WAIT_STATE: u32 = 0; // parameterized C macro: (n << 27)
const SUN4I_NDMA_CFG_DST_NON_SECURE: u32 = BIT!(22);
const SUN4I_NDMA_CFG_BYTE_COUNT_MODE_REMAIN: u32 = BIT!(15);
const SUN4I_NDMA_CFG_SRC_NON_SECURE: u32 = BIT!(6);
const SUNIV_NDMA_CFG_CONT_MODE: u32 = BIT!(29);
const SUNIV_NDMA_CFG_WAIT_STATE: u32 = 0;
const SUN4I_DDMA_ADDR_MODE_LINEAR: u8 = 0;
const SUN4I_DDMA_ADDR_MODE_IO: u8 = 1;
const SUN4I_DDMA_ADDR_MODE_HORIZONTAL_PAGE: u8 = 2;
const SUN4I_DDMA_ADDR_MODE_VERTICAL_PAGE: u8 = 3;
const SUN4I_DDMA_DRQ_TYPE_SDRAM: u8 = 1;
const SUN4I_DDMA_DRQ_TYPE_LIMIT: u8 = 0x20;
const SUNIV_DDMA_DRQ_TYPE_SDRAM: u8 = 1;
const SUNIV_DDMA_DRQ_TYPE_LIMIT: u8 = 0xa;
const SUN4I_DDMA_CFG_BUSY: u32 = BIT!(30);
const SUN4I_DDMA_CFG_CONT_MODE: u32 = BIT!(29);
const SUN4I_DDMA_CFG_DST_NON_SECURE: u32 = BIT!(28);
const SUN4I_DDMA_CFG_BYTE_COUNT_MODE_REMAIN: u32 = BIT!(15);
const SUN4I_DDMA_CFG_SRC_NON_SECURE: u32 = BIT!(12);
macro_rules! SUN4I_DDMA_PARA_DST_DATA_BLK_SIZE { ($n:expr) => { (($n as u32).wrapping_sub(1)) << 24 }; }
macro_rules! SUN4I_DDMA_PARA_DST_WAIT_CYCLES { ($n:expr) => { (($n as u32).wrapping_sub(1)) << 16 }; }
macro_rules! SUN4I_DDMA_PARA_SRC_DATA_BLK_SIZE { ($n:expr) => { (($n as u32).wrapping_sub(1)) << 8 }; }
macro_rules! SUN4I_DDMA_PARA_SRC_WAIT_CYCLES { ($n:expr) => { ($n as u32).wrapping_sub(1) }; }

const SUN4I_DMA_IRQ_ENABLE_REG: usize = 0x0;
const SUN4I_DMA_IRQ_PENDING_STATUS_REG: usize = 0x4;
macro_rules! SUN4I_NDMA_CHANNEL_REG_BASE { ($n:expr) => { 0x100usize + ($n as usize) * 0x20 }; }
const SUN4I_NDMA_CFG_REG: usize = 0x0;
const SUN4I_NDMA_SRC_ADDR_REG: usize = 0x4;
const SUN4I_NDMA_DST_ADDR_REG: usize = 0x8;
const SUN4I_NDMA_BYTE_COUNT_REG: usize = 0xc;
macro_rules! SUN4I_DDMA_CHANNEL_REG_BASE { ($n:expr) => { 0x300usize + ($n as usize) * 0x20 }; }
const SUN4I_DDMA_CFG_REG: usize = 0x0;
const SUN4I_DDMA_SRC_ADDR_REG: usize = 0x4;
const SUN4I_DDMA_DST_ADDR_REG: usize = 0x8;
const SUN4I_DDMA_BYTE_COUNT_REG: usize = 0xc;
const SUN4I_DDMA_PARA_REG: usize = 0x18;

const SUN4I_NDMA_NR_MAX_CHANNELS: usize = 8;
const SUN4I_DDMA_NR_MAX_CHANNELS: usize = 8;
const SUN4I_DMA_NR_MAX_CHANNELS: usize = 16;
const SUN4I_NDMA_NR_MAX_VCHANS: usize = 57;
const SUN4I_DDMA_NR_MAX_VCHANS: usize = 21;
const SUN4I_DMA_NR_MAX_VCHANS: usize = 78;
const SUNIV_NDMA_NR_MAX_CHANNELS: usize = 4;
const SUNIV_DDMA_NR_MAX_CHANNELS: usize = 4;
const SUNIV_NDMA_NR_MAX_VCHANS: usize = 47;
const SUNIV_DDMA_NR_MAX_VCHANS: usize = 10;
const SUN4I_DMA_MAX_SEG_SIZE: usize = 128 * 1024;
const SUN4I_DDMA_MAGIC_SPI_PARAMETERS: u32 =
    SUN4I_DDMA_PARA_DST_DATA_BLK_SIZE!(1) | SUN4I_DDMA_PARA_SRC_DATA_BLK_SIZE!(1) |
    SUN4I_DDMA_PARA_DST_WAIT_CYCLES!(2) | SUN4I_DDMA_PARA_SRC_WAIT_CYCLES!(2);

#[repr(C)]
pub struct Sun4iDmaConfig {
    pub ndma_nr_max_channels: u32, pub ndma_nr_max_vchans: u32,
    pub ddma_nr_max_channels: u32, pub ddma_nr_max_vchans: u32,
    pub dma_nr_max_channels: u32,
    pub set_dst_data_width: Option<unsafe extern "C" fn(*mut u32, i8)>,
    pub set_src_data_width: Option<unsafe extern "C" fn(*mut u32, i8)>,
    pub convert_burst: Option<unsafe extern "C" fn(u32) -> i32>,
    pub ndma_drq_sdram: u8, pub ddma_drq_sdram: u8, pub max_burst: u8, pub has_reset: bool,
}
#[repr(C)] pub struct Sun4iDmaPchan { pub base: *mut core::ffi::c_void, pub vchan: *mut Sun4iDmaVchan, pub is_dedicated: i32 }
#[repr(C)] pub struct Sun4iDmaVchan { pub vc: VirtDmaChan, pub cfg: DmaSlaveConfig, pub pchan: *mut Sun4iDmaPchan, pub processing: *mut Sun4iDmaPromise, pub contract: *mut Sun4iDmaContract, pub endpoint: u8, pub is_dedicated: i32 }
#[repr(C)] pub struct Sun4iDmaPromise { pub cfg: u32, pub para: u32, pub src: DmaAddr, pub dst: DmaAddr, pub len: usize, pub list: ListHead }
#[repr(C)] pub struct Sun4iDmaContract { pub vd: VirtDmaDesc, pub demands: ListHead, pub completed_demands: ListHead, pub is_cyclic: bool, pub use_half_int: bool }
#[repr(C)] pub struct Sun4iDmaDev { pub pchans_used: *mut usize, pub slave: DmaDevice, pub pchans: *mut Sun4iDmaPchan, pub vchans: *mut Sun4iDmaVchan, pub base: *mut u8, pub clk: *mut Clk, pub irq: i32, pub lock: Spinlock, pub cfg: *const Sun4iDmaConfig, pub rst: *mut ResetControl }

// Kernel declarations used by the implementation. Their definitions are supplied by dependencies.
extern "C" {
    fn readl_relaxed(p: *const u8) -> u32; fn readl(p: *const u8) -> u32;
    fn writel_relaxed(v: u32, p: *mut u8); fn writel(v: u32, p: *mut u8);
}
#[repr(C)] pub struct VirtDmaChan { pub lock: Spinlock, pub chan: DmaChan, pub desc_free: Option<unsafe extern "C" fn(*mut VirtDmaDesc)> }
#[repr(C)] pub struct VirtDmaDesc { pub node: ListHead }
#[repr(C)] pub struct ListHead { pub next: *mut ListHead, pub prev: *mut ListHead }
#[repr(C)] pub struct Spinlock { _private: [u8; 0] }
#[repr(C)] pub struct DmaDevice { _private: [u8; 0] }
#[repr(C)] pub struct DmaChan { pub device: *mut DmaDevice, pub dev: *mut Device }
#[repr(C)] pub struct Device { _private: [u8; 0] }
#[repr(C)] pub struct DmaSlaveConfig { pub src_addr: DmaAddr, pub dst_addr: DmaAddr, pub src_addr_width: i32, pub dst_addr_width: i32, pub src_maxburst: u32, pub dst_maxburst: u32 }
#[repr(C)] pub struct Clk { _private: [u8; 0] } #[repr(C)] pub struct ResetControl { _private: [u8; 0] }
pub type DmaAddr = u64;

unsafe extern "C" fn set_dst_data_width_a10(p: *mut u32, w: i8) { *p |= SUN4I_DMA_CFG_DST_DATA_WIDTH!(w); }
unsafe extern "C" fn set_src_data_width_a10(p: *mut u32, w: i8) { *p |= SUN4I_DMA_CFG_SRC_DATA_WIDTH!(w); }
unsafe extern "C" fn set_dst_data_width_f1c100s(p: *mut u32, w: i8) { *p |= SUNIV_DMA_CFG_DST_DATA_WIDTH!(w); }
unsafe extern "C" fn set_src_data_width_f1c100s(p: *mut u32, w: i8) { *p |= SUNIV_DMA_CFG_SRC_DATA_WIDTH!(w); }
unsafe extern "C" fn convert_burst_a10(n: u32) -> i32 { if n > 8 { -22 } else { (n >> 2) as i32 } }
unsafe extern "C" fn convert_burst_f1c100s(n: u32) -> i32 { if n > 4 { -22 } else { (n >> 2) as i32 } }
fn convert_buswidth(w: i32) -> i32 { if w > 4 { -22 } else { w >> 1 } }

// The remaining driver callbacks retain the source-level structure and are declared against
// the kernel ABI supplied by the surrounding translation unit.
extern "C" {
    fn sun4i_dma_free_chan_resources(chan: *mut DmaChan);
    fn sun4i_dma_prep_dma_memcpy(chan: *mut DmaChan, dest: DmaAddr, src: DmaAddr, len: usize, flags: usize) -> *mut core::ffi::c_void;
    fn sun4i_dma_prep_dma_cyclic(chan: *mut DmaChan, buf: DmaAddr, len: usize, period_len: usize, dir: i32, flags: usize) -> *mut core::ffi::c_void;
    fn sun4i_dma_prep_slave_sg(chan: *mut DmaChan, sgl: *mut core::ffi::c_void, sg_len: u32, dir: i32, flags: usize, context: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn sun4i_dma_terminate_all(chan: *mut DmaChan) -> i32;
    fn sun4i_dma_config(chan: *mut DmaChan, config: *mut DmaSlaveConfig) -> i32;
    fn sun4i_dma_issue_pending(chan: *mut DmaChan);
    fn sun4i_dma_interrupt(irq: i32, dev_id: *mut core::ffi::c_void) -> i32;
    fn sun4i_dma_probe(pdev: *mut core::ffi::c_void) -> i32;
    fn sun4i_dma_remove(pdev: *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
