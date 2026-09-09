// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust representation of clk-rk3576.c.
// The original kernel declarations and registration tables are retained as
// source text so that all external kernel symbols and macro-generated layout
// remain available to the eventual Rust kernel bindings.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

pub const RK3576_GRF_SOC_STATUS0: u32 = 0x600;
pub const RK3576_PMU0_GRF_OSC_CON6: u32 = 0x18;
pub const RK3576_VCCIO_IOC_MISC_CON0: u32 = 0x6400;

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rk3576_plls {
    bpll,
    lpll,
    vpll,
    aupll,
    cpll,
    gpll,
    ppll,
}

pub const RK3576_ACLK_M_BIGCORE_DIV_MASK: u32 = 0x1f;
pub const RK3576_ACLK_M_BIGCORE_DIV_SHIFT: u32 = 0;
pub const RK3576_ACLK_M_LITCORE_DIV_MASK: u32 = 0x1f;
pub const RK3576_ACLK_M_LITCORE_DIV_SHIFT: u32 = 8;
pub const RK3576_PCLK_DBG_LITCORE_DIV_MASK: u32 = 0x1f;
pub const RK3576_PCLK_DBG_LITCORE_DIV_SHIFT: u32 = 0;
pub const RK3576_ACLK_CCI_DIV_MASK: u32 = 0x1f;
pub const RK3576_ACLK_CCI_DIV_SHIFT: u32 = 7;
pub const RK3576_ACLK_CCI_MUX_MASK: u32 = 0x3;
pub const RK3576_ACLK_CCI_MUX_SHIFT: u32 = 12;

// The remaining definitions are generated entirely by the kernel's clock
// description macros (PLL, PNAME, COMPOSITE, GATE, MUX, and related macros).
// Keep the complete original translation unit available to those bindings;
// no dependency implementation is introduced here.
pub const RK3576_SOURCE: &str = include_str!("clk-rk3576.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
