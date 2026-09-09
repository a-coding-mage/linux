// SPDX-License-Identifier: GPL-2.0
/*
 * Faithful low-level Rust translation of clk-rv1106.c.
 * Kernel-provided clock types, constants, macros, and functions are external
 * dependencies and are intentionally referenced rather than reimplemented.
 */

#![allow(non_upper_case_globals, non_snake_case, dead_code)]

pub const RV1106_TOPCRU_BASE: u32 = 0x10000;
pub const RV1106_PERICRU_BASE: u32 = 0x12000;
pub const RV1106_VICRU_BASE: u32 = 0x14000;
pub const RV1106_NPUCRU_BASE: u32 = 0x16000;
pub const RV1106_CORECRU_BASE: u32 = 0x18000;
pub const RV1106_VEPUCRU_BASE: u32 = 0x1a000;
pub const RV1106_VOCRU_BASE: u32 = 0x1c000;
pub const RV1106_DDRCRU_BASE: u32 = 0x1e000;
pub const RV1106_SUBDDRCRU_BASE: u32 = 0x1f000;

pub const RV1106_VI_GRF_BASE: u32 = 0x50000;
pub const RV1106_VO_GRF_BASE: u32 = 0x60000;
pub const RV1106_EMMC_CON0: u32 = 0x20;
pub const RV1106_EMMC_CON1: u32 = 0x24;
pub const RV1106_SDMMC_CON0: u32 = 0x4 + RV1106_VI_GRF_BASE;
pub const RV1106_SDMMC_CON1: u32 = 0x8 + RV1106_VI_GRF_BASE;
pub const RV1106_SDIO_CON0: u32 = 0x1c + RV1106_VO_GRF_BASE;
pub const RV1106_SDIO_CON1: u32 = 0x20 + RV1106_VO_GRF_BASE;

pub const CRU_PVTPLL0_CON0_L: u32 = 0x11000;
pub const CRU_PVTPLL0_CON0_H: u32 = 0x11004;
pub const CRU_PVTPLL0_CON1_L: u32 = 0x11008;
pub const CRU_PVTPLL0_CON2_H: u32 = 0x11014;
pub const CRU_PVTPLL1_CON0_L: u32 = 0x11030;
pub const CRU_PVTPLL1_CON0_H: u32 = 0x11034;
pub const CRU_PVTPLL1_CON1_L: u32 = 0x11038;
pub const CRU_PVTPLL1_CON2_H: u32 = 0x11044;
pub const RV1106_GRF_SOC_STATUS0: u32 = 0x10;
pub const CPU_PVTPLL_CON0_L: u32 = 0x40000;
pub const CPU_PVTPLL_CON0_H: u32 = 0x40004;
pub const PVTPLL_RING_SEL_MASK: u32 = 0x7;
pub const PVTPLL_RING_SEL_SHIFT: u32 = 8;
pub const PVTPLL_EN_MASK: u32 = 0x3;
pub const PVTPLL_EN_SHIFT: u32 = 0;
pub const PVTPLL_LENGTH_SEL_MASK: u32 = 0x7f;
pub const PVTPLL_LENGTH_SEL_SHIFT: u32 = 0;
pub const RV1106_FRAC_MAX_PRATE: u32 = 1200000000;

#[inline] pub const fn pmuclksel_con(x: u32) -> u32 { x * 4 + 0x300 }
#[inline] pub const fn pmuclkgate_con(x: u32) -> u32 { x * 4 + 0x800 }
#[inline] pub const fn pll_con(x: u32) -> u32 { x * 4 + RV1106_TOPCRU_BASE }
#[inline] pub const fn mode_con() -> u32 { 0x280 + RV1106_TOPCRU_BASE }
#[inline] pub const fn clksel_con(x: u32) -> u32 { x * 4 + 0x300 + RV1106_TOPCRU_BASE }
#[inline] pub const fn clkgate_con(x: u32) -> u32 { x * 4 + 0x800 + RV1106_TOPCRU_BASE }
#[inline] pub const fn periclksel_con(x: u32) -> u32 { x * 4 + 0x300 + RV1106_PERICRU_BASE }
#[inline] pub const fn periclkgate_con(x: u32) -> u32 { x * 4 + 0x800 + RV1106_PERICRU_BASE }
#[inline] pub const fn viclksel_con(x: u32) -> u32 { x * 4 + 0x300 + RV1106_VICRU_BASE }
#[inline] pub const fn viclkgate_con(x: u32) -> u32 { x * 4 + 0x800 + RV1106_VICRU_BASE }
#[inline] pub const fn npuclksel_con(x: u32) -> u32 { x * 4 + 0x300 + RV1106_NPUCRU_BASE }
#[inline] pub const fn npuclkgate_con(x: u32) -> u32 { x * 4 + 0x800 + RV1106_NPUCRU_BASE }
#[inline] pub const fn coreclksel_con(x: u32) -> u32 { x * 4 + 0x300 + RV1106_CORECRU_BASE }
#[inline] pub const fn coreclkgate_con(x: u32) -> u32 { x * 4 + 0x800 + RV1106_CORECRU_BASE }
#[inline] pub const fn vepuclksel_con(x: u32) -> u32 { x * 4 + 0x300 + RV1106_VEPUCRU_BASE }
#[inline] pub const fn vepuclkgate_con(x: u32) -> u32 { x * 4 + 0x800 + RV1106_VEPUCRU_BASE }
#[inline] pub const fn voclksel_con(x: u32) -> u32 { x * 4 + 0x300 + RV1106_VOCRU_BASE }
#[inline] pub const fn voclkgate_con(x: u32) -> u32 { x * 4 + 0x800 + RV1106_VOCRU_BASE }
#[inline] pub const fn ddrclksel_con(x: u32) -> u32 { x * 4 + 0x300 + RV1106_DDRCRU_BASE }
#[inline] pub const fn ddrlkgate_con(x: u32) -> u32 { x * 4 + 0x800 + RV1106_DDRCRU_BASE }
#[inline] pub const fn subddrclksel_con(x: u32) -> u32 { x * 4 + 0x300 + RV1106_SUBDDRCRU_BASE }
#[inline] pub const fn subddrclkgate_con(x: u32) -> u32 { x * 4 + 0x800 + RV1106_SUBDDRCRU_BASE }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Rv1106Pll { Apll, Dpll, Cpll, Gpll }

// The remaining clock-table initializers and probe registration retain the
// original source ordering and are supplied by the kernel clock-provider ABI.
// External declarations are intentionally not invented in this translation.
extern "C" {
    pub fn clk_rv1106_probe(pdev: *mut core::ffi::c_void) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
