// SPDX-License-Identifier: GPL-2.0
/* RZ/G2L CPG driver; direct Rust translation of r9a07g044-cpg.c. */

// External kernel types, constants, and constructor macros are supplied by
// the surrounding translation unit.

const CPG_PL2SDHI_DSEL: u32 = 0x218;
const SEL_SDHI0: _ = SEL_PLL_PACK(CPG_PL2SDHI_DSEL, 0, 2);
const SEL_SDHI1: _ = SEL_PLL_PACK(CPG_PL2SDHI_DSEL, 4, 2);
const SEL_SDHI0_STS: _ = SEL_PLL_PACK(CPG_CLKSTATUS, 28, 1);
const SEL_SDHI1_STS: _ = SEL_PLL_PACK(CPG_CLKSTATUS, 29, 1);

#[repr(usize)]
enum ClkIds {
    LAST_DT_CORE_CLK = R9A07G054_CLK_DRP_A as usize,
    CLK_EXTAL,
    CLK_OSC_DIV1000, CLK_PLL1, CLK_PLL2, CLK_PLL2_DIV2, CLK_PLL2_DIV2_8,
    CLK_PLL2_DIV2_10, CLK_PLL3, CLK_PLL3_400, CLK_PLL3_533, CLK_M2_DIV2,
    CLK_PLL3_DIV2, CLK_PLL3_DIV2_2, CLK_PLL3_DIV2_4, CLK_PLL3_DIV2_4_2,
    CLK_SEL_PLL3_3, CLK_DIV_PLL3_C, CLK_PLL4, CLK_PLL5,
    CLK_PLL5_FOUTPOSTDIV, CLK_PLL5_FOUT1PH0, CLK_PLL5_FOUT3, CLK_PLL5_250,
    CLK_PLL6, CLK_PLL6_250, CLK_P1_DIV2, CLK_PLL2_800, CLK_PLL2_SDHI_533,
    CLK_PLL2_SDHI_400, CLK_PLL2_SDHI_266, CLK_SD0_DIV4, CLK_SD1_DIV4,
    CLK_SEL_GPU2, CLK_SEL_PLL5_4, CLK_DSI_DIV, CLK_PLL2_533,
    CLK_PLL2_533_DIV2, CLK_DIV_DSI_LPCLK, MOD_CLK_BASE,
}

static dtable_1_8: [ClkDivTable; 5] = [
    ClkDivTable { val: 0, div: 1 }, ClkDivTable { val: 1, div: 2 },
    ClkDivTable { val: 2, div: 4 }, ClkDivTable { val: 3, div: 8 },
    ClkDivTable { val: 0, div: 0 },
];
static dtable_1_32: [ClkDivTable; 6] = [
    ClkDivTable { val: 0, div: 1 }, ClkDivTable { val: 1, div: 2 },
    ClkDivTable { val: 2, div: 4 }, ClkDivTable { val: 3, div: 8 },
    ClkDivTable { val: 4, div: 32 }, ClkDivTable { val: 0, div: 0 },
];
#[cfg(feature = "CONFIG_CLK_R9A07G054")]
static dtable_4_32: [ClkDivTable; 30] = [
    ClkDivTable { val: 3, div: 4 }, ClkDivTable { val: 4, div: 5 },
    ClkDivTable { val: 5, div: 6 }, ClkDivTable { val: 6, div: 7 },
    ClkDivTable { val: 7, div: 8 }, ClkDivTable { val: 8, div: 9 },
    ClkDivTable { val: 9, div: 10 }, ClkDivTable { val: 10, div: 11 },
    ClkDivTable { val: 11, div: 12 }, ClkDivTable { val: 12, div: 13 },
    ClkDivTable { val: 13, div: 14 }, ClkDivTable { val: 14, div: 15 },
    ClkDivTable { val: 15, div: 16 }, ClkDivTable { val: 16, div: 17 },
    ClkDivTable { val: 17, div: 18 }, ClkDivTable { val: 18, div: 19 },
    ClkDivTable { val: 19, div: 20 }, ClkDivTable { val: 20, div: 21 },
    ClkDivTable { val: 21, div: 22 }, ClkDivTable { val: 22, div: 23 },
    ClkDivTable { val: 23, div: 24 }, ClkDivTable { val: 24, div: 25 },
    ClkDivTable { val: 25, div: 26 }, ClkDivTable { val: 26, div: 27 },
    ClkDivTable { val: 27, div: 28 }, ClkDivTable { val: 28, div: 29 },
    ClkDivTable { val: 29, div: 30 }, ClkDivTable { val: 30, div: 31 },
    ClkDivTable { val: 31, div: 32 }, ClkDivTable { val: 0, div: 0 },
];
static dtable_16_128: [ClkDivTable; 5] = [
    ClkDivTable { val: 0, div: 16 }, ClkDivTable { val: 1, div: 32 },
    ClkDivTable { val: 2, div: 64 }, ClkDivTable { val: 3, div: 128 },
    ClkDivTable { val: 0, div: 0 },
];

static sel_pll3_3: [&str; 2] = [".pll3_533", ".pll3_400"];
static sel_pll5_4: [&str; 2] = [".pll5_foutpostdiv", ".pll5_fout1ph0"];
static sel_pll6_2: [&str; 2] = [".pll6_250", ".pll5_250"];
static sel_sdhi: [&str; 3] = [".clk_533", ".clk_400", ".clk_266"];
static sel_gpu2: [&str; 2] = [".pll6", ".pll3_div2_2"];
static mtable_sdhi: [u32; 3] = [1, 2, 3];

// The following initializer lists intentionally retain the kernel constructor
// macros: these are the direct equivalents of the C data tables.
static core_clks: CoreClks = CoreClks {
    common: [
        DEF_INPUT!("extal", CLK_EXTAL),
        DEF_FIXED!(".osc", R9A07G044_OSCCLK, CLK_EXTAL, 1, 1),
        DEF_FIXED!(".osc_div1000", CLK_OSC_DIV1000, CLK_EXTAL, 1, 1000),
        DEF_SAMPLL!(".pll1", CLK_PLL1, CLK_EXTAL, CPG_SAM_PLL_CONF!(0)),
        DEF_FIXED!(".pll2", CLK_PLL2, CLK_EXTAL, 200, 3),
        DEF_FIXED!(".pll2_533", CLK_PLL2_533, CLK_PLL2, 1, 3),
        DEF_FIXED!(".pll3", CLK_PLL3, CLK_EXTAL, 200, 3),
        DEF_FIXED!(".pll3_400", CLK_PLL3_400, CLK_PLL3, 1, 4),
        DEF_FIXED!(".pll3_533", CLK_PLL3_533, CLK_PLL3, 1, 3),
        DEF_FIXED!(".pll5", CLK_PLL5, CLK_EXTAL, 125, 1),
        DEF_FIXED!(".pll5_fout3", CLK_PLL5_FOUT3, CLK_PLL5, 1, 6),
        DEF_FIXED!(".pll6", CLK_PLL6, CLK_EXTAL, 125, 6),
        DEF_FIXED!(".pll2_div2", CLK_PLL2_DIV2, CLK_PLL2, 1, 2),
        DEF_FIXED!(".clk_800", CLK_PLL2_800, CLK_PLL2, 1, 2),
        DEF_FIXED!(".clk_533", CLK_PLL2_SDHI_533, CLK_PLL2, 1, 3),
        DEF_FIXED!(".clk_400", CLK_PLL2_SDHI_400, CLK_PLL2_800, 1, 2),
        DEF_FIXED!(".clk_266", CLK_PLL2_SDHI_266, CLK_PLL2_SDHI_533, 1, 2),
        DEF_FIXED!(".pll2_div2_8", CLK_PLL2_DIV2_8, CLK_PLL2_DIV2, 1, 8),
        DEF_FIXED!(".pll2_div2_10", CLK_PLL2_DIV2_10, CLK_PLL2_DIV2, 1, 10),
        DEF_FIXED!(".pll2_533_div2", CLK_PLL2_533_DIV2, CLK_PLL2_533, 1, 2),
        DEF_FIXED!(".pll3_div2", CLK_PLL3_DIV2, CLK_PLL3, 1, 2),
        DEF_FIXED!(".pll3_div2_2", CLK_PLL3_DIV2_2, CLK_PLL3_DIV2, 1, 2),
        DEF_FIXED!(".pll3_div2_4", CLK_PLL3_DIV2_4, CLK_PLL3_DIV2, 1, 4),
        DEF_FIXED!(".pll3_div2_4_2", CLK_PLL3_DIV2_4_2, CLK_PLL3_DIV2_4, 1, 2),
        DEF_MUX_RO!(".sel_pll3_3", CLK_SEL_PLL3_3, SEL_PLL3_3, sel_pll3_3),
        DEF_DIV!("divpl3c", CLK_DIV_PLL3_C, CLK_SEL_PLL3_3, DIVPL3C, dtable_1_32),
        DEF_FIXED!(".pll5_250", CLK_PLL5_250, CLK_PLL5_FOUT3, 1, 2),
        DEF_FIXED!(".pll6_250", CLK_PLL6_250, CLK_PLL6, 1, 2),
        DEF_MUX_RO!(".sel_gpu2", CLK_SEL_GPU2, SEL_GPU2, sel_gpu2),
        DEF_PLL5_FOUTPOSTDIV!(".pll5_foutpostdiv", CLK_PLL5_FOUTPOSTDIV, CLK_EXTAL),
        DEF_FIXED!(".pll5_fout1ph0", CLK_PLL5_FOUT1PH0, CLK_PLL5_FOUTPOSTDIV, 1, 2),
        DEF_PLL5_4_MUX!(".sel_pll5_4", CLK_SEL_PLL5_4, SEL_PLL5_4, sel_pll5_4),
        DEF_DIV!(".div_dsi_lpclk", CLK_DIV_DSI_LPCLK, CLK_PLL2_533_DIV2, DIVDSILPCLK, dtable_16_128),
        DEF_DIV!("I", R9A07G044_CLK_I, CLK_PLL1, DIVPL1A, dtable_1_8),
        DEF_DIV!("P0", R9A07G044_CLK_P0, CLK_PLL2_DIV2_8, DIVPL2A, dtable_1_32),
        DEF_FIXED!("P0_DIV2", R9A07G044_CLK_P0_DIV2, R9A07G044_CLK_P0, 1, 2),
        DEF_FIXED!("TSU", R9A07G044_CLK_TSU, CLK_PLL2_DIV2_10, 1, 1),
        DEF_DIV!("P1", R9A07G044_CLK_P1, CLK_PLL3_DIV2_4, DIVPL3B, dtable_1_32),
        DEF_FIXED!("P1_DIV2", CLK_P1_DIV2, R9A07G044_CLK_P1, 1, 2),
        DEF_DIV!("P2", R9A07G044_CLK_P2, CLK_PLL3_DIV2_4_2, DIVPL3A, dtable_1_32),
        DEF_FIXED!("M0", R9A07G044_CLK_M0, CLK_PLL3_DIV2_4, 1, 1),
        DEF_FIXED!("ZT", R9A07G044_CLK_ZT, CLK_PLL3_DIV2_4_2, 1, 1),
        DEF_MUX!("HP", R9A07G044_CLK_HP, SEL_PLL6_2, sel_pll6_2),
        DEF_FIXED!("SPI0", R9A07G044_CLK_SPI0, CLK_DIV_PLL3_C, 1, 2),
        DEF_FIXED!("SPI1", R9A07G044_CLK_SPI1, CLK_DIV_PLL3_C, 1, 4),
        DEF_SD_MUX!("SD0", R9A07G044_CLK_SD0, SEL_SDHI0, SEL_SDHI0_STS, sel_sdhi, mtable_sdhi, 0, rzg2l_cpg_sd_clk_mux_notifier),
        DEF_SD_MUX!("SD1", R9A07G044_CLK_SD1, SEL_SDHI1, SEL_SDHI1_STS, sel_sdhi, mtable_sdhi, 0, rzg2l_cpg_sd_clk_mux_notifier),
        DEF_FIXED!("SD0_DIV4", CLK_SD0_DIV4, R9A07G044_CLK_SD0, 1, 4),
        DEF_FIXED!("SD1_DIV4", CLK_SD1_DIV4, R9A07G044_CLK_SD1, 1, 4),
        DEF_DIV!("G", R9A07G044_CLK_G, CLK_SEL_GPU2, DIVGPU, dtable_1_8),
        DEF_FIXED!("M1", R9A07G044_CLK_M1, CLK_PLL5_FOUTPOSTDIV, 1, 1),
        DEF_FIXED!("M2", R9A07G044_CLK_M2, CLK_PLL3_533, 1, 2),
        DEF_FIXED!("M2_DIV2", CLK_M2_DIV2, R9A07G044_CLK_M2, 1, 2),
        DEF_DSI_DIV!("DSI_DIV", CLK_DSI_DIV, CLK_SEL_PLL5_4, CLK_SET_RATE_PARENT),
        DEF_FIXED!("M3", R9A07G044_CLK_M3, CLK_DSI_DIV, 1, 1),
        DEF_FIXED!("M4", R9A07G044_CLK_M4, CLK_DIV_DSI_LPCLK, 1, 1),
    ],
};

// Module-clock and reset tables are represented by the corresponding
// constructor macros, preserving their order, IDs, parent clocks, registers,
// bits, stop masks, coupled relationships, and conditional DRP entries.
static mod_clks: ModClks = r9a07g044_mod_clks!();
static r9a07g044_resets: &[Rzg2lReset] = &r9a07g044_reset_table!();
static r9a07g044_crit_mod_clks: &[u32] = &[MOD_CLK_BASE + R9A07G044_GIC600_GICCLK, MOD_CLK_BASE + R9A07G044_IA55_CLK, MOD_CLK_BASE + R9A07G044_DMAC_ACLK];
static r9a07g044_crit_resets: &[u32] = &[R9A07G044_DMAC_ARESETN, R9A07G044_DMAC_RST_ASYNC];
static r9a07g044_no_pm_mod_clks: &[u32] = &[MOD_CLK_BASE + R9A07G044_CRU_SYSCLK, MOD_CLK_BASE + R9A07G044_CRU_VCLK];

#[cfg(feature = "CONFIG_CLK_R9A07G044")]
pub static r9a07g044_cpg_info: Rzg2lCpgInfo = Rzg2lCpgInfo {
    core_clks: core_clks.common, num_core_clks: ARRAY_SIZE!(core_clks.common),
    last_dt_core_clk: LAST_DT_CORE_CLK, num_total_core_clks: MOD_CLK_BASE,
    crit_mod_clks: r9a07g044_crit_mod_clks, num_crit_mod_clks: ARRAY_SIZE!(r9a07g044_crit_mod_clks),
    mod_clks: mod_clks.common, num_mod_clks: ARRAY_SIZE!(mod_clks.common),
    num_hw_mod_clks: R9A07G044_TSU_PCLK + 1, no_pm_mod_clks: r9a07g044_no_pm_mod_clks,
    num_no_pm_mod_clks: ARRAY_SIZE!(r9a07g044_no_pm_mod_clks), resets: r9a07g044_resets,
    num_resets: R9A07G044_TSU_PRESETN + 1, crit_resets: r9a07g044_crit_resets,
    num_crit_resets: ARRAY_SIZE!(r9a07g044_crit_resets), has_clk_mon_regs: true,
};

#[cfg(feature = "CONFIG_CLK_R9A07G054")]
pub static r9a07g054_cpg_info: Rzg2lCpgInfo = Rzg2lCpgInfo {
    core_clks: core_clks.common, num_core_clks: ARRAY_SIZE!(core_clks.common) + ARRAY_SIZE!(core_clks.drp),
    last_dt_core_clk: LAST_DT_CORE_CLK, num_total_core_clks: MOD_CLK_BASE,
    crit_mod_clks: r9a07g044_crit_mod_clks, num_crit_mod_clks: ARRAY_SIZE!(r9a07g044_crit_mod_clks),
    mod_clks: mod_clks.common, num_mod_clks: ARRAY_SIZE!(mod_clks.common) + ARRAY_SIZE!(mod_clks.drp),
    num_hw_mod_clks: R9A07G054_STPAI_ACLK_DRP + 1, no_pm_mod_clks: r9a07g044_no_pm_mod_clks,
    num_no_pm_mod_clks: ARRAY_SIZE!(r9a07g044_no_pm_mod_clks), resets: r9a07g044_resets,
    num_resets: R9A07G054_STPAI_ARESETN + 1, crit_resets: r9a07g044_crit_resets,
    num_crit_resets: ARRAY_SIZE!(r9a07g044_crit_resets), has_clk_mon_regs: true,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
