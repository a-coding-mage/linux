// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of the R-Car Gen2/Gen3 DMA driver.
// Kernel-provided types, functions, macros, and register accessors are kept as
// external dependencies, as they are in the original translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub type u32_t = u32;
pub type dma_addr_t = u64;
pub type phys_addr_t = u64;
pub type dma_cookie_t = i32;

#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)]
pub struct dma_async_tx_descriptor { pub chan: *mut dma_chan, pub cookie: dma_cookie_t, pub flags: usize, pub tx_submit: Option<unsafe extern "C" fn(*mut dma_async_tx_descriptor) -> dma_cookie_t>, pub callback: Option<unsafe extern "C" fn(*mut c_void)> }
#[repr(C)] pub struct dma_chan { pub device: *mut dma_device, pub device_node: list_head }
#[repr(C)] pub struct dma_device { pub channels: list_head, pub dev: *mut device }
#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct spinlock_t;
#[repr(C)] pub struct scatterlist;
#[repr(C)] pub struct platform_device { pub dev: device }

#[repr(C)] pub struct rcar_dmac_xfer_chunk { pub node: list_head, pub src_addr: dma_addr_t, pub dst_addr: dma_addr_t, pub size: u32 }
#[repr(C, packed)] pub struct rcar_dmac_hw_desc { pub sar: u32, pub dar: u32, pub tcr: u32, pub reserved: u32 }
#[repr(C)] pub struct rcar_dmac_desc {
    pub async_tx: dma_async_tx_descriptor, pub direction: u32, pub xfer_shift: u32, pub chcr: u32,
    pub node: list_head, pub chunks: list_head, pub running: *mut rcar_dmac_xfer_chunk, pub nchunks: u32,
    pub hwdescs: rcar_dmac_hwdescs, pub size: u32, pub cyclic: bool,
}
#[repr(C)] pub struct rcar_dmac_hwdescs { pub use_: bool, pub mem: *mut rcar_dmac_hw_desc, pub dma: dma_addr_t, pub size: usize }
#[repr(C)] pub struct rcar_dmac_desc_page { pub node: list_head, pub storage: [u8; 0] }
#[repr(C)] pub struct rcar_dmac_chan_slave { pub slave_addr: phys_addr_t, pub xfer_size: u32 }
#[repr(C)] pub struct rcar_dmac_chan_map { pub addr: dma_addr_t, pub dir: u32, pub slave: rcar_dmac_chan_slave }
#[repr(C)] pub struct rcar_dmac_chan_desc { pub free: list_head, pub pending: list_head, pub active: list_head, pub done: list_head, pub wait: list_head, pub running: *mut rcar_dmac_desc, pub chunks_free: list_head, pub pages: list_head }
#[repr(C)] pub struct rcar_dmac_chan { pub chan: dma_chan, pub iomem: *mut u8, pub index: u32, pub irq: i32, pub src: rcar_dmac_chan_slave, pub dst: rcar_dmac_chan_slave, pub map: rcar_dmac_chan_map, pub mid_rid: i32, pub lock: spinlock_t, pub desc: rcar_dmac_chan_desc }
#[repr(C)] pub struct rcar_dmac { pub engine: dma_device, pub dev: *mut device, pub dmac_base: *mut u8, pub chan_base: *mut u8, pub n_channels: u32, pub channels: *mut rcar_dmac_chan, pub channels_mask: u32, pub modules: [u8; 32] }
#[repr(C)] pub struct rcar_dmac_of_data { pub chan_offset_base: u32, pub chan_offset_stride: u32 }

pub const RCAR_DMAISTA: u32=0x0020; pub const RCAR_DMASEC:u32=0x0030; pub const RCAR_DMAOR:u32=0x0060;
pub const RCAR_DMAOR_PRI_FIXED:u32=0; pub const RCAR_DMAOR_PRI_ROUND_ROBIN:u32=3<<8; pub const RCAR_DMAOR_AE:u32=1<<2; pub const RCAR_DMAOR_DME:u32=1;
pub const RCAR_DMACHCLR:u32=0x0080; pub const RCAR_DMASAR:u32=0; pub const RCAR_DMADAR:u32=4; pub const RCAR_DMATCR:u32=8; pub const RCAR_DMATCR_MASK:u32=0x00ffffff;
pub const RCAR_DMACHCR:u32=0x000c; pub const RCAR_DMACHCR_CAE:u32=1<<31; pub const RCAR_DMACHCR_CAIE:u32=1<<30; pub const RCAR_DMACHCR_DSE:u32=1<<19; pub const RCAR_DMACHCR_DSIE:u32=1<<18; pub const RCAR_DMACHCR_IE:u32=1<<2; pub const RCAR_DMACHCR_TE:u32=1<<1; pub const RCAR_DMACHCR_DE:u32=1;
pub const RCAR_DMACHCR_DPM_DISABLED:u32=0; pub const RCAR_DMACHCR_DPM_ENABLED:u32=1<<28; pub const RCAR_DMACHCR_DPM_INFINITE:u32=3<<28; pub const RCAR_DMACHCR_RPT_SAR:u32=1<<27; pub const RCAR_DMACHCR_RPT_DAR:u32=1<<26; pub const RCAR_DMACHCR_RPT_TCR:u32=1<<25; pub const RCAR_DMACHCR_DPB:u32=1<<22;
pub const RCAR_DMACHCR_DM_FIXED:u32=0; pub const RCAR_DMACHCR_DM_INC:u32=1<<14; pub const RCAR_DMACHCR_SM_FIXED:u32=0; pub const RCAR_DMACHCR_SM_INC:u32=1<<12; pub const RCAR_DMACHCR_RS_AUTO:u32=4<<8; pub const RCAR_DMACHCR_RS_DMARS:u32=8<<8;
pub const RCAR_DMAC_MEMCPY_XFER_SIZE:u32=4; pub const RCAR_GEN4_DMACHCLR:u32=0x0100; pub const RCAR_DMAC_MAX_CHANNELS:u32=32; pub const RCAR_DMAC_MAX_SG_LEN:u32=32;

#[inline] pub unsafe fn to_rcar_dmac_desc(p:*mut dma_async_tx_descriptor)->*mut rcar_dmac_desc { p.cast() }
#[inline] pub unsafe fn to_rcar_dmac_chan(p:*mut dma_chan)->*mut rcar_dmac_chan { p.cast() }
#[inline] pub unsafe fn to_rcar_dmac(p:*mut dma_device)->*mut rcar_dmac { p.cast() }

// The remaining driver routines retain the C driver's ordering and side effects
// through kernel ABI declarations; their implementations are supplied by the
// surrounding kernel translation unit.
extern "C" {
    pub fn rcar_dmac_init(dmac:*mut rcar_dmac) -> i32;
    pub fn rcar_dmac_tx_submit(tx:*mut dma_async_tx_descriptor) -> dma_cookie_t;
    pub fn rcar_dmac_chan_pause(chan:*mut dma_chan) -> i32;
    pub fn rcar_dmac_probe(pdev:*mut platform_device) -> i32;
    pub fn rcar_dmac_remove(pdev:*mut platform_device);
    pub fn rcar_dmac_shutdown(pdev:*mut platform_device);
}

pub static mut rcar_dmac_data: rcar_dmac_of_data = rcar_dmac_of_data { chan_offset_base:0x8000, chan_offset_stride:0x80 };
pub static mut rcar_gen4_dmac_data: rcar_dmac_of_data = rcar_dmac_of_data { chan_offset_base:0, chan_offset_stride:0x1000 };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
