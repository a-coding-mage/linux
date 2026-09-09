/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from the PowerPC DMA header; originally guarded by __KERNEL__. */

/* Defines for using and allocating DMA channels. */

pub const MAX_DMA_CHANNELS: usize = 8;
pub const MAX_DMA_ADDRESS: usize = !0;

/* HAVE_REALLY_SLOW_DMA_CONTROLLER selects outb_p instead of outb. */
#[cfg(feature = "HAVE_REALLY_SLOW_DMA_CONTROLLER")]
macro_rules! dma_outb { ($value:expr, $port:expr) => { outb_p($value, $port) }; }
#[cfg(not(feature = "HAVE_REALLY_SLOW_DMA_CONTROLLER"))]
macro_rules! dma_outb { ($value:expr, $port:expr) => { outb($value, $port) }; }
macro_rules! dma_inb { ($port:expr) => { inb($port) }; }

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

pub const DMA_ADDR_0: u32 = 0x00; pub const DMA_ADDR_1: u32 = 0x02;
pub const DMA_ADDR_2: u32 = 0x04; pub const DMA_ADDR_3: u32 = 0x06;
pub const DMA_ADDR_4: u32 = 0xC0; pub const DMA_ADDR_5: u32 = 0xC4;
pub const DMA_ADDR_6: u32 = 0xC8; pub const DMA_ADDR_7: u32 = 0xCC;
pub const DMA_CNT_0: u32 = 0x01; pub const DMA_CNT_1: u32 = 0x03;
pub const DMA_CNT_2: u32 = 0x05; pub const DMA_CNT_3: u32 = 0x07;
pub const DMA_CNT_4: u32 = 0xC2; pub const DMA_CNT_5: u32 = 0xC6;
pub const DMA_CNT_6: u32 = 0xCA; pub const DMA_CNT_7: u32 = 0xCE;
pub const DMA_LO_PAGE_0: u32 = 0x87; pub const DMA_LO_PAGE_1: u32 = 0x83;
pub const DMA_LO_PAGE_2: u32 = 0x81; pub const DMA_LO_PAGE_3: u32 = 0x82;
pub const DMA_LO_PAGE_5: u32 = 0x8B; pub const DMA_LO_PAGE_6: u32 = 0x89;
pub const DMA_LO_PAGE_7: u32 = 0x8A;
pub const DMA_HI_PAGE_0: u32 = 0x487; pub const DMA_HI_PAGE_1: u32 = 0x483;
pub const DMA_HI_PAGE_2: u32 = 0x481; pub const DMA_HI_PAGE_3: u32 = 0x482;
pub const DMA_HI_PAGE_5: u32 = 0x48B; pub const DMA_HI_PAGE_6: u32 = 0x489;
pub const DMA_HI_PAGE_7: u32 = 0x48A;
pub const DMA1_EXT_REG: u32 = 0x40B;
pub const DMA2_EXT_REG: u32 = 0x4D6;

/* On 32-bit PowerPC these are external globals; on 64-bit they are constants. */
#[cfg(not(target_pointer_width = "64"))]
unsafe extern "C" { pub static mut DMA_MODE_WRITE: u32; pub static mut DMA_MODE_READ: u32; }
#[cfg(target_pointer_width = "64")]
pub const DMA_MODE_READ: u32 = 0x44;
#[cfg(target_pointer_width = "64")]
pub const DMA_MODE_WRITE: u32 = 0x48;
pub const DMA_MODE_CASCADE: u32 = 0xC0;
pub const DMA_AUTOINIT: u32 = 0x10;

unsafe extern "C" {
    pub static mut dma_spin_lock: spinlock_t;
    pub fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut u64);
    pub fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: u64);
    pub fn outb(value: u8, port: u32);
    pub fn outb_p(value: u8, port: u32);
    pub fn inb(port: u32) -> u8;
    pub fn request_dma(dmanr: u32, device_id: *const i8) -> i32;
    pub fn free_dma(dmanr: u32);
}

#[inline]
pub unsafe fn claim_dma_lock() -> u64 {
    let mut flags = 0u64;
    spin_lock_irqsave(&raw mut dma_spin_lock, &mut flags);
    flags
}

#[inline]
pub unsafe fn release_dma_lock(flags: u64) { spin_unlock_irqrestore(&raw mut dma_spin_lock, flags); }

#[inline]
pub unsafe fn enable_dma(dmanr: u32) {
    let uc_dma_cmd = 0u8;
    if dmanr != 4 { dma_outb!(0u8, DMA2_MASK_REG); dma_outb!(uc_dma_cmd, DMA2_CMD_REG); }
    if dmanr <= 3 { dma_outb!(dmanr as u8, DMA1_MASK_REG); dma_outb!(uc_dma_cmd, DMA1_CMD_REG); }
    else { dma_outb!((dmanr & 3) as u8, DMA2_MASK_REG); }
}

#[inline]
pub unsafe fn disable_dma(dmanr: u32) {
    if dmanr <= 3 { dma_outb!((dmanr | 4) as u8, DMA1_MASK_REG); }
    else { dma_outb!(((dmanr & 3) | 4) as u8, DMA2_MASK_REG); }
}

#[inline]
pub unsafe fn clear_dma_ff(dmanr: u32) {
    if dmanr <= 3 { dma_outb!(0u8, DMA1_CLEAR_FF_REG); } else { dma_outb!(0u8, DMA2_CLEAR_FF_REG); }
}

#[inline]
pub unsafe fn set_dma_mode(dmanr: u32, mode: i8) {
    if dmanr <= 3 { dma_outb!((mode as u8).wrapping_add(dmanr as u8), DMA1_MODE_REG); }
    else { dma_outb!((mode as u8).wrapping_add((dmanr & 3) as u8), DMA2_MODE_REG); }
}

#[inline]
pub unsafe fn set_dma_page(dmanr: u32, pagenr: i32) {
    let (lo, hi, mask) = match dmanr { 0 => (DMA_LO_PAGE_0, DMA_HI_PAGE_0, false), 1 => (DMA_LO_PAGE_1, DMA_HI_PAGE_1, false), 2 => (DMA_LO_PAGE_2, DMA_HI_PAGE_2, false), 3 => (DMA_LO_PAGE_3, DMA_HI_PAGE_3, false), 5 => (DMA_LO_PAGE_5, DMA_HI_PAGE_5, true), 6 => (DMA_LO_PAGE_6, DMA_HI_PAGE_6, true), 7 => (DMA_LO_PAGE_7, DMA_HI_PAGE_7, true), _ => return };
    dma_outb!((if mask { pagenr & 0xfe } else { pagenr }) as u8, lo);
    dma_outb!((pagenr >> 8) as u8, hi);
}

#[inline]
pub unsafe fn set_dma_addr(dmanr: u32, phys: u32) {
    if dmanr <= 3 { let p = ((dmanr & 3) << 1) + IO_DMA1_BASE; dma_outb!((phys & 0xff) as u8, p); dma_outb!(((phys >> 8) & 0xff) as u8, p); }
    else { let p = ((dmanr & 3) << 2) + IO_DMA2_BASE; dma_outb!(((phys >> 1) & 0xff) as u8, p); dma_outb!(((phys >> 9) & 0xff) as u8, p); }
    set_dma_page(dmanr, (phys >> 16) as i32);
}

#[inline]
pub unsafe fn set_dma_count(dmanr: u32, mut count: u32) {
    count = count.wrapping_sub(1);
    if dmanr <= 3 { let p = ((dmanr & 3) << 1) + 1 + IO_DMA1_BASE; dma_outb!((count & 0xff) as u8, p); dma_outb!(((count >> 8) & 0xff) as u8, p); }
    else { let p = ((dmanr & 3) << 2) + 2 + IO_DMA2_BASE; dma_outb!(((count >> 1) & 0xff) as u8, p); dma_outb!((count >> 9) as u8, p); }
}

#[inline]
pub unsafe fn get_dma_residue(dmanr: u32) -> i32 {
    let io_port = if dmanr <= 3 { ((dmanr & 3) << 1) + 1 + IO_DMA1_BASE } else { ((dmanr & 3) << 2) + 2 + IO_DMA2_BASE };
    let mut count = 1u16.wrapping_add(dma_inb!(io_port) as u16);
    count = count.wrapping_add((dma_inb!(io_port) as u16) << 8);
    if dmanr <= 3 { count as i32 } else { (count << 1) as i32 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
