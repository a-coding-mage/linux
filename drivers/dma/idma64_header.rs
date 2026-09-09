/* SPDX-License-Identifier: GPL-2.0-only */
/* Driver for the Intel integrated DMA 64-bit */

// Linux dependencies supplied by the surrounding translation unit.

/* Channel registers */
pub const IDMA64_CH_SAR: i32 = 0x00;
pub const IDMA64_CH_DAR: i32 = 0x08;
pub const IDMA64_CH_LLP: i32 = 0x10;
pub const IDMA64_CH_CTL_LO: i32 = 0x18;
pub const IDMA64_CH_CTL_HI: i32 = 0x1c;
pub const IDMA64_CH_SSTAT: i32 = 0x20;
pub const IDMA64_CH_DSTAT: i32 = 0x28;
pub const IDMA64_CH_SSTATAR: i32 = 0x30;
pub const IDMA64_CH_DSTATAR: i32 = 0x38;
pub const IDMA64_CH_CFG_LO: i32 = 0x40;
pub const IDMA64_CH_CFG_HI: i32 = 0x44;
pub const IDMA64_CH_SGR: i32 = 0x48;
pub const IDMA64_CH_DSR: i32 = 0x50;
pub const IDMA64_CH_LENGTH: i32 = 0x58;

pub const IDMA64C_CTLL_INT_EN: u32 = 1 << 0;
pub const fn IDMA64C_CTLL_DST_WIDTH(x: u32) -> u32 { x << 1 }
pub const fn IDMA64C_CTLL_SRC_WIDTH(x: u32) -> u32 { x << 4 }
pub const IDMA64C_CTLL_DST_INC: u32 = 0 << 8;
pub const IDMA64C_CTLL_DST_FIX: u32 = 1 << 8;
pub const IDMA64C_CTLL_SRC_INC: u32 = 0 << 10;
pub const IDMA64C_CTLL_SRC_FIX: u32 = 1 << 10;
pub const fn IDMA64C_CTLL_DST_MSIZE(x: u32) -> u32 { x << 11 }
pub const fn IDMA64C_CTLL_SRC_MSIZE(x: u32) -> u32 { x << 14 }
pub const IDMA64C_CTLL_FC_M2P: u32 = 1 << 20;
pub const IDMA64C_CTLL_FC_P2M: u32 = 2 << 20;
pub const IDMA64C_CTLL_LLP_D_EN: u32 = 1 << 27;
pub const IDMA64C_CTLL_LLP_S_EN: u32 = 1 << 28;

pub const IDMA64C_CTLH_BLOCK_TS_MASK: u32 = (1 << 17) - 1;
pub const fn IDMA64C_CTLH_BLOCK_TS(x: u32) -> u32 { x & IDMA64C_CTLH_BLOCK_TS_MASK }
pub const IDMA64C_CTLH_DONE: u32 = 1 << 17;

pub const IDMA64C_CFGL_DST_BURST_ALIGN: u32 = 1 << 0;
pub const IDMA64C_CFGL_SRC_BURST_ALIGN: u32 = 1 << 1;
pub const IDMA64C_CFGL_CH_SUSP: u32 = 1 << 8;
pub const IDMA64C_CFGL_FIFO_EMPTY: u32 = 1 << 9;
pub const IDMA64C_CFGL_CH_DRAIN: u32 = 1 << 10;
pub const IDMA64C_CFGL_DST_OPT_BL: u32 = 1 << 20;
pub const IDMA64C_CFGL_SRC_OPT_BL: u32 = 1 << 21;

pub const fn IDMA64C_CFGH_SRC_PER(x: u32) -> u32 { x << 0 }
pub const fn IDMA64C_CFGH_DST_PER(x: u32) -> u32 { x << 4 }
pub const fn IDMA64C_CFGH_RD_ISSUE_THD(x: u32) -> u32 { x << 8 }
pub const fn IDMA64C_CFGH_WR_ISSUE_THD(x: u32) -> u32 { x << 18 }

pub const IDMA64_INT_XFER: i32 = 0x00;
pub const IDMA64_INT_BLOCK: i32 = 0x08;
pub const IDMA64_INT_SRC_TRAN: i32 = 0x10;
pub const IDMA64_INT_DST_TRAN: i32 = 0x18;
pub const IDMA64_INT_ERROR: i32 = 0x20;
pub const fn IDMA64_RAW(x: i32) -> i32 { 0x2c0 + x }
pub const fn IDMA64_STATUS(x: i32) -> i32 { 0x2e8 + x }
pub const fn IDMA64_MASK(x: i32) -> i32 { 0x310 + x }
pub const fn IDMA64_CLEAR(x: i32) -> i32 { 0x338 + x }
pub const IDMA64_STATUS_INT: i32 = 0x360;
pub const IDMA64_CFG: i32 = 0x398;
pub const IDMA64_CH_EN: i32 = 0x3a0;
pub const IDMA64_CFG_DMA_EN: u32 = 1 << 0;

#[repr(C)]
pub struct idma64_lli { pub sar: u64, pub dar: u64, pub llp: u64, pub ctllo: u32, pub ctlhi: u32, pub sstat: u32, pub dstat: u32 }
#[repr(C)]
pub struct idma64_hw_desc { pub lli: *mut idma64_lli, pub llp: dma_addr_t, pub phys: dma_addr_t, pub len: u32 }
#[repr(C)]
pub struct idma64_desc { pub vdesc: virt_dma_desc, pub direction: dma_transfer_direction, pub hw: *mut idma64_hw_desc, pub ndesc: u32, pub length: usize, pub status: dma_status }

#[inline]
pub unsafe fn to_idma64_desc(vdesc: *mut virt_dma_desc) -> *mut idma64_desc { container_of!(vdesc, idma64_desc, vdesc) }

#[repr(C)]
pub struct idma64_chan { pub vchan: virt_dma_chan, pub regs: *mut u8, pub direction: dma_transfer_direction, pub mask: u32, pub config: dma_slave_config, pub pool: *mut core::ffi::c_void, pub desc: *mut idma64_desc }
#[inline]
pub unsafe fn to_idma64_chan(chan: *mut dma_chan) -> *mut idma64_chan { container_of!(chan, idma64_chan, vchan.chan) }

#[inline] pub unsafe fn channel_set_bit(idma64: *mut idma64, reg: i32, mask: u32) { dma_writel(idma64, reg, (mask << 8) | mask); }
#[inline] pub unsafe fn channel_clear_bit(idma64: *mut idma64, reg: i32, mask: u32) { dma_writel(idma64, reg, mask << 8); }
#[inline] pub unsafe fn idma64c_readl(c: *mut idma64_chan, offset: i32) -> u32 { readl((*c).regs.add(offset as usize)) }
#[inline] pub unsafe fn idma64c_writel(c: *mut idma64_chan, offset: i32, value: u32) { writel(value, (*c).regs.add(offset as usize)); }
#[inline] pub unsafe fn idma64c_readq(c: *mut idma64_chan, offset: i32) -> u64 { lo_hi_readq((*c).regs.add(offset as usize)) }
#[inline] pub unsafe fn idma64c_writeq(c: *mut idma64_chan, offset: i32, value: u64) { lo_hi_writeq(value, (*c).regs.add(offset as usize)); }
#[inline] pub unsafe fn channel_readl(c: *mut idma64_chan, reg: i32) -> u32 { idma64c_readl(c, reg) }
#[inline] pub unsafe fn channel_writel(c: *mut idma64_chan, reg: i32, value: u32) { idma64c_writel(c, reg, value); }
#[inline] pub unsafe fn channel_readq(c: *mut idma64_chan, reg: i32) -> u64 { idma64c_readq(c, reg) }
#[inline] pub unsafe fn channel_writeq(c: *mut idma64_chan, reg: i32, value: u64) { idma64c_writeq(c, reg, value); }

#[repr(C)]
pub struct idma64 { pub dma: dma_device, pub regs: *mut u8, pub all_chan_mask: u16, pub chan: *mut idma64_chan }
#[inline] pub unsafe fn to_idma64(ddev: *mut dma_device) -> *mut idma64 { container_of!(ddev, idma64, dma) }
#[inline] pub unsafe fn idma64_readl(d: *mut idma64, offset: i32) -> u32 { readl((*d).regs.add(offset as usize)) }
#[inline] pub unsafe fn idma64_writel(d: *mut idma64, offset: i32, value: u32) { writel(value, (*d).regs.add(offset as usize)); }
#[inline] pub unsafe fn dma_readl(d: *mut idma64, reg: i32) -> u32 { idma64_readl(d, reg) }
#[inline] pub unsafe fn dma_writel(d: *mut idma64, reg: i32, value: u32) { idma64_writel(d, reg, value); }

#[repr(C)]
pub struct idma64_chip { pub dev: *mut device, pub sysdev: *mut device, pub irq: i32, pub regs: *mut u8, pub idma64: *mut idma64 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
