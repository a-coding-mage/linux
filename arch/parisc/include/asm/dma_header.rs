/* SPDX-License-Identifier: GPL-2.0 */
/* asm/dma.h: Defines for using and allocating dma channels.
 * Written by Hennus Bergman, 1992.
 * High DMA channel support & info by Hannu Savolainen
 * and John Boyd, Nov. 1992.
 * (c) Copyright 2000, Grant Grundler
 */

// Dependency: <asm/io.h> supplies byte I/O; BITS_PER_LONG and PAGE_SIZE are
// supplied by the surrounding kernel translation.

unsafe extern "C" {
    pub static mut pcxl_dma_start: ::core::ffi::c_ulong;
    pub fn outb(value: ::core::ffi::c_uchar, port: ::core::ffi::c_ushort);
    pub fn inb(port: ::core::ffi::c_ushort) -> ::core::ffi::c_uchar;
}

pub const DMA_CHUNK_SIZE: usize = BITS_PER_LONG * PAGE_SIZE;
pub const MAX_DMA_ADDRESS: usize = !0usize;
pub const MAX_DMA_CHANNELS: u32 = 8;
pub const DMA_MODE_READ: u32 = 0x44;
pub const DMA_MODE_WRITE: u32 = 0x48;
pub const DMA_MODE_CASCADE: u32 = 0xC0;
pub const DMA_AUTOINIT: u32 = 0x10;

pub const IO_DMA1_BASE: u32 = 0x00;
pub const IO_DMA2_BASE: u32 = 0xC0;
pub const DMA1_CMD_REG: u32 = 0x08;
pub const DMA1_STAT_REG: u32 = 0x08;
pub const DMA1_REQ_REG: u32 = 0x09;
pub const DMA1_MASK_REG: u32 = 0x0A;
pub const DMA1_MODE_REG: u32 = 0x0B;
pub const DMA1_CLEAR_FF_REG: u32 = 0x0C;
pub const DMA1_TEMP_REG: u32 = 0x0D;
pub const DMA1_RESET_REG: u32 = 0x0D;
pub const DMA1_CLR_MASK_REG: u32 = 0x0E;
pub const DMA1_MASK_ALL_REG: u32 = 0x0F;
pub const DMA1_EXT_MODE_REG: u32 = 0x400 | DMA1_MODE_REG;
pub const DMA2_CMD_REG: u32 = 0xD0;
pub const DMA2_STAT_REG: u32 = 0xD0;
pub const DMA2_REQ_REG: u32 = 0xD2;
pub const DMA2_MASK_REG: u32 = 0xD4;
pub const DMA2_MODE_REG: u32 = 0xD6;
pub const DMA2_CLEAR_FF_REG: u32 = 0xD8;
pub const DMA2_TEMP_REG: u32 = 0xDA;
pub const DMA2_RESET_REG: u32 = 0xDA;
pub const DMA2_CLR_MASK_REG: u32 = 0xDC;
pub const DMA2_MASK_ALL_REG: u32 = 0xDE;
pub const DMA2_EXT_MODE_REG: u32 = 0x400 | DMA2_MODE_REG;

#[inline]
pub unsafe fn claim_dma_lock() -> ::core::ffi::c_ulong { 0 }

#[inline]
pub unsafe fn release_dma_lock(_flags: ::core::ffi::c_ulong) {}

#[inline]
pub unsafe fn get_dma_residue(dmanr: u32) -> i32 {
    let io_port: u32 = if dmanr <= 3 {
        ((dmanr & 3) << 1) + 1 + IO_DMA1_BASE
    } else {
        ((dmanr & 3) << 2) + 2 + IO_DMA2_BASE
    };
    let mut count: u16 = 1u16.wrapping_add(inb(io_port as u16) as u16);
    count = count.wrapping_add((inb(io_port as u16) as u16) << 8);
    if dmanr <= 3 { count as i32 } else { ((count as u32) << 1) as i32 }
}

#[inline]
pub unsafe fn enable_dma(dmanr: u32) {
    // CONFIG_SUPERIO conditionally includes the following hardware access.
    #[cfg(feature = "CONFIG_SUPERIO")]
    if dmanr <= 3 { outb(dmanr as u8, DMA1_MASK_REG as u16); }
    #[cfg(feature = "CONFIG_SUPERIO")]
    if dmanr > 3 { outb((dmanr & 3) as u8, DMA2_MASK_REG as u16); }
}

#[inline]
pub unsafe fn disable_dma(dmanr: u32) {
    // CONFIG_SUPERIO conditionally includes the following hardware access.
    #[cfg(feature = "CONFIG_SUPERIO")]
    if dmanr <= 3 { outb((dmanr | 4) as u8, DMA1_MASK_REG as u16); }
    #[cfg(feature = "CONFIG_SUPERIO")]
    if dmanr > 3 { outb(((dmanr & 3) | 4) as u8, DMA2_MASK_REG as u16); }
}

#[inline]
pub fn request_dma(_dmanr: u32, _device_id: impl Sized) -> i32 { 0 }

#[inline] pub unsafe fn clear_dma_ff(_dmanr: u32) {}
#[inline] pub unsafe fn set_dma_mode(_dmanr: u32, _mode: i8) {}
#[inline] pub unsafe fn set_dma_page(_dmanr: u32, _pagenr: i8) {}
#[inline] pub unsafe fn set_dma_addr(_dmanr: u32, _a: u32) {}
#[inline] pub unsafe fn set_dma_count(_dmanr: u32, _count: u32) {}

// free_dma(dmanr) is an empty macro.
#[inline] pub fn free_dma(_dmanr: impl Sized) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
