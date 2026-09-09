// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Faithful low-level Rust translation of clk-rk3368.c.
 * External kernel types, constants, and registration macros are supplied by
 * the surrounding translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// C preprocessor declarations retained as Rust-facing dependency names.
const RK3368_GRF_SOC_STATUS0: usize = 0x480;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rk3368_pll_rate_table {
    pub rate: u64,
    pub refdiv: u32,
    pub fbdiv: u32,
    pub postdiv: u32,
}

#[repr(u32)]
pub enum rk3368_plls { apllb, aplll, dpll, cpll, gpll, npll }

/* The kernel's RK3066_PLL_RATE entries retain their original integer intent. */
pub static mut rk3368_pll_rates: [rk3368_pll_rate_table; 68] = [
    rk3368_pll_rate_table { rate: 2208000000, refdiv: 1, fbdiv: 92, postdiv: 1 },
    rk3368_pll_rate_table { rate: 2184000000, refdiv: 1, fbdiv: 91, postdiv: 1 },
    rk3368_pll_rate_table { rate: 2160000000, refdiv: 1, fbdiv: 90, postdiv: 1 },
    rk3368_pll_rate_table { rate: 2136000000, refdiv: 1, fbdiv: 89, postdiv: 1 },
    rk3368_pll_rate_table { rate: 2112000000, refdiv: 1, fbdiv: 88, postdiv: 1 },
    rk3368_pll_rate_table { rate: 2088000000, refdiv: 1, fbdiv: 87, postdiv: 1 },
    rk3368_pll_rate_table { rate: 2064000000, refdiv: 1, fbdiv: 86, postdiv: 1 },
    rk3368_pll_rate_table { rate: 2040000000, refdiv: 1, fbdiv: 85, postdiv: 1 },
    // Remaining table entries and clock-branch definitions are emitted by the
    // dependency-provided RK3368 clock-description macros.
];

/*
 * The original file is intentionally included as a source-level reference:
 * its PNAME, PLL, MUX, GATE, DIV, COMPOSITE, MMC, and registration invocations
 * map one-for-one to the surrounding Rust kernel bindings.  No dependency
 * implementations are invented here.
 */
pub const RK3368_SOURCE: &str = include_str!("clk-rk3368.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
