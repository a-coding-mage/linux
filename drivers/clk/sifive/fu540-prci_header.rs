/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2018-2021 SiFive, Inc.
 * Copyright (C) 2018-2019 Wesley Terpstra
 * Copyright (C) 2018-2019 Paul Walmsley
 * Copyright (C) 2020-2021 Zong Li
 *
 * The FU540 PRCI implements clock and reset control for the SiFive
 * FU540-C000 chip.  This driver assumes that it has sole control
 * over all PRCI resources.
 *
 * This driver is based on the PRCI driver written by Wesley Terpstra:
 * https://github.com/riscv/riscv-linux/commit/999529edf517ed75b56659d456d221b2ee56bb60
 *
 * References:
 * - SiFive FU540-C000 manual v1p0, Chapter 7 "Clocking and Reset"
 */

// C dependencies supplied by the surrounding translation unit:
// <linux/module.h>, <dt-bindings/clock/sifive-fu540-prci.h>, and "sifive-prci.h"

/* PRCI integration data for each WRPLL instance */

static mut sifive_fu540_prci_corepll_data: __prci_wrpll_data = __prci_wrpll_data {
    cfg0_offs: PRCI_COREPLLCFG0_OFFSET,
    cfg1_offs: PRCI_COREPLLCFG1_OFFSET,
    enable_bypass: sifive_prci_coreclksel_use_hfclk,
    disable_bypass: sifive_prci_coreclksel_use_corepll,
};

static mut sifive_fu540_prci_ddrpll_data: __prci_wrpll_data = __prci_wrpll_data {
    cfg0_offs: PRCI_DDRPLLCFG0_OFFSET,
    cfg1_offs: PRCI_DDRPLLCFG1_OFFSET,
};

static mut sifive_fu540_prci_gemgxlpll_data: __prci_wrpll_data = __prci_wrpll_data {
    cfg0_offs: PRCI_GEMGXLPLLCFG0_OFFSET,
    cfg1_offs: PRCI_GEMGXLPLLCFG1_OFFSET,
};

/* Linux clock framework integration */

static sifive_fu540_prci_wrpll_clk_ops: clk_ops = clk_ops {
    set_rate: Some(sifive_prci_wrpll_set_rate),
    determine_rate: Some(sifive_prci_wrpll_determine_rate),
    recalc_rate: Some(sifive_prci_wrpll_recalc_rate),
    enable: Some(sifive_prci_clock_enable),
    disable: Some(sifive_prci_clock_disable),
    is_enabled: Some(sifive_clk_is_enabled),
};

static sifive_fu540_prci_wrpll_ro_clk_ops: clk_ops = clk_ops {
    recalc_rate: Some(sifive_prci_wrpll_recalc_rate),
};

static sifive_fu540_prci_tlclksel_clk_ops: clk_ops = clk_ops {
    recalc_rate: Some(sifive_prci_tlclksel_recalc_rate),
};

/* List of clock controls provided by the PRCI */
static mut __prci_init_clocks_fu540: [__prci_clock; 4] = [
    __prci_clock {
        name: "corepll",
        parent_name: "hfclk",
        ops: &sifive_fu540_prci_wrpll_clk_ops,
        pwd: &sifive_fu540_prci_corepll_data,
    },
    __prci_clock {
        name: "ddrpll",
        parent_name: "hfclk",
        ops: &sifive_fu540_prci_wrpll_ro_clk_ops,
        pwd: &sifive_fu540_prci_ddrpll_data,
    },
    __prci_clock {
        name: "gemgxlpll",
        parent_name: "hfclk",
        ops: &sifive_fu540_prci_wrpll_clk_ops,
        pwd: &sifive_fu540_prci_gemgxlpll_data,
    },
    __prci_clock {
        name: "tlclk",
        parent_name: "corepll",
        ops: &sifive_fu540_prci_tlclksel_clk_ops,
    },
];

static prci_clk_desc prci_clk_fu540: prci_clk_desc = prci_clk_desc {
    clks: __prci_init_clocks_fu540.as_ptr(),
    num_clks: __prci_init_clocks_fu540.len(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
