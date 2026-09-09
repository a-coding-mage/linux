/* SPDX-License-Identifier: GPL-2.0 */
/*
 * cs5536 mfgpt header file
 */

// Dependencies supplied by the corresponding CS5536 headers are intentionally
// left external to this translation.

// CONFIG_CS5536_MFGPT selects the external timer-control declarations.
#[cfg(feature = "CONFIG_CS5536_MFGPT")]
unsafe extern "C" {
    pub fn setup_mfgpt0_timer();
    pub fn disable_mfgpt0_counter();
    pub fn enable_mfgpt0_counter();
}

// Equivalent of the !CONFIG_CS5536_MFGPT static inline no-op functions.
#[cfg(not(feature = "CONFIG_CS5536_MFGPT"))]
#[inline]
pub fn setup_mfgpt0_timer() {}

#[cfg(not(feature = "CONFIG_CS5536_MFGPT"))]
#[inline]
pub fn disable_mfgpt0_counter() {}

#[cfg(not(feature = "CONFIG_CS5536_MFGPT"))]
#[inline]
pub fn enable_mfgpt0_counter() {}

pub const MFGPT_TICK_RATE: i32 = 14318000;

// HZ is supplied externally, as in the original macro.
#[macro_export]
macro_rules! COMPARE {
    () => {
        ($crate::MFGPT_TICK_RATE + HZ / 2) / HZ
    };
}

// mfgpt_base is supplied externally, as in the original macros.
#[macro_export]
macro_rules! MFGPT_BASE {
    () => {
        mfgpt_base
    };
}

#[macro_export]
macro_rules! MFGPT0_CMP2 {
    () => {
        MFGPT_BASE!() + 2
    };
}

#[macro_export]
macro_rules! MFGPT0_CNT {
    () => {
        MFGPT_BASE!() + 4
    };
}

#[macro_export]
macro_rules! MFGPT0_SETUP {
    () => {
        MFGPT_BASE!() + 6
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
