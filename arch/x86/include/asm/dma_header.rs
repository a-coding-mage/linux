/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/include/asm/dma.h: Defines for using and allocating dma channels.
 * Written by Hennus Bergman, 1992.
 * High DMA channel support & info by Hannu Savolainen
 * and John Boyd, Nov. 1992.
 */

// C dependencies: linux/spinlock.h and asm/io.h.

#[cfg(feature = "HAVE_REALLY_SLOW_DMA_CONTROLLER")]
#[inline]
unsafe fn dma_outb(value: u8, port: u16) { outb_p(value, port); }

#[cfg(not(feature = "HAVE_REALLY_SLOW_DMA_CONTROLLER"))]
#[inline]
unsafe fn dma_outb(value: u8, port: u16) { outb(value, port); }

#[inline]
unsafe fn dma_inb(port: u16) -> u8 { inb(port) }

pub const MAX_DMA_CHANNELS: u32 = 8;
pub const MAX_DMA_PFN: usize = ((16usize * 1024 * 1024) >> PAGE_SHIFT);
pub const MAX_DMA32_PFN: usize = 1usize << (32 - PAGE_SHIFT);

// CONFIG_X86_32 selects PAGE_OFFSET + 0x1000000; otherwise this is __va(MAX_DMA_PFN << PAGE_SHIFT).
#[cfg(feature = "CONFIG_X86_32")]
pub const MAX_DMA_ADDRESS: usize = PAGE_OFFSET + 0x1000000;
#[cfg(not(feature = "CONFIG_X86_32"))]
pub const MAX_DMA_ADDRESS: usize = __va(MAX_DMA_PFN << PAGE_SHIFT);

pub const IO_DMA1_BASE: u16 = 0x00;
pub const IO_DMA2_BASE: u16 = 0xC0;

pub const DMA1_CMD_REG: u16 = 0x08;
pub const DMA1_STAT_REG: u16 = 0x08;
pub const DMA1_REQ_REG: u16 = 0x09;
pub const DMA1_MASK_REG: u16 = 0x0A;
pub const DMA1_MODE_REG: u16 = 0x0B;
pub const DMA1_CLEAR_FF_REG: u16 = 0x0C;
pub const DMA1_TEMP_REG: u16 = 0x0D;
pub const DMA1_RESET_REG: u16 = 0x0D;
pub const DMA1_CLR_MASK_REG: u16 = 0x0E;
pub const DMA1_MASK_ALL_REG: u16 = 0x0F;
pub const DMA2_CMD_REG: u16 = 0xD0;
pub const DMA2_STAT_REG: u16 = 0xD0;
pub const DMA2_REQ_REG: u16 = 0xD2;
pub const DMA2_MASK_REG: u16 = 0xD4;
pub const DMA2_MODE_REG: u16 = 0xD6;
pub const DMA2_CLEAR_FF_REG: u16 = 0xD8;
pub const DMA2_TEMP_REG: u16 = 0xDA;
pub const DMA2_RESET_REG: u16 = 0xDA;
pub const DMA2_CLR_MASK_REG: u16 = 0xDC;
pub const DMA2_MASK_ALL_REG: u16 = 0xDE;

pub const DMA_ADDR_0: u16 = 0x00; pub const DMA_ADDR_1: u16 = 0x02;
pub const DMA_ADDR_2: u16 = 0x04; pub const DMA_ADDR_3: u16 = 0x06;
pub const DMA_ADDR_4: u16 = 0xC0; pub const DMA_ADDR_5: u16 = 0xC4;
pub const DMA_ADDR_6: u16 = 0xC8; pub const DMA_ADDR_7: u16 = 0xCC;
pub const DMA_CNT_0: u16 = 0x01; pub const DMA_CNT_1: u16 = 0x03;
pub const DMA_CNT_2: u16 = 0x05; pub const DMA_CNT_3: u16 = 0x07;
pub const DMA_CNT_4: u16 = 0xC2; pub const DMA_CNT_5: u16 = 0xC6;
pub const DMA_CNT_6: u16 = 0xCA; pub const DMA_CNT_7: u16 = 0xCE;
pub const DMA_PAGE_0: u16 = 0x87; pub const DMA_PAGE_1: u16 = 0x83;
pub const DMA_PAGE_2: u16 = 0x81; pub const DMA_PAGE_3: u16 = 0x82;
pub const DMA_PAGE_5: u16 = 0x8B; pub const DMA_PAGE_6: u16 = 0x89;
pub const DMA_PAGE_7: u16 = 0x8A;

pub const DMA_MODE_READ: u8 = 0x44;
pub const DMA_MODE_WRITE: u8 = 0x48;
pub const DMA_MODE_CASCADE: u8 = 0xC0;
pub const DMA_AUTOINIT: u8 = 0x10;

#[cfg(feature = "CONFIG_ISA_DMA_API")]
extern "C" {
    pub static mut dma_spin_lock: spinlock_t;
    pub fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    pub fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    pub fn request_dma(dmanr: u32, device_id: *const c_char) -> c_int;
    pub fn free_dma(dmanr: u32);
}

#[cfg(feature = "CONFIG_ISA_DMA_API")]
#[inline]
pub unsafe fn claim_dma_lock() -> c_ulong {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&raw mut dma_spin_lock, &mut flags);
    flags
}

#[cfg(feature = "CONFIG_ISA_DMA_API")]
#[inline]
pub unsafe fn release_dma_lock(flags: c_ulong) { spin_unlock_irqrestore(&raw mut dma_spin_lock, flags); }

#[inline]
pub unsafe fn enable_dma(dmanr: u32) {
    if dmanr <= 3 { dma_outb(dmanr as u8, DMA1_MASK_REG); }
    else { dma_outb((dmanr & 3) as u8, DMA2_MASK_REG); }
}

#[inline]
pub unsafe fn disable_dma(dmanr: u32) {
    if dmanr <= 3 { dma_outb((dmanr | 4) as u8, DMA1_MASK_REG); }
    else { dma_outb(((dmanr & 3) | 4) as u8, DMA2_MASK_REG); }
}

#[inline]
pub unsafe fn clear_dma_ff(dmanr: u32) {
    if dmanr <= 3 { dma_outb(0, DMA1_CLEAR_FF_REG); } else { dma_outb(0, DMA2_CLEAR_FF_REG); }
}

#[inline]
pub unsafe fn set_dma_mode(dmanr: u32, mode: u8) {
    if dmanr <= 3 { dma_outb(mode | dmanr as u8, DMA1_MODE_REG); }
    else { dma_outb(mode | (dmanr & 3) as u8, DMA2_MODE_REG); }
}

#[inline]
pub unsafe fn set_dma_page(dmanr: u32, pagenr: u8) {
    match dmanr {
        0 => dma_outb(pagenr, DMA_PAGE_0), 1 => dma_outb(pagenr, DMA_PAGE_1),
        2 => dma_outb(pagenr, DMA_PAGE_2), 3 => dma_outb(pagenr, DMA_PAGE_3),
        5 => dma_outb(pagenr & 0xfe, DMA_PAGE_5), 6 => dma_outb(pagenr & 0xfe, DMA_PAGE_6),
        7 => dma_outb(pagenr & 0xfe, DMA_PAGE_7), _ => {}
    }
}

#[inline]
pub unsafe fn set_dma_addr(dmanr: u32, a: u32) {
    set_dma_page(dmanr, (a >> 16) as u8);
    if dmanr <= 3 {
        dma_outb((a & 0xff) as u8, (((dmanr & 3) << 1) as u16) + IO_DMA1_BASE);
        dma_outb(((a >> 8) & 0xff) as u8, (((dmanr & 3) << 1) as u16) + IO_DMA1_BASE);
    } else {
        dma_outb(((a >> 1) & 0xff) as u8, (((dmanr & 3) << 2) as u16) + IO_DMA2_BASE);
        dma_outb(((a >> 9) & 0xff) as u8, (((dmanr & 3) << 2) as u16) + IO_DMA2_BASE);
    }
}

#[inline]
pub unsafe fn set_dma_count(dmanr: u32, mut count: u32) {
    count = count.wrapping_sub(1);
    if dmanr <= 3 {
        dma_outb((count & 0xff) as u8, (((dmanr & 3) << 1) as u16) + 1 + IO_DMA1_BASE);
        dma_outb(((count >> 8) & 0xff) as u8, (((dmanr & 3) << 1) as u16) + 1 + IO_DMA1_BASE);
    } else {
        dma_outb(((count >> 1) & 0xff) as u8, (((dmanr & 3) << 2) as u16) + 2 + IO_DMA2_BASE);
        dma_outb((count >> 9) as u8, (((dmanr & 3) << 2) as u16) + 2 + IO_DMA2_BASE);
    }
}

#[inline]
pub unsafe fn get_dma_residue(dmanr: u32) -> i32 {
    let io_port: u16 = if dmanr <= 3 { (((dmanr & 3) << 1) as u16) + 1 + IO_DMA1_BASE }
        else { (((dmanr & 3) << 2) as u16) + 2 + IO_DMA2_BASE };
    let mut count: u16 = 1u16.wrapping_add(dma_inb(io_port) as u16);
    count = count.wrapping_add((dma_inb(io_port) as u16) << 8);
    if dmanr <= 3 { count as i32 } else { count.wrapping_shl(1) as i32 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
