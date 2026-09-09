/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/include/asm/dma.h: Defines for using and allocating dma channels.
 * Written by Hennus Bergman, 1992.
 * High DMA channel support & info by Hannu Savolainen
 * and John Boyd, Nov. 1992.
 *
 * NOTE: all this is true *only* for ISA/EISA expansions on Mips boards
 * and can only be used for expansion cards. Onboard DMA controllers, such
 * as the R4030 on Jazz boards behave totally different!
 */

/* C dependencies: asm/io.h, linux/spinlock.h, and linux/delay.h. */

extern "C" {
    pub fn outb(value: u8, port: u16);
    pub fn inb(port: u16) -> u8;
    pub fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    pub fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    pub static mut dma_spin_lock: spinlock_t;
    pub fn request_dma(dmanr: u32, device_id: *const core::ffi::c_char) -> i32;
    pub fn free_dma(dmanr: u32);
}

/* External kernel type supplied by linux/spinlock.h. */
pub type spinlock_t = usize;

/* HAVE_REALLY_SLOW_DMA_CONTROLLER selects outb_p instead of outb. */
#[inline]
pub unsafe fn dma_outb(value: u8, port: u16) {
    outb(value, port);
}

#[inline]
pub unsafe fn dma_inb(port: u16) -> u8 {
    inb(port)
}

/* CONFIG_GENERIC_ISA_DMA_SUPPORT_BROKEN must be absent for this constant. */
pub const MAX_DMA_CHANNELS: u32 = 8;

/* MAX_DMA_ADDRESS depends on PAGE_OFFSET and the SGI configuration. */
/* MAX_DMA_PFN depends on PAGE_OFFSET, PAGE_SHIFT, PFN_DOWN, and virt_to_phys. */
/* MAX_DMA32_PFN is 1UL << (32 - PAGE_SHIFT) when not supplied externally. */

/* 8237 DMA controllers */
pub const IO_DMA1_BASE: u32 = 0x00;
pub const IO_DMA2_BASE: u32 = 0xC0;

/* DMA controller registers */
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

pub const DMA_ADDR_0: u32 = 0x00;
pub const DMA_ADDR_1: u32 = 0x02;
pub const DMA_ADDR_2: u32 = 0x04;
pub const DMA_ADDR_3: u32 = 0x06;
pub const DMA_ADDR_4: u32 = 0xC0;
pub const DMA_ADDR_5: u32 = 0xC4;
pub const DMA_ADDR_6: u32 = 0xC8;
pub const DMA_ADDR_7: u32 = 0xCC;

pub const DMA_CNT_0: u32 = 0x01;
pub const DMA_CNT_1: u32 = 0x03;
pub const DMA_CNT_2: u32 = 0x05;
pub const DMA_CNT_3: u32 = 0x07;
pub const DMA_CNT_4: u32 = 0xC2;
pub const DMA_CNT_5: u32 = 0xC6;
pub const DMA_CNT_6: u32 = 0xCA;
pub const DMA_CNT_7: u32 = 0xCE;

pub const DMA_PAGE_0: u32 = 0x87;
pub const DMA_PAGE_1: u32 = 0x83;
pub const DMA_PAGE_2: u32 = 0x81;
pub const DMA_PAGE_3: u32 = 0x82;
pub const DMA_PAGE_5: u32 = 0x8B;
pub const DMA_PAGE_6: u32 = 0x89;
pub const DMA_PAGE_7: u32 = 0x8A;

pub const DMA_MODE_READ: u8 = 0x44;
pub const DMA_MODE_WRITE: u8 = 0x48;
pub const DMA_MODE_CASCADE: u8 = 0xC0;
pub const DMA_AUTOINIT: u8 = 0x10;

#[inline]
pub unsafe fn claim_dma_lock() -> usize {
    let mut flags = 0usize;
    spin_lock_irqsave(&mut dma_spin_lock, &mut flags);
    flags
}

#[inline]
pub unsafe fn release_dma_lock(flags: usize) {
    spin_unlock_irqrestore(&mut dma_spin_lock, flags);
}

#[inline]
pub unsafe fn enable_dma(dmanr: u32) {
    if dmanr <= 3 { dma_outb(dmanr as u8, DMA1_MASK_REG as u16); }
    else { dma_outb((dmanr & 3) as u8, DMA2_MASK_REG as u16); }
}

#[inline]
pub unsafe fn disable_dma(dmanr: u32) {
    if dmanr <= 3 { dma_outb((dmanr | 4) as u8, DMA1_MASK_REG as u16); }
    else { dma_outb(((dmanr & 3) | 4) as u8, DMA2_MASK_REG as u16); }
}

#[inline]
pub unsafe fn clear_dma_ff(dmanr: u32) {
    if dmanr <= 3 { dma_outb(0, DMA1_CLEAR_FF_REG as u16); }
    else { dma_outb(0, DMA2_CLEAR_FF_REG as u16); }
}

#[inline]
pub unsafe fn set_dma_mode(dmanr: u32, mode: i8) {
    if dmanr <= 3 { dma_outb((mode as u8) | dmanr as u8, DMA1_MODE_REG as u16); }
    else { dma_outb((mode as u8) | (dmanr & 3) as u8, DMA2_MODE_REG as u16); }
}

#[inline]
pub unsafe fn set_dma_page(dmanr: u32, pagenr: i8) {
    match dmanr {
        0 => dma_outb(pagenr as u8, DMA_PAGE_0 as u16),
        1 => dma_outb(pagenr as u8, DMA_PAGE_1 as u16),
        2 => dma_outb(pagenr as u8, DMA_PAGE_2 as u16),
        3 => dma_outb(pagenr as u8, DMA_PAGE_3 as u16),
        5 => dma_outb((pagenr as u8) & 0xfe, DMA_PAGE_5 as u16),
        6 => dma_outb((pagenr as u8) & 0xfe, DMA_PAGE_6 as u16),
        7 => dma_outb((pagenr as u8) & 0xfe, DMA_PAGE_7 as u16),
        _ => {}
    }
}

#[inline]
pub unsafe fn set_dma_addr(dmanr: u32, a: u32) {
    set_dma_page(dmanr, (a >> 16) as i8);
    if dmanr <= 3 {
        dma_outb((a & 0xff) as u8, (((dmanr & 3) << 1) + IO_DMA1_BASE) as u16);
        dma_outb(((a >> 8) & 0xff) as u8, (((dmanr & 3) << 1) + IO_DMA1_BASE) as u16);
    } else {
        dma_outb(((a >> 1) & 0xff) as u8, (((dmanr & 3) << 2) + IO_DMA2_BASE) as u16);
        dma_outb(((a >> 9) & 0xff) as u8, (((dmanr & 3) << 2) + IO_DMA2_BASE) as u16);
    }
}

#[inline]
pub unsafe fn set_dma_count(dmanr: u32, mut count: u32) {
    count = count.wrapping_sub(1);
    if dmanr <= 3 {
        dma_outb((count & 0xff) as u8, (((dmanr & 3) << 1) + 1 + IO_DMA1_BASE) as u16);
        dma_outb(((count >> 8) & 0xff) as u8, (((dmanr & 3) << 1) + 1 + IO_DMA1_BASE) as u16);
    } else {
        dma_outb(((count >> 1) & 0xff) as u8, (((dmanr & 3) << 2) + 2 + IO_DMA2_BASE) as u16);
        dma_outb(((count >> 9) & 0xff) as u8, (((dmanr & 3) << 2) + 2 + IO_DMA2_BASE) as u16);
    }
}

#[inline]
pub unsafe fn get_dma_residue(dmanr: u32) -> i32 {
    let io_port = if dmanr <= 3 { ((dmanr & 3) << 1) + 1 + IO_DMA1_BASE }
                  else { ((dmanr & 3) << 2) + 2 + IO_DMA2_BASE };
    let mut count = 1u16.wrapping_add(dma_inb(io_port as u16) as u16);
    count = count.wrapping_add((dma_inb(io_port as u16) as u16) << 8);
    if dmanr <= 3 { count as i32 } else { (count << 1) as i32 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
