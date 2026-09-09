/* SPDX-License-Identifier: GPL-2.0 */

/***************************************************************************
 *  Header for amcc5920 pci chip
 *
 *   copyright          : (C) 2002 by Frank Mori Hess
 **************************************************************************/

// plx pci chip registers and bits
#[repr(u32)]
pub enum AmccRegisters {
    AMCC_INTCS_REG = 0x38,
    AMCC_PASS_THRU_REG = 0x60,
}

#[repr(u32)]
pub enum AmccIncsrBits {
    AMCC_ADDON_INTR_ENABLE_BIT = 0x2000,
    AMCC_ADDON_INTR_ACTIVE_BIT = 0x400000,
    AMCC_INTR_ACTIVE_BIT = 0x800000,
}

pub const BITS_PER_REGION: i32 = 8;

#[inline]
pub fn amcc_wait_state_bits(region: u32, num_wait_states: u32) -> u32 {
    (num_wait_states & 0x7) << (region.wrapping_sub(1) * BITS_PER_REGION as u32)
}

#[repr(u32)]
pub enum AmccPrefetchBits {
    PREFETCH_DISABLED = 0x0,
    PREFETCH_SMALL = 0x8,
    PREFETCH_MEDIUM = 0x10,
    PREFETCH_LARGE = 0x18,
}

#[inline]
pub fn amcc_prefetch_bits(region: u32, prefetch: AmccPrefetchBits) -> u32 {
    (prefetch as u32) << (region.wrapping_sub(1) * BITS_PER_REGION as u32)
}

#[inline]
pub fn amcc_PTADR_mode_bit(region: u32) -> u32 {
    0x80 << (region.wrapping_sub(1) * BITS_PER_REGION as u32)
}

#[inline]
pub fn amcc_disable_write_fifo_bit(region: u32) -> u32 {
    0x20 << (region.wrapping_sub(1) * BITS_PER_REGION as u32)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
