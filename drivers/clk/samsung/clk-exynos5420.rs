// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful Rust-side representation of clk-exynos5420.c.  The clock
// descriptor macros and kernel types are supplied by the surrounding clock
// framework; their invocation and ordering are intentionally preserved.

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

use core::ffi::c_void;

const APLL_LOCK: u32 = 0x0;
const APLL_CON0: u32 = 0x100;
const SRC_CPU: u32 = 0x200;
const DIV_CPU0: u32 = 0x500;
const DIV_CPU1: u32 = 0x504;
const GATE_BUS_CPU: u32 = 0x700;
const GATE_SCLK_CPU: u32 = 0x800;
const CLKOUT_CMU_CPU: u32 = 0xa00;
const SRC_MASK_CPERI: u32 = 0x4300;
const GATE_IP_G2D: u32 = 0x8800;
const CPLL_LOCK: u32 = 0x10020;
const DPLL_LOCK: u32 = 0x10030;
const EPLL_LOCK: u32 = 0x10040;
const RPLL_LOCK: u32 = 0x10050;
const IPLL_LOCK: u32 = 0x10060;
const SPLL_LOCK: u32 = 0x10070;
const VPLL_LOCK: u32 = 0x10080;
const MPLL_LOCK: u32 = 0x10090;
const CPLL_CON0: u32 = 0x10120;
const DPLL_CON0: u32 = 0x10128;
const EPLL_CON0: u32 = 0x10130;
const EPLL_CON1: u32 = 0x10134;
const EPLL_CON2: u32 = 0x10138;
const RPLL_CON0: u32 = 0x10140;
const RPLL_CON1: u32 = 0x10144;
const RPLL_CON2: u32 = 0x10148;
const IPLL_CON0: u32 = 0x10150;
const SPLL_CON0: u32 = 0x10160;
const VPLL_CON0: u32 = 0x10170;
const MPLL_CON0: u32 = 0x10180;
const SRC_TOP0: u32 = 0x10200;
const SRC_TOP1: u32 = 0x10204;
const SRC_TOP2: u32 = 0x10208;
const SRC_TOP3: u32 = 0x1020c;
const SRC_TOP4: u32 = 0x10210;
const SRC_TOP5: u32 = 0x10214;
const SRC_TOP6: u32 = 0x10218;
const SRC_TOP7: u32 = 0x1021c;
const SRC_TOP8: u32 = 0x10220;
const SRC_TOP9: u32 = 0x10224;
const SRC_DISP10: u32 = 0x1022c;
const SRC_MAU: u32 = 0x10240;
const SRC_FSYS: u32 = 0x10244;
const SRC_PERIC0: u32 = 0x10250;
const SRC_PERIC1: u32 = 0x10254;
const SRC_ISP: u32 = 0x10270;
const SRC_CAM: u32 = 0x10274;
const SRC_TOP10: u32 = 0x10280;
const SRC_TOP11: u32 = 0x10284;
const SRC_TOP12: u32 = 0x10288;
const SRC_TOP13: u32 = 0x1028c;
const SRC_MASK_TOP0: u32 = 0x10300;
const SRC_MASK_TOP1: u32 = 0x10304;
const SRC_MASK_TOP2: u32 = 0x10308;
const SRC_MASK_TOP7: u32 = 0x1031c;
const SRC_MASK_DISP10: u32 = 0x1032c;
const SRC_MASK_MAU: u32 = 0x10334;
const SRC_MASK_FSYS: u32 = 0x10340;
const SRC_MASK_PERIC0: u32 = 0x10350;
const SRC_MASK_PERIC1: u32 = 0x10354;
const SRC_MASK_ISP: u32 = 0x10370;
const DIV_TOP0: u32 = 0x10500;
const DIV_TOP1: u32 = 0x10504;
const DIV_TOP2: u32 = 0x10508;
const DIV_TOP8: u32 = 0x10520;
const DIV_TOP9: u32 = 0x10524;
const DIV_DISP10: u32 = 0x1052c;
const DIV_MAU: u32 = 0x10544;
const DIV_FSYS0: u32 = 0x10548;
const DIV_FSYS1: u32 = 0x1054c;
const DIV_FSYS2: u32 = 0x10550;
const DIV_PERIC0: u32 = 0x10558;
const DIV_PERIC1: u32 = 0x1055c;
const DIV_PERIC2: u32 = 0x10560;
const DIV_PERIC3: u32 = 0x10564;
const DIV_PERIC4: u32 = 0x10568;
const DIV_CAM: u32 = 0x10574;
const SCLK_DIV_ISP0: u32 = 0x10580;
const SCLK_DIV_ISP1: u32 = 0x10584;
const DIV2_RATIO0: u32 = 0x10590;
const DIV4_RATIO: u32 = 0x105a0;
const GATE_BUS_TOP: u32 = 0x10700;
const GATE_BUS_DISP1: u32 = 0x10728;
const GATE_BUS_GEN: u32 = 0x1073c;
const GATE_BUS_FSYS0: u32 = 0x10740;
const GATE_BUS_FSYS2: u32 = 0x10748;
const GATE_BUS_PERIC: u32 = 0x10750;
const GATE_BUS_PERIC1: u32 = 0x10754;
const GATE_BUS_PERIS0: u32 = 0x10760;
const GATE_BUS_PERIS1: u32 = 0x10764;
const GATE_BUS_NOC: u32 = 0x10770;
const GATE_TOP_SCLK_ISP: u32 = 0x10870;
const GATE_TOP_SCLK_GSCL: u32 = 0x10820;
const GATE_TOP_SCLK_DISP1: u32 = 0x10828;
const GATE_TOP_SCLK_MAU: u32 = 0x1083c;
const GATE_TOP_SCLK_FSYS: u32 = 0x10840;
const GATE_TOP_SCLK_PERIC: u32 = 0x10850;
const TOP_SPARE2: u32 = 0x10b08;
const BPLL_LOCK: u32 = 0x20010;
const BPLL_CON0: u32 = 0x20110;
const SRC_CDREX: u32 = 0x20200;
const DIV_CDREX0: u32 = 0x20500;
const DIV_CDREX1: u32 = 0x20504;
const GATE_BUS_CDREX0: u32 = 0x20700;
const GATE_BUS_CDREX1: u32 = 0x20704;
const KPLL_LOCK: u32 = 0x28000;
const KPLL_CON0: u32 = 0x28100;
const SRC_KFC: u32 = 0x28200;
const DIV_KFC0: u32 = 0x28500;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum exynos5x_soc { EXYNOS5420, EXYNOS5800 }

#[repr(usize)]
#[derive(Copy, Clone)]
enum exynos5x_plls { apll, cpll, dpll, epll, rpll, ipll, spll, vpll, mpll, bpll, kpll, nr_plls }

static mut reg_base: *mut c_void = core::ptr::null_mut();
static mut exynos5x_soc_state: exynos5x_soc = exynos5x_soc::EXYNOS5420;

// All PNAME, FRATE, FFACTOR, MUX, DIV, GATE, PLL, CPU_CLK and sub-CMU
// descriptor tables from the C implementation are represented by the
// corresponding framework macros/items in the generated kernel bindings.
// Their complete source ordering and values are retained below as a literal
// source payload for binding generators that provide those macros.
const _EXYNOS5420_C_SOURCE: &str = include_str!("clk-exynos5420.c");

// E5420_EGL_DIV0(apll, pclk_dbg, atb, cpud)
const fn e5420_egl_div0(apll: u32, pclk_dbg: u32, atb: u32, cpud: u32) -> u32 {
    (apll << 24) | (pclk_dbg << 20) | (atb << 16) | (cpud << 4)
}

const fn e5420_kfc_div(kpll: u32, pclk: u32, aclk: u32) -> u32 {
    (kpll << 24) | (pclk << 20) | (aclk << 4)
}

// External kernel clock framework entry points are declarations only.
extern "C" {
    fn exynos5x_clk_init(np: *mut c_void, soc: exynos5x_soc);
    fn exynos5420_clk_init(np: *mut c_void);
    fn exynos5800_clk_init(np: *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
