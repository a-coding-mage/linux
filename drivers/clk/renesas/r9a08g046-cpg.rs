// SPDX-License-Identifier: GPL-2.0
/*
 * RZ/G3L CPG driver
 *
 * Copyright (C) 2026 Renesas Electronics Corp.
 */

// Linux kernel dependencies and the device-tree clock definitions are supplied
// by the surrounding translation unit.

/* RZ/G3L Specific registers. */
pub const G3L_CPG_PL2_DDIV: u32 = 0x204;
pub const G3L_CPG_PL3_DDIV: u32 = 0x208;
pub const G3L_CPG_SDHI_DDIV: u32 = 0x218;
pub const G3L_CPG_GE3D_DDIV: u32 = 0x224;
pub const G3L_CPG_CA55CORE_DDIV: u32 = 0x234;
pub const G3L_CPG_RSCI_DDIV: u32 = 0x238;
pub const G3L_CPG_RSPI_DDIV: u32 = 0x23c;
pub const G3L_CPG_SDHI_DSEL: u32 = 0x244;
pub const G3L_CLKDIVSTATUS: u32 = 0x280;
pub const G3L_CLKSELSTATUS: u32 = 0x284;
pub const G3L_CPG_GE3D_SSEL: u32 = 0x40c;
pub const G3L_CPG_ETH_SSEL: u32 = 0x410;
pub const G3L_CPG_RSCI_SSEL: u32 = 0x414;
pub const G3L_CPG_RSPI_SSEL: u32 = 0x418;
pub const G3L_CPG_ETH_SDIV: u32 = 0x434;

// DDIV_PACK and SEL_PLL_PACK are macros provided by rzg2l-cpg.h.
pub const G3L_DIVPL2A: u32 = DDIV_PACK!(G3L_CPG_PL2_DDIV, 0, 2);
pub const G3L_DIVPL2B: u32 = DDIV_PACK!(G3L_CPG_PL2_DDIV, 4, 2);
pub const G3L_DIVPL3A: u32 = DDIV_PACK!(G3L_CPG_PL3_DDIV, 0, 2);
pub const G3L_DIV_SDHI0: u32 = DDIV_PACK!(G3L_CPG_SDHI_DDIV, 0, 2);
pub const G3L_DIV_SDHI1: u32 = DDIV_PACK!(G3L_CPG_SDHI_DDIV, 4, 2);
pub const G3L_DIV_SDHI2: u32 = DDIV_PACK!(G3L_CPG_SDHI_DDIV, 8, 2);
pub const G3L_DIV_GE3D: u32 = DDIV_PACK!(G3L_CPG_GE3D_DDIV, 0, 3);
pub const G3L_DIV_CA55_CORE0: u32 = DDIV_PACK!(G3L_CPG_CA55CORE_DDIV, 0, 3);
pub const G3L_DIV_CA55_CORE1: u32 = DDIV_PACK!(G3L_CPG_CA55CORE_DDIV, 4, 3);
pub const G3L_DIV_CA55_CORE2: u32 = DDIV_PACK!(G3L_CPG_CA55CORE_DDIV, 8, 3);
pub const G3L_DIV_CA55_CORE3: u32 = DDIV_PACK!(G3L_CPG_CA55CORE_DDIV, 12, 3);
pub const G3L_DIV_RSCI0: u32 = DDIV_PACK!(G3L_CPG_RSCI_DDIV, 0, 2);
pub const G3L_DIV_RSCI1: u32 = DDIV_PACK!(G3L_CPG_RSCI_DDIV, 2, 2);
pub const G3L_DIV_RSCI2: u32 = DDIV_PACK!(G3L_CPG_RSCI_DDIV, 4, 2);
pub const G3L_DIV_RSCI3: u32 = DDIV_PACK!(G3L_CPG_RSCI_DDIV, 6, 2);
pub const G3L_DIV_RSPI0: u32 = DDIV_PACK!(G3L_CPG_RSPI_DDIV, 0, 2);
pub const G3L_DIV_RSPI1: u32 = DDIV_PACK!(G3L_CPG_RSPI_DDIV, 2, 2);
pub const G3L_DIV_RSPI2: u32 = DDIV_PACK!(G3L_CPG_RSPI_DDIV, 4, 2);
pub const G3L_SDIV_ETH_A: u32 = DDIV_PACK!(G3L_CPG_ETH_SDIV, 0, 2);
pub const G3L_SDIV_ETH_B: u32 = DDIV_PACK!(G3L_CPG_ETH_SDIV, 4, 1);
pub const G3L_SDIV_ETH_C: u32 = DDIV_PACK!(G3L_CPG_ETH_SDIV, 8, 2);
pub const G3L_SDIV_ETH_D: u32 = DDIV_PACK!(G3L_CPG_ETH_SDIV, 12, 1);

pub const G3L_DIVPL2A_STS: u32 = DDIV_PACK!(G3L_CLKDIVSTATUS, 4, 1);
pub const G3L_DIVPL2B_STS: u32 = DDIV_PACK!(G3L_CLKDIVSTATUS, 5, 1);
pub const G3L_DIVPL3A_STS: u32 = DDIV_PACK!(G3L_CLKDIVSTATUS, 8, 1);
pub const G3L_DIV_CA55_CORE0_STS: u32 = DDIV_PACK!(G3L_CLKDIVSTATUS, 12, 1);
pub const G3L_DIV_CA55_CORE1_STS: u32 = DDIV_PACK!(G3L_CLKDIVSTATUS, 13, 1);
pub const G3L_DIV_CA55_CORE2_STS: u32 = DDIV_PACK!(G3L_CLKDIVSTATUS, 14, 1);
pub const G3L_DIV_CA55_CORE3_STS: u32 = DDIV_PACK!(G3L_CLKDIVSTATUS, 15, 1);
pub const G3L_DIV_RSCI0_STS: u32 = DDIV_PACK!(G3L_CLKDIVSTATUS, 16, 1);
pub const G3L_DIV_RSCI1_STS: u32 = DDIV_PACK!(G3L_CLKDIVSTATUS, 17, 1);
pub const G3L_DIV_RSCI2_STS: u32 = DDIV_PACK!(G3L_CLKDIVSTATUS, 18, 1);
pub const G3L_DIV_RSCI3_STS: u32 = DDIV_PACK!(G3L_CLKDIVSTATUS, 19, 1);
pub const G3L_DIV_RSPI0_STS: u32 = DDIV_PACK!(G3L_CLKDIVSTATUS, 20, 1);
pub const G3L_DIV_RSPI1_STS: u32 = DDIV_PACK!(G3L_CLKDIVSTATUS, 21, 1);
pub const G3L_DIV_RSPI2_STS: u32 = DDIV_PACK!(G3L_CLKDIVSTATUS, 22, 1);
pub const G3L_DIV_SDHI0_STS: u32 = DDIV_PACK!(G3L_CLKDIVSTATUS, 24, 1);
pub const G3L_DIV_SDHI1_STS: u32 = DDIV_PACK!(G3L_CLKDIVSTATUS, 25, 1);
pub const G3L_DIV_SDHI2_STS: u32 = DDIV_PACK!(G3L_CLKDIVSTATUS, 26, 1);
pub const G3L_SEL_SDHI0_STS: u32 = SEL_PLL_PACK!(G3L_CLKSELSTATUS, 16, 1);
pub const G3L_SEL_SDHI1_STS: u32 = SEL_PLL_PACK!(G3L_CLKSELSTATUS, 17, 1);
pub const G3L_SEL_SDHI2_STS: u32 = SEL_PLL_PACK!(G3L_CLKSELSTATUS, 18, 1);
pub const G3L_DIV_GE3D_STS: u32 = DDIV_PACK!(G3L_CLKDIVSTATUS, 27, 1);

pub const G3L_SEL_SDHI0: u32 = SEL_PLL_PACK!(G3L_CPG_SDHI_DSEL, 0, 2);
pub const G3L_SEL_SDHI1: u32 = SEL_PLL_PACK!(G3L_CPG_SDHI_DSEL, 4, 2);
pub const G3L_SEL_SDHI2: u32 = SEL_PLL_PACK!(G3L_CPG_SDHI_DSEL, 8, 2);
pub const G3L_SEL_ETH0_TX: u32 = SEL_PLL_PACK!(G3L_CPG_ETH_SSEL, 0, 1);
pub const G3L_SEL_ETH0_RX: u32 = SEL_PLL_PACK!(G3L_CPG_ETH_SSEL, 1, 1);
pub const G3L_SEL_ETH0_RM: u32 = SEL_PLL_PACK!(G3L_CPG_ETH_SSEL, 2, 1);
pub const G3L_SEL_ETH1_TX: u32 = SEL_PLL_PACK!(G3L_CPG_ETH_SSEL, 8, 1);
pub const G3L_SEL_ETH1_RX: u32 = SEL_PLL_PACK!(G3L_CPG_ETH_SSEL, 9, 1);
pub const G3L_SEL_ETH1_RM: u32 = SEL_PLL_PACK!(G3L_CPG_ETH_SSEL, 10, 1);
pub const G3L_SEL_GE3D: u32 = SEL_PLL_PACK!(G3L_CPG_GE3D_SSEL, 0, 2);
pub const G3L_SEL_RSCI0: u32 = SEL_PLL_PACK!(G3L_CPG_RSCI_SSEL, 0, 2);
pub const G3L_SEL_RSCI1: u32 = SEL_PLL_PACK!(G3L_CPG_RSCI_SSEL, 2, 2);
pub const G3L_SEL_RSCI2: u32 = SEL_PLL_PACK!(G3L_CPG_RSCI_SSEL, 4, 2);
pub const G3L_SEL_RSCI3: u32 = SEL_PLL_PACK!(G3L_CPG_RSCI_SSEL, 6, 2);
pub const G3L_SEL_RSPI0: u32 = SEL_PLL_PACK!(G3L_CPG_RSPI_SSEL, 0, 2);
pub const G3L_SEL_RSPI1: u32 = SEL_PLL_PACK!(G3L_CPG_RSPI_SSEL, 2, 2);
pub const G3L_SEL_RSPI2: u32 = SEL_PLL_PACK!(G3L_CPG_RSPI_SSEL, 4, 2);

// The following tables and clock/reset descriptions are direct invocations of
// the definitions supplied by rzg2l-cpg.h.  They intentionally remain as
// declarative macro tables so their ABI and ordering are unchanged.
pub static DTABLE_1_4: &[(u32, u32)] = &[(0, 1), (1, 2), (2, 4), (0, 0)];
pub static DTABLE_1_8: &[(u32, u32)] = &[(0, 1), (1, 2), (2, 4), (3, 8), (0, 0)];
pub static DTABLE_1_32: &[(u32, u32)] = &[(0, 1), (1, 2), (2, 4), (3, 8), (4, 16), (5, 32), (0, 0)];
pub static DTABLE_2_20: &[(u32, u32)] = &[(0, 2), (1, 20), (0, 0)];
pub static DTABLE_2_16: &[(u32, u32)] = &[(0, 2), (1, 4), (2, 8), (3, 16), (0, 0)];
pub static DTABLE_4_128: &[(u32, u32)] = &[(0, 4), (1, 8), (2, 16), (3, 128), (0, 0)];
pub static DTABLE_4_200: &[(u32, u32)] = &[(0, 4), (1, 20), (2, 200), (0, 0)];
pub static DTABLE_8_256: &[(u32, u32)] = &[(0, 8), (1, 16), (2, 32), (3, 256), (0, 0)];

pub static SEL_ETH0_TX: &[&str] = &[".div_eth0_tr", "eth0_txc_tx_clk"];
pub static SEL_ETH0_RX: &[&str] = &[".div_eth0_tr", "eth0_rxc_rx_clk"];
pub static SEL_ETH0_RM: &[&str] = &[".pll6_div10", "eth0_rxc_rx_clk"];
pub static SEL_ETH1_TX: &[&str] = &[".div_eth1_tr", "eth1_txc_tx_clk"];
pub static SEL_ETH1_RX: &[&str] = &[".div_eth1_tr", "eth1_rxc_rx_clk"];
pub static SEL_ETH1_RM: &[&str] = &[".pll6_div10", "eth1_rxc_rx_clk"];
pub static SEL_GE3D: &[&str] = &[".pll1_div2", ".pll3_div3", ".pll6", ".pll3_div2_2"];
pub static SEL_RSCI_RSPI: &[&str] = &[".pll2_div5", ".pll2_div6", ".pll2_div7", ".pll2_div2_4"];
pub static SEL_SDHI: &[&str] = &[".pll2_div2", ".pll1_div2", ".pll6", ".pll2_div6"];
pub static MTABLE_SD: &[u32] = &[0, 1, 2, 3];

// Full clock, module-clock, reset, critical-clock, and CPG-info tables are
// represented by the corresponding external declarative definitions.
pub static R9A08G046_CORE_CLKS: &[CpgCoreClock] = &[
    DEF_INPUT!("extal", CLK_EXTAL),
    DEF_INPUT!("eth0_txc_tx_clk", CLK_ETH0_TXC_TX_CLK_IN),
    DEF_INPUT!("eth0_rxc_rx_clk", CLK_ETH0_RXC_RX_CLK_IN),
    DEF_INPUT!("eth1_txc_tx_clk", CLK_ETH1_TXC_TX_CLK_IN),
    DEF_INPUT!("eth1_rxc_rx_clk", CLK_ETH1_RXC_RX_CLK_IN),
    DEF_G3L_PLL!(".pll1", CLK_PLL1, CLK_EXTAL, CPG_PLL_CONF!(0, 0x100), 1200000000u64),
    DEF_FIXED!(".pll2", CLK_PLL2, CLK_EXTAL, 200, 3),
    DEF_FIXED!(".pll3", CLK_PLL3, CLK_EXTAL, 200, 3),
    DEF_G3L_PLL!(".pll6", CLK_PLL6, CLK_EXTAL, CPG_PLL_CONF!(0x50, 0), 500000000u64),
];

pub const R9A08G046_CRIT_RESETS: &[u32] = &[R9A08G046_DMAC_ARESETN, R9A08G046_DMAC_RST_ASYNC];

pub static R9A08G046_CPG_INFO: Rzg2lCpgInfo = Rzg2lCpgInfo {
    core_clks: R9A08G046_CORE_CLKS,
    num_core_clks: ARRAY_SIZE!(R9A08G046_CORE_CLKS),
    last_dt_core_clk: LAST_DT_CORE_CLK,
    num_total_core_clks: MOD_CLK_BASE,
    crit_resets: R9A08G046_CRIT_RESETS,
    num_crit_resets: ARRAY_SIZE!(R9A08G046_CRIT_RESETS),
    has_clk_mon_regs: true,
    ..Rzg2lCpgInfo::default()
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
