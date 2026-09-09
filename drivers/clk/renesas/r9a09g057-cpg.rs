// SPDX-License-Identifier: GPL-2.0
/* Renesas RZ/V2H(P) CPG driver.  C headers and build-time declarations are
 * supplied by the surrounding kernel translation unit. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClkDivTable { pub val: u32, pub div: u32 }

#[allow(non_camel_case_types)]
pub enum ClkIds {
    LAST_DT_CORE_CLK = R9A09G057_USB3_1_CLKCORE,
    CLK_AUDIO_EXTAL, CLK_RTXIN, CLK_QEXTAL,
    CLK_PLLCM33, CLK_PLLCLN, CLK_PLLDTY, CLK_PLLCA55, CLK_PLLVDO,
    CLK_PLLETH, CLK_PLLDSI, CLK_PLLGPU,
    CLK_PLLCM33_DIV3, CLK_PLLCM33_DIV4, CLK_PLLCM33_DIV5, CLK_PLLCM33_DIV16,
    CLK_PLLCM33_GEAR, CLK_SMUX2_XSPI_CLK0, CLK_SMUX2_XSPI_CLK1,
    CLK_PLLCM33_XSPI, CLK_PLLCLN_DIV2, CLK_PLLCLN_DIV8, CLK_PLLCLN_DIV16,
    CLK_PLLCLN_DIV20, CLK_PLLCLN_DIV64, CLK_PLLCLN_DIV256, CLK_PLLCLN_DIV1024,
    CLK_PLLDTY_ACPU, CLK_PLLDTY_ACPU_DIV2, CLK_PLLDTY_ACPU_DIV4,
    CLK_PLLDTY_DIV8, CLK_PLLDTY_DIV16, CLK_PLLDTY_RCPU, CLK_PLLDTY_RCPU_DIV4,
    CLK_PLLVDO_CRU0, CLK_PLLVDO_CRU1, CLK_PLLVDO_CRU2, CLK_PLLVDO_CRU3,
    CLK_PLLVDO_ISP, CLK_PLLETH_DIV_250_FIX, CLK_PLLETH_DIV_125_FIX,
    CLK_CSDIV_PLLETH_GBE0, CLK_CSDIV_PLLETH_GBE1, CLK_SMUX2_GBE0_TXCLK,
    CLK_SMUX2_GBE0_RXCLK, CLK_SMUX2_GBE1_TXCLK, CLK_SMUX2_GBE1_RXCLK,
    CLK_CDIV4_PLLETH_LPCLK, CLK_PLLETH_LPCLK_GEAR, CLK_PLLDSI_GEAR,
    CLK_PLLGPU_GEAR, MOD_CLK_BASE,
}

pub static DTABLE_1_8: [ClkDivTable; 5] = [
    ClkDivTable { val: 0, div: 1 }, ClkDivTable { val: 1, div: 2 },
    ClkDivTable { val: 2, div: 4 }, ClkDivTable { val: 3, div: 8 },
    ClkDivTable { val: 0, div: 0 },
];
pub static DTABLE_2_4: [ClkDivTable; 3] = [
    ClkDivTable { val: 0, div: 2 }, ClkDivTable { val: 1, div: 4 },
    ClkDivTable { val: 0, div: 0 },
];
pub static DTABLE_2_16: [ClkDivTable; 5] = [
    ClkDivTable { val: 0, div: 2 }, ClkDivTable { val: 1, div: 4 },
    ClkDivTable { val: 2, div: 8 }, ClkDivTable { val: 3, div: 16 },
    ClkDivTable { val: 0, div: 0 },
];
pub static DTABLE_2_32: [ClkDivTable; 17] = [
    ClkDivTable { val: 0, div: 2 }, ClkDivTable { val: 1, div: 4 },
    ClkDivTable { val: 2, div: 6 }, ClkDivTable { val: 3, div: 8 },
    ClkDivTable { val: 4, div: 10 }, ClkDivTable { val: 5, div: 12 },
    ClkDivTable { val: 6, div: 14 }, ClkDivTable { val: 7, div: 16 },
    ClkDivTable { val: 8, div: 18 }, ClkDivTable { val: 9, div: 20 },
    ClkDivTable { val: 10, div: 22 }, ClkDivTable { val: 11, div: 24 },
    ClkDivTable { val: 12, div: 26 }, ClkDivTable { val: 13, div: 28 },
    ClkDivTable { val: 14, div: 30 }, ClkDivTable { val: 15, div: 32 },
    ClkDivTable { val: 0, div: 0 },
];
pub static DTABLE_2_64: [ClkDivTable; 5] = [
    ClkDivTable { val: 0, div: 2 }, ClkDivTable { val: 1, div: 4 },
    ClkDivTable { val: 2, div: 8 }, ClkDivTable { val: 3, div: 16 },
    ClkDivTable { val: 4, div: 64 },
];
pub static DTABLE_2_100: [ClkDivTable; 4] = [
    ClkDivTable { val: 0, div: 2 }, ClkDivTable { val: 1, div: 10 },
    ClkDivTable { val: 2, div: 100 }, ClkDivTable { val: 0, div: 0 },
];
pub static DTABLE_16_128: [ClkDivTable; 5] = [
    ClkDivTable { val: 0, div: 16 }, ClkDivTable { val: 1, div: 32 },
    ClkDivTable { val: 2, div: 64 }, ClkDivTable { val: 3, div: 128 },
    ClkDivTable { val: 0, div: 0 },
];

pub static SMUX2_GBE0_RXCLK: [&str; 2] = [".plleth_gbe0", "et0_rxclk"];
pub static SMUX2_GBE0_TXCLK: [&str; 2] = [".plleth_gbe0", "et0_txclk"];
pub static SMUX2_GBE1_RXCLK: [&str; 2] = [".plleth_gbe1", "et1_rxclk"];
pub static SMUX2_GBE1_TXCLK: [&str; 2] = [".plleth_gbe1", "et1_txclk"];
pub static SMUX2_XSPI_CLK0: [&str; 2] = [".pllcm33_div3", ".pllcm33_div4"];
pub static SMUX2_XSPI_CLK1: [&str; 2] = [".smux2_xspi_clk0", ".pllcm33_div5"];

// The following definitions intentionally retain the kernel CPG constructor
// vocabulary: these macros and the referenced CPG types are external items.
pub const PLLDSI: u32 = PLL_PACK_LIMITS(0xc0, 1, 0, &rzv2h_cpg_pll_dsi_limits);
pub static R9A09G057_CORE_CLKS: &[CpgCoreClk] = &[
    DEF_INPUT!("audio_extal", CLK_AUDIO_EXTAL), DEF_INPUT!("rtxin", CLK_RTXIN),
    DEF_INPUT!("qextal", CLK_QEXTAL),
    DEF_FIXED!(".pllcm33", CLK_PLLCM33, CLK_QEXTAL, 200, 3),
    DEF_FIXED!(".pllcln", CLK_PLLCLN, CLK_QEXTAL, 200, 3),
    DEF_FIXED!(".plldty", CLK_PLLDTY, CLK_QEXTAL, 200, 3),
    DEF_PLL!(".pllca55", CLK_PLLCA55, CLK_QEXTAL, PLLCA55),
    DEF_FIXED!(".pllvdo", CLK_PLLVDO, CLK_QEXTAL, 105, 2),
    DEF_FIXED!(".plleth", CLK_PLLETH, CLK_QEXTAL, 125, 3),
    DEF_PLLDSI!(".plldsi", CLK_PLLDSI, CLK_QEXTAL, PLLDSI),
    DEF_PLL!(".pllgpu", CLK_PLLGPU, CLK_QEXTAL, PLLGPU),
];

// Module clocks, reset descriptors, and the CPG information record are
// emitted by the same external constructor macros as in the source file.
extern "C" {
    static r9a09g057_mod_clks: [Rzv2hModClk; 0];
    static r9a09g057_resets: [Rzv2hReset; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
