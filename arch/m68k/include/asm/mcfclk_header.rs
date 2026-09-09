/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mcfclk.h -- coldfire specific clock structure
 */

use core::ffi::c_ulong;

#[repr(C)]
pub struct clk_ops {
    pub enable: Option<unsafe extern "C" fn(*mut clk)>,
    pub disable: Option<unsafe extern "C" fn(*mut clk)>,
}

#[repr(C)]
pub struct clk {
    pub clk_ops: *mut clk_ops,
    pub rate: c_ulong,
    pub enabled: c_ulong,
    pub slot: u8,
}

/* The following declarations are present when MCFPM_PPMCR0 is defined. */
#[cfg(MCFPM_PPMCR0)]
unsafe extern "C" {
    pub static mut clk_ops0: clk_ops;
    pub static mut clk_ops2: clk_ops;
}

#[cfg(all(MCFPM_PPMCR0, MCFPM_PPMCR1))]
unsafe extern "C" {
    pub static mut clk_ops1: clk_ops;
}

/*
 * C token pasting in DEFINE_CLK produces __clk_<bank>_<slot>. Rust
 * macro_rules! cannot concatenate identifiers without an external facility;
 * the final identifier is therefore supplied explicitly.
 */
#[cfg(MCFPM_PPMCR0)]
#[macro_export]
macro_rules! DEFINE_CLK {
    ($clk_bank:ident, $clk_name:ident, $clk_slot:expr, $clk_rate:expr, $clk_ident:ident) => {
        static mut $clk_ident: $crate::clk = $crate::clk {
            clk_ops: unsafe { &raw mut $crate::clk_ops0 },
            rate: $clk_rate,
            enabled: 0,
            slot: $clk_slot,
        };
    };
}

#[cfg(MCFPM_PPMCR0)]
unsafe extern "C" {
    pub fn __clk_init_enabled(clk: *mut clk);
    pub fn __clk_init_disabled(clk: *mut clk);
}

/* When MCFPM_PPMCR0 is not defined, DEFINE_CLK creates a rate-only clock. */
#[cfg(not(MCFPM_PPMCR0))]
#[macro_export]
macro_rules! DEFINE_CLK {
    ($clk_ref:ident, $clk_name:ident, $clk_rate:expr, $clk_ident:ident) => {
        static mut $clk_ident: $crate::clk = $crate::clk {
            clk_ops: core::ptr::null_mut(),
            rate: $clk_rate,
            enabled: 0,
            slot: 0,
        };
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
