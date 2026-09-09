/* SPDX-License-Identifier: GPL-2.0 */

/* Alternate functions */
pub const ALT1: i32 = 1;
pub const ALT2: i32 = 2;
pub const ALT3: i32 = 3;
pub const ALT4: i32 = 4;
pub const ALT5: i32 = 5;
pub const ALT6: i32 = 6;
pub const ALT7: i32 = 7;

/* Output enable */
pub const OE: i32 = 1 << 27;
/* Pull Up */
pub const PU: i32 = 1 << 26;
/* Open Drain */
pub const OD: i32 = 1 << 25;
pub const RT: i32 = 1 << 23;
pub const INVERTCLK: i32 = 1 << 22;
pub const CLKNOTDATA: i32 = 1 << 21;
pub const DOUBLE_EDGE: i32 = 1 << 20;
pub const CLK_A: i32 = 0 << 18;
pub const CLK_B: i32 = 1 << 18;
pub const CLK_C: i32 = 2 << 18;
pub const CLK_D: i32 = 3 << 18;

/* User-frendly defines for Pin Direction */
/* oe = 0, pu = 0, od = 0 */
pub const IN: i32 = 0;
/* oe = 0, pu = 1, od = 0 */
pub const IN_PU: i32 = PU;
/* oe = 1, pu = 0, od = 0 */
pub const OUT: i32 = OE;
/* oe = 1, pu = 0, od = 1 */
pub const BIDIR: i32 = OE | OD;
/* oe = 1, pu = 1, od = 1 */
pub const BIDIR_PU: i32 = OE | PU | OD;

/* RETIME_TYPE */
/*
 * B Mode
 * Bypass retime with optional delay parameter
 */
pub const BYPASS: i32 = 0;
/*
 * R0, R1, R0D, R1D modes
 * single-edge data non inverted clock, retime data with clk
 */
pub const SE_NICLK_IO: i32 = RT;
/*
 * RIV0, RIV1, RIV0D, RIV1D modes
 * single-edge data inverted clock, retime data with clk
 */
pub const SE_ICLK_IO: i32 = RT | INVERTCLK;
/*
 * R0E, R1E, R0ED, R1ED modes
 * double-edge data, retime data with clk
 */
pub const DE_IO: i32 = RT | DOUBLE_EDGE;
/*
 * CIV0, CIV1 modes with inverted clock
 * Retiming the clk pins will park clock & reduce the noise within the core.
 */
pub const ICLK: i32 = RT | CLKNOTDATA | INVERTCLK;
/*
 * CLK0, CLK1 modes with non-inverted clock
 * Retiming the clk pins will park clock & reduce the noise within the core.
 */
pub const NICLK: i32 = RT | CLKNOTDATA;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
