// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2022 MaxLinear, Inc.
 * Copyright (C) 2020 Intel Corporation.
 * Zhu Yixin <yzhu@maxlinear.com>
 * Rahul Tanwar <rtanwar@maxlinear.com>
 */
// Linux clock-provider, syscon, OF, platform-device, and clock binding
// dependencies are supplied by the surrounding kernel translation.

const PLL_DIV_WIDTH: u32 = 4;
const PLL_DDIV_WIDTH: u32 = 3;

const G_C55_SHIFT: u32 = 7; const G_QSPI_SHIFT: u32 = 9;
const G_EIP197_SHIFT: u32 = 11; const G_VAULT130_SHIFT: u32 = 12;
const G_TOE_SHIFT: u32 = 13; const G_SDXC_SHIFT: u32 = 14;
const G_EMMC_SHIFT: u32 = 15; const G_SPIDBG_SHIFT: u32 = 17;
const G_DMA3_SHIFT: u32 = 28;
const G_DMA0_SHIFT: u32 = 0; const G_LEDC0_SHIFT: u32 = 1;
const G_LEDC1_SHIFT: u32 = 2; const G_I2S0_SHIFT: u32 = 3;
const G_I2S1_SHIFT: u32 = 4; const G_EBU_SHIFT: u32 = 5;
const G_PWM_SHIFT: u32 = 6; const G_I2C0_SHIFT: u32 = 7;
const G_I2C1_SHIFT: u32 = 8; const G_I2C2_SHIFT: u32 = 9;
const G_I2C3_SHIFT: u32 = 10; const G_SSC0_SHIFT: u32 = 12;
const G_SSC1_SHIFT: u32 = 13; const G_SSC2_SHIFT: u32 = 14;
const G_SSC3_SHIFT: u32 = 15; const G_GPTC0_SHIFT: u32 = 17;
const G_GPTC1_SHIFT: u32 = 18; const G_GPTC2_SHIFT: u32 = 19;
const G_GPTC3_SHIFT: u32 = 20; const G_ASC0_SHIFT: u32 = 22;
const G_ASC1_SHIFT: u32 = 23; const G_ASC2_SHIFT: u32 = 24;
const G_ASC3_SHIFT: u32 = 25; const G_PCM0_SHIFT: u32 = 27;
const G_PCM1_SHIFT: u32 = 28; const G_PCM2_SHIFT: u32 = 29;
const G_PCIE10_SHIFT: u32 = 1; const G_PCIE11_SHIFT: u32 = 2;
const G_PCIE30_SHIFT: u32 = 3; const G_PCIE31_SHIFT: u32 = 4;
const G_PCIE20_SHIFT: u32 = 5; const G_PCIE21_SHIFT: u32 = 6;
const G_PCIE40_SHIFT: u32 = 7; const G_PCIE41_SHIFT: u32 = 8;
const G_XPCS0_SHIFT: u32 = 10; const G_XPCS1_SHIFT: u32 = 11;
const G_XPCS2_SHIFT: u32 = 12; const G_XPCS3_SHIFT: u32 = 13;
const G_SATA0_SHIFT: u32 = 14; const G_SATA1_SHIFT: u32 = 15;
const G_SATA2_SHIFT: u32 = 16; const G_SATA3_SHIFT: u32 = 17;
const G_ARCEM4_SHIFT: u32 = 0; const G_IDMAR1_SHIFT: u32 = 2;
const G_IDMAT0_SHIFT: u32 = 3; const G_IDMAT1_SHIFT: u32 = 4;
const G_IDMAT2_SHIFT: u32 = 5; const G_PPV4_SHIFT: u32 = 8;
const G_GSWIPO_SHIFT: u32 = 9; const G_CQEM_SHIFT: u32 = 10;
const G_XPCS5_SHIFT: u32 = 14; const G_USB1_SHIFT: u32 = 25;
const G_USB2_SHIFT: u32 = 26;

const CGU_PLL0CZ_CFG0: u32 = 0x000; const CGU_PLL0CM0_CFG0: u32 = 0x020;
const CGU_PLL0CM1_CFG0: u32 = 0x040; const CGU_PLL0B_CFG0: u32 = 0x060;
const CGU_PLL1_CFG0: u32 = 0x080; const CGU_PLL2_CFG0: u32 = 0x0a0;
const CGU_PLLPP_CFG0: u32 = 0x0c0; const CGU_LJPLL3_CFG0: u32 = 0x0e0;
const CGU_LJPLL4_CFG0: u32 = 0x100; const CGU_C55_PCMCR: u32 = 0x18c;
const CGU_PCMCR: u32 = 0x190; const CGU_IF_CLK1: u32 = 0x1a0;
const CGU_IF_CLK2: u32 = 0x1a4; const CGU_GATE0: u32 = 0x300;
const CGU_GATE1: u32 = 0x310; const CGU_GATE2: u32 = 0x320;
const CGU_GATE3: u32 = 0x310;
const CLK_NR_CLKS: usize = (LGM_GCLK_USB2 + 1) as usize;

const fn pll_div(x: u32) -> u32 { x + 0x04 }
const fn pll_ssc(x: u32) -> u32 { x + 0x10 }

static PLL_DIV: &[ClkDivTable] = &[
    ClkDivTable { val: 0, div: 1 }, ClkDivTable { val: 1, div: 2 },
    ClkDivTable { val: 2, div: 3 }, ClkDivTable { val: 3, div: 4 },
    ClkDivTable { val: 4, div: 5 }, ClkDivTable { val: 5, div: 6 },
    ClkDivTable { val: 6, div: 8 }, ClkDivTable { val: 7, div: 10 },
    ClkDivTable { val: 8, div: 12 }, ClkDivTable { val: 9, div: 16 },
    ClkDivTable { val: 10, div: 20 }, ClkDivTable { val: 11, div: 24 },
    ClkDivTable { val: 12, div: 32 }, ClkDivTable { val: 13, div: 40 },
    ClkDivTable { val: 14, div: 48 }, ClkDivTable { val: 15, div: 64 },
    ClkDivTable { val: 0, div: 0 },
];
static DCL_DIV: &[ClkDivTable] = &[
    ClkDivTable { val: 0, div: 6 }, ClkDivTable { val: 1, div: 12 },
    ClkDivTable { val: 2, div: 24 }, ClkDivTable { val: 3, div: 32 },
    ClkDivTable { val: 4, div: 48 }, ClkDivTable { val: 5, div: 96 },
    ClkDivTable { val: 0, div: 0 },
];

static PLL_P: &[ClkParentData] = &[ClkParentData { fw_name: "osc", name: "osc" }];
static PLLCM_P: &[ClkParentData] = &[ClkParentData { fw_name: "cpu_cm", name: "cpu_cm" }];
static EMMC_P: &[ClkParentData] = &[ClkParentData { fw_name: "emmc4", name: "emmc4" }, ClkParentData { fw_name: "noc4", name: "noc4" }];
static SDXC_P: &[ClkParentData] = &[ClkParentData { fw_name: "sdxc3", name: "sdxc3" }, ClkParentData { fw_name: "sdxc2", name: "sdxc2" }];
static PCM_P: &[ClkParentData] = &[ClkParentData { fw_name: "v_docsis", name: "v_docsis" }, ClkParentData { fw_name: "dcl", name: "dcl" }];
static CBPHY_P: &[ClkParentData] = &[ClkParentData { fw_name: "dd_serdes", name: "dd_serdes" }, ClkParentData { fw_name: "dd_pcie", name: "dd_pcie" }];

// The following table entries retain the kernel's source macros and ordering.
static LGM_PLL_CLKS: &[LgmPllClkData] = &[
    lgm_pll!(LGM_CLK_PLL0CZ, "pll0cz", PLL_P, CLK_IGNORE_UNUSED, CGU_PLL0CZ_CFG0, TYPE_ROPLL),
    lgm_pll!(LGM_CLK_PLL0CM0, "pll0cm0", PLLCM_P, CLK_IGNORE_UNUSED, CGU_PLL0CM0_CFG0, TYPE_ROPLL),
    lgm_pll!(LGM_CLK_PLL0CM1, "pll0cm1", PLLCM_P, CLK_IGNORE_UNUSED, CGU_PLL0CM1_CFG0, TYPE_ROPLL),
    lgm_pll!(LGM_CLK_PLL0B, "pll0b", PLL_P, CLK_IGNORE_UNUSED, CGU_PLL0B_CFG0, TYPE_ROPLL),
    lgm_pll!(LGM_CLK_PLL1, "pll1", PLL_P, 0, CGU_PLL1_CFG0, TYPE_ROPLL),
    lgm_pll!(LGM_CLK_PLL2, "pll2", PLL_P, CLK_IGNORE_UNUSED, CGU_PLL2_CFG0, TYPE_ROPLL),
    lgm_pll!(LGM_CLK_PLLPP, "pllpp", PLL_P, 0, CGU_PLLPP_CFG0, TYPE_ROPLL),
    lgm_pll!(LGM_CLK_LJPLL3, "ljpll3", PLL_P, 0, CGU_LJPLL3_CFG0, TYPE_LJPLL),
    lgm_pll!(LGM_CLK_LJPLL4, "ljpll4", PLL_P, 0, CGU_LJPLL4_CFG0, TYPE_LJPLL),
];

// Branch and gate declarations are preserved as direct macro translations.
static LGM_BRANCH_CLKS: &[LgmClkBranch] = &[
    lgm_div!(LGM_CLK_PP_HW,"pp_hw","pllpp",0,pll_div(CGU_PLLPP_CFG0),0,PLL_DIV_WIDTH,24,1,0,0,PLL_DIV),
    lgm_div!(LGM_CLK_PP_UC,"pp_uc","pllpp",0,pll_div(CGU_PLLPP_CFG0),4,PLL_DIV_WIDTH,25,1,0,0,PLL_DIV),
    lgm_div!(LGM_CLK_PP_FXD,"pp_fxd","pllpp",0,pll_div(CGU_PLLPP_CFG0),8,PLL_DIV_WIDTH,26,1,0,0,PLL_DIV),
    lgm_div!(LGM_CLK_PP_TBM,"pp_tbm","pllpp",0,pll_div(CGU_PLLPP_CFG0),12,PLL_DIV_WIDTH,27,1,0,0,PLL_DIV),
    lgm_div!(LGM_CLK_DDR,"ddr","pll2",CLK_IGNORE_UNUSED,pll_div(CGU_PLL2_CFG0),0,PLL_DIV_WIDTH,24,1,0,0,PLL_DIV),
    lgm_div!(LGM_CLK_CM,"cpu_cm","pll0cz",0,pll_div(CGU_PLL0CZ_CFG0),0,PLL_DIV_WIDTH,24,1,0,0,PLL_DIV),
    lgm_div!(LGM_CLK_IC,"cpu_ic","pll0cz",CLK_IGNORE_UNUSED,pll_div(CGU_PLL0CZ_CFG0),4,PLL_DIV_WIDTH,25,1,0,0,PLL_DIV),
    lgm_div!(LGM_CLK_SDXC3,"sdxc3","pll0cz",0,pll_div(CGU_PLL0CZ_CFG0),8,PLL_DIV_WIDTH,26,1,0,0,PLL_DIV),
    lgm_div!(LGM_CLK_CPU0,"cm0","pll0cm0",CLK_IGNORE_UNUSED,pll_div(CGU_PLL0CM0_CFG0),0,PLL_DIV_WIDTH,24,1,0,0,PLL_DIV),
    lgm_div!(LGM_CLK_CPU1,"cm1","pll0cm1",CLK_IGNORE_UNUSED,pll_div(CGU_PLL0CM1_CFG0),0,PLL_DIV_WIDTH,24,1,0,0,PLL_DIV),
    // ngi and noc4 are critical shared parent clock sources.
    lgm_div!(LGM_CLK_NGI,"ngi","pll0b",CLK_IGNORE_UNUSED|CLK_IS_CRITICAL,pll_div(CGU_PLL0B_CFG0),0,PLL_DIV_WIDTH,24,1,0,0,PLL_DIV),
    lgm_div!(LGM_CLK_NOC4,"noc4","pll0b",CLK_IGNORE_UNUSED|CLK_IS_CRITICAL,pll_div(CGU_PLL0B_CFG0),4,PLL_DIV_WIDTH,25,1,0,0,PLL_DIV),
    lgm_div!(LGM_CLK_SW,"switch","pll0b",0,pll_div(CGU_PLL0B_CFG0),8,PLL_DIV_WIDTH,26,1,0,0,PLL_DIV),
    lgm_div!(LGM_CLK_QSPI,"qspi","pll0b",0,pll_div(CGU_PLL0B_CFG0),12,PLL_DIV_WIDTH,27,1,0,0,PLL_DIV),
    lgm_div!(LGM_CLK_CT,"v_ct","pll1",0,pll_div(CGU_PLL1_CFG0),0,PLL_DIV_WIDTH,24,1,0,0,PLL_DIV),
    lgm_div!(LGM_CLK_DSP,"v_dsp","pll1",0,pll_div(CGU_PLL1_CFG0),8,PLL_DIV_WIDTH,26,1,0,0,PLL_DIV),
    lgm_div!(LGM_CLK_VIF,"v_ifclk","pll1",0,pll_div(CGU_PLL1_CFG0),12,PLL_DIV_WIDTH,27,1,0,0,PLL_DIV),
    lgm_fixed_factor!(LGM_CLK_EMMC4,"emmc4","sdxc3",0,0,0,0,0,0,1,4),
    lgm_fixed_factor!(LGM_CLK_SDXC2,"sdxc2","noc4",0,0,0,0,0,0,1,4),
    lgm_mux!(LGM_CLK_EMMC,"emmc",EMMC_P,0,CGU_IF_CLK1,0,1,CLK_MUX_ROUND_CLOSEST,0),
    lgm_mux!(LGM_CLK_SDXC,"sdxc",SDXC_P,0,CGU_IF_CLK1,1,1,CLK_MUX_ROUND_CLOSEST,0),
    lgm_fixed!(LGM_CLK_OSC,"osc",None,0,0,0,0,0,40000000,0),
    lgm_fixed!(LGM_CLK_SLIC,"slic",None,0,CGU_IF_CLK1,8,2,CLOCK_FLAG_VAL_INIT,8192000,2),
    lgm_fixed!(LGM_CLK_DOCSIS,"v_docsis",None,0,0,0,0,0,16000000,0),
    lgm_div!(LGM_CLK_DCL,"dcl","v_ifclk",CLK_SET_RATE_PARENT,CGU_PCMCR,25,3,0,0,DIV_CLK_NO_MASK,0,DCL_DIV),
    lgm_mux!(LGM_CLK_PCM,"pcm",PCM_P,0,CGU_C55_PCMCR,0,1,CLK_MUX_ROUND_CLOSEST,0),
    lgm_fixed_factor!(LGM_CLK_DDR_PHY,"ddr_phy","ddr",CLK_IGNORE_UNUSED,0,0,0,0,0,2,1),
    lgm_fixed_factor!(LGM_CLK_PONDEF,"pondef","dd_pool",CLK_SET_RATE_PARENT,0,0,0,0,0,1,2),
    lgm_mux!(LGM_CLK_CBPHY0,"cbphy0",CBPHY_P,0,0,0,0,MUX_CLK_SW|CLK_MUX_ROUND_CLOSEST,0),
    lgm_mux!(LGM_CLK_CBPHY1,"cbphy1",CBPHY_P,0,0,0,0,MUX_CLK_SW|CLK_MUX_ROUND_CLOSEST,0),
    lgm_mux!(LGM_CLK_CBPHY2,"cbphy2",CBPHY_P,0,0,0,0,MUX_CLK_SW|CLK_MUX_ROUND_CLOSEST,0),
    lgm_mux!(LGM_CLK_CBPHY3,"cbphy3",CBPHY_P,0,0,0,0,MUX_CLK_SW|CLK_MUX_ROUND_CLOSEST,0),
    lgm_gate!(LGM_GCLK_C55,"g_c55",None,0,CGU_GATE0,G_C55_SHIFT,0,0), lgm_gate!(LGM_GCLK_QSPI,"g_qspi",Some("qspi"),0,CGU_GATE0,G_QSPI_SHIFT,0,0),
    lgm_gate!(LGM_GCLK_EIP197,"g_eip197",None,0,CGU_GATE0,G_EIP197_SHIFT,0,0), lgm_gate!(LGM_GCLK_VAULT,"g_vault130",None,0,CGU_GATE0,G_VAULT130_SHIFT,0,0),
    lgm_gate!(LGM_GCLK_TOE,"g_toe",None,0,CGU_GATE0,G_TOE_SHIFT,0,0), lgm_gate!(LGM_GCLK_SDXC,"g_sdxc",Some("sdxc"),0,CGU_GATE0,G_SDXC_SHIFT,0,0),
    lgm_gate!(LGM_GCLK_EMMC,"g_emmc",Some("emmc"),0,CGU_GATE0,G_EMMC_SHIFT,0,0), lgm_gate!(LGM_GCLK_SPI_DBG,"g_spidbg",None,0,CGU_GATE0,G_SPIDBG_SHIFT,0,0), lgm_gate!(LGM_GCLK_DMA3,"g_dma3",None,0,CGU_GATE0,G_DMA3_SHIFT,0,0),
    lgm_gate!(LGM_GCLK_DMA0,"g_dma0",None,0,CGU_GATE1,G_DMA0_SHIFT,0,0), lgm_gate!(LGM_GCLK_LEDC0,"g_ledc0",None,0,CGU_GATE1,G_LEDC0_SHIFT,0,0), lgm_gate!(LGM_GCLK_LEDC1,"g_ledc1",None,0,CGU_GATE1,G_LEDC1_SHIFT,0,0),
    lgm_gate!(LGM_GCLK_I2S0,"g_i2s0",None,0,CGU_GATE1,G_I2S0_SHIFT,0,0), lgm_gate!(LGM_GCLK_I2S1,"g_i2s1",None,0,CGU_GATE1,G_I2S1_SHIFT,0,0), lgm_gate!(LGM_GCLK_EBU,"g_ebu",None,0,CGU_GATE1,G_EBU_SHIFT,0,0), lgm_gate!(LGM_GCLK_PWM,"g_pwm",None,0,CGU_GATE1,G_PWM_SHIFT,0,0),
    lgm_gate!(LGM_GCLK_I2C0,"g_i2c0",None,0,CGU_GATE1,G_I2C0_SHIFT,0,0), lgm_gate!(LGM_GCLK_I2C1,"g_i2c1",None,0,CGU_GATE1,G_I2C1_SHIFT,0,0), lgm_gate!(LGM_GCLK_I2C2,"g_i2c2",None,0,CGU_GATE1,G_I2C2_SHIFT,0,0), lgm_gate!(LGM_GCLK_I2C3,"g_i2c3",None,0,CGU_GATE1,G_I2C3_SHIFT,0,0),
    lgm_gate!(LGM_GCLK_SSC0,"g_ssc0",Some("noc4"),0,CGU_GATE1,G_SSC0_SHIFT,0,0), lgm_gate!(LGM_GCLK_SSC1,"g_ssc1",Some("noc4"),0,CGU_GATE1,G_SSC1_SHIFT,0,0), lgm_gate!(LGM_GCLK_SSC2,"g_ssc2",Some("noc4"),0,CGU_GATE1,G_SSC2_SHIFT,0,0), lgm_gate!(LGM_GCLK_SSC3,"g_ssc3",Some("noc4"),0,CGU_GATE1,G_SSC3_SHIFT,0,0),
    lgm_gate!(LGM_GCLK_GPTC0,"g_gptc0",Some("noc4"),0,CGU_GATE1,G_GPTC0_SHIFT,0,0), lgm_gate!(LGM_GCLK_GPTC1,"g_gptc1",Some("noc4"),0,CGU_GATE1,G_GPTC1_SHIFT,0,0), lgm_gate!(LGM_GCLK_GPTC2,"g_gptc2",Some("noc4"),0,CGU_GATE1,G_GPTC2_SHIFT,0,0), lgm_gate!(LGM_GCLK_GPTC3,"g_gptc3",Some("osc"),0,CGU_GATE1,G_GPTC3_SHIFT,0,0),
    lgm_gate!(LGM_GCLK_ASC0,"g_asc0",Some("noc4"),0,CGU_GATE1,G_ASC0_SHIFT,0,0), lgm_gate!(LGM_GCLK_ASC1,"g_asc1",Some("noc4"),0,CGU_GATE1,G_ASC1_SHIFT,0,0), lgm_gate!(LGM_GCLK_ASC2,"g_asc2",Some("noc4"),0,CGU_GATE1,G_ASC2_SHIFT,0,0), lgm_gate!(LGM_GCLK_ASC3,"g_asc3",Some("osc"),0,CGU_GATE1,G_ASC3_SHIFT,0,0),
    lgm_gate!(LGM_GCLK_PCM0,"g_pcm0",None,0,CGU_GATE1,G_PCM0_SHIFT,0,0), lgm_gate!(LGM_GCLK_PCM1,"g_pcm1",None,0,CGU_GATE1,G_PCM1_SHIFT,0,0), lgm_gate!(LGM_GCLK_PCM2,"g_pcm2",None,0,CGU_GATE1,G_PCM2_SHIFT,0,0),
    lgm_gate!(LGM_GCLK_PCIE10,"g_pcie10",None,0,CGU_GATE2,G_PCIE10_SHIFT,0,0), lgm_gate!(LGM_GCLK_PCIE11,"g_pcie11",None,0,CGU_GATE2,G_PCIE11_SHIFT,0,0), lgm_gate!(LGM_GCLK_PCIE30,"g_pcie30",None,0,CGU_GATE2,G_PCIE30_SHIFT,0,0), lgm_gate!(LGM_GCLK_PCIE31,"g_pcie31",None,0,CGU_GATE2,G_PCIE31_SHIFT,0,0),
    lgm_gate!(LGM_GCLK_PCIE20,"g_pcie20",None,0,CGU_GATE2,G_PCIE20_SHIFT,0,0), lgm_gate!(LGM_GCLK_PCIE21,"g_pcie21",None,0,CGU_GATE2,G_PCIE21_SHIFT,0,0), lgm_gate!(LGM_GCLK_PCIE40,"g_pcie40",None,0,CGU_GATE2,G_PCIE40_SHIFT,0,0), lgm_gate!(LGM_GCLK_PCIE41,"g_pcie41",None,0,CGU_GATE2,G_PCIE41_SHIFT,0,0),
    lgm_gate!(LGM_GCLK_XPCS0,"g_xpcs0",None,0,CGU_GATE2,G_XPCS0_SHIFT,0,0), lgm_gate!(LGM_GCLK_XPCS1,"g_xpcs1",None,0,CGU_GATE2,G_XPCS1_SHIFT,0,0), lgm_gate!(LGM_GCLK_XPCS2,"g_xpcs2",None,0,CGU_GATE2,G_XPCS2_SHIFT,0,0), lgm_gate!(LGM_GCLK_XPCS3,"g_xpcs3",None,0,CGU_GATE2,G_XPCS3_SHIFT,0,0),
    lgm_gate!(LGM_GCLK_SATA0,"g_sata0",None,0,CGU_GATE2,G_SATA0_SHIFT,0,0), lgm_gate!(LGM_GCLK_SATA1,"g_sata1",None,0,CGU_GATE2,G_SATA1_SHIFT,0,0), lgm_gate!(LGM_GCLK_SATA2,"g_sata2",None,0,CGU_GATE2,G_SATA2_SHIFT,0,0), lgm_gate!(LGM_GCLK_SATA3,"g_sata3",None,0,CGU_GATE2,G_SATA3_SHIFT,0,0),
    lgm_gate!(LGM_GCLK_ARCEM4,"g_arcem4",None,0,CGU_GATE3,G_ARCEM4_SHIFT,0,0), lgm_gate!(LGM_GCLK_IDMAR1,"g_idmar1",None,0,CGU_GATE3,G_IDMAR1_SHIFT,0,0), lgm_gate!(LGM_GCLK_IDMAT0,"g_idmat0",None,0,CGU_GATE3,G_IDMAT0_SHIFT,0,0), lgm_gate!(LGM_GCLK_IDMAT1,"g_idmat1",None,0,CGU_GATE3,G_IDMAT1_SHIFT,0,0), lgm_gate!(LGM_GCLK_IDMAT2,"g_idmat2",None,0,CGU_GATE3,G_IDMAT2_SHIFT,0,0),
    lgm_gate!(LGM_GCLK_PPV4,"g_ppv4",None,0,CGU_GATE3,G_PPV4_SHIFT,0,0), lgm_gate!(LGM_GCLK_GSWIPO,"g_gswipo",Some("switch"),0,CGU_GATE3,G_GSWIPO_SHIFT,0,0), lgm_gate!(LGM_GCLK_CQEM,"g_cqem",Some("switch"),0,CGU_GATE3,G_CQEM_SHIFT,0,0), lgm_gate!(LGM_GCLK_XPCS5,"g_xpcs5",None,0,CGU_GATE3,G_XPCS5_SHIFT,0,0), lgm_gate!(LGM_GCLK_USB1,"g_usb1",None,0,CGU_GATE3,G_USB1_SHIFT,0,0), lgm_gate!(LGM_GCLK_USB2,"g_usb2",None,0,CGU_GATE3,G_USB2_SHIFT,0,0),
];

static LGM_DDIV_CLKS: &[LgmClkDdivData] = &[
    lgm_ddiv!(LGM_CLK_CML,"dd_cml","ljpll3",0,pll_div(CGU_LJPLL3_CFG0),0,PLL_DDIV_WIDTH,3,PLL_DDIV_WIDTH,24,1,29,0),
    lgm_ddiv!(LGM_CLK_SERDES,"dd_serdes","ljpll3",0,pll_div(CGU_LJPLL3_CFG0),6,PLL_DDIV_WIDTH,9,PLL_DDIV_WIDTH,25,1,28,0),
    lgm_ddiv!(LGM_CLK_POOL,"dd_pool","ljpll3",0,pll_div(CGU_LJPLL3_CFG0),12,PLL_DDIV_WIDTH,15,PLL_DDIV_WIDTH,26,1,28,0),
    lgm_ddiv!(LGM_CLK_PTP,"dd_ptp","ljpll3",0,pll_div(CGU_LJPLL3_CFG0),18,PLL_DDIV_WIDTH,21,PLL_DDIV_WIDTH,27,1,28,0),
    lgm_ddiv!(LGM_CLK_PCIE,"dd_pcie","ljpll4",0,pll_div(CGU_LJPLL4_CFG0),0,PLL_DDIV_WIDTH,3,PLL_DDIV_WIDTH,24,1,29,0),
];

unsafe fn lgm_cgu_probe(pdev: *mut PlatformDevice) -> i32 {
    let mut ctx: *mut LgmClkProvider;
    let dev = unsafe { &mut (*pdev).dev };
    let np = dev.of_node;
    let ret: i32;
    ctx = unsafe { devm_kzalloc(dev, struct_size!(ctx, clk_data.hws, CLK_NR_CLKS), GFP_KERNEL) };
    if ctx.is_null() { return -ENOMEM; }
    unsafe { (*ctx).clk_data.num = CLK_NR_CLKS; }
    unsafe { (*ctx).membase = syscon_node_to_regmap(np); }
    if unsafe { is_err((*ctx).membase) } {
        unsafe { dev_err(dev, "Failed to get clk CGU iomem"); }
        return unsafe { ptr_err((*ctx).membase) };
    }
    unsafe { (*ctx).np = np; (*ctx).dev = dev; }
    ret = unsafe { lgm_clk_register_plls(ctx, LGM_PLL_CLKS.as_ptr(), LGM_PLL_CLKS.len()) }; if ret != 0 { return ret; }
    ret = unsafe { lgm_clk_register_branches(ctx, LGM_BRANCH_CLKS.as_ptr(), LGM_BRANCH_CLKS.len()) }; if ret != 0 { return ret; }
    ret = unsafe { lgm_clk_register_ddiv(ctx, LGM_DDIV_CLKS.as_ptr(), LGM_DDIV_CLKS.len()) }; if ret != 0 { return ret; }
    unsafe { devm_of_clk_add_hw_provider(dev, of_clk_hw_onecell_get, &(*ctx).clk_data) }
}

static OF_LGM_CGU_MATCH: &[OfDeviceId] = &[
    OfDeviceId { compatible: "intel,cgu-lgm" }, OfDeviceId::default(),
];

static LGM_CGU_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(lgm_cgu_probe),
    driver: Driver { name: "cgu-lgm", of_match_table: OF_LGM_CGU_MATCH },
};

builtin_platform_driver!(LGM_CGU_DRIVER);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
