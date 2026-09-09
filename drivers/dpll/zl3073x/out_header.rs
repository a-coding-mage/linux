/* SPDX-License-Identifier: GPL-2.0-only */

// C dependencies: <linux/bitfield.h>, <linux/stddef.h>, <linux/types.h>, "regs.h"

use core::ffi::c_int;

#[repr(C)]
pub struct zl3073x_dev {
    _private: [u8; 0],
}

/**
 * struct zl3073x_out - output state
 * @div: output divisor
 * @width: output pulse width
 * @esync_n_period: embedded sync or n-pin period (for n-div formats)
 * @esync_n_width: embedded sync or n-pin pulse width
 * @phase_comp: phase compensation
 * @mode: output mode
 * @ctrl: output control
 */
#[repr(C)]
pub struct zl3073x_out {
    // Config
    pub div: u32,
    pub width: u32,
    pub esync_n_period: u32,
    pub esync_n_width: u32,
    pub phase_comp: i32,
    pub mode: u8,
    // Invariants
    pub ctrl: u8,
}

extern "C" {
    pub fn zl3073x_out_state_fetch(zldev: *mut zl3073x_dev, index: u8) -> c_int;
    pub fn zl3073x_out_state_get(
        zldev: *mut zl3073x_dev,
        index: u8,
    ) -> *const zl3073x_out;
    pub fn zl3073x_out_state_set(
        zldev: *mut zl3073x_dev,
        index: u8,
        out: *const zl3073x_out,
    ) -> c_int;
}

#[inline]
unsafe fn field_get(mask: u8, value: u8) -> u8 {
    (value & mask) >> mask.trailing_zeros()
}

#[inline]
unsafe fn field_modify(mask: u8, value: *mut u8, replacement: u8) {
    *value = (*value & !mask) | ((replacement << mask.trailing_zeros()) & mask);
}

/** Return the clock type of the given output. */
#[inline]
pub unsafe fn zl3073x_out_clock_type_get(out: *const zl3073x_out) -> u8 {
    field_get(ZL_OUTPUT_MODE_CLOCK_TYPE, (*out).mode)
}

/** Set the clock type of the given output. */
#[inline]
pub unsafe fn zl3073x_out_clock_type_set(out: *mut zl3073x_out, type_: u8) {
    field_modify(ZL_OUTPUT_MODE_CLOCK_TYPE, &mut (*out).mode, type_);
}

/** Return the signal format of the given output. */
#[inline]
pub unsafe fn zl3073x_out_signal_format_get(out: *const zl3073x_out) -> u8 {
    field_get(ZL_OUTPUT_MODE_SIGNAL_FORMAT, (*out).mode)
}

/** Return true if the given output is differential. */
#[inline]
pub unsafe fn zl3073x_out_is_diff(out: *const zl3073x_out) -> bool {
    match zl3073x_out_signal_format_get(out) {
        ZL_OUTPUT_MODE_SIGNAL_FORMAT_LVDS
        | ZL_OUTPUT_MODE_SIGNAL_FORMAT_DIFF
        | ZL_OUTPUT_MODE_SIGNAL_FORMAT_LOWVCM => true,
        _ => false,
    }
}

/** Return true if the given output is enabled. */
#[inline]
pub unsafe fn zl3073x_out_is_enabled(out: *const zl3073x_out) -> bool {
    field_get(ZL_OUTPUT_CTRL_EN, (*out).ctrl) != 0
}

/** Return true if the given output is in N-div mode. */
#[inline]
pub unsafe fn zl3073x_out_is_ndiv(out: *const zl3073x_out) -> bool {
    match zl3073x_out_signal_format_get(out) {
        ZL_OUTPUT_MODE_SIGNAL_FORMAT_2_NDIV
        | ZL_OUTPUT_MODE_SIGNAL_FORMAT_2_NDIV_INV => true,
        _ => false,
    }
}

/** Return the index of the synth connected to the given output. */
#[inline]
pub unsafe fn zl3073x_out_synth_get(out: *const zl3073x_out) -> u8 {
    field_get(ZL_OUTPUT_CTRL_SYNTH_SEL, (*out).ctrl)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
