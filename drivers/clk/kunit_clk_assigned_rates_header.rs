/* SPDX-License-Identifier: GPL-2.0 */

// Dependency equivalent of <dt-bindings/clock/clock.h> is supplied externally.

pub const ASSIGNED_RATES_0_RATE: u32 = 1_600_000;
pub const ASSIGNED_RATES_1_RATE: u32 = 9_700_000;

pub const ASSIGNED_SSCS_0_MODFREQ: u32 = 10_000;
pub const ASSIGNED_SSCS_0_SPREAD: u32 = 30_000;
pub const ASSIGNED_SSCS_0_METHOD: _ = CLK_SSC_CENTER_SPREAD;
pub const ASSIGNED_SSCS_1_MODFREQ: u32 = 20_000;
pub const ASSIGNED_SSCS_1_SPREAD: u32 = 40_000;
pub const ASSIGNED_SSCS_1_METHOD: _ = CLK_SSC_UP_SPREAD;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
