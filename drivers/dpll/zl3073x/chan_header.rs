/* SPDX-License-Identifier: GPL-2.0-only */

// The Linux headers and regs.h provide the integer aliases, bit-field
// definitions, and external types referenced by this declaration header.

#[repr(C)]
pub struct ptp_system_timestamp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct zl3073x_dev {
    _private: [u8; 0],
}

/**
 * struct zl3073x_chan - DPLL channel state
 * @ctrl: DPLL control register value
 * @mode_refsel: mode and reference selection register value
 * @ref_prio: reference priority registers (4 bits per ref, P/N packed)
 * @mon_status: monitor status register value
 * @refsel_status: reference selection status register value
 * @df_offset: frequency offset vs tracked reference in 2^-48 steps
 */
#[repr(C)]
pub struct zl3073x_chan {
    pub ctrl: u8,
    pub mode_refsel: u8,
    pub ref_prio: [u8; ZL3073X_NUM_REFS / 2],
    pub mon_status: u8,
    pub refsel_status: u8,
    pub df_offset: i64,
}

extern "C" {
    pub fn zl3073x_chan_state_fetch(zldev: *mut zl3073x_dev, index: u8) -> i32;
    pub fn zl3073x_chan_state_get(zldev: *mut zl3073x_dev, index: u8) -> *const zl3073x_chan;
    pub fn zl3073x_chan_state_set(
        zldev: *mut zl3073x_dev,
        index: u8,
        chan: *const zl3073x_chan,
    ) -> i32;

    pub fn zl3073x_chan_state_update(zldev: *mut zl3073x_dev, index: u8) -> i32;
    pub fn zl3073x_chan_nco_mode_set(zldev: *mut zl3073x_dev, index: u8) -> i32;

    pub fn zl3073x_chan_tod_ready_wait(zldev: *mut zl3073x_dev, ch: u8) -> i32;
    pub fn zl3073x_chan_tod_read(
        zldev: *mut zl3073x_dev,
        ch: u8,
        next_hz: bool,
        ts: *mut timespec64,
        sts: *mut ptp_system_timestamp,
    ) -> i32;
    pub fn zl3073x_chan_tod_write(zldev: *mut zl3073x_dev, ch: u8, ts: timespec64) -> i32;
    pub fn zl3073x_chan_tod_adjust(zldev: *mut zl3073x_dev, ch: u8, delta: timespec64) -> i32;
    pub fn zl3073x_chan_phase_step(
        zldev: *mut zl3073x_dev,
        ch: u8,
        out_mask: u16,
        step_cycles: i32,
        tod_step: bool,
    ) -> i32;

    pub fn zl3073x_chan_df_offset_set(zldev: *mut zl3073x_dev, ch: u8, offset: i64) -> i32;
    pub fn zl3073x_chan_tie_write(zldev: *mut zl3073x_dev, ch: u8, delta_ns: i64) -> i32;
}

#[repr(C)]
pub struct timespec64 {
    _private: [u8; 0],
}

#[inline]
pub unsafe fn zl3073x_chan_df_offset_get(chan: *const zl3073x_chan) -> i64 {
    (*chan).df_offset
}

#[inline]
pub unsafe fn zl3073x_chan_mode_get(chan: *const zl3073x_chan) -> u8 {
    FIELD_GET!(ZL_DPLL_MODE_REFSEL_MODE, (*chan).mode_refsel)
}

#[inline]
pub unsafe fn zl3073x_chan_ref_get(chan: *const zl3073x_chan) -> u8 {
    FIELD_GET!(ZL_DPLL_MODE_REFSEL_REF, (*chan).mode_refsel)
}

#[inline]
pub unsafe fn zl3073x_chan_mode_set(chan: *mut zl3073x_chan, mode: u8) {
    FIELD_MODIFY!(ZL_DPLL_MODE_REFSEL_MODE, &mut (*chan).mode_refsel, mode);
}

#[inline]
pub unsafe fn zl3073x_chan_ref_set(chan: *mut zl3073x_chan, r#ref: u8) {
    FIELD_MODIFY!(ZL_DPLL_MODE_REFSEL_REF, &mut (*chan).mode_refsel, r#ref);
}

#[inline]
pub unsafe fn zl3073x_chan_ref_prio_get(chan: *const zl3073x_chan, r#ref: u8) -> u8 {
    let val = (*chan).ref_prio[(r#ref / 2) as usize];
    if r#ref & 1 == 0 {
        FIELD_GET!(ZL_DPLL_REF_PRIO_REF_P, val)
    } else {
        FIELD_GET!(ZL_DPLL_REF_PRIO_REF_N, val)
    }
}

#[inline]
pub unsafe fn zl3073x_chan_ref_prio_set(chan: *mut zl3073x_chan, r#ref: u8, prio: u8) {
    let val = &mut (*chan).ref_prio[(r#ref / 2) as usize];
    if r#ref & 1 == 0 {
        FIELD_MODIFY!(ZL_DPLL_REF_PRIO_REF_P, val, prio);
    } else {
        FIELD_MODIFY!(ZL_DPLL_REF_PRIO_REF_N, val, prio);
    }
}

#[inline]
pub unsafe fn zl3073x_chan_ref_is_selectable(chan: *const zl3073x_chan, r#ref: u8) -> bool {
    zl3073x_chan_ref_prio_get(chan, r#ref) != ZL_DPLL_REF_PRIO_NONE
}

#[inline]
pub unsafe fn zl3073x_chan_lock_state_get(chan: *const zl3073x_chan) -> u8 {
    FIELD_GET!(ZL_DPLL_MON_STATUS_STATE, (*chan).mon_status)
}

#[inline]
pub unsafe fn zl3073x_chan_is_locked(chan: *const zl3073x_chan) -> bool {
    zl3073x_chan_lock_state_get(chan) == ZL_DPLL_MON_STATUS_STATE_LOCK
}

#[inline]
pub unsafe fn zl3073x_chan_mode_is_auto(chan: *const zl3073x_chan) -> bool {
    zl3073x_chan_mode_get(chan) == ZL_DPLL_MODE_REFSEL_MODE_AUTO
}

#[inline]
pub unsafe fn zl3073x_chan_mode_is_nco(chan: *const zl3073x_chan) -> bool {
    zl3073x_chan_mode_get(chan) == ZL_DPLL_MODE_REFSEL_MODE_NCO
}

#[inline]
pub unsafe fn zl3073x_chan_mode_is_reflock(chan: *const zl3073x_chan) -> bool {
    zl3073x_chan_mode_get(chan) == ZL_DPLL_MODE_REFSEL_MODE_REFLOCK
}

#[inline]
pub unsafe fn zl3073x_chan_mode_supports_tie(chan: *const zl3073x_chan) -> bool {
    zl3073x_chan_mode_is_auto(chan) || zl3073x_chan_mode_is_reflock(chan)
}

#[inline]
pub unsafe fn zl3073x_chan_is_ho_ready(chan: *const zl3073x_chan) -> bool {
    FIELD_GET!(ZL_DPLL_MON_STATUS_HO_READY, (*chan).mon_status) != 0
}

#[inline]
pub unsafe fn zl3073x_chan_refsel_state_get(chan: *const zl3073x_chan) -> u8 {
    FIELD_GET!(ZL_DPLL_REFSEL_STATUS_STATE, (*chan).refsel_status)
}

#[inline]
pub unsafe fn zl3073x_chan_refsel_ref_get(chan: *const zl3073x_chan) -> u8 {
    FIELD_GET!(ZL_DPLL_REFSEL_STATUS_REFSEL, (*chan).refsel_status)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
