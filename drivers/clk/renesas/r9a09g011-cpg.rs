// SPDX-License-Identifier: GPL-2.0
/*
 * RZ/V2M Clock Pulse Generator / Module Standby and Software Reset
 *
 * Copyright (C) 2022 Renesas Electronics Corp.
 *
 * Based on r9a07g044-cpg.c
 */

// Linux headers and the device-tree binding header are external dependencies.
// The following macros and types are supplied by the surrounding translation.

const DIV_A: _ = DDIV_PACK!(0x200, 0, 3);
const DIV_B: _ = DDIV_PACK!(0x204, 0, 2);
const DIV_D: _ = DDIV_PACK!(0x204, 4, 2);
const DIV_E: _ = DDIV_PACK!(0x204, 8, 1);
const DIV_W: _ = DDIV_PACK!(0x328, 0, 3);

const SEL_B: _ = SEL_PLL_PACK!(0x214, 0, 1);
const SEL_CSI0: _ = SEL_PLL_PACK!(0x330, 0, 1);
const SEL_CSI4: _ = SEL_PLL_PACK!(0x330, 4, 1);
const SEL_D: _ = SEL_PLL_PACK!(0x214, 1, 1);
const SEL_E: _ = SEL_PLL_PACK!(0x214, 2, 1);
const SEL_SDI: _ = SEL_PLL_PACK!(0x300, 0, 1);
const SEL_W0: _ = SEL_PLL_PACK!(0x32c, 0, 1);

#[repr(usize)]
enum ClkIds {
    // Core Clock Outputs exported to DT
    LastDtCoreClk = 0,
    // External Input Clocks
    ClkExtal,
    // Internal Core Clocks
    ClkMain,
    ClkMain24,
    ClkMain2,
    ClkPll1,
    ClkPll2,
    ClkPll2_800,
    ClkPll2_400,
    ClkPll2_200,
    ClkPll2_100,
    ClkPll4,
    ClkDivA,
    ClkDivB,
    ClkDivD,
    ClkDivE,
    ClkDivW,
    ClkSelB,
    ClkSelB_D2,
    ClkSelCsi0,
    ClkSelCsi4,
    ClkSelD,
    ClkSelE,
    ClkSelSdi,
    ClkSelW0,
    // Module Clocks
    ModClkBase,
}

static DTABLE_DIVA: [ClkDivTable; 8] = [
    ClkDivTable { val: 0, div: 1 }, ClkDivTable { val: 1, div: 2 },
    ClkDivTable { val: 2, div: 3 }, ClkDivTable { val: 3, div: 4 },
    ClkDivTable { val: 4, div: 6 }, ClkDivTable { val: 5, div: 12 },
    ClkDivTable { val: 6, div: 24 }, ClkDivTable { val: 0, div: 0 },
];
static DTABLE_DIVB: [ClkDivTable; 5] = [
    ClkDivTable { val: 0, div: 1 }, ClkDivTable { val: 1, div: 2 },
    ClkDivTable { val: 2, div: 4 }, ClkDivTable { val: 3, div: 8 },
    ClkDivTable { val: 0, div: 0 },
];
static DTABLE_DIVD: [ClkDivTable; 4] = [
    ClkDivTable { val: 0, div: 1 }, ClkDivTable { val: 1, div: 2 },
    ClkDivTable { val: 2, div: 4 }, ClkDivTable { val: 0, div: 0 },
];
static DTABLE_DIVW: [ClkDivTable; 8] = [
    ClkDivTable { val: 0, div: 6 }, ClkDivTable { val: 1, div: 7 },
    ClkDivTable { val: 2, div: 8 }, ClkDivTable { val: 3, div: 9 },
    ClkDivTable { val: 4, div: 10 }, ClkDivTable { val: 5, div: 11 },
    ClkDivTable { val: 6, div: 12 }, ClkDivTable { val: 0, div: 0 },
];

static SEL_B_NAMES: [&str; 2] = [".main", ".divb"];
static SEL_CSI_NAMES: [&str; 2] = [".main_24", ".main"];
static SEL_D_NAMES: [&str; 2] = [".main", ".divd"];
static SEL_E_NAMES: [&str; 2] = [".main", ".dive"];
static SEL_W_NAMES: [&str; 2] = [".main", ".divw"];
static SEL_SDI_NAMES: [&str; 2] = [".main", ".pll2_200"];

static R9A09G011_CORE_CLKS: &[CpgCoreClk] = &[
    DEF_INPUT!("extal", CLK_EXTAL),
    DEF_FIXED!(".main", CLK_MAIN, CLK_EXTAL, 1, 1),
    DEF_FIXED!(".main_24", CLK_MAIN_24, CLK_MAIN, 1, 2),
    DEF_FIXED!(".main_2", CLK_MAIN_2, CLK_MAIN, 1, 24),
    DEF_FIXED!(".pll1", CLK_PLL1, CLK_MAIN_2, 498, 1),
    DEF_FIXED!(".pll2", CLK_PLL2, CLK_MAIN_2, 800, 1),
    DEF_FIXED!(".pll2_800", CLK_PLL2_800, CLK_PLL2, 1, 2),
    DEF_FIXED!(".pll2_400", CLK_PLL2_400, CLK_PLL2_800, 1, 2),
    DEF_FIXED!(".pll2_200", CLK_PLL2_200, CLK_PLL2_800, 1, 4),
    DEF_FIXED!(".pll2_100", CLK_PLL2_100, CLK_PLL2_800, 1, 8),
    DEF_SAMPLL!(".pll4", CLK_PLL4, CLK_MAIN_2, CPG_SAM_PLL_CONF!(0x100)),
    DEF_DIV_RO!(".diva", CLK_DIV_A, CLK_PLL1, DIV_A, &DTABLE_DIVA),
    DEF_DIV_RO!(".divb", CLK_DIV_B, CLK_PLL2_400, DIV_B, &DTABLE_DIVB),
    DEF_DIV_RO!(".divd", CLK_DIV_D, CLK_PLL2_200, DIV_D, &DTABLE_DIVD),
    DEF_DIV_RO!(".dive", CLK_DIV_E, CLK_PLL2_100, DIV_E, None),
    DEF_DIV_RO!(".divw", CLK_DIV_W, CLK_PLL4, DIV_W, &DTABLE_DIVW),
    DEF_MUX_RO!(".selb", CLK_SEL_B, SEL_B, &SEL_B_NAMES),
    DEF_MUX_RO!(".seld", CLK_SEL_D, SEL_D, &SEL_D_NAMES),
    DEF_MUX_RO!(".sele", CLK_SEL_E, SEL_E, &SEL_E_NAMES),
    DEF_MUX!(".selsdi", CLK_SEL_SDI, SEL_SDI, &SEL_SDI_NAMES),
    DEF_MUX!(".selcsi0", CLK_SEL_CSI0, SEL_CSI0, &SEL_CSI_NAMES),
    DEF_MUX!(".selcsi4", CLK_SEL_CSI4, SEL_CSI4, &SEL_CSI_NAMES),
    DEF_MUX!(".selw0", CLK_SEL_W0, SEL_W0, &SEL_W_NAMES),
    DEF_FIXED!(".selb_d2", CLK_SEL_B_D2, CLK_SEL_B, 1, 2),
];

static R9A09G011_MOD_CLKS: &[Rzg2lModClk] = &[
    DEF_MOD!("pfc", R9A09G011_PFC_PCLK, CLK_MAIN, 0x400, 2, 0),
    DEF_MOD!("gic", R9A09G011_GIC_CLK, CLK_SEL_B_D2, 0x400, 5, 0),
    DEF_MOD!("sdi0_aclk", R9A09G011_SDI0_ACLK, CLK_SEL_D, 0x408, 0, 0),
    DEF_MOD!("sdi0_imclk", R9A09G011_SDI0_IMCLK, CLK_SEL_SDI, 0x408, 1, 0),
    DEF_MOD!("sdi0_imclk2", R9A09G011_SDI0_IMCLK2, CLK_SEL_SDI, 0x408, 2, 0),
    DEF_MOD!("sdi0_clk_hs", R9A09G011_SDI0_CLK_HS, CLK_PLL2_800, 0x408, 3, 0),
    DEF_MOD!("sdi1_aclk", R9A09G011_SDI1_ACLK, CLK_SEL_D, 0x408, 4, 0),
    DEF_MOD!("sdi1_imclk", R9A09G011_SDI1_IMCLK, CLK_SEL_SDI, 0x408, 5, 0),
    DEF_MOD!("sdi1_imclk2", R9A09G011_SDI1_IMCLK2, CLK_SEL_SDI, 0x408, 6, 0),
    DEF_MOD!("sdi1_clk_hs", R9A09G011_SDI1_CLK_HS, CLK_PLL2_800, 0x408, 7, 0),
    DEF_MOD!("emm_aclk", R9A09G011_EMM_ACLK, CLK_SEL_D, 0x408, 8, 0),
    DEF_MOD!("emm_imclk", R9A09G011_EMM_IMCLK, CLK_SEL_SDI, 0x408, 9, 0),
    DEF_MOD!("emm_imclk2", R9A09G011_EMM_IMCLK2, CLK_SEL_SDI, 0x408, 10, 0),
    DEF_MOD!("emm_clk_hs", R9A09G011_EMM_CLK_HS, CLK_PLL2_800, 0x408, 11, 0),
    DEF_COUPLED!("eth_axi", R9A09G011_ETH0_CLK_AXI, CLK_PLL2_200, 0x40c, 8, 0),
    DEF_COUPLED!("eth_chi", R9A09G011_ETH0_CLK_CHI, CLK_PLL2_100, 0x40c, 8, 0),
    DEF_MOD!("eth_clk_gptp", R9A09G011_ETH0_GPTP_EXT, CLK_PLL2_100, 0x40c, 9, 0),
    DEF_MOD!("usb_aclk_h", R9A09G011_USB_ACLK_H, CLK_SEL_D, 0x40c, 4, 0),
    DEF_MOD!("usb_aclk_p", R9A09G011_USB_ACLK_P, CLK_SEL_D, 0x40c, 5, 0),
    DEF_MOD!("usb_pclk", R9A09G011_USB_PCLK, CLK_SEL_E, 0x40c, 6, 0),
    DEF_MOD!("syc_cnt_clk", R9A09G011_SYC_CNT_CLK, CLK_MAIN_24, 0x41c, 12, 0),
    DEF_MOD!("iic_pclk0", R9A09G011_IIC_PCLK0, CLK_SEL_E, 0x420, 12, 0),
    DEF_MOD!("cperi_grpb", R9A09G011_CPERI_GRPB_PCLK, CLK_SEL_E, 0x424, 0, 0),
    DEF_MOD!("tim_clk_8", R9A09G011_TIM8_CLK, CLK_MAIN_2, 0x424, 4, 0),
    DEF_MOD!("tim_clk_9", R9A09G011_TIM9_CLK, CLK_MAIN_2, 0x424, 5, 0),
    DEF_MOD!("tim_clk_10", R9A09G011_TIM10_CLK, CLK_MAIN_2, 0x424, 6, 0),
    DEF_MOD!("tim_clk_11", R9A09G011_TIM11_CLK, CLK_MAIN_2, 0x424, 7, 0),
    DEF_MOD!("tim_clk_12", R9A09G011_TIM12_CLK, CLK_MAIN_2, 0x424, 8, 0),
    DEF_MOD!("tim_clk_13", R9A09G011_TIM13_CLK, CLK_MAIN_2, 0x424, 9, 0),
    DEF_MOD!("tim_clk_14", R9A09G011_TIM14_CLK, CLK_MAIN_2, 0x424, 10, 0),
    DEF_MOD!("tim_clk_15", R9A09G011_TIM15_CLK, CLK_MAIN_2, 0x424, 11, 0),
    DEF_MOD!("iic_pclk1", R9A09G011_IIC_PCLK1, CLK_SEL_E, 0x424, 12, 0),
    DEF_MOD!("cperi_grpc", R9A09G011_CPERI_GRPC_PCLK, CLK_SEL_E, 0x428, 0, 0),
    DEF_MOD!("tim_clk_16", R9A09G011_TIM16_CLK, CLK_MAIN_2, 0x428, 4, 0),
    DEF_MOD!("tim_clk_17", R9A09G011_TIM17_CLK, CLK_MAIN_2, 0x428, 5, 0),
    DEF_MOD!("tim_clk_18", R9A09G011_TIM18_CLK, CLK_MAIN_2, 0x428, 6, 0),
    DEF_MOD!("tim_clk_19", R9A09G011_TIM19_CLK, CLK_MAIN_2, 0x428, 7, 0),
    DEF_MOD!("tim_clk_20", R9A09G011_TIM20_CLK, CLK_MAIN_2, 0x428, 8, 0),
    DEF_MOD!("tim_clk_21", R9A09G011_TIM21_CLK, CLK_MAIN_2, 0x428, 9, 0),
    DEF_MOD!("tim_clk_22", R9A09G011_TIM22_CLK, CLK_MAIN_2, 0x428, 10, 0),
    DEF_MOD!("tim_clk_23", R9A09G011_TIM23_CLK, CLK_MAIN_2, 0x428, 11, 0),
    DEF_MOD!("wdt0_pclk", R9A09G011_WDT0_PCLK, CLK_SEL_E, 0x428, 12, 0),
    DEF_MOD!("wdt0_clk", R9A09G011_WDT0_CLK, CLK_MAIN, 0x428, 13, 0),
    DEF_MOD!("cperi_grpf", R9A09G011_CPERI_GRPF_PCLK, CLK_SEL_E, 0x434, 0, 0),
    DEF_MOD!("pwm8_clk", R9A09G011_PWM8_CLK, CLK_MAIN, 0x434, 4, 0),
    DEF_MOD!("pwm9_clk", R9A09G011_PWM9_CLK, CLK_MAIN, 0x434, 5, 0),
    DEF_MOD!("pwm10_clk", R9A09G011_PWM10_CLK, CLK_MAIN, 0x434, 6, 0),
    DEF_MOD!("pwm11_clk", R9A09G011_PWM11_CLK, CLK_MAIN, 0x434, 7, 0),
    DEF_MOD!("pwm12_clk", R9A09G011_PWM12_CLK, CLK_MAIN, 0x434, 8, 0),
    DEF_MOD!("pwm13_clk", R9A09G011_PWM13_CLK, CLK_MAIN, 0x434, 9, 0),
    DEF_MOD!("pwm14_clk", R9A09G011_PWM14_CLK, CLK_MAIN, 0x434, 10, 0),
    DEF_MOD!("cperi_grpg", R9A09G011_CPERI_GRPG_PCLK, CLK_SEL_E, 0x438, 0, 0),
    DEF_MOD!("cperi_grph", R9A09G011_CPERI_GRPH_PCLK, CLK_SEL_E, 0x438, 1, 0),
    DEF_MOD!("urt_pclk", R9A09G011_URT_PCLK, CLK_SEL_E, 0x438, 4, 0),
    DEF_MOD!("urt0_clk", R9A09G011_URT0_CLK, CLK_SEL_W0, 0x438, 5, 0),
    DEF_MOD!("csi0_clk", R9A09G011_CSI0_CLK, CLK_SEL_CSI0, 0x438, 8, 0),
    DEF_MOD!("csi4_clk", R9A09G011_CSI4_CLK, CLK_SEL_CSI4, 0x438, 12, 0),
    DEF_MOD!("ca53", R9A09G011_CA53_CLK, CLK_DIV_A, 0x448, 0, 0),
];

static R9A09G011_RESETS: &[Rzg2lReset] = &[
    DEF_RST!(R9A09G011_PFC_PRESETN, 0x600, 2),
    DEF_RST_MON!(R9A09G011_SDI0_IXRST, 0x608, 0, 6),
    DEF_RST_MON!(R9A09G011_SDI1_IXRST, 0x608, 1, 7),
    DEF_RST_MON!(R9A09G011_EMM_IXRST, 0x608, 2, 8),
    DEF_RST!(R9A09G011_USB_PRESET_N, 0x608, 7),
    DEF_RST!(R9A09G011_USB_DRD_RESET, 0x608, 8),
    DEF_RST!(R9A09G011_USB_ARESETN_P, 0x608, 9),
    DEF_RST!(R9A09G011_USB_ARESETN_H, 0x608, 10),
    DEF_RST_MON!(R9A09G011_ETH0_RST_HW_N, 0x608, 11, 11),
    DEF_RST_MON!(R9A09G011_SYC_RST_N, 0x610, 9, 13),
    DEF_RST!(R9A09G011_TIM_GPB_PRESETN, 0x614, 1),
    DEF_RST!(R9A09G011_TIM_GPC_PRESETN, 0x614, 2),
    DEF_RST_MON!(R9A09G011_PWM_GPF_PRESETN, 0x614, 5, 23),
    DEF_RST_MON!(R9A09G011_CSI_GPG_PRESETN, 0x614, 6, 22),
    DEF_RST_MON!(R9A09G011_CSI_GPH_PRESETN, 0x614, 7, 23),
    DEF_RST!(R9A09G011_IIC_GPA_PRESETN, 0x614, 8),
    DEF_RST!(R9A09G011_IIC_GPB_PRESETN, 0x614, 9),
    DEF_RST_MON!(R9A09G011_WDT0_PRESETN, 0x614, 12, 19),
];

static R9A09G011_CRIT_MOD_CLKS: &[u32] = &[
    MOD_CLK_BASE + R9A09G011_CA53_CLK,
    MOD_CLK_BASE + R9A09G011_CPERI_GRPB_PCLK,
    MOD_CLK_BASE + R9A09G011_CPERI_GRPC_PCLK,
    MOD_CLK_BASE + R9A09G011_CPERI_GRPF_PCLK,
    MOD_CLK_BASE + R9A09G011_CPERI_GRPG_PCLK,
    MOD_CLK_BASE + R9A09G011_CPERI_GRPH_PCLK,
    MOD_CLK_BASE + R9A09G011_GIC_CLK,
    MOD_CLK_BASE + R9A09G011_SYC_CNT_CLK,
    MOD_CLK_BASE + R9A09G011_URT_PCLK,
];

const R9A09G011_CPG_INFO: Rzg2lCpgInfo = Rzg2lCpgInfo {
    core_clks: R9A09G011_CORE_CLKS,
    num_core_clks: R9A09G011_CORE_CLKS.len(),
    last_dt_core_clk: LAST_DT_CORE_CLK,
    num_total_core_clks: MOD_CLK_BASE,
    crit_mod_clks: R9A09G011_CRIT_MOD_CLKS,
    num_crit_mod_clks: R9A09G011_CRIT_MOD_CLKS.len(),
    mod_clks: R9A09G011_MOD_CLKS,
    num_mod_clks: R9A09G011_MOD_CLKS.len(),
    num_hw_mod_clks: R9A09G011_CA53_CLK + 1,
    resets: R9A09G011_RESETS,
    num_resets: R9A09G011_RESETS.len(),
    has_clk_mon_regs: false,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
