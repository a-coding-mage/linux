// SPDX-License-Identifier: GPL-2.0
/*
 * Marvell Armada 370 SoC clocks
 *
 * Copyright (C) 2012 Marvell
 *
 * Gregory CLEMENT <gregory.clement@free-electrons.com>
 * Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
 * Andrew Lunn <andrew@lunn.ch>
 */

// Dependencies supplied by the surrounding kernel translation.

/* Core Clocks */

const SARL: u32 = 0;
const SARL_A370_SSCG_ENABLE: u32 = 1 << 10;
const SARL_A370_PCLK_FREQ_OPT: u32 = 11;
const SARL_A370_PCLK_FREQ_OPT_MASK: u32 = 0xF;
const SARL_A370_FAB_FREQ_OPT: u32 = 15;
const SARL_A370_FAB_FREQ_OPT_MASK: u32 = 0x1F;
const SARL_A370_TCLK_FREQ_OPT: u32 = 20;
const SARL_A370_TCLK_FREQ_OPT_MASK: u32 = 0x1;

const A370_CPU_TO_NBCLK: i32 = 0;
const A370_CPU_TO_HCLK: i32 = 1;
const A370_CPU_TO_DRAMCLK: i32 = 2;

static A370_CORECLK_RATIOS: [coreclk_ratio; 3] = [
    coreclk_ratio { id: A370_CPU_TO_NBCLK, name: "nbclk" },
    coreclk_ratio { id: A370_CPU_TO_HCLK, name: "hclk" },
    coreclk_ratio { id: A370_CPU_TO_DRAMCLK, name: "dramclk" },
];

static A370_TCLK_FREQS: [u32; 2] = [166000000, 200000000];

unsafe fn a370_get_tclk_freq(sar: *mut core::ffi::c_void) -> u32 {
    let tclk_freq_select: u8 = ((readl(sar) >> SARL_A370_TCLK_FREQ_OPT)
        & SARL_A370_TCLK_FREQ_OPT_MASK) as u8;
    A370_TCLK_FREQS[tclk_freq_select as usize]
}

static A370_CPU_FREQS: [u32; 7] = [
    400000000, 533000000, 667000000, 800000000,
    1000000000, 1067000000, 1200000000,
];

unsafe fn a370_get_cpu_freq(sar: *mut core::ffi::c_void) -> u32 {
    let cpu_freq_select = ((readl(sar) >> SARL_A370_PCLK_FREQ_OPT)
        & SARL_A370_PCLK_FREQ_OPT_MASK) as u8;
    let cpu_freq;
    if cpu_freq_select as usize >= A370_CPU_FREQS.len() {
        pr_err!("CPU freq select unsupported {}\n", cpu_freq_select);
        cpu_freq = 0;
    } else {
        cpu_freq = A370_CPU_FREQS[cpu_freq_select as usize];
    }
    cpu_freq
}

static A370_NBCLK_RATIOS: [[i32; 2]; 32] = [
    [0, 1], [1, 2], [2, 2], [2, 2], [1, 2], [1, 2], [1, 1], [2, 3],
    [0, 1], [1, 2], [2, 4], [0, 1], [1, 2], [0, 1], [0, 1], [2, 2],
    [0, 1], [0, 1], [0, 1], [1, 1], [2, 3], [0, 1], [0, 1], [0, 1],
    [0, 1], [0, 1], [0, 1], [1, 1], [0, 1], [0, 1], [0, 1], [0, 1],
];

static A370_HCLK_RATIOS: [[i32; 2]; 32] = [
    [0, 1], [1, 2], [2, 6], [2, 3], [1, 3], [1, 4], [1, 2], [2, 6],
    [0, 1], [1, 6], [2, 10], [0, 1], [1, 4], [0, 1], [0, 1], [2, 5],
    [0, 1], [0, 1], [0, 1], [1, 2], [2, 6], [0, 1], [0, 1], [0, 1],
    [0, 1], [0, 1], [0, 1], [1, 1], [0, 1], [0, 1], [0, 1], [0, 1],
];

static A370_DRAMCLK_RATIOS: [[i32; 2]; 32] = [
    [0, 1], [1, 2], [2, 3], [2, 3], [1, 3], [1, 2], [1, 2], [2, 6],
    [0, 1], [1, 3], [2, 5], [0, 1], [1, 4], [0, 1], [0, 1], [2, 5],
    [0, 1], [0, 1], [0, 1], [1, 1], [2, 3], [0, 1], [0, 1], [0, 1],
    [0, 1], [0, 1], [0, 1], [1, 1], [0, 1], [0, 1], [0, 1], [0, 1],
];

unsafe fn a370_get_clk_ratio(sar: *mut core::ffi::c_void, id: i32, mult: *mut i32, div: *mut i32) {
    let opt = ((readl(sar) >> SARL_A370_FAB_FREQ_OPT) & SARL_A370_FAB_FREQ_OPT_MASK) as usize;
    let ratio = match id {
        A370_CPU_TO_NBCLK => &A370_NBCLK_RATIOS[opt],
        A370_CPU_TO_HCLK => &A370_HCLK_RATIOS[opt],
        A370_CPU_TO_DRAMCLK => &A370_DRAMCLK_RATIOS[opt],
        _ => return,
    };
    *mult = ratio[0];
    *div = ratio[1];
}

unsafe fn a370_is_sscg_enabled(sar: *mut core::ffi::c_void) -> bool {
    (readl(sar) & SARL_A370_SSCG_ENABLE) == 0
}

static A370_CORECLKS: coreclk_soc_desc = coreclk_soc_desc {
    get_tclk_freq: Some(a370_get_tclk_freq),
    get_cpu_freq: Some(a370_get_cpu_freq),
    get_clk_ratio: Some(a370_get_clk_ratio),
    is_sscg_enabled: Some(a370_is_sscg_enabled),
    fix_sscg_deviation: Some(kirkwood_fix_sscg_deviation),
    ratios: &A370_CORECLK_RATIOS,
    num_ratios: A370_CORECLK_RATIOS.len(),
};

/* Clock Gating Control */

static A370_GATING_DESC: [clk_gating_soc_desc; 14] = [
    clk_gating_soc_desc { name: "audio", parent: core::ptr::null(), bit_idx: 0, flags: 0 },
    clk_gating_soc_desc { name: "pex0_en", parent: core::ptr::null(), bit_idx: 1, flags: 0 },
    clk_gating_soc_desc { name: "pex1_en", parent: core::ptr::null(), bit_idx: 2, flags: 0 },
    clk_gating_soc_desc { name: "ge1", parent: core::ptr::null(), bit_idx: 3, flags: 0 },
    clk_gating_soc_desc { name: "ge0", parent: core::ptr::null(), bit_idx: 4, flags: 0 },
    clk_gating_soc_desc { name: "pex0", parent: "pex0_en", bit_idx: 5, flags: 0 },
    clk_gating_soc_desc { name: "pex1", parent: "pex1_en", bit_idx: 9, flags: 0 },
    clk_gating_soc_desc { name: "sata0", parent: core::ptr::null(), bit_idx: 15, flags: 0 },
    clk_gating_soc_desc { name: "sdio", parent: core::ptr::null(), bit_idx: 17, flags: 0 },
    clk_gating_soc_desc { name: "crypto", parent: core::ptr::null(), bit_idx: 23, flags: CLK_IGNORE_UNUSED },
    clk_gating_soc_desc { name: "tdm", parent: core::ptr::null(), bit_idx: 25, flags: 0 },
    clk_gating_soc_desc { name: "ddr", parent: core::ptr::null(), bit_idx: 28, flags: CLK_IGNORE_UNUSED },
    clk_gating_soc_desc { name: "sata1", parent: core::ptr::null(), bit_idx: 30, flags: 0 },
    clk_gating_soc_desc::default(),
];

unsafe fn a370_clk_init(np: *mut device_node) {
    let cgnp = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(),
        "marvell,armada-370-gating-clock");
    mvebu_coreclk_setup(np, &A370_CORECLKS);
    if !cgnp.is_null() {
        mvebu_clk_gating_setup(cgnp, &A370_GATING_DESC);
        of_node_put(cgnp);
    }
}

// CLK_OF_DECLARE(a370_clk, "marvell,armada-370-core-clock", a370_clk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
