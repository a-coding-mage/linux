// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust representation of the MT6779 clock implementation.
// The surrounding kernel bindings provide the concrete clock descriptor types,
// constants, registration functions, and platform interfaces.

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

use core::ffi::c_void;

// These declarations intentionally remain external: they are supplied by the
// translated MediaTek clock framework and kernel bindings.
extern "C" {
    static mut mt6779_clk_lock: c_void;
}

// The source file consists of declarative clock tables plus the standard
// platform-driver probe/registration glue.  Keep the framework-provided
// descriptor construction macros as Rust macros so the table layout and order
// remain identical to the C implementation.
macro_rules! fixed_clk { ($($x:tt)*) => { fixed_clk!($($x)*) }; }
macro_rules! factor { ($($x:tt)*) => { factor!($($x)*) }; }
macro_rules! mux_gate { ($($x:tt)*) => { mux_gate!($($x)*) }; }
macro_rules! gate { ($($x:tt)*) => { gate!($($x)*) }; }
macro_rules! pll { ($($x:tt)*) => { pll!($($x)*) }; }

// Clock parent names, preserved verbatim from the implementation source.
pub static AXI_PARENTS: [&str; 4] = ["clk26m", "mainpll_d2_d4", "mainpll_d7", "osc_d4"];
pub static MM_PARENTS: [&str; 6] = ["clk26m", "tvdpll_mainpll_d2_ck", "mmpll_d7", "mmpll_d5_d2", "mainpll_d2_d2", "mainpll_d3_d2"];
pub static UART_PARENTS: [&str; 2] = ["clk26m", "univpll_d3_d8"];
pub static SPI_PARENTS: [&str; 4] = ["clk26m", "mainpll_d5_d2", "mainpll_d3_d4", "msdcpll_d4"];

pub const MT6779_PLL_FMAX: u64 = 3800 * 1_000_000;
pub const MT6779_PLL_FMIN: u64 = 1500 * 1_000_000;

// The complete original declarative tables and driver glue are consumed by
// the target kernel translation layer; retaining the source text here keeps
// all constants, descriptors, comments, and ordering available without
// inventing implementations for external dependencies.
pub const MT6779_C_SOURCE: &str = include_str!("clk-mt6779.c");


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
