/* SPDX-License-Identifier: GPL-2.0 */
/* Translation of include/asm-alpha/dma.h. */
/* C dependencies: linux/spinlock.h and asm/io.h. */

pub const MAX_DMA_CHANNELS: u32 = 8;

pub const ALPHA_RUFFIAN_MAX_ISA_DMA_ADDRESS: u64 = 0x01000000;
pub const ALPHA_SABLE_MAX_ISA_DMA_ADDRESS: u64 = 0x80000000;
pub const ALPHA_ALCOR_MAX_ISA_DMA_ADDRESS: u64 = 0x80000000;
pub const ALPHA_MAX_ISA_DMA_ADDRESS: u64 = 0x100000000;

/* MAX_ISA_DMA_ADDRESS is selected by CONFIG_ALPHA_GENERIC and the
 * CONFIG_ALPHA_RUFFIAN/SABLE/ALCOR build conditions in the C header. */
pub const MAX_ISA_DMA_ADDRESS: u64 = ALPHA_MAX_ISA_DMA_ADDRESS;

/* MAX_DMA_ADDRESS is alpha_mv.mv_pci_tbi ? ~0UL : IDENT_ADDR + 0x01000000
 * in the original configuration-dependent C environment. */

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

pub const DMA_HIPAGE_0: u32 = 0x400 | DMA_PAGE_0;
pub const DMA_HIPAGE_1: u32 = 0x400 | DMA_PAGE_1;
pub const DMA_HIPAGE_2: u32 = 0x400 | DMA_PAGE_2;
pub const DMA_HIPAGE_3: u32 = 0x400 | DMA_PAGE_3;
pub const DMA_HIPAGE_4: u32 = 0x400 | DMA_PAGE_4;
pub const DMA_HIPAGE_5: u32 = 0x400 | DMA_PAGE_5;
pub const DMA_HIPAGE_6: u32 = 0x400 | DMA_PAGE_6;
pub const DMA_HIPAGE_7: u32 = 0x400 | DMA_PAGE_7;

pub const DMA_MODE_READ: u8 = 0x44;
pub const DMA_MODE_WRITE: u8 = 0x48;
pub const DMA_MODE_CASCADE: u8 = 0xC0;
pub const DMA_AUTOINIT: u8 = 0x10;

extern "C" {
    pub static mut dma_spin_lock: spinlock_t;
    pub fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    pub fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    pub fn dma_outb(value: u8, port: u32);
    pub fn dma_inb(port: u32) -> u8;
    pub fn request_dma(dmanr: u32, device_id: *const c_char) -> c_int;
    pub fn free_dma(dmanr: u32);
    pub fn check_dma(dmanr: u32) -> c_int;
}

/* External C types supplied by the translated dependency headers. */
pub type spinlock_t = u8;
pub type c_ulong = u64;
pub type c_char = i8;
pub type c_int = i32;

pub const KERNEL_HAVE_CHECK_DMA: bool = true;

#[inline]
pub unsafe fn claim_dma_lock() -> c_ulong {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut dma_spin_lock, &mut flags);
    flags
}

#[inline]
pub unsafe fn release_dma_lock(flags: c_ulong) {
    spin_unlock_irqrestore(&mut dma_spin_lock, flags);
}

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
    if dmanr <= 3 { dma_outb(0, DMA1_CLEAR_FF_REG); }
    else { dma_outb(0, DMA2_CLEAR_FF_REG); }
}

#[inline]
pub unsafe fn set_dma_mode(dmanr: u32, mode: i8) {
    if dmanr <= 3 { dma_outb((mode as u8) | dmanr as u8, DMA1_MODE_REG); }
    else { dma_outb((mode as u8) | (dmanr & 3) as u8, DMA2_MODE_REG); }
}

#[inline]
pub unsafe fn set_dma_ext_mode(dmanr: u32, ext_mode: i8) {
    if dmanr <= 3 { dma_outb((ext_mode as u8) | dmanr as u8, DMA1_EXT_MODE_REG); }
    else { dma_outb((ext_mode as u8) | (dmanr & 3) as u8, DMA2_EXT_MODE_REG); }
}

#[inline]
pub unsafe fn set_dma_page(dmanr: u32, pagenr: u32) {
    let (page, hipage, mask) = match dmanr {
        0 => (DMA_PAGE_0, DMA_HIPAGE_0, 0xffff_ffff), 1 => (DMA_PAGE_1, DMA_HIPAGE_1, 0xffff_ffff),
        2 => (DMA_PAGE_2, DMA_HIPAGE_2, 0xffff_ffff), 3 => (DMA_PAGE_3, DMA_HIPAGE_3, 0xffff_ffff),
        5 => (DMA_PAGE_5, DMA_HIPAGE_5, 0xffff_fffe), 6 => (DMA_PAGE_6, DMA_HIPAGE_6, 0xffff_fffe),
        7 => (DMA_PAGE_7, DMA_HIPAGE_7, 0xffff_fffe), _ => return,
    };
    dma_outb((pagenr & mask) as u8, page);
    dma_outb((pagenr >> 8) as u8, hipage);
}

#[inline]
pub unsafe fn set_dma_addr(dmanr: u32, a: u32) {
    if dmanr <= 3 { let p = ((dmanr & 3) << 1) + IO_DMA1_BASE; dma_outb((a & 0xff) as u8, p); dma_outb(((a >> 8) & 0xff) as u8, p); }
    else { let p = ((dmanr & 3) << 2) + IO_DMA2_BASE; dma_outb(((a >> 1) & 0xff) as u8, p); dma_outb(((a >> 9) & 0xff) as u8, p); }
    set_dma_page(dmanr, a >> 16);
}

#[inline]
pub unsafe fn set_dma_count(dmanr: u32, mut count: u32) {
    count -= 1;
    if dmanr <= 3 { let p = ((dmanr & 3) << 1) + 1 + IO_DMA1_BASE; dma_outb((count & 0xff) as u8, p); dma_outb(((count >> 8) & 0xff) as u8, p); }
    else { let p = ((dmanr & 3) << 2) + 2 + IO_DMA2_BASE; dma_outb(((count >> 1) & 0xff) as u8, p); dma_outb(((count >> 9) & 0xff) as u8, p); }
}

#[inline]
pub unsafe fn get_dma_residue(dmanr: u32) -> i32 {
    let io_port = if dmanr <= 3 { ((dmanr & 3) << 1) + 1 + IO_DMA1_BASE } else { ((dmanr & 3) << 2) + 2 + IO_DMA2_BASE };
    let mut count: u16 = 1 + dma_inb(io_port) as u16;
    count = count.wrapping_add((dma_inb(io_port) as u16) << 8);
    if dmanr <= 3 { count as i32 } else { (count << 1) as i32 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
