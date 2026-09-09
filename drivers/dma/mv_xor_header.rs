/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2007, 2008, Marvell International Ltd. */

// Dependencies supplied by the surrounding kernel translation:
// linux/types.h, linux/io.h, linux/dmaengine.h, linux/interrupt.h

pub const MV_XOR_POOL_SIZE: usize = MV_XOR_SLOT_SIZE * 3072;
pub const MV_XOR_SLOT_SIZE: usize = 64;
pub const MV_XOR_THRESHOLD: usize = 1;
pub const MV_XOR_MAX_CHANNELS: usize = 2;

pub const MV_XOR_MIN_BYTE_COUNT: usize = SZ_128;
pub const MV_XOR_MAX_BYTE_COUNT: usize = SZ_16M - 1;

pub const XOR_OPERATION_MODE_XOR: u32 = 0;
pub const XOR_OPERATION_MODE_MEMCPY: u32 = 2;
pub const XOR_OPERATION_MODE_IN_DESC: u32 = 7;
pub const XOR_DESCRIPTOR_SWAP: u32 = 1 << 14;
pub const XOR_DESC_SUCCESS: u32 = 0x4000_0000;

pub const XOR_DESC_OPERATION_XOR: u32 = 0 << 24;
pub const XOR_DESC_OPERATION_CRC32C: u32 = 1 << 24;
pub const XOR_DESC_OPERATION_MEMCPY: u32 = 2 << 24;
pub const XOR_DESC_DMA_OWNED: u32 = 1 << 31;
pub const XOR_DESC_EOD_INT_EN: u32 = 1 << 31;

#[inline] pub unsafe fn XOR_CURR_DESC(chan: *mut mv_xor_chan) -> usize { (*chan).mmr_high_base as usize + 0x10 + ((*chan).idx * 4) }
#[inline] pub unsafe fn XOR_NEXT_DESC(chan: *mut mv_xor_chan) -> usize { (*chan).mmr_high_base as usize + ((*chan).idx * 4) }
#[inline] pub unsafe fn XOR_BYTE_COUNT(chan: *mut mv_xor_chan) -> usize { (*chan).mmr_high_base as usize + 0x20 + ((*chan).idx * 4) }
#[inline] pub unsafe fn XOR_DEST_POINTER(chan: *mut mv_xor_chan) -> usize { (*chan).mmr_high_base as usize + 0xb0 + ((*chan).idx * 4) }
#[inline] pub unsafe fn XOR_BLOCK_SIZE(chan: *mut mv_xor_chan) -> usize { (*chan).mmr_high_base as usize + 0xc0 + ((*chan).idx * 4) }
#[inline] pub unsafe fn XOR_INIT_VALUE_LOW(chan: *mut mv_xor_chan) -> usize { (*chan).mmr_high_base as usize + 0xe0 }
#[inline] pub unsafe fn XOR_INIT_VALUE_HIGH(chan: *mut mv_xor_chan) -> usize { (*chan).mmr_high_base as usize + 0xe4 }
#[inline] pub unsafe fn XOR_CONFIG(chan: *mut mv_xor_chan) -> usize { (*chan).mmr_base as usize + 0x10 + ((*chan).idx * 4) }
#[inline] pub unsafe fn XOR_ACTIVATION(chan: *mut mv_xor_chan) -> usize { (*chan).mmr_base as usize + 0x20 + ((*chan).idx * 4) }
#[inline] pub unsafe fn XOR_INTR_CAUSE(chan: *mut mv_xor_chan) -> usize { (*chan).mmr_base as usize + 0x30 }
#[inline] pub unsafe fn XOR_INTR_MASK(chan: *mut mv_xor_chan) -> usize { (*chan).mmr_base as usize + 0x40 }
#[inline] pub unsafe fn XOR_ERROR_CAUSE(chan: *mut mv_xor_chan) -> usize { (*chan).mmr_base as usize + 0x50 }
#[inline] pub unsafe fn XOR_ERROR_ADDR(chan: *mut mv_xor_chan) -> usize { (*chan).mmr_base as usize + 0x60 }

pub const XOR_INT_END_OF_DESC: u32 = 1 << 0;
pub const XOR_INT_END_OF_CHAIN: u32 = 1 << 1;
pub const XOR_INT_STOPPED: u32 = 1 << 2;
pub const XOR_INT_PAUSED: u32 = 1 << 3;
pub const XOR_INT_ERR_DECODE: u32 = 1 << 4;
pub const XOR_INT_ERR_RDPROT: u32 = 1 << 5;
pub const XOR_INT_ERR_WRPROT: u32 = 1 << 6;
pub const XOR_INT_ERR_OWN: u32 = 1 << 7;
pub const XOR_INT_ERR_PAR: u32 = 1 << 8;
pub const XOR_INT_ERR_MBUS: u32 = 1 << 9;
pub const XOR_INTR_ERRORS: u32 = XOR_INT_ERR_DECODE | XOR_INT_ERR_RDPROT | XOR_INT_ERR_WRPROT | XOR_INT_ERR_OWN | XOR_INT_ERR_PAR | XOR_INT_ERR_MBUS;
pub const XOR_INTR_MASK_VALUE: u32 = XOR_INT_END_OF_DESC | XOR_INT_END_OF_CHAIN | XOR_INT_STOPPED | XOR_INTR_ERRORS;

#[inline] pub const fn WINDOW_BASE(w: usize) -> usize { 0x50 + (w << 2) }
#[inline] pub const fn WINDOW_SIZE(w: usize) -> usize { 0x70 + (w << 2) }
#[inline] pub const fn WINDOW_REMAP_HIGH(w: usize) -> usize { 0x90 + (w << 2) }
#[inline] pub const fn WINDOW_BAR_ENABLE(chan: usize) -> usize { 0x40 + (chan << 2) }
#[inline] pub const fn WINDOW_OVERRIDE_CTRL(chan: usize) -> usize { 0xa0 + (chan << 2) }
pub const WINDOW_COUNT: usize = 8;

#[repr(C)]
pub struct mv_xor_device {
    pub xor_base: *mut core::ffi::c_void, pub xor_high_base: *mut core::ffi::c_void,
    pub clk: *mut clk, pub channels: [*mut mv_xor_chan; MV_XOR_MAX_CHANNELS], pub xor_type: i32,
    pub win_start: [u32; WINDOW_COUNT], pub win_end: [u32; WINDOW_COUNT],
}

#[repr(C)]
pub struct mv_xor_chan {
    pub pending: i32, pub lock: spinlock_t, pub mmr_base: *mut core::ffi::c_void,
    pub mmr_high_base: *mut core::ffi::c_void, pub idx: usize, pub irq: i32,
    pub chain: list_head, pub free_slots: list_head, pub allocated_slots: list_head, pub completed_slots: list_head,
    pub dma_desc_pool: dma_addr_t, pub dma_desc_pool_virt: *mut core::ffi::c_void, pub pool_size: usize,
    pub dmadev: dma_device, pub dmachan: dma_chan, pub slots_allocated: i32, pub irq_tasklet: tasklet_struct,
    pub op_in_desc: i32, pub dummy_src: [u8; MV_XOR_MIN_BYTE_COUNT], pub dummy_dst: [u8; MV_XOR_MIN_BYTE_COUNT],
    pub dummy_src_addr: dma_addr_t, pub dummy_dst_addr: dma_addr_t, pub saved_config_reg: u32, pub saved_int_mask_reg: u32,
    pub xordev: *mut mv_xor_device,
}

#[repr(C)]
pub struct mv_xor_desc_slot { pub node: list_head, pub sg_tx_list: list_head, pub type_: dma_transaction_type, pub hw_desc: *mut core::ffi::c_void, pub idx: u16, pub async_tx: dma_async_tx_descriptor }

#[repr(C)]
pub struct mv_xor_desc { pub status: u32, pub crc32_result: u32, pub desc_command: u32, pub phy_next_desc: u32, pub byte_count: u32, pub phy_dest_addr: u32, pub phy_src_addr: [u32; 8], pub reserved0: u32, pub reserved1: u32 }

#[inline] pub const fn mv_phy_src_idx(src_idx: usize) -> usize { src_idx }

// C container_of(addr_hw_desc, struct mv_xor_desc_slot, hw_desc)
#[inline] pub unsafe fn to_mv_sw_desc(addr_hw_desc: *mut core::ffi::c_void) -> *mut mv_xor_desc_slot { (addr_hw_desc as *mut u8).sub(core::mem::offset_of!(mv_xor_desc_slot, hw_desc)) as *mut mv_xor_desc_slot }
#[inline] pub unsafe fn mv_hw_desc_slot_idx(hw_desc: *mut core::ffi::c_void, idx: usize) -> *mut core::ffi::c_void { (hw_desc as *mut u8).add(idx << 5) as *mut core::ffi::c_void }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
