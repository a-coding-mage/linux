/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 1995-2004 Russell King
 *
 * Delay routines, using a pre-computed "loops_per_second" value.
 */

/* Dependencies supplied by the surrounding kernel translation. */

pub const MAX_UDELAY_MS: u64 = 2;
pub const UDELAY_MULT: u64 = 2147 * (HZ as u64) + 483648 * (HZ as u64) / 1_000_000;
pub const UDELAY_SHIFT: u32 = 31;

#[repr(C)]
pub struct delay_timer {
    pub read_current_timer: Option<unsafe extern "C" fn() -> libc::c_ulong>,
    pub freq: libc::c_ulong,
}

#[repr(C)]
pub struct arm_delay_ops {
    pub delay: Option<unsafe extern "C" fn(libc::c_ulong)>,
    pub const_udelay: Option<unsafe extern "C" fn(libc::c_ulong)>,
    pub udelay: Option<unsafe extern "C" fn(libc::c_ulong)>,
    pub ticks_per_jiffy: libc::c_ulong,
}

unsafe extern "C" {
    pub static mut arm_delay_ops: arm_delay_ops;

    /* This function intentionally does not exist for out-of-range delays. */
    pub fn __bad_udelay() -> !;

    pub fn __loop_delay(loops: libc::c_ulong);
    pub fn __loop_udelay(usecs: libc::c_ulong);
    pub fn __loop_const_udelay(value: libc::c_ulong);

    pub fn register_current_timer_delay(timer: *const delay_timer);
}

#[inline]
pub unsafe fn __delay(n: libc::c_ulong) {
    if let Some(delay) = (*(&raw mut arm_delay_ops)).delay {
        delay(n);
    }
}

#[inline]
pub unsafe fn __udelay(n: libc::c_ulong) {
    if let Some(udelay) = (*(&raw mut arm_delay_ops)).udelay {
        udelay(n);
    }
}

#[inline]
pub unsafe fn __const_udelay(n: libc::c_ulong) {
    if let Some(const_udelay) = (*(&raw mut arm_delay_ops)).const_udelay {
        const_udelay(n);
    }
}

/*
 * The C version selects __bad_udelay for a compile-time constant that is
 * outside the supported range; Rust has no direct equivalent of
 * __builtin_constant_p, so this preserves the same runtime decision.
 */
#[inline]
pub unsafe fn udelay(n: libc::c_ulong) {
    if n > MAX_UDELAY_MS * 1000 {
        __bad_udelay();
    } else {
        __const_udelay(n * UDELAY_MULT as libc::c_ulong);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
