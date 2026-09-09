/* SPDX-License-Identifier: GPL-2.0-only */

/* Dependencies supplied by the surrounding translation unit:
 * linux/bitfield.h, linux/math64.h, linux/stddef.h, linux/types.h, and regs.h
 */

use core::ffi::c_void;

#[repr(C)]
pub struct zl3073x_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct zl3073x_synth {
    pub freq_mult: u32,
    pub freq_base: u16,
    pub freq_m: u16,
    pub freq_n: u16,
    pub ctrl: u8,
}

unsafe extern "C" {
    pub fn zl3073x_synth_state_fetch(zldev: *mut zl3073x_dev, synth_id: u8) -> i32;

    pub fn zl3073x_synth_state_get(
        zldev: *mut zl3073x_dev,
        synth_id: u8,
    ) -> *const zl3073x_synth;

    fn mul_u64_u32_div(a: u64, b: u32, c: u32) -> u64;
}

// These register-field constants are supplied by regs.h's Rust translation.
unsafe extern "C" {
    static ZL_SYNTH_CTRL_DPLL_SEL: u8;
    static ZL_SYNTH_CTRL_EN: u8;
}

#[inline]
pub unsafe fn zl3073x_synth_dpll_get(synth: *const zl3073x_synth) -> u8 {
    let mask = ZL_SYNTH_CTRL_DPLL_SEL;
    ((*synth).ctrl & mask) >> mask.trailing_zeros()
}

#[inline]
pub unsafe fn zl3073x_synth_freq_get(synth: *const zl3073x_synth) -> u32 {
    mul_u64_u32_div(
        (*synth).freq_base as u64 * (*synth).freq_m as u64,
        (*synth).freq_mult,
        (*synth).freq_n as u32,
    ) as u32
}

#[inline]
pub unsafe fn zl3073x_synth_is_enabled(synth: *const zl3073x_synth) -> bool {
    let mask = ZL_SYNTH_CTRL_EN;
    (((*synth).ctrl & mask) >> mask.trailing_zeros()) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
