/* SPDX-License-Identifier: GPL-2.0 */
/*
 * win_minmax.h: windowed min/max tracker by Kathleen Nichols.
 *
 */

//! Original C layout and inline windowed tracker operations.
pub use kernel::bindings::{minmax, minmax_sample};

/// Returns the current best measurement.
///
/// # Safety
/// `m` must point to aligned tracker storage with readable, initialized `s[0].v`.
/// The other fields need not be initialized.
pub unsafe fn minmax_get(m: *const minmax) -> u32 {
    // SAFETY: the raw field projection reads only the initialized first value.
    unsafe { (*m).s[0].v }
}

/// Replaces all three samples with the new measurement.
///
/// # Safety
/// `m` must point to an aligned writable tracker, without concurrent access.
/// The old contents need not be initialized.
pub unsafe fn minmax_reset(m: *mut minmax, t: u32, meas: u32) -> u32 {
    // SAFETY: write does not read the possibly uninitialized old contents.
    unsafe { m.write(minmax { s: [minmax_sample { t, v: meas }; 3] }) };
    meas
}

extern "C" {
    /// Updates the running maximum; only fields read on the executed path
    /// need initialization in the aligned, exclusively accessible tracker.
    pub fn minmax_running_max(m: *mut minmax, win: u32, t: u32, meas: u32) -> u32;
    /// Updates the running minimum; only fields read on the executed path
    /// need initialization in the aligned, exclusively accessible tracker.
    pub fn minmax_running_min(m: *mut minmax, win: u32, t: u32, meas: u32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
