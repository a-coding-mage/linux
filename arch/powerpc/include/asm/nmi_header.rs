/* SPDX-License-Identifier: GPL-2.0 */

// CONFIG_PPC_WATCHDOG controls whether the watchdog declarations are
// provided by the watchdog implementation or by the no-op inline fallback.
#[cfg(feature = "CONFIG_PPC_WATCHDOG")]
extern "C" {
    pub fn soft_nmi_interrupt(regs: *mut pt_regs) -> core::ffi::c_long;
    pub fn watchdog_hardlockup_set_timeout_pct(pct: u64);
}

#[cfg(not(feature = "CONFIG_PPC_WATCHDOG"))]
#[inline]
pub fn watchdog_hardlockup_set_timeout_pct(_pct: u64) {}

extern "C" {
    pub fn hv_nmi_check_nonrecoverable(regs: *mut pt_regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
