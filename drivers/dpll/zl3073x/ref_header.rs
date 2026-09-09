/* SPDX-License-Identifier: GPL-2.0-only */

// Dependencies supplied by the surrounding translation unit include the
// register field constants and the 64-bit multiplication/division helper.

use core::ffi::c_int;

#[repr(C)]
pub struct zl3073x_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct zl3073x_ref {
    // Configuration
    pub phase_comp: u64,
    pub esync_n_div: u32,
    pub freq_base: u16,
    pub freq_mult: u16,
    pub freq_ratio_m: u16,
    pub freq_ratio_n: u16,
    pub sync_ctrl: u8,
    // Invariants
    pub config: u8,
    // Status
    pub meas_freq: u32,
    pub mon_status: u8,
}

extern "C" {
    pub fn zl3073x_ref_state_fetch(zldev: *mut zl3073x_dev, index: u8) -> c_int;
    pub fn zl3073x_ref_state_get(
        zldev: *mut zl3073x_dev,
        index: u8,
    ) -> *const zl3073x_ref;
    pub fn zl3073x_ref_state_set(
        zldev: *mut zl3073x_dev,
        index: u8,
        r#ref: *const zl3073x_ref,
    ) -> c_int;
    pub fn zl3073x_ref_state_update(zldev: *mut zl3073x_dev, index: u8) -> c_int;
    pub fn zl3073x_ref_freq_factorize(
        freq: u32,
        base: *mut u16,
        mult: *mut u16,
    ) -> c_int;
    pub fn mul_u64_u32_div(a: u64, b: u32, c: u32) -> u32;
}

#[inline]
pub unsafe fn zl3073x_ref_meas_freq_get(r#ref: *const zl3073x_ref) -> u32 {
    (*r#ref).meas_freq
}

#[inline]
pub unsafe fn zl3073x_ref_freq_get(r#ref: *const zl3073x_ref) -> u32 {
    mul_u64_u32_div(
        ((*r#ref).freq_base as u64).wrapping_mul((*r#ref).freq_mult as u64),
        (*r#ref).freq_ratio_m as u32,
        (*r#ref).freq_ratio_n as u32,
    )
}

#[inline]
pub unsafe fn zl3073x_ref_freq_set(r#ref: *mut zl3073x_ref, freq: u32) -> c_int {
    let mut base: u16 = 0;
    let mut mult: u16 = 0;
    let rc = zl3073x_ref_freq_factorize(freq, &mut base, &mut mult);
    if rc != 0 {
        return rc;
    }

    (*r#ref).freq_base = base;
    (*r#ref).freq_mult = mult;
    (*r#ref).freq_ratio_m = 1;
    (*r#ref).freq_ratio_n = 1;
    0
}

#[inline]
pub unsafe fn zl3073x_ref_sync_mode_get(r#ref: *const zl3073x_ref) -> u8 {
    (((*r#ref).sync_ctrl as u32 & ZL_REF_SYNC_CTRL_MODE as u32)
        >> (ZL_REF_SYNC_CTRL_MODE as u32).trailing_zeros()) as u8
}

#[inline]
pub unsafe fn zl3073x_ref_sync_mode_set(r#ref: *mut zl3073x_ref, mode: u8) {
    let mask = ZL_REF_SYNC_CTRL_MODE as u8;
    (*r#ref).sync_ctrl = ((*r#ref).sync_ctrl & !mask) | ((mode << mask.trailing_zeros()) & mask);
}

#[inline]
pub unsafe fn zl3073x_ref_sync_pair_get(r#ref: *const zl3073x_ref) -> u8 {
    (((*r#ref).sync_ctrl as u32 & ZL_REF_SYNC_CTRL_PAIR as u32)
        >> (ZL_REF_SYNC_CTRL_PAIR as u32).trailing_zeros()) as u8
}

#[inline]
pub unsafe fn zl3073x_ref_sync_pair_set(r#ref: *mut zl3073x_ref, pair: u8) {
    let mask = ZL_REF_SYNC_CTRL_PAIR as u8;
    (*r#ref).sync_ctrl = ((*r#ref).sync_ctrl & !mask) | ((pair << mask.trailing_zeros()) & mask);
}

#[inline]
pub unsafe fn zl3073x_ref_is_diff(r#ref: *const zl3073x_ref) -> bool {
    ((*r#ref).config & ZL_REF_CONFIG_DIFF_EN as u8) != 0
}

#[inline]
pub unsafe fn zl3073x_ref_is_enabled(r#ref: *const zl3073x_ref) -> bool {
    ((*r#ref).config & ZL_REF_CONFIG_ENABLE as u8) != 0
}

#[inline]
pub unsafe fn zl3073x_ref_is_status_ok(r#ref: *const zl3073x_ref) -> bool {
    (*r#ref).mon_status == ZL_REF_MON_STATUS_OK as u8
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
