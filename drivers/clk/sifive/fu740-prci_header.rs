/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2021 SiFive, Inc.
 * Copyright (C) 2020-2021 Zong Li
 */

// Translated from fu740-prci.h.  The Linux and SiFive PRCI dependencies are
// supplied by the surrounding translation unit.

/* PRCI integration data for each WRPLL instance */

static mut sifive_fu740_prci_corepll_data: __prci_wrpll_data = __prci_wrpll_data {
    cfg0_offs: PRCI_COREPLLCFG0_OFFSET,
    cfg1_offs: PRCI_COREPLLCFG1_OFFSET,
    enable_bypass: Some(sifive_prci_coreclksel_use_hfclk),
    disable_bypass: Some(sifive_prci_coreclksel_use_final_corepll),
};

static mut sifive_fu740_prci_ddrpll_data: __prci_wrpll_data = __prci_wrpll_data {
    cfg0_offs: PRCI_DDRPLLCFG0_OFFSET,
    cfg1_offs: PRCI_DDRPLLCFG1_OFFSET,
    ..__prci_wrpll_data::default()
};

static mut sifive_fu740_prci_gemgxlpll_data: __prci_wrpll_data = __prci_wrpll_data {
    cfg0_offs: PRCI_GEMGXLPLLCFG0_OFFSET,
    cfg1_offs: PRCI_GEMGXLPLLCFG1_OFFSET,
    ..__prci_wrpll_data::default()
};

static mut sifive_fu740_prci_dvfscorepll_data: __prci_wrpll_data = __prci_wrpll_data {
    cfg0_offs: PRCI_DVFSCOREPLLCFG0_OFFSET,
    cfg1_offs: PRCI_DVFSCOREPLLCFG1_OFFSET,
    enable_bypass: Some(sifive_prci_corepllsel_use_corepll),
    disable_bypass: Some(sifive_prci_corepllsel_use_dvfscorepll),
};

static mut sifive_fu740_prci_hfpclkpll_data: __prci_wrpll_data = __prci_wrpll_data {
    cfg0_offs: PRCI_HFPCLKPLLCFG0_OFFSET,
    cfg1_offs: PRCI_HFPCLKPLLCFG1_OFFSET,
    enable_bypass: Some(sifive_prci_hfpclkpllsel_use_hfclk),
    disable_bypass: Some(sifive_prci_hfpclkpllsel_use_hfpclkpll),
};

static mut sifive_fu740_prci_cltxpll_data: __prci_wrpll_data = __prci_wrpll_data {
    cfg0_offs: PRCI_CLTXPLLCFG0_OFFSET,
    cfg1_offs: PRCI_CLTXPLLCFG1_OFFSET,
    ..__prci_wrpll_data::default()
};

/* Linux clock framework integration */

static sifive_fu740_prci_wrpll_clk_ops: clk_ops = clk_ops {
    set_rate: Some(sifive_prci_wrpll_set_rate),
    determine_rate: Some(sifive_prci_wrpll_determine_rate),
    recalc_rate: Some(sifive_prci_wrpll_recalc_rate),
    enable: Some(sifive_prci_clock_enable),
    disable: Some(sifive_prci_clock_disable),
    is_enabled: Some(sifive_clk_is_enabled),
};

static sifive_fu740_prci_wrpll_ro_clk_ops: clk_ops = clk_ops {
    recalc_rate: Some(sifive_prci_wrpll_recalc_rate),
    ..clk_ops::default()
};

static sifive_fu740_prci_tlclksel_clk_ops: clk_ops = clk_ops {
    recalc_rate: Some(sifive_prci_tlclksel_recalc_rate),
    ..clk_ops::default()
};

static sifive_fu740_prci_hfpclkplldiv_clk_ops: clk_ops = clk_ops {
    recalc_rate: Some(sifive_prci_hfpclkplldiv_recalc_rate),
    ..clk_ops::default()
};

static sifive_fu740_prci_pcie_aux_clk_ops: clk_ops = clk_ops {
    enable: Some(sifive_prci_pcie_aux_clock_enable),
    disable: Some(sifive_prci_pcie_aux_clock_disable),
    is_enabled: Some(sifive_prci_pcie_aux_clock_is_enabled),
    ..clk_ops::default()
};

/* List of clock controls provided by the PRCI */
static mut __prci_init_clocks_fu740: [__prci_clock; 9] = [
    __prci_clock { name: "corepll", parent_name: "hfclk", ops: &sifive_fu740_prci_wrpll_clk_ops, pwd: Some(unsafe { &mut sifive_fu740_prci_corepll_data }) },
    __prci_clock { name: "ddrpll", parent_name: "hfclk", ops: &sifive_fu740_prci_wrpll_ro_clk_ops, pwd: Some(unsafe { &mut sifive_fu740_prci_ddrpll_data }) },
    __prci_clock { name: "gemgxlpll", parent_name: "hfclk", ops: &sifive_fu740_prci_wrpll_clk_ops, pwd: Some(unsafe { &mut sifive_fu740_prci_gemgxlpll_data }) },
    __prci_clock { name: "dvfscorepll", parent_name: "hfclk", ops: &sifive_fu740_prci_wrpll_clk_ops, pwd: Some(unsafe { &mut sifive_fu740_prci_dvfscorepll_data }) },
    __prci_clock { name: "hfpclkpll", parent_name: "hfclk", ops: &sifive_fu740_prci_wrpll_clk_ops, pwd: Some(unsafe { &mut sifive_fu740_prci_hfpclkpll_data }) },
    __prci_clock { name: "cltxpll", parent_name: "hfclk", ops: &sifive_fu740_prci_wrpll_clk_ops, pwd: Some(unsafe { &mut sifive_fu740_prci_cltxpll_data }) },
    __prci_clock { name: "tlclk", parent_name: "corepll", ops: &sifive_fu740_prci_tlclksel_clk_ops, ..__prci_clock::default() },
    __prci_clock { name: "pclk", parent_name: "hfpclkpll", ops: &sifive_fu740_prci_hfpclkplldiv_clk_ops, ..__prci_clock::default() },
    __prci_clock { name: "pcie_aux", parent_name: "hfclk", ops: &sifive_fu740_prci_pcie_aux_clk_ops, ..__prci_clock::default() },
];

static prci_clk_fu740: prci_clk_desc = prci_clk_desc {
    clks: unsafe { &mut __prci_init_clocks_fu740 },
    num_clks: __prci_init_clocks_fu740.len(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
